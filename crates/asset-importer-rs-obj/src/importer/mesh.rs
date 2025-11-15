use asset_importer_rs_scene::{AiColor4D, AiMesh, AiNode, AiPrimitiveType, AiReal, AiVector3D};
use tobj::Model;

use crate::importer::ObjImporter;

#[derive(Debug)]
pub struct ImportMeshes(pub Vec<AiMesh>, pub Vec<AiNode>);

impl ObjImporter {
    pub(crate) fn import_meshes(models: Vec<Model>) -> ImportMeshes {
        //Create Models and Node in Scene
        let mut ai_meshes: Vec<AiMesh> = Vec::with_capacity(models.capacity());
        let mut ai_nodes: Vec<AiNode> = Vec::with_capacity(models.capacity());
        for model in &models {
            let mut node = AiNode {
                name: model.name.clone(),
                ..AiNode::default()
            };
            let mesh = &model.mesh;

            let mut ai_mesh = AiMesh {
                name: model.name.clone(),
                material_index: mesh.material_id.unwrap_or(0) as u32,
                ..AiMesh::default()
            };

            //Handle Primitive Types
            if mesh.face_arities.is_empty() {
                ai_mesh.primitive_types |= AiPrimitiveType::Triangle;
            } else {
                for arity in &mesh.face_arities {
                    match arity {
                        1 => ai_mesh.primitive_types |= AiPrimitiveType::Point,
                        2 => ai_mesh.primitive_types |= AiPrimitiveType::Line,
                        3 => ai_mesh.primitive_types |= AiPrimitiveType::Triangle,
                        4.. => ai_mesh.primitive_types |= AiPrimitiveType::Polygon,
                        _ => {}
                    }
                }
            }

            //Handle Vertices
            let chunk_length = mesh.positions.len() / 3;
            ai_mesh.vertices = Vec::with_capacity(chunk_length);
            for i in 0..chunk_length {
                let offset = 3 * i;
                ai_mesh.vertices.push(AiVector3D::new(
                    mesh.positions[offset] as AiReal,
                    mesh.positions[offset + 1] as AiReal,
                    mesh.positions[offset + 2] as AiReal,
                ));
            }

            //Handle Normals
            let chunk_length = mesh.normals.len() / 3;
            ai_mesh.normals = Vec::with_capacity(chunk_length);
            for i in 0..chunk_length {
                let offset = 3 * i;
                ai_mesh.normals.push(AiVector3D::new(
                    mesh.normals[offset] as AiReal,
                    mesh.normals[offset + 1] as AiReal,
                    mesh.normals[offset + 2] as AiReal,
                ));
            }

            //Handle Vertex Colors
            if !mesh.vertex_color.is_empty() {
                let chunk_length = mesh.vertex_color.len() / 3;
                let mut color_channel_one: Vec<AiColor4D> = Vec::with_capacity(chunk_length);
                for i in 0..chunk_length {
                    let offset = 3 * i;
                    color_channel_one.push(AiColor4D::new(
                        mesh.vertex_color[offset],
                        mesh.vertex_color[offset + 1],
                        mesh.vertex_color[offset + 2],
                        1.0,
                    ));
                }
                ai_mesh.colors[0] = Some(color_channel_one);
            }

            //Handle Texture Coordinates
            if !mesh.texcoords.is_empty() {
                let textures = mesh.texcoords.len() / 2;
                let mut texture_channel_one: Vec<AiVector3D> = Vec::with_capacity(textures);
                for i in 0..textures {
                    let offset = 2 * i;
                    texture_channel_one.push(AiVector3D::new(
                        mesh.texcoords[offset] as AiReal,
                        mesh.texcoords[offset + 1] as AiReal,
                        0.0 as AiReal,
                    ));
                }
                ai_mesh.texture_coords[0] = Some(texture_channel_one);
            }

            //Handle Faces
            ai_mesh.faces = if mesh.face_arities.is_empty() {
                let faces_count = mesh.indices.len() / 3;
                let mut faces: Vec<Vec<usize>> = Vec::with_capacity(faces_count);
                for i in 0..faces_count {
                    let offset = 3 * i;
                    let face = vec![
                        mesh.indices[offset] as usize,
                        mesh.indices[offset + 1] as usize,
                        mesh.indices[offset + 2] as usize,
                    ];
                    faces.push(face);
                }
                faces
            } else {
                let mut faces: Vec<Vec<usize>> = Vec::with_capacity(mesh.face_arities.len());
                let mut offset = 0;
                for i in 0..mesh.face_arities.len() {
                    let face_arity = mesh.face_arities[i];
                    let mut face = Vec::with_capacity(face_arity as usize);
                    for j in 0..face_arity {
                        face.push(mesh.indices[offset + j as usize] as usize);
                    }
                    offset += face_arity as usize;
                    faces.push(face);
                }
                faces
            };

            node.mesh_indexes.push(ai_meshes.len());
            ai_nodes.push(node);
            ai_meshes.push(ai_mesh);
        }
        ImportMeshes(ai_meshes, ai_nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_mesh() -> tobj::Mesh {
        tobj::Mesh {
            positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], // 3 vertices
            normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],   // 3 normals
            texcoords: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0],                // 3 texture coords
            vertex_color: vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], // 3 colors
            indices: vec![0, 1, 2],                                       // 1 triangle
            face_arities: vec![],                                         // empty (triangulated)
            material_id: Some(1),                                         // material id
            texcoord_indices: vec![],
            normal_indices: vec![],
        }
    }

    fn create_test_model(name: &str) -> tobj::Model {
        tobj::Model {
            mesh: create_test_mesh(),
            name: name.to_string(),
        }
    }

    #[test]
    fn test_import_meshes_empty() {
        let models = vec![];
        let ImportMeshes(ai_meshes, ai_nodes) = ObjImporter::import_meshes(models);

        assert_eq!(ai_meshes.len(), 0);
        assert_eq!(ai_nodes.len(), 0);
    }

    #[test]
    fn test_import_meshes_single_triangle() {
        let models = vec![create_test_model("test_mesh")];
        let ImportMeshes(ai_meshes, ai_nodes) = ObjImporter::import_meshes(models);

        assert_eq!(ai_meshes.len(), 1);
        assert_eq!(ai_nodes.len(), 1);

        let mesh = &ai_meshes[0];
        let node = &ai_nodes[0];

        // Test mesh properties
        assert_eq!(mesh.name, "test_mesh");
        assert_eq!(mesh.material_index, 1);
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle));

        // Test vertices
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.vertices[0], AiVector3D::new(0.0, 0.0, 0.0));
        assert_eq!(mesh.vertices[1], AiVector3D::new(1.0, 0.0, 0.0));
        assert_eq!(mesh.vertices[2], AiVector3D::new(0.0, 1.0, 0.0));

        // Test normals
        assert_eq!(mesh.normals.len(), 3);
        assert_eq!(mesh.normals[0], AiVector3D::new(0.0, 0.0, 1.0));
        assert_eq!(mesh.normals[1], AiVector3D::new(0.0, 0.0, 1.0));
        assert_eq!(mesh.normals[2], AiVector3D::new(0.0, 0.0, 1.0));

        // Test texture coordinates
        assert!(mesh.texture_coords[0].is_some());
        let texcoords = mesh.texture_coords[0].as_ref().unwrap();
        assert_eq!(texcoords.len(), 3);
        assert_eq!(texcoords[0], AiVector3D::new(0.0, 0.0, 0.0));
        assert_eq!(texcoords[1], AiVector3D::new(1.0, 0.0, 0.0));
        assert_eq!(texcoords[2], AiVector3D::new(0.0, 1.0, 0.0));

        // Test vertex colors
        assert!(mesh.colors[0].is_some());
        let colors = mesh.colors[0].as_ref().unwrap();
        assert_eq!(colors.len(), 3);
        assert_eq!(colors[0], AiColor4D::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(colors[1], AiColor4D::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(colors[2], AiColor4D::new(0.0, 0.0, 1.0, 1.0));

        // Test faces
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0], vec![0, 1, 2]);

        // Test node
        assert_eq!(node.name, "test_mesh");
        assert_eq!(node.mesh_indexes, vec![0]);
    }

    #[test]
    fn test_import_meshes_multiple_models() {
        let models = vec![create_test_model("mesh1"), create_test_model("mesh2")];
        let ImportMeshes(ai_meshes, ai_nodes) = ObjImporter::import_meshes(models);

        assert_eq!(ai_meshes.len(), 2);
        assert_eq!(ai_nodes.len(), 2);

        assert_eq!(ai_meshes[0].name, "mesh1");
        assert_eq!(ai_meshes[1].name, "mesh2");
        assert_eq!(ai_nodes[0].name, "mesh1");
        assert_eq!(ai_nodes[1].name, "mesh2");
        assert_eq!(ai_nodes[0].mesh_indexes, vec![0]);
        assert_eq!(ai_nodes[1].mesh_indexes, vec![1]);
    }

    #[test]
    fn test_import_meshes_with_face_arities() {
        let mut mesh = create_test_mesh();
        mesh.face_arities = vec![3, 4]; // 1 triangle, 1 quad
        mesh.indices = vec![0, 1, 2, 3, 4, 5, 6]; // 7 indices total

        let model = tobj::Model {
            mesh,
            name: "test_mesh".to_string(),
        };

        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        let mesh = &ai_meshes[0];

        // Should have both Triangle and Polygon primitive types
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle));
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Polygon));

        // Should have 2 faces
        assert_eq!(mesh.faces.len(), 2);
        assert_eq!(mesh.faces[0], vec![0, 1, 2]); // triangle
        assert_eq!(mesh.faces[1], vec![3, 4, 5, 6]); // quad
    }

    #[test]
    fn test_import_meshes_primitive_types() {
        // Test points
        let mut mesh = create_test_mesh();
        mesh.face_arities = vec![1];
        mesh.indices = vec![0];
        let model = tobj::Model {
            mesh,
            name: "points".to_string(),
        };
        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        assert!(
            ai_meshes[0]
                .primitive_types
                .contains(AiPrimitiveType::Point)
        );

        // Test lines
        let mut mesh = create_test_mesh();
        mesh.face_arities = vec![2];
        mesh.indices = vec![0, 1];
        let model = tobj::Model {
            mesh,
            name: "lines".to_string(),
        };
        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        assert!(ai_meshes[0].primitive_types.contains(AiPrimitiveType::Line));

        // Test triangles
        let mut mesh = create_test_mesh();
        mesh.face_arities = vec![3];
        mesh.indices = vec![0, 1, 2];
        let model = tobj::Model {
            mesh,
            name: "triangles".to_string(),
        };
        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        assert!(
            ai_meshes[0]
                .primitive_types
                .contains(AiPrimitiveType::Triangle)
        );

        // Test polygons
        let mut mesh = create_test_mesh();
        mesh.face_arities = vec![5];
        mesh.indices = vec![0, 1, 2, 3, 4];
        let model = tobj::Model {
            mesh,
            name: "polygons".to_string(),
        };
        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        assert!(
            ai_meshes[0]
                .primitive_types
                .contains(AiPrimitiveType::Polygon)
        );
    }

    #[test]
    fn test_import_meshes_mixed_primitive_types() {
        let mut mesh = create_test_mesh();
        mesh.face_arities = vec![1, 2, 3, 4]; // point, line, triangle, quad
        mesh.indices = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let model = tobj::Model {
            mesh,
            name: "mixed".to_string(),
        };

        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        let mesh = &ai_meshes[0];

        assert!(mesh.primitive_types.contains(AiPrimitiveType::Point));
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Line));
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle));
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Polygon));

        assert_eq!(mesh.faces.len(), 4);
        assert_eq!(mesh.faces[0], vec![0]); // point
        assert_eq!(mesh.faces[1], vec![1, 2]); // line
        assert_eq!(mesh.faces[2], vec![3, 4, 5]); // triangle
        assert_eq!(mesh.faces[3], vec![6, 7, 8, 9]); // quad
    }

    #[test]
    fn test_import_meshes_no_material_id() {
        let mut mesh = create_test_mesh();
        mesh.material_id = None;

        let model = tobj::Model {
            mesh,
            name: "no_material".to_string(),
        };

        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        assert_eq!(ai_meshes[0].material_index, 0); // should default to 0
    }

    #[test]
    fn test_import_meshes_empty_attributes() {
        let mesh = tobj::Mesh {
            positions: vec![],
            normals: vec![],
            texcoords: vec![],
            vertex_color: vec![],
            indices: vec![],
            face_arities: vec![],
            material_id: None,
            texcoord_indices: vec![],
            normal_indices: vec![],
        };

        let model = tobj::Model {
            mesh,
            name: "empty".to_string(),
        };

        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        let mesh = &ai_meshes[0];

        assert_eq!(mesh.vertices.len(), 0);
        assert_eq!(mesh.normals.len(), 0);
        assert!(mesh.texture_coords[0].is_none());
        assert!(mesh.colors[0].is_none());
        assert_eq!(mesh.faces.len(), 0);
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle)); // default when no face_arities
    }

    #[test]
    fn test_import_meshes_partial_attributes() {
        let mesh = tobj::Mesh {
            positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], // 2 vertices
            normals: vec![],                               // no normals
            texcoords: vec![0.0, 0.0, 1.0, 0.0],           // 2 texture coords
            vertex_color: vec![],                          // no colors
            indices: vec![0, 1],                           // 1 line
            face_arities: vec![2],
            material_id: Some(5),
            texcoord_indices: vec![],
            normal_indices: vec![],
        };

        let model = tobj::Model {
            mesh,
            name: "partial".to_string(),
        };

        let ImportMeshes(ai_meshes, _) = ObjImporter::import_meshes(vec![model]);
        let mesh = &ai_meshes[0];

        assert_eq!(mesh.vertices.len(), 2);
        assert_eq!(mesh.normals.len(), 0);
        assert!(mesh.texture_coords[0].is_some());
        assert_eq!(mesh.texture_coords[0].as_ref().unwrap().len(), 2);
        assert!(mesh.colors[0].is_none());
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0], vec![0, 1]);
        assert_eq!(mesh.material_index, 5);
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Line));
    }

    #[test]
    fn test_import_meshes_complex_face_arities() {
        let mut mesh = create_test_mesh();
        mesh.face_arities = vec![3, 4, 5, 3]; // triangle, quad, pentagon, triangle
        mesh.indices = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

        let model = tobj::Model {
            mesh,
            name: "complex".to_string(),
        };

        let result = ObjImporter::import_meshes(vec![model]);
        let mesh = &result.0[0];

        assert_eq!(mesh.faces.len(), 4);
        assert_eq!(mesh.faces[0], vec![0, 1, 2]); // triangle
        assert_eq!(mesh.faces[1], vec![3, 4, 5, 6]); // quad
        assert_eq!(mesh.faces[2], vec![7, 8, 9, 10, 11]); // pentagon
        assert_eq!(mesh.faces[3], vec![12, 13, 14]); // triangle

        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle));
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Polygon));
    }
}
