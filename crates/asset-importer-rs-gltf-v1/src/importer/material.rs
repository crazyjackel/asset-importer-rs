use std::collections::HashMap;

use gltf_v1::document::Document;

use super::GltfImporter;
use asset_importer_rs_core::AiReadError;
use asset_importer_rs_scene::{
    AiColor4D, AiMaterial, AiPropertyTypeInfo, AiTextureType,
    matkey::{
        _AI_MATKEY_TEXTURE_BASE, AI_MATKEY_COLOR_AMBIENT, AI_MATKEY_COLOR_DIFFUSE,
        AI_MATKEY_COLOR_EMISSIVE, AI_MATKEY_COLOR_SPECULAR, AI_MATKEY_NAME, AI_MATKEY_OPACITY,
        AI_MATKEY_SHININESS, AI_MATKEY_TWOSIDED,
    },
};

impl GltfImporter {
    pub(crate) fn import_embedded_materials(
        document: &Document,
        embedded_tex_ids: &HashMap<String, usize>,
    ) -> Result<(Vec<AiMaterial>, HashMap<String, usize>), AiReadError> {
        let mut materials: Vec<AiMaterial> = Vec::with_capacity(document.materials().len());
        let mut material_index_map: HashMap<String, usize> = HashMap::new();
        for material in document.materials() {
            let mut ai_material = AiMaterial::new();

            //Handle Name
            if let Some(name) = material.name() {
                ai_material.add_property(
                    AI_MATKEY_NAME,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(name.bytes().collect()),
                    0,
                );
            }

            //Handle Ambient
            match material.ambient() {
                gltf_v1::material::TexProperty::Texture(texture) => {
                    let index = texture.source().index();
                    let uri = match &embedded_tex_ids.get(index) {
                        Some(str) => format!("*{}", str),
                        None => format!("*{}", index),
                    };
                    ai_material.add_property(
                        _AI_MATKEY_TEXTURE_BASE,
                        Some(AiTextureType::Ambient),
                        AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                        0,
                    );
                }
                gltf_v1::material::TexProperty::Color(color) => {
                    ai_material.add_property(
                        AI_MATKEY_COLOR_AMBIENT,
                        Some(AiTextureType::None),
                        AiPropertyTypeInfo::Binary(
                            bytemuck::bytes_of(&AiColor4D::from(color)).to_vec(),
                        ),
                        0,
                    );
                }
            }

            //Handle Diffuse
            match material.diffuse() {
                gltf_v1::material::TexProperty::Texture(texture) => {
                    let index = texture.source().index();
                    let uri = match &embedded_tex_ids.get(index) {
                        Some(str) => format!("*{}", str),
                        None => format!("*{}", index),
                    };
                    ai_material.add_property(
                        _AI_MATKEY_TEXTURE_BASE,
                        Some(AiTextureType::Diffuse),
                        AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                        0,
                    );
                }
                gltf_v1::material::TexProperty::Color(color) => {
                    ai_material.add_property(
                        AI_MATKEY_COLOR_DIFFUSE,
                        Some(AiTextureType::None),
                        AiPropertyTypeInfo::Binary(
                            bytemuck::bytes_of(&AiColor4D::from(color)).to_vec(),
                        ),
                        0,
                    );
                }
            }

            //Handle Specular
            match material.specular() {
                gltf_v1::material::TexProperty::Texture(texture) => {
                    let index = texture.source().index();
                    let uri = match &embedded_tex_ids.get(index) {
                        Some(str) => format!("*{}", str),
                        None => format!("*{}", index),
                    };
                    ai_material.add_property(
                        _AI_MATKEY_TEXTURE_BASE,
                        Some(AiTextureType::Specular),
                        AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                        0,
                    );
                }
                gltf_v1::material::TexProperty::Color(color) => {
                    ai_material.add_property(
                        AI_MATKEY_COLOR_SPECULAR,
                        Some(AiTextureType::None),
                        AiPropertyTypeInfo::Binary(
                            bytemuck::bytes_of(&AiColor4D::from(color)).to_vec(),
                        ),
                        0,
                    );
                }
            }

            //Handle Emissive
            match material.emission() {
                gltf_v1::material::TexProperty::Texture(texture) => {
                    let index = texture.source().index();
                    let uri = match &embedded_tex_ids.get(index) {
                        Some(str) => format!("*{}", str),
                        None => format!("*{}", index),
                    };
                    ai_material.add_property(
                        _AI_MATKEY_TEXTURE_BASE,
                        Some(AiTextureType::Emissive),
                        AiPropertyTypeInfo::Binary(uri.bytes().collect()),
                        0,
                    );
                }
                gltf_v1::material::TexProperty::Color(color) => {
                    ai_material.add_property(
                        AI_MATKEY_COLOR_EMISSIVE,
                        Some(AiTextureType::None),
                        AiPropertyTypeInfo::Binary(
                            bytemuck::bytes_of(&AiColor4D::from(color)).to_vec(),
                        ),
                        0,
                    );
                }
            }

            //Handle Two Sided
            ai_material.add_property(
                AI_MATKEY_TWOSIDED,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(vec![material.double_sided() as u8]),
                0,
            );

            //Handle Opacity
            if material.transparent() && material.transparency() != 1.0f32 {
                ai_material.add_property(
                    AI_MATKEY_OPACITY,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(material.transparency().to_le_bytes().to_vec()),
                    0,
                );
            }

            //Handle Shininess
            if material.shininess() > 0.0f32 {
                ai_material.add_property(
                    AI_MATKEY_SHININESS,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(material.shininess().to_le_bytes().to_vec()),
                    0,
                );
            }

            material_index_map.insert(material.index().to_string(), materials.len());
            materials.push(ai_material);
        }
        Ok((materials, material_index_map))
    }
}

