#[macro_use]
extern crate impl_ops;

use {
    anyhow::{anyhow, Context, Result},
    argh::FromArgs,
    image::RgbImage,
    std::{
        io::{self, Write},
        str::FromStr,
    },
};

mod algebra;
mod camera;
mod color;
mod material;
mod random;
mod scene;

use crate::{
    algebra::{Point3, Ray},
    camera::{Camera, CameraParams},
    color::RgbFloat,
    material::{Dielectric, Lambertian, Metal},
    random::Rng,
    scene::{Hittable, Scene, Sphere},
};

// Defaults.
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;
const HEIGHT: u32 = 675;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: u32 = (ASPECT_RATIO * HEIGHT as f32) as u32;

enum SceneType {
    Simple,
    Cover,
}

impl FromStr for SceneType {
    type Err = &'static str;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "simple" => Ok(SceneType::Simple),
            "cover" => Ok(SceneType::Cover),
            _ => Err("scene must be 'simple' or 'cover'"),
        }
    }
}

/// Ray Tracing In One Weekend: CPU renderer
#[derive(FromArgs)]
struct Args {
    /// number of samples per pixel
    #[argh(option, short = 'p', default = "SAMPLES_PER_PIXEL")]
    samples_per_pixel: u32,

    /// maximum number of ray bounces per traced path
    #[argh(option, short = 'm', default = "MAX_DEPTH - 1")]
    max_bounces: u32,

    /// the scene to render ("simple" or "cover")
    #[argh(option, short = 's', default = "SceneType::Cover")]
    scene: SceneType,

    /// image width
    #[argh(option, short = 'w')]
    width: Option<u32>,

    /// image height
    #[argh(option, short = 'h')]
    height: Option<u32>,
}

// The scene from the middle chapters with 3 spheres.
fn simple_scene(aspect_ratio: f32) -> (Scene, Camera) {
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

// The final scene which is the cover of the book.
fn cover_scene(rng: &Rng, aspect_ratio: f32) -> (Scene, Camera) {
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

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    // Image
    let (width, height, aspect_ratio) = match (args.width, args.height) {
        (None, None) => (WIDTH, HEIGHT, ASPECT_RATIO),
        (Some(w), None) => (w, ((w as f32) / ASPECT_RATIO) as u32, ASPECT_RATIO),
        (None, Some(h)) => (((h as f32) * ASPECT_RATIO) as u32, h, ASPECT_RATIO),
        (Some(w), Some(h)) => (w, h, (w as f32) / (h as f32)),
    };
    if width == 0 || height == 0 {
        return Err(anyhow!("image dimensions must be non-zero"));
    }

    // Scene
    let rng = Rng::new();
    let (scene, camera) = match args.scene {
        SceneType::Simple => simple_scene(aspect_ratio),
        SceneType::Cover => cover_scene(&rng, aspect_ratio),
    };

    // Render
    let mut img = RgbImage::new(width, height);

    for row in (0..height).rev() {
        print!("\rScanlines remaining: {}", row);
        io::stdout().flush()?;
        for col in 0..width {
            let mut pixel_color = RgbFloat::black();

            for _ in 0..args.samples_per_pixel {
                let u = ((col as f32) + rng.random_float()) / (width as f32 - 1.0);
                let v = ((row as f32) + rng.random_float()) / (height as f32 - 1.0);
                let ray = camera.ray(u, v);

                // Add 1 for at least one recursion for the primary rays.
                pixel_color += ray_color(&ray, &scene, &rng, args.max_bounces + 1);
            }

            // Divide the color by the number of samples and gamma-correct for gamma=2.0.
            pixel_color /= args.samples_per_pixel as f32;
            let pixel_color = RgbFloat::new(
                pixel_color.r().sqrt(),
                pixel_color.g().sqrt(),
                pixel_color.b().sqrt(),
            );

            img.put_pixel(col, height - row - 1, pixel_color.into());
        }
    }

    println!("\nDone");

    img.save("image.ppm").context("failed to write PPM image")
}
