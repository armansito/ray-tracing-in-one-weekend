use crate::algebra::Vec3;
use {
    rand::{
        distributions::{Distribution, Uniform},
        rngs::ThreadRng,
    },
    rand_distr::UnitSphere,
};

pub struct Rng {
    rng: ThreadRng,
    uniform: Uniform<f32>,
}

impl Rng {
    pub fn new() -> Rng {
        Rng { rng: rand::thread_rng(), uniform: Uniform::new_inclusive(0.0, 1.0) }
    }

    /// Return a random number from a uniform distribution between 0.0 and 1.0.
    pub fn random_float(&mut self) -> f32 {
        self.uniform.sample(&mut self.rng)
    }

    /// Return a random vector from a uniform distribution between 0.0 and 1.0.
    #[allow(dead_code)]
    pub fn random_vec3(&mut self) -> Vec3 {
        Vec3::new(
            self.uniform.sample(&mut self.rng),
            self.uniform.sample(&mut self.rng),
            self.uniform.sample(&mut self.rng),
        )
    }

    /// Return a random point on the surface of a unit sphere centered at origin.
    pub fn sample_unit_sphere(&mut self) -> Vec3 {
        Vec3 { data: UnitSphere.sample(&mut self.rng) }
    }
}