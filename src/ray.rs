use crate::vec3::Vec3;

pub type Point = Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_at() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(1.0, 1.0, 1.0);

        let at = Ray::new(origin, direction).at(3.0);
        assert_eq!(at, Point::new(3.0, 3.0, 3.0));
    }
}
