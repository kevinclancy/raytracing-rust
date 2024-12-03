use core::f64;

pub struct Interval {
  pub min: f64,
  pub max: f64
}

pub fn interval(min: f64, max: f64) -> Interval {
  Interval{min,max}
}

impl Interval {
  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }

  pub const EMPTY : Interval = Interval{min: f64::INFINITY, max: -f64::INFINITY};
  pub const UNIVERSE : Interval = Interval{min: -f64::INFINITY, max: f64::INFINITY};
}