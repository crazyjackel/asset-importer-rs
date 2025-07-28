use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use asset_importer_rs_core::{AiReadError, DataLoader};
use asset_importer_rs_scene::{
    AiColor4D, AiMaterial, AiPropertyTypeInfo, AiShadingMode, AiTexel, AiTexture, AiTextureFormat,
    AiTextureType,
    matkey::{
        _AI_MATKEY_TEXTURE_BASE, AI_MATKEY_COLOR_AMBIENT, AI_MATKEY_COLOR_DIFFUSE,
        AI_MATKEY_COLOR_SPECULAR, AI_MATKEY_NAME, AI_MATKEY_OBJ_ILLUM, AI_MATKEY_OPACITY,
        AI_MATKEY_REFRACTI, AI_MATKEY_SHADING_MODEL, AI_MATKEY_SHININESS,
    },
};
use image::ImageFormat;
use tobj::Material;

use crate::importer::{ObjImporter, error::ObjImportError};

#[derive(Debug)]
pub struct ImportMaterials(pub Vec<AiMaterial>, pub Vec<AiTexture>);

impl ObjImporter {
    pub(crate) fn import_materials(
        path: &Path,
        materials: Vec<Material>,
        loader: &DataLoader<'_>,
    ) -> Result<ImportMaterials, ObjImportError> {
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
                AiPropertyTypeInfo::Binary(vec![material.illumination_model.unwrap_or(0)]),
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
        Ok(ImportMaterials(ai_materials, ai_textures))
    }
}

fn import_texture(
    path: &Path,
    loader: &DataLoader<'_>,
    textures: &mut HashMap<String, usize>,
    texture: &String,
) -> Result<(String, Option<AiTexture>), ObjImportError> {
    let (uri, ai_texture) = if !textures.contains_key(texture) {
        // Load Texture
        let new_texture = texture.replace("\\", "/");
        let file_path = path.with_file_name(new_texture);
        let mut data =
            loader(&file_path).map_err(|x| ObjImportError::FileOpenError(x, file_path.clone()))?;
        let mut buffer: Vec<u8> = Vec::new();
        data.read_to_end(&mut buffer)
            .map_err(|x| ObjImportError::FileReadError(x))?;

        //Guess Format
        let format = match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("png") => Ok(ImageFormat::Png),
            Some("jpg") | Some("jpeg") => Ok(ImageFormat::Jpeg),
            Some("bmp") => Ok(ImageFormat::Bmp),
            Some("gif") => Ok(ImageFormat::Gif),
            _ => image::guess_format(&buffer),
        }
        .map_err(|x| {
            ObjImportError::UnsupportedFileFormat(
                x,
                file_path
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            )
        })?;

        //Load Image
        let texture_image = image::load_from_memory_with_format(&buffer, format)
            .map_err(ObjImportError::ImageLoadError)?
            .to_rgba8();

        //Convert to AiTexture
        let width = texture_image.width();
        let height = texture_image.height();
        let texels: Vec<AiTexel> = texture_image
            .pixels()
            .map(|pixel| AiTexel::from(pixel.0))
            .collect();
        let ai_texture = AiTexture::new(
            file_path.file_name().unwrap().to_string_lossy().to_string(),
            width,
            height,
            format.try_into().unwrap_or(AiTextureFormat::Unknown),
            texels,
        );

        //Add Texture to HashMap
        let index = textures.len();
        textures.insert(texture.clone(), index);

        //Add Texture to Material
        (format!("*{}", index), Some(ai_texture))
    } else {
        let index = textures[texture];
        (format!("*{}", index), None)
    };
    Ok((uri, ai_texture))
}

#[cfg(test)]
mod tests {
    use super::*;
    use asset_importer_rs_core::ReadSeek;
    use asset_importer_rs_scene::{AiMaterialProperty, AiShadingMode};
    use std::{
        fs::File,
        io::{Cursor, Read},
    };

    // Mock data loader for testing
    fn mock_loader(_: &Path) -> std::io::Result<Box<dyn ReadSeek>> {
        let binding = std::env::current_dir().expect("Failed to get the current executable path");
        let exe_path = binding.as_path();
        println!("exe_path: {:?}", exe_path);
        let mut file = File::open(exe_path.join("assets/Unicode❤♻Texture.png"))
            .expect("Failed to open test.png");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Failed to read test.png");

        Ok(Box::new(Cursor::new(buffer)))
    }

