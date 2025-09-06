use std::io::Write;

use crate::ray::{Point, Ray};
use crate::vec3::Vec3;

pub type Colour = Vec3;

/// Solve the quadratic formula!
fn hit_sphere(centre: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = *centre - r.origin;
    let a = r.direction.dot(&r.direction);
    let h = r.direction.dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

/// Blend a value via linear interpolation via the y coordinate.
pub fn ray_colour(r: &Ray) -> Colour {
    let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        return Colour::new(n.0 + 1.0, n.1 + 1.0, n.2 + 1.0) * 0.5;
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
