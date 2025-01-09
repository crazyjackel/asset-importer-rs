use std::{collections::HashMap, fs, io::BufReader, path::Path};

use enumflags2::BitFlags;
use gltf::{buffer, image, Document, Gltf};

use crate::{
    core::{
        error::AiReadError,
        import::AiImport,
        importer::AiImporter,
        importer_desc::{AiImporterDesc, AiImporterFlags},
    },
    structs::{scene::AiScene, AiMaterial, AiTexel, AiTexture, AiTextureFormat},
};

#[derive(Debug)]
pub struct Gltf2Importer;

impl AiImport for Gltf2Importer {
    fn info(&self) -> AiImporterDesc {
        AiImporterDesc {
            name: "glTF2 Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: Default::default(),
            flags: BitFlags::from(
                AiImporterFlags::SupportBinaryFlavor
                    | AiImporterFlags::LimitedSupport
                    | AiImporterFlags::SupportTextFlavor
                    | AiImporterFlags::Experimental,
            ),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["gltf".to_string(), "glb".to_string(), "vrm".to_string()],
        }
    }

    fn can_read<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        //Match Extension Guard Clause
        match path.as_ref().extension() {
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
        //Check if File can be Opened
        let file_result = fs::File::open(path);
        if file_result.is_err() {
            return false;
        }

        //Attempt to Read JSON
        let file = file_result.unwrap();
        let reader = BufReader::new(file);
        let gltf = Gltf::from_reader(reader);

        //If Result is Good, we can Read
        !gltf.is_err()
    }

    fn read_file<P>(&self, importer: &mut AiImporter, path: P) -> Result<AiScene, AiReadError>
    where
        P: AsRef<std::path::Path>,
    {
        //Collect File Info
        let base = path.as_ref().parent().unwrap_or_else(|| Path::new("./"));
        let file_result =
            fs::File::open(path).map_err(|x| AiReadError::FileOpenError(Box::new(x)))?;
        let reader = BufReader::new(file_result);

        //Load Gltf Info
        let Gltf { document, blob } =
            Gltf::from_reader(reader).map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        //@todo: Buffer Data loads all Buffer Data, it would be better to load on an "as-needed case".
        let buffer_data = gltf::import_buffers(&document, Some(base), blob)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        let (embedded_textures, embedded_tex_ids) = Gltf2Importer::import_embedded_textures(&document, Some(base), &buffer_data)?;
        

        Ok(AiScene {})
    }
}

impl Gltf2Importer {
    fn import_embedded_materials(
        document: &Document,
        embedded_tex_ids: &HashMap<usize, usize>
    ) -> Result<Vec<AiMaterial>, AiReadError>{
        let mut materials: Vec<AiMaterial> = Vec::new();
        for material in document.materials(){
            let ai_material = AiMaterial::new();
            if let Some(name) = material.name(){
                ai_material.add_property()
            }
        }
        Ok(materials)
    }

    fn import_embedded_textures(
        document: &Document,
        base: Option<&Path>,
        buffer_data: &[buffer::Data],
    ) -> Result<(Vec<AiTexture>, HashMap<usize, usize>), AiReadError> {
        let mut textures: Vec<AiTexture> = Vec::new();
        let mut embedded_tex_ids: HashMap<usize, usize> = HashMap::new();
        for image in document.images() {
            let source = image.source();
            let (filename, mime_type) = match source {
                image::Source::View { view: _, mime_type } => {
                    (image.index().to_string(), Some(mime_type))
                }
                image::Source::Uri { uri, mime_type } => (
                    if let Some(pos) = uri.find('.') {
                        &uri[..pos]
                    } else {
                        uri
                    }
                    .to_string(),
                    mime_type,
                ),
            };
            let format = match mime_type {
                Some(mime_type_str) => match mime_type_str {
                    "image/jpeg" => AiTextureFormat::JPEG,
                    "image/png" => AiTextureFormat::Png,
                    _ => AiTextureFormat::Unknown,
                },
                None => AiTextureFormat::Unknown,
            };

            let data: image::Data = image::Data::from_source(image.source(), base, buffer_data)
                .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

            let texels: Vec<AiTexel> = match data.format {
                image::Format::R8 => data
                    .pixels
                    .into_iter()
                    .map(|r| AiTexel::new(r, r, r, 255))
                    .collect(),
                image::Format::R8G8 => data
                    .pixels
                    .chunks_exact(2)
                    .map(|x| AiTexel::new(x[0], x[1], 0, 255))
                    .collect(),
                image::Format::R8G8B8 => data
                    .pixels
                    .chunks_exact(3)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], 255))
                    .collect(),
                image::Format::R8G8B8A8 => data
                    .pixels
                    .chunks_exact(4)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], chunk[3]))
                    .collect(),
                image::Format::R16 => {
                    data.pixels
                        .chunks_exact(2)
                        .map(|chunk| {
                            let r = chunk[0]; // Take the most significant byte
                            AiTexel::new(r, r, r, 255)
                        })
                        .collect()
                }
                image::Format::R16G16 => data
                    .pixels
                    .chunks_exact(4)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], 0, 255))
                    .collect(),
                image::Format::R16G16B16 => data
                    .pixels
                    .chunks_exact(6)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], 255))
                    .collect(),
                image::Format::R16G16B16A16 => data
                    .pixels
                    .chunks_exact(8)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], chunk[6]))
                    .collect(),
                image::Format::R32G32B32FLOAT => data
                    .pixels
                    .chunks_exact(12)
                    .map(|chunk| {
                        let r = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
                        let g = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
                        let b = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
                        AiTexel::new(
                            (r.clamp(0.0, 1.0) * 255.0) as u8,
                            (g.clamp(0.0, 1.0) * 255.0) as u8,
                            (b.clamp(0.0, 1.0) * 255.0) as u8,
                            255,
                        )
                    })
                    .collect(),
                image::Format::R32G32B32A32FLOAT => data
                    .pixels
                    .chunks_exact(16)
                    .map(|chunk| {
                        let r = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
                        let g = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
                        let b = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
                        let a = f32::from_le_bytes(chunk[12..16].try_into().unwrap());
                        AiTexel::new(
                            (r.clamp(0.0, 1.0) * 255.0) as u8,
                            (g.clamp(0.0, 1.0) * 255.0) as u8,
                            (b.clamp(0.0, 1.0) * 255.0) as u8,
                            (a.clamp(0.0, 1.0) * 255.0) as u8,
                        )
                    })
                    .collect(),
            };

            textures.push(AiTexture::new(
                filename,
                data.width,
                data.height,
                format,
                texels
            ));
            embedded_tex_ids.insert(image.index(), textures.len() - 1);
        }
        Ok((textures, embedded_tex_ids))
    }
}
