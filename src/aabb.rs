use crate::{
    ray::Ray,
    vec3::Point3,
    utils::{fmax, fmin}
};

#[derive(Clone, Copy)]
pub struct Aabb {
    minimum: Point3 ,
    maximum: Point3
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum}
    }
    pub fn default() -> Self {
        Self { minimum: Point3::zero(), maximum: Point3::zero() }
    }
    pub fn modify(&mut self, minimum: Point3, maximum: Point3)  {
        self.minimum = minimum;
        self.maximum = maximum;
    }

    pub fn max(&self) -> Point3 {self.maximum}
    pub fn min(&self) -> Point3 {self.minimum}

    pub fn hit(&self, r:&Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let mut t0 = fmin((self.minimum[a] - r.origin[a]) / r.direction[a],
                           (self.maximum[a] - r.origin[a]) / r.direction[a]);
            let mut t1 = fmax((self.minimum[a] - r.origin[a]) / r.direction[a],
                           (self.maximum[a] - r.origin[a]) / r.direction[a]);
            t0 = fmax(t0, t_min);
            t1 = fmin(t1, t_max);
            if t1 <= t0 { return false }
        }
        return true
    }

    pub fn area(&self) ->f64 {
        let a = self.maximum.x - self.minimum.x;
        let b = self.maximum.y - self.minimum.y;
        let c = self.maximum.z - self.minimum.z;
        return 2.0*(a*b + b*c + c*a);
    }

    pub fn longest_axis(&self) -> u32 {
        let a = self.maximum.x - self.minimum.x;
        let b = self.maximum.y - self.minimum.y;
        let c = self.maximum.z - self.minimum.z;
        return if a > b && a > c { 0 } else if b > c { 1 } else { 2 }
    }

}

pub(crate) fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Point3::from([fmin(box0.min().x, box1.min().x),
        fmin(box0.min().y, box1.min().y),
        fmin(box0.min().z, box1.min().z)]);

    let big = Point3::from([fmax(box0.max().x, box1.max().x),
        fmax(box0.max().y, box1.max().y),
        fmax(box0.max().z, box1.max().z)]);

    return Aabb::new(small,big);
}
