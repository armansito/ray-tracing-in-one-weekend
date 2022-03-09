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

    pub fn normalized(&self) -> Float3 {
        *self / self.length()
    }
}

impl ops::Add for Float3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { data: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()] }
    }
}

impl ops::Add<f32> for Float3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self { data: [self.x() + rhs, self.y() + rhs, self.z() + rhs] }
    }
}

impl ops::AddAssign for Float3 {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl ops::Neg for &Float3 {
    type Output = Float3;

    fn neg(self) -> Float3 {
        Float3 { data: [-self.x(), -self.y(), -self.z()] }
    }
}

impl ops::Neg for Float3 {
    type Output = Self;

    fn neg(self) -> Float3 {
        Float3 { data: [-self.x(), -self.y(), -self.z()] }
    }
}

impl ops::Sub for Float3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { data: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()] }
    }
}

impl ops::Mul<f32> for Float3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { data: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}

impl ops::Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Float3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl ops::Div<f32> for Float3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self { data: [self.x() / rhs, self.y() / rhs, self.z() / rhs] }
    }
}

impl ops::DivAssign<f32> for Float3 {
    fn div_assign(&mut self, rhs: f32) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
    }
}

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
