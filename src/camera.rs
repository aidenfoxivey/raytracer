use std::fs::{self, remove_file};
use std::io::Write;
use std::path::Path;

use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::{Point, Ray};
use crate::vec3::Vec3;

pub struct Camera {
    /// Ratio of image width vs height
    pub aspect_ratio: f64,
    /// Image width
    pub image_width: i32,
    /// Image height
    image_height: i32,
    /// Centre of camera
    centre: Point,
    /// Location of pixel 0, 0
    pixel00_loc: Point,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
}

pub type Colour = Vec3;

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        let centre = Point::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_width, 0.);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            centre - Vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn write_colour(v: &mut Vec<u8>, colour: &Colour) -> std::io::Result<()> {
        // I'm a survivor.
        //
        // We're a dying breed...
        //
        // - Xavier
        let r = colour.0;
        let g = colour.1;
        let b = colour.2;

        let rbyte = (255.999 * r) as i32;
        let gbyte = (255.999 * g) as i32;
        let bbyte = (255.999 * b) as i32;

        writeln!(v, "{} {} {}", rbyte, gbyte, bbyte)
    }

    /// Blend a value via linear interpolation via the y coordinate.
    pub fn ray_colour<T>(r: &Ray, world: &T) -> Colour
    where
        T: Hittable,
    {
        let mut rec = HitRecord::default();

        if world.hit(
            r,
            Interval {
                min: 0.0,
                max: f64::INFINITY,
            },
            &mut rec,
        ) {
            return (rec.norm + Colour::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = r.direction.unit();
        let a = 0.5 * (unit_direction.1 + 1.0);

        Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a
    }

    pub fn render<T>(&self, world: &T) -> std::io::Result<()>
    where
        T: Hittable,
    {
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

                let pixel_colour = Camera::ray_colour(&r, world);

                Camera::write_colour(&mut s, &pixel_colour)?;
            }
        }

        println!("\nDone.");

        let mut f = fs::File::create(path)?;

        f.write_all(&s)?;

        Ok(())
    }
}
