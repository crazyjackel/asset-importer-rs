use std::collections::HashMap;

use gltf::{
    json::{
        Image, Index, Material, Root, Texture,
        buffer::View,
        extensions::material::Material as ExtensionMaterial,
        image::MimeType,
        material::{AlphaCutoff, EmissiveFactor, NormalTexture, OcclusionTexture, StrengthFactor},
        texture::{Info, Sampler},
        validation::{Checked, USize64},
    },
    material::AlphaMode,
};

use asset_importer_rs_core::AiExportError;
use asset_importer_rs_scene::{
    AiColor3D, AiColor4D, AiMaterial, AiPropertyTypeInfo, AiScene, AiShadingMode, AiTextureType,
    matkey,
};
use bytemuck::try_from_bytes;
use matkey::{
    _AI_MATKEY_MAPPINGMODE_U_BASE, _AI_MATKEY_MAPPINGMODE_V_BASE, _AI_MATKEY_TEXTURE_BASE,
    AI_MATKEY_BASE_COLOR, AI_MATKEY_COLOR_DIFFUSE, AI_MATKEY_COLOR_EMISSIVE,
    AI_MATKEY_COLOR_SPECULAR, AI_MATKEY_METALLIC_FACTOR, AI_MATKEY_OPACITY,
    AI_MATKEY_ROUGHNESS_FACTOR, AI_MATKEY_SHADING_MODEL, AI_MATKEY_SHININESS, AI_MATKEY_TWOSIDED,
};

use super::{
    gltf2_exporter::{APPROVED_FORMATS, Gltf2Exporter, Output, generate_unique_name},
    gltf2_importer_material::{
        _AI_MATKEY_GLTF_MAPPINGFILTER_MAG_BASE, _AI_MATKEY_GLTF_MAPPINGFILTER_MIN_BASE,
        _AI_MATKEY_GLTF_MAPPINGID_BASE, _AI_MATKEY_GLTF_MAPPINGNAME_BASE,
        _AI_MATKEY_GLTF_SCALE_BASE, AI_MATKEY_GLTF_ALPHACUTOFF,
        AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE,
    },
};
impl Gltf2Exporter {
    pub(crate) fn export_materials(
        &self,
        scene: &AiScene,
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        unique_names_map: &mut HashMap<String, u32>,
        use_gltf_pbr_specular_glossiness: bool,
    ) -> Result<(), AiExportError> {
        let mut texture_name_to_index_map: HashMap<String, u32> = HashMap::new();
        for ai_material in &scene.materials {
            let name = if let Some(AiPropertyTypeInfo::Binary(binary)) = ai_material
                .get_property_type_info(matkey::AI_MATKEY_NAME, Some(AiTextureType::None), 0)
            {
                let str = String::from_utf8(binary.to_vec())
                    .map_err(|err| AiExportError::ConversionError(Box::new(err)))?;
                Some(generate_unique_name(&str, unique_names_map))
            } else {
                Some(generate_unique_name("material", unique_names_map))
            };

            let mut material = Material {
                name,
                ..Material::default()
            };
            handle_pbr(
                scene,
                root,
                buffer_data,
                unique_names_map,
                &mut texture_name_to_index_map,
                ai_material,
                &mut material,
                self.output_type == Output::Binary,
            );

            handle_base(
                scene,
                root,
                buffer_data,
                unique_names_map,
                &mut texture_name_to_index_map,
                ai_material,
                &mut material,
                self.output_type == Output::Binary,
            );

            if use_gltf_pbr_specular_glossiness {
                root.extensions_used
                    .push("KHR_materials_pbrSpecularGlossiness".to_string());
                handle_specular_glossiness(
                    scene,
                    root,
                    buffer_data,
                    unique_names_map,
                    &mut texture_name_to_index_map,
                    ai_material,
                    &mut material,
                    self.output_type == Output::Binary,
                );
            }

            let shading = ai_material
                .get_property_byte(AI_MATKEY_SHADING_MODEL, Some(AiTextureType::None), 0)
                .and_then(|byte| AiShadingMode::try_from(byte).ok())
                .unwrap_or(AiShadingMode::PBR);

            if shading == AiShadingMode::Unlit {
                //handle unlit shading
                root.extensions_used.push("KHR_materials_unlit".to_string());
            } else {
                //handle everything else
                let extensions = material
                    .extensions
                    .get_or_insert(ExtensionMaterial::default());
                #[cfg(not(feature = "KHR_materials_pbrSpecularGlossiness"))]
                let has_specular_glossiness = false;
                #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
                let has_specular_glossiness = extensions.pbr_specular_glossiness.is_some();
                if !has_specular_glossiness {
                    //handle specular
                    if handle_specular(
                        scene,
                        root,
                        buffer_data,
                        unique_names_map,
                        &mut texture_name_to_index_map,
                        ai_material,
                        extensions,
                        self.output_type == Output::Binary,
                    ) {
                        root.extensions_used
                            .push("KHR_materials_specular".to_string());
                        if let Some(color) = ai_material
                            .get_property_ai_color_rgba(
                                AI_MATKEY_COLOR_DIFFUSE,
                                Some(AiTextureType::None),
                                0,
                            )
                            .map(|x| {
                                gltf::json::material::PbrBaseColorFactor(Into::<[f32; 4]>::into(x))
                            })
                        {
                            material.pbr_metallic_roughness.base_color_factor = color;
                        }
                    }
                    //handle sheen
                    //handle clearcoat

                    //handle transmission
                    if handle_transmission(
                        scene,
                        root,
                        buffer_data,
                        unique_names_map,
                        &mut texture_name_to_index_map,
                        ai_material,
                        extensions,
                        self.output_type == Output::Binary,
                    ) {
                        root.extensions_used
                            .push("KHR_materials_transmission".to_string());
                    }
                    //handle volume
                    if handle_volume(
                        scene,
                        root,
                        buffer_data,
                        unique_names_map,
                        &mut texture_name_to_index_map,
                        ai_material,
                        extensions,
                        self.output_type == Output::Binary,
                    ) {
                        root.extensions_used
                            .push("KHR_materials_volume".to_string());
                    }
                    //handle ior
                    if handle_ior(ai_material, extensions) {
                        root.extensions_used.push("KHR_materials_ior".to_string());
                    }
                    //handle emissive strength
                    if handle_emissive_strength(ai_material, extensions) {
                        root.extensions_used
                            .push("KHR_materials_emissive_strength".to_string());
                    }

                    //handle anisotropy
                }
            }

            root.materials.push(material);
        }
        Ok(())
    }
}

