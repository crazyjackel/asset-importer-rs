use bytemuck::{Pod, Zeroable};

use super::{base_types::AiReal, type_def::EPSILON_F, AiVector3D};

/// A C-Based Color representation for RGB
#[repr(C)]
#[derive(Debug, PartialEq, Pod, Clone, Copy, Zeroable, Default)]
pub struct AiColor3D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<[f32; 3]> for AiColor3D {
    fn from(value: [f32; 3]) -> Self {
        AiColor3D {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl From<AiColor3D> for [f32; 3] {
    fn from(val: AiColor3D) -> Self {
        [val.r, val.g, val.b]
    }
}

impl From<[f32; 4]> for AiColor3D {
    fn from(value: [f32; 4]) -> Self {
        AiColor3D {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl From<AiColor3D> for [f32; 4] {
    fn from(val: AiColor3D) -> Self {
        [val.r, val.g, val.b, 1.0]
    }
}

impl From<AiColor3D> for AiVector3D {
    fn from(value: AiColor3D) -> Self {
        AiVector3D {
            x: value.r as AiReal,
            y: value.g as AiReal,
            z: value.b as AiReal,
        }
    }
}

impl AiColor3D {
    pub fn is_black(&self) -> bool {
        f32::abs(self.r) < EPSILON_F && f32::abs(self.g) < EPSILON_F && f32::abs(self.b) < EPSILON_F
    }
    pub fn new(pr: f32, pg: f32, pb: f32) -> Self {
        AiColor3D {
            r: pr,
            g: pg,
            b: pb,
        }
    }
}

/// A C-Based Color representation for RGBA
#[repr(C)]
#[derive(Debug, PartialEq, Pod, Clone, Copy, Zeroable, Default)]
pub struct AiColor4D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<[f32; 4]> for AiColor4D {
    fn from(value: [f32; 4]) -> Self {
        AiColor4D {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl From<AiColor4D> for [f32; 4] {
    fn from(val: AiColor4D) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}
impl From<[f32; 3]> for AiColor4D {
    fn from(value: [f32; 3]) -> Self {
        AiColor4D {
            r: value[0],
            g: value[1],
            b: value[2],
            a: 1.0,
        }
    }
}

impl From<AiColor4D> for [f32; 3] {
    fn from(val: AiColor4D) -> Self {
        [val.r, val.g, val.b]
    }
}

impl AiColor4D {
    pub fn is_black(&self) -> bool {
        f32::abs(self.r) < EPSILON_F && f32::abs(self.g) < EPSILON_F && f32::abs(self.b) < EPSILON_F
    }
    pub fn new(pr: f32, pg: f32, pb: f32, pa: f32) -> Self {
        AiColor4D {
            r: pr,
            g: pg,
            b: pb,
            a: pa,
        }
    }
}
