use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use gltf::{Document, Node, Semantic, buffer};
use serde_json::Value;

use asset_importer_rs_core::AiReadError;
use asset_importer_rs_scene::{
    AiBone, AiCamera, AiLight, AiMatrix4x4, AiMesh, AiMetadataEntry, AiNode, AiNodeTree, AiReal,
    AiVertexWeight,
};

use super::{gltf2_importer::Gltf2Importer, gltf2_importer_mesh::ExtractData};

impl Gltf2Importer {
    pub(crate) fn import_nodes(
        document: &Document,
        buffer_data: &[buffer::Data],
        meshes: &mut [AiMesh],
        mesh_offsets: &[u32],
        remap_table: &[Vec<usize>],
        lights: &mut [AiLight],
        cameras: &mut [AiCamera],
    ) -> Result<(AiNodeTree, String), AiReadError> {
        let mut default_scene = document.default_scene();
        if default_scene.is_none() {
            default_scene = document.scenes().next();
        }
        if default_scene.is_none() {
            return Ok((AiNodeTree::default(), "".to_string()));
        }

        let asset_root_nodes: Vec<gltf::Node<'_>> =
            default_scene.as_ref().unwrap().nodes().collect();

        match asset_root_nodes.len().cmp(&1) {
            Ordering::Equal => Ok((
                import_node(
                    asset_root_nodes[0].clone(),
                    buffer_data,
                    meshes,
                    mesh_offsets,
                    remap_table,
                    lights,
                    cameras,
                )?,
                default_scene.unwrap().name().unwrap_or("").to_string(),
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
                        buffer_data,
                        meshes,
                        mesh_offsets,
                        remap_table,
                        lights,
                        cameras,
                    )?);
                }
                Ok((
                    default_node,
                    default_scene.unwrap().name().unwrap_or("").to_string(),
                ))
            }
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
                    default_scene.unwrap().name().unwrap_or("").to_string(),
                ))
            }
        }
    }
}

