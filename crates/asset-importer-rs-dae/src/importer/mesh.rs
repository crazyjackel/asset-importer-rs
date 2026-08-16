use asset_importer_rs_scene::AiNode;
use dae_parser::{Document, Node};

use crate::DaeImportError;

use super::DaeImporter;

impl DaeImporter {
    pub(crate) fn build_meshes_for_node(
        _document: &Document,
        _node: &Node,
        _ai_node: &mut AiNode,
    ) -> Result<(), DaeImportError> {
        Ok(())
    }
}
