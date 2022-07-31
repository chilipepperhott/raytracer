use std::ops::Range;

use ultraviolet::DVec3;

use crate::{material::Material, random::Random, ray::Ray};

/// Result of a ray intersection test.
///
/// Provides where ray hit, the objects normal at that location, as well as the distance along the
/// ray.
#[derive(Copy, Clone, Debug)]
pub struct Hit {
    pub point: DVec3,
    pub normal: DVec3,
    pub distance: f64,
    pub material: Material,
}

pub trait Object: Send + Sync {
    /// Calculate ray intersection with Object.
    fn hit(&self, ray: Ray, tolerance: Range<f64>) -> Option<Hit>;
}

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: Ray, tolerance: Range<f64>) -> Option<Hit> {
        let relative_to_sphere = ray.origin - self.center;

        let a = ray.direction.mag_sq();
        let half_b = relative_to_sphere.dot(ray.direction);
        let c = relative_to_sphere.mag_sq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        let sqrtd = discriminant.sqrt();

        if discriminant < 0.0 {
            return None;
        }

        let root = (-half_b - sqrtd) / a;
        if !tolerance.contains(&root) {
            let root = (-half_b + sqrtd) / a;

            if !tolerance.contains(&root) {
                return None;
            }
        }

        let distance = (-half_b - discriminant.sqrt()) / a;

        let point = ray.at(distance);
        let normal = (point - self.center).normalized();

        Some(Hit {
            point,
            normal,
            distance,
            material: self.material,
        })
    }
}
