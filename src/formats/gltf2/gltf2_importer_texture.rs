use std::{collections::HashMap, path::Path};

use gltf::{buffer, image, Document};

use crate::{
    core::error::AiReadError,
    structs::{AiTexel, AiTexture, AiTextureFormat},
};

use super::gltf2_importer::Gltf2Importer;

impl Gltf2Importer {
    pub(crate) fn import_embedded_textures(
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

            //@todo: add better mime_type predictions
            //format is used for exporting from texels
            let format: AiTextureFormat = match mime_type {
                Some(mime_type_str) => match mime_type_str {
                    "image/jpeg" => AiTextureFormat::JPEG,
                    "image/png" => AiTextureFormat::PNG,
                    "image/webp" => AiTextureFormat::WEBP,
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
                texels,
            ));
            embedded_tex_ids.insert(image.index(), textures.len() - 1);
        }
        Ok((textures, embedded_tex_ids))
    }
}

#[test]
fn test_texture_import() {
    let binding = std::env::current_dir()
        .expect("Failed to get the current executable path");
    let exe_path = binding
        .as_path();

    let gltf_data = 
        r#"{
            "asset": {
                "version": "2.0"
            },
            "images": [
                {
                    "uri": "tests/textures/Unicode❤♻Texture.png"
                }
            ],
            "textures": [
                {
                    "source": 0
                }
            ]
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let (embedded_textures, _tex_ids) = Gltf2Importer::import_embedded_textures(&document, Some(exe_path), &[]).unwrap();
    assert_eq!(1, embedded_textures.len());
}
