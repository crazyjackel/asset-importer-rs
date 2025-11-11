#[allow(unused_imports)] // different features use different imports
use super::light::Lights;
use gltf_v1_derive::Validate;
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    #[cfg(feature = "KHR_materials_common")]
    #[serde(rename = "KHR_materials_common")]
    pub ktr_materials_common: Option<Lights>,
    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}
