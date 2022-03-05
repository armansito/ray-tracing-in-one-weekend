use {
    anyhow::{Context, Result},
    image::RgbImage,
    std::io::{self, Write},
};

mod algebra;
mod color;
mod objects;

use crate::{
    algebra::{Point3, Ray, Vec3},
    color::RgbFloat,
    objects::{Hittable, Sphere},
};

type Scene = Vec<Box<dyn Hittable>>;

fn ray_color(ray: &Ray, scene: &Scene) -> RgbFloat {
    if let Some(hit) = scene.hit(ray) {
        return RgbFloat(0.5 * (hit.normal + 1.0));
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

    // Scene
    let mut scene: Scene = Vec::new();
    scene.push(Box::new(Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 }));
    scene.push(Box::new(Sphere { center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 }));

    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Point3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    for row in (0..HEIGHT).rev() {
        print!("\rScanlines remaining: {}", row);
        io::stdout().flush()?;

        for col in 0..WIDTH {
            let u = (col as f32) / (WIDTH as f32 - 1.0);
            let v = (row as f32) / (HEIGHT as f32 - 1.0);
            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let color = ray_color(&ray, &scene);
            img.put_pixel(col, HEIGHT - row - 1, color.into());
        }
    }

    println!("\nDone");

    img.save("image.ppm").context("failed to write PPM image")
}
