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
use enumflags2::{BitFlags, bitflags};

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

#[derive(Debug)]
pub enum InsertError {
    InvalidParent,
    RootAlreadyExists,
}

#[derive(Debug, PartialEq, Default)]
pub struct AiNodeTree {
    pub root: Option<usize>,
    pub arena: Vec<AiNode>,
}

impl AiNodeTree {
    /// Creates a new `AiNodeTree` with a single root node.
    ///
    /// This initializes the tree with:
    /// - A root node at index 0
    /// - An empty default node as the root
    /// - No children or parent references
    ///
    /// # Returns
    /// A new `AiNodeTree` instance with a single root node
    pub fn with_root() -> Self {
        Self {
            root: Some(0),
            arena: vec![AiNode::default()],
        }
    }

    pub fn merge(&mut self, other: AiNodeTree) {
        let offset = self.arena.len();
        let mut new_root_indices: Vec<usize> = Vec::with_capacity(offset);
        let mut new_nodes: Vec<AiNode> = other
            .arena
            .into_iter()
            .enumerate()
            .map(|(index, mut node)| {
                if let Some(parent_idx) = node.parent {
                    node.parent = Some(parent_idx + offset);
                } else {
                    node.parent = self.root;
                    new_root_indices.push(index);
                }
                for child in &mut node.children {
                    *child += offset;
                }
                node
            })
            .collect();
        self.arena.append(&mut new_nodes);
        if let Some(root) = self.root {
            if let Some(root_ele) = self.arena.get_mut(root) {
                root_ele.children.append(&mut new_root_indices);
            }
        }
    }

    pub fn insert(
        &mut self,
        mut node: AiNode,
        parent_index: Option<usize>,
    ) -> Result<usize, InsertError> {
        let new_index = self.arena.len();
        node.parent = parent_index;

        // Validate first
        match parent_index {
            Some(parent_idx) => {
                if self.arena.get(parent_idx).is_none() {
                    return Err(InsertError::InvalidParent);
                }
            }
            None => {
                if self.root.is_some() {
                    return Err(InsertError::RootAlreadyExists);
                }
            }
        }

        // Apply side effects
        match parent_index {
            Some(parent_idx) => {
                self.arena[parent_idx].children.push(new_index);
            }
            None => {
                self.root = Some(new_index);
            }
        }

        self.arena.push(node);
        Ok(new_index)
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
/// Represents a complete 3D scene containing all the elements needed for rendering.
///
/// This structure serves as the root container for all scene data, including:
/// - Scene name and metadata
/// - Scene-wide flags controlling behavior
/// - Node hierarchy defining the scene structure
/// - Collections of scene elements (meshes, materials, etc.)
///
/// # Fields
/// * `name` - The name of the scene
/// * `flags` - Bit flags controlling scene behavior and state
/// * `nodes` - The scene graph containing all nodes and their hierarchy
/// * `meshes` - Collection of all meshes in the scene
/// * `materials` - Collection of all materials used by meshes
/// * `animations` - Collection of all animations in the scene
/// * `textures` - Collection of all textures used by materials
/// * `lights` - Collection of all light sources in the scene
/// * `cameras` - Collection of all cameras in the scene
/// * `skeletons` - Collection of all skeletal animations
/// * `metadata` - Additional scene metadata and properties
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
