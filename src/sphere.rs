use crate::interval::Interval;
use crate::vec3::{dot, Point3};
use crate::ray::{Ray};
use crate::hit_record::{Hittable, HitRecord};

pub struct Sphere {
  center: Point3,
  radius: f64
}

pub fn sphere(center: Point3, radius: f64) -> Sphere {
  return Sphere{center,radius: f64::max(0., radius)}
}

impl Hittable for Sphere {
  fn hit(self: &Sphere, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let oc = &self.center - &r.origin;
    let a = r.direction.length_squared();
    let h = dot(&r.direction, &oc);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = h*h - a*c;

    if (discriminant < 0.) {
      return None;
    }

    let sqrtd = f64::sqrt(discriminant);
    let root = {
      let root1 = (h - sqrtd) / a;
      match ray_t.contains(root1) {
        true => Some(root1),
        false => {
          let root2 = (h + sqrtd) / a;
          match ray_t.contains(root2) {
            true => Some(root2),
            false => None
          }
        }
      }
    }?;

    let outward_normal = (r.at(root) - &self.center) / self.radius;

    return Some(HitRecord::from_ray_norm(r, root, outward_normal));
  }
}