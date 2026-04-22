//! FBX `NodeAttribute` / `Light` — Assimp [`Light`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

const PROP_COLOR: &str = "Color";
const PROP_LIGHT_TYPE: &str = "LightType";
const PROP_CAST_LIGHT_ON_OBJECT: &str = "CastLightOnObject";
const PROP_DRAW_VOLUMETRIC_LIGHT: &str = "DrawVolumetricLight";
const PROP_DRAW_GROUND_PROJECTION: &str = "DrawGroundProjection";
const PROP_DRAW_FRONT_FACING_VOLUMETRIC_LIGHT: &str = "DrawFrontFacingVolumetricLight";
const PROP_INTENSITY: &str = "Intensity";
const PROP_INNER_ANGLE: &str = "InnerAngle";
const PROP_OUTER_ANGLE: &str = "OuterAngle";
const PROP_FOG: &str = "Fog";
const PROP_DECAY_TYPE: &str = "DecayType";
const PROP_DECAY_START: &str = "DecayStart";
const PROP_FILE_NAME: &str = "FileName";
const PROP_ENABLE_NEAR_ATTENUATION: &str = "EnableNearAttenuation";
const PROP_NEAR_ATTENUATION_START: &str = "NearAttenuationStart";
const PROP_NEAR_ATTENUATION_END: &str = "NearAttenuationEnd";
const PROP_ENABLE_FAR_ATTENUATION: &str = "EnableFarAttenuation";
const PROP_FAR_ATTENUATION_START: &str = "FarAttenuationStart";
const PROP_FAR_ATTENUATION_END: &str = "FarAttenuationEnd";
const PROP_CAST_SHADOWS: &str = "CastShadows";
const PROP_SHADOW_COLOR: &str = "ShadowColor";
const PROP_AREA_LIGHT_SHAPE: &str = "AreaLightShape";
const PROP_LEFT_BARN_DOOR: &str = "LeftBarnDoor";
const PROP_RIGHT_BARN_DOOR: &str = "RightBarnDoor";
const PROP_TOP_BARN_DOOR: &str = "TopBarnDoor";
const PROP_BOTTOM_BARN_DOOR: &str = "BottomBarnDoor";
const PROP_ENABLE_BARN_DOOR: &str = "EnableBarnDoor";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Point,
    Directional,
    Spot,
    Area,
    Volume,
}

