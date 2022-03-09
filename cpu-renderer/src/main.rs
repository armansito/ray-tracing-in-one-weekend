use {
    anyhow::{Context, Result},
    image::RgbImage,
    std::io::{self, Write},
};

mod algebra;
mod camera;
mod color;
mod objects;
mod random;

use crate::{
    algebra::{Point3, Ray},
    camera::Camera,
    color::RgbFloat,
    objects::{Hittable, Sphere},
    random::Rng,
};

type Scene = Vec<Box<dyn Hittable>>;

fn ray_color(ray: &Ray, scene: &Scene, rng: &mut Rng, depth: u32) -> RgbFloat {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return RgbFloat::black();
    }

    if let Some(hit) = scene.hit(ray) {
        let ray = Ray { origin: hit.point, direction: rng.sample_hemisphere(&hit.normal) };
        return 0.5 * ray_color(&ray, scene, rng, depth - 1);
    }

    // Paint a blue-white gradient background if no objects were intersected.
    let dir = ray.direction.normalized();
    let t = 0.5 * (dir.y() + 1.0);
    RgbFloat((1.0 - t) * RgbFloat::new(1.0, 1.0, 1.0).0 + t * RgbFloat::new(0.5, 0.7, 1.0).0)
}

fn main() -> Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const HEIGHT: u32 = 600;
    const WIDTH: u32 = ((HEIGHT as f32) * ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // Scene
    let mut scene: Scene = Vec::new();
    scene.push(Box::new(Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 }));
    scene.push(Box::new(Sphere { center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 }));

    // Camera
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), ASPECT_RATIO, 1.0);

    // Render
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    let mut rng = Rng::new();

    for row in (0..HEIGHT).rev() {
        print!("\rScanlines remaining: {}", row);
        io::stdout().flush()?;
        for col in 0..WIDTH {
            let mut pixel_color = RgbFloat::black();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((col as f32) + rng.random_float()) / (WIDTH as f32 - 1.0);
                let v = ((row as f32) + rng.random_float()) / (HEIGHT as f32 - 1.0);
                let ray = camera.ray(u, v);
                pixel_color += ray_color(&ray, &scene, &mut rng, MAX_DEPTH);
            }

            // Divide the color by the number of samples and gamma-correct for gamma=2.0.
            pixel_color /= SAMPLES_PER_PIXEL as f32;
            let pixel_color = RgbFloat::new(
                pixel_color.r().sqrt(),
                pixel_color.g().sqrt(),
                pixel_color.b().sqrt(),
            );

            img.put_pixel(col, HEIGHT - row - 1, pixel_color.into());
        }
    }

    println!("\nDone");

    img.save("image.ppm").context("failed to write PPM image")
}
