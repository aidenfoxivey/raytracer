use std::fs::{self, remove_file};
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use crate::colour::{ray_colour, write_colour};
use crate::hittable_list::HittableList;
use crate::ray::{Point, Ray};
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod colour;
mod hit;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() -> std::io::Result<()> {
    let path = Path::new("example.ppm");
    if fs::metadata(path).is_ok() {
        println!("Removing old path: {:#?}", path);
        remove_file(path)?;
    }

    let mut s = Vec::new();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height and ensure a height of minimum 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0., -100.5, -1.), 100.)));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_centre = Point::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    writeln!(&mut s, "P3")?;
    writeln!(&mut s, "{} {}", image_width, image_height)?;
    writeln!(&mut s, "255")?;

    for j in 0..image_height {
        print!(".");
        for i in 0..image_width {
            let pixel_centre =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_centre - camera_centre;
            let r = Ray::new(camera_centre, ray_direction);

            let pixel_colour = ray_colour(&r, &world);

            write_colour(&mut s, &pixel_colour)?;
        }
    }

    println!("\nDone.");

    let mut f = fs::File::create(path)?;

    f.write_all(&s)?;

    Ok(())
}
