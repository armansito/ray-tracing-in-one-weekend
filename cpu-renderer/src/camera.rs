// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use crate::{
    algebra::{Point3, Ray, Vec3},
    random::Rng,
};

pub struct CameraParams {
    pub origin: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub aspect_ratio: f32,

    /// Vertical field of view in radians
    pub fov_y: f32,

    /// Aperture and focus distance control the depth of field (defocus distance) effect.
    pub aperture: f32,
    pub focus_distance: f32,
}

pub struct Camera {
    origin: Point3,

    // Vectors representing the camera view plane.
    horizontal: Vec3,
    vertical: Vec3,

    // The lower-left corner of the viewport in world space coordinates. This maps to
    // (0.0, 0.0) in viewport coordinates.
    lower_left_corner: Point3,

    // The orthonormal basis vectors defining the camera orientation.
    u: Vec3, v: Vec3, #[allow(dead_code)] w: Vec3,

    lens_radius: f32,
}

impl Camera {
    /// `fov_y`: the vertical field of view, in radians
    pub fn new(params: CameraParams) -> Camera {
        // Viewport dimensions.
        let h = (params.fov_y / 2.0).tan();
        let height = 2.0 * h;
        let width = height * params.aspect_ratio;

        let w = (params.origin - params.look_at).normalized();
        let u = params.up.cross(&w).normalized();
        let v = w.cross(&u);

        let horizontal = params.focus_distance * width * u;
        let vertical = params.focus_distance * height * v;
        Camera {
            origin: params.origin,
            horizontal,
            vertical,
            lower_left_corner: params.origin
                - horizontal / 2.0
                - vertical / 2.0
                - params.focus_distance * w,
            u, v, w,
            lens_radius: params.aperture / 2.0,
        }
    }

    /// Returns a world-space primary ray from the eye of the camera towards the viewport coordinates (s, t).
    /// The only valid coordinates are in [0.0, 1.0]
    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Rng::sample_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
