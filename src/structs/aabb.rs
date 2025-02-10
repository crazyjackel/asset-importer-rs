use super::vector::AiVector3D;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct AiAABB{
    min: AiVector3D,
    max: AiVector3D
}