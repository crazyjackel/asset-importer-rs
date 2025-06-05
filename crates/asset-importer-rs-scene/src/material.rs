use std::{fmt::Display, string::FromUtf8Error};

use bytemuck::{Pod, Zeroable};
use enumflags2::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::error::{AiFailure, AiReturnError};

use super::{AiColor3D, AiColor4D, type_def::base_types::AiReal, vector::AiVector2D};

//@todo Add an Enum to Matkey that can be used to convert to and from binary based on format
pub mod matkey {
    use super::AiTextureType;

    pub const AI_MATKEY_NAME: &str = "?mat.name";
    pub const AI_MATKEY_TWOSIDED: &str = "$mat.twosided";
    pub const AI_MATKEY_SHADING_MODEL: &str = "$mat.shadingm";
    pub const AI_MATKEY_ENABLE_WIREFRAME: &str = "$mat.wireframe";
    pub const AI_MATKEY_BLEND_FUNC: &str = "$mat.blend";
    pub const AI_MATKEY_OPACITY: &str = "$mat.opacity";
    pub const AI_MATKEY_TRANSPARENCYFACTOR: &str = "$mat.transparencyfactor";
    pub const AI_MATKEY_BUMPSCALING: &str = "$mat.bumpscaling";
    pub const AI_MATKEY_SHININESS: &str = "$mat.shininess";
    pub const AI_MATKEY_REFLECTIVITY: &str = "$mat.reflectivity";
    pub const AI_MATKEY_SHININESS_STRENGTH: &str = "$mat.shinpercent";
    pub const AI_MATKEY_REFRACTI: &str = "$mat.refracti";
    pub const AI_MATKEY_COLOR_DIFFUSE: &str = "$clr.diffuse";
    pub const AI_MATKEY_COLOR_AMBIENT: &str = "$clr.ambient";
    pub const AI_MATKEY_COLOR_SPECULAR: &str = "$clr.specular";
    pub const AI_MATKEY_COLOR_EMISSIVE: &str = "$clr.emissive";
    pub const AI_MATKEY_COLOR_TRANSPARENT: &str = "$clr.transparent";
    pub const AI_MATKEY_COLOR_REFLECTIVE: &str = "$clr.reflective";
    pub const AI_MATKEY_GLOBAL_BACKGROUND_IMAGE: &str = "?bg.global";
    pub const AI_MATKEY_GLOBAL_SHADERLANG: &str = "?sh.lang";
    pub const AI_MATKEY_SHADER_VERTEX: &str = "?sh.vs";
    pub const AI_MATKEY_SHADER_FRAGMENT: &str = "?sh.fs";
    pub const AI_MATKEY_SHADER_GEO: &str = "?sh.gs";
    pub const AI_MATKEY_SHADER_TESSELATION: &str = "?sh.ts";
    pub const AI_MATKEY_SHADER_PRIMITIVE: &str = "?sh.ps";
    pub const AI_MATKEY_SHADER_COMPUTE: &str = "?sh.cs";

    // ---------------------------------------------------------------------------
    // PBR material support
    // --------------------
    // Properties defining PBR rendering techniques
    pub const AI_MATKEY_USE_COLOR_MAP: &str = "$mat.useColorMap";

    // Metallic/Roughness Workflow
    // ---------------------------
    // Base RGBA color factor. Will be multiplied by final base color texture values if extant
    // Note: Importers may choose to copy this into AI_MATKEY_COLOR_DIFFUSE for compatibility
    // with renderers and formats that do not support Metallic/Roughness PBR
    pub const AI_MATKEY_BASE_COLOR: &str = "$clr.base";
    pub const AI_MATKEY_BASE_COLOR_TEXTURE: AiTextureType = AiTextureType::BaseColor;
    pub const AI_MATKEY_USE_METALLIC_MAP: &str = "$mat.useMetallicMap";
    // Metallic factor. 0.0 = Full Dielectric, 1.0 = Full Metal
    pub const AI_MATKEY_METALLIC_FACTOR: &str = "$mat.metallicFactor";
    pub const AI_MATKEY_METALLIC_TEXTURE: AiTextureType = AiTextureType::Metalness;
    pub const AI_MATKEY_USE_ROUGHNESS_MAP: &str = "$mat.useRoughnessMap";
    // Roughness factor. 0.0 = Perfectly Smooth, 1.0 = Completely Rough
    pub const AI_MATKEY_ROUGHNESS_FACTOR: &str = "$mat.roughnessFactor";
    pub const AI_MATKEY_ROUGHNESS_TEXTURE: AiTextureType = AiTextureType::DiffuseRoughness;
    // Anisotropy factor. 0.0 = isotropic, 1.0 = anisotropy along tangent direction,
    // -1.0 = anisotropy along bitangent direction
    pub const AI_MATKEY_ANISOTROPY_FACTOR: &str = "$mat.anisotropyFactor";

