use std::{borrow::Cow, collections::HashMap, path::Path};

use asset_importer_rs_core::{
    AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON, AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON_DEFAULT,
    AI_METADATA_SOURCE_COPYRIGHT, AiExport, AiExportError, DataExporter, ExportProperties,
    ExportProperty,
};
use asset_importer_rs_scene::{AiMetadataEntry, AiScene};
use gltf_v1::{
    Glb,
    binary::Header,
    json::{Buffer, Scene, StringIndex, buffer::BufferType, validation::Checked},
};

mod anim;
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
            .export_materials(scene, &mut root)
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
            .export_nodes(scene, &mut root, config_epsilon)
            .map_err(|e| AiExportError::ConversionError(Box::new(e)))?;

        self.export_meshes(
            scene,
            &mut root,
            &mut body_buffer_data,
            &mesh_index_map,
            &material_index_map,
        )
        .map_err(|e| AiExportError::ConversionError(Box::new(e)))?;

        //export animations
        self.export_animations(scene, &mut root, &mut body_buffer_data)
            .map_err(|e| AiExportError::ConversionError(Box::new(e)))?;

        //export scene
        let scene_name = scene.name.clone();
        let nodes = if let Some((index, _)) = root.nodes.first() {
            vec![StringIndex::new(index.clone())]
        } else {
            vec![]
        };
        let root_scene = Scene {
            nodes,
            name: Some(scene_name.clone()),
        };
        root.scenes.insert(scene_name.clone(), root_scene);
        root.scene = Some(StringIndex::new(scene_name));

        match self.output_type {
            Output::Standard => {
                let bin = path.with_extension("bin");
                let bin_uri = bin
                    .file_name()
                    .and_then(|x| x.to_os_string().into_string().ok())
                    .unwrap_or("0.bin".to_string());
                root.buffers.insert(
                    "body".to_string(),
                    Buffer {
                        uri: bin_uri.to_string(),
                        byte_length: body_buffer_data.len().into(),
                        name: Some("body".to_string()),
                        type_: Some(Checked::Valid(BufferType::ArrayBuffer)),
                    },
                );

                let mut writer = exporter(bin.as_path())
                    .map_err(|x| AiExportError::FileWriteError(Box::new(x)))?;
                writer
                    .write_all(&body_buffer_data)
                    .map_err(|x| AiExportError::FileWriteError(Box::new(x)))?;

                let gltf = path.with_extension("gltf");
                let writer = exporter(gltf.as_path())
                    .map_err(|x| AiExportError::FileWriteError(Box::new(x)))?;
                serde_json::to_writer_pretty(writer, &root)
                    .map_err(|x| AiExportError::FileWriteError(Box::new(x)))?;
            }
            Output::Binary => {
                let length = body_buffer_data.len();
                //We might need to pad the bin with some extra elements to align to multiples of 4 bytes
                let bin = body_buffer_data;
                for (_, buffer) in root.buffers.iter_mut() {
                    buffer.uri = "binary_glTF".to_string();
                }
                //Prepare Final Buffer
                root.buffers.insert(
                    "binary_glTF".to_string(),
                    Buffer {
                        uri: "data:,".to_string(),
                        byte_length: length.into(),
                        name: Some("binary_glTF".to_string()),
                        type_: Some(Checked::Valid(BufferType::ArrayBuffer)),
                    },
                );

                let json_string = serde_json::to_string(&root)
                    .map_err(|x| AiExportError::FileWriteError(Box::new(x)))?;

                let glb = Glb {
                    header: Header::default(), //Header is calculated by 'to_writer'
                    content: Cow::Owned(json_string.into_bytes()),
                    body: Some(Cow::Owned(bin)),
                };

                let glb_path: std::path::PathBuf = path.with_extension("glb");
                let writer = exporter(glb_path.as_path())
                    .map_err(|x| AiExportError::FileWriteError(Box::new(x)))?;
                glb.to_writer(writer)
                    .map_err(|x| AiExportError::FileWriteError(Box::new(x)))?;
            }
        }
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
