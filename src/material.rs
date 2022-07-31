use ultraviolet::DVec3;

use crate::{random::Random, ray::Ray, object::Hit};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { color: DVec3 },
    Metal { color: DVec3, fuzz: f64 },
}

impl Material {
    /// Create a new diffuse Lambertian material.
    pub fn new_lambertian(color: DVec3) -> Self {
        Self::Lambertian { color }
    }

    /// Create a new reflective metal material.
    pub fn new_metal(color: DVec3, fuzz: f64) -> Self{
        Self::Metal { color, fuzz }
    }

    /// Scatter a ray hit.
    pub fn scatter(
        &self,
        previous_direction: DVec3,
        hit: Hit,
        rng: &mut Random,
    ) -> Ray {
        match self {
            Self::Lambertian { .. } => {
                Ray::new(hit.point, (hit.normal + rng.gen_in_hemisphere(hit.normal)).normalized())
            }
            Self::Metal { fuzz, .. } => Ray::new(hit.point, previous_direction.reflected(hit.normal) + *fuzz * rng.gen_in_sphere()),
        }
    }

    /// Subtract the material's color.
    pub fn attenuate(&self, color: DVec3) -> DVec3 {
        match self {
            Material::Lambertian { color: filter } => color * *filter,
            Material::Metal { color: filter, .. } => color * *filter,
        }
    }
}
