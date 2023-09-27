use std::ops::Range;

use ultraviolet::DVec3;

use crate::object::{Hit, Object};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, length: f64) -> DVec3 {
        self.origin + self.direction * length
    }

    pub fn hits(&self, object: &dyn Object, min_dist: f64, max_dist: f64) -> Option<Hit> {
        object.hit(*self, min_dist, max_dist)
    }
}
