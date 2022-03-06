use crate::algebra::Float3;
use {image::Rgb, std::convert};

pub struct RgbFloat(pub Float3);

impl RgbFloat {
    pub fn new(r: f32, g: f32, b: f32) -> RgbFloat {
        RgbFloat(Float3 { data: [r, g, b] })
    }

    pub fn black() -> RgbFloat {
        RgbFloat::new(0.0, 0.0, 0.0)
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

impl std::ops::Div<f32> for RgbFloat {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        RgbFloat(self.0 / rhs)
    }
}
