use std::fmt;

use super::{common::StringIndex, image::Image, validation::Checked};
use gltf_v1_derive::Validate;
use serde::de::{self, value};
use serde::{Deserialize, Serialize};

pub const NEAREST: u32 = 9728;
pub const LINEAR: u32 = 9729;
pub const NEAREST_MIPMAP_NEAREST: u32 = 9984;
pub const LINEAR_MIPMAP_NEAREST: u32 = 9985;
pub const NEAREST_MIPMAP_LINEAR: u32 = 9986;
pub const LINEAR_MIPMAP_LINEAR: u32 = 9987;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SamplerMagFilter {
    #[default]
    Nearest,
    Linear,
}

impl SamplerMagFilter {
    pub const VALID_SAMPLER_MAG_FILTER: &[u32] = &[NEAREST, LINEAR];
}

impl From<SamplerMagFilter> for u32 {
    fn from(value: SamplerMagFilter) -> Self {
        match value {
            SamplerMagFilter::Nearest => NEAREST,
            SamplerMagFilter::Linear => LINEAR,
        }
    }
}

impl TryFrom<u32> for SamplerMagFilter {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as u32 {
            NEAREST => Ok(SamplerMagFilter::Nearest),
            LINEAR => Ok(SamplerMagFilter::Linear),
            _ => Err(()),
        }
    }
}
impl Serialize for SamplerMagFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<SamplerMagFilter> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<SamplerMagFilter>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "any of: {:?}",
                    SamplerMagFilter::VALID_SAMPLER_MAG_FILTER
                )
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value as u32)
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SamplerMinFilter {
    #[default]
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl From<SamplerMinFilter> for u32 {
    fn from(value: SamplerMinFilter) -> Self {
        match value {
            SamplerMinFilter::Nearest => NEAREST,
            SamplerMinFilter::Linear => LINEAR,
            SamplerMinFilter::NearestMipmapNearest => NEAREST_MIPMAP_NEAREST,
            SamplerMinFilter::LinearMipmapNearest => LINEAR_MIPMAP_NEAREST,
            SamplerMinFilter::NearestMipmapLinear => NEAREST_MIPMAP_LINEAR,
            SamplerMinFilter::LinearMipmapLinear => LINEAR_MIPMAP_LINEAR,
        }
    }
}

impl TryFrom<u32> for SamplerMinFilter {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            NEAREST => Ok(SamplerMinFilter::Nearest),
            LINEAR => Ok(SamplerMinFilter::Linear),
            NEAREST_MIPMAP_NEAREST => Ok(SamplerMinFilter::NearestMipmapNearest),
            LINEAR_MIPMAP_NEAREST => Ok(SamplerMinFilter::LinearMipmapNearest),
            NEAREST_MIPMAP_LINEAR => Ok(SamplerMinFilter::NearestMipmapLinear),
            LINEAR_MIPMAP_LINEAR => Ok(SamplerMinFilter::LinearMipmapLinear),
            _ => Err(()),
        }
    }
}

impl SamplerMinFilter {
    pub const VALID_SAMPLER_MIN_FILTER: &[u32] = &[
        NEAREST,
        LINEAR,
        NEAREST_MIPMAP_NEAREST,
        LINEAR_MIPMAP_NEAREST,
        NEAREST_MIPMAP_LINEAR,
        LINEAR_MIPMAP_LINEAR,
    ];
}

impl Serialize for SamplerMinFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<SamplerMinFilter> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<SamplerMinFilter>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "any of: {:?}",
                    SamplerMinFilter::VALID_SAMPLER_MIN_FILTER
                )
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value as u32)
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

pub const CLAMP_TO_EDGE: u32 = 33071;
pub const MIRRORED_REPEAT: u32 = 33648;
pub const REPEAT: u32 = 10497;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SamplerWrap {
    #[default]
    ClampToEdge,
    MirroredRepeat,
    Repeat,
}

impl From<SamplerWrap> for u32 {
    fn from(value: SamplerWrap) -> Self {
        match value {
            SamplerWrap::ClampToEdge => CLAMP_TO_EDGE,
            SamplerWrap::MirroredRepeat => MIRRORED_REPEAT,
            SamplerWrap::Repeat => REPEAT,
        }
    }
}

