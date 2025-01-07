use super::type_def::base_types::AiReal;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AiQuaternion{
    w: AiReal,
    x: AiReal,
    y: AiReal,
    z: AiReal
}

impl Default for AiQuaternion{
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default(), w: 1.0 }
    }
}

impl AiQuaternion{
    fn new(pw: AiReal, px: AiReal, py: AiReal, pz: AiReal) -> Self{
        Self { x: px, y: py, z: pz, w: pw }
    }
}