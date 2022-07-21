// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use crate::algebra::Vec3;
use {
    rand::distributions::{Distribution, Uniform},
    rand_distr::{UnitDisc, UnitSphere},
};

pub struct Rng {
    uniform: Uniform<f32>,
}

impl Rng {
    pub fn new() -> Rng {
        Rng { uniform: Uniform::new_inclusive(0.0, 1.0) }
    }

    /// Return a random number from a uniform distribution between 0.0 and 1.0.
    pub fn random_float(&self) -> f32 {
        let mut rng = rand::thread_rng();
        self.uniform.sample(&mut rng)
    }

    /// Return a random vector from a uniform distribution between 0.0 and 1.0.
    pub fn random_vec3(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            self.uniform.sample(&mut rng),
            self.uniform.sample(&mut rng),
            self.uniform.sample(&mut rng),
        )
    }

    /// Return a random vector with values within the given range.
    pub fn random_vec3_in_range(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new_inclusive(min, max);
        Vec3::new(
            uniform.sample(&mut rng),
            uniform.sample(&mut rng),
            uniform.sample(&mut rng),
        )
    }

    /// Return a random point inside a unit disk centered at origin and on the XY plane.
    pub fn sample_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let p = UnitDisc.sample(&mut rng);
        Vec3::new(p[0], p[1], 0.0)
    }

    /// Return a random point on the surface of a unit sphere centered at origin.
    pub fn sample_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 { data: UnitSphere.sample(&mut rng) }
    }

    /// Return a random point on the urface of a unit hemisphere centered at origin, with its apex
    /// aligned towards the given normal.
    pub fn sample_hemisphere(normal: &Vec3) -> Vec3 {
        let vec = Rng::sample_sphere();

        // Flip the vector if its not in the same hemisphere as the normal.
        if vec.dot(normal) < 0.0 {
            -vec
        } else {
            vec
        }
    }
}