fn get_material_texture_normal(
    ai_scene: &AiScene,
    ai_material: &AiMaterial,
    root: &mut Root,
    texture_type: AiTextureType,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    unique_names_map: &mut HashMap<String, u32>,
    index: u32,
    is_binary: bool,
    buffer: &mut Vec<u8>,
) -> Option<NormalTexture> {
    get_material_texture(
        ai_scene,
        ai_material,
        root,
        texture_type,
        texture_name_to_index_map,
        unique_names_map,
        index,
        is_binary,
        buffer,
    )
    .map(|x| {
        let scale = ai_material
            .get_property_type_info(_AI_MATKEY_GLTF_SCALE_BASE, Some(texture_type), index)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                    Some(f32::from_le_bytes([
                        binary[0], binary[1], binary[2], binary[3],
                    ]))
                }
                _ => None,
            })
            .unwrap_or(1.0);
        NormalTexture {
            index: x.index,
            scale,
            tex_coord: x.tex_coord,
            extensions: Default::default(),
            extras: x.extras,
        }
    })
}

fn get_material_texture_occlusion(
    ai_scene: &AiScene,
    ai_material: &AiMaterial,
    root: &mut Root,
    texture_type: AiTextureType,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    unique_names_map: &mut HashMap<String, u32>,
    index: u32,
    is_binary: bool,
    buffer: &mut Vec<u8>,
) -> Option<OcclusionTexture> {
    get_material_texture(
        ai_scene,
        ai_material,
        root,
        texture_type,
        texture_name_to_index_map,
        unique_names_map,
        index,
        is_binary,
        buffer,
    )
    .map(|x| {
        let texture_strength_key = format!("{}.strength", _AI_MATKEY_TEXTURE_BASE);
        let strength = ai_material
            .get_property_type_info(&texture_strength_key, Some(texture_type), index)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                    Some(f32::from_le_bytes([
                        binary[0], binary[1], binary[2], binary[3],
                    ]))
                }
                _ => None,
            })
            .unwrap_or(1.0);
        OcclusionTexture {
            index: x.index,
            tex_coord: x.tex_coord,
            extensions: Default::default(),
            extras: x.extras,
            strength: StrengthFactor(strength),
        }
    })
}

