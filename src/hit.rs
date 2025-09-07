use crate::ray::{Point, Ray};
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Point,
    pub norm: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