    // Specular/Glossiness Workflow
    // ---------------------------
    // Diffuse/Albedo Color. Note: Pure Metals have a diffuse of {0,0,0}
    // AI_MATKEY_COLOR_DIFFUSE
    // Specular Color.
    // Note: Metallic/Roughness may also have a Specular Color
    // AI_MATKEY_COLOR_SPECULAR
    pub const AI_MATKEY_SPECULAR_FACTOR: &str = "$mat.specularFactor";
    // Glossiness factor. 0.0 = Completely Rough, 1.0 = Perfectly Smooth
    pub const AI_MATKEY_GLOSSINESS_FACTOR: &str = "$mat.glossinessFactor";

    // Sheen
    // -----
    // Sheen base RGB color. Default {0,0,0}
    pub const AI_MATKEY_SHEEN_COLOR_FACTOR: &str = "$clr.sheen.factor";
    // Sheen Roughness Factor.
    pub const AI_MATKEY_SHEEN_ROUGHNESS_FACTOR: &str = "$mat.sheen.roughnessFactor";

    /// Sheen Color Textures use String PropertyType
    pub const AI_MATKEY_SHEEN_COLOR_TEXTURE: AiTextureType = AiTextureType::Sheen;
    /// Sheen Roughness Textures use FloatArray PropertyType
    pub const AI_MATKEY_SHEEN_ROUGHNESS_TEXTURE: AiTextureType = AiTextureType::Sheen;

    // Clearcoat
    // ---------
    // Clearcoat layer intensity. 0.0 = none (disabled)
    pub const AI_MATKEY_CLEARCOAT_FACTOR: &str = "$mat.clearcoat.factor";
    pub const AI_MATKEY_CLEARCOAT_ROUGHNESS_FACTOR: &str = "$mat.clearcoat.roughnessFactor";
    /// Sheen Color Textures use String PropertyType
    pub const AI_MATKEY_CLEARCOAT_TEXTURE: AiTextureType = AiTextureType::ClearCoat;
    /// Sheen Roughness Textures use FloatArray PropertyType
    pub const AI_MATKEY_CLEARCOAT_ROUGHNESS_TEXTURE: AiTextureType = AiTextureType::ClearCoat;
    /// Sheen Roughness Textures use DoubleArray PropertyType
    pub const AI_MATKEY_CLEARCOAT_NORMAL_TEXTURE: AiTextureType = AiTextureType::ClearCoat;

    // Transmission
    // ------------
    // https://github.com/KhronosGroup/glTF/tree/master/extensions/2.0/Khronos/KHR_materials_transmission
    // Base percentage of light transmitted through the surface. 0.0 = Opaque, 1.0 = Fully transparent
    pub const AI_MATKEY_TRANSMISSION_FACTOR: &str = "$mat.transmission.factor";
    // Texture defining percentage of light transmitted through the surface.
    // Multiplied by AI_MATKEY_TRANSMISSION_FACTOR
    pub const AI_MATKEY_TRANSMISSION_TEXTURE: AiTextureType = AiTextureType::Transmission;

    // Volume
    // ------------
    // https://github.com/KhronosGroup/glTF/tree/main/extensions/2.0/Khronos/KHR_materials_volume
    // The thickness of the volume beneath the surface. If the value is 0 the material is thin-walled. Otherwise the material is a volume boundary.
    pub const AI_MATKEY_VOLUME_THICKNESS_FACTOR: &str = "$mat.volume.thicknessFactor";
    // Texture that defines the thickness.
    // Multiplied by AI_MATKEY_THICKNESS_FACTOR
    pub const AI_MATKEY_VOLUME_THICKNESS_TEXTURE: AiTextureType = AiTextureType::Transmission;
    // Density of the medium given as the average distance that light travels in the medium before interacting with a particle.
    pub const AI_MATKEY_VOLUME_ATTENUATION_DISTANCE: &str = "$mat.volume.attenuationDistance";
    // The color that white light turns into due to absorption when reaching the attenuation distance.
    pub const AI_MATKEY_VOLUME_ATTENUATION_COLOR: &str = "$mat.volume.attenuationColor";

    // Emissive
    // --------
    pub const AI_MATKEY_USE_EMISSIVE_MAP: &str = "$mat.useEmissiveMap";
    pub const AI_MATKEY_EMISSIVE_INTENSITY: &str = "$mat.emissiveIntensity";
    pub const AI_MATKEY_USE_AO_MAP: &str = "$mat.useAOMap";

