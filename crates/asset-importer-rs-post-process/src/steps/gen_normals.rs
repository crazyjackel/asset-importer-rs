use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::{AiPrimitiveType, AiScene, AiSceneFlag, AiVector3D};
use enumflags2::BitFlags;

#[derive(Debug, PartialEq)]
pub enum GenNormalsError {
    NonVerboseFormat,
}

impl std::fmt::Display for GenNormalsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenNormalsError::NonVerboseFormat => write!(
                f,
                "Non-verbose vertex format is not supported for normals generation. Have you run JoinIdenticalVertices?"
            ),
        }
    }
}

impl std::error::Error for GenNormalsError {}

/// Generate normals for meshes
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GenNormals {
    pub force_gen_normals: bool,
    pub flip_winding_order: bool,
    pub left_handed: bool,
}

impl AiPostProcess for GenNormals {
    type Error = GenNormalsError;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        self.force_gen_normals = steps.contains(AiPostProcessSteps::ForceGenNormals);
        self.flip_winding_order = steps.contains(AiPostProcessSteps::FlipWindingOrder);
        self.left_handed = steps.contains(AiPostProcessSteps::MakeLeftHanded);
        steps.contains(AiPostProcessSteps::GenNormals)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        if scene.flags.contains(AiSceneFlag::NonVerboseFormat) {
            return Err(GenNormalsError::NonVerboseFormat);
        }

        let empty_normal = [0.0, 0.0, 0.0].into();
        let default_normal = [0.0, 1.0, 0.0].into();
        for mesh in scene.meshes.iter_mut() {
            let vertex_count = mesh.vertices.len();
            // Only process meshes with polygon or triangle primitives
            if !(mesh.primitive_types.contains(AiPrimitiveType::Polygon)
                || mesh.primitive_types.contains(AiPrimitiveType::Triangle))
            {
                continue;
            }

            // Unless forced, skip normals that are already generated and not degenerate
            if !self.force_gen_normals
                && mesh.normals.len() == vertex_count
                && !mesh.normals.contains(&empty_normal)
            {
                continue;
            }

            let mut normals: Vec<AiVector3D> = Vec::with_capacity(vertex_count);
            normals.resize(vertex_count, default_normal);
            let mut already_referenced: Vec<bool> = Vec::with_capacity(vertex_count);
            already_referenced.resize(vertex_count, false);
            let mut duplicate_vertices: Vec<AiVector3D> = Vec::with_capacity(vertex_count);

            fn store_normal(
                index: usize,
                normal: &AiVector3D,
                already_referenced: &mut [bool],
                normals: &mut Vec<AiVector3D>,
                duplicate_vertices: &mut Vec<AiVector3D>,
                mesh_vertices: &[AiVector3D],
                vertex_count: usize,
            ) -> usize {
                if !already_referenced[index] {
                    normals[index] = *normal;
                    already_referenced[index] = true;
                    index
                } else {
                    duplicate_vertices.push(mesh_vertices[index]);
                    normals.push(*normal);
                    vertex_count + duplicate_vertices.len() - 1
                }
            }

            for face in mesh.faces.iter_mut() {
                if face.len() < 3 {
                    for index in face.iter_mut() {
                        *index = store_normal(
                            *index,
                            &default_normal,
                            &mut already_referenced,
                            &mut normals,
                            &mut duplicate_vertices,
                            &mesh.vertices,
                            vertex_count,
                        );
                    }
                    continue;
                }

                let winding_order_changed = self.flip_winding_order != self.left_handed;

                let v1 = &mesh.vertices[face[0]];
                let v2 = &mesh.vertices[face[1]];
                let v3 = &mesh.vertices[face[face.len() - 1]];

                let n1 = v2 - v1;
                let n2 = v3 - v1;

                let normal = if winding_order_changed {
                    n1.cross(&n2).norm()
                } else {
                    n2.cross(&n1).norm()
                };

                if winding_order_changed {
                    let temp = mesh.vertices[face[1]];
                    mesh.vertices[face[1]] = mesh.vertices[face[face.len() - 1]];
                    mesh.vertices[face[face.len() - 1]] = temp;
                }

                for index in face.iter_mut() {
                    *index = store_normal(
                        *index,
                        &normal,
                        &mut already_referenced,
                        &mut normals,
                        &mut duplicate_vertices,
                        &mesh.vertices,
                        vertex_count,
                    );
                }
            }

            if !duplicate_vertices.is_empty() {
                mesh.vertices.extend(duplicate_vertices);
            }
            mesh.normals = normals;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asset_importer_rs_scene::{AiFace, AiMesh, AiPrimitiveType, AiSceneFlag};
    use enumflags2::BitFlags;

    fn create_test_mesh() -> AiMesh {
        AiMesh {
            vertices: vec![
                [-1.0, -1.0, -1.0].into(),
                [1.0, -1.0, -1.0].into(),
                [1.0, 1.0, -1.0].into(),
                [-1.0, 1.0, -1.0].into(),
                [-1.0, -1.0, 1.0].into(),
                [1.0, -1.0, 1.0].into(),
                [1.0, 1.0, 1.0].into(),
                [-1.0, 1.0, 1.0].into(),
            ],
            normals: vec![],
            faces: vec![
                AiFace::from(vec![0, 1, 2]), // back face (-Z), f 1 2 3
                AiFace::from(vec![0, 2, 3]), // back face (-Z), f 1 3 4
                AiFace::from(vec![4, 7, 6]), // front face (+Z), f 5 8 7
                AiFace::from(vec![4, 6, 5]), // front face (+Z), f 5 7 6
                AiFace::from(vec![0, 4, 5]), // bottom face (-Y), f 1 5 6
                AiFace::from(vec![0, 5, 1]), // bottom face (-Y), f 1 6 2
                AiFace::from(vec![1, 5, 6]), // right face (+X), f 2 6 7
                AiFace::from(vec![1, 6, 2]), // right face (+X), f 2 7 3
                AiFace::from(vec![2, 6, 7]), // top face (+Y), f 3 7 8
                AiFace::from(vec![2, 7, 3]), // top face (+Y), f 3 8 4
                AiFace::from(vec![4, 0, 3]), // left face (-X), f 5 1 4
                AiFace::from(vec![4, 3, 7]), // left face (-X), f 5 4 8
            ],
            primitive_types: BitFlags::from(AiPrimitiveType::Triangle),
            ..Default::default()
        }
    }

    fn create_test_scene() -> AiScene {
        AiScene {
            meshes: vec![create_test_mesh()],
            flags: BitFlags::empty(), // Not non-verbose
            ..Default::default()
        }
    }

    #[test]
    fn test_gen_normals_basic() {
        let mut scene = create_test_scene();
        let mut gen_normals = GenNormals {
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        };

        // Test prepare method
        let steps = BitFlags::from(AiPostProcessSteps::GenNormals);
        assert!(gen_normals.prepare(steps));

        // Test process method
        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());

        assert_eq!(mesh.normals.len(), 36);
        assert_eq!(mesh.vertices.len(), 36);
        assert_eq!(mesh.faces.len(), 12);

        // Check that normals are not zero vectors
        for normal in &mesh.normals {
            assert!(normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0);
        }
    }

