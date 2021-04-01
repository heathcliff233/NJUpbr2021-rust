use crate::{
    hittable::{HitRecord, Hittable, Shape},
    ray::Ray,
    aabb::{Aabb, surrounding_box}
};
//use std::sync::Arc;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Shape>,
}

impl HittableList {
    pub fn add(&mut self, object: Shape) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() { return false }

        let mut temp_box = Aabb::default();
        let mut first_box = true;

        for object in self.objects.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box) { return false }
            *output_box = if first_box { temp_box.clone() } else {surrounding_box(output_box, &temp_box)};
            first_box = false;
        }

        true
    }

}
