use std::fmt;

use super::{image::Image, root::StringIndex, validation::Checked};
use serde::de;
use serde::{Deserialize, Serialize};

pub const NEAREST: u32 = 9728;
pub const LINEAR: u32 = 9729;
pub const NEAREST_MIPMAP_NEAREST: u32 = 9984;
pub const LINEAR_MIPMAP_NEAREST: u32 = 9985;
pub const NEAREST_MIPMAP_LINEAR: u32 = 9986;
pub const LINEAR_MIPMAP_LINEAR: u32 = 9987;

pub const VALID_SAMPLER_MAG_FILTER: &[u32] = &[NEAREST, LINEAR];
pub const VALID_SAMPLER_MIN_FILTER: &[u32] = &[
    NEAREST,
    LINEAR,
    NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SamplerMagFilter {
    Nearest,
    Linear,
}

impl SamplerMagFilter {
    pub const fn to_repr(self) -> u32 {
        match self {
            SamplerMagFilter::Nearest => NEAREST,
            SamplerMagFilter::Linear => LINEAR,
        }
    }
}
impl Serialize for SamplerMagFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_SAMPLER_MAG_FILTER)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::SamplerMagFilter::*;
                Ok(match value as u32 {
                    NEAREST => Checked::Valid(Nearest),
                    LINEAR => Checked::Valid(Linear),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SamplerMinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl SamplerMinFilter {
    pub const fn to_repr(self) -> u32 {
        match self {
            SamplerMinFilter::Nearest => NEAREST,
            SamplerMinFilter::Linear => LINEAR,
            SamplerMinFilter::NearestMipmapNearest => NEAREST_MIPMAP_NEAREST,
            SamplerMinFilter::LinearMipmapNearest => LINEAR_MIPMAP_NEAREST,
            SamplerMinFilter::NearestMipmapLinear => NEAREST_MIPMAP_LINEAR,
            SamplerMinFilter::LinearMipmapLinear => LINEAR_MIPMAP_LINEAR,
        }
    }
}
impl Serialize for SamplerMinFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_SAMPLER_MIN_FILTER)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::SamplerMinFilter::*;
                Ok(match value as u32 {
                    NEAREST => Checked::Valid(Nearest),
                    LINEAR => Checked::Valid(Linear),
                    NEAREST_MIPMAP_NEAREST => Checked::Valid(NearestMipmapNearest),
                    LINEAR_MIPMAP_NEAREST => Checked::Valid(LinearMipmapNearest),
                    NEAREST_MIPMAP_LINEAR => Checked::Valid(NearestMipmapLinear),
                    LINEAR_MIPMAP_LINEAR => Checked::Valid(LinearMipmapLinear),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

pub const CLAMP_TO_EDGE: u32 = 33071;
pub const MIRRORED_REPEAT: u32 = 33648;
pub const REPEAT: u32 = 10497;

pub const VALID_SAMPLER_WRAP: &[u32] = &[CLAMP_TO_EDGE, MIRRORED_REPEAT, REPEAT];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SamplerWrap {
    ClampToEdge,
    MirroredRepeat,
    Repeat,
}

impl SamplerWrap {
    pub const fn to_repr(self) -> u32 {
        match self {
            SamplerWrap::ClampToEdge => CLAMP_TO_EDGE,
            SamplerWrap::MirroredRepeat => MIRRORED_REPEAT,
            SamplerWrap::Repeat => REPEAT,
        }
    }
}
impl Serialize for SamplerWrap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_SAMPLER_WRAP)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::SamplerWrap::*;
                Ok(match value as u32 {
                    CLAMP_TO_EDGE => Checked::Valid(ClampToEdge),
                    MIRRORED_REPEAT => Checked::Valid(MirroredRepeat),
                    REPEAT => Checked::Valid(Repeat),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct Sampler {
    #[serde(rename = "magFilter", skip_serializing_if = "Option::is_none")]
    mag_filter: Option<Checked<SamplerMagFilter>>,
    #[serde(rename = "minFilter", skip_serializing_if = "Option::is_none")]
    min_filter: Option<Checked<SamplerMinFilter>>,
    #[serde(rename = "wrapS", skip_serializing_if = "Option::is_none")]
    wrap_s: Option<Checked<SamplerWrap>>,
    #[serde(rename = "wrapT", skip_serializing_if = "Option::is_none")]
    wrap_t: Option<Checked<SamplerWrap>>,
    name: Option<String>,
}

pub const ALPHA: u32 = 6406;
pub const RGB: u32 = 6407;
pub const RGBA: u32 = 6408;
pub const LUMINANCE: u32 = 6409;
pub const LUMINANCE_ALPHA: u32 = 6410;

pub const VALID_TEXTURE_FORMAT: &[u32] = &[ALPHA, RGB, RGBA, LUMINANCE, LUMINANCE_ALPHA];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureFormat {
    Alpha,
    Rgb,
    Rgba,
    Luminance,
    LuminanceAlpha,
}

impl TextureFormat {
    pub const fn to_repr(self) -> u32 {
        match self {
            TextureFormat::Alpha => ALPHA,
            TextureFormat::Rgb => RGB,
            TextureFormat::Rgba => RGBA,
            TextureFormat::Luminance => LUMINANCE,
            TextureFormat::LuminanceAlpha => LUMINANCE_ALPHA,
        }
    }
}

impl Serialize for TextureFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_TEXTURE_FORMAT)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::TextureFormat::*;
                Ok(match value as u32 {
                    ALPHA => Checked::Valid(Alpha),
                    RGB => Checked::Valid(Rgb),
                    RGBA => Checked::Valid(Rgba),
                    LUMINANCE => Checked::Valid(Luminance),
                    LUMINANCE_ALPHA => Checked::Valid(LuminanceAlpha),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

pub const TEXTURE_2D: u32 = 3553;

pub const VALID_TEXTURE_TARGET: &[u32] = &[TEXTURE_2D];
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureTarget {
    Texture2d,
}

impl TextureTarget {
    pub const fn to_repr(self) -> u32 {
        match self {
            TextureTarget::Texture2d => TEXTURE_2D,
        }
    }
}

impl Serialize for TextureTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_TEXTURE_TARGET)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::TextureTarget::*;
                Ok(match value as u32 {
                    TEXTURE_2D => Checked::Valid(Texture2d),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

pub const UNSIGNED_BYTE: u32 = 5121;
pub const UNSIGNED_SHORT5_6_5: u32 = 33635;
pub const UNSIGNED_SHORT4_4_4_4: u32 = 32819;
pub const UNSIGNED_SHORT5_5_5_1: u32 = 32820;

pub const VALID_TEXTURE_TYPE: &[u32] = &[
    UNSIGNED_BYTE,
    UNSIGNED_SHORT5_6_5,
    UNSIGNED_SHORT4_4_4_4,
    UNSIGNED_SHORT5_5_5_1,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TextureType {
    UnsignedByte,
    UnsignedShort5_6_5,
    UnsignedShort4_4_4_4,
    UnsignedShort5_5_5_1,
}

impl TextureType {
    pub const fn to_repr(self) -> u32 {
        match self {
            TextureType::UnsignedByte => UNSIGNED_BYTE,
            TextureType::UnsignedShort5_6_5 => UNSIGNED_SHORT5_6_5,
            TextureType::UnsignedShort4_4_4_4 => UNSIGNED_SHORT4_4_4_4,
            TextureType::UnsignedShort5_5_5_1 => UNSIGNED_SHORT5_5_5_1,
        }
    }
}

impl Serialize for TextureType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_TEXTURE_TYPE)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::TextureType::*;
                Ok(match value as u32 {
                    UNSIGNED_BYTE => Checked::Valid(UnsignedByte),
                    UNSIGNED_SHORT5_6_5 => Checked::Valid(UnsignedShort5_6_5),
                    UNSIGNED_SHORT4_4_4_4 => Checked::Valid(UnsignedShort4_4_4_4),
                    UNSIGNED_SHORT5_5_5_1 => Checked::Valid(UnsignedShort5_5_5_1),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct Texture {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Checked<TextureFormat>>,
    #[serde(rename = "internalFormat", skip_serializing_if = "Option::is_none")]
    internal_format: Option<Checked<TextureFormat>>,
    sampler: StringIndex<Sampler>,
    source: StringIndex<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Checked<TextureTarget>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    texture_type: Option<Checked<TextureType>>,
    name: Option<String>,
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
