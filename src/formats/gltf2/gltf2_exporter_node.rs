use std::collections::HashMap;

use gltf::json::{scene::UnitQuaternion, Index, Node, Root};

use crate::{core::error::AiExportError, structs::{base_types::AiReal, scene::AiScene}};

use super::gltf2_exporter::{generate_unique_name, Gltf2Exporter};

impl Gltf2Exporter {
    pub(crate) fn export_nodes(
        &self,
        scene: &AiScene,
        root: &mut Root,
        unique_names_map: &mut HashMap<String, u32>,
        name_to_camera_index: HashMap<String, usize>,
        config_epsilon: f32,
        use_translate_rotate_scale: bool
    ) -> Result<HashMap<usize, Vec<usize>>, AiExportError> {
        let mut node_to_mesh_indexes: HashMap<usize, Vec<usize>> = HashMap::new();
        for (index, ai_node) in scene.nodes.arena.iter().enumerate() {
            let mut node = Node::default();
            node.name = Some(generate_unique_name("node", unique_names_map));

            node.children = if ai_node.children.len() == 0 {
                None
            } else {
                Some(
                    ai_node
                        .children
                        .iter()
                        .map(|x| Index::new(*x as u32))
                        .collect(),
                )
            };
            if ai_node.transformation.is_identity(config_epsilon as AiReal){
                if use_translate_rotate_scale{
                    let decompose = ai_node.transformation.clone().decompose();
                    node.translation = Some(decompose.translation.into());
                    node.rotation = Some(UnitQuaternion(decompose.rotation.into()));
                    node.scale = Some(decompose.scale.into());
                }
                else{
                    node.matrix = Some(ai_node.transformation.clone().into());
                }
            }

            //handle camera by finding matching names
            if let Some(name) = &node.name{
                node.camera = name_to_camera_index.get(name).map(|x| Index::new(*x as u32));
            }

            //handle mesh
            node_to_mesh_indexes.insert(index, ai_node.mesh_indexes.clone());
            
            root.nodes.push(node);
        }
        Ok(node_to_mesh_indexes)
    }
}
