use std::collections::HashMap;

use asset_importer_rs_scene::{
    AiMaterial, AiPropertyTypeInfo, AiScene, AiTextureMapMode, AiTextureType,
    matkey::{
        self, _AI_MATKEY_MAPPINGMODE_U_BASE, _AI_MATKEY_MAPPINGMODE_V_BASE,
        _AI_MATKEY_TEXTURE_BASE, AI_MATKEY_COLOR_AMBIENT, AI_MATKEY_COLOR_DIFFUSE,
        AI_MATKEY_COLOR_EMISSIVE, AI_MATKEY_COLOR_SPECULAR, AI_MATKEY_OPACITY, AI_MATKEY_SHININESS,
        AI_MATKEY_TWOSIDED,
    },
};
use gltf_v1::json::{
    Image, Material, Root, Sampler, StringIndex, Texture,
    material::ParameterValue,
    texture::{SamplerMagFilter, SamplerMinFilter, SamplerWrap},
    validation::Checked,
};

use crate::exporter::{error::GltfExportError, generate_unique_name};

use super::GltfExporter;

impl GltfExporter {
    pub(crate) fn export_materials(
        &self,
        scene: &AiScene,
        root: &mut Root,
    ) -> Result<HashMap<usize, String>, GltfExportError> {
        let unique_names_map: &mut HashMap<String, u32> = &mut HashMap::new();
        let mut textures_by_path: HashMap<String, String> = HashMap::new();
        let mut material_index_map = HashMap::new();
        for material_index in 0..scene.materials.len() {
            let ai_material = &scene.materials[material_index];
            let mut material = Material::default();

            let name_option = ai_material
                .get_property(matkey::AI_MATKEY_NAME, Some(AiTextureType::None), 0)
                .and_then(|prop| {
                    let str =
                        String::from_utf8(prop.data.to_vec()).map_err(GltfExportError::UTFError);
                    Some(str)
                });
            let name = if let Some(name) = name_option {
                generate_unique_name(&name?, unique_names_map)
            } else {
                generate_unique_name("material", unique_names_map)
            };

            material.name = Some(name.clone());

            export_material_texture(
                scene,
                root,
                unique_names_map,
                &mut textures_by_path,
                &mut material,
                &MaterialExport {
                    material: ai_material,
                    material_name: AI_MATKEY_COLOR_AMBIENT,
                    property_name: "ambient",
                    texture_type: AiTextureType::Ambient,
                },
            );
            export_material_texture(
                scene,
                root,
                unique_names_map,
                &mut textures_by_path,
                &mut material,
                &MaterialExport {
                    material: ai_material,
                    material_name: AI_MATKEY_COLOR_DIFFUSE,
                    property_name: "diffuse",
                    texture_type: AiTextureType::Diffuse,
                },
            );
            export_material_texture(
                scene,
                root,
                unique_names_map,
                &mut textures_by_path,
                &mut material,
                &MaterialExport {
                    material: ai_material,
                    material_name: AI_MATKEY_COLOR_SPECULAR,
                    property_name: "specular",
                    texture_type: AiTextureType::Specular,
                },
            );
            export_material_texture(
                scene,
                root,
                unique_names_map,
                &mut textures_by_path,
                &mut material,
                &MaterialExport {
                    material: ai_material,
                    material_name: AI_MATKEY_COLOR_EMISSIVE,
                    property_name: "emission",
                    texture_type: AiTextureType::Emissive,
                },
            );

            // Handle Double Sided
            if let Some(twosided) =
                ai_material.get_property_ai_bool(AI_MATKEY_TWOSIDED, Some(AiTextureType::None), 0)
            {
                material.values.insert(
                    "doubleSided".to_string(),
                    Checked::Valid(ParameterValue::Boolean(twosided)),
                );
            }

            //Handle Opacity
            if let Some(opacity) =
                ai_material.get_property_ai_float(AI_MATKEY_OPACITY, Some(AiTextureType::None), 0)
            {
                material.values.insert(
                    "transparency".to_string(),
                    Checked::Valid(ParameterValue::Number(opacity)),
                );
                if opacity != 1.0 {
                    material.values.insert(
                        "transparent".to_string(),
                        Checked::Valid(ParameterValue::Boolean(true)),
                    );
                }
            }

            //Handle Shininess
            if let Some(shininess) =
                ai_material.get_property_ai_float(AI_MATKEY_SHININESS, Some(AiTextureType::None), 0)
            {
                material.values.insert(
                    "shininess".to_string(),
                    Checked::Valid(ParameterValue::Number(shininess)),
                );
            }
            material_index_map.insert(material_index, name.clone());
            root.materials.insert(name, material);
        }
        Ok(material_index_map)
    }
}

struct MaterialExport<'a> {
    material: &'a AiMaterial,
    material_name: &'a str,
    property_name: &'a str,
    texture_type: AiTextureType,
}

