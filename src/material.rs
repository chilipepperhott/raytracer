use ultraviolet::DVec3;

use crate::{object::Hit, random::Random, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { color: DVec3 },
    Metal { color: DVec3, fuzz: f64 },
    Dielectric { ior: f64 },
}

impl Material {
    /// Create a new diffuse Lambertian material.
    pub fn new_lambertian(color: DVec3) -> Self {
        Self::Lambertian { color }
    }

    /// Create a new reflective metal material.
    pub fn new_metal(color: DVec3, fuzz: f64) -> Self {
        Self::Metal { color, fuzz }
    }

    /// Create a new refractive dielectric material.
    pub fn new_dielectric(ior: f64) -> Self {
        Self::Dielectric { ior }
    }

    /// Scatter a ray hit.
    pub fn scatter(&self, previous_direction: DVec3, hit: Hit, rng: &mut Random) -> Ray {
        match self {
            Self::Lambertian { .. } => Ray::new(
                hit.point,
                (hit.normal + rng.gen_in_hemisphere(hit.normal)).normalized(),
            ),
            Self::Metal { fuzz, .. } => Ray::new(
                hit.point,
                (previous_direction.reflected(hit.normal) + *fuzz * rng.gen_in_sphere())
                    .normalized(),
            ),
            Self::Dielectric { ior } => {
                let (normal, eta) = if hit.faces_ray {
                    (hit.normal, 1.0 / *ior)
                } else {
                    (-hit.normal, *ior)
                };

                let cos_theta = (-previous_direction).dot(normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = eta * sin_theta > 1.0;

                // Compute Shlick's approximation
                let r0 = (1.0 - eta) / (1.0 + eta);
                let r0 = r0 * r0;
                let r1 = 1.0 - cos_theta;
                let reflectance = r0 + (1.0 - r0) * r1 * r1 * r1 * r1 * r1;

                let new_direction = if cannot_refract || reflectance > rng.gen_f64(0.0..1.0) {
                    previous_direction.reflected(normal).normalized()
                } else {
                    previous_direction.refracted(normal, eta).normalized()
                };

                Ray::new(hit.point, new_direction)
            }
        }
    }

    /// Subtract the material's color.
    pub fn attenuate(&self, color: DVec3) -> DVec3 {
        match self {
            Material::Lambertian { color: filter } => color * *filter,
            Material::Metal { color: filter, .. } => color * *filter,
            Self::Dielectric { .. } => color,
        }
    }
}
