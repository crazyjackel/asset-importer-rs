use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::Path,
};

use asset_importer_rs_core::{AiReadError, DataLoader, ReadSeek};
use asset_importer_rs_scene::{
    AiColor4D, AiMaterial, AiPropertyTypeInfo, AiScene, AiShadingMode, AiTexel, AiTexture,
    AiTextureFormat, AiTextureType,
    matkey::{
        _AI_MATKEY_TEXTURE_BASE, AI_MATKEY_COLOR_AMBIENT, AI_MATKEY_COLOR_DIFFUSE,
        AI_MATKEY_COLOR_SPECULAR, AI_MATKEY_NAME, AI_MATKEY_OBJ_ILLUM, AI_MATKEY_OPACITY,
        AI_MATKEY_REFRACTI, AI_MATKEY_SHADING_MODEL, AI_MATKEY_SHININESS,
    },
};
use image::ImageFormat;
use tobj::Material;

use crate::importer::ObjImporter;

impl ObjImporter {
    pub(crate) fn import_materials(
        path: &Path,
        materials: Vec<Material>,
        loader: &DataLoader<'_>,
    ) -> Result<(Vec<AiMaterial>, Vec<AiTexture>), AiReadError> {
        let mut ai_textures: Vec<AiTexture> = Vec::new();
        let mut ai_materials: Vec<AiMaterial> = Vec::with_capacity(materials.len());
        let mut textures: HashMap<String, usize> = HashMap::new();
        //Create Materials in Scene
        for material in &materials {
            //@todo: Work with tObj for missing properties:
            // - Emissive (map_emissive, map_Ke)
            // - Normal (map_Kn, norm)
            // - Reflection (refl)
            // - Displacement (map_disp, disp)
            // - Roughness (map_Pr)
            // - Metallic (map_Pm)
            // - Sheen (map_Ps)
            // - Roughness Factor (Pr)
            // - Metallic Factor (Pm)
            // - Sheen Factor (Ps)
            // - Clearcoat Roughness Factor (Pcr)
            // - Clearcoat Thickness Factor (Pct)
            // - Anisotropy Factor (a)
            // - Transmission Color (Tf)
            // - Tranmission Alpha (Tr)
            // - Emissive Factor (Ke)

            let mut ai_material = AiMaterial::new();

            ai_material.add_property(
                AI_MATKEY_NAME,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(material.name.bytes().collect()),
                0,
            );

            //Handle Ambient
            if let Some(color) = material.ambient {
                ai_material.add_property(
                    AI_MATKEY_COLOR_AMBIENT,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(
                        bytemuck::bytes_of(&AiColor4D::from(color)).to_vec(),
                    ),
                    0,
                );
            }

            if let Some(texture) = &material.ambient_texture {
                let (uri, ai_texture) = import_texture(path, loader, &mut textures, texture)?;
                if let Some(ai_texture) = ai_texture {
                    ai_textures.push(ai_texture);
                }

                ai_material.add_property(
                    _AI_MATKEY_TEXTURE_BASE,
                    Some(AiTextureType::Ambient),
                    AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                    0,
                );
            }

            //Handle Diffuse
            if let Some(color) = material.diffuse {
                ai_material.add_property(
                    AI_MATKEY_COLOR_DIFFUSE,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(
                        bytemuck::bytes_of(&AiColor4D::from(color)).to_vec(),
                    ),
                    0,
                );
            }

            if let Some(texture) = &material.diffuse_texture {
                let (uri, ai_texture) = import_texture(path, loader, &mut textures, texture)?;
                if let Some(ai_texture) = ai_texture {
                    ai_textures.push(ai_texture);
                }
                ai_material.add_property(
                    _AI_MATKEY_TEXTURE_BASE,
                    Some(AiTextureType::Diffuse),
                    AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                    0,
                );
            }

            //Handle Specular
            if let Some(color) = material.specular {
                ai_material.add_property(
                    AI_MATKEY_COLOR_SPECULAR,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(
                        bytemuck::bytes_of(&AiColor4D::from(color)).to_vec(),
                    ),
                    0,
                );
            }

            if let Some(texture) = &material.specular_texture {
                let (uri, ai_texture) = import_texture(path, loader, &mut textures, texture)?;
                if let Some(ai_texture) = ai_texture {
                    ai_textures.push(ai_texture);
                }
                ai_material.add_property(
                    _AI_MATKEY_TEXTURE_BASE,
                    Some(AiTextureType::Specular),
                    AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                    0,
                );
            }

            //Handle Opacity
            //Note: tObj uses dissolve for opacity for mtl standard.
            if let Some(opacity) = material.dissolve {
                ai_material.add_property(
                    AI_MATKEY_OPACITY,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(opacity.to_le_bytes().to_vec()),
                    0,
                );
            }

            //Handle Opacity Texture
            if let Some(texture) = &material.dissolve_texture {
                let (uri, ai_texture) = import_texture(path, loader, &mut textures, texture)?;
                if let Some(ai_texture) = ai_texture {
                    ai_textures.push(ai_texture);
                }
                ai_material.add_property(
                    _AI_MATKEY_TEXTURE_BASE,
                    Some(AiTextureType::Opacity),
                    AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                    0,
                );
            }

            //Handle Bump
            //Note: tObj uses normal for bump
            if let Some(texture) = &material.normal_texture {
                let (uri, ai_texture) = import_texture(path, loader, &mut textures, texture)?;
                if let Some(ai_texture) = ai_texture {
                    ai_textures.push(ai_texture);
                }

                ai_material.add_property(
                    _AI_MATKEY_TEXTURE_BASE,
                    Some(AiTextureType::Height),
                    AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                    0,
                );
            }

            //Handle Shading Model
            let shading_mode: AiShadingMode = match material.illumination_model {
                Some(0) => AiShadingMode::Flat,
                Some(1) => AiShadingMode::Gouraud,
                Some(2) => AiShadingMode::Phong,
                _ => AiShadingMode::Gouraud,
            };
            ai_material.add_property(
                AI_MATKEY_SHADING_MODEL,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(vec![shading_mode as u8]),
                0,
            );
            ai_material.add_property(
                AI_MATKEY_OBJ_ILLUM,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(vec![material.illumination_model.unwrap_or(0) as u8]),
                0,
            );

            //Handle Shininess
            if let Some(shininess) = material.shininess {
                ai_material.add_property(
                    AI_MATKEY_SHININESS,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(shininess.to_le_bytes().to_vec()),
                    0,
                );
            }

            //Handle Specularity
            if let Some(texture) = &material.shininess_texture {
                let (uri, ai_texture) = import_texture(path, loader, &mut textures, texture)?;
                if let Some(ai_texture) = ai_texture {
                    ai_textures.push(ai_texture);
                }
                ai_material.add_property(
                    _AI_MATKEY_TEXTURE_BASE,
                    Some(AiTextureType::Specular),
                    AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                    0,
                );
            }

            //Handle Refraction Index/IOR
            //Note: tObj uses optical_density for ior
            if let Some(ior) = material.optical_density {
                ai_material.add_property(
                    AI_MATKEY_REFRACTI,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(ior.to_le_bytes().to_vec()),
                    0,
                );
            }

            ai_materials.push(ai_material);
        }
        Ok((ai_materials, ai_textures))
    }
}

