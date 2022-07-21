// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use crate::{
    algebra::Point3,
    camera::{Camera, CameraParams},
    color::RgbFloat,
    material::{Dielectric, Lambertian, Metal},
    random::Rng,
    scene::{Scene, Sphere},
};

/// The scene from the middle chapters with 3 spheres.
pub fn simple_scene(aspect_ratio: f32) -> (Scene, Camera) {
    let mut scene: Scene = Vec::new();

    // Ground
    scene.push(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Lambertian::new(RgbFloat::new(0.8, 0.8, 0.0)),
    }));

    // Middle sphere
    scene.push(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Lambertian::new(RgbFloat::new(0.1, 0.2, 0.5)),
    }));

    // Left glass sphere, outer surface
    let dielectric = Dielectric::new(1.5);
    scene.push(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: dielectric.clone(),
    }));
    // Left glass sphere, inner surface.
    scene.push(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: dielectric,
    }));

    // Right metal sphere.
    scene.push(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Metal::new(RgbFloat::new(0.8, 0.6, 0.2), 0.0),
    }));

    // Camera
    let origin = Point3::new(-3.0, 3.0, 2.0);
    let focus = Point3::new(0.0, 0.0, -1.0);
    (
        scene,
        Camera::new(CameraParams {
            origin,
            look_at: focus,
            up: Point3::new(0.0, 1.0, 0.0),
            aspect_ratio,
            fov_y: 30_f32.to_radians(),
            aperture: 0.2,
            focus_distance: (focus - origin).length(),
        }),
    )
}

/// The final scene which is the cover of the book.
pub fn cover_scene(rng: &Rng, aspect_ratio: f32) -> (Scene, Camera) {
    let mut scene: Scene = Vec::new();

    // Ground
    scene.push(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, -1.0),
        radius: 1000.0,
        material: Lambertian::new(RgbFloat::gray(0.5)),
    }));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_material = rng.random_float();
            let center = Point3::new(
                (a as f32) + 0.9 * rng.random_float(),
                0.2,
                (b as f32) + 0.9 * rng.random_float(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_material < 0.8 {
                    let albedo = RgbFloat::random(&rng) * RgbFloat::random(&rng);
                    Lambertian::new(albedo)
                } else if choose_material < 0.95 {
                    let albedo = RgbFloat::random_in_range(0.5, 1.0);
                    let fuzz = rng.random_float();
                    Metal::new(albedo, fuzz)
                } else {
                    Dielectric::new(1.5)
                };
                scene.push(Box::new(Sphere { center, radius: 0.2, material }));
            }
        }

        scene.push(Box::new(Sphere {
            center: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Dielectric::new(1.5),
        }));

        scene.push(Box::new(Sphere {
            center: Point3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Lambertian::new(RgbFloat::new(0.4, 0.2, 0.1)),
        }));

        scene.push(Box::new(Sphere {
            center: Point3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Metal::new(RgbFloat::new(0.7, 0.6, 0.5), 0.0),
        }));
    }

    (
        scene,
        Camera::new(CameraParams {
            origin: Point3::new(13.0, 2.0, 3.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            up: Point3::new(0.0, 1.0, 0.0),
            aspect_ratio,
            fov_y: 20_f32.to_radians(),
            aperture: 0.1,
            focus_distance: 10.0,
        }),
    )
}
