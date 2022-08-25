// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use {image::RgbImage, rayon::prelude::*};

use crate::{
    algebra::Ray,
    camera::Camera,
    color::RgbFloat,
    random::Rng,
    scene::{Hittable, Scene},
};

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

pub fn render_scene(
    scene: &Scene,
    camera: &Camera,
    rng: &Rng,
    depth: u32,
    samples_per_pixel: u32,
    img: Arc<Mutex<RgbImage>>,
) {
    let (width, height, img_size) = {
        let img = img.lock().unwrap();
        (img.width(), img.height(), img.width() * img.height())
    };
    (0..img_size).into_par_iter().for_each(|index| {
        let row = index / width;
        let col = index % width;

        let mut pixel_color = RgbFloat::black();
        for _ in 0..samples_per_pixel {
            let u = ((col as f32) + rng.random_float()) / (width as f32 - 1.0);
            let v = 1.0 - ((row as f32) + rng.random_float()) / (height as f32 - 1.0);
            let ray = camera.ray(u, v);

            // Add 1 for at least one recursion for the primary rays.
            pixel_color += ray_color(&ray, &scene, &rng, depth);
        }

        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        pixel_color /= samples_per_pixel as f32;
        let pixel_color = RgbFloat::new(
            pixel_color.r().sqrt(),
            pixel_color.g().sqrt(),
            pixel_color.b().sqrt(),
        );

        let img = img.clone();
        img.lock().unwrap().put_pixel(col, row, pixel_color.into());
    });
}

fn ray_color(ray: &Ray, scene: &Scene, rng: &Rng, depth: u32) -> RgbFloat {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return RgbFloat::black();
    }

    if let Some(hit) = scene.hit(ray) {
        return match hit.material.scatter(ray, &hit) {
            None => RgbFloat::black(),
            Some((attenuation, ray)) => attenuation * ray_color(&ray, scene, rng, depth - 1),
        };
    }

    // Paint a blue-white gradient background if no objects were intersected.
    let dir = ray.direction.normalized();
    let t = 0.5 * (dir.y() + 1.0);
    RgbFloat((1.0 - t) * RgbFloat::new(1.0, 1.0, 1.0).0 + t * RgbFloat::new(0.5, 0.7, 1.0).0)
}
