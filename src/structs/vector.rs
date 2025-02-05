use std::ops;

use bytemuck::{Pod, Zeroable};

use super::{type_def::base_types::AiReal, AiQuaternion};

#[repr(C)]
#[derive(Debug, PartialEq, Pod, Clone, Copy, Zeroable)]
pub struct AiVector2D {
    pub x: AiReal,
    pub y: AiReal,
}

impl Default for AiVector2D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Into<[f32; 2]> for AiVector2D {
    fn into(self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}

impl From<[f32; 2]> for AiVector2D {
    fn from(value: [f32; 2]) -> Self {
        AiVector2D {
            x: value[0] as AiReal,
            y: value[1] as AiReal,
        }
    }
}

impl AiVector2D {
    pub fn new(x: AiReal, y: AiReal) -> AiVector2D {
        AiVector2D { x, y }
    }

    pub fn set(&mut self, x: AiReal, y: AiReal) {
        self.x = x;
        self.y = y;
    }

    pub fn square_length(&self) -> AiReal {
        self.x * self.x + self.y + self.y
    }

    pub fn len(&self) -> AiReal {
        AiReal::sqrt(self.square_length())
    }

    pub fn norm(self) -> AiVector2D {
        let length = &self.len();
        self / *length
    }

    pub fn normalize(&mut self) {
        *self /= self.len()
    }

    pub fn cross(&self, other: &Self) -> AiReal {
        self.x * other.y - self.y * other.x
    }
}

impl ops::AddAssign<AiVector2D> for AiVector2D {
    fn add_assign(&mut self, rhs: AiVector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Div<AiReal> for AiVector2D {
    type Output = Self;

    fn div(self, rhs: AiReal) -> Self::Output {
        AiVector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<AiReal> for AiVector2D {
    fn div_assign(&mut self, rhs: AiReal) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Index<u8> for AiVector2D {
    type Output = AiReal;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.x,
        }
    }
}

impl ops::Mul<f32> for AiVector2D {
    type Output = AiVector2D;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Pod, Clone, Copy, Zeroable)]
pub struct AiVector3D {
    pub x: AiReal,
    pub y: AiReal,
    pub z: AiReal,
}

impl Default for AiVector3D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl Into<[f32; 3]> for AiVector3D {
    fn into(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}

impl From<[f32; 3]> for AiVector3D {
    fn from(value: [f32; 3]) -> Self {
        AiVector3D {
            x: value[0] as AiReal,
            y: value[1] as AiReal,
            z: value[2] as AiReal,
        }
    }
}

impl AiVector3D {
    pub fn new(x: AiReal, y: AiReal, z: AiReal) -> AiVector3D {
        AiVector3D { x, y, z }
    }

    pub fn set(&mut self, x: AiReal, y: AiReal, z: AiReal) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn square_length(&self) -> AiReal {
        self.x * self.x + self.y + self.y + self.z * self.z
    }

    pub fn len(&self) -> AiReal {
        AiReal::sqrt(self.square_length())
    }

    pub fn norm(self) -> AiVector3D {
        let length = &self.len();
        self / *length
    }

    pub fn to_quat(self, w: AiReal) -> AiQuaternion{
        AiQuaternion { w, x: self.x, y: self.y, z: self.z }
    }

    pub fn normalize(&mut self) {
        *self /= self.len()
    }

    pub fn cross(&self, other: &Self) -> Self {
        AiVector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl ops::AddAssign<AiVector3D> for AiVector3D {
    fn add_assign(&mut self, rhs: AiVector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::BitXor<AiVector3D> for AiVector3D {
    type Output = AiVector3D;

    fn bitxor(self, rhs: AiVector3D) -> Self::Output {
        self.cross(&rhs)
    }
}
impl ops::BitXor<&AiVector3D> for AiVector3D {
    type Output = AiVector3D;

    fn bitxor(self, rhs: &AiVector3D) -> Self::Output {
        self.cross(rhs)
    }
}
impl ops::BitXor<AiVector3D> for &AiVector3D {
    type Output = AiVector3D;

    fn bitxor(self, rhs: AiVector3D) -> Self::Output {
        self.cross(&rhs)
    }
}
impl ops::BitXor<&AiVector3D> for &AiVector3D {
    type Output = AiVector3D;

    fn bitxor(self, rhs: &AiVector3D) -> Self::Output {
        self.cross(rhs)
    }
}

impl ops::Div<AiReal> for AiVector3D {
    type Output = Self;

    fn div(self, rhs: AiReal) -> Self::Output {
        AiVector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ops::DivAssign<AiReal> for AiVector3D {
    fn div_assign(&mut self, rhs: AiReal) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Index<u8> for AiVector3D {
    type Output = AiReal;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
}

impl ops::Mul<AiVector3D> for AiVector3D {
    type Output = AiReal;

    fn mul(self, rhs: AiVector3D) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<&AiVector3D> for AiVector3D {
    type Output = AiReal;

    fn mul(self, rhs: &AiVector3D) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl ops::Mul<AiVector3D> for &AiVector3D {
    type Output = AiReal;

    fn mul(self, rhs: AiVector3D) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl ops::Mul<&AiVector3D> for &AiVector3D {
    type Output = AiReal;

    fn mul(self, rhs: &AiVector3D) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<f32> for AiVector3D {
    type Output = AiVector3D;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl ops::Sub<AiVector3D> for AiVector3D{
    type Output = AiVector3D;

    fn sub(self, rhs: AiVector3D) -> Self::Output {
        AiVector3D{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
