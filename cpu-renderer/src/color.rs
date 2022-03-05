use {
    image::Rgb,
    std::convert,
};
use crate::algebra::Float3;

pub struct RgbFloat(pub Float3);

impl RgbFloat {
    pub fn new(r: f32, g: f32, b: f32) -> RgbFloat {
        RgbFloat(Float3 { data: [r, g, b] })
    }

    #[inline(always)]
    pub fn r(&self) -> f32 { self.0.x() }

    #[inline(always)]
    pub fn g(&self) -> f32 { self.0.y() }

    #[inline(always)]
    pub fn b(&self) -> f32 { self.0.z() }
}

impl convert::From<&RgbFloat> for Rgb<u8> {
    fn from(src: &RgbFloat) -> Self {
        Self([
             (255.999 * src.r().clamp(0.0, 1.0)) as u8,
             (255.999 * src.g().clamp(0.0, 1.0)) as u8,
             (255.999 * src.b().clamp(0.0, 1.0)) as u8,
        ])
    }
}

impl convert::From<RgbFloat> for Rgb<u8> {
    fn from(src: RgbFloat) -> Self {
        Self::from(&src) 
    }
}