impl LightType {
    fn from_i32(value: i32) -> Self {
        match value {
            1 => Self::Directional,
            2 => Self::Spot,
            3 => Self::Area,
            4 => Self::Volume,
            _ => Self::Point,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightDecay {
    None,
    Linear,
    Quadratic,
    Cubic,
}

impl LightDecay {
    fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Linear,
            2 => Self::Quadratic,
            3 => Self::Cubic,
            _ => Self::Quadratic,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Light(pub OwnedObject);

impl Light {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }

    /// Temporary bridge to Assimp-style light properties until typed accessors are added.
    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.0.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.0.properties.get(name)
    }

    pub fn color(&self) -> [f32; 3] {
        match self.property(PROP_COLOR) {
            Some(Property::Vec3(v)) => *v,
            _ => [1.0, 1.0, 1.0],
        }
    }

    pub fn light_type(&self) -> LightType {
        match self.property(PROP_LIGHT_TYPE) {
            Some(Property::Int(v)) => LightType::from_i32(*v),
            _ => LightType::Point,
        }
    }

    pub fn cast_light_on_object(&self) -> bool {
        match self.property(PROP_CAST_LIGHT_ON_OBJECT) {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }

    pub fn draw_volumetric_light(&self) -> bool {
        match self.property(PROP_DRAW_VOLUMETRIC_LIGHT) {
            Some(Property::Bool(v)) => *v,
            _ => true,
        }
    }

    pub fn draw_ground_projection(&self) -> bool {
        match self.property(PROP_DRAW_GROUND_PROJECTION) {
            Some(Property::Bool(v)) => *v,
            _ => true,
        }
    }

    pub fn draw_front_facing_volumetric_light(&self) -> bool {
        match self.property(PROP_DRAW_FRONT_FACING_VOLUMETRIC_LIGHT) {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }

    pub fn intensity(&self) -> f32 {
        match self.property(PROP_INTENSITY) {
            Some(Property::Float(v)) => *v,
            _ => 100.0,
        }
    }

    pub fn inner_angle(&self) -> f32 {
        match self.property(PROP_INNER_ANGLE) {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn outer_angle(&self) -> f32 {
        match self.property(PROP_OUTER_ANGLE) {
            Some(Property::Float(v)) => *v,
            _ => 45.0,
        }
    }

    pub fn fog(&self) -> i32 {
        match self.property(PROP_FOG) {
            Some(Property::Int(v)) => *v,
            _ => 50,
        }
    }

    pub fn decay_type(&self) -> LightDecay {
        match self.property(PROP_DECAY_TYPE) {
            Some(Property::Int(v)) => LightDecay::from_i32(*v),
            _ => LightDecay::Quadratic,
        }
    }

    pub fn decay_start(&self) -> f32 {
        match self.property(PROP_DECAY_START) {
            Some(Property::Float(v)) => *v,
            _ => 1.0,
        }
    }

    pub fn file_name(&self) -> &str {
        match self.property(PROP_FILE_NAME) {
            Some(Property::String(v)) => v.as_str(),
            _ => "",
        }
    }

    pub fn enable_near_attenuation(&self) -> bool {
        match self.property(PROP_ENABLE_NEAR_ATTENUATION) {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }

    pub fn near_attenuation_start(&self) -> f32 {
        match self.property(PROP_NEAR_ATTENUATION_START) {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn near_attenuation_end(&self) -> f32 {
        match self.property(PROP_NEAR_ATTENUATION_END) {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn enable_far_attenuation(&self) -> bool {
        match self.property(PROP_ENABLE_FAR_ATTENUATION) {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }

    pub fn far_attenuation_start(&self) -> f32 {
        match self.property(PROP_FAR_ATTENUATION_START) {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn far_attenuation_end(&self) -> f32 {
        match self.property(PROP_FAR_ATTENUATION_END) {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn cast_shadows(&self) -> bool {
        match self.property(PROP_CAST_SHADOWS) {
            Some(Property::Bool(v)) => *v,
            _ => true,
        }
    }

    pub fn shadow_color(&self) -> [f32; 3] {
        match self.property(PROP_SHADOW_COLOR) {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }

    pub fn area_light_shape(&self) -> i32 {
        match self.property(PROP_AREA_LIGHT_SHAPE) {
            Some(Property::Int(v)) => *v,
            _ => 0,
        }
    }

    pub fn left_barn_door(&self) -> f32 {
        match self.property(PROP_LEFT_BARN_DOOR) {
            Some(Property::Float(v)) => *v,
            _ => 20.0,
        }
    }

    pub fn right_barn_door(&self) -> f32 {
        match self.property(PROP_RIGHT_BARN_DOOR) {
            Some(Property::Float(v)) => *v,
            _ => 20.0,
        }
    }

    pub fn top_barn_door(&self) -> f32 {
        match self.property(PROP_TOP_BARN_DOOR) {
            Some(Property::Float(v)) => *v,
            _ => 20.0,
        }
    }

    pub fn bottom_barn_door(&self) -> f32 {
        match self.property(PROP_BOTTOM_BARN_DOOR) {
            Some(Property::Float(v)) => *v,
            _ => 20.0,
        }
    }

    pub fn enable_barn_door(&self) -> bool {
        match self.property(PROP_ENABLE_BARN_DOOR) {
            Some(Property::Bool(v)) => *v,
            _ => true,
        }
    }
}

impl TryFrom<OwnedObject> for Light {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::Light) => Ok(Light(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "Light".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{NODE_ATTRIBUTE_LIGHT_CLASS_NAME, NODE_ATTRIBUTE_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::{Light, LightDecay, LightType};

    #[test]
    fn property_accessors_return_owned_object_properties() {
        let mut properties = HashMap::new();
        properties.insert("Intensity".to_string(), Property::Float(100.0));
        let o = OwnedObject {
            object_index: 99,
            name: "Light".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_LIGHT_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let light = Light::try_from(o).unwrap();
        assert_eq!(light.property("Intensity"), Some(&Property::Float(100.0)));
        assert_eq!(light.properties().len(), 1);
    }

    #[test]
    fn typed_accessors_return_defaults() {
        let o = OwnedObject {
            object_index: 99,
            name: "Light".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_LIGHT_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let light = Light::try_from(o).unwrap();
        assert_eq!(light.color(), [1.0, 1.0, 1.0]);
        assert_eq!(light.light_type(), LightType::Point);
        assert_eq!(light.decay_type(), LightDecay::Quadratic);
        assert_eq!(light.intensity(), 100.0);
        assert_eq!(light.outer_angle(), 45.0);
        assert_eq!(light.cast_shadows(), true);
        assert_eq!(light.file_name(), "");
    }

    #[test]
    fn typed_accessors_return_property_values() {
        let mut properties = HashMap::new();
        properties.insert("Color".to_string(), Property::Vec3([0.2, 0.3, 0.4]));
        properties.insert("LightType".to_string(), Property::Int(2));
        properties.insert("CastLightOnObject".to_string(), Property::Bool(true));
        properties.insert("DrawVolumetricLight".to_string(), Property::Bool(false));
        properties.insert("DrawGroundProjection".to_string(), Property::Bool(false));
        properties.insert(
            "DrawFrontFacingVolumetricLight".to_string(),
            Property::Bool(true),
        );
        properties.insert("Intensity".to_string(), Property::Float(250.0));
        properties.insert("InnerAngle".to_string(), Property::Float(5.0));
        properties.insert("OuterAngle".to_string(), Property::Float(30.0));
        properties.insert("Fog".to_string(), Property::Int(10));
        properties.insert("DecayType".to_string(), Property::Int(1));
        properties.insert("DecayStart".to_string(), Property::Float(2.0));
        properties.insert("FileName".to_string(), Property::String("gobo.png".into()));
        properties.insert("EnableNearAttenuation".to_string(), Property::Bool(true));
        properties.insert("NearAttenuationStart".to_string(), Property::Float(1.0));
        properties.insert("NearAttenuationEnd".to_string(), Property::Float(2.0));
        properties.insert("EnableFarAttenuation".to_string(), Property::Bool(true));
        properties.insert("FarAttenuationStart".to_string(), Property::Float(20.0));
        properties.insert("FarAttenuationEnd".to_string(), Property::Float(30.0));
        properties.insert("CastShadows".to_string(), Property::Bool(false));
        properties.insert("ShadowColor".to_string(), Property::Vec3([0.1, 0.1, 0.1]));
        properties.insert("AreaLightShape".to_string(), Property::Int(3));
        properties.insert("LeftBarnDoor".to_string(), Property::Float(11.0));
        properties.insert("RightBarnDoor".to_string(), Property::Float(12.0));
        properties.insert("TopBarnDoor".to_string(), Property::Float(13.0));
        properties.insert("BottomBarnDoor".to_string(), Property::Float(14.0));
        properties.insert("EnableBarnDoor".to_string(), Property::Bool(false));
        let o = OwnedObject {
            object_index: 99,
            name: "Light".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_LIGHT_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let light = Light::try_from(o).unwrap();
        assert_eq!(light.color(), [0.2, 0.3, 0.4]);
        assert_eq!(light.light_type(), LightType::Spot);
        assert_eq!(light.cast_light_on_object(), true);
        assert_eq!(light.draw_volumetric_light(), false);
        assert_eq!(light.draw_ground_projection(), false);
        assert_eq!(light.draw_front_facing_volumetric_light(), true);
        assert_eq!(light.intensity(), 250.0);
        assert_eq!(light.inner_angle(), 5.0);
        assert_eq!(light.outer_angle(), 30.0);
        assert_eq!(light.fog(), 10);
        assert_eq!(light.decay_type(), LightDecay::Linear);
        assert_eq!(light.decay_start(), 2.0);
        assert_eq!(light.file_name(), "gobo.png");
        assert_eq!(light.enable_near_attenuation(), true);
        assert_eq!(light.near_attenuation_start(), 1.0);
        assert_eq!(light.near_attenuation_end(), 2.0);
        assert_eq!(light.enable_far_attenuation(), true);
        assert_eq!(light.far_attenuation_start(), 20.0);
        assert_eq!(light.far_attenuation_end(), 30.0);
        assert_eq!(light.cast_shadows(), false);
        assert_eq!(light.shadow_color(), [0.1, 0.1, 0.1]);
        assert_eq!(light.area_light_shape(), 3);
        assert_eq!(light.left_barn_door(), 11.0);
        assert_eq!(light.right_barn_door(), 12.0);
        assert_eq!(light.top_barn_door(), 13.0);
        assert_eq!(light.bottom_barn_door(), 14.0);
        assert_eq!(light.enable_barn_door(), false);
    }
}
