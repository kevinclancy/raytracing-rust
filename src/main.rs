use raytracer::camera::Camera;
use raytracer::hittable_list::HittableList;
use raytracer::vec3::*;
use raytracer::sphere::sphere;

fn main() {
  let mut world = HittableList::new();
  world.add(Box::new(sphere(point3(0., 0., -1.), 0.5)));
  world.add(Box::new(sphere(point3(0., -100.5, -1.), 100.)));

  let camera = Camera::new(16.0 / 9.0, 400);

  camera.render(&world);
}

