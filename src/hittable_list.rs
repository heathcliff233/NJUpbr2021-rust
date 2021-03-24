use crate::{
    hittable::{HitRecord, Hittable, Shape},
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Shape>,
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
}
