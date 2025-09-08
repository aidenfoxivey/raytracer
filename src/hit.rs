use crate::ray::{Point, Ray};
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Point,
    pub norm: Vec3,
    pub t: f64,
    pub front_face: bool, // did the ray hit the front face?
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: &Vec3) {
        // Hit record's normal vector.
        // NOTE: Assume `outward` normal has unit length.

        self.front_face = r.direction.dot(outward_norm) < 0.0;

        if self.front_face {
            self.norm = *outward_norm;
        } else {
            // -* is quite a funny concept
            self.norm = -*outward_norm;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
