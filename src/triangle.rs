use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{dot, Point3, unit_vector, cross},
    aabb::Aabb,
    utils::{fmax, fmin}
};
use std::f64::consts::PI;

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
}
