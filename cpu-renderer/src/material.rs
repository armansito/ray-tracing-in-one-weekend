// Copyright (c) 2022 Arman Uguray
//
// Use of this source code is governed by the MIT License described
// in the LICENSE file.

use crate::{algebra::Ray, color::RgbFloat, random::Rng, scene::HitRecord};
use std::sync::Arc;

type Ref<T> = Arc<Box<T>>;

pub trait Material: Send + Sync {
    /// Scatter the given ray at the given surface. Returns a bounced ray and albedo that
    /// represents the attenuation factor for the outgoing radiance. Retuns `None` if incoming
    /// radiance is entirely absorbed.
    fn scatter(&self, incident: &Ray, surface: &HitRecord) -> Option<(RgbFloat, Ray)>;
}

pub type MaterialRef = Ref<dyn Material>;

/// Lambertian diffuse material.
pub struct Lambertian {
    albedo: RgbFloat,
}

impl Lambertian {
    pub fn new(albedo: RgbFloat) -> Ref<dyn Material> {
        Arc::new(Box::new(Lambertian { albedo }))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, surface: &HitRecord) -> Option<(RgbFloat, Ray)> {
        let scattered = Rng::sample_hemisphere(&surface.normal);
        Some((self.albedo, Ray { origin: surface.point, direction: scattered }))
    }
}

/// Metallic material.
pub struct Metal {
    albedo: RgbFloat,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: RgbFloat, fuzz: f32) -> Ref<dyn Material> {
        Arc::new(Box::new(Metal { albedo, fuzz: fuzz.clamp(0.0, 1.0) }))
    }
}

impl Material for Metal {
    fn scatter(&self, incident: &Ray, surface: &HitRecord) -> Option<(RgbFloat, Ray)> {
        let mirror = incident.direction.normalized().reflect(&surface.normal);
        let scattered = mirror + self.fuzz * Rng::sample_sphere();
        Some((self.albedo, Ray { origin: surface.point, direction: scattered }))
    }
}

/// Glass-like material.
pub struct Dielectric {
    index_of_refraction: f32,
    rng: Rng,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Ref<dyn Material> {
        Arc::new(Box::new(Dielectric { index_of_refraction, rng: Rng::new() }))
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident: &Ray, surface: &HitRecord) -> Option<(RgbFloat, Ray)> {
        let refraction_ratio = if surface.is_front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let incident = incident.direction.normalized();
        let cos_theta = (-incident).dot(&surface.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > self.rng.random_float()
        {
            incident.reflect(&surface.normal)
        } else {
            incident.refract(&surface.normal, refraction_ratio)
        };
        Some((RgbFloat::white(), Ray { origin: surface.point, direction }))
    }
}

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    // Use Schlick's approximation for reflectance:
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
