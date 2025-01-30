use std::collections::HashMap;

use gltf::json::{
    buffer::View,
    image::MimeType,
    texture::{Info, Sampler},
    validation::{Checked, USize64}, Image, Index, Material, Root, Texture,
};

use crate::{
    core::error::AiExportError,
    structs::{matkey::{self, AI_MATKEY_BASE_COLOR, AI_MATKEY_COLOR_DIFFUSE, _AI_MATKEY_MAPPINGMODE_U_BASE, _AI_MATKEY_MAPPINGMODE_V_BASE}, scene::AiScene, AiColor4D, AiMaterial, AiPropertyTypeInfo, AiTextureType},
};

use super::{
    gltf2_exporter::{generate_unique_name, Gltf2Exporter, Output},
    gltf2_importer_material::{AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE, _AI_MATKEY_GLTF_MAPPINGFILTER_MAG_BASE, _AI_MATKEY_GLTF_MAPPINGFILTER_MIN_BASE, _AI_MATKEY_GLTF_MAPPINGID_BASE, _AI_MATKEY_GLTF_MAPPINGNAME_BASE},
};

impl Gltf2Exporter {
    pub(crate) fn export_materials(
        &self,
        scene: &AiScene,
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        mut unique_names_map: &mut HashMap<String, u32>
    ) -> Result<(), AiExportError> {
        let mut texture_name_to_index_map : HashMap<String, u32> = HashMap::new();
        for ai_material in &scene.materials {
            let mut material = Material::default();
            material.name = if let Some(AiPropertyTypeInfo::Binary(binary)) = ai_material
                .get_property_type_info(matkey::AI_MATKEY_NAME, Some(AiTextureType::None), 0)
            {
                let str = String::from_utf8(binary.to_vec())
                    .map_err(|err| AiExportError::ConversionError(Box::new(err)))?;
                Some(generate_unique_name(&str, &mut unique_names_map))
            } else {
                Some(generate_unique_name("material", &mut unique_names_map))
            };

            material.pbr_metallic_roughness.base_color_texture = 
                get_material_texture(scene, ai_material, root, AiTextureType::BaseColor, &mut texture_name_to_index_map, &mut unique_names_map, 0, self.output_type == Output::Binary, buffer_data)
                .or(get_material_texture(scene, ai_material, root, AiTextureType::Diffuse, &mut texture_name_to_index_map, &mut unique_names_map, 0, self.output_type == Output::Binary, buffer_data));

            material.pbr_metallic_roughness.metallic_roughness_texture =
                get_material_texture(scene, ai_material, root, AiTextureType::DiffuseRoughness, &mut texture_name_to_index_map, &mut unique_names_map, 0, self.output_type == Output::Binary, buffer_data)
                .or(get_material_texture(scene, ai_material, root, AiTextureType::Metalness, &mut texture_name_to_index_map, &mut unique_names_map, 0, self.output_type == Output::Binary, buffer_data))
                .or(get_material_texture(scene, ai_material, root, AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE, &mut texture_name_to_index_map, &mut unique_names_map, 0, self.output_type == Output::Binary, buffer_data));

            material.pbr_metallic_roughness.base_color_factor = ai_material
                .get_property_type_info(AI_MATKEY_BASE_COLOR, Some(AiTextureType::None), 0)
                .and_then(|info| match info {
                    AiPropertyTypeInfo::Binary(binary) => bytemuck::try_from_bytes::<AiColor4D>(&binary).ok().map(|x| gltf::json::material::PbrBaseColorFactor(Into::<[f32;4]>::into(*x))),
                    _ => None,
                })
                .or(ai_material
                    .get_property_type_info(AI_MATKEY_COLOR_DIFFUSE, Some(AiTextureType::None), 0)
                    .and_then(|info| match info {
                        AiPropertyTypeInfo::Binary(binary) => bytemuck::try_from_bytes::<AiColor4D>(&binary).ok().map(|x| gltf::json::material::PbrBaseColorFactor(Into::<[f32;4]>::into(*x))),
                        _ => None,
                    })).unwrap_or_default();
    
            
            root.materials.push(material);
        }
        Ok(())
    }
}

