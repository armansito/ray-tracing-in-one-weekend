// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Float3 {
    pub data: [f32; 3],
}

pub type Point3 = Float3;
pub type Vec3 = Float3;

impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Float3 {
        Float3 { data: [x, y, z] }
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.data[0]
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.data[1]
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(&self, rhs: &Float3) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Float3) -> Float3 {
        Float3 {
            data: [
                self.y() * rhs.z() - self.z() * rhs.y(),
                self.z() * rhs.x() - self.x() * rhs.z(),
                self.x() * rhs.y() - self.y() * rhs.x(),
            ],
        }
    }

    pub fn reflect(&self, normal: &Float3) -> Float3 {
        self - 2.0 * self.dot(normal) * normal
    }

    /// `refraction_ratio` equals n / n_prime, where n is the refractive index of the
    /// incident ray's environment and n_prime is that of the transmitting environment.
    pub fn refract(&self, normal: &Float3, refraction_ratio: f32) -> Float3 {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = refraction_ratio * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }

    pub fn normalized(&self) -> Float3 {
        *self / self.length()
    }
}

// Overloads for binary arithmetic operators. They apply component-wise to all operands of type
// `Float3`.
impl_op_ex!(+ |lhs: &Float3, rhs: &Float3| -> Float3 {
    Float3 { data: [lhs.x() + rhs.x(), lhs.y() + rhs.y(), lhs.z() + rhs.z()] }
});
impl_op_ex_commutative!(+ |lhs: &Float3, rhs: f32| -> Float3 {
    Float3 { data: [lhs.x() + rhs, lhs.y() + rhs, lhs.z() + rhs] }
});
impl_op_ex!(- |lhs: &Float3, rhs: &Float3| -> Float3 {
    Float3 { data: [lhs.x() - rhs.x(), lhs.y() - rhs.y(), lhs.z() - rhs.z()] }
});
impl_op_ex!(* |lhs: &Float3, rhs: &Float3| -> Float3 {
    Float3 { data: [lhs.x() * rhs.x(), lhs.y() * rhs.y(), lhs.z() * rhs.z()] }
});
impl_op_ex_commutative!(* |lhs: &Float3, rhs: f32| -> Float3 {
    Float3 { data: [lhs.x() * rhs, lhs.y() * rhs, lhs.z() * rhs] }
});
impl_op_ex!(/ |lhs: &Float3, rhs: f32| -> Float3 {
    Float3 { data: [lhs.x() / rhs, lhs.y() / rhs, lhs.z() / rhs] }
});

// Unary negation operator.
impl_op_ex!(- |v: &Float3| -> Float3 { Float3 { data: [-v.x(), -v.y(), -v.z()] } });

// Arithmetic assignment operators.
impl_op_ex!(+= |lhs: &mut Float3, rhs: &Float3| {
    lhs.data[0] += rhs.data[0];
    lhs.data[1] += rhs.data[1];
    lhs.data[2] += rhs.data[2];
});
impl_op!(*= |lhs: &mut Float3, rhs: f32| {
    lhs.data[0] *= rhs;
    lhs.data[1] *= rhs;
    lhs.data[2] *= rhs;
});
impl_op!(/= |lhs: &mut Float3, rhs: f32| {
    lhs.data[0] /= rhs;
    lhs.data[1] /= rhs;
    lhs.data[2] /= rhs;
});

//////////////////////////

pub struct Ray {
    pub origin: Point3,

    /// The direction vector is not guaranteed to be normalized and the caller must take care to
    /// normalize it when necessary.
    pub direction: Vec3,
}

impl Ray {
    /// Constructs a ray with a normalized direction vector.
    #[allow(dead_code)]
    pub fn with_unit_direction(origin: &Point3, direction: &Vec3) -> Ray {
        Ray { origin: *origin, direction: direction.normalized() }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
