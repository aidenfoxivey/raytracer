use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList<T>
where
    T: Hittable,
{
    pub objects: Vec<Rc<T>>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    pub fn new(object: Rc<T>) -> Self {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Rc<T>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in self.objects.iter() {
            if obj.hit(
                ray,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
