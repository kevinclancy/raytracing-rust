use std::io::Write;

use crate::{color::{color, write_color, Color}, hit_record::Hittable, interval::interval, ray::Ray, vec3::{point3, unit_vector, vec3, Point3, Vec3}};

pub struct Camera {
  aspect_ratio: f64,
  image_width: i32,
  image_height: i32,
  center: Point3,
  pixel00_loc: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3
}

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
  match world.hit(r, interval(0., f64::INFINITY)) {
    Some(hit_record) => {
      0.5 * (&hit_record.normal + color(1., 1., 1.))
    }
    None => {
      let unit_direction = unit_vector(&r.direction);
      let a = 0.5 * (&unit_direction.y + 1.);
      return (1. - a) * color(1., 1., 1.) + a * color(0.5, 0.7, 1.)
    }
  }
}

impl Camera {
  /// Ratio of image width over height
  pub fn aspect_ratio(&self) -> f64 {
    self.aspect_ratio
  }

  pub fn image_width(&self) -> i32 {
    self.image_width
  }

  pub fn render(&self, world: &dyn Hittable) {
    let mut stdout = std::io::stdout();
    writeln!(&mut stdout, "P3\n{} {}\n255\n", self.image_width, self.image_height).expect("write should succeed");
    for j in 0..self.image_height {
      eprintln!("Scanlines remaining: {}", self.image_height-j);
      for i in 0..self.image_width {
        let pixel_center =
          &self.pixel00_loc +
          ((i as f64) * &self.pixel_delta_u) +
          ((j as f64) * &self.pixel_delta_v);

        let ray_direction = &pixel_center - &self.center;
        let r = Ray{origin: self.center.clone(), direction: ray_direction.clone()};

        let pixel_color = ray_color(&r, world);
        write_color(&mut stdout, &pixel_color);
      }
    }
  }

  pub fn new(aspect_ratio: f64, image_width: i32) -> Camera {
    let image_height = {
      let image_height_candidate = ((image_width as f64) / aspect_ratio) as i32;
      if image_height_candidate < 1 { 1 } else { image_height_candidate }
    };
    let center = point3(0., 0., 0.);

    // determine viewport dimensions
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));

    //calculuate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = vec3(viewport_width, 0., 0.);
    let viewport_v = vec3(0., -viewport_height, 0.);

    let pixel_delta_u = &viewport_u / (image_width as f64);
    let pixel_delta_v = &viewport_v / (image_height as f64);

    let viewport_upper_left =
      &center - vec3(0., 0., focal_length) - (&viewport_u/2.) - (&viewport_v/2.);

    let pixel00_loc = &viewport_upper_left + (0.5 * (&pixel_delta_u + &pixel_delta_v));

    Camera{
      aspect_ratio,
      image_width,
      image_height,
      center,
      pixel00_loc,
      pixel_delta_u,
      pixel_delta_v
    }
  }
}

