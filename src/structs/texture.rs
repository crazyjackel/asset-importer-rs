use std::io::{BufWriter, Cursor};

use image::{codecs::png::PngEncoder, write_buffer_with_format, ExtendedColorType, ImageFormat};

use super::color::AiColor4D;

#[derive(Debug, PartialEq, Clone)]
pub enum AiTextureFormat {
    Unknown,
    Png,
    JPEG,
}

impl AiTextureFormat {
    pub fn get_mime_type(&self) -> String {
        match self {
            AiTextureFormat::Unknown => "image/unknown".to_string(),
            AiTextureFormat::Png => "image/png".to_string(),
            AiTextureFormat::JPEG => "image/jpg".to_string(),
        }
    }

    pub fn get_extension(&self) -> String {
        match self {
            AiTextureFormat::Unknown => "unknown".to_string(),
            AiTextureFormat::Png => "png".to_string(),
            AiTextureFormat::JPEG => "jpeg".to_string(),
        }
    }
}

impl Default for AiTextureFormat {
    fn default() -> Self {
        AiTextureFormat::Unknown
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiTexel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl AiTexel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        AiTexel { b, g, r, a }
    }
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
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub ach_format_hint: AiTextureFormat,
    pub texel: Vec<AiTexel>,
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

impl AiTexture {
    pub fn new(
        filename: String,
        width: u32,
        height: u32,
        ach_format_hint: AiTextureFormat,
        texel: Vec<AiTexel>,
    ) -> Self {
        AiTexture {
            filename,
            width,
            height,
            ach_format_hint,
            texel,
        }
    }

    pub fn export(&self) -> Vec<u8> {
        let mut bufwriter = Cursor::new(Vec::new());
        let texels: Vec<u8> = self.texel.iter().flat_map(|x| [x.b, x.g, x.r, x.a]).collect();
        let _ = write_buffer_with_format(
            &mut bufwriter,
            &texels,
            self.width,
            self.height,
            ExtendedColorType::Bgra8,
            match self.ach_format_hint{
                AiTextureFormat::Unknown | AiTextureFormat::Png => ImageFormat::Png,
                AiTextureFormat::JPEG => ImageFormat::Jpeg
            }
        );
        bufwriter.into_inner()
    }
}
