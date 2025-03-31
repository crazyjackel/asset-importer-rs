use std::collections::HashMap;

use gltf_v1::{light::Light, Document};

use crate::{core::error::AiReadError, structs::AiLight};

use super::gltf_importer::GltfImporter;

pub struct ImportLights(pub Vec<AiLight>, pub HashMap<String, usize>);
impl GltfImporter {
    #[cfg(not(feature = "gltf_KHR_materials_common"))]
    pub(crate) fn import_lights(document: &Document) -> Result<ImportLights, AiReadError> {
        //@todo: Handle KHR_materials_common ext for lights
        Ok(ImportLights(Vec::new(), HashMap::new()))
    }
    pub(crate) fn import_lights(document: &Document) -> Result<ImportLights, AiReadError> {
        let asset_lights: Vec<Light<'_>> = document.lights().map(|x| x.collect()).unwrap_or_default();
        let mut lights: Vec<AiLight> = Vec::with_capacity(asset_lights.len());
        let mut light_map: HashMap<String, usize> = HashMap::new();
        for light in asset_lights {
            let index = lights.len();
            let name = light
                .name()
                .map(|x| x.to_string())
                .unwrap_or(format!("{}", index));
            light_map.insert(name.clone(), index);
            if light_map.contains_key(&name) {
                return Err(AiReadError::FileFormatError(Box::new(
                    super::gltf_error::Error::DuplicateName,
                )));
            }
            light_map.insert(name.clone(), index);
            let mut ai_light = AiLight {
                name,
                ambient_color: light.color().into()
                ,diffuse_color: light.color().into(),
                specular_color: light.color().into(),
                outer_cone_angle: light.falloff_angle(),
                inner_cone_angle: light.falloff_angle() * (1.0 - 1.0 / (1.0 + light.falloff_exponent())),
                attenuation: light.constant_attenuation(),
                attenuation_linear: light.linear_attenuation(),
                attenuation_quadratic: light.quadratic_attenuation(),
                ..AiLight::default()
            };
            ai_light.source_type = match light.kind(){
                gltf_v1::light::Kind::Ambient => crate::structs::AiLightSourceType::Ambient,
                gltf_v1::light::Kind::Directional => crate::structs::AiLightSourceType::Directional,
                gltf_v1::light::Kind::Point => crate::structs::AiLightSourceType::Point,
                gltf_v1::light::Kind::Spot => crate::structs::AiLightSourceType::Spot,
            };
            lights.push(ai_light);
        }
        Ok(ImportLights(Vec::new(), HashMap::new()))
    }
}
