use asset_importer_rs_scene::AiAnimation;
use dae_parser::Document;

use crate::DaeImportError;

use super::DaeImporter;

impl DaeImporter {
    pub(crate) fn import_animations(
        &self,
        _document: &Document,
    ) -> Result<Vec<AiAnimation>, DaeImportError> {
        Ok(Vec::new())
    }
}
