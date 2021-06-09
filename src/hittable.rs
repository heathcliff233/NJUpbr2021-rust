use crate::{
    material::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
    sphere::Sphere,
    triangle::Triangle,
    hittable_list::HittableList,
    aabb::Aabb,
    bvh::BvhNode
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(material: Material) -> Self {
        Self {
            p: Point3::zero(),
            normal: Point3::zero(),
            material,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
    fn pdf_value(&self, origin: Point3, direction: Point3, time: f64) -> f64;
    fn random(&self, origin: Point3) -> Point3;
    fn add(&mut self, obj: Shape);
}

#[derive(Clone)]
pub enum Shape {
    Sphere(Box<Sphere>),
    Triangle(Box<Triangle>),
    BvhNode(Box<BvhNode>),
    HittableList(Box<HittableList>),
}

impl Shape {
    pub fn new_sphere(center: Point3, radius: f64, material: Material) -> Self {
        Shape::Sphere(Box::new(Sphere::new(center, radius, material)))
    }
    pub fn new_triangle(a0: Point3, a1: Point3, a2: Point3, material: Material) -> Self {
        Shape::Triangle(Box::new(Triangle::new(a0, a1, a2, material)))
    }
    pub fn new_hittable_list() -> Self {
        Shape::HittableList(Box::new(HittableList::default()))
    }
}

impl Hittable for Shape {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord
    ) -> bool {
        match self {
            Shape::Sphere(m) => m.hit(r, t_min, t_max, rec),
            Shape::Triangle(m) => m.hit(r, t_min, t_max, rec),
            Shape::BvhNode(m) => m.hit(r, t_min, t_max, rec),
            Shape::HittableList(m) => m.hit(r, t_min, t_max, rec),
        }
    }

    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        bounding_box: &mut Aabb
    ) -> bool {
        match self {
            Shape::Sphere(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::Triangle(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::BvhNode(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::HittableList(m) => m.bounding_box(time0, time1, bounding_box),
        }
    }

    fn pdf_value(&self, origin: Point3, direction: Point3, time: f64) -> f64 {
        match self {
            Shape::Sphere(m) => m.pdf_value(origin, direction, time),
            Shape::Triangle(m) => m.pdf_value(origin, direction, time),
            _ => 0.0
        }
    }

    fn random(&self, origin: Point3) -> Point3 {
        match self {
            Shape::Sphere(m) => m.random(origin),
            Shape::Triangle(m) => m.random(origin),
            Shape::HittableList(m) => m.random(origin),
            _ => Vec3::ones(),
        }
    }

    fn add(&mut self, obj: Shape) {
        match self {
            Shape::HittableList(m) => m.add(obj),
            _ => {}
        }
    }

}
