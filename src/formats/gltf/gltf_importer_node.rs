use gltf_v1::{buffer::Data, json::map::IndexMap, Document};

use crate::{core::error::AiReadError, structs::AiNodeTree};

use super::gltf_importer::GltfImporter;

impl GltfImporter {
    pub(crate) fn import_nodes(
        document: &Document,
        buffer_data: &IndexMap<String, Data>,
    ) -> Result<(AiNodeTree, String), AiReadError> {
        let scene = document
            .default_scene()
            .or_else(|| document.scenes().nth(0));
        if scene.is_none() {
            return Ok((AiNodeTree::default(), "".to_string()));
        }

        Ok((AiNodeTree::default(), "".to_string()))
    }
}