    #[test]
    fn test_import_materials_empty() {
        let path = Path::new("test.obj");
        let materials = vec![];
        let result = ObjImporter::import_materials(path, materials, &mock_loader);

        assert!(result.is_ok());
        let ImportMaterials(ai_materials, ai_textures) = result.unwrap();
        assert_eq!(ai_materials.len(), 0);
        assert_eq!(ai_textures.len(), 0);
    }

    #[test]
    fn test_import_materials_basic() {
        let path = Path::new("test.obj");
        let material = Material {
            name: "test_material".to_string(),
            ambient: Some([0.1, 0.2, 0.3]),
            diffuse: Some([0.4, 0.5, 0.6]),
            specular: Some([0.7, 0.8, 0.9]),
            shininess: Some(32.0),
            dissolve: Some(0.8),
            optical_density: Some(1.5),
            illumination_model: Some(2),
            ambient_texture: None,
            diffuse_texture: None,
            specular_texture: None,
            normal_texture: None,
            dissolve_texture: None,
            shininess_texture: None,
            ..Default::default()
        };

        let materials = vec![material];
        let result = ObjImporter::import_materials(path, materials, &mock_loader);

        assert!(result.is_ok());
        let ImportMaterials(ai_materials, ai_textures) = result.unwrap();
        assert_eq!(ai_materials.len(), 1);
        assert_eq!(ai_textures.len(), 0);

        let ai_material = &ai_materials[0];

        // Check that the material has the expected properties
        let properties: Vec<&AiMaterialProperty> = ai_material.iter().collect();
        assert!(!properties.is_empty());

        // Verify name property exists
        let name_props: Vec<_> = properties
            .iter()
            .filter(|p| p.key == AI_MATKEY_NAME)
            .collect();
        assert_eq!(name_props.len(), 1);

        let ambient_props: Vec<_> = properties
            .iter()
            .filter(|p| p.key == AI_MATKEY_COLOR_AMBIENT)
            .collect();
        assert_eq!(ambient_props.len(), 1);
    }

    #[test]
    fn test_import_materials_with_textures() {
        let path = Path::new("test.obj");
        let material = Material {
            name: "textured_material".to_string(),
            ambient: Some([0.1, 0.2, 0.3]),
            diffuse: Some([0.4, 0.5, 0.6]),
            specular: Some([0.7, 0.8, 0.9]),
            shininess: Some(32.0),
            dissolve: Some(0.8),
            optical_density: Some(1.5),
            illumination_model: Some(1),
            ambient_texture: Some("ambient.png".to_string()),
            diffuse_texture: Some("diffuse.png".to_string()),
            specular_texture: Some("specular.png".to_string()),
            normal_texture: Some("normal.png".to_string()),
            dissolve_texture: Some("opacity.png".to_string()),
            shininess_texture: Some("shininess.png".to_string()),
            ..Default::default()
        };

        let materials = vec![material];
        let result = ObjImporter::import_materials(path, materials, &mock_loader);

        assert!(
            result.is_ok(),
            "Expected Ok(..), got Err: {:?}",
            result.unwrap_err()
        );
        let ImportMaterials(ai_materials, ai_textures) = result.unwrap();
        assert_eq!(ai_materials.len(), 1);
        assert_eq!(ai_textures.len(), 6); // All 6 textures should be loaded

        // Verify textures are unique
        let texture_names: std::collections::HashSet<_> =
            ai_textures.iter().map(|t| t.filename.clone()).collect();
        assert_eq!(texture_names.len(), 6);
    }

