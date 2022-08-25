// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use crate::{
    algebra::{Point3, Ray, Vec3},
    material::MaterialRef,
};

pub type Scene = Vec<Box<dyn Hittable>>;

pub struct HitRecord {
    pub point: Point3,

    /// The normal vector here is defined to be always against the incident ray, i.e the angle
    /// between the ray and the normal is acute regardless of whether an inside or outside surface
    /// has been hit. The `is_front_face` field stores this information, following the convention
    /// defined in section 6.4 "Front Faces Versus Back Faces".
    ///
    /// This vector is always of unit length.
    pub normal: Vec3,

    /// Surface properties at the point of intersection.
    pub material: MaterialRef,

    /// The interpolation distance along the input ray that results in `point`.
    pub t: f32,

    /// True if the ray intersected the surface at its front side.
    pub is_front_face: bool,
}

/// Ignore nearby intersections that are closer to the ray origin than this factor. This helps
/// prevent self-intersections
const EPSILON: f32 = 0.001;

pub trait Hittable: Sync {
    /// Find and return the closest intersection point along the ray within [t_max, t_max].
    fn bounded_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    /// Find and return the closest intersection point along the ray in front of its origin.
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.bounded_hit(ray, EPSILON, f32::INFINITY)
    }
}

/// The hittable trait is implemented for a dynamic list of hittables.
impl Hittable for Vec<Box<dyn Hittable + '_>> {
    fn bounded_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut nearest_hit: Option<HitRecord> = None;
        for entry in self.iter() {
            if let Some(hit) =
                entry.bounded_hit(ray, t_min, nearest_hit.as_ref().map_or(t_max, |h| h.t))
            {
                nearest_hit = Some(hit);
            }
        }
        nearest_hit
    }
}

pub struct Sphere {
    pub center: Point3,

    /// A negative radius is legal and results in inverted surface normals (pointing towards the
    /// center).
    pub radius: f32,

    pub material: MaterialRef,
}

impl Hittable for Sphere {
    fn bounded_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root within the acceptable range.
        let root = {
            let sqrt_d = discriminant.sqrt();
            let root = (-half_b - sqrt_d) / a;
            if root >= t_min && root <= t_max {
                root
            } else {
                let root = (-half_b + sqrt_d) / a;
                if root < t_min || root > t_max {
                    return None;
                }
                root
            }
        };

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let (is_front_face, normal) = align_face_normal(ray, &normal);
        Some(HitRecord { point, normal, material: self.material.clone(), t: root, is_front_face })
    }
}

// Transform the given "outward" facing normal such that the result is against the direction of the
// incident ray. Returns the transformed normal vector and whether or not the ray intersected the
// surface from the front or back.
fn align_face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
    let is_front = ray.direction.dot(outward_normal) <= 0.0;
    let normal = if is_front { *outward_normal } else { -outward_normal };
    (is_front, normal)
}