fn get_material_texture(
    ai_scene: &AiScene,
    ai_material: &AiMaterial,
    root: &mut Root,
    texture_type: AiTextureType,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    unique_names_map: &mut HashMap<String, u32>,
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
                        .find(|texture| texture_name.ends_with(&texture.filename)) //Texture Name is in Format *Name
                        .map(|ai_texture| {
                            //Handle Image
                            //If GLTF2 Exporter is binary, update the buffers, if not images will be exported based on Uri Later
                            let image = if is_binary {
                                let buffer_offset = buffer.len();
                                let export = ai_texture.export(APPROVED_FORMATS).unwrap();
                                let mut exported_buffer = export.data;
                                let length = exported_buffer.len();
                                buffer.append(&mut exported_buffer);
                                let buffer_view = View {
                                    buffer: Index::new(0), //Body Buffer will be 0.bin
                                    byte_length: USize64(length as u64),
                                    byte_offset: Some(USize64(buffer_offset as u64)),
                                    byte_stride: None,
                                    name: Some(generate_unique_name(
                                        "imgdata",
                                        unique_names_map,
                                    )),
                                    target: None,
                                    extensions: Default::default(),
                                    extras: Default::default(),
                                };
                                let format = ai_texture.get_approved_format(APPROVED_FORMATS);
                                Image {
                                    buffer_view: Some(root.push(buffer_view)),
                                    mime_type: Some(MimeType(
                                        format.get_mime_type(),
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
                                .map(|name| {
                                    if let Some((index, _)) = root
                                        .samplers
                                        .iter()
                                        .enumerate()
                                        .find(|(_, sampler)| sampler.name == Some(name.clone()))
                                    {
                                        Index::new(index as u32)
                                    } else {
                                        root.push(Sampler {
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
                                        })
                                    }
                                });

                            let texture = Texture {
                                name: Some(ai_texture.filename.clone()),
                                sampler,
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
        index: Index::new(result?),
        tex_coord,
        extensions: Default::default(),
        extras: Default::default(),
    };
    Some(texture_info)
}

fn handle_pbr(
    scene: &AiScene,
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    mut unique_names_map: &mut HashMap<String, u32>,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    ai_material: &AiMaterial,
    material: &mut Material,
    is_binary: bool,
) {
    material.pbr_metallic_roughness.base_color_texture = get_material_texture(
        scene,
        ai_material,
        root,
        AiTextureType::BaseColor,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    )
    .or(get_material_texture(
        scene,
        ai_material,
        root,
        AiTextureType::Diffuse,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    ));

    material.pbr_metallic_roughness.metallic_roughness_texture = get_material_texture(
        scene,
        ai_material,
        root,
        AiTextureType::DiffuseRoughness,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    )
    .or(get_material_texture(
        scene,
        ai_material,
        root,
        AiTextureType::Metalness,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    ))
    .or(get_material_texture(
        scene,
        ai_material,
        root,
        AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    ));

    material.pbr_metallic_roughness.base_color_factor = ai_material
        .get_property_type_info(AI_MATKEY_BASE_COLOR, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) => try_from_bytes::<AiColor4D>(binary)
                .ok()
                .map(|x| gltf::json::material::PbrBaseColorFactor(Into::<[f32; 4]>::into(*x))),
            _ => None,
        })
        .or(ai_material
            .get_property_type_info(AI_MATKEY_COLOR_DIFFUSE, Some(AiTextureType::None), 0)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) => try_from_bytes::<AiColor4D>(binary)
                    .ok()
                    .map(|x| gltf::json::material::PbrBaseColorFactor(Into::<[f32; 4]>::into(*x))),
                _ => None,
            }))
        .unwrap_or_default();

    material.pbr_metallic_roughness.metallic_factor = ai_material
        .get_property_type_info(AI_MATKEY_METALLIC_FACTOR, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                Some(StrengthFactor(f32::from_le_bytes([
                    binary[0], binary[1], binary[2], binary[3],
                ])))
            }
            _ => None,
        })
        .unwrap_or(StrengthFactor(0.0));

    material.pbr_metallic_roughness.roughness_factor = ai_material
        .get_property_type_info(AI_MATKEY_ROUGHNESS_FACTOR, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                Some(StrengthFactor(f32::from_le_bytes([
                    binary[0], binary[1], binary[2], binary[3],
                ])))
            }
            _ => None,
        })
        .or_else(|| {
            if let Some(shininess) = ai_material
                .get_property_type_info(AI_MATKEY_SHININESS, Some(AiTextureType::None), 0)
                .and_then(|info| match info {
                    AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                        Some(StrengthFactor(f32::from_le_bytes([
                            binary[0], binary[1], binary[2], binary[3],
                        ])))
                    }
                    _ => None,
                })
            {
                if let Some(specular) = ai_material
                    .get_property_type_info(AI_MATKEY_COLOR_SPECULAR, Some(AiTextureType::None), 0)
                    .and_then(|info| match info {
                        AiPropertyTypeInfo::Binary(binary) => {
                            try_from_bytes::<AiColor3D>(binary).ok()
                        }
                        _ => None,
                    })
                {
                    let specular_intensity =
                        specular.r * 0.2125 + specular.g * 0.2125 + specular.b * 0.2125;
                    let mut normalized_shininess = f32::sqrt(shininess.0 / 1000.0);
                    normalized_shininess = normalized_shininess.clamp(0.0, 1.0);
                    normalized_shininess *= specular_intensity;
                    return Some(StrengthFactor(1.0 - normalized_shininess));
                }
            }
            None
        })
        .unwrap_or(StrengthFactor(0.0));
}

