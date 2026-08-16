use std::collections::HashMap;

use asset_importer_rs_scene::{
    AiMaterial, AiPropertyTypeInfo, AiTextureType, matkey::AI_MATKEY_NAME,
};
use dae_parser::{Document, Material};

use crate::DaeImportError;

use super::DaeImporter;

pub(crate) fn get_material_name(material: &Material) -> Option<String> {
    material
        .name
        .as_ref()
        .or(material.id.as_ref())
        .map(|name| name.clone())
}

impl DaeImporter {
    pub(crate) fn import_materials(
        document: &Document,
    ) -> Result<(Vec<AiMaterial>, HashMap<String, usize>), DaeImportError> {
        let mut materials = Vec::new();
        let mut material_index_map: HashMap<String, usize> = HashMap::new();
        let document_local_map = document
            .local_map()
            .map_err(DaeImportError::FileFormatError)?;
        let library_materials = document.library_iter::<Material>();
        for library in library_materials {
            materials.reserve(library.items.len());
            for (index, material) in library.items.iter().enumerate() {
                let mut ai_material = AiMaterial::new();
                // Handle Name
                let name: String = get_material_name(material).unwrap_or(index.to_string());
                ai_material.add_property(
                    AI_MATKEY_NAME,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary,
                    0,
                    name.bytes().collect(),
                );

                //Handle

                let instance_effect = document_local_map
                    .get(&material.instance_effect.url)
                    .ok_or(DaeImportError::MissingLocalMapEntry(
                        material.instance_effect.url.to_string(),
                    ))?;

                materials.push(ai_material);
                material_index_map.insert(name, index);
            }
        }
        Ok((materials, material_index_map))
    }
}
