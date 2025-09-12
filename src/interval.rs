pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    const fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    fn size(&self) -> f64 {
        self.max - self.min
    }

    fn contains(&self, x: f64) -> bool {
        x <= self.max && x >= self.min
    }

    fn surrounds(&self, x: f64) -> bool {
        x < self.max && x > self.min
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}
