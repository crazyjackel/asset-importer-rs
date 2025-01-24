use std::collections::{HashMap, VecDeque};

use gltf::{buffer, Document, Node, Semantic};
use serde_json::Value;

use crate::{
    core::error::AiReadError,
    structs::{
        base_types::AiReal, AiBone, AiCamera, AiLight, AiMatrix4x4, AiMesh, AiMetadataEntry,
        AiNode, AiNodeTree, AiVertexWeight,
    },
};

use super::{
    gltf2_importer::Gltf2Importer,
    gltf2_importer_mesh::{remap_data, GetPointer},
};

impl Gltf2Importer {
    pub(crate) fn import_nodes<'a>(
        document: &Document,
        buffer_data: &'a [buffer::Data],
        meshes: &mut Vec<AiMesh>,
        mesh_offsets: &Vec<u32>,
        remap_table: &Vec<Vec<u32>>,
        lights: &mut Vec<AiLight>,
        cameras: &mut Vec<AiCamera>,
    ) -> Result<(AiNodeTree, String), AiReadError> {
        let mut default_scene = document.default_scene();
        if default_scene.is_none() {
            default_scene = document.scenes().next();
        }
        if default_scene.is_none() {
            return Ok((AiNodeTree::default(), "".to_string()));
        }

        let asset_root_nodes: Vec<gltf::Node<'_>> = default_scene.as_ref().unwrap().nodes().collect();
        return if asset_root_nodes.len() == 1 {
            Ok((
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
            ))
        } else if asset_root_nodes.len() > 1 {
            let mut ai_node = AiNode::default();
            ai_node.name = "ROOT".to_string();
            let mut default_node = AiNodeTree::default();
            default_node.root = Some(0);
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
        } else {
            let mut ai_node = AiNode::default();
            ai_node.name = "ROOT".to_string();
            let mut default_node = AiNodeTree::default();
            default_node.root = Some(0);
            default_node.arena.push(ai_node);
            Ok((
                default_node,
                default_scene.unwrap().name().unwrap_or("").to_string(),
            ))
        };
    }
}

