use std::ops;

use super::type_def::base_types::AiReal;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AiQuaternion {
    pub w: AiReal,
    pub x: AiReal,
    pub y: AiReal,
    pub z: AiReal,
}

impl Default for AiQuaternion {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: 1.0,
        }
    }
}

impl From<AiQuaternion> for [AiReal; 4] {
    fn from(val: AiQuaternion) -> Self {
        [val.w, val.x, val.y, val.z]
    }
}

impl From<[AiReal; 4]> for AiQuaternion {
    fn from(value: [AiReal; 4]) -> Self {
        AiQuaternion {
            w: value[0],
            x: value[1],
            y: value[2],
            z: value[3],
        }
    }
}

impl AiQuaternion {
    pub fn new(pw: AiReal, px: AiReal, py: AiReal, pz: AiReal) -> Self {
        Self {
            x: px,
            y: py,
            z: pz,
            w: pw,
        }
    }
}

impl ops::Index<u32> for AiQuaternion {
    type Output = AiReal;

    fn index(&self, index: u32) -> &Self::Output {
        match index {
            0 => &self.w,
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            _ => &self.w,
        }
    }
}

impl ops::Index<usize> for AiQuaternion {
    type Output = AiReal;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.w,
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            _ => &self.w,
        }
    }
}
