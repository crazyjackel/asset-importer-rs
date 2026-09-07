use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::{
    AI_MATH_PI, AiFace, AiMesh, AiPrimitiveType, AiReal, AiScene, AiVector3D, NewellNormal,
};
use earcut::Earcut;
use enumflags2::BitFlags;

/// Split n-gons into triangles, matching Assimp's triangulate post-process.
///
/// Faces with three indices are NGON-encoded as standalone triangles, quads are
/// fanned (from a concave vertex when present), and n ≥ 5 are projected to 2D
/// and earcut. The mesh is flagged `Triangle | NgonEncodingFlag`.
#[derive(Default)]
pub struct Triangulate;

/// Mesh-level triangulation used by [`Triangulate`].
trait TriangulateMesh {
    /// Replace polygon faces on this mesh with triangles.
    fn triangulate(&mut self);
}

impl AiPostProcess for Triangulate {
    type Error = String;

    /// Active when [`AiPostProcessSteps::Triangulate`] is set.
    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::Triangulate)
    }

    /// Triangulate every mesh in `scene`.
    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        for mesh in scene.meshes.iter_mut() {
            mesh.triangulate();
        }
        Ok(())
    }
}

/// Encode a real triangle so it is not merged with the previous n-gon.
///
/// Consecutive triangles that share the same first index are one n-gon, fanned
/// from that vertex. If `tri[0]` matches `last_ngon_first_index`, right-rotate
/// (`ABC` → `CAB`) so a decoder starts a new n-gon.
fn ngon_encode_triangle(tri: &mut [usize], last_ngon_first_index: &mut Option<usize>) {
    debug_assert_eq!(tri.len(), 3);
    if *last_ngon_first_index == Some(tri[0]) {
        tri.swap(0, 2);
        tri.swap(1, 2);
    }
    *last_ngon_first_index = Some(tri[0]);
}

/// Encode a quad as two triangles that reconstruct as a single n-gon.
///
/// Both triangles must already share a fan vertex at index 0. If that vertex
/// matches the previous n-gon, fan from the opposite vertex instead (right-rotate
/// `tri1`, left-rotate `tri2`) so the pair stays one n-gon without merging.
fn ngon_encode_quad(
    tri1: &mut [usize],
    tri2: &mut [usize],
    last_ngon_first_index: &mut Option<usize>,
) {
    debug_assert_eq!(tri1.len(), 3);
    debug_assert_eq!(tri2.len(), 3);
    debug_assert_eq!(tri1[0], tri2[0]);
    if *last_ngon_first_index == Some(tri1[0]) {
        tri1.swap(0, 2);
        tri1.swap(1, 2);
        tri2.swap(1, 2);
        tri2.swap(0, 2);
        debug_assert_eq!(tri1[0], tri2[0]);
    }
    *last_ngon_first_index = Some(tri1[0]);
}

