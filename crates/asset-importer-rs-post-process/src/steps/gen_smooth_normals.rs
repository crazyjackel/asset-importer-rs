use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps, Spatial, SpatialLookup};
use asset_importer_rs_scene::{
    AiPrimitiveType, AiReal, AiScene, AiSceneFlag, AiVector3D, degrees_to_radians,
};
use enumflags2::BitFlags;

use crate::helper::EpsilonCompute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenSmoothNormalsError {
    NonVerboseFormat,
}

impl std::fmt::Display for GenSmoothNormalsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenSmoothNormalsError::NonVerboseFormat => {
                write!(f, "Non-verbose format is not supported")
            }
        }
    }
}

impl std::error::Error for GenSmoothNormalsError {}

/// Generate smooth normals for meshes
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GenSmoothNormals {
    pub smooth_angle: f32,
    pub force_gen_normals: bool,
    pub flip_winding_order: bool,
    pub left_handed: bool,
}

impl Default for GenSmoothNormals {
    fn default() -> Self {
        Self {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        }
    }
}

impl AiPostProcess for GenSmoothNormals {
    type Error = GenSmoothNormalsError;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        self.force_gen_normals = steps.contains(AiPostProcessSteps::ForceGenNormals);
        self.flip_winding_order = steps.contains(AiPostProcessSteps::FlipWindingOrder);
        self.left_handed = steps.contains(AiPostProcessSteps::MakeLeftHanded);
        steps.contains(AiPostProcessSteps::GenSmoothNormals)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        if scene.flags.contains(AiSceneFlag::NonVerboseFormat) {
            return Err(GenSmoothNormalsError::NonVerboseFormat);
        }

        for mesh in scene.meshes.iter_mut() {
            let vertex_count = mesh.vertices.len();
            if !(mesh.primitive_types.contains(AiPrimitiveType::Polygon)
                || mesh.primitive_types.contains(AiPrimitiveType::Triangle))
            {
                continue;
            }

            let empty_normal = [0.0, 0.0, 0.0].into();
            let default_normal = [0.0, 1.0, 0.0].into();
            // Unless forced, skip normals that are already generated and not degenerate
            if !self.force_gen_normals
                && mesh.normals.len() == vertex_count
                && !mesh.normals.contains(&empty_normal)
            {
                continue;
            }
            let mut normals = Vec::with_capacity(vertex_count);
            normals.resize(vertex_count, default_normal);

            for face in mesh.faces.iter() {
                if face.len() < 3 {
                    for index in face.iter() {
                        normals[*index] = default_normal;
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

                for index in face.iter() {
                    normals[*index] = normal;
                }
            }

            let spatial = Spatial::new(&mesh.vertices);
            let epsilon: AiReal = mesh.epsilon();

            let mut new_normals = vec![default_normal; vertex_count];
            if self.smooth_angle >= degrees_to_radians(175.0) {
                let mut indexes_processed: Vec<bool> = vec![false; vertex_count];
                for index in 0..vertex_count {
                    if indexes_processed[index] {
                        continue;
                    }
                    let position = mesh.vertices[index];
                    let found = spatial.find_position(position, epsilon);
                    let mut new_normal = AiVector3D::zero();
                    for normal_index in &found {
                        new_normal += normals[*normal_index];
                    }
                    new_normal.norm();
                    if new_normal.len() == 0.0 {
                        new_normal = default_normal;
                    }
                    for normal_index in &found {
                        new_normals[*normal_index] = new_normal;
                        indexes_processed[*normal_index] = true;
                    }
                }
            } else {
                let limit: AiReal = self.smooth_angle.cos();
                for index in 0..vertex_count {
                    let found = spatial.find_position(mesh.vertices[index], epsilon);
                    let normal = normals[index];
                    let mut new_normal = AiVector3D::zero();
                    for normal_index in &found {
                        if *normal_index == index {
                            continue;
                        }
                        let index_normal = normals[*normal_index];
                        if normal * index_normal >= limit {
                            new_normal += index_normal;
                        }
                    }
                    new_normal.normalize();
                    new_normals[index] = new_normal;
                }
            }

            mesh.normals = new_normals;
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
                AiFace::from(vec![0, 1, 2]), // back face (-Z)
                AiFace::from(vec![0, 2, 3]), // back face (-Z)
                AiFace::from(vec![4, 7, 6]), // front face (+Z)
                AiFace::from(vec![4, 6, 5]), // front face (+Z)
                AiFace::from(vec![0, 4, 5]), // bottom face (-Y)
                AiFace::from(vec![0, 5, 1]), // bottom face (-Y)
                AiFace::from(vec![1, 5, 6]), // right face (+X)
                AiFace::from(vec![1, 6, 2]), // right face (+X)
                AiFace::from(vec![2, 6, 7]), // top face (+Y)
                AiFace::from(vec![2, 7, 3]), // top face (+Y)
                AiFace::from(vec![4, 0, 3]), // left face (-X)
                AiFace::from(vec![4, 3, 7]), // left face (-X)
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
    fn test_gen_smooth_normals_basic() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        };

        // Test prepare method
        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        assert!(gen_smooth_normals.prepare(steps));

        // Test process method
        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());

        // Check that normals are not zero vectors
        for normal in &mesh.normals {
            assert_eq!(normal.len(), 1.0);
            assert!(normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0);
        }
    }

