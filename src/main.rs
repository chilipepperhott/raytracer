mod material;
mod object;
mod random;
mod ray;
mod world;

use image::{DynamicImage, RgbImage};
use material::Material;
use object::Object;
use random::Random;
use ray::Ray;
use rayon::prelude::*;
use ultraviolet::DVec3;
use world::World;

fn main() {
    let width = 500_usize;
    let height = 500_usize;
    let samples = 40_usize;
    let max_bounces = 10_usize;

    let aspect_ratio: f64 = (width as f64) / (height as f64);

    let virtual_height = 2.0;
    let virtual_width = aspect_ratio * virtual_height;
    let focal_length = 1.0;

    let view_x = DVec3::unit_x() * virtual_width;
    let view_y = DVec3::unit_y() * virtual_height;
    let view_z = DVec3::unit_z() * focal_length;
    let origin = DVec3::unit_y() * 0.0;
    let top_left_corner = origin - view_x / 2.0 + view_y / 2.0 + view_z;

    let world = World::empty()
        .with_sphere(
            DVec3::new(0.0, 0.0, 1.0),
            0.5,
            Material::new_lambertian(DVec3::unit_x() * 0.5),
        )
        .with_sphere(
            DVec3::new(0.0, -100.5, 1.0),
            100.0,
            Material::new_lambertian(DVec3::one()),
        )
        .with_sphere(
            DVec3::new(1.0, 0.0, 1.0),
            0.5,
            Material::new_metal(DVec3::unit_z(), 0.3),
        );

    let mut lines: Vec<Vec<u8>> = (0..height)
        .into_par_iter()
        .map(|y| {
            let mut line = Vec::with_capacity(width);
            let mut rng = Random::new(y.overflowing_add(width).0 as u64);

            (0..width).for_each(|x| {
                let pixel = (0..samples)
                    .map(|_sample| {
                        let u = (x as f64 + rng.gen_f64(0.0..1.0)) / width as f64;
                        let v = (y as f64 + rng.gen_f64(0.0..1.0)) / height as f64;

                        sample_ray(
                            Ray::new(origin, top_left_corner + u * view_x - v * view_y + origin),
                            &world,
                            &mut rng,
                            max_bounces,
                        )
                    })
                    .sum::<DVec3>()
                    / samples as f64;

                line.push((pixel.x * 256.0) as u8);
                line.push((pixel.y * 256.0) as u8);
                line.push((pixel.z * 256.0) as u8);
            });

            line
        })
        .collect();

    let mut image = Vec::with_capacity(width * height * 3);

    for line in &mut lines {
        image.append(line);
    }

    let image =
        DynamicImage::ImageRgb8(RgbImage::from_raw(width as u32, height as u32, image).unwrap());

    viuer::print(&image, &viuer::Config::default()).unwrap();

    image.save("render.png").unwrap();
}

fn sample_ray(ray: Ray, object: &dyn Object, rng: &mut Random, max_bounces: usize) -> DVec3 {
    let mut bounce_history = Vec::with_capacity(max_bounces);
    let mut current_ray = ray;

    for _bounce in 0..max_bounces {
        if let Some(hit) = current_ray.hits(object, 0.00001..f64::INFINITY) {
            // Replace this with the object's color
            bounce_history.push(hit.material);

            current_ray = hit
                .material
                .scatter(ray.direction, hit, rng);
        } else {
            let mut final_color = sky(ray.direction);

            while let Some(material) = bounce_history.pop() {
                final_color = material.attenuate(final_color);
            }

            return final_color;
        };
    }

    DVec3::zero()
}

fn sky(direction: DVec3) -> DVec3 {
    let t = 0.5 * (direction.normalized().y + 1.0);
    (1.0 - t) * DVec3::one() + t * DVec3::new(0.5, 0.7, 1.0)
}
