use crate::{
    hittable::Shape,
    onb::ONB,
    vec3::{Vec3,unit_vector, dot},
    random_double,
};
use rand::prelude::*;
use std::f64::consts::PI;
use crate::hittable::Hittable;

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double!();
    let r2 = random_double!();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::from([x,y,z])
}

pub enum PDF<'a> {
    CosinePDF(CosinePDF),
    HitablePDF(HitablePDF<'a>),
    MixturePDF(MixturePDF<'a>),
    ZeroPDF(ZeroPDF),
}

impl<'a> PDF<'a> {
    pub fn value(&self, direction: Vec3, time: f64) -> f64 {
        match self {
            PDF::CosinePDF(p) => p.value(direction),
            PDF::HitablePDF(p) => p.value(direction, 0.001),
            PDF::MixturePDF(p) => p.value(direction, 0.001),
            PDF::ZeroPDF(p) => p.value(direction),
        }
    }
    pub fn generate(&self) -> Vec3 {
        match self {
            PDF::CosinePDF(p) => p.generate(),
            PDF::HitablePDF(p) => p.generate(),
            PDF::MixturePDF(p) => p.generate(),
            PDF::ZeroPDF(p) => p.generate(),
        }
    }
}

pub struct CosinePDF {
    uvw: ONB,
}

impl<'a> CosinePDF {
    pub fn new(w: Vec3) -> PDF<'a> {
        PDF::CosinePDF(CosinePDF {
            uvw: ONB::build_from_w(w),
        })
    }

    pub fn value(&self, direction: Vec3) -> f64 {
        let cosine = dot(&unit_vector(direction), &self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    pub fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}

pub struct HitablePDF<'a> {
    origin: Vec3,
    hitable: &'a Shape,
}

impl<'a> HitablePDF<'a> {
    pub fn new(hitable: &'a Shape, origin: Vec3) -> PDF {
        PDF::HitablePDF(HitablePDF { origin, hitable })
    }

    pub fn value(&self, direction: Vec3, time: f64) -> f64 {
        self.hitable.pdf_value(self.origin, direction, time)
    }

    pub fn generate(&self) -> Vec3 {
        self.hitable.random(self.origin)
    }
}

pub struct MixturePDF<'a> {
    // Arc to prevent infinite size
    pdf1: Box<PDF<'a>>,
    pdf2: Box<PDF<'a>>,
}

impl<'a> MixturePDF<'a> {
    pub fn new(pdf1: PDF<'a>, pdf2: PDF<'a>) -> PDF<'a> {
        PDF::MixturePDF(MixturePDF {
            pdf1: Box::new(pdf1),
            pdf2: Box::new(pdf2),
        })
    }

    pub fn value(&self, direction: Vec3, time: f64) -> f64 {
        0.5 * self.pdf1.value(direction, time) + 0.5 * self.pdf2.value(direction, time)
    }

    pub fn generate(&self) -> Vec3 {
        if random_double!() < 0.5 {
            self.pdf1.generate()
        } else {
            self.pdf2.generate()
        }
    }
}

// TODO: this is an ugly hack due to tutorial saying `srec.pdf_ptr = 0;` in 12.2 Handling Specular for Metal
pub struct ZeroPDF {}

impl<'a> ZeroPDF {
    pub fn new() -> PDF<'a> {
        PDF::ZeroPDF(ZeroPDF {})
    }

    pub fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }

    pub fn generate(&self) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}