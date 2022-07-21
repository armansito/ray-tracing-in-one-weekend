// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use crate::{algebra::Float3, random::Rng};
use {
    image::Rgb,
    std::{convert, ops},
};

#[derive(Debug, Copy, Clone)]
pub struct RgbFloat(pub Float3);

impl RgbFloat {
    pub fn new(r: f32, g: f32, b: f32) -> RgbFloat {
        RgbFloat(Float3 { data: [r, g, b] })
    }

    pub fn gray(value: f32) -> RgbFloat {
        RgbFloat::new(value, value, value)
    }

    pub fn black() -> RgbFloat {
        RgbFloat::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> RgbFloat {
        RgbFloat::new(1.0, 1.0, 1.0)
    }

    pub fn random(rng: &Rng) -> RgbFloat {
        RgbFloat(rng.random_vec3())
    }

    pub fn random_in_range(min: f32, max: f32) -> RgbFloat {
        RgbFloat(Rng::random_vec3_in_range(min.clamp(0.0, 1.0), max.clamp(0.0, 1.0)))
    }

    #[inline(always)]
    pub fn r(&self) -> f32 {
        self.0.x()
    }

    #[inline(always)]
    pub fn g(&self) -> f32 {
        self.0.y()
    }

    #[inline(always)]
    pub fn b(&self) -> f32 {
        self.0.z()
    }
}

impl convert::From<&RgbFloat> for Rgb<u8> {
    fn from(src: &RgbFloat) -> Self {
        Self([
            (256.0 * src.r().clamp(0.0, 0.999)) as u8,
            (256.0 * src.g().clamp(0.0, 0.999)) as u8,
            (256.0 * src.b().clamp(0.0, 0.999)) as u8,
        ])
    }
}

impl convert::From<RgbFloat> for Rgb<u8> {
    fn from(src: RgbFloat) -> Self {
        Self::from(&src)
    }
}

impl std::ops::AddAssign for RgbFloat {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl_op_ex!(* |lhs: &RgbFloat, rhs: &RgbFloat| -> RgbFloat { RgbFloat(lhs.0 * rhs.0) });
impl_op_ex_commutative!(* |lhs: &RgbFloat, rhs: f32| -> RgbFloat { RgbFloat(lhs.0 * rhs) });
impl_op_ex!(/ |lhs: &RgbFloat, rhs: f32| -> RgbFloat { RgbFloat(lhs.0 / rhs) });
impl_op_ex!(/= |lhs: &mut RgbFloat, rhs: f32| { lhs.0 /= rhs });
