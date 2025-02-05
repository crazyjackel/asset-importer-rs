use std::io::{BufWriter, Cursor};

use image::{codecs::png::PngEncoder, save_buffer_with_format, write_buffer_with_format, ColorType, ExtendedColorType, ImageBuffer, ImageError, ImageFormat};

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

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct AiTexel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

    pub fn export(&self) -> Result<Vec<u8>, ImageError> {
        let format = 
        match self.ach_format_hint{
            AiTextureFormat::Unknown | AiTextureFormat::Png => ImageFormat::Png,
            AiTextureFormat::JPEG => ImageFormat::Jpeg
        };

        let mut bytes: Vec<u8> = Vec::new();
        bytes.reserve((self.width * self.height * 4) as usize);
        let byte_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(self.texel.as_ptr() as *const u8, self.texel.len() * std::mem::size_of::<AiTexel>())
        };
        let _ = write_buffer_with_format(
            &mut Cursor::new(&mut bytes),
            byte_slice,
            self.width,
            self.height,
            ColorType::Rgba8,
            format,
        );
        Ok(bytes)
    }
}