fn import_node<'a>(
    root_node: gltf::Node<'a>,
    buffer_data: &'a [buffer::Data],
    meshes: &mut [AiMesh],
    mesh_offsets: &[u32],
    remap_table: &[Vec<usize>],
    lights: &mut [AiLight],
    cameras: &mut [AiCamera],
) -> Result<AiNodeTree, AiReadError> {
    let mut ai_node_tree = AiNodeTree::default();
    let mut node_queue: VecDeque<(gltf::Node<'_>, Option<usize>)> = VecDeque::new();
    node_queue.push_back((root_node, None));
    while let Some((node, parent_index)) = node_queue.pop_front() {
        let mut ai_node = AiNode::default();

        //handle extensions
        handle_extensions(&mut ai_node, &node);
        //handle extras

        //handle transform
        ai_node.transformation = node
            .transform()
            .matrix()
            .map(|x| x.map(|y| y as AiReal))
            .into();

        //handle meshes
        if let Some(mesh) = node.mesh() {
            let index = mesh.index();
            let start = mesh_offsets[index] as usize;
            let end = mesh_offsets[index + 1] as usize;
            let asset_primitives: Vec<gltf::Primitive<'_>> = mesh.primitives().collect();
            ai_node.mesh_indexes.reserve(end - start);
            if let Some(skin) = node.skin() {
                let asset_joints: Vec<Node<'_>> = skin.joints().collect();
                let num_bones = asset_joints.len();
                let bind_matrices = skin.inverse_bind_matrices().and_then(|x| {
                    let data_matrices: Vec<[f32; 16]> = x.extract_data(buffer_data, None).ok()?;
                    let data: Vec<AiMatrix4x4> = data_matrices
                        .iter()
                        .map(|x| AiMatrix4x4::from(x.map(|x| x as AiReal)))
                        .collect();
                    Some(data)
                });
                for primitive_no in 0..(end - start) {
                    let ai_mesh = &mut meshes[primitive_no + start];
                    let remap_table = remap_table.get(primitive_no + start);
                    let primitive = &asset_primitives[primitive_no];

                    //build vertex weights, length is number of bones and contains which vertices are moved by much much
                    let mut weighting: Vec<Vec<AiVertexWeight>> = Vec::new();
                    let attr_joints: Vec<(gltf::Accessor<'_>, u32)> = primitive
                        .attributes()
                        .filter_map(|x| match x.0 {
                            Semantic::Joints(n) => Some((x.1, n)),
                            _ => None,
                        })
                        .collect();
                    let attr_weights: Vec<(gltf::Accessor<'_>, u32)> = primitive
                        .attributes()
                        .filter_map(|x| match x.0 {
                            Semantic::Weights(n) => Some((x.1, n)),
                            _ => None,
                        })
                        .collect();
                    weighting.resize(num_bones, Vec::new());
                    for (acc_joint, index) in attr_joints {
                        if let Some((acc_weight, _)) = attr_weights.iter().find(|x| x.1 == index) {
                            let joint_data: Vec<[u32; 4]> = match acc_joint.data_type() {
                                gltf::accessor::DataType::U8 => {
                                    let data_joint: Vec<[u8; 4]> =
                                        acc_joint.extract_data(buffer_data, remap_table).map_err(
                                            |err| AiReadError::FileFormatError(Box::new(err)),
                                        )?;

                                    data_joint
                                        .iter()
                                        .map(|chunk| {
                                            [
                                                chunk[0] as u32,
                                                chunk[1] as u32,
                                                chunk[2] as u32,
                                                chunk[3] as u32,
                                            ]
                                        })
                                        .collect()
                                }
                                gltf::accessor::DataType::U16 => {
                                    let data_joint: Vec<[u16; 4]> =
                                        acc_joint.extract_data(buffer_data, remap_table).map_err(
                                            |err| AiReadError::FileFormatError(Box::new(err)),
                                        )?;

                                    data_joint
                                        .iter()
                                        .map(|chunk| {
                                            [
                                                chunk[0] as u32,
                                                chunk[1] as u32,
                                                chunk[2] as u32,
                                                chunk[3] as u32,
                                            ]
                                        })
                                        .collect()
                                }
                                _ => {
                                    continue;
                                }
                            };

                            let weight_data: Vec<[AiReal; 4]> = match acc_weight.data_type() {
                                gltf::accessor::DataType::U8 => {
                                    let data_weight: Vec<[u8; 4]> =
                                        acc_weight.extract_data(buffer_data, remap_table).map_err(
                                            |err| AiReadError::FileFormatError(Box::new(err)),
                                        )?;

                                    data_weight
                                        .iter()
                                        .map(|chunk| {
                                            [
                                                chunk[0] as AiReal / 255.0,
                                                chunk[1] as AiReal / 255.0,
                                                chunk[2] as AiReal / 255.0,
                                                chunk[3] as AiReal / 255.0,
                                            ]
                                        })
                                        .collect()
                                }
                                gltf::accessor::DataType::U16 => {
                                    let data_weight: Vec<[u8; 4]> =
                                        acc_weight.extract_data(buffer_data, remap_table).map_err(
                                            |err| AiReadError::FileFormatError(Box::new(err)),
                                        )?;

                                    data_weight
                                        .iter()
                                        .map(|chunk| {
                                            [
                                                chunk[0] as AiReal / 65535.0,
                                                chunk[1] as AiReal / 65535.0,
                                                chunk[2] as AiReal / 65535.0,
                                                chunk[3] as AiReal / 65535.0,
                                            ]
                                        })
                                        .collect()
                                }
                                gltf::accessor::DataType::F32 => {
                                    let data_weight: Vec<[f32; 4]> =
                                        acc_weight.extract_data(buffer_data, remap_table).map_err(
                                            |err| AiReadError::FileFormatError(Box::new(err)),
                                        )?;

                                    data_weight
                                        .iter()
                                        .map(|chunk| {
                                            [
                                                chunk[0] as AiReal,
                                                chunk[1] as AiReal,
                                                chunk[2] as AiReal,
                                                chunk[3] as AiReal,
                                            ]
                                        })
                                        .collect()
                                }
                                _ => {
                                    continue;
                                }
                            };
                            for i in 0..usize::min(joint_data.len(), weight_data.len()) {
                                for j in 0..4 {
                                    let joint = joint_data[i][j] as usize;
                                    let weight = weight_data[i][j] as AiReal;
                                    if weight > 0.0 && joint < num_bones {
                                        let vertex_weight: AiVertexWeight =
                                            AiVertexWeight::new(i, weight);
                                        weighting[joint].push(vertex_weight);
                                    }
                                }
                            }
                        }
                    }

                    for i in 0..num_bones {
                        let mut ai_bone = AiBone::default();
                        let weights = &weighting[i];
                        let joint = &asset_joints[i];

                        ai_bone.name = joint
                            .name()
                            .unwrap_or(format!("{}{}", "bone_", i).as_str())
                            .to_string();
                        ai_bone.offset_matrix = joint
                            .transform()
                            .matrix()
                            .map(|x| x.map(|y| y as AiReal))
                            .into();
                        if let Some(bind_matrix) = &bind_matrices {
                            ai_bone.offset_matrix = bind_matrix[i].clone();
                        }
                        if !weights.is_empty() {
                            ai_bone.weights = weights.to_vec();
                        } else {
                            ai_bone.weights = vec![AiVertexWeight::new(0, 0.0)]
                        }
                        ai_mesh.bones.push(ai_bone);
                    }
                }
            }
            for i in start..end {
                ai_node.mesh_indexes.push(i);
            }
        }
        //handle cameras
        if let Some(camera) = node.camera() {
            if let Some(ai_camera) = cameras.get_mut(camera.index()) {
                ai_camera.name = ai_node.name.to_string();
            }
        }
        //handle lights
        handle_lights(lights, &mut ai_node, &node);

        ai_node.parent = parent_index;
        ai_node_tree.arena.push(ai_node);
        let index = ai_node_tree.arena.len() - 1;
        if parent_index.is_none() {
            ai_node_tree.root = Some(index);
        }
        for child in node.children() {
            node_queue.push_back((child, Some(index)));
        }
    }
    Ok(ai_node_tree)
}

#[cfg(not(feature = "KHR_lights_punctual"))]
fn handle_lights(_lights: &mut [AiLight], _ai_node: &mut AiNode, _node: &Node<'_>) {}
#[cfg(feature = "KHR_lights_punctual")]
fn handle_lights(lights: &mut [AiLight], ai_node: &mut AiNode, node: &Node<'_>) {
    if let Some(light) = node.light() {
        if let Some(ai_light) = lights.get_mut(light.index()) {
            ai_light.name = ai_node.name.to_string();
        }

        if let Some(range) = light.range() {
            let hashmap = ai_node.metadata.get_or_insert(HashMap::new());
            hashmap.insert("PBR_LightRange".to_string(), AiMetadataEntry::AiF32(range));
        }
    }
}

#[cfg(not(feature = "extensions"))]
fn handle_extensions(ai_node: &mut AiNode, node: &gltf::Node<'_>) {}
#[cfg(feature = "extensions")]
fn handle_extensions(ai_node: &mut AiNode, node: &gltf::Node<'_>) {
    fn parse_extension(value: &Value) -> Option<AiMetadataEntry> {
        match value {
            Value::Null => None,
            Value::Bool(b) => Some(AiMetadataEntry::AiBool(*b)),
            Value::Number(number) => {
                if let Some(num) = number.as_u64() {
                    Some(AiMetadataEntry::AiU64(num))
                } else if let Some(num2) = number.as_i64() {
                    Some(AiMetadataEntry::AiI64(num2))
                } else {
                    number.as_f64().map(AiMetadataEntry::AiF64)
                }
            }
            Value::String(str) => Some(AiMetadataEntry::AiStr(str.to_string())),
            Value::Array(_) => None,
            Value::Object(map) => {
                let mut meta_map: HashMap<String, AiMetadataEntry> = HashMap::new();
                for (str, val) in map {
                    if let Some(value) = parse_extension(val) {
                        meta_map.insert(str.to_string(), value);
                    }
                }
                Some(AiMetadataEntry::AiMetadata(meta_map))
            }
        }
    }

    if let Some(ext) = node.extensions() {
        let metadata = ai_node.metadata.get_or_insert(HashMap::new());
        for (str, val) in ext {
            if let Some(value) = parse_extension(val) {
                metadata.insert(str.to_string(), value);
            }
        }
    }
}
