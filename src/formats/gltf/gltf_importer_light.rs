use std::collections::HashMap;

use gltf_v1::Document;

use crate::{core::error::AiReadError, structs::AiLight};

use super::gltf_importer::GltfImporter;

pub struct ImportLights(pub Vec<AiLight>, pub HashMap<String, usize>);
impl GltfImporter {
    pub(crate) fn import_lights(document: &Document) -> Result<ImportLights, AiReadError> {
        //@todo: Handle KHR_materials_common ext for lights
        Ok(ImportLights(Vec::new(), HashMap::new()))
    }
}
