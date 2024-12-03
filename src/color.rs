use crate::vec3;
use std::io::Write;

/// x - red component
/// y - green component
/// z - blue component
pub type Color = vec3::Vec3;

impl Color {
  pub fn r(&self) -> f64 {
    self.x
  }

  pub fn g(&self) -> f64 {
    self.y
  }

  pub fn b(&self) -> f64 {
    self.z
  }
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
  return Color{x: r, y: g, z: b};
}

pub fn write_color(w: &mut (impl Write), pixel_color: &Color) {
  let (r, g, b) = (pixel_color.x, pixel_color.y, pixel_color.z);

  let rbyte = f64::floor(255.999 * r) as i32;
  let gbyte = f64::floor(255.999 * g) as i32;
  let bbyte = f64::floor(255.999 * b) as i32;

  write!(w, "{rbyte} {gbyte} {bbyte}\n").expect("unable to write pixel color");
}
