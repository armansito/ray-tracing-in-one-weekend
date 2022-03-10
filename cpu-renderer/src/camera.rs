use crate::algebra::{Point3, Ray, Vec3};

pub struct CameraParams {
    pub origin: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub aspect_ratio: f32,

    /// Vertical field of view in radians
    pub fov_y: f32,
}

pub struct Camera {
    origin: Point3,

    // Vectors representing the camera view plane.
    horizontal: Vec3,
    vertical: Vec3,

    // The lower-left corner of the viewport in world space coordinates. This maps to
    // (0.0, 0.0) in viewport coordinates.
    lower_left_corner: Point3,
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

        let horizontal = width * u;
        let vertical = height * v;
        Camera {
            origin: params.origin,
            horizontal,
            vertical,
            lower_left_corner: params.origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    /// Returns a world-space primary ray from the eye of the camera towards the viewport coordinates (u, v).
    /// The only valid coordinates are in [0.0, 1.0]
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
