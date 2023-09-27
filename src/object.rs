use std::ops::Range;

use ultraviolet::DVec3;

use crate::{material::Material, random::Random, ray::Ray};

/// Result of a ray intersection test.
///
/// Provides where ray hit, the objects normal at that location, as well as the distance along the
/// ray.
#[derive(Copy, Clone, Debug)]
pub struct Hit {
    /// The point in space where the ray hit.
    pub point: DVec3,
    /// The normal of the object, relative to the hit.
    pub normal: DVec3,
    /// The distance along the ray where it hit.
    pub distance: f64,
    /// Whether the surface normal faces the ray.
    pub faces_ray: bool,
    /// The material of the hit object.
    pub material: Material,
}

pub trait Object: Send + Sync {
    /// Calculate ray intersection with Object.
    fn hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit>;
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
    fn hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
        let relative_to_sphere = ray.origin - self.center;

        let a = ray.direction.mag_sq();
        let half_b = relative_to_sphere.dot(ray.direction);
        let c = relative_to_sphere.mag_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        let sqrtd = discriminant.sqrt();

        if discriminant < 0.0 {
            return None;
        }

        let mut distance = (-half_b - sqrtd) / a;
        if distance < min_dist || distance > max_dist {
            distance = (-half_b + sqrtd) / a;
            if distance < min_dist || distance > max_dist {
                return None;
            }
        }

        let point = ray.at(distance);
        let normal = (point - self.center) / self.radius;
        let faces_ray = ray.direction.dot(normal) < 0.0;

        Some(Hit {
            point,
            normal,
            distance,
            faces_ray,
            material: self.material,
        })
    }
}
