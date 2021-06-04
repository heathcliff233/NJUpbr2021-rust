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
//use std::sync::Arc;
//use image::GenericImageView;

#[macro_use]
extern crate rayt;
#[macro_use]
extern crate itertools;

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

/*
fn read_image() -> HittableList {
    let mut world = HittableList::default();
    let ground_material = Material::new_metal(Color::from([0.5, 0.5, 0.5]), 0.0);
    world.add(Shape::new_sphere(
        Point3::from([0.0, -1000.0, 0.0]),
        1000.0,
        ground_material,
    ));
    let image1 = image::open("assets/1.png").unwrap();
    for a in 0..image1.width() {
        for b in 0..image1.height() {
            let pixel_color = image1.get_pixel(a, b);
            if pixel_color[0]+pixel_color[1]+pixel_color[2] > 150  {
                let color = Color::new(0.9, 0.2, 0.3);
                let sphere_material = Material::new_metal(color, 0.0);
                world.add(Shape::new_sphere(
                    Point3::new(a as f64 / 5.0 - 7.0, (image1.height() - b) as f64 / 5.0, 0.0),
                    0.18,
                    sphere_material,
                ));
            }
        }
    }
    world
}
*/

fn cyl() -> HittableList {
    let mut world = HittableList::default();
    let ground_material = Material::new_metal(Color::from([0.7, 0.2, 0.1]), 0.7);
    //let ground_material = Material::new_lambertian(Color::from([0.5, 0.5, 0.5]));
    /*
    world.add(Shape::new_sphere(
        Point3::from([0.0, -1000.0, 0.0]),
        1000.0,
        ground_material,
    ));
    */
    world.add(Shape::new_triangle(Point3::from([1000.0,0.0,0.0]),Point3::from([0.0,0.0,-1000.0]),Point3::from([0.0,0.0,1000.0]),ground_material.clone()));
    world.add(Shape::new_triangle(Point3::from([0.0,0.0,-1000.0]),Point3::from([-1000.0,0.0,0.0]),Point3::from([0.0,0.0,1000.0]),ground_material.clone()));

    let cyl_mat = Material::new_metal(Color::from([0.8, 0.6, 0.4]), 0.1);
    //world.add(Shape::new_cylinder(1.0,1.0,cyl_mat));
    world.add(Shape::new_sphere(Vec3::from([0.0, 1.0, 0.0]), 1.0, Material::new_noise_lamb(4.0)));
    world.add(Shape::new_sphere_blur(Vec3::from([0.5, 0.3, -0.5]), Vec3::from([1.0,0.3,-0.5]),0.3,cyl_mat.clone(),0.0,1.0));
    world.add(Shape::new_sphere(Vec3::from([4.0, 0.3,-2.0]), 0.3, Material::new_image_tex(&String::from("assets/1.png"))));
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
const IMAGE_WIDTH: u32 = 12000;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 50;
const MAX_DEPTH: usize = 50;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    //let world = read_image();
    let world = cyl();
    let lookfrom = Point3::from([8.0, 1.0, -5.0])*1.5;
    let lookat = Point3::from([0.0,0.5,0.0]);
    let vup = Vec3::from([0.0, 1.0, 0.0]);
    let dist_to_focus = (lookat - lookfrom).length();
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
        1.0,
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