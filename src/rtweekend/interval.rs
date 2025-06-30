#[derive(Copy, Clone)]
pub(crate) struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    //in Default should be [INFINITY,NEG_INFINITY]

    pub(crate) fn default() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    pub(crate) fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
    pub(crate) fn new_merge(a: &Interval, b: &Interval) -> Self {
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    pub(crate) fn size(&self) -> f64 {
        self.max - self.min
    }

    pub(crate) fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub(crate) fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub(crate) const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    pub(crate) const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}
