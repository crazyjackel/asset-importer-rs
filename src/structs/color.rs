
use bytemuck::{Pod, Zeroable};

use super::type_def::EPSILON_F;

#[repr(C)]
#[derive(Debug, PartialEq, Pod, Clone, Copy, Zeroable)]
pub struct AiColor3D {
    pub r: f32,
    pub g: f32,
    pub b: f32
}


impl Default for AiColor3D{
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default() }
    }
}

impl From<[f32;3]> for AiColor3D{
    fn from(value: [f32;3]) -> Self {
        AiColor3D { r: value[0], g: value[1], b: value[2] }
    }
}

impl AiColor3D {
    pub fn is_black(&self) -> bool {
        f32::abs(self.r) < EPSILON_F && f32::abs(self.g) < EPSILON_F && f32::abs(self.b) < EPSILON_F
    }
    pub fn new(pr: f32, pg: f32, pb: f32) -> Self{
        AiColor3D{r: pr, g: pg, b: pb}
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Pod, Clone, Copy, Zeroable)]
pub struct AiColor4D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}


impl Default for AiColor4D{
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default(), a: Default::default() }
    }
}

impl From<[f32;4]> for AiColor4D{
    fn from(value: [f32;4]) -> Self {
        AiColor4D { r: value[0], g: value[1], b: value[2], a: value[3] }
    }
}

impl AiColor4D {
    pub fn is_black(&self) -> bool {
        f32::abs(self.r) < EPSILON_F && f32::abs(self.g) < EPSILON_F && f32::abs(self.b) < EPSILON_F
    }
    pub fn new(pr: f32, pg: f32, pb: f32, pa: f32) -> Self{
        AiColor4D{r: pr, g: pg, b: pb, a: pa}
    }
}