fn handle_base(
    scene: &AiScene,
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    unique_names_map: &mut HashMap<String, u32>,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    ai_material: &AiMaterial,
    material: &mut Material,
    is_binary: bool,
) {
    material.normal_texture = get_material_texture_normal(
        scene,
        ai_material,
        root,
        AiTextureType::Normals,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    );
    material.occlusion_texture = get_material_texture_occlusion(
        scene,
        ai_material,
        root,
        AiTextureType::Normals,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    );
    material.emissive_texture = get_material_texture(
        scene,
        ai_material,
        root,
        AiTextureType::Normals,
        texture_name_to_index_map,
        unique_names_map,
        0,
        is_binary,
        buffer_data,
    );

    material.emissive_factor = ai_material
        .get_property_type_info(AI_MATKEY_COLOR_EMISSIVE, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) => try_from_bytes::<AiColor3D>(binary)
                .ok()
                .map(|x| EmissiveFactor([x.r, x.g, x.b])),
            _ => None,
        })
        .unwrap_or_default();

    material.double_sided = ai_material
        .get_property_type_info(AI_MATKEY_TWOSIDED, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) if binary.len() == 1 => Some(binary[0] == 1),
            _ => None,
        })
        .unwrap_or_default();
    material.alpha_cutoff = ai_material
        .get_property_type_info(AI_MATKEY_GLTF_ALPHACUTOFF, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                Some(AlphaCutoff(f32::from_le_bytes([
                    binary[0], binary[1], binary[2], binary[3],
                ])))
            }
            _ => None,
        });

    if let Some(opacity) = ai_material
        .get_property_type_info(AI_MATKEY_OPACITY, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => Some(f32::from_le_bytes([
                binary[0], binary[1], binary[2], binary[3],
            ])),
            _ => None,
        })
    {
        material.alpha_mode = Checked::Valid(AlphaMode::Blend);
        material.pbr_metallic_roughness.base_color_factor.0[3] *= opacity;
    }

    material.alpha_mode = ai_material
        .get_property_type_info(AI_MATKEY_GLTF_ALPHACUTOFF, Some(AiTextureType::None), 0)
        .and_then(|info| match info {
            AiPropertyTypeInfo::Binary(binary) => match String::from_utf8(binary.to_vec()).ok() {
                Some(str) => match str.as_str() {
                    "OPAQUE" => Some(Checked::Valid(AlphaMode::Opaque)),
                    "BLEND" => Some(Checked::Valid(AlphaMode::Blend)),
                    "MASK" => Some(Checked::Valid(AlphaMode::Mask)),
                    _ => None,
                },
                None => None,
            },
            _ => None,
        })
        .unwrap_or(material.alpha_mode);
}