fn get_material_texture(
    ai_scene: &AiScene,
    ai_material: &AiMaterial,
    root: &mut Root,
    texture_type: AiTextureType,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    mut unique_names_map: &mut HashMap<String, u32>,
    index: u32,
    is_binary: bool,
    buffer: &mut Vec<u8>,
) -> Option<Info> {
    //Get Texture Coordinate
    let tex_coord = ai_material
        .get_property_type_info(matkey::_AI_MATKEY_UVWSRC_BASE, Some(texture_type), index)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => Some(u32::from_le_bytes([
                binary[0], binary[1], binary[2], binary[3],
            ])),
            _ => None,
        })
        .unwrap_or(0);

    let result = ai_material
        .get_property_type_info(matkey::_AI_MATKEY_TEXTURE_BASE, Some(texture_type), index)
        .and_then(|property_info| match property_info {
            AiPropertyTypeInfo::Binary(binary) => String::from_utf8(binary.to_vec()).ok(),
            _ => None,
        })
        .and_then(|texture_name| {
            texture_name_to_index_map
                .get(&texture_name)
                .copied()
                .or_else(|| {
                    ai_scene
                        .textures
                        .iter()
                        .find(|texture| texture.filename == texture_name)
                        .map(|ai_texture| {
                            //Handle Image
                            //If GLTF2 Exporter is binary, update the buffers, if not images will be exported based on Uri Later
                            let image = if is_binary {
                                let buffer_offset = buffer.len();
                                let mut exported_buffer = ai_texture.export();
                                let length = exported_buffer.len();
                                buffer.append(&mut exported_buffer);
                                let buffer_view = View {
                                    buffer: Index::new(0), //Body Buffer will be 0.bin
                                    byte_length: USize64(length as u64),
                                    byte_offset: Some(USize64(buffer_offset as u64)),
                                    byte_stride: None,
                                    name: Some(generate_unique_name(
                                        "imgdata",
                                        &mut unique_names_map,
                                    )),
                                    target: None,
                                    extensions: Default::default(),
                                    extras: Default::default(),
                                };
                                Image {
                                    buffer_view: Some(root.push(buffer_view)),
                                    mime_type: Some(MimeType(
                                        ai_texture.ach_format_hint.get_mime_type(),
                                    )),
                                    name: Some(ai_texture.filename.clone()),
                                    uri: None,
                                    extensions: Default::default(),
                                    extras: Default::default(),
                                }
                            } else {
                                Image {
                                    buffer_view: None,
                                    mime_type: Some(MimeType(
                                        ai_texture.ach_format_hint.get_mime_type(),
                                    )),
                                    name: Some(ai_texture.filename.clone()),
                                    uri: Some(format!(
                                        "{}.{}",
                                        ai_texture.filename,
                                        ai_texture.ach_format_hint.get_extension()
                                    )),
                                    extensions: Default::default(),
                                    extras: Default::default(),
                                }
                            };

                            //handle sampler
                            let sampler = ai_material
                                .get_property_type_info(
                                    _AI_MATKEY_GLTF_MAPPINGID_BASE,
                                    Some(texture_type),
                                    index,
                                )
                                .and_then(|property_type| match property_type {
                                    AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                                        Some(
                                            u32::from_le_bytes([
                                                binary[0], binary[1], binary[2], binary[3],
                                            ])
                                            .to_string(),
                                        )
                                    }
                                    _ => None,
                                })
                                .and_then(|name| {
                                    if let Some((index, _)) = root
                                        .samplers
                                        .iter()
                                        .enumerate()
                                        .find(|(_, sampler)| sampler.name == Some(name.clone()))
                                    {
                                        Some(Index::new(index as u32))
                                    } else {
                                        Some(root.push(Sampler {
                                            mag_filter: ai_material.get_property_type_info(_AI_MATKEY_GLTF_MAPPINGFILTER_MAG_BASE, 
                                                Some(texture_type), 0).and_then(|x|{
                                                    match x{
                                                        AiPropertyTypeInfo::Binary(items) if items.len() == 1 => match items[0]{
                                                            2 => Some(Checked::Valid(gltf::texture::MagFilter::Linear)),
                                                            1 => Some(Checked::Valid(gltf::texture::MagFilter::Nearest)),
                                                            _ => None,
                                                        },
                                                        _ => None
                                                    }
                                                }),
                                            min_filter: ai_material.get_property_type_info(_AI_MATKEY_GLTF_MAPPINGFILTER_MIN_BASE, 
                                                Some(texture_type), 0).and_then(|x|{
                                                    match x{
                                                        AiPropertyTypeInfo::Binary(items) if items.len() == 1 => match items[0]{
                                                            2 => Some(Checked::Valid(gltf::texture::MinFilter::Linear)),
                                                            1 => Some(Checked::Valid(gltf::texture::MinFilter::Nearest)),
                                                            _ => None,
                                                        },
                                                        _ => None
                                                    }
                                                }),
                                            name: ai_material.get_property_type_info(_AI_MATKEY_GLTF_MAPPINGNAME_BASE, 
                                                Some(texture_type), 0).and_then(|x|{
                                                    match x{
                                                        AiPropertyTypeInfo::Binary(items) => String::from_utf8(items.to_vec()).ok(),
                                                        _ => None
                                                    }
                                                }),
                                            wrap_s: ai_material.get_property_type_info(_AI_MATKEY_MAPPINGMODE_U_BASE, 
                                                Some(texture_type), 0).and_then(|x|{
                                                    match x{
                                                        AiPropertyTypeInfo::Binary(items) if items.len() == 1 => match items[0]{
                                                            1 => Some(Checked::Valid(gltf::texture::WrappingMode::ClampToEdge)),
                                                            2 => Some(Checked::Valid(gltf::texture::WrappingMode::MirroredRepeat)),
                                                            3 => Some(Checked::Valid(gltf::texture::WrappingMode::Repeat)),
                                                            _ => None,
                                                        },
                                                        _ => None
                                                    }
                                                }).unwrap_or(Checked::Valid(gltf::texture::WrappingMode::ClampToEdge)),
                                            wrap_t: ai_material.get_property_type_info(_AI_MATKEY_MAPPINGMODE_V_BASE, 
                                                Some(texture_type), 0).and_then(|x|{
                                                    match x{
                                                        AiPropertyTypeInfo::Binary(items) if items.len() == 1 => match items[0]{
                                                            1 => Some(Checked::Valid(gltf::texture::WrappingMode::ClampToEdge)),
                                                            2 => Some(Checked::Valid(gltf::texture::WrappingMode::MirroredRepeat)),
                                                            3 => Some(Checked::Valid(gltf::texture::WrappingMode::Repeat)),
                                                            _ => None,
                                                        },
                                                        _ => None
                                                    }
                                                }).unwrap_or(Checked::Valid(gltf::texture::WrappingMode::ClampToEdge)),
                                            extensions: Default::default(),
                                            extras: Default::default(),
                                        }))
                                    }
                                });

                            let texture = Texture {
                                name: Some(ai_texture.filename.clone()),
                                sampler: sampler,
                                source: root.push(image),
                                extensions: Default::default(),
                                extras: Default::default(),
                            };
                            let value = root.push(texture).value() as u32;
                            texture_name_to_index_map.insert(texture_name, value);
                            value
                        })
                })
        });

    let texture_info = Info {
        index: Index::new(result.unwrap()),
        tex_coord,
        extensions: Default::default(),
        extras: Default::default(),
    };
    Some(texture_info)
}
