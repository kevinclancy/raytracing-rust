use std::ops::*;

#[derive(Clone)]
pub struct Vec3{
  pub x:f64,
  pub y:f64,
  pub z:f64
}

pub type Point3 = Vec3;

impl Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Vec3 {
    Vec3{x : -self.x, y : -self.y, z : -self.z}
  }
}

impl Neg for &Vec3 {
  type Output = Vec3;

  fn neg(self) -> Vec3 {
    Vec3{x : -self.x, y : -self.y, z : -self.z}
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, s: f64) -> () {
    self.x *= s;
    self.y *= s;
    self.z *= s;
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, s: f64) -> () {
    self.x /= s;
    self.y /= s;
    self.z /= s;
  }
}

impl AddAssign<&Vec3> for Vec3 {
  fn add_assign(&mut self, other: &Vec3) -> () {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3{
      x : self.x + other.x,
      y : self.y + other.y,
      z : self.z + other.z
    }
  }
}

impl Add<&Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, other: &Vec3) -> Vec3 {
    Vec3{
      x : self.x + other.x,
      y : self.y + other.y,
      z : self.z + other.z
    }
  }
}

impl Add<Vec3> for &Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3{
      x : self.x + other.x,
      y : self.y + other.y,
      z : self.z + other.z
    }
  }
}

impl Add<&Vec3> for &Vec3 {
  type Output = Vec3;

  fn add(self, other: &Vec3) -> Vec3 {
    Vec3{
      x : self.x + other.x,
      y : self.y + other.y,
      z : self.z + other.z
    }
  }
}

impl Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, other: Vec3) -> Vec3 {
    Vec3{
      x : self.x - other.x,
      y : self.y - other.y,
      z : self.z - other.z
    }
  }
}

impl Sub<Vec3> for &Vec3 {
  type Output = Vec3;

  fn sub(self, other: Vec3) -> Vec3 {
    Vec3{
      x : self.x - other.x,
      y : self.y - other.y,
      z : self.z - other.z
    }
  }
}

impl Sub<&Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, other: &Vec3) -> Vec3 {
    Vec3{
      x : self.x - other.x,
      y : self.y - other.y,
      z : self.z - other.z
    }
  }
}

impl Sub<&Vec3> for &Vec3 {
  type Output = Vec3;

  fn sub(self, other: &Vec3) -> Vec3 {
    Vec3{
      x : self.x - other.x,
      y : self.y - other.y,
      z : self.z - other.z
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, t: f64) -> Vec3 {
    t * self
  }
}

impl Mul<f64> for &Vec3 {
  type Output = Vec3;

  fn mul(self, t: f64) -> Vec3 {
    t * self
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, v: Vec3) -> Vec3 {
    Vec3{
      x: self * v.x,
      y: self * v.y,
      z: self * v.z
    }
  }
}

impl Mul<&Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, v: &Vec3) -> Vec3 {
    Vec3{
      x: self * v.x,
      y: self * v.y,
      z: self * v.z
    }
  }
}

impl Mul<&Vec3> for &Vec3 {
  type Output = Vec3;

  fn mul(self, other: &Vec3) -> Vec3 {
    Vec3{
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z
    }
  }
}

impl Div<f64> for &Vec3 {
  type Output = Vec3;

  fn div(self, t: f64) -> Vec3 {
    Vec3{
      x: self.x/t,
      y: self.y/t,
      z: self.z/t
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, t: f64) -> Vec3 {
    Vec3{
      x: self.x/t,
      y: self.y/t,
      z: self.z/t
    }
  }
}

impl Vec3 {
  pub fn length_squared(&self) -> f64 {
    (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
  }

  pub fn length(&self) -> f64 {
    f64::sqrt(self.length_squared())
  }

  pub fn origin() -> Self {
    Vec3{x : 0.0, y : 0.0, z : 0.0}
  }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
  u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
  Vec3{
    x: u.y * v.z - u.z * v.y,
    y: u.z * v.x - u.x * v.z,
    z: u.x * v.y - u.y * v.x
  }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
  v / v.length()
}

pub fn vec3(x: f64, y: f64, z:f64) -> Vec3 {
  Vec3{x,y,z}
}

pub fn point3(x: f64, y: f64, z:f64) -> Point3 {
  Point3{x,y,z}
}