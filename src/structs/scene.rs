use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

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

#[derive(Debug, PartialEq, Clone)]
pub struct AiNode {
    pub name: String,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub mesh_indexes: Vec<usize>,
    pub transformation: AiMatrix4x4,
    pub metadata: Option<AiMetadata>,
}

impl Default for AiNode {
    fn default() -> Self {
        Self {
            name: Default::default(),
            parent: Default::default(),
            children: Default::default(),
            mesh_indexes: Default::default(),
            transformation: AiMatrix4x4::identity(),
            metadata: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AiNodeTree {
    pub root: Option<usize>,
    pub arena: Vec<AiNode>,
}

impl Default for AiNodeTree {
    fn default() -> Self {
        Self {
            root: Default::default(),
            arena: Default::default(),
        }
    }
}

impl AiNodeTree{
    pub fn merge(&mut self, other: AiNodeTree){
        let offset = self.arena.len();
        let mut new_nodes : Vec<AiNode> = other.arena.into_iter().map(|mut node|{
            if let Some(parent_idx) = node.parent{
                node.parent = Some(parent_idx + offset);
            }else{
                node.parent = self.root;
            }
            for child in &mut node.children{
                *child += offset;
            }
            node
        }).collect();
        self.arena.append(&mut new_nodes);
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
pub struct AiScene {
    name: String,
    flags: BitFlags<AiSceneFlag>,
    root: usize,
    nodes: AiNodeTree,
    meshes: Vec<AiMesh>,
    materials: Vec<AiMaterial>,
    animations: Vec<AiAnimation>,
    textures: Vec<AiTexture>,
    lights: Vec<AiLight>,
    cameras: Vec<AiCamera>,
    skeletons: Vec<AiSkeleton>,
    metadata: AiMetadata,
}
