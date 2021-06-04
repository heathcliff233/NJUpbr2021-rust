use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Vec3, Point3},
    aabb::Aabb
};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct Cylinder {
    r: f64,
    d: f64,
    material: Material,
}

impl Cylinder {
    pub fn new(r: f64, d: f64, material: Material) -> Self {
        Self {
            r,
            d,
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

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let a = r.direction.x.powi(2) + r.direction.z.powi(2);
        let hb = r.direction.x * r.origin.x + r.direction.z * r.origin.z;
        let c = r.origin.x.powi(2) + r.origin.z.powi(2) - self.r.powi(2);
        let det2 = hb.powi(2) - a * c;
        if det2 < 0.0 {
            return false
        }
        let root = f64::sqrt(det2);
        let tmp1 = r.origin.y + r.direction.y * (-hb - root) / a;
        let tmp2 = r.origin.y + r.direction.y * (-hb + root) / a;
        if !(((0.0<tmp1)&&(tmp1<self.d))||((0.0<tmp2)&&(tmp2<self.d))) {
            return false
        }
        if (0.0<tmp1)&&(tmp1<self.d) {
            if (tmp1<t_min)||(tmp1>t_max) {
                return false
            }
            rec.t = (-hb-root)/a;
            rec.p = r.at(rec.t);
            let outward_normal = Vec3::from([rec.p.x, 0.0, rec.p.z]) / self.r;
            rec.set_face_normal(r, &outward_normal);
            Cylinder::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
            rec.material = self.material.clone();
            return true;
        } else {
            rec.t = (self.d - r.origin.y)/r.direction.y;
            rec.p = r.at(rec.t);
            let outward_normal = Vec3::from([0.0,1.0,0.0]);
            Cylinder::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
            rec.material = self.material.clone();
            return true;

        }

    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        output_box.modify(Point3::from([(-1.0)*self.r, 0.0, (-1.0)*self.r]), Point3::from([self.r, self.d, self.r]));
        true
    }
}
