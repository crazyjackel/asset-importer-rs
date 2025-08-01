use std::io::{self, Read, Seek};

use std::path::Path;

use gltf::{Document, Gltf, buffer};

use asset_importer_rs_core::{AiImporter, AiImporterDesc, AiImporterFlags, AiImporterInfo};
use asset_importer_rs_scene::{AiMaterial, AiScene, AiSceneFlag};

use crate::importer::error::Gltf2ImportError;

#[derive(Debug, Default)]
pub struct Gltf2Importer;

impl Gltf2Importer {
    pub fn new() -> Self {
        Self
    }

    pub fn import_buffers<R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        document: &Document,
        base: Option<&Path>,
        loader: &F,
        mut blob: Option<Vec<u8>>,
    ) -> Result<Vec<buffer::Data>, gltf::Error> {
        //Goes through Document Loading Buffers
        let mut buffers: Vec<buffer::Data> = Vec::new();
        for buffer in document.buffers() {
            let mut data: Vec<u8> = match buffer.source() {
                buffer::Source::Uri(uri) => {
                    let data: Result<Vec<u8>, gltf::Error> = if uri.contains(':') {
                        if let Some(rest) = uri.strip_prefix("data:") {
                            let mut it = rest.split(";base64,");

                            match (it.next(), it.next()) {
                                (_, Some(match1)) => base64::decode(match1)
                                    .map_err(|arg0: base64::DecodeError| gltf::Error::Base64(arg0)),
                                (Some(match0), _) => {
                                    base64::decode(match0).map_err(gltf::Error::Base64)
                                }
                                _ => Err(gltf::Error::UnsupportedScheme),
                            }
                        } else if base.is_some() {
                            if let Some(rest) = uri.strip_prefix("file://") {
                                let mut reader = loader(rest.as_ref()).map_err(gltf::Error::Io)?;
                                let mut data = Vec::new();
                                reader.read_to_end(&mut data).map_err(gltf::Error::Io)?;
                                Ok(data)
                            } else if let Some(rest) = uri.strip_prefix("file:") {
                                let mut reader = loader(rest.as_ref()).map_err(gltf::Error::Io)?;
                                let mut data = Vec::new();
                                reader.read_to_end(&mut data).map_err(gltf::Error::Io)?;
                                Ok(data)
                            } else {
                                Err(gltf::Error::UnsupportedScheme)
                            }
                        } else {
                            Err(gltf::Error::UnsupportedScheme)
                        }
                    } else if base.is_some() {
                        let mut reader =
                            loader(base.unwrap().join(uri).as_ref()).map_err(gltf::Error::Io)?;
                        let mut data = Vec::new();
                        reader.read_to_end(&mut data).map_err(gltf::Error::Io)?;
                        Ok(data)
                    } else {
                        Err(gltf::Error::UnsupportedScheme)
                    };
                    data
                }
                buffer::Source::Bin => blob.take().ok_or(gltf::Error::MissingBlob),
            }?;
            while data.len() % 4 != 0 {
                data.push(0);
            }
            if data.len() < buffer.length() {
                return Err(gltf::Error::BufferLength {
                    buffer: buffer.index(),
                    expected: buffer.length(),
                    actual: data.len(),
                });
            }
            buffers.push(buffer::Data(data));
        }
        Ok(buffers)
    }
}

impl AiImporterInfo for Gltf2Importer {
    fn info(&self) -> AiImporterDesc {
        AiImporterDesc {
            name: "glTF2 Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: Default::default(),
            flags: (AiImporterFlags::SupportBinaryFlavor
                | AiImporterFlags::LimitedSupport
                | AiImporterFlags::SupportTextFlavor
                | AiImporterFlags::Experimental),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["gltf".to_string(), "glb".to_string(), "vrm".to_string()],
        }
    }
}

impl AiImporter for Gltf2Importer {
    type Error = Gltf2ImportError;

    fn can_read_dyn<'data>(
        &self,
        path: &Path,
        loader: &dyn Fn(&Path) -> io::Result<Box<dyn asset_importer_rs_core::ReadSeek + 'data>>,
    ) -> bool {
        //Match Extension Guard Clause
        match path.extension() {
            None => {
                return false;
            }
            Some(os_str) => match os_str.to_str() {
                Some("gltf") => {}
                Some("glb") => {}
                Some("vrm") => {}
                Some(_) | None => return false,
            },
        };

        let reader_result = loader(path.as_ref());
        //Check if File can be Opened
        if reader_result.is_err() {
            return false;
        }

        //Attempt to Read JSON
        let reader = reader_result.unwrap();
        let gltf = Gltf::from_reader(reader);

        //If Result is Good, we can Read
        gltf.is_ok()
    }

    fn read_file_dyn<'data>(
        &self,
        path: &Path,
        loader: &dyn Fn(&Path) -> io::Result<Box<dyn asset_importer_rs_core::ReadSeek + 'data>>,
    ) -> Result<AiScene, Gltf2ImportError> {
        //Collect File Info
        let base = path.parent().unwrap_or_else(|| Path::new("./"));
        let reader =
            loader(path).map_err(|x| Gltf2ImportError::FileOpenError(x, path.to_path_buf()))?;

        //Load Gltf Info
        let Gltf { document, blob } =
            Gltf::from_reader(reader).map_err(Gltf2ImportError::FileFormatError)?;

        //@todo: Buffer Data loads all Buffer Data, it would be better to load on an "as-needed case".
        let buffer_data = Gltf2Importer::import_buffers(&document, Some(base), &loader, blob)
            .map_err(Gltf2ImportError::FileFormatError)?;

        //import textures
        let (embedded_textures, embedded_tex_ids) =
            Gltf2Importer::import_embedded_textures(&document, Some(base), &loader, &buffer_data)?;
        //import materials
        let mut embedded_materials =
            Gltf2Importer::import_embedded_materials(&document, &embedded_tex_ids)?;
        //add default material
        embedded_materials.push(AiMaterial::default());
        //import meshes
        let (mut meshes, mesh_offsets, remapping_tables) =
            Gltf2Importer::import_meshes(&document, &buffer_data, embedded_materials.len() - 1)?;

        //import cameras
        let mut cameras = Gltf2Importer::import_cameras(&document)?;
        //import lights
        let mut lights = Gltf2Importer::import_lights(&document)?;

        //import nodes
        let (nodes, scene_name) = Gltf2Importer::import_nodes(
            &document,
            &buffer_data,
            &mut meshes,
            &mesh_offsets,
            &remapping_tables,
            &mut lights,
            &mut cameras,
        )?;

        //import animations
        let animations = Gltf2Importer::import_animations(&document, &buffer_data)?;

        //import metadata
        let metadata = Gltf2Importer::import_metadata(&document)?;

        let mut scene = AiScene {
            name: scene_name,
            animations,
            cameras,
            meshes,
            lights,
            materials: embedded_materials,
            textures: embedded_textures,
            nodes,
            metadata,
            ..AiScene::default()
        };

        if !scene.meshes.is_empty() {
            scene.flags |= AiSceneFlag::Incomplete;
        }

        Ok(scene)
    }
}
