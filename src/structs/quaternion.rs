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

impl Into<[f32; 4]> for AiQuaternion {
    fn into(self) -> [f32; 4] {
        [self.w as f32, self.x as f32, self.y as f32, self.z as f32]
    }
}

impl From<[f32; 4]> for AiQuaternion {
    fn from(value: [f32; 4]) -> Self {
        AiQuaternion {
            w: value[0] as AiReal,
            x: value[1] as AiReal,
            y: value[2] as AiReal,
            z: value[3] as AiReal,
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
