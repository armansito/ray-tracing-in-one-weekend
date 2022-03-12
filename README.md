# Ray Tracing In One Weekend - in Rust

This is my implementation of the [_Ray Tracing in One
Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) series. I had been
thinking about making my own Rust port for this series for a long time now, so here it is!

I tried to keep the code as close to the book as possible with only a few minor deviations in
type and variable names. All of the CPU renderer code is self-contained but I decided to use the
following third-party crates for a few things that are implemented directly in the book:

- [images](https://crates.io/crates/image) to convert sRGB to the stored image format (in this case
  PPM)
- [rand]((https://crates.io/crates/rand) and [rand_distr](https://crates.io/crates/rand_distr) to
  sample from uniform distributions. I made use of the
  [UnitSphere](https://docs.rs/rand_distr/0.4.3/rand_distr/struct.UnitSphere.html) and
  [UnitDisc](https://docs.rs/rand_distr/0.4.3/rand_distr/struct.UnitDisc.html) utilities instead of
  implementing rejection sampling myself (which wouldn't be hard but I chose to explore the crate
  for learning purposes).

---

You can build and run the code by executing the following inside the `cpu-renderer` directory:
```
cargo run --release
```

`cargo run --release -- --help` will output the following:
```
Usage: cpu-renderer [-p <samples-per-pixel>] [-m <max-bounces>] [-s <scene>] [-w <width>] [-h <height>]

Ray Tracing In One Weekend: CPU renderer

Options:
  -p, --samples-per-pixel
                      number of samples per pixel
  -m, --max-bounces maximum number of ray bounces per traced path
  -s, --scene       the scene to render ("simple" or "cover")
  -w, --width       image width
  -h, --height      image height
  --help            display usage information
```

## Images

![](https://raw.githubusercontent.com/armansito/ray-tracing-in-one-weekend/main/cpu-renderer/images/final_scene.jpg)

![](https://raw.githubusercontent.com/armansito/ray-tracing-in-one-weekend/main/cpu-renderer/images/three_balls.jpg)

![](https://raw.githubusercontent.com/armansito/ray-tracing-in-one-weekend/main/cpu-renderer/images/three_balls_angle_view.jpg)
