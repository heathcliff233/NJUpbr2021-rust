use crate::{
    material::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
    sphere::Sphere,
    triangle::Triangle
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
}

pub enum Shape {
    Sphere(Sphere),
    Triangle(Triangle)
}

impl Shape {
    pub fn new_sphere(center: Point3, radius: f64, material: Material) -> Self {
        Shape::Sphere(Sphere::new(center, radius, material))
    }
    pub fn new_triangle(a0: Point3, a1: Point3, a2: Point3, material: Material) -> Self {
        Shape::Triangle(Triangle::new(a0, a1, a2, material))
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
        }
    }
}
