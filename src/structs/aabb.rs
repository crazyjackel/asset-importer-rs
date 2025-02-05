use super::vector::AiVector3D;


#[derive(Debug, PartialEq, Clone)]
pub struct AiAABB{
    min: AiVector3D,
    max: AiVector3D
}

impl Default for AiAABB{
    fn default() -> Self {
        Self { min: Default::default(), max: Default::default() }
    }
}