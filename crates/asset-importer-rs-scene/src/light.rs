use super::{
    color::AiColor3D,
    type_def::AI_MATH_PI_F,
    vector::{AiVector2D, AiVector3D},
};

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AiLightSourceType {
    #[default]
    Undefined,
    Directional,
    Point,
    Spot,
    Ambient,
    Area,
}

#[derive(Debug, PartialEq)]
pub struct AiLight {
    pub name: String,
    pub source_type: AiLightSourceType,
    pub position: AiVector3D,
    pub direction: AiVector3D,
    pub up: AiVector3D,
    pub attenuation: f32,
    pub attenuation_linear: f32,
    pub attenuation_quadratic: f32,
    pub diffuse_color: AiColor3D,
    pub specular_color: AiColor3D,
    pub ambient_color: AiColor3D,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
    pub size: AiVector2D,
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
            attenuation_linear: 1.0,
            attenuation_quadratic: Default::default(),
            diffuse_color: Default::default(),
            specular_color: Default::default(),
            ambient_color: Default::default(),
            inner_cone_angle: AI_MATH_PI_F,
            outer_cone_angle: AI_MATH_PI_F,
            size: Default::default(),
        }
    }
}
