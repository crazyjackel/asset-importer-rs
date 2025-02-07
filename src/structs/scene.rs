
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

#[derive(Debug, PartialEq, Default)]
pub struct AiNodeTree {
    pub root: Option<usize>,
    pub arena: Vec<AiNode>,
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

#[derive(Debug, PartialEq, Default)]
pub struct AiScene {
    pub name: String,
    pub flags: BitFlags<AiSceneFlag>,
    pub nodes: AiNodeTree,
    pub meshes: Vec<AiMesh>,
    pub materials: Vec<AiMaterial>,
    pub animations: Vec<AiAnimation>,
    pub textures: Vec<AiTexture>,
    pub lights: Vec<AiLight>,
    pub cameras: Vec<AiCamera>,
    pub skeletons: Vec<AiSkeleton>,
    pub metadata: AiMetadata,
}