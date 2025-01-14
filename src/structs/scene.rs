use std::rc::Rc;

use super::{
    animation::AiAnimation,
    camera::AiCamera,
    light::AiLight,
    material::AiMaterial,
    matrix::AiMatrix4x4,
    mesh::{AiMesh, AiSkeleton},
    metadata::AiMetadata,
    texture::AiTexture,
};
use enumflags2::{bitflags, BitFlags};

#[derive(Debug, PartialEq)]
pub struct AiNode<'a> {
    pub name: String,
    pub parent: Option<&'a AiNode<'a>>,
    pub children: Vec<&'a AiNode<'a>>,
    pub meshes: Vec<&'a AiMesh<'a>>,
    pub transformation: AiMatrix4x4,
    pub metadata: Option<AiMetadata>,
}

impl<'a> Default for AiNode<'a> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            parent: Default::default(),
            children: Default::default(),
            meshes: Default::default(),
            transformation: AiMatrix4x4::identity(),
            metadata: Default::default(),
        }
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AiSceneFlag {
    Incomplete = 0x01,
    Validated = 0x02,
    ValidationWarning = 0x04,
    NonVerboseFormat = 0x08,
    Terrain = 0x10,
    AllowShared = 0x20,
}

#[derive(Debug, PartialEq)]
pub struct AiScene<'a> {
    name: String,
    flags: BitFlags<AiSceneFlag>,
    root: AiNode<'a>,
    meshes: Vec<AiMesh<'a>>,
    materials: Vec<Rc<AiMaterial>>,
    animations: Vec<Rc<AiAnimation>>,
    textures: Vec<Rc<AiTexture>>,
    lights: Vec<Rc<AiLight>>,
    cameras: Vec<Rc<AiCamera>>,
    skeletons: Vec<AiSkeleton<'a>>,
    metadata: AiMetadata,
}
