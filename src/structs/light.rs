use super::{
    color::AiColor3D,
    vector::{AiVector2D, AiVector3D},
};

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum AiLightSourceType {
    Undefined,
    Directional,
    Point,
    Spot,
    Ambient,
    Area,
}

impl Default for AiLightSourceType {
    fn default() -> Self {
        AiLightSourceType::Undefined
    }
}

#[derive(Debug, PartialEq)]
pub struct AiLight {
    name: String,
    source_type: AiLightSourceType,
    position: AiVector3D,
    direction: AiVector3D,
    up: AiVector3D,
    attenuation: f32,
    attenuation_linear: f32,
    attenuation_quadratic: f32,
    diffuse_color: AiColor3D,
    ambient_color: AiColor3D,
    inner_cone_angle: f32,
    outer_cone_angle: f32,
    size: AiVector2D,
}

impl Default for AiLight {
    fn default() -> Self {
        Self {
            name: Default::default(),
            source_type: Default::default(),
            position: Default::default(),
            direction: Default::default(),
            up: Default::default(),
            attenuation: Default::default(),
            attenuation_linear: Default::default(),
            attenuation_quadratic: Default::default(),
            diffuse_color: Default::default(),
            ambient_color: Default::default(),
            inner_cone_angle: Default::default(),
            outer_cone_angle: Default::default(),
            size: Default::default(),
        }
    }
}
