use std::collections::HashMap;

use super::AiVector3D;


// #[repr(u8)]
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub enum AiMetadataType{
//     AiBool = 0,
//     AiI32 = 1,
//     AiU64 = 2,
//     AiF32 = 3,
//     AiF64 = 4,
//     AiStr = 5,
//     AiVec3D = 6,
//     AiMetadata = 7,
//     AiI64 = 8,
//     AiU32 = 9,
//     AiMax = 10
// }

// pub trait GetAiType : Sized{
//     fn get_ai_type(&self) -> AiMetadataType;
// }

// impl GetAiType for bool{
//     fn get_ai_type(&self) -> AiMetadataType {
//         AiMetadataType::AiBool
//     }
// }

// impl GetAiType for i32 {
//     fn get_ai_type(&self) -> AiMetadataType {
//         AiMetadataType::AiI32
//     }
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct AiMetadataEntry{
//     metadata_type: AiMetadataType,
//     data: Vec<u8>
// }

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum AiMetadataEntry{
    AiBool(bool),
    AiI32(i32),
    AiU64(u64),
    AiF32(f32),
    AiF64(f64),
    AiStr(String),
    AiVec3D(AiVector3D),
    AiMetadata(HashMap<String, AiMetadataEntry>),
    AiI64(i64),
    AiU32(u32)
}

pub type AiMetadata = HashMap<String, AiMetadataEntry>;