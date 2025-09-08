use crate::{Point, hit::HitRecord, hit::Hittable, ray::Ray};

#[derive(Default)]
pub struct Sphere {
    centre: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point, radius: f64) -> Self {
        Sphere { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: (f64, f64), rec: &mut HitRecord) -> bool {
        let oc = self.centre - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_t.0 || ray_t.1 <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.0 || ray_t.1 <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_norm = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(r, &outward_norm);

        true
    }
}
