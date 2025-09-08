use std::io::Write;

use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub type Colour = Vec3;

/// Blend a value via linear interpolation via the y coordinate.
pub fn ray_colour<T>(r: &Ray, world: &T) -> Colour
where
    T: Hittable,
{
    let mut rec = HitRecord::default();

    if world.hit(r, (0.0, f64::INFINITY), &mut rec) {
        return (rec.norm + Colour::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.1 + 1.0);

    Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a
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
