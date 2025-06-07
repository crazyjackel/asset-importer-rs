use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use gltf_v1::Document;

use asset_importer_rs_core::AiReadError;
use asset_importer_rs_scene::{AiCamera, AiLight, AiNode, AiNodeTree, AiReal};

use super::{GltfImporter, mesh::IndexSpan};

impl GltfImporter {
    pub(crate) fn import_nodes(
        document: &Document,
        mesh_offsets: &HashMap<String, IndexSpan>,
        lights: &mut [AiLight],
        light_map: &HashMap<String, usize>,
        cameras: &mut [AiCamera],
        camera_map: &HashMap<String, usize>,
    ) -> Result<(AiNodeTree, String), AiReadError> {
        let scene = document
            .default_scene()
            .or_else(|| document.scenes().nth(0));
        if scene.is_none() {
            return Ok((AiNodeTree::default(), "".to_string()));
        }

        let asset_root_nodes: Vec<gltf_v1::Node<'_>> = scene.as_ref().unwrap().nodes().collect();

        match asset_root_nodes.len().cmp(&1) {
            Ordering::Less => {
                let ai_node = AiNode {
                    name: "ROOT".to_string(),
                    ..AiNode::default()
                };
                let mut default_node = AiNodeTree {
                    root: Some(0),
                    ..AiNodeTree::default()
                };
                default_node.arena.push(ai_node);
                Ok((
                    default_node,
                    scene.unwrap().name().unwrap_or("").to_string(),
                ))
            }
            Ordering::Equal => Ok((
                import_node(
                    asset_root_nodes[0].clone(),
                    mesh_offsets,
                    lights,
                    light_map,
                    cameras,
                    camera_map,
                )?,
                scene.unwrap().name().unwrap_or("").to_string(),
            )),
            Ordering::Greater => {
                let ai_node = AiNode {
                    name: "ROOT".to_string(),
                    ..AiNode::default()
                };
                let mut default_node = AiNodeTree {
                    root: Some(0),
                    ..AiNodeTree::default()
                };
                default_node.arena.push(ai_node);
                for asset_root_node in asset_root_nodes {
                    default_node.merge(import_node(
                        asset_root_node,
                        mesh_offsets,
                        lights,
                        light_map,
                        cameras,
                        camera_map,
                    )?);
                }
                Ok((
                    default_node,
                    scene.unwrap().name().unwrap_or("").to_string(),
                ))
            }
        }
    }
}

fn import_node(
    root_node: gltf_v1::Node<'_>,
    mesh_offsets: &HashMap<String, IndexSpan>,
    lights: &mut [AiLight],
    light_map: &HashMap<String, usize>,
    cameras: &mut [AiCamera],
    camera_map: &HashMap<String, usize>,
) -> Result<AiNodeTree, AiReadError> {
    let mut ai_node_tree = AiNodeTree::default();
    let mut node_queue: VecDeque<(gltf_v1::Node<'_>, Option<usize>)> = VecDeque::new();
    node_queue.push_back((root_node, None));
    while let Some((node, parent_index)) = node_queue.pop_front() {
        let index = ai_node_tree.arena.len();
        let mut ai_node = AiNode {
            children: Vec::with_capacity(node.children().len()),
            parent: parent_index,
            name: node
                .name()
                .map(|x| x.to_string())
                .unwrap_or(format!("{}", index)),
            transformation: node
                .transform()
                .matrix()
                .map(|x| x.map(|y| y as AiReal))
                .into(),
            ..Default::default()
        };

        let mut count: usize = 0;
        for mesh in node.meshes() {
            if let Some(IndexSpan(_, span)) = mesh_offsets.get(mesh.index()) {
                count += *span as usize;
            }
        }
        ai_node.mesh_indexes = Vec::with_capacity(count);
        for mesh in node.meshes() {
            if let Some(IndexSpan(start, span)) = mesh_offsets.get(mesh.index()) {
                for i in *start..(*start + *span) {
                    ai_node.mesh_indexes.push(i as usize);
                }
            }
        }

        if let Some(camera) = node.camera() {
            if let Some(name) = camera.name() {
                if let Some(index) = camera_map.get(name) {
                    if let Some(ai_camera) = cameras.get_mut(*index) {
                        ai_camera.name = ai_node.name.clone();
                    }
                }
            }
        }

        if let Some(light) = node.light() {
            if let Some(name) = light.name() {
                if let Some(index) = light_map.get(name) {
                    if let Some(ai_light) = lights.get_mut(*index) {
                        ai_light.name = ai_node.name.clone();
                    }
                }
            }
        }

        ai_node_tree.arena.push(ai_node);
        if let Some(parent) = parent_index {
            ai_node_tree.arena[parent].children.push(index);
        } else {
            ai_node_tree.root = Some(index);
        }
        for child in node.children() {
            node_queue.push_back((child, Some(index)));
        }
    }
    Ok(ai_node_tree)
}
