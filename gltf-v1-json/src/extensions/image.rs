#[allow(unused_imports)] // different features use different imports
use crate::{validation::USize64, BufferView, StringIndex};
use gltf_v1_derive::Validate;
use serde::{Deserialize, Serialize};
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

#[cfg(feature = "KHR_binary_glTF")]
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BinaryImage {
    #[serde(rename = "bufferView")]
    pub buffer_view: StringIndex<BufferView>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub width: USize64,
    pub height: USize64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Image {
    #[cfg(feature = "KHR_binary_glTF")]
    #[serde(rename = "KHR_binary_glTF")]
    pub khr_binary_gltf: Option<BinaryImage>,
    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}