impl TryFrom<u32> for SamplerWrap {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            CLAMP_TO_EDGE => Ok(SamplerWrap::ClampToEdge),
            MIRRORED_REPEAT => Ok(SamplerWrap::MirroredRepeat),
            REPEAT => Ok(SamplerWrap::Repeat),
            _ => Err(()),
        }
    }
}

impl SamplerWrap {
    pub const VALID_SAMPLER_WRAP: &[u32] = &[CLAMP_TO_EDGE, MIRRORED_REPEAT, REPEAT];
}

impl Serialize for SamplerWrap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<SamplerWrap> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<SamplerWrap>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", SamplerWrap::VALID_SAMPLER_WRAP)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value as u32)
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate, Default)]
pub struct Sampler {
    #[serde(rename = "magFilter", default = "default_mag_filter")]
    pub mag_filter: Checked<SamplerMagFilter>,
    #[serde(rename = "minFilter", default = "default_min_filter")]
    pub min_filter: Checked<SamplerMinFilter>,
    #[serde(rename = "wrapS", default = "default_wrap")]
    pub wrap_s: Checked<SamplerWrap>,
    #[serde(rename = "wrapT", default = "default_wrap")]
    pub wrap_t: Checked<SamplerWrap>,
    pub name: Option<String>,
}

fn default_mag_filter() -> Checked<SamplerMagFilter> {
    Checked::Valid(SamplerMagFilter::Linear)
}

fn default_min_filter() -> Checked<SamplerMinFilter> {
    Checked::Valid(SamplerMinFilter::LinearMipmapNearest)
}

fn default_wrap() -> Checked<SamplerWrap> {
    Checked::Valid(SamplerWrap::Repeat)
}

pub const ALPHA: u32 = 6406;
pub const RGB: u32 = 6407;
pub const RGBA: u32 = 6408;
pub const LUMINANCE: u32 = 6409;
pub const LUMINANCE_ALPHA: u32 = 6410;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum TextureFormat {
    Alpha,
    Rgb,
    #[default]
    Rgba,
    Luminance,
    LuminanceAlpha,
}

impl From<TextureFormat> for u32 {
    fn from(value: TextureFormat) -> Self {
        match value {
            TextureFormat::Alpha => ALPHA,
            TextureFormat::Rgb => RGB,
            TextureFormat::Rgba => RGBA,
            TextureFormat::Luminance => LUMINANCE,
            TextureFormat::LuminanceAlpha => LUMINANCE_ALPHA,
        }
    }
}

impl TryFrom<u32> for TextureFormat {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            ALPHA => Ok(TextureFormat::Alpha),
            RGB => Ok(TextureFormat::Rgb),
            RGBA => Ok(TextureFormat::Rgba),
            LUMINANCE => Ok(TextureFormat::Luminance),
            LUMINANCE_ALPHA => Ok(TextureFormat::LuminanceAlpha),
            _ => Err(()),
        }
    }
}

impl TextureFormat {
    pub const VALID_TEXTURE_FORMAT: &[u32] = &[ALPHA, RGB, RGBA, LUMINANCE, LUMINANCE_ALPHA];
}

impl Serialize for TextureFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<TextureFormat> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<TextureFormat>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", TextureFormat::VALID_TEXTURE_FORMAT)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value as u32)
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

pub const TEXTURE_2D: u32 = 3553;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum TextureTarget {
    #[default]
    Texture2d,
}

impl From<TextureTarget> for u32 {
    fn from(value: TextureTarget) -> Self {
        match value {
            TextureTarget::Texture2d => TEXTURE_2D,
        }
    }
}

impl TryFrom<u32> for TextureTarget {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            TEXTURE_2D => Ok(TextureTarget::Texture2d),
            _ => Err(()),
        }
    }
}

impl TextureTarget {
    pub const VALID_TEXTURE_TARGET: &[u32] = &[TEXTURE_2D];
}

impl Serialize for TextureTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<TextureTarget> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<TextureTarget>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", TextureTarget::VALID_TEXTURE_TARGET)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value as u32)
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

