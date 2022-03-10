use crate::{algebra::Ray, color::RgbFloat, random::Rng, scene::HitRecord};
use std::rc::Rc;

type Ref<T> = Rc<Box<T>>;

pub trait Material {
    /// Scatter the given ray at the given surface. Returns a bounced ray and albedo that
    /// represents the attenuation factor for the outgoing radiance. Retuns `None` if incoming
    /// radiance is entirely absorbed.
    fn scatter(&self, incident: &Ray, surface: &HitRecord) -> Option<(RgbFloat, Ray)>;
}

pub type MaterialRef = Ref<dyn Material>;

/// Lambertian diffuse material.
pub struct Lambertian {
    albedo: RgbFloat,
    rng: Rng,
}

impl Lambertian {
    pub fn new(albedo: RgbFloat) -> Ref<dyn Material> {
        Rc::new(Box::new(Lambertian { albedo, rng: Rng::new() }))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, surface: &HitRecord) -> Option<(RgbFloat, Ray)> {
        let scattered = self.rng.sample_hemisphere(&surface.normal);
        Some((self.albedo, Ray { origin: surface.point, direction: scattered }))
    }
}

/// Metallic material.
pub struct Metal {
    albedo: RgbFloat,
    fuzz: f32,
    rng: Rng,
}

impl Metal {
    pub fn new(albedo: RgbFloat, fuzz: f32) -> Ref<dyn Material> {
        Rc::new(Box::new(Metal { albedo, fuzz: fuzz.clamp(0.0, 1.0), rng: Rng::new() }))
    }
}

impl Material for Metal {
    fn scatter(&self, incident: &Ray, surface: &HitRecord) -> Option<(RgbFloat, Ray)> {
        let mirror = incident.direction.normalized().reflect(&surface.normal);
        let scattered = mirror + self.fuzz * self.rng.sample_sphere();
        Some((self.albedo, Ray { origin: surface.point, direction: scattered }))
    }
}
