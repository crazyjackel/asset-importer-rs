use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use asset_importer_rs_core::AiExportError;
use asset_importer_rs_scene::{AiReal, AiScene};
use gltf_v1::json::{Node, Root, StringIndex};

use crate::{
    GltfExporter,
    exporter::{error::Error, generate_unique_name},
};

impl GltfExporter {
    pub(crate) fn export_nodes(
        &self,
        scene: &AiScene,
        root: &mut Root,
        config_epsilon: f32,
    ) -> Result<HashMap<usize, String>, Error> {
        if scene.nodes.arena.is_empty() {
            return Ok(HashMap::new());
        }
        let mut unique_names_map: HashMap<String, u32> = HashMap::new();
        let mut mesh_index_map = HashMap::new();
        let mut queue: VecDeque<(usize, Option<String>)> = VecDeque::new();
        queue.push_back((scene.nodes.root.unwrap_or(0), None));
        while let Some((node_index, parent_name)) = queue.pop_front() {
            let ai_node = &scene.nodes.arena[node_index];
            let mut node = Node::default();
            let base_name = if ai_node.name.is_empty() {
                "node"
            } else {
                &ai_node.name
            };
            node.name = Some(generate_unique_name(base_name, &mut unique_names_map));
            if let Some(parent_name) = parent_name {
                if let Some(parent) = root.nodes.get_mut(&parent_name) {
                    parent
                        .children
                        .push(StringIndex::new(node.name.clone().unwrap()));
                }
            }
            if !ai_node.transformation.is_identity(config_epsilon as AiReal) {
                node.matrix = Some(ai_node.transformation.clone().into());
            }
            for mesh_index in &ai_node.mesh_indexes {
                let ai_mesh = &scene.meshes[*mesh_index];
                let base_name = if ai_mesh.name.is_empty() {
                    "mesh"
                } else {
                    &ai_mesh.name
                };
                let unique_name = generate_unique_name(base_name, &mut unique_names_map);
                mesh_index_map.insert(*mesh_index, unique_name.clone());
                node.meshes.push(StringIndex::new(unique_name));
            }
            for child_index in &ai_node.children {
                queue.push_back((*child_index, node.name.clone()));
            }
            root.nodes.insert(node.name.clone().unwrap(), node);
        }
        Ok(mesh_index_map)
    }
}
