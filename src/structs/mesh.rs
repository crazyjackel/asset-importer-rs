use enumflags2::BitFlags;

use super::aabb::AiAABB;
use super::matrix::AiMatrix4x4;
use super::{color::AiColor4D, scene::AiNode, type_def::base_types::AiReal, vector::AiVector3D};

pub const AI_MAX_NUMBER_OF_COLORS_SETS: usize = 0x8;
pub const AI_MAX_NUMBER_OF_TEXTURECOORDS: usize = 0x8;

pub type AiFace = Vec<usize>;

#[derive(Debug, PartialEq, Clone)]
pub struct AiVertexWeight {
    pub vertex_id: usize,
    pub weight: AiReal,
}

impl AiVertexWeight {
    pub fn new(p_id: usize, p_weight: AiReal) -> Self {
        Self {
            vertex_id: p_id,
            weight: p_weight,
        }
    }
}

impl Default for AiVertexWeight {
    fn default() -> Self {
        Self {
            vertex_id: Default::default(),
            weight: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiBone {
    pub name: String,
    pub weights: Vec<AiVertexWeight>,
    pub node_index: usize,
    pub armature_index: usize,
    pub offset_matrix: AiMatrix4x4,
}

impl Default for AiBone {
    fn default() -> Self {
        Self {
            name: Default::default(),
            weights: Default::default(),
            node_index: Default::default(),
            armature_index: Default::default(),
            offset_matrix: Default::default(),
        }
    }
}

impl AiBone {
    fn copyVertexWeights(&mut self, other: AiBone) {
        if other.weights.is_empty() {
            self.weights.clear();
            return;
        }
        self.weights = other.weights.clone();
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiPrimitiveType {
    None = 0x00,
    Point = 0x01,
    Line = 0x02,
    Triangle = 0x04,
    Polygon = 0x08,
    NgonEncodingFlag = 0x10,
}

impl ::enumflags2::_internal::core::ops::Not for AiPrimitiveType {
    type Output = ::enumflags2::BitFlags<Self>;
    #[inline(always)]
    fn not(self) -> Self::Output {
        use ::enumflags2::BitFlags;
        BitFlags::from_flag(self).not()
    }
}
impl ::enumflags2::_internal::core::ops::BitOr for AiPrimitiveType {
    type Output = ::enumflags2::BitFlags<Self>;
    #[inline(always)]
    fn bitor(self, other: Self) -> Self::Output {
        use ::enumflags2::BitFlags;
        BitFlags::from_flag(self) | other
    }
}
impl ::enumflags2::_internal::core::ops::BitAnd for AiPrimitiveType {
    type Output = ::enumflags2::BitFlags<Self>;
    #[inline(always)]
    fn bitand(self, other: Self) -> Self::Output {
        use ::enumflags2::BitFlags;
        BitFlags::from_flag(self) & other
    }
}
impl ::enumflags2::_internal::core::ops::BitXor for AiPrimitiveType {
    type Output = ::enumflags2::BitFlags<Self>;
    #[inline(always)]
    fn bitxor(self, other: Self) -> Self::Output {
        use ::enumflags2::BitFlags;
        BitFlags::from_flag(self) ^ other
    }
}
unsafe impl ::enumflags2::_internal::RawBitFlags for AiPrimitiveType {
    type Numeric = u8;
    const EMPTY: Self::Numeric = 0;
    const DEFAULT: Self::Numeric = 0;
    const ALL_BITS: Self::Numeric = 0
        | (Self::Point as u8)
        | (Self::Line as u8)
        | (Self::Triangle as u8)
        | (Self::Polygon as u8)
        | (Self::NgonEncodingFlag as u8);
    const BITFLAGS_TYPE_NAME: &'static str = "BitFlags<AiPrimitiveType>";
    fn bits(self) -> Self::Numeric {
        self as u8
    }
}
impl ::enumflags2::BitFlag for AiPrimitiveType {}

impl AiPrimitiveType {
    const fn primitive_type_for_n_indices(n: u8) -> AiPrimitiveType {
        match n {
            4u8..=u8::MAX => AiPrimitiveType::Polygon,
            3 => AiPrimitiveType::Triangle,
            2 => AiPrimitiveType::Line,
            1 => AiPrimitiveType::Point,
            0 => AiPrimitiveType::None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiAnimMesh {
    pub name: String,
    pub vertices: Vec<AiVector3D>,
    pub normals: Vec<AiVector3D>,
    pub tangents: Vec<AiVector3D>,
    pub bi_tangents: Vec<AiVector3D>,
    pub colors: [Option<Vec<AiColor4D>>; AI_MAX_NUMBER_OF_COLORS_SETS],
    pub texture_coords: [Option<Vec<AiVector3D>>; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    pub weight: f32,
}

impl Default for AiAnimMesh {
    fn default() -> Self {
        Self {
            name: Default::default(),
            vertices: Default::default(),
            normals: Default::default(),
            tangents: Default::default(),
            bi_tangents: Default::default(),
            colors: Default::default(),
            texture_coords: Default::default(),
            weight: Default::default(),
        }
    }
}

impl AiAnimMesh {
    fn has_vertex_color(&self, p_index: usize) -> bool {
        if p_index >= AI_MAX_NUMBER_OF_COLORS_SETS {
            false
        } else {
            p_index < self.colors.len()
        }
    }
    fn has_texture_coord(&self, p_index: usize) -> bool {
        if p_index >= AI_MAX_NUMBER_OF_TEXTURECOORDS {
            false
        } else {
            p_index < self.texture_coords.len()
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiMorphingTarget {
    Unknown = 0x00,
    VertexBlend = 0x01,
    MorphNormalized = 0x02,
    MorphRelative = 0x03,
}

impl Default for AiMorphingTarget {
    fn default() -> Self {
        AiMorphingTarget::Unknown
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiMesh {
    pub name: String,
    pub primitive_types: BitFlags<AiPrimitiveType>,
    pub vertices: Vec<AiVector3D>,
    pub normals: Vec<AiVector3D>,
    pub tangents: Vec<AiVector3D>,
    pub bi_tangents: Vec<AiVector3D>,
    pub colors: [Option<Vec<AiColor4D>>; AI_MAX_NUMBER_OF_COLORS_SETS],
    pub texture_coords: [Option<Vec<AiVector3D>>; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    //pub num_uv_components: [u32; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    pub faces: Vec<AiFace>,
    pub bones: Vec<AiBone>,
    pub material_index: u32,
    pub anim_meshes: Vec<AiAnimMesh>,
    pub method: AiMorphingTarget,
    pub aabb: AiAABB,
    pub texture_coordinate_names: [String; AI_MAX_NUMBER_OF_TEXTURECOORDS],
}

impl Default for AiMesh {
    fn default() -> Self {
        Self {
            name: Default::default(),
            primitive_types: Default::default(),
            vertices: Default::default(),
            normals: Default::default(),
            tangents: Default::default(),
            bi_tangents: Default::default(),
            colors: Default::default(),
            texture_coords: Default::default(),
            //num_uv_components: Default::default(),
            faces: Default::default(),
            bones: Default::default(),
            material_index: Default::default(),
            anim_meshes: Default::default(),
            method: Default::default(),
            aabb: Default::default(),
            texture_coordinate_names: Default::default(),
        }
    }
}

impl AiMesh {
    fn get_num_uv_channels(&self) -> u32 {
        let mut n: u32 = 0;
        for i in 0..AI_MAX_NUMBER_OF_TEXTURECOORDS {
            if self.texture_coords[i].is_some() {
                n += 1;
            }
        }
        n
    }

    fn get_num_color_channels(&self) -> u32 {
        let mut n: u32 = 0;
        while n < AI_MAX_NUMBER_OF_COLORS_SETS
            .try_into()
            .unwrap_or_else(|_| u32::MAX)
            && self.colors[n as usize].is_some()
        {
            n += 1;
        }
        n
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiSkeletonBone {
    parent: i32,
    node_index: usize,
    armature_index: usize,
    mesh_index: usize,
    weights: Vec<AiVertexWeight>,
    offset_matrix: AiMatrix4x4,
    local_matrix: AiMatrix4x4,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiSkeleton {
    name: String,
    bones: Vec<AiSkeletonBone>,
}
