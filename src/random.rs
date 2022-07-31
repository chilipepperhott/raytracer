use std::ops::Range;

use ultraviolet::DVec3;

pub struct Random {
    state: u64,
}

impl Random {
    pub fn new(initial_state: u64) -> Self {
        Self {
            state: initial_state,
        }
    }

    /// Generate a random u64
    pub fn gen_u64(&mut self) -> u64 {
        let mut x = self.state;
        x = x ^ (x << 13);
        x = x ^ (x >> 7);
        x = x ^ (x << 17);
        self.state = x;
        x
    }

    /// Generate a random f64 value between a specified range.
    pub fn gen_f64(&mut self, range: Range<f64>) -> f64 {
        let x = self.gen_u64();
        x as f64 / u64::MAX as f64 * (range.end - range.start) + range.start
    }

    /// Generate a random point within a unit sphere.
    pub fn gen_in_sphere(&mut self) -> DVec3 {
        loop {
            let point = DVec3::new(
                self.gen_f64(-1.0..1.0),
                self.gen_f64(-1.0..1.0),
                self.gen_f64(-1.0..1.0),
            );

            if point.mag() < 1.0 {
                break point;
            }
        }
    }

    /// Generate a random point within a unit hemisphere.
    ///
    /// The given `normal` will point out the top of the hemisphere
    pub fn gen_in_hemisphere(&mut self, normal: DVec3) -> DVec3 {
        let in_sphere = self.gen_in_sphere();

        if in_sphere.dot(normal) > 0.0 {
            in_sphere
        } else {
            -in_sphere
        }
    }
}