fn export_material_texture(
    scene: &AiScene,
    root: &mut Root,
    unique_names_map: &mut HashMap<String, u32>,
    textures_by_path: &mut HashMap<String, String>,
    material: &mut Material,
    material_export: &MaterialExport,
) {
    if let Some(color) = material_export.material.get_property_ai_color_rgba(
        material_export.material_name,
        Some(AiTextureType::None),
        0,
    ) {
        material.values.insert(
            material_export.property_name.to_string(),
            Checked::Valid(ParameterValue::NumberArray(vec![
                color.r, color.g, color.b, color.a,
            ])),
        );
    }

    if let Some(Ok(path)) = material_export.material.get_property_ai_str(
        _AI_MATKEY_TEXTURE_BASE,
        Some(material_export.texture_type),
        0,
    ) {
        if !path.is_empty() {
            if !path.starts_with("*") {
                if let Some(texture_name) = textures_by_path.get(&path) {
                    material.values.insert(
                        material_export.property_name.to_string(),
                        Checked::Valid(ParameterValue::String(texture_name.clone())),
                    );
                }
            }

            if !material.values.contains_key(material_export.material_name) {
                let texture_name = generate_unique_name("texture", unique_names_map);
                textures_by_path.insert(path.clone(), texture_name.clone());

                // Create Sampler
                let sampler_name = generate_unique_name("sampler", unique_names_map);
                let sampler = Sampler {
                    mag_filter: Checked::Valid(SamplerMagFilter::Linear),
                    min_filter: Checked::Valid(SamplerMinFilter::Linear),
                    wrap_s: material_export
                        .material
                        .get_property(
                            _AI_MATKEY_MAPPINGMODE_U_BASE,
                            Some(material_export.texture_type),
                            0,
                        )
                        .and_then(|prop| {
                            if prop.data.len() == 1 {
                                match prop.data[0].try_into() {
                                    Ok(AiTextureMapMode::Clamp) => {
                                        Some(Checked::Valid(SamplerWrap::ClampToEdge))
                                    }
                                    Ok(AiTextureMapMode::Mirror) => {
                                        Some(Checked::Valid(SamplerWrap::MirroredRepeat))
                                    }
                                    Ok(_) => None,
                                    Err(_) => None,
                                }
                            } else {
                                None
                            }
                        })
                        .unwrap_or(Checked::Valid(SamplerWrap::Repeat)),
                    wrap_t: material_export
                        .material
                        .get_property(
                            _AI_MATKEY_MAPPINGMODE_V_BASE,
                            Some(material_export.texture_type),
                            0,
                        )
                        .and_then(|prop| {
                            if prop.data.len() == 1 {
                                match prop.data[0].try_into() {
                                    Ok(AiTextureMapMode::Clamp) => {
                                        Some(Checked::Valid(SamplerWrap::ClampToEdge))
                                    }
                                    Ok(AiTextureMapMode::Mirror) => {
                                        Some(Checked::Valid(SamplerWrap::MirroredRepeat))
                                    }
                                    Ok(_) => None,
                                    Err(_) => None,
                                }
                            } else {
                                None
                            }
                        })
                        .unwrap_or(Checked::Valid(SamplerWrap::Repeat)),
                    name: None,
                };
                root.samplers.insert(sampler_name.clone(), sampler);

                let source_name = generate_unique_name("image", unique_names_map);
                let mut source = Image::default();

                // Handle embedded textures
                if path.starts_with("*") {
                    let trimmed_path = path.trim_start_matches('*');
                    if let Ok(parsed_index) = trimmed_path.parse::<u32>() {
                        if let Some(texture) = scene.textures.get(parsed_index as usize) {
                            source.name = Some(texture.filename.clone());
                            if let Ok(data) = texture.export(&[]) {
                                let mimetype = data.format.get_mime_type();
                                source.uri = format!(
                                    "data:{};base64,{}",
                                    mimetype,
                                    base64::encode(data.data)
                                );
                            }
                        }
                    }
                } else {
                    source.uri = path.to_string();
                }
                root.images.insert(source_name.clone(), source);

                let texture = Texture::new(
                    StringIndex::new(source_name),
                    StringIndex::new(sampler_name),
                );
                root.textures.insert(texture_name.clone(), texture);

                material.values.insert(
                    material_export.property_name.to_string(),
                    Checked::Valid(ParameterValue::String(texture_name)),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Output;

    use super::*;
    use asset_importer_rs_scene::{
        AiColor4D, AiPropertyTypeInfo, AiTexel, AiTexture, AiTextureType,
    };

    fn create_test_material() -> AiMaterial {
        let mut material = AiMaterial::default();

        // Add a name property
        material.add_property(
            matkey::AI_MATKEY_NAME,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary("TestMaterial".as_bytes().to_vec()),
            0,
        );

        // Add diffuse color
        material.add_property(
            AI_MATKEY_COLOR_DIFFUSE,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(
                bytemuck::bytes_of(&AiColor4D::from([1.0, 0.5, 0.25, 1.0])).to_vec(),
            ),
            0,
        );

        // Add two-sided property
        material.add_property(
            AI_MATKEY_TWOSIDED,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(vec![1]),
            0,
        );

        material
    }

    #[test]
    fn test_export_materials_basic() {
        let mut scene = AiScene::default();
        scene.materials.push(create_test_material());

        let mut root = Root::default();
        let exporter = GltfExporter::new(Output::Standard);

        let result = exporter.export_materials(&scene, &mut root);
        assert!(result.is_ok());

        // Check if material was exported
        assert!(!root.materials.is_empty());

        // Get the exported material
        let material = root.materials.values().next().unwrap();

        // Check material name
        assert_eq!(material.name.as_ref().unwrap(), "TestMaterial");

        // Check diffuse color
        let diffuse = material.values.get("diffuse").unwrap();
        if let Checked::Valid(ParameterValue::NumberArray(color)) = diffuse {
            assert_eq!(color[0], 1.0);
            assert_eq!(color[1], 0.5);
            assert_eq!(color[2], 0.25);
            assert_eq!(color[3], 1.0);
        } else {
            panic!("Diffuse color not found or wrong type");
        }

        // Check two-sided property
        let two_sided = material.values.get("doubleSided").unwrap();
        if let Checked::Valid(ParameterValue::Boolean(value)) = two_sided {
            assert!(value);
        } else {
            panic!("Two-sided property not found or wrong type");
        }
    }

    #[test]
    fn test_export_materials_with_texture() {
        let mut scene = AiScene::default();
        let mut material = create_test_material();

        // Add a diffuse texture
        material.add_property(
            _AI_MATKEY_TEXTURE_BASE,
            Some(AiTextureType::Diffuse),
            AiPropertyTypeInfo::Binary("test_texture.png".as_bytes().to_vec()),
            0,
        );

        // Add texture mapping modes
        material.add_property(
            _AI_MATKEY_MAPPINGMODE_U_BASE,
            Some(AiTextureType::Diffuse),
            AiPropertyTypeInfo::Binary(vec![AiTextureMapMode::Clamp as u8]),
            0,
        );

        material.add_property(
            _AI_MATKEY_MAPPINGMODE_V_BASE,
            Some(AiTextureType::Diffuse),
            AiPropertyTypeInfo::Binary(vec![AiTextureMapMode::Mirror as u8]),
            0,
        );

        scene.materials.push(material);

        let mut root = Root::default();
        let exporter = GltfExporter::new(Output::Standard);

        let result = exporter.export_materials(&scene, &mut root);
        assert!(result.is_ok());

        // Check if texture was created
        assert!(!root.textures.is_empty());
        assert!(!root.samplers.is_empty());
        assert!(!root.images.is_empty());

        // Get the exported material
        let material = root.materials.values().next().unwrap();

        // Check if texture reference was added to material
        let diffuse = material.values.get("diffuse").unwrap();
        if let Checked::Valid(ParameterValue::String(texture_name)) = diffuse {
            assert!(root.textures.contains_key(texture_name));
        } else {
            panic!("Diffuse texture reference not found or wrong type");
        }
    }

    #[test]
    fn test_export_materials_with_embedded_texture() {
        let mut scene = AiScene::default();
        let mut material = create_test_material();

        // Add an embedded texture
        let texture = AiTexture {
            filename: "embedded_texture.png".to_string(),
            width: 2,
            height: 2,
            ach_format_hint: asset_importer_rs_scene::AiTextureFormat::PNG,
            texel: vec![
                AiTexel::new(255, 0, 0, 255),
                AiTexel::new(0, 255, 0, 255),
                AiTexel::new(0, 0, 255, 255),
                AiTexel::new(255, 255, 255, 255),
            ],
        };
        scene.textures.push(texture);

        // Add a diffuse texture reference to the embedded texture
        material.add_property(
            _AI_MATKEY_TEXTURE_BASE,
            Some(AiTextureType::Diffuse),
            AiPropertyTypeInfo::Binary("*0".as_bytes().to_vec()),
            0,
        );

        scene.materials.push(material);

        let mut root = Root::default();
        let exporter = GltfExporter::new(Output::Standard);

        let result = exporter.export_materials(&scene, &mut root);
        assert!(result.is_ok());

        // Check if embedded texture was exported
        assert!(!root.images.is_empty());

        // Get the exported image
        let image = root.images.values().next().unwrap();

        // Check if the image URI is a data URI
        assert!(image.uri.starts_with("data:image/png;baseW64,"));
    }
}
