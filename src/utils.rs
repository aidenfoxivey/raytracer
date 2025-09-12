use rand::prelude::*;
use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double(rng: &mut ThreadRng) -> f64 {
    rng.random_range(0.0..1.0)
}

pub fn random_double_range(rng: &mut ThreadRng, min: f64, max: f64) -> f64 {
    min * (max - min) * random_double(rng)
}
