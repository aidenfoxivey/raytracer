use crate::hit::{HitRecord, Hittable};
use std::rc::Rc;

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

    pub fn hit(ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let temp_rec = 
    }
}

impl<T> Default for HittableList<T>
where
    T: Hittable,
{
    fn default() -> Self {
        HittableList { objects: vec![] }
    }
}
