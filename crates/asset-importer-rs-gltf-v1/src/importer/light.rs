use std::collections::HashMap;

use gltf_v1::{
    Document,
    light::{Kind, Light},
};

use asset_importer_rs_core::AiReadError;
use asset_importer_rs_scene::{AiLight, AiLightSourceType};

use super::GltfImporter;

pub struct ImportLights(pub Vec<AiLight>, pub HashMap<String, usize>);
impl GltfImporter {
    #[cfg(not(feature = "KHR_materials_common"))]
    pub(crate) fn import_lights(document: &Document) -> Result<ImportLights, AiReadError> {
        //@todo: Handle KHR_materials_common ext for lights
        Ok(ImportLights(Vec::new(), HashMap::new()))
    }
    #[cfg(feature = "KHR_materials_common")]
    pub(crate) fn import_lights(document: &Document) -> Result<ImportLights, AiReadError> {
        let asset_lights: Vec<Light<'_>> =
            document.lights().map(|x| x.collect()).unwrap_or_default();
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
                    super::error::Error::DuplicateName,
                )));
            }
            light_map.insert(name.clone(), index);
            let mut ai_light = AiLight {
                name,
                ambient_color: light.color().into(),
                diffuse_color: light.color().into(),
                specular_color: light.color().into(),
                outer_cone_angle: light.falloff_angle(),
                inner_cone_angle: light.falloff_angle()
                    * (1.0 - 1.0 / (1.0 + light.falloff_exponent())),
                attenuation: light.constant_attenuation(),
                attenuation_linear: light.linear_attenuation(),
                attenuation_quadratic: light.quadratic_attenuation(),
                ..AiLight::default()
            };
            ai_light.source_type = match light.kind() {
                Kind::Ambient => AiLightSourceType::Ambient,
                Kind::Directional => AiLightSourceType::Directional,
                Kind::Point => AiLightSourceType::Point,
                Kind::Spot => AiLightSourceType::Spot,
            };
            lights.push(ai_light);
        }
        Ok(ImportLights(Vec::new(), HashMap::new()))
    }
}