#[test]
fn test_gltf_material_import() {
    let gltf_data = r#"
    {
        "materials": {
            "m0Avocado_M-fx": {
                "name": "m0Avocado_M",
                "technique": "technique0",
                "values": {
                    "ambient": [
                        0,
                        0,
                        0,
                        1
                    ],
                    "diffuse": "texture_m0Avocado_M-diffuse-image",
                    "emission": [
                        0,
                        0,
                        0,
                        1
                    ],
                    "shininess": 20,
                    "specular": "texture_m0Avocado_M-specular-image"
                }
            }
        },
        "techniques": {
            "technique0": {
                "attributes": {
                    "a_normal": "normal",
                    "a_position": "position",
                    "a_texcoord0": "texcoord0"
                },
                "parameters": {
                    "ambient": {
                        "type": 35666
                    },
                    "diffuse": {
                        "type": 35678
                    },
                    "emission": {
                        "type": 35666
                    },
                    "modelViewMatrix": {
                        "semantic": "MODELVIEW",
                        "type": 35676
                    },
                    "normal": {
                        "semantic": "NORMAL",
                        "type": 35665
                    },
                    "normalMatrix": {
                        "semantic": "MODELVIEWINVERSETRANSPOSE",
                        "type": 35675
                    },
                    "position": {
                        "semantic": "POSITION",
                        "type": 35665
                    },
                    "projectionMatrix": {
                        "semantic": "PROJECTION",
                        "type": 35676
                    },
                    "shininess": {
                        "type": 5126
                    },
                    "specular": {
                        "type": 35678
                    },
                    "texcoord0": {
                        "semantic": "TEXCOORD_0",
                        "type": 35664
                    }
                },
                "program": "program_0",
                "states": {
                    "enable": [
                        2929,
                        2884
                    ],
                    "disable": []
                },
                "uniforms": {
                    "u_ambient": "ambient",
                    "u_diffuse": "diffuse",
                    "u_emission": "emission",
                    "u_modelViewMatrix": "modelViewMatrix",
                    "u_normalMatrix": "normalMatrix",
                    "u_projectionMatrix": "projectionMatrix",
                    "u_shininess": "shininess",
                    "u_specular": "specular"
                }
            }
        }
    }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let (materials, _) =
        GltfImporter::import_embedded_materials(&document, &HashMap::new()).unwrap();
    assert_eq!(1, materials.len())
}
