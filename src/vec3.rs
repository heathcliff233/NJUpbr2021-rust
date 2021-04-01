use crate::{random_double, utils::PI};
use std::fmt;
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub, Index};
use ply_rs::ply;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn random(min_max: Option<[f64; 2]>) -> Self {
        Self {
            x: random_double!(min_max),
            y: random_double!(min_max),
            z: random_double!(min_max),
        }
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3 ({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}


impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(e: [f64; 3]) -> Self {
        Self {
            x: e[0],
            y: e[1],
            z: e[2],
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Type aliases for Vec3
pub type Point3 = Vec3; // 3D point
pub type Color = Vec3; // RGB color

// Vec3 utility functions
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from([
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    ])
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(Some([-1.0, 1.0]));
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_double!(0.0, 2.0 * PI);
    let z = random_double!(-1.0, 1.0);
    let r = f64::sqrt(1.0 - z.powi(2));
    Vec3::from([r * f64::cos(a), r * f64::sin(a), z])
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::from([random_double!(-1.0, 1.0), random_double!(-1.0, 1.0), 0.0]);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * n
}

pub fn identity(a1: Vec3, a2: Vec3, a3: Vec3) -> f64 {
    - dot(&cross(a1, a2), &a3)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, &n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -f64::sqrt(1.0 - r_out_parallel.length_squared()) * n;
    r_out_parallel + r_out_perp
}


impl ply::PropertyAccess for Point3 {
    fn new() -> Self {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("x", ply::Property::Float(v)) => self.x = v as f64,
            ("y", ply::Property::Float(v)) => self.y = v as f64,
            ("z", ply::Property::Float(v)) => self.z = v as f64,
            ("confidence", _) => {},
            ("intensity", _) => {},
            (_k, _) => {},
            //(k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}
