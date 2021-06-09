use crate::vec3::{Vec3, unit_vector, cross};

pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn build_from_w(normal: Vec3) -> ONB {
        let w = unit_vector(normal);
        let a: Vec3 = if (w.x).abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = unit_vector(cross(w, a));
        let u = cross(w, v);

        ONB { u, v, w }
    }

    pub fn local(&self, vec: Vec3) -> Vec3 {
        vec.x * self.u + vec.y * self.v + vec.z * self.w
    }
}