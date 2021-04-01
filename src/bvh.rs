use crate::{
    hittable::{HitRecord, Hittable, Shape},
    ray::Ray,
    aabb::{Aabb, surrounding_box}
};
use rand::Rng;
use std::cmp::Ordering;
//use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    left: Box<Shape>,
    right: Box<Shape>,
    bbox: Aabb
}

fn box_x_compare (a: &Shape, b: &Shape) -> Ordering {
    let mut box_a = Aabb::default();
    let mut box_b = Aabb::default();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.\n");
    };
    if box_a.min().x < box_b.min().x {
        return Ordering::Less
    } else if box_a.min().y > box_b.min().y {
        return Ordering::Greater
    }
    return Ordering::Equal
}

fn box_y_compare (a: &Shape, b: &Shape) -> Ordering {
    let mut box_a = Aabb::default();
    let mut box_b = Aabb::default();
    if !a.bounding_box(0.0,0.0, &mut box_a) || !b.bounding_box(0.0,0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.\n");
    };
    if box_a.min().y < box_b.min().y {
        return Ordering::Less
    } else if box_a.min().y > box_b.min().y {
        return Ordering::Greater
    }
    return Ordering::Equal
}

fn box_z_compare (a: &Shape, b: &Shape) -> Ordering {
    let mut box_a = Aabb::default();
    let mut box_b = Aabb::default();
    if !a.bounding_box(0.0,0.0, &mut box_a) || !b.bounding_box(0.0,0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.\n");
    };
    if box_a.min().x < box_b.min().x {
        return Ordering::Less
    } else if box_a.min().y > box_b.min().y {
        return Ordering::Greater
    }
    return Ordering::Equal
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Shape>, start: usize, end: usize) -> Shape {
        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0,3);
        let comparator = if axis==0 {box_x_compare} else if axis==1 {box_y_compare} else {box_z_compare};
        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (Box::from(objects[start].clone()), Box::from(objects[start].clone())),
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    (Box::from(objects[start].clone()), Box::from(objects[start + 1].clone()))
                } else {
                    (Box::from(objects[start+1].clone()), Box::from(objects[start].clone()))
                }
            }
            _ => {
                objects.as_mut_slice()[start..end].sort_by(comparator);
                let mid = (start + end)/2;
                (Box::new(BvhNode::new(objects, start, mid)), Box::new(BvhNode::new(objects, mid, end)))
            }
        };
        let mut box_left= Aabb::default();
        let mut box_right= Aabb::default();
        if !left.bounding_box(0.0, 0.0, &mut box_left)
            || !right.bounding_box(0.0, 0.0, &mut box_right)
        {
            println!("No bounding box in bvh_node constructor.\n");
        }
        let bbox = surrounding_box(&box_left, &box_right);
        Shape::BvhNode(Box::from(BvhNode{
            left,
            right,
            bbox
        }))
    }
}


impl Hittable for BvhNode {
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t_min, t_max) {return false}

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, if hit_left {rec.t} else {t_max}, rec);

        hit_left || hit_right

    }


    fn bounding_box(&self, _time0: f64, _time1:f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox.clone();
        true
    }
}
