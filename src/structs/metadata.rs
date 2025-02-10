use std::collections::HashMap;

use super::AiVector3D;

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