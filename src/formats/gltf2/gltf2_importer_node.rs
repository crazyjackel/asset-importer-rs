use gltf::Document;

use crate::{core::error::AiReadError, structs::AiNode};

use super::gltf2_importer::Gltf2Importer;

impl Gltf2Importer {
    pub(crate) fn import_nodes(document: &Document) -> Result<AiNode, AiReadError> {
        let mut default_scene = document.default_scene();
        if default_scene.is_none() {
            default_scene = document.scenes().next();
        }
        if default_scene.is_none() {
            return Ok(AiNode::default());
        }

        let asset_root_nodes: Vec<gltf::Node<'_>> = default_scene.unwrap().nodes().collect();
        return if asset_root_nodes.len() == 1 {
            return;
        } else if asset_root_nodes.len() > 1 {
            let ai_node = AiNode::default();
            ai_node.name = "ROOT".to_string();
            Ok(ai_node)
        } else {
            let ai_node = AiNode::default();
            ai_node.name = "ROOT".to_string();
            Ok(ai_node)
        };
    }
}