#[cfg(not(feature = "KHR_materials_pbrSpecularGlossiness"))]
fn handle_specular_glossiness(
    _scene: &AiScene,
    _root: &mut Root,
    _buffer_data: &mut Vec<u8>,
    _unique_names_map: &mut HashMap<String, u32>,
    _texture_name_to_index_map: &mut HashMap<String, u32>,
    _ai_material: &AiMaterial,
    _material: &mut Material,
    _is_binary: bool,
) {
}
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
fn handle_specular_glossiness(
    scene: &AiScene,
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    unique_names_map: &mut HashMap<String, u32>,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    ai_material: &AiMaterial,
    material: &mut Material,
    is_binary: bool,
) {
    use gltf::json::extensions::material::{
        PbrDiffuseFactor, PbrSpecularFactor, PbrSpecularGlossiness,
    };

    let extensions = material
        .extensions
        .get_or_insert(ExtensionMaterial::default());
    extensions.pbr_specular_glossiness = Some(PbrSpecularGlossiness {
        diffuse_factor: ai_material
            .get_property_ai_color_rgba(AI_MATKEY_COLOR_DIFFUSE, Some(AiTextureType::None), 0)
            .map(|x| PbrDiffuseFactor([x.r, x.g, x.b, x.a]))
            .unwrap_or_default(),
        diffuse_texture: get_material_texture(
            scene,
            ai_material,
            root,
            AiTextureType::Diffuse,
            texture_name_to_index_map,
            unique_names_map,
            0,
            is_binary,
            buffer_data,
        ),
        specular_factor: ai_material
            .get_property_ai_color_rgb(AI_MATKEY_COLOR_DIFFUSE, Some(AiTextureType::None), 0)
            .map(|x| PbrSpecularFactor([x.r, x.g, x.b]))
            .unwrap_or_default(),
        glossiness_factor: ai_material
            .get_property_ai_float(
                matkey::AI_MATKEY_GLOSSINESS_FACTOR,
                Some(AiTextureType::None),
                0,
            )
            .map(|x| StrengthFactor(x))
            .or(ai_material
                .get_property_ai_float(
                    matkey::AI_MATKEY_ROUGHNESS_FACTOR,
                    Some(AiTextureType::None),
                    0,
                )
                .map(|x| StrengthFactor(1.0 - x)))
            .or(ai_material
                .get_property_ai_float(matkey::AI_MATKEY_SHININESS, Some(AiTextureType::None), 0)
                .map(|x| StrengthFactor(x / 1000.0)))
            .unwrap_or_default(),
        specular_glossiness_texture: get_material_texture(
            scene,
            ai_material,
            root,
            AiTextureType::Specular,
            texture_name_to_index_map,
            unique_names_map,
            0,
            is_binary,
            buffer_data,
        ),
        others: Default::default(),
        extras: Default::default(),
    });
}

