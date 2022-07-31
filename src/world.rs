use std::ops::Range;

use ultraviolet::DVec3;

use crate::{
    material::{self, Material},
    object::{Hit, Object, Sphere},
    ray::Ray,
};

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn with_object(mut self, object: Box<dyn Object>) -> Self {
        self.add_object(object);
        self
    }

    pub fn add_sphere(&mut self, center: DVec3, radius: f64, material: Material) {
        self.add_object(Box::new(Sphere::new(center, radius, material)));
    }

    pub fn with_sphere(mut self, center: DVec3, radius: f64, material: Material) -> Self {
        self.add_sphere(center, radius, material);
        self
    }
}

impl Object for World {
    fn hit(&self, ray: Ray, tolerance: Range<f64>) -> Option<Hit> {
        let mut closest_hit = None;

        // Probably a more expressive way of writing this.
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, tolerance.clone()) {
                if let Some(Hit {
                    distance: closest_distance,
                    ..
                }) = closest_hit
                {
                    if closest_distance > hit.distance {
                        closest_hit = Some(hit);
                    }
                } else {
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }
}
