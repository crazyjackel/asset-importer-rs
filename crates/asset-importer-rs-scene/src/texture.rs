use std::io::Cursor;

use image::{ColorType, ImageError, ImageFormat, write_buffer_with_format};

use super::color::AiColor4D;

#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub enum AiTextureFormat {
    #[default]
    Unknown,
    PNG,
    JPEG,
    WEBP,
    BMP,
    GIF,
}

impl From<AiTextureFormat> for ImageFormat {
    fn from(value: AiTextureFormat) -> Self {
        match value {
            AiTextureFormat::Unknown | AiTextureFormat::PNG => ImageFormat::Png,
            AiTextureFormat::JPEG => ImageFormat::Jpeg,
            AiTextureFormat::WEBP => ImageFormat::WebP,
            AiTextureFormat::BMP => ImageFormat::Bmp,
            AiTextureFormat::GIF => ImageFormat::Gif,
        }
    }
}

impl TryFrom<ImageFormat> for AiTextureFormat {
    type Error = ();

    fn try_from(value: ImageFormat) -> Result<Self, Self::Error> {
        match value {
            ImageFormat::Png => Ok(AiTextureFormat::PNG),
            ImageFormat::Jpeg => Ok(AiTextureFormat::JPEG),
            ImageFormat::WebP => Ok(AiTextureFormat::WEBP),
            ImageFormat::Bmp => Ok(AiTextureFormat::BMP),
            ImageFormat::Gif => Ok(AiTextureFormat::GIF),
            _ => Err(()),
        }
    }
}

impl AiTextureFormat {
    pub fn get_mime_type(&self) -> String {
        match self {
            AiTextureFormat::Unknown => "image/unknown".to_string(),
            AiTextureFormat::PNG => "image/png".to_string(),
            AiTextureFormat::JPEG => "image/jpg".to_string(),
            AiTextureFormat::WEBP => "image/webp".to_string(),
            AiTextureFormat::BMP => "image/bmp".to_string(),
            AiTextureFormat::GIF => "image/gif".to_string(),
        }
    }

    pub fn get_extension(&self) -> String {
        match self {
            AiTextureFormat::Unknown => "unknown".to_string(),
            AiTextureFormat::PNG => "png".to_string(),
            AiTextureFormat::JPEG => "jpeg".to_string(),
            AiTextureFormat::WEBP => "webp".to_string(),
            AiTextureFormat::BMP => "bmp".to_string(),
            AiTextureFormat::GIF => "gif".to_string(),
        }
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

impl From<[u8; 4]> for AiTexel {
    fn from(value: [u8; 4]) -> Self {
        AiTexel::new(value[0], value[1], value[2], value[3])
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct AiTexture {
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub ach_format_hint: AiTextureFormat,
    pub texel: Vec<AiTexel>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TextureExport {
    pub data: Vec<u8>,
    pub format: AiTextureFormat,
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

    pub fn get_approved_format(&self, approved_formats: &[AiTextureFormat]) -> AiTextureFormat {
        if approved_formats.is_empty() {
            return self.ach_format_hint;
        }

        if approved_formats.contains(&self.ach_format_hint) {
            self.ach_format_hint
        } else {
            *approved_formats.first().unwrap()
        }
    }

    pub fn export(
        &self,
        approved_formats: &[AiTextureFormat],
    ) -> Result<TextureExport, ImageError> {
        let format = self.get_approved_format(approved_formats);
        let mut data: Vec<u8> = Vec::with_capacity((self.width * self.height * 4) as usize);
        let byte_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.texel.as_ptr() as *const u8,
                self.texel.len() * std::mem::size_of::<AiTexel>(),
            )
        };
        let _ = write_buffer_with_format(
            &mut Cursor::new(&mut data),
            byte_slice,
            self.width,
            self.height,
            ColorType::Rgba8,
            format.into(),
        );
        Ok(TextureExport { data, format })
    }
}