#[cfg(not(feature = "KHR_materials_specular"))]
fn handle_specular(
    _scene: &AiScene,
    _root: &mut Root,
    _buffer_data: &mut Vec<u8>,
    _unique_names_map: &mut HashMap<String, u32>,
    _texture_name_to_index_map: &mut HashMap<String, u32>,
    _ai_material: &AiMaterial,
    _material: &mut ExtensionMaterial,
    _is_binary: bool,
) -> bool {
    false
}
#[cfg(feature = "KHR_materials_specular")]
fn handle_specular(
    scene: &AiScene,
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    unique_names_map: &mut HashMap<String, u32>,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    ai_material: &AiMaterial,
    material: &mut ExtensionMaterial,
    is_binary: bool,
) -> bool {
    use gltf::json::extensions::material::{Specular, SpecularColorFactor, SpecularFactor};

    use matkey::AI_MATKEY_SPECULAR_FACTOR;

    let color_specular_option = ai_material.get_property_ai_color_rgb(
        AI_MATKEY_COLOR_SPECULAR,
        Some(AiTextureType::None),
        0,
    );
    if color_specular_option.is_none() {
        return false;
    }
    let color_specular = color_specular_option.unwrap();
    let specular_factory_option =
        ai_material.get_property_ai_float(AI_MATKEY_SPECULAR_FACTOR, Some(AiTextureType::None), 0);
    if specular_factory_option.is_none() {
        return false;
    }
    let mut specular_factory = specular_factory_option.unwrap();

    let color_factor_is_zero = color_specular == AiColor3D::new(1.0, 1.0, 1.0);
    if specular_factory == 0.0 && color_factor_is_zero {
        return false;
    } else if specular_factory == 0.0 {
        specular_factory = 1.0;
    }

    material.specular = Some(Specular {
        specular_factor: SpecularFactor(specular_factory),
        specular_texture: get_material_texture(
            scene,
            ai_material,
            root,
            AiTextureType::Specular,
            texture_name_to_index_map,
            unique_names_map,
            0,
            is_binary,
            buffer_data,
        ),
        specular_color_factor: SpecularColorFactor([
            color_specular.r,
            color_specular.g,
            color_specular.b,
        ]),
        specular_color_texture: get_material_texture(
            scene,
            ai_material,
            root,
            AiTextureType::Specular,
            texture_name_to_index_map,
            unique_names_map,
            1,
            is_binary,
            buffer_data,
        ),
        extras: Default::default(),
    });
    true
}

#[cfg(not(feature = "KHR_materials_transmission"))]
fn handle_transmission(
    _scene: &AiScene,
    _root: &mut Root,
    _buffer_data: &mut Vec<u8>,
    _unique_names_map: &mut HashMap<String, u32>,
    _texture_name_to_index_map: &mut HashMap<String, u32>,
    _ai_material: &AiMaterial,
    _material: &mut ExtensionMaterial,
    _is_binary: bool,
) -> bool {
    false
}
#[cfg(feature = "KHR_materials_transmission")]
fn handle_transmission(
    scene: &AiScene,
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    unique_names_map: &mut HashMap<String, u32>,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    ai_material: &AiMaterial,
    material: &mut ExtensionMaterial,
    is_binary: bool,
) -> bool {
    use gltf::json::extensions::material::{Transmission, TransmissionFactor};

    use matkey::{AI_MATKEY_TRANSMISSION_FACTOR, AI_MATKEY_TRANSMISSION_TEXTURE};

    let transmission_factor_option = ai_material.get_property_ai_float(
        AI_MATKEY_TRANSMISSION_FACTOR,
        Some(AiTextureType::None),
        0,
    );
    if transmission_factor_option.is_none() {
        return false;
    }
    let transmission_factor = transmission_factor_option.unwrap();

    material.transmission = Some(Transmission {
        transmission_factor: TransmissionFactor(transmission_factor),
        transmission_texture: get_material_texture(
            scene,
            ai_material,
            root,
            AI_MATKEY_TRANSMISSION_TEXTURE,
            texture_name_to_index_map,
            unique_names_map,
            0,
            is_binary,
            buffer_data,
        ),
        extras: Default::default(),
    });
    true
}

