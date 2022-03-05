use {
    anyhow::{Context, Result},
    image::RgbImage,
    std::io::{self, Write},
};

mod algebra;
mod color;

use crate::{
    algebra::{Point3, Vec3, Ray},
    color::RgbFloat,
};

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> RgbFloat {
    // Intersect a unit sphere that is unit length away from the origin along the -Z axis.
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return RgbFloat::new(1.0, 0.0, 0.0);
    }

    let dir = ray.direction.normalized();
    let t = 0.5 * (dir.y() + 1.0);
    RgbFloat(
        (1.0 - t) * RgbFloat::new(1.0, 1.0, 1.0).0 +
        t * RgbFloat::new(0.5, 0.7, 1.0).0
    )
}

fn main() -> Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const HEIGHT: u32 = 400;
    const WIDTH: u32 = ((HEIGHT as f32) * ASPECT_RATIO) as u32;

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
            let color = ray_color(&ray);
            img.put_pixel(col, HEIGHT - row - 1, color.into());
        }
    }

    println!("\nDone");

    img.save("image.ppm").context("failed to write PPM image")
}
