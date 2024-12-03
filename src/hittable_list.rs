use std::{vec::*};
use crate::{hit_record::{HitRecord, Hittable}, interval::{interval, Interval}, ray::Ray};

pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new() -> HittableList {
    HittableList{
      objects: vec![]
    }
  }

  pub fn clear(&mut self) {
    self.objects.clear()
  }

  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object)
  }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let mut closest_so_far = ray_t.max;
    let mut hit_anything = false;
    let mut res = None;

    for object in self.objects.iter() {
      match object.hit(r, interval(ray_t.min, closest_so_far)) {
        Some(hit_record) => {
          hit_anything = true;
          closest_so_far = hit_record.t;
          res = Some(hit_record)
        }
        None => ()
      }
    };

    res
  }
}