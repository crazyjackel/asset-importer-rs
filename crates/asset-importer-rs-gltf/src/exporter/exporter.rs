use std::{borrow::Cow, collections::HashMap, io::Write, path::Path};

use gltf::json::{Buffer, Index, Scene};

use asset_importer_rs_core::{
    AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON, AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON_DEFAULT,
    AI_CONFIG_EXPORT_GLTF_UNLIMITED_SKINNING_BONES_PER_VERTEX,
    AI_CONFIG_USE_GLTF_PBR_SPECULAR_GLOSSINESS, AiExport, DataExporter, ExportProperties,
    ExportProperty, GLTF2_NODE_IN_TRS, GLTF2_TARGET_NORMAL_EXP,
};

use asset_importer_rs_scene::{AiMetadataEntry, AiScene, AiTextureFormat};

use crate::{AI_METADATA_SOURCE_COPYRIGHT, exporter::error::Gltf2ExportError};

pub const APPROVED_FORMATS: &[AiTextureFormat] = &[AiTextureFormat::PNG, AiTextureFormat::JPEG];

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
pub struct Gltf2Exporter {
    pub output_type: Output,
}

impl Gltf2Exporter {
    pub fn new(output_type: Output) -> Self {
        Self { output_type }
    }
}

impl AiExport for Gltf2Exporter {
    type Error = Gltf2ExportError;

    fn export_file_dyn(
        &self,
        scene: &AiScene,
        path: &Path,
        properties: &ExportProperties,
        exporter: &DataExporter<'_>,
    ) -> Result<(), Self::Error> {
        //GLTF Root
        let mut root = gltf::json::Root::default();
        //Buffer Data for Accessors
        let mut body_buffer_data: Vec<u8> = Vec::new();
        //Map for Unique Name Generation
        let mut unique_names_map: HashMap<String, u32> = HashMap::new();

        root.extensions_used.push("FB_ngon_encoding".to_string());

        //Handle Metadata
        let asset = &mut root.asset;
        asset.version = "2.0".to_string();
        let version = env!("CARGO_PKG_VERSION");
        asset.generator = Some(format!("{} {}", "Asset Importer RS", version));
        if let Some(AiMetadataEntry::AiStr(copyright)) =
            scene.metadata.get(AI_METADATA_SOURCE_COPYRIGHT)
        {
            asset.copyright = Some(copyright.to_string());
        }

        //Handle Materials
        let use_gltf_pbr_specular_glossiness = if let Some(ExportProperty::Int(i)) =
            properties.get(AI_CONFIG_USE_GLTF_PBR_SPECULAR_GLOSSINESS)
        {
            i != &0
        } else {
            false
        };
        self.export_materials(
            scene,
            &mut root,
            &mut body_buffer_data,
            &mut unique_names_map,
            use_gltf_pbr_specular_glossiness,
        )?;

        //Handle Cameras
        let camera_name_to_index = self.export_cameras(scene, &mut root, &mut unique_names_map)?;

        //@todo: Handle Lights

        //Handle Nodes
        let config_epsilon = if let Some(ExportProperty::Real(r)) =
            properties.get(AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON)
        {
            *r
        } else {
            AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON_DEFAULT
        };
        let use_translation_rotation_scale =
            if let Some(ExportProperty::Int(i)) = properties.get(GLTF2_NODE_IN_TRS) {
                i != &0
            } else {
                false
            };
        let node_index_to_mesh_indexes = self.export_nodes(
            scene,
            &mut root,
            &mut unique_names_map,
            camera_name_to_index,
            config_epsilon,
            use_translation_rotation_scale,
        )?;

        //Handle Meshes
        let unlimited_bones_per_vertex = if let Some(ExportProperty::Int(i)) =
            properties.get(AI_CONFIG_EXPORT_GLTF_UNLIMITED_SKINNING_BONES_PER_VERTEX)
        {
            i != &0
        } else {
            false
        };
        let export_anim_normals =
            if let Some(ExportProperty::Int(i)) = properties.get(GLTF2_TARGET_NORMAL_EXP) {
                i != &0
            } else {
                false
            };
        self.export_meshes(
            scene,
            &mut root,
            &mut unique_names_map,
            &mut body_buffer_data,
            &node_index_to_mesh_indexes,
            unlimited_bones_per_vertex,
            export_anim_normals,
        )?;

        //Handle Scene
        let node = scene.nodes.root.unwrap_or(0);
        root.scene = Some(root.push(Scene {
            name: Some(scene.name.clone()),
            nodes: vec![Index::new(node as u32)],
            extensions: Default::default(),
            extras: Default::default(),
        }));

        //Handle Animations
        self.export_animations(
            scene,
            &mut root,
            &mut body_buffer_data,
            &mut unique_names_map,
        )?;

        //@todo: callback?

        match self.output_type {
            Output::Standard => {
                //Prepare Final Buffer
                let bin = path.with_extension("bin");
                let uri = bin
                    .file_name()
                    .and_then(|x| x.to_os_string().into_string().ok());
                root.push(Buffer {
                    byte_length: body_buffer_data.len().into(),
                    name: uri.clone(),
                    uri,
                    extensions: Default::default(),
                    extras: Default::default(),
                });

                //Export GLTF
                let gltf = path.with_extension("gltf");
                let writer = exporter(gltf.as_path()).map_err(Gltf2ExportError::FileOpen)?;
                serde_json::to_writer_pretty(writer, &root).map_err(Gltf2ExportError::Json)?;

                //Export Bin
                let mut writer = exporter(bin.as_path()).map_err(Gltf2ExportError::FileOpen)?;
                writer
                    .write_all(&body_buffer_data)
                    .map_err(Gltf2ExportError::FileOpen)?;

                //Export Textures
                for texture in &scene.textures {
                    let image = path.with_file_name(format!(
                        "{}.{}",
                        texture.filename,
                        texture.ach_format_hint.get_extension()
                    ));
                    let mut writer =
                        exporter(image.as_path()).map_err(Gltf2ExportError::FileOpen)?;
                    let export = &texture.export(APPROVED_FORMATS).unwrap();
                    writer
                        .write_all(&export.data)
                        .map_err(Gltf2ExportError::FileOpen)?;
                }
            }
            Output::Binary => {
                let length = body_buffer_data.len();
                //We might need to pad the bin with some extra elements to align to multiples of 4 bytes
                let bin = body_buffer_data;
                //Prepare Final Buffer
                root.push(Buffer {
                    byte_length: length.into(),
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: Default::default(),
                    uri: Default::default(),
                });

                let json_string = serde_json::to_string(&root).map_err(Gltf2ExportError::Json)?;
                let json_offset = json_string.len();

                let glb = gltf::binary::Glb {
                    header: gltf::binary::Header {
                        magic: *b"glTF",
                        version: 2,
                        // N.B., the size of binary glTF file is limited to range of `u32`.
                        length: (json_offset + length)
                            .try_into()
                            .map_err(Gltf2ExportError::IntConversion)?,
                    },
                    bin: Some(Cow::Owned(bin)),
                    json: Cow::Owned(json_string.into_bytes()),
                };

                let glb_path: std::path::PathBuf = path.with_extension("glb");
                let writer = exporter(glb_path.as_path()).map_err(Gltf2ExportError::FileOpen)?;
                glb.to_writer(writer)
                    .map_err(Gltf2ExportError::FileFormat)?;
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
