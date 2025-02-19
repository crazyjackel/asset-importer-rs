use gltf_v1::Document;

use crate::{core::error::AiReadError, structs::AiLight};

use super::gltf_importer::GltfImporter;

impl GltfImporter {
    pub(crate) fn import_lights(document: &Document) -> Result<Vec<AiLight>, AiReadError> {
        //@todo: Handle KHR_materials_common ext for lights
        Ok(Vec::new())
    }
}
