pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x <= self.max && x >= self.min
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x < self.max && x > self.min
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}
