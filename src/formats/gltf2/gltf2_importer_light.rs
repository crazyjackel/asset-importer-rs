use super::gltf2_importer::Gltf2Importer;
use crate::core::error::AiReadError;
use crate::structs::AiColor3D;
use crate::structs::AiVector3D;
use crate::structs::{AiLight, AiLightSourceType};
use gltf::Document;

impl Gltf2Importer {
    #[cfg(not(feature = "KHR_lights_punctual"))]
    pub(crate) fn import_lights(_document: &Document) -> Result<Vec<AiLight>, AiReadError> {
        Ok(Vec::new())
    }
    #[cfg(feature = "KHR_lights_punctual")]
    pub(crate) fn import_lights(document: &Document) -> Result<Vec<AiLight>, AiReadError> {
        if document.lights().is_none() {
            return Ok(Vec::new());
        }
        let mut lights: Vec<AiLight> = Vec::new(); //Final Lights to return
        let asset_lights: Vec<gltf::khr_lights_punctual::Light<'_>> =
            document.lights().unwrap().collect();
        lights.reserve(asset_lights.len());
        for light in asset_lights {
            let mut ai_light = AiLight::default();
            ai_light.source_type = match light.kind() {
                gltf::khr_lights_punctual::Kind::Directional => {
                    ai_light.attenuation = 1.0;
                    ai_light.attenuation_linear = 0.0;
                    ai_light.attenuation_quadratic = 0.0;
                    ai_light.direction = AiVector3D::new(0.0, 0.0, -1.0);
                    ai_light.up = AiVector3D::new(0.0, 1.0, 0.0);
                    AiLightSourceType::Directional
                }
                gltf::khr_lights_punctual::Kind::Point => {
                    ai_light.attenuation = 0.0;
                    ai_light.attenuation_linear = 0.0;
                    ai_light.attenuation_quadratic = 1.0;
                    AiLightSourceType::Point
                }
                gltf::khr_lights_punctual::Kind::Spot {
                    inner_cone_angle,
                    outer_cone_angle,
                } => {
                    ai_light.attenuation = 0.0;
                    ai_light.attenuation_linear = 0.0;
                    ai_light.attenuation_quadratic = 1.0;
                    ai_light.inner_cone_angle = inner_cone_angle;
                    ai_light.outer_cone_angle = outer_cone_angle;
                    ai_light.direction = AiVector3D::new(0.0, 0.0, -1.0);
                    ai_light.up = AiVector3D::new(0.0, 1.0, 0.0);
                    AiLightSourceType::Spot
                }
            };
            let color = light.color();
            let insensity = light.intensity();
            let color_with_intensity = AiColor3D::new(
                color[0] * insensity,
                color[1] * insensity,
                color[2] * insensity,
            );
            ai_light.ambient_color = color_with_intensity;
            ai_light.diffuse_color = color_with_intensity;
            ai_light.specular_color = color_with_intensity;

            lights.push(ai_light);
        }
        Ok(lights)
    }
}

#[cfg(feature = "KHR_lights_punctual")]
#[test]
fn test_light_import() {
    let gltf_data = r#"{
            "extensions": {
                "KHR_lights_punctual": {
                    "lights": [
                        {
                        "color": [
                            1,
                            0.63187497854232788,
                            0.23909975588321689
                        ],
                        "intensity": 15,
                        "name": "Point",
                        "type": "point"
                        },
                        {
                        "intensity": 1.5,
                        "name": "Point.002",
                        "type": "point"
                        },
                        {
                        "color": [
                            0.21223080158233645,
                            0.59061902761459351,
                            0.55834054946899414
                        ],
                        "intensity": 80,
                        "name": "Point.003",
                        "type": "point"
                        },
                        {
                        "color": [
                            0.21223080158233645,
                            0.59061902761459351,
                            0.55834054946899414
                        ],
                        "intensity": 80,
                        "name": "Point.001",
                        "type": "point"
                        },
                        {
                        "color": [
                            1,
                            0.62783652544021606,
                            0.50124037265777588
                        ],
                        "intensity": 180,
                        "name": "Point.004",
                        "type": "point"
                        }
                    ]
                }
            },
            "extensionsUsed": [
                "KHR_lights_punctual"
            ],
            "asset" : {
                "version" : "2.0"
            }
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let lights = Gltf2Importer::import_lights(&document).unwrap();
    assert_eq!(5, lights.len());
}