    // ---------------------------------------------------------------------------
    // Pure key names for all texture-related properties
    pub const _AI_MATKEY_TEXTURE_BASE: &str = "$tex.file";
    pub const _AI_MATKEY_UVWSRC_BASE: &str = "$tex.uvwsrc";
    pub const _AI_MATKEY_TEXOP_BASE: &str = "$tex.op";
    pub const _AI_MATKEY_MAPPING_BASE: &str = "$tex.mapping";
    pub const _AI_MATKEY_TEXBLEND_BASE: &str = "$tex.blend";
    pub const _AI_MATKEY_MAPPINGMODE_U_BASE: &str = "$tex.mapmodeu";
    pub const _AI_MATKEY_MAPPINGMODE_V_BASE: &str = "$tex.mapmodev";
    pub const _AI_MATKEY_TEXMAP_AXIS_BASE: &str = "$tex.mapaxis";
    pub const _AI_MATKEY_UVTRANSFORM_BASE: &str = "$tex.uvtrafo";
    pub const _AI_MATKEY_TEXFLAGS_BASE: &str = "$tex.flags";
}

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
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum AiTextureMapMode {
    Wrap = 3,
    #[default]
    Clamp = 1,
    Mirror = 2,
    Decal = 4,
}

impl TryFrom<u8> for AiTextureMapMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Clamp),
            2 => Ok(Self::Mirror),
            3 => Ok(Self::Wrap),
            4 => Ok(Self::Decal),
            _ => Err(()),
        }
    }
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

impl Display for AiTextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiTextureType::None => f.write_str("n/a"),
            AiTextureType::Diffuse => f.write_str("Diffuse"),
            AiTextureType::Specular => f.write_str("Specular"),
            AiTextureType::Ambient => f.write_str("Ambient"),
            AiTextureType::Emissive => f.write_str("Emissive"),
            AiTextureType::Height => f.write_str("Height"),
            AiTextureType::Normals => f.write_str("Normals"),
            AiTextureType::Shininess => f.write_str("Shininess"),
            AiTextureType::Opacity => f.write_str("Opacity"),
            AiTextureType::Displacement => f.write_str("Displacement"),
            AiTextureType::Lightmap => f.write_str("Lightmap"),
            AiTextureType::Reflection => f.write_str("Reflection"),
            AiTextureType::BaseColor => f.write_str("BaseColor"),
            AiTextureType::NormalCamera => f.write_str("NormalCamera"),
            AiTextureType::EmissionColor => f.write_str("EmissionColor"),
            AiTextureType::Metalness => f.write_str("Metalness"),
            AiTextureType::DiffuseRoughness => f.write_str("DiffuseRoughness"),
            AiTextureType::AmbientOcclusion => f.write_str("AmbientOcclusion"),
            AiTextureType::Unknown => f.write_str("Unknown"),
            AiTextureType::Sheen => f.write_str("Sheen"),
            AiTextureType::ClearCoat => f.write_str("Clearcoat"),
            AiTextureType::Transmission => f.write_str("Transmission"),
            AiTextureType::MayaBase => f.write_str("MayaBase"),
            AiTextureType::MayaSpecular => f.write_str("MayaSpecular"),
            AiTextureType::MayaSpecularColor => f.write_str("MayaSpecularColor"),
            AiTextureType::MayaSpecularRoughness => f.write_str("MayaSpecularRoughness"),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
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

#[repr(C)]
#[derive(Debug, PartialEq, Pod, Zeroable, Clone, Copy)]
pub struct AiUvTransform {
    pub translation: AiVector2D,
    pub scaling: AiVector2D,
    pub rotation: AiReal,
}