impl TriangulateMesh for AiMesh {
    /// Split this mesh's faces into triangles and apply NGON encoding.
    fn triangulate(&mut self) {
        // If the mesh does not contain polygons as a primitive type, return early.
        if !self.primitive_types.contains(AiPrimitiveType::Polygon) {
            return;
        }

        // If the mesh is already triangulated, return early.
        let mut is_triangulated = true;
        for face in self.faces.iter() {
            if face.len() != 3 {
                is_triangulated = false;
                break;
            }
        }
        if is_triangulated {
            return;
        }

        // Precalculate output faces count and maximum n-gon size.
        let mut max_face_size = 0;
        let mut output_faces_count = 0;
        for face in self.faces.iter() {
            if face.len() <= 3 {
                output_faces_count += 1;
            } else {
                output_faces_count += face.len() - 2;
                max_face_size = max_face_size.max(face.len());
            }
        }

        // Replacement face list plus reusable earcut scratch (Assimp: out, temp_verts3d, temp_poly).
        let mut output_faces: Vec<AiFace> = Vec::with_capacity(output_faces_count);
        let mut last_ngon_first_index: Option<usize> = None;
        let mut temp_verts3d: Vec<AiVector3D> = Vec::with_capacity(max_face_size);
        let mut temp_verts2d: Vec<[AiReal; 2]> = Vec::with_capacity(max_face_size);
        let mut earcut = Earcut::<AiReal>::new();
        let mut earcut_indices: Vec<usize> = Vec::new();

        self.primitive_types |= AiPrimitiveType::Triangle | AiPrimitiveType::NgonEncodingFlag; // Set triangle and ngon encoding flags.
        self.primitive_types &= !AiPrimitiveType::Polygon; // Remove polygon flag.

        for face in self.faces.iter_mut() {
            match face.len() {
                0..=3 => {
                    // No faces less than 3 vertices exist by definition, but we assert it here for sanity.
                    debug_assert_eq!(face.len(), 3);
                    // Encode the Triangle face
                    ngon_encode_triangle(face, &mut last_ngon_first_index);
                    output_faces.push(face.clone());
                }
                4 => {
                    // Quads have at most one concave vertex; fan from it when present.
                    let vertices = &self.vertices;

                    // Find the start vertex for the fan
                    // This is the first vertex
                    let mut start_vertex = 0;
                    for i in 0..4 {
                        // Select a quad ordering of vertices
                        let v = vertices[face[i]];
                        let v0 = vertices[face[(i + 3) % 4]];
                        let v1 = vertices[face[(i + 2) % 4]];
                        let v2 = vertices[face[(i + 1) % 4]];

                        // Calculate the angles between the edges
                        let left = (v0 - v).norm();
                        let diag = (v1 - v).norm();
                        let right = (v2 - v).norm();
                        // Angle is the sum of left and right angles between diagonals
                        let angle = (left * diag).acos() + (right * diag).acos();

                        // If the angle is greater than PI, then vertex is a concave corner
                        if angle > AI_MATH_PI {
                            start_vertex = i;
                            break;
                        }
                    }

                    // Create the two triangles that make up the quad
                    let mut tri1 = vec![
                        face[start_vertex],
                        face[(start_vertex + 1) % 4],
                        face[(start_vertex + 2) % 4],
                    ];
                    let mut tri2 = vec![
                        face[start_vertex],
                        face[(start_vertex + 2) % 4],
                        face[(start_vertex + 3) % 4],
                    ];

                    // Encode the quad as two triangles
                    ngon_encode_quad(&mut tri1, &mut tri2, &mut last_ngon_first_index);
                    output_faces.push(tri1);
                    output_faces.push(tri2);
                }
                _ => {
                    // Set up earcut scratch space
                    let vertices = &self.vertices;
                    temp_verts3d.clear();
                    temp_verts2d.clear();
                    earcut_indices.clear();

                    // fill temp vertices 3d
                    for i in 0..face.len() {
                        temp_verts3d.push(vertices[face[i]]);
                    }

                    // Calculate the newell normal from the temp vertices 3d
                    let newell_normal = temp_verts3d.newell_normal();

                    // Get the absolute values of the newell normal
                    let abs_newell_x = if newell_normal.x > 0.0 {
                        newell_normal.x
                    } else {
                        -newell_normal.x
                    };
                    let abs_newell_y = if newell_normal.y > 0.0 {
                        newell_normal.y
                    } else {
                        -newell_normal.y
                    };
                    let abs_newell_z = if newell_normal.z > 0.0 {
                        newell_normal.z
                    } else {
                        -newell_normal.z
                    };

                    // Determine the primary and secondary components based on the newell normal
                    let mut primary_component: usize = 0;
                    let mut secondary_component: usize = 1;
                    // The inverse value is the largest signed component of the newell normal
                    let mut inverse_value = newell_normal.z;
                    if abs_newell_x > abs_newell_y && abs_newell_x > abs_newell_z {
                        primary_component = 1;
                        secondary_component = 2;
                        inverse_value = newell_normal.x;
                    } else if abs_newell_y > abs_newell_z {
                        primary_component = 2;
                        secondary_component = 0;
                        inverse_value = newell_normal.y;
                    }

                    if inverse_value < 0.0 {
                        std::mem::swap(&mut primary_component, &mut secondary_component);
                    }

                    // Fill Temp Vertices 2D using the primary and secondary indices to project the 3d vertices onto the plane
                    for i in 0..face.len() {
                        let vertex = vertices[face[i]];
                        let primary_value = vertex[primary_component] as AiReal;
                        let secondary_value = vertex[secondary_component] as AiReal;
                        temp_verts2d.push([primary_value, secondary_value]);
                    }

                    // Earcut the projected 2d vertices to get the new triangle indices.
                    earcut.earcut(temp_verts2d.iter().copied(), &[], &mut earcut_indices);

                    // Earcut returns local ring indices; map them back to mesh vertex ids.
                    for chunk in earcut_indices.as_chunks::<3>().0 {
                        let mut tri = [face[chunk[0]], face[chunk[1]], face[chunk[2]]];
                        ngon_encode_triangle(&mut tri, &mut last_ngon_first_index);
                        output_faces.push(tri.to_vec());
                    }
                }
            }
        }

        self.faces = output_faces;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// NGON-encode a triangle that does not share a fan vertex with the previous n-gon.
    #[test]
    fn triangle_keeps_indices_when_not_colliding() {
        let mut last = None;
        let mut tri = [0, 1, 2];
        ngon_encode_triangle(&mut tri, &mut last);
        assert_eq!(tri, [0, 1, 2]);
        assert_eq!(last, Some(0));
    }

    /// Right-rotate a triangle when its first index matches the last n-gon.
    #[test]
    fn triangle_rotates_when_first_index_matches_last_ngon() {
        let mut last = Some(0);
        let mut tri = [0, 1, 2];
        ngon_encode_triangle(&mut tri, &mut last);
        assert_eq!(tri, [2, 0, 1]);
        assert_eq!(last, Some(2));
    }

    /// Sequential triangles that share a fan vertex stay separate n-gons.
    #[test]
    fn sequential_triangles_with_same_fan_do_not_merge() {
        let mut last = None;
        let mut first = [0, 1, 2];
        let mut second = [0, 3, 4];
        ngon_encode_triangle(&mut first, &mut last);
        ngon_encode_triangle(&mut second, &mut last);
        assert_eq!(first, [0, 1, 2]);
        assert_eq!(second, [4, 0, 3]);
        assert_ne!(first[0], second[0]);
    }

    /// A quad keeps a shared fan vertex when it does not collide with the last n-gon.
    #[test]
    fn quad_keeps_shared_fan_when_not_colliding() {
        let mut last = None;
        let mut tri1 = [0, 1, 2];
        let mut tri2 = [0, 2, 3];
        ngon_encode_quad(&mut tri1, &mut tri2, &mut last);
        assert_eq!(tri1, [0, 1, 2]);
        assert_eq!(tri2, [0, 2, 3]);
        assert_eq!(last, Some(0));
    }

    /// A colliding quad fans from the opposite vertex so it does not merge.
    #[test]
    fn quad_fans_from_opposite_vertex_when_colliding() {
        let mut last = Some(0);
        let mut tri1 = [0, 1, 2];
        let mut tri2 = [0, 2, 3];
        ngon_encode_quad(&mut tri1, &mut tri2, &mut last);
        assert_eq!(tri1, [2, 0, 1]);
        assert_eq!(tri2, [2, 3, 0]);
        assert_eq!(tri1[0], tri2[0]);
        assert_eq!(last, Some(2));
    }

    /// Build a single-polygon mesh for triangulation tests.
    fn mesh_with_face(verts: Vec<AiVector3D>, indices: Vec<usize>) -> AiMesh {
        AiMesh {
            vertices: verts,
            faces: vec![indices],
            primitive_types: AiPrimitiveType::Polygon.into(),
            ..Default::default()
        }
    }

    /// Build a quad mesh (same as [`mesh_with_face`]).
    fn mesh_with_quad(verts: Vec<AiVector3D>, indices: Vec<usize>) -> AiMesh {
        mesh_with_face(verts, indices)
    }

    /// Assert an n-gon became `n - 2` wound triangles using only the original indices.
    fn assert_valid_triangulation(mesh: &AiMesh, original: &[usize]) {
        let expected = original.len().saturating_sub(2);
        assert_eq!(mesh.faces.len(), expected);
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle));
        assert!(
            mesh.primitive_types
                .contains(AiPrimitiveType::NgonEncodingFlag)
        );
        assert!(!mesh.primitive_types.contains(AiPrimitiveType::Polygon));