fn import_texture(
    path: &Path,
    loader: &DataLoader<'_>,
    textures: &mut HashMap<String, usize>,
    texture: &String,
) -> Result<(String, Option<AiTexture>), AiReadError> {
    let (uri, ai_texture) = if !textures.contains_key(texture) {
        // Load Texture
        let file_path = path.with_file_name(texture);
        let mut data = loader(&file_path).map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;
        let mut buffer: Vec<u8> = Vec::new();
        data.read_to_end(&mut buffer)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        //Guess Format
        let format = match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("png") => Ok(ImageFormat::Png),
            Some("jpg") | Some("jpeg") => Ok(ImageFormat::Jpeg),
            Some("bmp") => Ok(ImageFormat::Bmp),
            Some("gif") => Ok(ImageFormat::Gif),
            _ => image::guess_format(&buffer),
        }
        .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        //Load Image
        let texture_image = image::load_from_memory_with_format(&buffer, format)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?
            .to_rgba8();

        //Convert to AiTexture
        let width = texture_image.width();
        let height = texture_image.height();
        let texels: Vec<AiTexel> = texture_image
            .pixels()
            .map(|pixel| AiTexel::from(pixel.0))
            .collect();
        let ai_texture = AiTexture::new(
            texture.clone().to_string(),
            width,
            height,
            format.try_into().unwrap_or(AiTextureFormat::Unknown),
            texels,
        );

        //Add Texture to HashMap
        let index = textures.len() - 1;
        textures.insert(texture.clone(), index);

        //Add Texture to Material
        (format!("*{}", index), Some(ai_texture))
    } else {
        let index = textures[texture];
        (format!("*{}", index), None)
    };
    Ok((uri, ai_texture))
}
