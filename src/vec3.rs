use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::utils::{random_double, random_double_range};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self::new(
            random_double(&mut rng),
            random_double(&mut rng),
            random_double(&mut rng),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::rng();
        Self::new(
            random_double_range(&mut rng, min, max),
            random_double_range(&mut rng, min, max),
            random_double_range(&mut rng, min, max),
        )
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1., 1.);
            let lensq = p.length_squared();
            // Avoid blackholding by underflow
            if 1e-160 < lensq && lensq <= 1. {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {} {}", self.0, self.1, self.0)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, x: f64) -> Self {
        Vec3(self.0 * x, self.1 * x, self.2 * x)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, x: f64) -> Self {
        self * (1.0 / x)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, x: f64) {
        *self *= 1.0 / x;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, x: f64) {
        *self = Self(self.0 * x, self.1 * x, self.2 * x)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(v1.dot(&v2), 3.0);
    }
}
