use std::collections::HashMap;

use gltf::json::{Index, Node, Root, scene::UnitQuaternion};

use asset_importer_rs_core::AiExportError;
use asset_importer_rs_scene::{AiReal, AiScene};

use super::gltf2_exporter::{Gltf2Exporter, generate_unique_name};

impl Gltf2Exporter {
    pub(crate) fn export_nodes(
        &self,
        scene: &AiScene,
        root: &mut Root,
        unique_names_map: &mut HashMap<String, u32>,
        name_to_camera_index: HashMap<String, usize>,
        config_epsilon: AiReal,
        use_translate_rotate_scale: bool,
    ) -> Result<HashMap<usize, Vec<usize>>, AiExportError> {
        let mut node_to_mesh_indexes: HashMap<usize, Vec<usize>> = HashMap::new();
        for (index, ai_node) in scene.nodes.arena.iter().enumerate() {
            let mut node = Node {
                name: Some(generate_unique_name(&ai_node.name, unique_names_map)),
                ..Node::default()
            };

            node.children = if ai_node.children.is_empty() {
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
            if !ai_node.transformation.is_identity(config_epsilon as AiReal) {
                if use_translate_rotate_scale {
                    let decompose = ai_node.transformation.clone().decompose();
                    let translation: [AiReal; 3] = decompose.translation.into();
                    node.translation = Some(translation.map(|x| x as f32));
                    let rotation: [AiReal; 4] = decompose.rotation.into();
                    node.rotation = Some(UnitQuaternion(rotation.map(|x| x as f32)));
                    let scale: [AiReal; 3] = decompose.scale.into();
                    node.scale = Some(scale.map(|x| x as f32));
                } else {
                    let transform_array: [AiReal; 16] = ai_node.transformation.clone().into();
                    node.matrix = Some(transform_array.map(|x| x as f32));
                }
            }

            //handle camera by finding matching names
            if let Some(name) = &node.name {
                node.camera = name_to_camera_index
                    .get(name)
                    .map(|x| Index::new(*x as u32));
            }

            //handle mesh
            node_to_mesh_indexes.insert(index, ai_node.mesh_indexes.clone());

            root.nodes.push(node);
        }
        Ok(node_to_mesh_indexes)
    }
}
