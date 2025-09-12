use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::ray::Point;
use crate::sphere::Sphere;

mod camera;
mod hit;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0., -100.5, -1.), 100.)));

    let cam = Camera::new(16.0 / 9.0, 400, 100, 50);
    cam.render(&world)?;

    Ok(())
}