        let poly: Vec<AiVector3D> = original.iter().map(|&i| mesh.vertices[i]).collect();
        let normal = poly.newell_normal();
        let mut seen = vec![false; original.len()];
        for face in &mesh.faces {
            assert_eq!(face.len(), 3);
            for &idx in face {
                let pos = original.iter().position(|&i| i == idx);
                assert!(pos.is_some(), "index {idx} was not in the original n-gon");
                seen[pos.unwrap()] = true;
            }
            let a = mesh.vertices[face[0]];
            let b = mesh.vertices[face[1]];
            let c = mesh.vertices[face[2]];
            let area = (b - a).cross(&(c - a)) * normal;
            assert!(
                area > 0.0,
                "triangle {face:?} has non-positive area along the polygon normal"
            );
        }
        assert!(
            seen.iter().all(|&v| v),
            "every original vertex should appear in some triangle"
        );
    }

    /// A convex unit square fans from vertex 0.
    #[test]
    fn convex_quad_fans_from_first_vertex() {
        let mut mesh = mesh_with_quad(
            vec![
                [0.0, 0.0, 0.0].into(),
                [1.0, 0.0, 0.0].into(),
                [1.0, 1.0, 0.0].into(),
                [0.0, 1.0, 0.0].into(),
            ],
            vec![0, 1, 2, 3],
        );
        mesh.triangulate();
        assert_eq!(mesh.faces, vec![vec![0, 1, 2], vec![0, 2, 3]]);
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle));
        assert!(
            mesh.primitive_types
                .contains(AiPrimitiveType::NgonEncodingFlag)
        );
        assert!(!mesh.primitive_types.contains(AiPrimitiveType::Polygon));
    }

    /// A concave dart fans from the reflex vertex.
    #[test]
    fn concave_quad_fans_from_reflex_vertex() {
        let mut mesh = mesh_with_quad(
            vec![
                [0.0, 0.0, 0.0].into(),
                [2.0, 0.0, 0.0].into(),
                [0.5, 0.5, 0.0].into(),
                [0.0, 2.0, 0.0].into(),
            ],
            vec![0, 1, 2, 3],
        );
        mesh.triangulate();
        assert_eq!(mesh.faces.len(), 2);
        assert_eq!(mesh.faces[0][0], mesh.faces[1][0]);
        assert_eq!(mesh.faces[0][0], 2);
    }

    /// A convex pentagon earcuts into three triangles.
    #[test]
    fn convex_pentagon_earcuts_into_three_triangles() {
        let original = vec![0, 1, 2, 3, 4];
        let mut mesh = mesh_with_face(
            vec![
                [0.0, 0.0, 0.0].into(),
                [2.0, 0.0, 0.0].into(),
                [3.0, 1.0, 0.0].into(),
                [1.0, 2.0, 0.0].into(),
                [0.0, 1.0, 0.0].into(),
            ],
            original.clone(),
        );
        mesh.triangulate();
        assert_valid_triangulation(&mesh, &original);
    }

    /// Earcut ring indices are remapped to mesh vertex ids.
    #[test]
    fn earcut_remaps_local_indices_to_mesh_vertex_ids() {
        let original = vec![5, 6, 7, 8, 9];
        let mut verts = vec![AiVector3D::zero(); 10];
        verts[5] = [0.0, 0.0, 0.0].into();
        verts[6] = [2.0, 0.0, 0.0].into();
        verts[7] = [3.0, 1.0, 0.0].into();
        verts[8] = [1.0, 2.0, 0.0].into();
        verts[9] = [0.0, 1.0, 0.0].into();
        let mut mesh = mesh_with_face(verts, original.clone());
        mesh.triangulate();
        assert_valid_triangulation(&mesh, &original);
        for face in &mesh.faces {
            for &idx in face {
                assert!((5..=9).contains(&idx));
            }
        }
    }

    /// A concave pentagon does not emit a hull triangle over the notch.
    #[test]
    fn concave_pentagon_earcuts_without_covering_the_notch() {
        let original = vec![0, 1, 2, 3, 4];
        let mut mesh = mesh_with_face(
            vec![
                [0.0, 0.0, 0.0].into(),
                [3.0, 0.0, 0.0].into(),
                [1.0, 1.0, 0.0].into(),
                [3.0, 2.0, 0.0].into(),
                [0.0, 2.0, 0.0].into(),
            ],
            original.clone(),
        );
        mesh.triangulate();
        assert_valid_triangulation(&mesh, &original);
        // The notch at vertex 2 must not form a triangle that skips it as an ear
        // of the outer hull (1,3,something) covering the concavity.
        for face in &mesh.faces {
            let mut sorted = face.clone();
            sorted.sort_unstable();
            assert_ne!(sorted, vec![1, 3, 4]);
            assert_ne!(sorted, vec![0, 1, 3]);
        }
    }

    /// An XZ-plane pentagon projects to 2D then triangulates.
    #[test]
    fn pentagon_in_xz_plane_projects_and_triangulates() {
        let original = vec![0, 1, 2, 3, 4];
        let mut mesh = mesh_with_face(
            vec![
                [0.0, 0.0, 0.0].into(),
                [2.0, 0.0, 0.0].into(),
                [3.0, 0.0, 1.0].into(),
                [1.0, 0.0, 2.0].into(),
                [0.0, 0.0, 1.0].into(),
            ],
            original.clone(),
        );
        mesh.triangulate();
        assert_valid_triangulation(&mesh, &original);
    }

    /// Clockwise n-gons keep winding after the Newell axis swap.
    #[test]
    fn clockwise_pentagon_keeps_winding() {
        let original = vec![0, 1, 2, 3, 4];
        let mut mesh = mesh_with_face(
            vec![
                [0.0, 1.0, 0.0].into(),
                [1.0, 2.0, 0.0].into(),
                [3.0, 1.0, 0.0].into(),
                [2.0, 0.0, 0.0].into(),
                [0.0, 0.0, 0.0].into(),
            ],
            original.clone(),
        );
        mesh.triangulate();
        assert_valid_triangulation(&mesh, &original);
    }

    /// A triangle plus an n-gon both appear in the output.
    #[test]
    fn mixed_triangle_and_ngon_are_both_emitted() {
        let mut mesh = AiMesh {
            vertices: vec![
                [0.0, 0.0, 0.0].into(),
                [1.0, 0.0, 0.0].into(),
                [0.0, 1.0, 0.0].into(),
                [3.0, 0.0, 0.0].into(),
                [5.0, 0.0, 0.0].into(),
                [6.0, 1.0, 0.0].into(),
                [4.0, 2.0, 0.0].into(),
                [3.0, 1.0, 0.0].into(),
            ],
            faces: vec![vec![0, 1, 2], vec![3, 4, 5, 6, 7]],
            primitive_types: (AiPrimitiveType::Triangle | AiPrimitiveType::Polygon).into(),
            ..Default::default()
        };
        mesh.triangulate();
        assert_eq!(mesh.faces.len(), 4);
        assert_eq!(mesh.faces[0].len(), 3);
        assert_valid_triangulation(
            &AiMesh {
                vertices: mesh.vertices.clone(),
                faces: mesh.faces[1..].to_vec(),
                primitive_types: mesh.primitive_types,
                ..Default::default()
            },
            &[3, 4, 5, 6, 7],
        );
    }

    /// An already-triangle mesh keeps its faces even if `Polygon` is set.
    #[test]
    fn already_triangulated_polygon_flag_mesh_is_left_alone() {
        let mut mesh = AiMesh {
            vertices: vec![
                [0.0, 0.0, 0.0].into(),
                [1.0, 0.0, 0.0].into(),
                [0.0, 1.0, 0.0].into(),
            ],
            faces: vec![vec![0, 1, 2]],
            primitive_types: (AiPrimitiveType::Triangle | AiPrimitiveType::Polygon).into(),
            ..Default::default()
        };
        mesh.triangulate();
        assert_eq!(mesh.faces, vec![vec![0, 1, 2]]);
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Polygon));
    }
}
