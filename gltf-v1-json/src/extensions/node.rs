#[allow(unused_imports)]
use super::light::Light;
#[allow(unused_imports)]
use crate::StringIndex;
#[allow(unused_imports)] // different features use different imports
use gltf_v1_derive::Validate;
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "KHR_materials_common")]
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct NodeLight {
    pub light: StringIndex<Light>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Node {
    #[cfg(feature = "KHR_materials_common")]
    #[serde(rename = "KHR_materials_common")]
    pub ktr_materials_common: Option<NodeLight>,
    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}
