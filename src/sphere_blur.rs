use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{dot, Point3, align_min, align_max},
    aabb::Aabb
};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct SphereBlur {
    center1: Point3,
    center2: Point3,
    radius: f64,
    material: Material,
    time1: f64,
    time2: f64,
}

impl SphereBlur {
    pub fn new(center1: Point3, center2: Point3, radius: f64, material: Material, time1: f64, time2: f64,) -> Self {
        Self {
            center1,
            center2,
            radius,
            material,
            time1,
            time2,
        }
    }

    pub fn get_center(&self, time:f64) -> Point3 {
        self.center1 + (time - self.time1)/(self.time2 - self.time1)*(self.center2 - self.center1)
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        *u = 1.0 - (phi + PI) / (2.0 * PI);
        *v = (theta + PI / 2.0) / PI;
    }
}

impl Hittable for SphereBlur {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.get_center(r.time);
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let root = f64::sqrt(discriminant);
        let mut temp = (-half_b - root) / a;
        if temp > t_max || temp < t_min {
            temp = (-half_b + root) / a;
            if temp > t_max || temp < t_min {
                return false;
            }
        }

        rec.t = temp;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.get_center(r.time)) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        SphereBlur::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.clone();
        return true;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let r = Point3::from([self.radius, self.radius, self.radius]);
        output_box.modify(align_min(self.center1, self.center2)-r, align_max(self.center1, self.center2)+r);
        return true
    }
}