    #[test]
    fn test_import_materials_duplicate_textures() {
        let path = Path::new("test.obj");
        let material1 = Material {
            name: "material1".to_string(),
            ambient: Some([0.1, 0.2, 0.3]),
            diffuse: Some([0.4, 0.5, 0.6]),
            specular: Some([0.7, 0.8, 0.9]),
            shininess: Some(32.0),
            dissolve: Some(0.8),
            optical_density: Some(1.5),
            illumination_model: Some(1),
            ambient_texture: Some("shared.png".to_string()),
            diffuse_texture: Some("shared.png".to_string()),
            specular_texture: None,
            normal_texture: None,
            dissolve_texture: None,
            shininess_texture: None,
            ..Default::default()
        };

        let material2 = Material {
            name: "material2".to_string(),
            ambient: Some([0.2, 0.3, 0.4]),
            diffuse: Some([0.5, 0.6, 0.7]),
            specular: Some([0.8, 0.9, 1.0]),
            shininess: Some(64.0),
            dissolve: Some(0.9),
            optical_density: Some(1.6),
            illumination_model: Some(2),
            ambient_texture: Some("shared.png".to_string()),
            diffuse_texture: None,
            specular_texture: None,
            normal_texture: None,
            dissolve_texture: None,
            shininess_texture: None,
            ..Default::default()
        };

        let materials = vec![material1, material2];
        let result = ObjImporter::import_materials(path, materials, &mock_loader);

        assert!(
            result.is_ok(),
            "Expected Ok(..), got Err: {:?}",
            result.unwrap_err()
        );
        let ImportMaterials(ai_materials, ai_textures) = result.unwrap();
        assert_eq!(ai_materials.len(), 2);
        assert_eq!(ai_textures.len(), 1); // Only one texture should be loaded

        // Verify the shared texture is only loaded once
        assert_eq!(ai_textures[0].filename, "shared.png");
    }

    #[test]
    fn test_import_materials_shading_modes() {
        let path = Path::new("test.obj");

        // Test different illumination models
        let test_cases = vec![
            (Some(0), AiShadingMode::Flat),
            (Some(1), AiShadingMode::Gouraud),
            (Some(2), AiShadingMode::Phong),
            (Some(3), AiShadingMode::Gouraud), // Default case
            (None, AiShadingMode::Gouraud),    // None case
        ];

        for (illum_model, expected_shading_mode) in test_cases {
            let material = Material {
                name: format!("material_{:?}", illum_model),
                ambient: Some([0.1, 0.2, 0.3]),
                diffuse: Some([0.4, 0.5, 0.6]),
                specular: Some([0.7, 0.8, 0.9]),
                shininess: Some(32.0),
                dissolve: Some(0.8),
                optical_density: Some(1.5),
                illumination_model: illum_model,
                ambient_texture: None,
                diffuse_texture: None,
                specular_texture: None,
                normal_texture: None,
                dissolve_texture: None,
                shininess_texture: None,
                ..Default::default()
            };

            let materials = vec![material];
            let result = ObjImporter::import_materials(path, materials, &mock_loader);

            assert!(result.is_ok());
            let ImportMaterials(ai_materials, _) = result.unwrap();
            assert_eq!(ai_materials.len(), 1);

            let ai_material = &ai_materials[0];
            let properties: Vec<&AiMaterialProperty> = ai_material.iter().collect();

            // Find the shading model property
            let shading_props: Vec<_> = properties
                .iter()
                .filter(|p| p.key == AI_MATKEY_SHADING_MODEL)
                .collect();

            assert_eq!(shading_props.len(), 1);
            if let AiPropertyTypeInfo::Binary(data) = &shading_props[0].property_type {
                assert_eq!(data.len(), 1);
                assert_eq!(data[0], expected_shading_mode as u8);
            } else {
                panic!("Shading model property should be binary");
            }
        }
    }

    #[test]
    fn test_import_materials_optional_properties() {
        let path = Path::new("test.obj");

        // Test material with only required properties
        let material = Material {
            name: "minimal_material".to_string(),
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
            dissolve: None,
            optical_density: None,
            illumination_model: None,
            ambient_texture: None,
            diffuse_texture: None,
            specular_texture: None,
            normal_texture: None,
            dissolve_texture: None,
            shininess_texture: None,
            ..Default::default()
        };

        let materials = vec![material];
        let result = ObjImporter::import_materials(path, materials, &mock_loader);

        assert!(result.is_ok());
        let ImportMaterials(ai_materials, ai_textures) = result.unwrap();
        assert_eq!(ai_materials.len(), 1);
        assert_eq!(ai_textures.len(), 0);

        let ai_material = &ai_materials[0];
        let properties: Vec<&AiMaterialProperty> = ai_material.iter().collect();

        // Should have at least name, shading model, and illum properties
        assert!(properties.len() >= 3);

        // Verify name property exists
        let name_props: Vec<_> = properties
            .iter()
            .filter(|p| p.key == AI_MATKEY_NAME)
            .collect();
        assert_eq!(name_props.len(), 1);
    }

