use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use gltf::{
    Semantic,
    accessor::{DataType, Dimensions},
    buffer::Target,
    json::{
        Accessor, Index, Mesh, Root, Skin,
        accessor::GenericComponentType,
        buffer::View,
        mesh::{MorphTarget, Primitive},
        validation::{Checked, USize64},
    },
    mesh::Mode,
};

use asset_importer_rs_core::AiExportError;
use asset_importer_rs_scene::{
    AiColor4D, AiMatrix4x4, AiPrimitiveType, AiQuaternion, AiReal, AiScene, AiVector2D, AiVector3D,
};
use serde_json::{Number, Value};

use super::gltf2_exporter::{Gltf2Exporter, generate_unique_name};

impl Gltf2Exporter {
    pub(crate) fn export_meshes(
        &self,
        scene: &AiScene,
        root: &mut Root,
        unique_names_map: &mut HashMap<String, u32>,
        buffer_data: &mut Vec<u8>,
        node_index_to_meshes: &HashMap<usize, Vec<usize>>,
        unlimited_bones_per_vertex: bool,
        //export_anim_sparse: bool,
        export_anim_normals: bool,
        //export_skeleton: bool,
    ) -> Result<(), AiExportError> {
        let create_skin = scene.meshes.iter().any(|x| !x.bones.is_empty());
        let mut inverse_bind_matrices_data: Vec<AiMatrix4x4> = Vec::new();
        let mut skin = if create_skin {
            Some(Skin {
                extensions: Default::default(),
                extras: Default::default(),
                inverse_bind_matrices: Default::default(),
                joints: Default::default(),
                name: Some(generate_unique_name("skin", unique_names_map)),
                skeleton: Default::default(),
            })
        } else {
            None
        };
        let mut meshes: Vec<Mesh> = Vec::new();
        for ai_mesh in &scene.meshes {
            let mut attributes: BTreeMap<Checked<Semantic>, Index<Accessor>> = BTreeMap::new();

            //handle positions
            let positions =
                AccessorExporter::export_vector_3d(root, buffer_data, &ai_mesh.vertices);
            // If Mesh has no positions, skip the mesh
            if let Some(positions) = positions {
                attributes.insert(Checked::Valid(Semantic::Positions), root.push(positions));
            } else {
                continue;
            }

            //handle normals
            let normals = AccessorExporter::export_vector_3d(
                root,
                buffer_data,
                &ai_mesh.normals.iter().map(|x| x.norm()).collect(),
            );
            if let Some(normals) = normals {
                attributes.insert(Checked::Valid(Semantic::Normals), root.push(normals));
            }

            //handle tangents
            let tangents = AccessorExporter::export_quaternion(
                root,
                buffer_data,
                &ai_mesh
                    .tangents
                    .iter()
                    .map(|x| x.norm().to_quat(1.0))
                    .collect(),
            );
            if let Some(tangents) = tangents {
                attributes.insert(Checked::Valid(Semantic::Tangents), root.push(tangents));
            }

            //handle texture coordinates
            for (index, uv) in ai_mesh.texture_coords.iter().enumerate() {
                if let Some(ai_uvs) = uv {
                    let is_2d = ai_uvs.iter().all(|x| x.z == 0.0);
                    let uvs = if is_2d {
                        AccessorExporter::export_vector_2d(
                            root,
                            buffer_data,
                            &ai_uvs
                                .iter()
                                .map(|x| AiVector2D::new(x.x, 1.0 - x.y))
                                .collect(),
                        )
                    } else {
                        AccessorExporter::export_vector_3d(
                            root,
                            buffer_data,
                            &ai_uvs
                                .iter()
                                .map(|x| AiVector3D::new(x.x, 1.0 - x.y, x.z))
                                .collect(),
                        )
                    };
                    if let Some(uvs) = uvs {
                        attributes.insert(
                            Checked::Valid(Semantic::TexCoords(index as u32)),
                            root.push(uvs),
                        );
                    }
                }
            }

            //handle colors
            for (index, color) in ai_mesh.colors.iter().enumerate() {
                if let Some(ai_color) = color {
                    let colors = AccessorExporter::export_color(root, buffer_data, ai_color);
                    if let Some(colors) = colors {
                        attributes.insert(
                            Checked::Valid(Semantic::Colors(index as u32)),
                            root.push(colors),
                        );
                    }
                }
            }

            //handle indices
            let indices = if !ai_mesh.faces.is_empty() {
                let mut indices: Vec<u32> = Vec::new();
                let n_indices_per_face: u32 = ai_mesh.faces[0].len() as u32;
                indices.resize(ai_mesh.faces.len() * n_indices_per_face as usize, 0);
                for i in 0..ai_mesh.faces.len() {
                    for j in 0..n_indices_per_face as usize {
                        indices[i * n_indices_per_face as usize + j] = ai_mesh.faces[i][j] as u32;
                    }
                }
                let accessor = AccessorExporter::export_u32(root, buffer_data, &indices);
                if let Some(accessor) = accessor {
                    Some(root.push(accessor))
                } else {
                    None
                }
            } else {
                None
            };

            //handle mode
            let mode = if ai_mesh.primitive_types.contains(AiPrimitiveType::Triangle) {
                Mode::Triangles
            } else if ai_mesh.primitive_types.contains(AiPrimitiveType::Line) {
                Mode::Lines
            } else if ai_mesh.primitive_types.contains(AiPrimitiveType::Point) {
                Mode::Points
            } else {
                Mode::Triangles
            };

            //handle skin
            if let Some(ref mut skin) = skin {
                if !ai_mesh.bones.is_empty() {
                    let num_verts = ai_mesh.vertices.len();
                    let mut all_vertices_pairs: Vec<Vec<(usize, AiReal)>> = Vec::new();
                    let mut joints_per_vertex: Vec<u32> = Vec::new();
                    let mut max_joint_per_vertex: u32 = 0;
                    joints_per_vertex.resize(num_verts, 0);
                    all_vertices_pairs.resize(num_verts, Vec::new());
                    for bone in &ai_mesh.bones {
                        //We do this Check to make sure nodes exist and to potentially get validate node names once GLTF-RS supports Node Names
                        if root.nodes.get(bone.node_index).is_some() {
                            //Get Joint Index
                            let joint_index_option = skin
                                .joints
                                .iter()
                                .enumerate()
                                .find(|(_, y)| y.value() == bone.node_index);
                            //we grab the Index Node for node names
                            let (joint_index, _) = if let Some(opt) = joint_index_option {
                                opt
                            } else {
                                assert_eq!(skin.joints.len(), inverse_bind_matrices_data.len());
                                let index = Index::new(bone.node_index as u32);
                                skin.joints.push(index);
                                inverse_bind_matrices_data.push(bone.offset_matrix.clone());
                                let last_index = skin.joints.len() - 1;
                                (last_index, &skin.joints[last_index])
                            };

                            //Populate Data Structures
                            for weight in &bone.weights {
                                all_vertices_pairs[weight.vertex_id]
                                    .push((joint_index, weight.weight));
                                joints_per_vertex[weight.vertex_id] += 1;
                                max_joint_per_vertex = u32::max(
                                    max_joint_per_vertex,
                                    joints_per_vertex[weight.vertex_id],
                                );
                            }
                        }
                    }

                    if unlimited_bones_per_vertex {
                        max_joint_per_vertex = 4;
                    }

                    //Flatten and Sort Vertex Pairs into Vectors of Joint Indexes and Weight Indexes
                    let num_groups = ((max_joint_per_vertex - 1) / 4) + 1;
                    let mut vertex_joint_data: Vec<[usize; 4]> = Vec::new();
                    let mut vertex_weight_data: Vec<[AiReal; 4]> = Vec::new();
                    vertex_joint_data.resize(num_verts * num_groups as usize, Default::default());
                    vertex_weight_data.resize(num_verts * num_groups as usize, Default::default());
                    for (vertex_index, vertice_pair) in all_vertices_pairs.iter_mut().enumerate() {
                        vertice_pair.sort_by(|x, y| {
                            if x.1 == y.1 {
                                Ordering::Equal
                            } else if x.1 > y.1 {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        });
                        for group_index in 0..num_groups as usize {
                            for joint_index in 0..4_usize {
                                let index_bone = group_index * 4 + joint_index;
                                let index_data = vertex_index + num_verts * group_index;
                                if index_bone >= vertice_pair.len() {
                                    vertex_joint_data[index_data][joint_index] = 0;
                                    vertex_weight_data[index_data][joint_index] = 0.0;
                                } else {
                                    vertex_joint_data[index_data][joint_index] =
                                        vertice_pair[index_bone].0;
                                    vertex_weight_data[index_data][joint_index] =
                                        vertice_pair[index_bone].1;
                                }
                            }
                        }
                    }

                    //for each group export
                    for group_index in 0..num_groups as usize {
                        let start_index = group_index * num_verts;
                        let end_index = start_index + num_verts;
                        let slice = &vertex_joint_data[start_index..end_index];
                        let joints = AccessorExporter::export_usize_4(root, buffer_data, slice);
                        if let Some(joints) = joints {
                            attributes.insert(
                                Checked::Valid(Semantic::Joints(group_index as u32)),
                                root.push(joints),
                            );
                        }

                        let slice = &vertex_weight_data[start_index..end_index];
                        let joints = AccessorExporter::export_real_4(root, buffer_data, slice);
                        if let Some(joints) = joints {
                            attributes.insert(
                                Checked::Valid(Semantic::Weights(group_index as u32)),
                                root.push(joints),
                            );
                        }
                    }
                }
            }

            //handle targets
            let mut targets: Option<Vec<MorphTarget>> = None;
            let mut weights: Option<Vec<f32>> = None;
            if !ai_mesh.anim_meshes.is_empty() {
                let mut targets_vec: Vec<MorphTarget> =
                    Vec::with_capacity(ai_mesh.anim_meshes.len());
                let mut weights_vec: Vec<f32> = Vec::with_capacity(ai_mesh.anim_meshes.len());
                for animation in &ai_mesh.anim_meshes {
                    //@todo: handle sparse exports for anim mesh

                    //handle positions
                    let positon_diff = animation
                        .vertices
                        .iter()
                        .zip(&ai_mesh.vertices)
                        .map(|(x, y)| *x - *y)
                        .collect();
                    let positions =
                        AccessorExporter::export_vector_3d(root, buffer_data, &positon_diff);
                    let positions = if let Some(positions) = positions {
                        Some(root.push(positions))
                    } else {
                        continue;
                    };

                    //handle normals
                    let normals = if export_anim_normals {
                        let normals_diff = animation
                            .normals
                            .iter()
                            .zip(&ai_mesh.normals)
                            .map(|(x, y)| *x - *y)
                            .collect();
                        let normals =
                            AccessorExporter::export_vector_3d(root, buffer_data, &normals_diff);
                        if let Some(normals) = normals {
                            Some(root.push(normals))
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    let morph_target = MorphTarget {
                        positions,
                        normals,
                        tangents: None,
                    };
                    targets_vec.push(morph_target);
                    weights_vec.push(animation.weight);
                }
                targets = Some(targets_vec);
                weights = Some(weights_vec);
            }

            let primitive = Primitive {
                attributes,
                indices,
                material: Some(Index::new(ai_mesh.material_index)),
                mode: Checked::Valid(mode),
                targets,
                extensions: Default::default(),
                extras: Default::default(),
            };

            let name = generate_unique_name(&ai_mesh.name, unique_names_map);
            let mesh = Mesh {
                name: Some(name),
                primitives: vec![primitive],
                weights,
                extensions: Default::default(),
                extras: Default::default(),
            };

            meshes.push(mesh);
        }

        //finish skin export
        if skin.is_some() {
            let mut skin_ref = skin.unwrap();
            //export inverse_bind_matrices
            let inverse_mat_data =
                AccessorExporter::export_mat4(root, buffer_data, inverse_bind_matrices_data);
            if let Some(inverse_mat_data) = inverse_mat_data {
                skin_ref.inverse_bind_matrices = Some(root.push(inverse_mat_data));
            }

            // Find nodes that contain a mesh with bones and add "skeletons" and "skin" attributes to those nodes.
            // @todo: add skeleton support
            let skin = root.push(skin_ref);
            for node_index in 0..root.nodes.len() {
                if let Some(node) = root.nodes.get_mut(node_index) {
                    if let Some(node_meshes) = node_index_to_meshes.get(&node_index) {
                        for mesh_index in node_meshes {
                            if let Some(mesh) = meshes.get(*mesh_index) {
                                if mesh.weights.is_some() {
                                    node.skin = Some(skin);
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut merged_meshes: Vec<Mesh> = Vec::new();
        for node_index in 0..root.nodes.len() {
            if let Some(node) = root.nodes.get_mut(node_index) {
                if let Some(node_meshes) = node_index_to_meshes.get(&node_index) {
                    let mut first_mesh: Option<Mesh> = None;
                    for mesh_index in node_meshes {
                        if let Some(mesh) = meshes.get_mut(*mesh_index) {
                            if let Some(first) = first_mesh.as_mut() {
                                first.primitives.append(&mut mesh.primitives);
                            } else {
                                first_mesh = Some(mesh.clone());
                            }
                        }
                    }

                    if let Some(first) = first_mesh {
                        merged_meshes.push(first);
                        node.mesh = Some(Index::new((merged_meshes.len() - 1) as u32));
                    }
                }
            }
        }

        for mesh in merged_meshes {
            root.push(mesh);
        }
        //handle merging meshes before pushing to root
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct AccessorExporter;

impl AccessorExporter {
    pub(crate) fn export_mat4(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: Vec<AiMatrix4x4>,
    ) -> Option<Accessor> {
        let mut min = if vector_data.is_empty() {
            [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]
        } else {
            [
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
                f32::MAX,
            ]
        };
        let mut max = if vector_data.is_empty() {
            [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]
        } else {
            [
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
                f32::MIN,
            ]
        };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 4 * 16);
        for vector_base in vector_data {
            let matrix: [AiReal; 16] = vector_base.into();
            for (i, a) in matrix.iter().enumerate() {
                let b = *a as f32;
                if b < min[i] {
                    min[i] = b;
                }
                if b > max[i] {
                    max[i] = b;
                }
            }

            let vec: Vec<u8> = matrix.iter().flat_map(|x| x.to_le_bytes()).collect();
            data.extend_from_slice(&vec);
        }
        let min = Some(Value::Array(vec![
            Value::Number(Number::from_f64(min[0] as f64).unwrap()),
            Value::Number(Number::from_f64(min[1] as f64).unwrap()),
            Value::Number(Number::from_f64(min[2] as f64).unwrap()),
            Value::Number(Number::from_f64(min[3] as f64).unwrap()),
            Value::Number(Number::from_f64(min[4] as f64).unwrap()),
            Value::Number(Number::from_f64(min[5] as f64).unwrap()),
            Value::Number(Number::from_f64(min[6] as f64).unwrap()),
            Value::Number(Number::from_f64(min[7] as f64).unwrap()),
            Value::Number(Number::from_f64(min[8] as f64).unwrap()),
            Value::Number(Number::from_f64(min[9] as f64).unwrap()),
            Value::Number(Number::from_f64(min[10] as f64).unwrap()),
            Value::Number(Number::from_f64(min[11] as f64).unwrap()),
            Value::Number(Number::from_f64(min[12] as f64).unwrap()),
            Value::Number(Number::from_f64(min[13] as f64).unwrap()),
            Value::Number(Number::from_f64(min[14] as f64).unwrap()),
            Value::Number(Number::from_f64(min[15] as f64).unwrap()),
        ]));
        let max = Some(Value::Array(vec![
            Value::Number(Number::from_f64(max[0] as f64).unwrap()),
            Value::Number(Number::from_f64(max[1] as f64).unwrap()),
            Value::Number(Number::from_f64(max[2] as f64).unwrap()),
            Value::Number(Number::from_f64(max[3] as f64).unwrap()),
            Value::Number(Number::from_f64(max[4] as f64).unwrap()),
            Value::Number(Number::from_f64(max[5] as f64).unwrap()),
            Value::Number(Number::from_f64(max[6] as f64).unwrap()),
            Value::Number(Number::from_f64(max[7] as f64).unwrap()),
            Value::Number(Number::from_f64(max[8] as f64).unwrap()),
            Value::Number(Number::from_f64(max[9] as f64).unwrap()),
            Value::Number(Number::from_f64(max[10] as f64).unwrap()),
            Value::Number(Number::from_f64(max[11] as f64).unwrap()),
            Value::Number(Number::from_f64(max[12] as f64).unwrap()),
            Value::Number(Number::from_f64(max[13] as f64).unwrap()),
            Value::Number(Number::from_f64(max[14] as f64).unwrap()),
            Value::Number(Number::from_f64(max[15] as f64).unwrap()),
        ]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::F32,
            Target::ArrayBuffer,
            Dimensions::Mat4,
            min,
            max,
        )
    }

    pub(crate) fn export_real_4(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &[[AiReal; 4]],
    ) -> Option<Accessor> {
        if vector_data.is_empty() {
            return None;
        }
        let mut min_x = if vector_data.is_empty() {
            0.0
        } else {
            f32::MAX
        };
        let mut max_x = if vector_data.is_empty() {
            0.0
        } else {
            f32::MIN
        };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 4 * 4);
        for vector_base in vector_data {
            for vector in vector_base {
                let a = *vector as f32;
                if a < min_x {
                    min_x = a;
                }
                if a > max_x {
                    max_x = a;
                }

                data.extend_from_slice(&a.to_le_bytes());
            }
        }
        let min = Some(Value::Array(vec![Value::Number(
            Number::from_f64(min_x as f64).unwrap(),
        )]));
        let max = Some(Value::Array(vec![Value::Number(
            Number::from_f64(max_x as f64).unwrap(),
        )]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::F32,
            Target::ElementArrayBuffer,
            Dimensions::Vec4,
            min,
            max,
        )
    }

    pub(crate) fn export_usize_4(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &[[usize; 4]],
    ) -> Option<Accessor> {
        if vector_data.is_empty() {
            return None;
        }
        let mut min_x = if vector_data.is_empty() { 0 } else { u32::MAX } as usize;
        let mut max_x = if vector_data.is_empty() { 0 } else { u32::MIN } as usize;
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 4 * 4);
        for vector_base in vector_data {
            for vector in vector_base {
                if vector < &min_x {
                    min_x = *vector;
                }
                if vector > &max_x {
                    max_x = *vector;
                }

                let new_data = *vector as u32;
                data.extend_from_slice(&new_data.to_le_bytes());
            }
        }
        let min = Some(Value::Array(vec![Value::Number(Number::from(min_x))]));
        let max = Some(Value::Array(vec![Value::Number(Number::from(max_x))]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::U32,
            Target::ElementArrayBuffer,
            Dimensions::Vec4,
            min,
            max,
        )
    }

    pub(crate) fn export_u32(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &Vec<u32>,
    ) -> Option<Accessor> {
        if vector_data.is_empty() {
            return None;
        }
        let mut min_x = if vector_data.is_empty() { 0 } else { u32::MAX };
        let mut max_x = if vector_data.is_empty() { 0 } else { u32::MIN };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 4);
        for vector in vector_data {
            if vector < &min_x {
                min_x = *vector;
            }
            if vector > &max_x {
                max_x = *vector;
            }

            data.extend_from_slice(&vector.to_le_bytes());
        }
        let min = Some(Value::Array(vec![Value::Number(Number::from(min_x))]));
        let max = Some(Value::Array(vec![Value::Number(Number::from(max_x))]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::U32,
            Target::ElementArrayBuffer,
            Dimensions::Scalar,
            min,
            max,
        )
    }

    pub(crate) fn export_real(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &Vec<f32>,
    ) -> Option<Accessor> {
        if vector_data.is_empty() {
            return None;
        }
        let mut min_x = if vector_data.is_empty() {
            0.0
        } else {
            f32::MAX
        };
        let mut max_x = if vector_data.is_empty() {
            0.0
        } else {
            f32::MIN
        };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 4);
        for vector in vector_data {
            if vector < &min_x {
                min_x = *vector;
            }
            if vector > &max_x {
                max_x = *vector;
            }

            data.extend_from_slice(&vector.to_le_bytes());
        }
        let min = Some(Value::Array(vec![Value::Number(
            Number::from_f64(min_x as f64).unwrap(),
        )]));
        let max = Some(Value::Array(vec![Value::Number(
            Number::from_f64(max_x as f64).unwrap(),
        )]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::F32,
            Target::ArrayBuffer,
            Dimensions::Scalar,
            min,
            max,
        )
    }

    pub(crate) fn export_vector_2d(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &Vec<AiVector2D>,
    ) -> Option<Accessor> {
        if vector_data.is_empty() {
            return None;
        }
        let (mut min_x, mut min_y) = if vector_data.is_empty() {
            (0.0, 0.0)
        } else {
            (AiReal::MAX, AiReal::MAX)
        };
        let (mut max_x, mut max_y) = if vector_data.is_empty() {
            (0.0, 0.0)
        } else {
            (AiReal::MIN, AiReal::MIN)
        };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 2 * 4);
        for vector in vector_data {
            if vector.x < min_x {
                min_x = vector.x;
            }
            if vector.x > max_x {
                max_x = vector.x;
            }
            if vector.y < min_y {
                min_y = vector.y;
            }
            if vector.y > max_y {
                max_y = vector.y;
            }

            data.extend_from_slice(&vector.x.to_le_bytes());
            data.extend_from_slice(&vector.y.to_le_bytes());
        }
        let min = Some(Value::Array(vec![
            Value::Number(Number::from_f64(min_x as f64).unwrap()),
            Value::Number(Number::from_f64(min_y as f64).unwrap()),
        ]));
        let max = Some(Value::Array(vec![
            Value::Number(Number::from_f64(max_x as f64).unwrap()),
            Value::Number(Number::from_f64(max_y as f64).unwrap()),
        ]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::F32,
            Target::ArrayBuffer,
            Dimensions::Vec2,
            min,
            max,
        )
    }

    pub(crate) fn export_vector_3d(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &Vec<AiVector3D>,
    ) -> Option<Accessor> {
        let (mut min_x, mut min_y, mut min_z) = if vector_data.is_empty() {
            (0.0, 0.0, 0.0)
        } else {
            (AiReal::MAX, AiReal::MAX, AiReal::MAX)
        };
        let (mut max_x, mut max_y, mut max_z) = if vector_data.is_empty() {
            (0.0, 0.0, 0.0)
        } else {
            (AiReal::MIN, AiReal::MIN, AiReal::MIN)
        };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 3 * 4);
        for vector in vector_data {
            if vector.x < min_x {
                min_x = vector.x;
            }
            if vector.x > max_x {
                max_x = vector.x;
            }
            if vector.y < min_y {
                min_y = vector.y;
            }
            if vector.y > max_y {
                max_y = vector.y;
            }
            if vector.z < min_z {
                min_z = vector.z;
            }
            if vector.z > max_z {
                max_z = vector.z;
            }

            data.extend_from_slice(&vector.x.to_le_bytes());
            data.extend_from_slice(&vector.y.to_le_bytes());
            data.extend_from_slice(&vector.z.to_le_bytes());
        }
        let min = Some(Value::Array(vec![
            Value::Number(Number::from_f64(min_x as f64).unwrap()),
            Value::Number(Number::from_f64(min_y as f64).unwrap()),
            Value::Number(Number::from_f64(min_z as f64).unwrap()),
        ]));
        let max = Some(Value::Array(vec![
            Value::Number(Number::from_f64(max_x as f64).unwrap()),
            Value::Number(Number::from_f64(max_y as f64).unwrap()),
            Value::Number(Number::from_f64(max_z as f64).unwrap()),
        ]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::F32,
            Target::ArrayBuffer,
            Dimensions::Vec3,
            min,
            max,
        )
    }
    pub(crate) fn export_quaternion(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &Vec<AiQuaternion>,
    ) -> Option<Accessor> {
        if vector_data.is_empty() {
            return None;
        }
        let (mut min_x, mut min_y, mut min_z, mut min_w) = if vector_data.is_empty() {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            (AiReal::MAX, AiReal::MAX, AiReal::MAX, AiReal::MAX)
        };
        let (mut max_x, mut max_y, mut max_z, mut max_w) = if vector_data.is_empty() {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            (AiReal::MIN, AiReal::MIN, AiReal::MIN, AiReal::MIN)
        };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 3 * 4);
        for vector in vector_data {
            if vector.x < min_x {
                min_x = vector.x;
            }
            if vector.x > max_x {
                max_x = vector.x;
            }
            if vector.y < min_y {
                min_y = vector.y;
            }
            if vector.y > max_y {
                max_y = vector.y;
            }
            if vector.z < min_z {
                min_z = vector.z;
            }
            if vector.z > max_z {
                max_z = vector.z;
            }
            if vector.w < min_w {
                min_w = vector.w;
            }
            if vector.w > max_w {
                max_w = vector.w;
            }

            data.extend_from_slice(&vector.x.to_le_bytes());
            data.extend_from_slice(&vector.y.to_le_bytes());
            data.extend_from_slice(&vector.z.to_le_bytes());
            data.extend_from_slice(&vector.w.to_le_bytes());
        }
        let min = Some(Value::Array(vec![
            Value::Number(Number::from_f64(min_x as f64).unwrap()),
            Value::Number(Number::from_f64(min_y as f64).unwrap()),
            Value::Number(Number::from_f64(min_z as f64).unwrap()),
            Value::Number(Number::from_f64(min_w as f64).unwrap()),
        ]));
        let max = Some(Value::Array(vec![
            Value::Number(Number::from_f64(max_x as f64).unwrap()),
            Value::Number(Number::from_f64(max_y as f64).unwrap()),
            Value::Number(Number::from_f64(max_z as f64).unwrap()),
            Value::Number(Number::from_f64(max_w as f64).unwrap()),
        ]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::F32,
            Target::ArrayBuffer,
            Dimensions::Vec4,
            min,
            max,
        )
    }

    pub(crate) fn export_color(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        vector_data: &Vec<AiColor4D>,
    ) -> Option<Accessor> {
        if vector_data.is_empty() {
            return None;
        }
        let (mut min_x, mut min_y, mut min_z, mut min_w) = if vector_data.is_empty() {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            (f32::MAX, f32::MAX, f32::MAX, f32::MAX)
        };
        let (mut max_x, mut max_y, mut max_z, mut max_w) = if vector_data.is_empty() {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            (f32::MIN, f32::MIN, f32::MIN, f32::MAX)
        };
        let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 4 * 4);
        for vector in vector_data {
            if vector.r < min_x {
                min_x = vector.r;
            }
            if vector.r > max_x {
                max_x = vector.r;
            }
            if vector.g < min_y {
                min_y = vector.g;
            }
            if vector.g > max_y {
                max_y = vector.g;
            }
            if vector.b < min_z {
                min_z = vector.b;
            }
            if vector.b > max_z {
                max_z = vector.b;
            }
            if vector.a < min_w {
                min_w = vector.a;
            }
            if vector.a > max_w {
                max_w = vector.a;
            }

            data.extend_from_slice(&vector.r.to_le_bytes());
            data.extend_from_slice(&vector.g.to_le_bytes());
            data.extend_from_slice(&vector.b.to_le_bytes());
            data.extend_from_slice(&vector.a.to_le_bytes());
        }
        let min = Some(Value::Array(vec![
            Value::Number(Number::from_f64(min_x as f64).unwrap()),
            Value::Number(Number::from_f64(min_y as f64).unwrap()),
            Value::Number(Number::from_f64(min_z as f64).unwrap()),
            Value::Number(Number::from_f64(min_w as f64).unwrap()),
        ]));
        let max = Some(Value::Array(vec![
            Value::Number(Number::from_f64(max_x as f64).unwrap()),
            Value::Number(Number::from_f64(max_y as f64).unwrap()),
            Value::Number(Number::from_f64(max_z as f64).unwrap()),
            Value::Number(Number::from_f64(max_w as f64).unwrap()),
        ]));
        Self::export_data(
            root,
            buffer_data,
            data,
            DataType::F32,
            Target::ArrayBuffer,
            Dimensions::Vec4,
            min,
            max,
        )
    }

    pub(crate) fn export_data(
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        mut data: Vec<u8>,
        component_type: DataType,
        target: Target,
        acc_type: Dimensions,
        min: Option<Value>,
        max: Option<Value>,
    ) -> Option<Accessor> {
        if data.is_empty() {
            return None;
        }
        let bytes_per_component = component_type.size();
        let dimension_size = acc_type.multiplicity();
        let mut buffer_offset = buffer_data.len();
        let padding = buffer_offset % bytes_per_component;
        buffer_offset += padding;

        let length = data.len();
        let count = (length / dimension_size) / bytes_per_component;
        buffer_data.resize(buffer_data.len() + padding, 0);
        buffer_data.append(&mut data);
        let buffer_view = View {
            buffer: Index::new(0), //Body Buffer will be 0.bin
            byte_length: USize64(length as u64),
            byte_offset: Some(USize64(buffer_offset as u64)),
            byte_stride: None,
            name: None,
            target: Some(Checked::Valid(target)),
            extensions: Default::default(),
            extras: Default::default(),
        };

        Some(Accessor {
            buffer_view: Some(root.push(buffer_view)),
            byte_offset: Some(USize64(0)),
            component_type: Checked::Valid(GenericComponentType(component_type)),
            type_: Checked::Valid(acc_type),
            count: USize64(count as u64),
            min,
            max,
            name: None,
            normalized: Default::default(),
            sparse: None,
            extensions: Default::default(),
            extras: Default::default(),
        })
    }
}
