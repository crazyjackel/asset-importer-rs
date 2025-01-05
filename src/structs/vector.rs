use std::ops;

use super::types::base_types::AiReal;

#[derive(Debug, PartialEq)]
pub struct AiVector2D {
    x: AiReal,
    y: AiReal,
}

impl Default for AiVector2D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl ops::Div<AiReal> for AiVector2D{
    type Output = Self;

    fn div(self, rhs: AiReal) -> Self::Output {
        AiVector2D { x: self.x / rhs, y: self.y / rhs}
    }
}

impl ops::DivAssign<AiReal> for AiVector2D{
    fn div_assign(&mut self, rhs: AiReal) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Index<u8> for AiVector2D{
    type Output = AiReal;

    fn index(&self, index: u8) -> &Self::Output {
        match index{
            0 => &self.x,
            1 => &self.y,
            _ => &self.x
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

    pub fn square_length(&self) -> AiReal{
        self.x * self.x + self.y + self.y
    }

    pub fn len(&self) -> AiReal{
        AiReal::sqrt(self.square_length())
    }

    pub fn norm(self) -> AiVector2D {
        let length = &self.len();
        self / *length
    }

    pub fn normalize(&mut self){
        *self /= self.len()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiVector3D {
    x: AiReal,
    y: AiReal,
    z: AiReal
}

impl Default for AiVector3D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default()
        }
    }
}

impl ops::Div<AiReal> for AiVector3D{
    type Output = Self;

    fn div(self, rhs: AiReal) -> Self::Output {
        AiVector3D { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl ops::DivAssign<AiReal> for AiVector3D{
    fn div_assign(&mut self, rhs: AiReal) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Index<u8> for AiVector3D{
    type Output = AiReal;

    fn index(&self, index: u8) -> &Self::Output {
        match index{
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x
        }
    }
}

impl AiVector3D {
    fn new(x: AiReal, y: AiReal, z:AiReal) -> AiVector3D {
        AiVector3D { x, y, z }
    }

    fn set(&mut self, x: AiReal, y: AiReal, z: AiReal) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn square_length(&self) -> AiReal{
        self.x * self.x + self.y + self.y + self.z * self.z
    }

    fn len(&self) -> AiReal{
        AiReal::sqrt(self.square_length())
    }

    fn norm(self) -> AiVector3D {
        let length = &self.len();
        self / *length
    }

    fn normalize(&mut self){
        *self /= self.len()
    }
}