impl Default for AiUvTransform {
    fn default() -> Self {
        Self {
            translation: AiVector2D::new(0.0, 0.0),
            scaling: AiVector2D::new(1.0, 1.0),
            rotation: 0.0,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum AiPropertyTypeInfo {
    Binary(Vec<u8>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    StringArray(Vec<String>),
    IntegerArray(Vec<u32>),
    Buffer(Vec<u8>),
}

impl AiPropertyTypeInfo {
    pub fn variant_eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (AiPropertyTypeInfo::Binary(_), AiPropertyTypeInfo::Binary(_))
                | (
                    AiPropertyTypeInfo::FloatArray(_),
                    AiPropertyTypeInfo::FloatArray(_)
                )
                | (
                    AiPropertyTypeInfo::DoubleArray(_),
                    AiPropertyTypeInfo::DoubleArray(_)
                )
                | (
                    AiPropertyTypeInfo::StringArray(_),
                    AiPropertyTypeInfo::StringArray(_)
                )
                | (
                    AiPropertyTypeInfo::IntegerArray(_),
                    AiPropertyTypeInfo::IntegerArray(_)
                )
                | (AiPropertyTypeInfo::Buffer(_), AiPropertyTypeInfo::Buffer(_))
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct AiMaterialProperty {
    //Property Key
    pub key: String,
    //Texture Index. Used only for Texture Properties, elsewise 0. Typically used for Texture Slot Reference
    pub index: u32,
    //Texture Semantic. Used only for Texture Properties, elsewise None. Typically used for determining how a Texture wants to be rendered.
    pub semantic: AiTextureType,
    //Property Type Info, encodes data on how a Property works
    pub property_type: AiPropertyTypeInfo,
}

#[derive(Debug, PartialEq, Default)]
pub struct AiMaterial {
    properties: Vec<AiMaterialProperty>,
}

impl AiMaterial {
    pub fn new() -> Self {
        Self {
            properties: Default::default(),
        }
    }
}

impl AiMaterial {
    pub fn iter(&self) -> core::slice::Iter<AiMaterialProperty> {
        self.properties.iter()
    }
}

impl AiMaterial {
    pub fn get_property_type_info(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<&AiPropertyTypeInfo> {
        for property in &self.properties {
            if property.key == key
                && (semantic_type.is_none() || semantic_type.unwrap() == property.semantic)
                && property.index == index
            {
                return Some(&property.property_type);
            }
        }
        None
    }
    pub fn get_property_type_info_mut(
        &mut self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<&mut AiPropertyTypeInfo> {
        for property in self.properties.iter_mut() {
            if property.key == key
                && (semantic_type.is_none() || semantic_type.unwrap() == property.semantic)
                && property.index == index
            {
                return Some(&mut property.property_type);
            }
        }
        None
    }

    pub fn add_property(
        &mut self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        property_type: AiPropertyTypeInfo,
        index: u32,
    ) -> bool {
        if let Some(property) = self.get_property_type_info_mut(key, semantic_type, index) {
            *property = property_type;
            return true;
        } else if let Some(semantic) = semantic_type {
            self.properties.push(AiMaterialProperty {
                key: key.to_owned(),
                index,
                semantic,
                property_type,
            });
        }
        false
    }

    pub fn get_property_ai_color_rgba(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<AiColor4D> {
        self.get_property_type_info(key, semantic_type, index)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) => {
                    bytemuck::try_from_bytes::<AiColor4D>(binary).copied().ok()
                }
                _ => None,
            })
    }
    pub fn get_property_ai_color_rgb(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<AiColor3D> {
        self.get_property_type_info(key, semantic_type, index)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) => {
                    bytemuck::try_from_bytes::<AiColor3D>(binary).copied().ok()
                }
                _ => None,
            })
    }
    pub fn get_property_ai_float(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<f32> {
        self.get_property_type_info(key, semantic_type, index)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) if binary.len() == 4 => {
                    Some(f32::from_le_bytes([
                        binary[0], binary[1], binary[2], binary[3],
                    ]))
                }
                _ => None,
            })
    }

    pub fn get_property_ai_str(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<Result<String, FromUtf8Error>> {
        self.get_property_type_info(key, semantic_type, index)
            .and_then(|x| match x {
                AiPropertyTypeInfo::Binary(binary) => {
                    let str = String::from_utf8(binary.to_vec());
                    Some(str)
                }
                _ => None,
            })
    }

    pub fn get_property_ai_bool(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<bool> {
        self.get_property_type_info(key, semantic_type, index)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) if binary.len() == 1 => Some(binary[0] != 0),
                _ => None,
            })
    }

    pub fn get_property_byte(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<u8> {
        self.get_property_type_info(key, semantic_type, index)
            .and_then(|info| match info {
                AiPropertyTypeInfo::Binary(binary) if binary.len() == 1 => Some(binary[0]),
                _ => None,
            })
    }

    pub fn get_property_real_vec(
        &self,
        key: &str,
        semantic_type: Option<AiTextureType>,
        index: u32,
    ) -> Option<Vec<AiReal>> {
        let property = self.get_property_type_info(key, semantic_type, index)?;
        match property {
            AiPropertyTypeInfo::FloatArray(vec) => Some(vec.iter().map(|x| *x as AiReal).collect()),
            AiPropertyTypeInfo::DoubleArray(vec) => {
                Some(vec.iter().map(|x| *x as AiReal).collect())
            }
            AiPropertyTypeInfo::IntegerArray(vec) => {
                Some(vec.iter().map(|x| *x as AiReal).collect())
            }
            AiPropertyTypeInfo::StringArray(vec) => vec
                .iter()
                .map(|s| s.parse::<AiReal>())
                .collect::<Result<Vec<AiReal>, _>>()
                .ok(),
            AiPropertyTypeInfo::Binary(vec) | AiPropertyTypeInfo::Buffer(vec) => {
                String::from_utf8_lossy(vec.as_slice())
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<AiReal>())
                    .collect::<Result<Vec<AiReal>, _>>()
                    .ok()
            }
        }
    }
}

/// Tests the code of get_real_vector's String | Buffer Match
/// Check that strings made up of joined floats can be parsed successfully
#[test]
fn base_vec_to_real_vec() {
    let base_vec = vec![0.0, 12.0, 509.0];
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
