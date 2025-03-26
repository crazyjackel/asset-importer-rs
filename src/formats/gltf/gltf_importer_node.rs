use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use gltf_v1::{
    buffer::Data,
    json::{map::IndexMap, node},
    Document,
};

use crate::{
    core::error::AiReadError,
    structs::{base_types::AiReal, AiNode, AiNodeTree},
};

use super::{gltf_importer::GltfImporter, gltf_importer_mesh::IndexSpan};

impl GltfImporter {
    pub(crate) fn import_nodes(
        document: &Document,
        buffer_data: &IndexMap<String, Data>,
        mesh_offsets: &HashMap<String, IndexSpan>,
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
                import_node(asset_root_nodes[0].clone(), buffer_data, mesh_offsets)?,
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
                    default_node.merge(import_node(asset_root_node, buffer_data, mesh_offsets)?);
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
    buffer_data: &IndexMap<String, Data>,
    mesh_offsets: &HashMap<String, IndexSpan>,
) -> Result<AiNodeTree, AiReadError> {
    let mut ai_node_tree = AiNodeTree::default();
    let mut node_queue: VecDeque<(gltf_v1::Node<'_>, Option<usize>)> = VecDeque::new();
    node_queue.push_back((root_node, None));
    while let Some((node, parent_index)) = node_queue.pop_front() {
        let index = ai_node_tree.arena.len() - 1;
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
