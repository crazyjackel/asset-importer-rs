use std::future::poll_fn;

use enumflags2::bitflags;

use super::{
    error::{AiFailure, AiReturnError},
    type_def::base_types::AiReal,
    vector::AiVector2D,
};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiTextureOp {
    Multiply,
    Add,
    Subtract,
    Divide,
    SmoothAdd,
    SignedAdd,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiTextureMapMode {
    Wrap,
    Clamp,
    Mirror,
    Decal,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiTextureMapping {
    UV,
    Sphere,
    Cylinder,
    Box,
    Plane,
    Other,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiTextureType {
    None,
    Diffuse,
    Specular,
    Ambient,
    Emissive,
    Height,
    Normals,
    Shininess,
    Opacity,
    Displacement,
    Lightmap,
    Reflection,
    BaseColor,
    NormalCamera,
    EmissionColor,
    Metalness,
    DiffuseRoughness,
    AmbientOcclusion,
    Unknown,
    Sheen,
    ClearCoat,
    Transmission,
    MayaBase,
    MayaSpecular,
    MayaSpecularColor,
    MayaSpecularRoughness,
}

impl ToString for AiTextureType {
    fn to_string(&self) -> String {
        match self {
            AiTextureType::None => String::from("n/a"),
            AiTextureType::Diffuse => String::from("Diffuse"),
            AiTextureType::Specular => String::from("Specular"),
            AiTextureType::Ambient => String::from("Ambient"),
            AiTextureType::Emissive => String::from("Emissive"),
            AiTextureType::Height => String::from("Height"),
            AiTextureType::Normals => String::from("Normals"),
            AiTextureType::Shininess => String::from("Shininess"),
            AiTextureType::Opacity => String::from("Opacity"),
            AiTextureType::Displacement => String::from("Displacement"),
            AiTextureType::Lightmap => String::from("Lightmap"),
            AiTextureType::Reflection => String::from("Reflection"),
            AiTextureType::BaseColor => String::from("BaseColor"),
            AiTextureType::NormalCamera => String::from("NormalCamera"),
            AiTextureType::EmissionColor => String::from("EmissionColor"),
            AiTextureType::Metalness => String::from("Metalness"),
            AiTextureType::DiffuseRoughness => String::from("DiffuseRoughness"),
            AiTextureType::AmbientOcclusion => String::from("AmbientOcclusion"),
            AiTextureType::Unknown => String::from("Unknown"),
            AiTextureType::Sheen => String::from("Sheen"),
            AiTextureType::ClearCoat => String::from("Clearcoat"),
            AiTextureType::Transmission => String::from("Transmission"),
            AiTextureType::MayaBase => String::from("MayaBase"),
            AiTextureType::MayaSpecular => String::from("MayaSpecular"),
            AiTextureType::MayaSpecularColor => String::from("MayaSpecularColor"),
            AiTextureType::MayaSpecularRoughness => String::from("MayaSpecularRoughness"),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiShadingMode {
    Flat,
    Gouraud,
    Phong,
    Blinn,
    Toon,
    OrenNayar,
    Minnaert,
    CookTorrance,
    Unlit,
    Fresnel,
    PBR,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum AiTextureFlags {
    Invert = 0x01,
    UseAlpha = 0x02,
    IgnoreAlpha = 0x04,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum AiBlendMode {
    Default,
    Additive,
}

#[derive(Debug, PartialEq)]
pub struct AiUvTransform {
    translation: AiVector2D,
    scaling: AiVector2D,
    rotation: AiReal,
}

impl Default for AiUvTransform {
    fn default() -> Self {
        Self {
            translation: AiVector2D::new(0f32, 0f32),
            scaling: AiVector2D::new(1f32, 1f32),
            rotation: 0f32,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiPropertyTypeInfo {
    Float = 0x01,
    Double = 0x02,
    String = 0x03,
    Integer = 0x04,
    Buffer = 0x05,
}

pub trait AiPropertyConvert{
    fn ai_property(&self) -> AiPropertyTypeInfo;
    fn to_binary(&self) -> Vec<u8>;
}

impl AiPropertyConvert for f32{
    fn ai_property(&self) -> AiPropertyTypeInfo {
        AiPropertyTypeInfo::Float
    }

    fn to_binary(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

#[derive(Debug, PartialEq)]
pub struct AiMaterialProperty {
    key: String,
    index: u32,
    semantic: AiTextureType,
    property_type: AiPropertyTypeInfo,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct AiMaterial {
    properties: Vec<AiMaterialProperty>,
}

impl Default for AiMaterial{
    fn default() -> Self {
        Self { properties: Default::default() }
    }
}

impl AiMaterial{
    pub fn new() -> Self{
        Self { properties: Default::default() }
    }
}

impl AiMaterial {
    pub fn add_property<T>(&self, key: String, 
        semantic_type: Option<AiTextureType>,
        property_type: Option<AiPropertyTypeInfo>,
        data: T)
        where T: AiPropertyConvert
    {

    }

    pub fn get_property(
        &self,
        key: String,
        semantic_type: Option<AiTextureType>,
        property_type: Option<AiPropertyTypeInfo>,
    ) -> Result<&AiMaterialProperty, AiReturnError> {
        for property in &self.properties {
            if property.key == key
                && (semantic_type == None || Some(property.semantic) == semantic_type)
                && (property_type == None || Some(property.property_type) == property_type)
            {
                return Ok(property);
            }
        }
        Err(AiReturnError::Failure(AiFailure))
    }

    pub fn get_real_vector(
        &self,
        key: String,
        semantic_type: Option<AiTextureType>,
        property_type: Option<AiPropertyTypeInfo>,
    ) -> Result<Vec<AiReal>, AiReturnError> {
        let property = self.get_property(key, semantic_type, property_type)?;
        match property.property_type {
            /* Conversion from Vec<u8> representing Vec<f32> to Vec<AiReal>
               Chunks u8 into [u8;4] to then convert to f32 and then cast as AiReal (either f32 or f64)
            */
            AiPropertyTypeInfo::Float => {
                let real_vec: Vec<AiReal> = property
                    .data
                    .chunks_exact(std::mem::size_of::<f32>())
                    .map(|chunk| {
                        chunk
                            .try_into()
                            .map(f32::from_le_bytes)
                            .map(AiReal::from)
                            .map_err(|_| AiReturnError::Failure(AiFailure))
                    })
                    .collect::<Result<Vec<AiReal>, AiReturnError>>()?;
                return Ok(real_vec);
            }
            /* Conversion from Vec<u8> representing Vec<f64> to Vec<AiReal>
               Chunks u8 into [u8;8] to then convert to f64 and then cast as AiReal (either f32 or f64)
            */
            AiPropertyTypeInfo::Double => {
                let real_vec: Vec<AiReal> = property
                    .data
                    .chunks_exact(std::mem::size_of::<f64>())
                    .map(|chunk| {
                        chunk
                            .try_into()
                            .map(f64::from_le_bytes)
                            .map(|x| x as AiReal)
                            .map_err(|_| AiReturnError::Failure(AiFailure))
                    })
                    .collect::<Result<Vec<AiReal>, AiReturnError>>()?;
                return Ok(real_vec);
            }
            /* Conversion from Vec<u8> representing Vec<i32> to Vec<AiReal>
               Chunks u8 into [u8;4] to then convert to i32 and then cast as AiReal (either f32 or f64)
            */
            AiPropertyTypeInfo::Integer => {
                let real_vec: Vec<AiReal> = property
                    .data
                    .chunks_exact(std::mem::size_of::<i32>())
                    .map(|chunk| {
                        chunk
                            .try_into()
                            .map(i32::from_le_bytes)
                            .map(|x| x as AiReal)
                            .map_err(|_| AiReturnError::Failure(AiFailure))
                    })
                    .collect::<Result<Vec<AiReal>, AiReturnError>>()?;
                return Ok(real_vec);
            }
            /* Conversion from Vec<u8> a String/Buffer
               Produces a UTF-8 Str to extract floats from
            */
            AiPropertyTypeInfo::String | AiPropertyTypeInfo::Buffer => {
                let buffer = String::from_utf8_lossy(property.data.as_slice());
                let real_vec: Vec<AiReal> = buffer
                    .split_ascii_whitespace()
                    .map(|x| {
                        x.parse::<AiReal>()
                            .map_err(|_| AiReturnError::Failure(AiFailure))
                    })
                    .collect::<Result<Vec<AiReal>, AiReturnError>>()?;
                return Ok(real_vec);
            }
        }
    }


}

/// Tests the code of get_real_vector's String | Buffer Match
/// Check that strings made up of joined floats can be parsed successfully
#[test]
fn base_vec_to_real_vec() {
    let base_vec = vec![0f32, 12f32, 509f32];
    let base_str = base_vec
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    let buffer_data: Vec<u8> = base_str.bytes().collect();
    let buffer = String::from_utf8_lossy(buffer_data.as_slice());
    let real_vec: Vec<AiReal> = buffer
        .split_ascii_whitespace()
        .map(|x| {
            x.parse::<AiReal>()
                .map_err(|_| AiReturnError::Failure(AiFailure))
        })
        .collect::<Result<Vec<AiReal>, AiReturnError>>()
        .unwrap();
    assert_eq!(base_vec, real_vec);
}