    #[test]
    fn test_gen_smooth_normals_non_verbose_format() {
        let mut scene = create_test_scene();
        scene.flags = BitFlags::from(AiSceneFlag::NonVerboseFormat);

        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GenSmoothNormalsError::NonVerboseFormat);
    }

    #[test]
    fn test_gen_smooth_normals_force_generation() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
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

        let steps = AiPostProcessSteps::GenSmoothNormals | AiPostProcessSteps::ForceGenNormals;
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were regenerated despite existing ones
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());

        // Check that new normals are not zero vectors
        for normal in &mesh.normals {
            assert_eq!(normal.len(), 1.0);
            assert!(normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0);
        }
    }

    #[test]
    fn test_gen_smooth_normals_skip_existing() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
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

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were not regenerated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals, original_normals);
    }

    #[test]
    fn test_gen_smooth_normals_regenerates_for_degenerate_normals() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: false,
            flip_winding_order: false,
            left_handed: false,
        };

        // Add degenerate existing normals (only 7 out of 8 vertices)
        scene.meshes[0].normals = vec![
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
        ];

        let original_normals = scene.meshes[0].normals.clone();

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were regenerated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
        assert_ne!(mesh.normals.len(), original_normals.len());
    }

    #[test]
    fn test_gen_smooth_normals_winding_order() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: true,
            flip_winding_order: true,
            left_handed: false,
        };

        let steps = AiPostProcessSteps::GenSmoothNormals | AiPostProcessSteps::FlipWindingOrder;
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_smooth_normals_left_handed() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: true,
        };

        let steps = AiPostProcessSteps::GenSmoothNormals | AiPostProcessSteps::MakeLeftHanded;
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_smooth_normals_polygon_primitives() {
        let mut scene = create_test_scene();
        scene.meshes[0].primitive_types = BitFlags::from(AiPrimitiveType::Polygon);

        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_smooth_normals_unsupported_primitives() {
        let mut scene = create_test_scene();
        scene.meshes[0].primitive_types = BitFlags::from(AiPrimitiveType::Line);

        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were not generated for unsupported primitives
        let mesh = &scene.meshes[0];
        assert!(mesh.normals.is_empty());
    }

    #[test]
    fn test_gen_smooth_normals_small_faces() {
        let mut scene = create_test_scene();
        // Create a mesh with faces that have less than 3 vertices
        scene.meshes[0].faces = vec![
            AiFace::from(vec![0, 1]),    // Only 2 vertices
            AiFace::from(vec![2, 3, 4]), // Valid triangle
        ];

        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0),
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_smooth_normals_high_smooth_angle() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(175.0), // High angle threshold
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_smooth_normals_low_smooth_angle() {
        let mut scene = create_test_scene();
        let mut gen_smooth_normals = GenSmoothNormals {
            smooth_angle: degrees_to_radians(30.0), // Low angle threshold
            force_gen_normals: true,
            flip_winding_order: false,
            left_handed: false,
        };

        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        gen_smooth_normals.prepare(steps);

        let result = gen_smooth_normals.process(&mut scene);
        assert!(result.is_ok());

        // Check that normals were generated
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_gen_smooth_normals_default_values() {
        let gen_smooth_normals = GenSmoothNormals::default();

        // Check default values
        assert_eq!(gen_smooth_normals.smooth_angle, degrees_to_radians(175.0));
        assert_eq!(gen_smooth_normals.force_gen_normals, false);
        assert_eq!(gen_smooth_normals.flip_winding_order, false);
        assert_eq!(gen_smooth_normals.left_handed, false);
    }

    #[test]
    fn test_gen_smooth_normals_prepare_method() {
        let mut gen_smooth_normals = GenSmoothNormals::default();

        // Test with GenSmoothNormals step
        let steps = BitFlags::from(AiPostProcessSteps::GenSmoothNormals);
        assert!(gen_smooth_normals.prepare(steps));
        assert_eq!(gen_smooth_normals.force_gen_normals, false);
        assert_eq!(gen_smooth_normals.flip_winding_order, false);
        assert_eq!(gen_smooth_normals.left_handed, false);

        // Test with additional steps
        let steps = AiPostProcessSteps::GenSmoothNormals
            | AiPostProcessSteps::ForceGenNormals
            | AiPostProcessSteps::FlipWindingOrder
            | AiPostProcessSteps::MakeLeftHanded;
        assert!(gen_smooth_normals.prepare(steps));
        assert_eq!(gen_smooth_normals.force_gen_normals, true);
        assert_eq!(gen_smooth_normals.flip_winding_order, true);
        assert_eq!(gen_smooth_normals.left_handed, true);

        // Test without GenSmoothNormals step
        let steps = BitFlags::from(AiPostProcessSteps::ForceGenNormals);
        assert!(!gen_smooth_normals.prepare(steps));
    }
}
