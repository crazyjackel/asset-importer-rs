use std::{collections::HashMap, path::Path};

use asset_importer_rs_core::{
    AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON, AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON_DEFAULT,
    AI_METADATA_SOURCE_COPYRIGHT, AiExport, AiExportError, DataExporter, ExportProperties,
    ExportProperty,
};
use asset_importer_rs_scene::{AiMetadataEntry, AiScene};

mod error;
mod material;
mod mesh;
mod node;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum Output {
    /// Output standard glTF.
    #[default]
    Standard,

    /// Output binary glTF.
    Binary,
}

#[derive(Debug, Default)]
pub struct GltfExporter {
    pub output_type: Output,
}

impl GltfExporter {
    pub fn new(output_type: Output) -> Self {
        Self { output_type }
    }
}

impl AiExport for GltfExporter {
    fn export_file_dyn(
        &self,
        scene: &AiScene,
        path: &Path,
        properties: &ExportProperties,
        exporter: &DataExporter<'_>,
    ) -> Result<(), AiExportError> {
        //@TODO: GLTF should always have large meshes split on export.
        let mut body_buffer_data: Vec<u8> = Vec::new();
        let mut unique_names_map: HashMap<String, u32> = HashMap::new();

        let mut root = gltf_v1::json::Root {
            asset: Some(gltf_v1::json::Asset {
                version: "1.0".to_string(),
                generator: Some(format!(
                    "{} {}",
                    "Asset Importer RS",
                    env!("CARGO_PKG_VERSION")
                )),
                copyright: scene
                    .metadata
                    .get(AI_METADATA_SOURCE_COPYRIGHT)
                    .and_then(|entry| {
                        if let AiMetadataEntry::AiStr(s) = entry {
                            Some(s.clone())
                        } else {
                            None
                        }
                    }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let material_index_map = self
            .export_materials(scene, &mut root, &mut unique_names_map)
            .map_err(|e| AiExportError::ConversionError(Box::new(e)))?;

        let config_epsilon = properties
            .get(AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON)
            .and_then(|prop| {
                if let ExportProperty::Real(r) = prop {
                    Some(*r)
                } else {
                    None
                }
            })
            .unwrap_or(AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON_DEFAULT);

        let mesh_index_map = self
            .export_nodes(scene, &mut root, &mut unique_names_map, config_epsilon)
            .map_err(|e| AiExportError::ConversionError(Box::new(e)))?;

        self.export_meshes(
            scene,
            &mut root,
            &mut unique_names_map,
            &mut body_buffer_data,
            &mesh_index_map,
            &material_index_map,
        )
        .map_err(|e| AiExportError::ConversionError(Box::new(e)))?;

        Ok(())
    }
}

pub(crate) fn generate_unique_name(
    base_name: &str,
    unique_names_map: &mut HashMap<String, u32>,
) -> String {
    let mut unique_name = base_name.to_string();

    if !unique_names_map.contains_key(&unique_name) {
        unique_names_map.insert(unique_name.clone(), 0);
        return unique_name;
    }

    loop {
        let counter = unique_names_map.get_mut(&unique_name).unwrap();
        unique_name = format!("{}_{}", base_name, counter);
        *counter += 1;

        // Ensure the generated name is truly unique in the map
        if !unique_names_map.contains_key(&unique_name) {
            return unique_name;
        }
    }
}
