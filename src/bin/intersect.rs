use rayt::{
    camera::Camera,
    hittable::{HitRecord, Hittable, Shape},
    hittable_list::HittableList,
    material::{Material, Scatter},
    ray::Ray,
    utils::{INFINITY, clamp},
    vec3::{unit_vector, Color, Point3, Vec3},
};

use rayon::prelude::*;

#[macro_use]
extern crate rayt;
#[macro_use]
extern crate itertools;
extern crate ply_rs;
use ply_rs::ply;
use ply_rs::parser;

fn ray_color(r: &Ray, world: &HittableList, depth: usize) -> Color {
    let mut rec = HitRecord::new(Material::new_lambertian(Color::zero()));
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point3::zero(), Vec3::zero(), 0.0);
        let mut attenuation = Color::zero();
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0);
}

struct Face {
    vertex_index: Vec<i32>,
}

struct Vertex {
    vert: Point3,
    norm: Point3
}

impl ply::PropertyAccess for Face {
    fn new() -> Self {
        Face {
            vertex_index: Vec::new(),
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("vertex_indices", ply::Property::ListInt(vec)) => self.vertex_index = vec,
            (k, _) => panic!("Face: Unexpected key/value combination: key: {}", k),
        }
    }
}

impl ply::PropertyAccess for Vertex {
    fn new() -> Self {
        Vertex {
            vert: Point3::zero(),
            norm: Point3::zero(),
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("x", ply::Property::Float(v)) => self.vert.x = v as f64,
            ("y", ply::Property::Float(v)) => self.vert.y = v as f64,
            ("z", ply::Property::Float(v)) => self.vert.z = v as f64,
            ("nx",ply::Property::Float(v)) => self.norm.x = v as f64,
            ("ny",ply::Property::Float(v)) => self.norm.y = v as f64,
            ("nz",ply::Property::Float(v)) => self.norm.z = v as f64,
            ("confidence", _) => {},
            ("intensity", _) => {},
            (_k, _) => {},
            //(k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}


fn read_ply() -> HittableList {
    let mut world = HittableList::default();
    let ground_material = Material::new_metal(Color::from([0.5, 0.5, 0.5]), 0.0);
    world.add(Shape::new_triangle(Point3::from([1000.0,-0.5,0.0]),Point3::from([0.0,-0.5,-1000.0]),Point3::from([0.0,-0.5,1000.0]),ground_material));
    world.add(Shape::new_triangle(Point3::from([0.0,-0.5,-1000.0]),Point3::from([-1000.0,-0.5,0.0]),Point3::from([0.0,-0.5,1000.0]),ground_material));
    let path = "assets/key.ply";
    let f = std::fs::File::open(path).unwrap();
    let mut f = std::io::BufReader::new(f);

    //let vertex_parser = parser::Parser::<Point3>::new();
    let vertex_parser = parser::Parser::<Vertex>::new();
    let face_parser = parser::Parser::<Face>::new();

    let header = vertex_parser.read_header(&mut f).unwrap();

    let mut vertex_list = Vec::new();
    let mut face_list = Vec::new();
    for (_ignore_key, element) in &header.elements {
        match element.name.as_ref() {
            "vertex" => {vertex_list = vertex_parser.read_payload_for_element(&mut f, &element, &header).unwrap();},
            "face" => {face_list = face_parser.read_payload_for_element(&mut f, &element, &header).unwrap();},
            _ => panic!("Enexpeced element!"),
        }
    }
    let cube_mat = Material::new_lambertian(Color::from([0.7,0.2,0.1]));
    for fc in face_list.iter() {
        //world.add(Shape::new_triangle(vertex_list[fc.vertex_index[0] as usize]/20.0,vertex_list[fc.vertex_index[1] as usize]/20.0, vertex_list[fc.vertex_index[2] as usize]/20.0, cube_mat));
        //world.add(Shape::new_triangle(vertex_list[fc.vertex_index[0] as usize].vert/20.0,vertex_list[fc.vertex_index[1] as usize].vert/20.0,vertex_list[fc.vertex_index[2] as usize].vert/20.0, cube_mat));

        world.add(Shape::new_mesh(
            vertex_list[fc.vertex_index[0] as usize].vert/20.0,
            vertex_list[fc.vertex_index[1] as usize].vert/20.0,
            vertex_list[fc.vertex_index[2] as usize].vert/20.0,
            vertex_list[fc.vertex_index[0] as usize].norm,
            vertex_list[fc.vertex_index[1] as usize].norm,
            vertex_list[fc.vertex_index[2] as usize].norm,
            cube_mat));

    }
    world
}

fn render(cam: &Camera, world: &HittableList) -> Vec<Pixel> {
    let pix_coord: Vec<(u32,u32)> = iproduct!((0..IMAGE_HEIGHT).rev(), 0..IMAGE_WIDTH).collect();
    let img: Vec<Pixel> = pix_coord.par_iter().map(|(row, col)| simu(*row, *col, cam, world)).collect();
    img
}

fn simu(row: u32, col: u32, cam: &Camera, world: &HittableList) -> Pixel {
    let pixel_color = (1..=SAMPLES_PER_PIXEL)
        .map(|_| {
            let u = (col as f64 + random_double!()) / (IMAGE_WIDTH - 1) as f64;
            let v = (row as f64 + random_double!()) / ( IMAGE_HEIGHT - 1) as f64;
            ray_color(&cam.get_ray(u, v), world, MAX_DEPTH)
        })
        .fold(Color::default(), |sum, c| sum + c);
    //write_color(pixel_color, 20);
    let scale = 1.0 / 20.0 as f64;
    let get_color = |c| (255.999 * clamp(f64::sqrt(scale * c), 0.0, 0.999)) as u32;
    let r = get_color(pixel_color.x);
    let g = get_color(pixel_color.y);
    let b = get_color(pixel_color.z);
    Pixel {r, g, b}
}

pub struct Pixel {
    r: u32,
    g: u32,
    b: u32,
}
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 20;
const MAX_DEPTH: usize = 5;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let world = read_ply();
    let lookfrom = Point3::from([7.0, 3.0, -7.0]);
    //let lookfrom = Point3::from([0.7,0.1,0.7]);
    let lookat = Point3::from([0.0, -0.5, 0.0]);
    let vup = Vec3::from([0.0, 1.0, 0.0]);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        0.0,
    );

    let img: Vec<Pixel> = render(&cam, &world);
    img.iter().for_each(|it| {
        println!("{} {} {}", it.r, it.g, it.b);
    });
    //println!("{}", img.len());
    /*
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let pixel_color = (1..=SAMPLES_PER_PIXEL)
                .map(|_| {
                    let u = (i as f64 + random_double!()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random_double!()) / (IMAGE_HEIGHT - 1) as f64;
                    ray_color(&cam.get_ray(u, v), &world, MAX_DEPTH)
                })
                .fold(Color::default(), |sum, c| sum + c);
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    */

    eprintln!("\nDone.");
}