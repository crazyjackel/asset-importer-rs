use std::{collections::HashMap, path::Path};

use gltf_v1::{buffer, document::Document, json::map::IndexMap};

use asset_importer_rs_scene::{AiTexel, AiTexture, AiTextureFormat};

use super::{GltfImporter, error::GLTFImportError};

impl GltfImporter {
    pub(crate) fn import_embedded_textures(
        document: &Document,
        base: Option<&Path>,
        buffer_data: &IndexMap<String, buffer::Data>,
    ) -> Result<(Vec<AiTexture>, HashMap<String, usize>), GLTFImportError> {
        let mut textures: Vec<AiTexture> = Vec::new();
        let mut embedded_tex_ids: HashMap<String, usize> = HashMap::new();
        for image in document.images() {
            let filename = image.name().unwrap_or(image.index());
            let source = image.source();
            let ach_format_hint = match source.mime_type() {
                Some(mime_type) => match mime_type {
                    "image/jpeg" => AiTextureFormat::JPEG,
                    "image/png" => AiTextureFormat::PNG,
                    "image/bmp" => AiTextureFormat::BMP,
                    "image/gif" => AiTextureFormat::GIF,
                    _ => AiTextureFormat::Unknown,
                },
                None => AiTextureFormat::Unknown,
            };
            let data: gltf_v1::image::Data =
                gltf_v1::image::Data::from_source(image.source(), base, buffer_data)
                    .map_err(GLTFImportError::FileFormatError)?;

            let texels = get_texels(&data);
            textures.push(AiTexture::new(
                filename.to_string(),
                data.width,
                data.height,
                ach_format_hint,
                texels,
            ));
            embedded_tex_ids.insert(image.index().to_string(), textures.len() - 1);
        }
        Ok((textures, embedded_tex_ids))
    }
}

fn get_texels(data: &gltf_v1::image::Data) -> Vec<AiTexel> {
    let texels: Vec<AiTexel> = match data.format {
        gltf_v1::image::Format::R8 => data
            .pixels
            .iter()
            .map(|r| AiTexel::new(*r, *r, *r, 255))
            .collect(),
        gltf_v1::image::Format::R8G8 => data
            .pixels
            .chunks_exact(2)
            .map(|x| AiTexel::new(x[0], x[1], 0, 255))
            .collect(),
        gltf_v1::image::Format::R8G8B8 => data
            .pixels
            .chunks_exact(3)
            .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], 255))
            .collect(),
        gltf_v1::image::Format::R8G8B8A8 => data
            .pixels
            .chunks_exact(4)
            .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect(),
        gltf_v1::image::Format::R16 => {
            data.pixels
                .chunks_exact(2)
                .map(|chunk| {
                    let r = chunk[0]; // Take the most significant byte
                    AiTexel::new(r, r, r, 255)
                })
                .collect()
        }
        gltf_v1::image::Format::R16G16 => data
            .pixels
            .chunks_exact(4)
            .map(|chunk| AiTexel::new(chunk[0], chunk[2], 0, 255))
            .collect(),
        gltf_v1::image::Format::R16G16B16 => data
            .pixels
            .chunks_exact(6)
            .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], 255))
            .collect(),
        gltf_v1::image::Format::R16G16B16A16 => data
            .pixels
            .chunks_exact(8)
            .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], chunk[6]))
            .collect(),
        gltf_v1::image::Format::R32G32B32FLOAT => data
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
        gltf_v1::image::Format::R32G32B32A32FLOAT => data
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
    texels
}

#[test]
fn test_gltf_texture_import() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let exe_path = binding.as_path();

    let gltf_data = r#"{
            "images": {
                "UnicodeTexture": {
                    "name": "UnicodeTexture",
                    "uri": "tests/textures/Unicode❤♻Texture.png"
                }
            }
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let (embedded_textures, _tex_ids) =
        GltfImporter::import_embedded_textures(&document, Some(exe_path), &IndexMap::new())
            .unwrap();
    assert_eq!(1, embedded_textures.len());
}
