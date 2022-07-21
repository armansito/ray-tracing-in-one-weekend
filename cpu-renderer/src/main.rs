// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

#[macro_use]
extern crate impl_ops;

use {
    anyhow::{anyhow, Context, Result},
    argh::FromArgs,
    image::RgbImage,
    std::str::FromStr,
};

mod algebra;
mod camera;
mod color;
mod material;
mod random;
mod render;
mod scene;
mod scenes;

use crate::{
    random::Rng,
    scenes::{cover_scene, simple_scene},
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

    /// display the render in a native window
    #[argh(switch, short = 'v')]
    view: bool,
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
    render::render_scene(
        &scene,
        &camera,
        &rng,
        args.max_bounces + 1,
        args.samples_per_pixel,
        &mut img,
    );
    println!("\nDone");

    // Save the image to a file
    img.save("image.ppm").context("failed to write PPM image")
}