pub const UNSIGNED_BYTE: u32 = 5121;
pub const UNSIGNED_SHORT5_6_5: u32 = 33635;
pub const UNSIGNED_SHORT4_4_4_4: u32 = 32819;
pub const UNSIGNED_SHORT5_5_5_1: u32 = 32820;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum TextureType {
    #[default]
    UnsignedByte,
    UnsignedShort5_6_5,
    UnsignedShort4_4_4_4,
    UnsignedShort5_5_5_1,
}

impl From<TextureType> for u32 {
    fn from(value: TextureType) -> Self {
        match value {
            TextureType::UnsignedByte => UNSIGNED_BYTE,
            TextureType::UnsignedShort5_6_5 => UNSIGNED_SHORT5_6_5,
            TextureType::UnsignedShort4_4_4_4 => UNSIGNED_SHORT4_4_4_4,
            TextureType::UnsignedShort5_5_5_1 => UNSIGNED_SHORT5_5_5_1,
        }
    }
}

impl TryFrom<u32> for TextureType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            UNSIGNED_BYTE => Ok(TextureType::UnsignedByte),
            UNSIGNED_SHORT5_6_5 => Ok(TextureType::UnsignedShort5_6_5),
            UNSIGNED_SHORT4_4_4_4 => Ok(TextureType::UnsignedShort4_4_4_4),
            UNSIGNED_SHORT5_5_5_1 => Ok(TextureType::UnsignedShort5_5_5_1),
            _ => Err(()),
        }
    }
}

impl TextureType {
    pub const VALID_TEXTURE_TYPE: &[u32] = &[
        UNSIGNED_BYTE,
        UNSIGNED_SHORT5_6_5,
        UNSIGNED_SHORT4_4_4_4,
        UNSIGNED_SHORT5_5_5_1,
    ];
}

impl Serialize for TextureType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<TextureType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<TextureType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", TextureType::VALID_TEXTURE_TYPE)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value as u32)
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct Texture {
    #[serde(default = "default_texture_format")]
    pub format: Checked<TextureFormat>,
    #[serde(rename = "internalFormat", default = "default_texture_format")]
    pub internal_format: Checked<TextureFormat>,
    pub sampler: StringIndex<Sampler>,
    pub source: StringIndex<Image>,
    #[serde(default = "default_texture_target")]
    pub target: Checked<TextureTarget>,
    #[serde(rename = "type", default = "default_texture_type")]
    pub type_: Checked<TextureType>,
    pub name: Option<String>,
}

impl Texture {
    pub fn new(source: StringIndex<Image>, sampler: StringIndex<Sampler>) -> Self {
        Self {
            source,
            format: Default::default(),
            internal_format: Default::default(),
            sampler,
            target: Default::default(),
            type_: Default::default(),
            name: None,
        }
    }
}

fn default_texture_format() -> Checked<TextureFormat> {
    Checked::Valid(TextureFormat::Rgba)
}

fn default_texture_target() -> Checked<TextureTarget> {
    Checked::Valid(TextureTarget::Texture2d)
}
fn default_texture_type() -> Checked<TextureType> {
    Checked::Valid(TextureType::UnsignedByte)
}

#[test]
fn test_sampler_deserialize() {
    let data = r#"{
            "magFilter": 9729,
            "minFilter": 9987,
            "name": "user-defined sampler name",
            "wrapS": 10497,
            "wrapT": 10497,
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let sampler: Result<Sampler, _> = serde_json::from_str(data);
    let sampler_unwrap = sampler.unwrap();
    println!("{}", serde_json::to_string(&sampler_unwrap).unwrap());
    assert_eq!(
        Some("user-defined sampler name".to_string()),
        sampler_unwrap.name
    );
}

#[test]
fn test_texture_deserialize() {
    let data = r#"{
            "format": 6408,
            "internalFormat": 6408,
            "name": "user-defined texture name",
            "sampler": "sampler_id",
            "source": "image_id",
            "target": 3553,
            "type": 5121,
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let texture: Result<Texture, _> = serde_json::from_str(data);
    let texture_unwrap = texture.unwrap();
    println!("{}", serde_json::to_string(&texture_unwrap).unwrap());
    assert_eq!(
        Some("user-defined texture name".to_string()),
        texture_unwrap.name
    );
}