    #[test]
    fn test_gen_normals_non_verbose_format() {
        let mut scene = create_test_scene();
        scene.flags = BitFlags::from(AiSceneFlag::NonVerboseFormat);

        let mut gen_normals = GenNormals {
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenNormals);
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GenNormalsError::NonVerboseFormat);
    }

    #[test]
    fn test_gen_normals_force_generation() {
        let mut scene = create_test_scene();
        let mut gen_normals = GenNormals {
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        // Add some existing normals
        scene.meshes[0].normals = vec![
            [0.0, 0.0, 0.0].into(), // Empty normal
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 0.0].into(),
        ];

        let steps = AiPostProcessSteps::GenNormals | AiPostProcessSteps::ForceGenNormals;
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were regenerated despite existing ones
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());

        // Check that new normals are not zero vectors
        for normal in &mesh.normals {
            assert!(normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0);
        }
    }

    #[test]
    fn test_gen_normals_skip_existing() {
        let mut scene = create_test_scene();
        let mut gen_normals = GenNormals {
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        };

        // Add valid existing normals
        scene.meshes[0].normals = vec![
            [0.0, 0.0, 1.0].into(), // Valid normal
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
        ];

        let original_normals = scene.meshes[0].normals.clone();

        let steps = BitFlags::from(AiPostProcessSteps::GenNormals);
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were not regenerated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals, original_normals);
    }

    #[test]
    fn test_gen_normals_regenerates_for_degenerate_normals() {
        let mut scene = create_test_scene();
        let mut gen_normals = GenNormals {
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        };

        // Add valid existing normals
        scene.meshes[0].normals = vec![
            [0.0, 0.0, 1.0].into(), // Invalid normals (7/8)
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
        ];

        let original_normals = scene.meshes[0].normals.clone();

        let steps = BitFlags::from(AiPostProcessSteps::GenNormals);
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were not regenerated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
        assert_ne!(mesh.normals, original_normals);
    }

    #[test]
    fn test_gen_normals_winding_order() {
        let mut scene = create_test_scene();
        let mut gen_normals = GenNormals {
            force_gen_normals: true,
            flip_winding_order: true,
            left_handed: false,
        };

        let steps = AiPostProcessSteps::GenNormals | AiPostProcessSteps::FlipWindingOrder;
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_normals_left_handed() {
        let mut scene = create_test_scene();
        let mut gen_normals = GenNormals {
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: true,
        };

        let steps = AiPostProcessSteps::GenNormals | AiPostProcessSteps::MakeLeftHanded;
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_normals_polygon_primitives() {
        let mut scene = create_test_scene();
        scene.meshes[0].primitive_types = BitFlags::from(AiPrimitiveType::Polygon);

        let mut gen_normals = GenNormals {
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenNormals);
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_normals_unsupported_primitives() {
        let mut scene = create_test_scene();
        scene.meshes[0].primitive_types = BitFlags::from(AiPrimitiveType::Line);

        let mut gen_normals = GenNormals {
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenNormals);
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were not generated for unsupported primitives
        let mesh = &scene.meshes[0];
        assert!(mesh.normals.is_empty());
    }

    #[test]
    fn test_gen_normals_duplicate_vertices() {
        let mut scene = create_test_scene();
        // Create a mesh with duplicate vertices
        scene.meshes[0].faces = vec![
            AiFace::from(vec![0, 1, 2]), // First triangle
            AiFace::from(vec![0, 1, 2]), // Same triangle (duplicate vertices)
        ];

        let mut gen_normals = GenNormals {
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenNormals);
        gen_normals.prepare(steps);

        let result = gen_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated and duplicate vertices were handled
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }
}
