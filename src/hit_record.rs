use crate::interval::Interval;
use crate::vec3::{Point3, Vec3, dot};
use crate::ray::Ray;

pub struct HitRecord {
  /// The position of the hit
  pub p: Point3,
  /// The surface normal in the opposite direction of the hit ray
  pub normal: Vec3,
  /// The position along the ray of the hit, i.e. p = t*r
  pub t: f64,
  /// True if the hitting ray came from outside the object, false if it came from inside
  pub front_face: bool
}

impl HitRecord {
  /// ## Parameters
  ///
  /// * r - The ray along which the hit takes place
  /// * t - the point along r which the hit happens, i.e. the hit position is t*r
  /// * outward_normal - The normal facing outward from the object that was hit
  ///
  /// ## Returns
  ///
  /// * A record describing the hit
  pub fn from_ray_norm(r: &Ray, t: f64, outward_normal: Vec3) -> HitRecord {
    let front_face = dot(&r.direction, &outward_normal) < 0.;
    HitRecord{
      p: r.at(t),
      normal : match front_face { true => outward_normal, false => -outward_normal },
      t,
      front_face
    }
  }
}

pub trait Hittable {
  fn hit(self: &Self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}