fn import_node<'a>(
    root_node: gltf::Node<'a>,
    buffer_data: &'a [buffer::Data],
    meshes: &mut Vec<AiMesh>,
    mesh_offsets: &Vec<u32>,
    remap_table: &Vec<Vec<u32>>,
    lights: &mut Vec<AiLight>,
    cameras: &mut Vec<AiCamera>,
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
        ai_node.transformation = node.transform().matrix().into();

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
                    let data_matrices = x.get_pointer(buffer_data).ok()?;
                    let data = remap_data(None, data_matrices, 64, |chunk| AiMatrix4x4 {
                        a1: f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
                        a2: f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
                        a3: f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]),
                        a4: f32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]),
                        b1: f32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]),
                        b2: f32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]),
                        b3: f32::from_le_bytes([chunk[24], chunk[25], chunk[26], chunk[27]]),
                        b4: f32::from_le_bytes([chunk[28], chunk[29], chunk[30], chunk[31]]),
                        c1: f32::from_le_bytes([chunk[32], chunk[33], chunk[34], chunk[35]]),
                        c2: f32::from_le_bytes([chunk[36], chunk[37], chunk[38], chunk[39]]),
                        c3: f32::from_le_bytes([chunk[40], chunk[41], chunk[42], chunk[43]]),
                        c4: f32::from_le_bytes([chunk[44], chunk[45], chunk[46], chunk[47]]),
                        d1: f32::from_le_bytes([chunk[48], chunk[49], chunk[50], chunk[51]]),
                        d2: f32::from_le_bytes([chunk[52], chunk[53], chunk[54], chunk[55]]),
                        d3: f32::from_le_bytes([chunk[56], chunk[57], chunk[58], chunk[59]]),
                        d4: f32::from_le_bytes([chunk[60], chunk[61], chunk[62], chunk[63]]),
                    });
                    Some(data)
                });
                for primitive_no in 0..(start - end) {
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
                            let data_joint = acc_joint
                                .get_pointer(buffer_data)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            let joint_data = match acc_joint.data_type() {
                                gltf::accessor::DataType::U8 => {
                                    remap_data(remap_table, data_joint, 4, |chunk| {
                                        [
                                            chunk[0] as u32,
                                            chunk[1] as u32,
                                            chunk[2] as u32,
                                            chunk[3] as u32,
                                        ]
                                    })
                                }
                                gltf::accessor::DataType::U16 => {
                                    remap_data(remap_table, data_joint, 8, |chunk| {
                                        [
                                            u16::from_le_bytes([chunk[0], chunk[1]]) as u32,
                                            u16::from_le_bytes([chunk[2], chunk[3]]) as u32,
                                            u16::from_le_bytes([chunk[4], chunk[5]]) as u32,
                                            u16::from_le_bytes([chunk[6], chunk[7]]) as u32,
                                        ]
                                    })
                                }
                                _ => {
                                    continue;
                                }
                            };
                            let data_weight = acc_weight
                                .get_pointer(buffer_data)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            let weight_data = match acc_weight.data_type() {
                                gltf::accessor::DataType::U8 => {
                                    remap_data(remap_table, data_weight, 4, |chunk| {
                                        [
                                            chunk[0] as AiReal / 255.0,
                                            chunk[1] as AiReal / 255.0,
                                            chunk[2] as AiReal / 255.0,
                                            chunk[3] as AiReal / 255.0,
                                        ]
                                    })
                                }
                                gltf::accessor::DataType::U16 => {
                                    remap_data(remap_table, data_weight, 8, |chunk| {
                                        [
                                            u16::from_le_bytes([chunk[0], chunk[1]]) as AiReal
                                                / 65535.0,
                                            u16::from_le_bytes([chunk[2], chunk[3]]) as AiReal
                                                / 65535.0,
                                            u16::from_le_bytes([chunk[4], chunk[5]]) as AiReal
                                                / 65535.0,
                                            u16::from_le_bytes([chunk[6], chunk[7]]) as AiReal
                                                / 65535.0,
                                        ]
                                    })
                                }
                                gltf::accessor::DataType::F32 => {
                                    remap_data(remap_table, data_weight, 16, |chunk| {
                                        [
                                            f32::from_le_bytes([
                                                chunk[0], chunk[1], chunk[2], chunk[3],
                                            ])
                                                as AiReal,
                                            f32::from_le_bytes([
                                                chunk[4], chunk[5], chunk[6], chunk[7],
                                            ])
                                                as AiReal,
                                            f32::from_le_bytes([
                                                chunk[8], chunk[9], chunk[10], chunk[11],
                                            ])
                                                as AiReal,
                                            f32::from_le_bytes([
                                                chunk[12], chunk[13], chunk[14], chunk[15],
                                            ])
                                                as AiReal,
                                        ]
                                    })
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
                        ai_bone.offset_matrix = joint.transform().matrix().into();
                        if let Some(bind_matrix) = &bind_matrices {
                            ai_bone.offset_matrix = bind_matrix[i].clone();
                        }
                        if weights.len() > 0 {
                            ai_bone.weights = weights.to_vec();
                        } else {
                            ai_bone.weights = vec![AiVertexWeight::new(0, 0.0)]
                        }
                        ai_mesh.bones.push(ai_bone);
                    }
                }
            }
            for i in start..end {
                ai_node.mesh_indexes.push(i as usize);
            }
        }
        //handle cameras
        if let Some(camera) = node.camera() {
            if let Some(ai_camera) = cameras.get_mut(camera.index()) {
                ai_camera.name = ai_node.name.to_string();
            }
        }
        //handle lights
        if let Some(light) = node.light() {
            if let Some(ai_light) = lights.get_mut(light.index()) {
                ai_light.name = ai_node.name.to_string();
            }

            if let Some(range) = light.range() {
                let hashmap = ai_node.metadata.get_or_insert(HashMap::new());
                hashmap.insert("PBR_LightRange".to_string(), AiMetadataEntry::AiF32(range));
            }
        }

        ai_node.parent = parent_index;
        ai_node_tree.arena.push(ai_node);
        let index = ai_node_tree.arena.len() - 1;
        if parent_index.is_none() {
            ai_node_tree.root = Some(index);
        }
        for child in node.children().into_iter() {
            node_queue.push_back((child, Some(index)));
        }
    }
    Ok(ai_node_tree)
}

fn handle_extensions(ai_node: &mut AiNode, node: &gltf::Node<'_>) {
    fn parse_extension(value: &Value) -> Option<AiMetadataEntry> {
        match value {
            Value::Null => None,
            Value::Bool(b) => Some(AiMetadataEntry::AiBool(*b)),
            Value::Number(number) => {
                if let Some(num) = number.as_u64() {
                    Some(AiMetadataEntry::AiU64(num))
                } else {
                    if let Some(num2) = number.as_i64() {
                        Some(AiMetadataEntry::AiI64(num2))
                    } else if let Some(num3) = number.as_f64() {
                        Some(AiMetadataEntry::AiF64(num3))
                    } else {
                        None
                    }
                }
            }
            Value::String(str) => Some(AiMetadataEntry::AiStr(str.to_string())),
            Value::Array(_) => None,
            Value::Object(map) => {
                let mut meta_map: HashMap<String, AiMetadataEntry> = HashMap::new();
                for (str, val) in map {
                    if let Some(value) = parse_extension(&val) {
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
            if let Some(value) = parse_extension(&val) {
                metadata.insert(str.to_string(), value);
            }
        }
    }
}