#[cfg(not(feature = "KHR_materials_volume"))]
fn handle_volume(
    _scene: &AiScene,
    _root: &mut Root,
    _buffer_data: &mut Vec<u8>,
    _unique_names_map: &mut HashMap<String, u32>,
    _texture_name_to_index_map: &mut HashMap<String, u32>,
    _ai_material: &AiMaterial,
    _material: &mut ExtensionMaterial,
    _is_binary: bool,
) -> bool {
    false
}
#[cfg(feature = "KHR_materials_volume")]
fn handle_volume(
    scene: &AiScene,
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    unique_names_map: &mut HashMap<String, u32>,
    texture_name_to_index_map: &mut HashMap<String, u32>,
    ai_material: &AiMaterial,
    material: &mut ExtensionMaterial,
    is_binary: bool,
) -> bool {
    use gltf::json::extensions::material::{
        AttenuationColor, AttenuationDistance, ThicknessFactor, Volume,
    };

    use matkey::{
        AI_MATKEY_VOLUME_ATTENUATION_COLOR, AI_MATKEY_VOLUME_ATTENUATION_DISTANCE,
        AI_MATKEY_VOLUME_THICKNESS_FACTOR, AI_MATKEY_VOLUME_THICKNESS_TEXTURE,
    };

    let thickness_option = ai_material.get_property_ai_float(
        AI_MATKEY_VOLUME_THICKNESS_FACTOR,
        Some(AiTextureType::None),
        0,
    );
    if thickness_option.is_none() {
        return false;
    }
    let thickness = thickness_option.unwrap();

    let attenuation_distance_option = ai_material.get_property_ai_float(
        AI_MATKEY_VOLUME_ATTENUATION_DISTANCE,
        Some(AiTextureType::None),
        0,
    );
    if attenuation_distance_option.is_none() {
        return false;
    }
    let attenuation_distance = attenuation_distance_option.unwrap();

    let attenuation_color_option = ai_material.get_property_ai_color_rgb(
        AI_MATKEY_VOLUME_ATTENUATION_COLOR,
        Some(AiTextureType::None),
        0,
    );
    if attenuation_color_option.is_none() {
        return false;
    }
    let attenuation_color = attenuation_color_option.unwrap();

    material.volume = Some(Volume {
        thickness_factor: ThicknessFactor(thickness),
        thickness_texture: get_material_texture(
            scene,
            ai_material,
            root,
            AI_MATKEY_VOLUME_THICKNESS_TEXTURE,
            texture_name_to_index_map,
            unique_names_map,
            0,
            is_binary,
            buffer_data,
        ),
        attenuation_distance: AttenuationDistance(attenuation_distance),
        attenuation_color: AttenuationColor(attenuation_color.into()),
        extras: Default::default(),
    });
    true
}

#[cfg(not(feature = "KHR_materials_ior"))]
fn handle_ior(_ai_material: &AiMaterial, _material: &mut ExtensionMaterial) -> bool {
    false
}
#[cfg(feature = "KHR_materials_ior")]
fn handle_ior(ai_material: &AiMaterial, material: &mut ExtensionMaterial) -> bool {
    use gltf::json::extensions::material::{IndexOfRefraction, Ior};

    use matkey::AI_MATKEY_REFRACTI;

    material.ior = ai_material
        .get_property_ai_float(AI_MATKEY_REFRACTI, Some(AiTextureType::None), 0)
        .map(|x| Ior {
            ior: IndexOfRefraction(x),
            extras: Default::default(),
        });
    true
}

#[cfg(not(feature = "KHR_materials_emissive_strength"))]
fn handle_emissive_strength(_ai_material: &AiMaterial, _material: &mut ExtensionMaterial) -> bool {
    false
}
#[cfg(feature = "KHR_materials_emissive_strength")]
fn handle_emissive_strength(ai_material: &AiMaterial, material: &mut ExtensionMaterial) -> bool {
    use gltf::json::extensions::material::{EmissiveStrength, EmissiveStrengthFactor};

    use matkey::AI_MATKEY_EMISSIVE_INTENSITY;

    material.emissive_strength = ai_material
        .get_property_ai_float(AI_MATKEY_EMISSIVE_INTENSITY, Some(AiTextureType::None), 0)
        .map(|x| EmissiveStrength {
            emissive_strength: EmissiveStrengthFactor(x),
        });
    true
}
