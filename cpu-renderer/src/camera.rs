use crate::algebra::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,

    // The viewport dimensions.
    viewport_height: f32,
    viewport_width: f32,

    // The lower-left corner of the viewport in world space coordinates. This maps to
    // (0.0, 0.0) in viewport coordinates.
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(origin: Point3, aspect_ratio: f32, focal_length: f32) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        Camera {
            origin,
            viewport_height,
            viewport_width,
            lower_left_corner: origin
                - Vec3::new(viewport_width / 2.0, viewport_height / 2.0, focal_length),
        }
    }

    /// Returns a world-space primary ray from the eye of the camera towards the viewport coordinates (u, v).
    /// The only valid coordinates are in [0.0, 1.0]
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner
                + Vec3::new(u * self.viewport_width, v * self.viewport_height, 0.0)
                - self.origin,
        }
    }
}