    #[test]
    fn test_import_texture_success() {
        let path = Path::new("test.obj");
        let mut textures = HashMap::new();
        let texture_name = "test.png".to_string();

        let result = import_texture(path, &mock_loader, &mut textures, &texture_name);

        assert!(result.is_ok());
        let (uri, ai_texture) = result.unwrap();

        // Should return a texture and URI
        assert!(ai_texture.is_some());
        assert_eq!(uri, "*0");

        // Texture should be added to the map
        assert!(textures.contains_key(&texture_name));
        assert_eq!(textures[&texture_name], 0);

        // Test duplicate texture loading
        let result2 = import_texture(path, &mock_loader, &mut textures, &texture_name);
        assert!(result2.is_ok());
        let (uri2, ai_texture2) = result2.unwrap();

        // Should return the same URI but no texture (already loaded)
        assert_eq!(uri2, "*0");
        assert!(ai_texture2.is_none());
    }

    #[test]
    fn test_import_texture_file_not_found() {
        let path = Path::new("test.obj");
        let mut textures = HashMap::new();
        let texture_name = "nonexistent.png".to_string();

        // Create a loader that fails
        let failing_loader = |_path: &Path| -> std::io::Result<Box<dyn ReadSeek>> {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ))
        };

        let result = import_texture(path, &failing_loader, &mut textures, &texture_name);

        assert!(result.is_err());
        match result.unwrap_err() {
            ObjImportError::FileOpenError(_, _) => {}
            _ => panic!("Expected FileOpenError"),
        }
    }

    #[test]
    fn test_import_materials_multiple_materials() {
        let path = Path::new("test.obj");
        let materials = vec![
            Material {
                name: "material1".to_string(),
                ambient: Some([0.1, 0.2, 0.3]),
                diffuse: Some([0.4, 0.5, 0.6]),
                specular: Some([0.7, 0.8, 0.9]),
                shininess: Some(32.0),
                dissolve: Some(0.8),
                optical_density: Some(1.5),
                illumination_model: Some(1),
                ambient_texture: None,
                diffuse_texture: None,
                specular_texture: None,
                normal_texture: None,
                dissolve_texture: None,
                shininess_texture: None,
                ..Default::default()
            },
            Material {
                name: "material2".to_string(),
                ambient: Some([0.2, 0.3, 0.4]),
                diffuse: Some([0.5, 0.6, 0.7]),
                specular: Some([0.8, 0.9, 1.0]),
                shininess: Some(64.0),
                dissolve: Some(0.9),
                optical_density: Some(1.6),
                illumination_model: Some(2),
                ambient_texture: None,
                diffuse_texture: None,
                specular_texture: None,
                normal_texture: None,
                dissolve_texture: None,
                shininess_texture: None,
                ..Default::default()
            },
        ];

        let result = ObjImporter::import_materials(path, materials, &mock_loader);

        assert!(result.is_ok());
        let ImportMaterials(ai_materials, ai_textures) = result.unwrap();
        assert_eq!(ai_materials.len(), 2);
        assert_eq!(ai_textures.len(), 0);

        // Verify both materials have different names
        let material_names: Vec<_> = ai_materials
            .iter()
            .map(|m| {
                let properties: Vec<&AiMaterialProperty> = m.iter().collect();
                let name_props: Vec<_> = properties
                    .iter()
                    .filter(|p| p.key == AI_MATKEY_NAME)
                    .collect();
                if let AiPropertyTypeInfo::Binary(data) = &name_props[0].property_type {
                    String::from_utf8_lossy(data).to_string()
                } else {
                    String::new()
                }
            })
            .collect();

        assert_eq!(material_names[0], "material1");
        assert_eq!(material_names[1], "material2");
    }
}
