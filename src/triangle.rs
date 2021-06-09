use crate::{
    hittable::{HitRecord, Hittable, Shape},
    material::Material,
    ray::Ray,
    vec3::{dot, Point3, unit_vector, cross},
    aabb::Aabb,
    utils::{fmax, fmin},
    random_double,
};
use std::f64::consts::PI;
use std::f64::INFINITY;

#[derive(Clone)]
pub struct Triangle {
    a0: Point3,
    a1: Point3,
    a2: Point3,
    material: Material,
}

impl Triangle {
    pub fn new(a0: Point3, a1: Point3, a2: Point3, material: Material) -> Self {
        Self {
            a0,
            a1,
            a2,
            material
        }
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        *u = 1.0 - (phi + PI) / (2.0 * PI);
        *v = (theta + PI / 2.0) / PI;
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = r.origin - self.a0;
        //let d = unit_vector(r.direction);
        let d = r.direction;
        let e1 = self.a1 - self.a0;
        let e2 = self.a2 - self.a0;
        let q = cross(t, e1);
        let p = cross(d, e2);
        let res: Point3 = 1.0/(dot(&p, &e1)) * Point3::new(dot(&q,&e2), dot(&p,&t), dot(&q,&d));

        if res.y<0.0 || res.z<0.0 || res.y+res.z>1.0 || res.x<t_min || res.x>t_max {
            return false
        }

        rec.t = res.x;
        rec.p = r.at(rec.t);
        let outward_normal = unit_vector(cross(e1, e2));
        rec.set_face_normal(r, &outward_normal);
        Triangle::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.clone();
        return true;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let x_mx = fmax(fmax(self.a0.x, self.a1.x),self.a2.x);
        let y_mx = fmax(fmax(self.a0.y, self.a1.y),self.a2.y);
        let z_mx = fmax(fmax(self.a0.z, self.a1.z),self.a2.z);
        let x_mn = fmin(fmin(self.a0.x, self.a1.x),self.a2.x);
        let y_mn = fmin(fmin(self.a0.y, self.a1.y),self.a2.y);
        let z_mn = fmin(fmin(self.a0.z, self.a1.z),self.a2.z);
        output_box.modify(Point3::from([x_mn+0.0001, y_mn+0.0001, z_mn+0.0001]), Point3::from([x_mx, y_mx, z_mx]));
        true

    }

    fn pdf_value(&self, origin: Point3, direction: Point3, time: f64) -> f64 {
        let mut rec = HitRecord::new(Material::new_lambertian(Point3::zero()));
        let center = (self.a0 + self.a1 + self.a2) / 3.0;
        let radius = (self.a0 - center).length();
        if !self.hit(&Ray::new(origin, direction, time), 0.001, INFINITY, &mut rec) {
            0.0
        } else {
            let cos_theta_max = (1.0 - radius * radius / (origin - center).length_squared()).sqrt();
            return 1.0 / (4.0 * PI * (1.0 - cos_theta_max))
        }
    }

    fn random(&self, origin: Point3) -> Point3 {
        let p1 = random_double!();
        let p2 = p1.sqrt();
        let chosen = prop(self.a0, self.a1, p2) + prop(self.a1, self.a2, p1);
        chosen - origin
    }

    fn add(&mut self, obj: Shape) {}

}

pub fn prop(origin: Point3, dest: Point3, p: f64) -> Point3 {
    origin + p * (dest - origin)
}
