use super::color::AiColor4D;

#[derive(Debug, PartialEq, Clone)]
pub struct AiTexel {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

impl From<AiTexel> for AiColor4D {
    fn from(value: AiTexel) -> Self {
        AiColor4D::new(
            value.r as f32 / 255.0f32,
            value.g as f32 / 255.0f32,
            value.b as f32 / 255.0f32,
            value.a as f32 / 255.0f32,
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiTexture {
    filename: String,
    width: u32,
    height: u32,
    ach_format_hint: [u8; 9],
    texel: Vec<AiTexel>,
}

impl Default for AiTexture {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            width: Default::default(),
            height: Default::default(),
            ach_format_hint: Default::default(),
            texel: Default::default(),
        }
    }
}
