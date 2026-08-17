use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use asset_importer_rs_scene::{
    AI_MATH_PI, AiMatrix4x4, AiMesh, AiNode, AiNodeTree, AiReal, AiVector3D,
};
use dae_parser::{Document, LocalMap, Node, Transform, Url, VisualScene};

use crate::DaeImportError;

use super::DaeImporter;

impl DaeImporter {
    pub(crate) fn import_nodes(
        &self,
        document: &Document,
        visual_scene: &VisualScene,
        material_index_map: &HashMap<String, usize>,
    ) -> Result<(AiNodeTree, Vec<AiMesh>, HashMap<String, usize>), DaeImportError> {
        let node_map = document
            .local_map::<Node>()
            .map_err(DaeImportError::FileFormatError)?;
        let mesh_library = self.import_mesh_library(document)?;

        let mut tree = AiNodeTree::default();
        let mut scene_meshes = Vec::new();
        let mut scene_mesh_name_map = HashMap::new();
        let mut seen_node_names: HashMap<String, ()> = HashMap::new();
        let mut node_name_counter = 0u32;
        // Depth-first: pop from the back so the queue's memory usage stays small.
        let mut queue: VecDeque<(&Node, Option<usize>)> = VecDeque::new();

        match visual_scene.nodes.len().cmp(&1) {
            Ordering::Equal => {
                queue.push_back((&visual_scene.nodes[0], None));
            }
            Ordering::Greater => {
                let root = AiNode {
                    name: visual_scene
                        .name
                        .clone()
                        .or_else(|| visual_scene.id.clone())
                        .unwrap_or_else(|| "ROOT".to_string()),
                    transformation: AiMatrix4x4::identity(),
                    ..AiNode::default()
                };
                let root_index = tree
                    .insert(root, None)
                    .expect("visual scene root is the first node");
                for child in visual_scene.nodes.iter().rev() {
                    queue.push_back((child, Some(root_index)));
                }
            }
            Ordering::Less => {
                return Err(DaeImportError::MissingRootNode);
            }
        }

        while let Some((node, parent_index)) = queue.pop_back() {
            let name = find_name_for_node(node, self.use_collada_name, &mut node_name_counter);
            if seen_node_names.contains_key(&name) {
                continue;
            }
            seen_node_names.insert(name.clone(), ());

            let mut ai_node = AiNode {
                name,
                transformation: calculate_result_transform(&node.transforms),
                ..AiNode::default()
            };

            let (local_meshes, local_name_map) =
                self.build_meshes_for_node(document, node, &mesh_library, material_index_map)?;
            let offset = scene_meshes.len();
            ai_node.mesh_indexes = (offset..offset + local_meshes.len()).collect();
            for (name, index) in local_name_map {
                scene_mesh_name_map.insert(name, offset + index);
            }
            scene_meshes.extend(local_meshes);

            Self::build_cameras_for_node(document, node, &mut ai_node)?;
            Self::build_lights_for_node(document, node, &mut ai_node)?;

            let index = tree
                .insert(ai_node, parent_index)
                .expect("parent was already inserted into the tree");

            let instances = resolve_node_instances(node, &node_map, visual_scene);
            for instance in instances.into_iter().rev() {
                queue.push_back((instance, Some(index)));
            }
            for child in node.children.iter().rev() {
                queue.push_back((child, Some(index)));
            }
        }

        Ok((tree, scene_meshes, scene_mesh_name_map))
    }
}

fn find_name_for_node(node: &Node, use_collada_name: bool, node_name_counter: &mut u32) -> String {
    // Use Collada Name if available and we enabled usage
    if use_collada_name && let Some(name) = node.name.as_ref().filter(|name| !name.is_empty()) {
        return name.clone();
    }
    // Use ID if available
    if let Some(id) = node.id.as_ref().filter(|id| !id.is_empty()) {
        return id.clone();
    }
    // Use SID if available
    if let Some(sid) = node.sid.as_ref().filter(|sid| !sid.is_empty()) {
        return sid.clone();
    }
    // Fallback to auto-generated name
    let name = format!("$ColladaAutoName$_{}", *node_name_counter);
    *node_name_counter += 1;
    name
}

fn calculate_result_transform(transforms: &[Transform]) -> AiMatrix4x4 {
    let mut res = AiMatrix4x4::identity();
    for transform in transforms {
        match transform {
            Transform::LookAt(look_at) => {
                let pos = AiVector3D::from(look_at.eye().map(|v| v as AiReal));
                let dst_pos = AiVector3D::from(look_at.target().map(|v| v as AiReal));
                let up = AiVector3D::from(look_at.up().map(|v| v as AiReal)).norm();
                let dir = (dst_pos - pos).norm();
                let right = (dir ^ up).norm();
                res *= AiMatrix4x4::from([
                    right.x, up.x, -dir.x, pos.x, right.y, up.y, -dir.y, pos.y, right.z, up.z,
                    -dir.z, pos.z, 0.0, 0.0, 0.0, 1.0,
                ]);
            }
            Transform::Rotate(rotate) => {
                let angle = rotate.angle() as AiReal * AI_MATH_PI / 180.0;
                let axis = AiVector3D::from(rotate.axis().map(|v| v as AiReal));
                res *= AiMatrix4x4::rotation(angle, &axis);
            }
            Transform::Translate(translate) => {
                let translation = AiVector3D::from(translate.0.map(|v| v as AiReal));
                res *= AiMatrix4x4::translation(&translation);
            }
            Transform::Scale(scale) => {
                let [sx, sy, sz] = *scale.0;
                res *= AiMatrix4x4::from([
                    sx as AiReal,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    sy as AiReal,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    sz as AiReal,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ]);
            }
            Transform::Skew(_) => {
                // Assimp leaves <skew> unimplemented.
            }
            Transform::Matrix(matrix) => {
                let m = *matrix.0;
                res *= AiMatrix4x4::from(m.map(|value| value as AiReal));
            }
        }
    }
    res
}

// =============================================================================
// dae_parser's `LocalMap<Node>` only indexes nodes that have an `id`. Instance
// targets may be named without an id, or may live outside that map, so looking
// at `Vec<Node>` / the map alone is not enough. Resolve `instance_node` URLs
// ourselves: try the id map, then walk the visual scene by name or id.
// =============================================================================
fn resolve_node_instances<'a>(
    node: &'a Node,
    node_map: &LocalMap<'a, Node>,
    visual_scene: &'a VisualScene,
) -> Vec<&'a Node> {
    let mut resolved = Vec::with_capacity(node.instance_node.len());
    for instance in &node.instance_node {
        if let Some(instanced) = node_map.get(&instance.url) {
            resolved.push(instanced);
            continue;
        }
        let name = match &instance.url.val {
            Url::Fragment(fragment) => fragment.as_str(),
            Url::Other(other) => other.as_str(),
        };
        if let Some(instanced) = visual_scene
            .nodes
            .iter()
            .find_map(|node| find_node(node, name))
        {
            resolved.push(instanced);
        }
    }
    resolved
}

fn find_node<'a>(node: &'a Node, name: &str) -> Option<&'a Node> {
    if node.name.as_deref() == Some(name) || node.id.as_deref() == Some(name) {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_node(child, name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dae_parser::{LookAt, Matrix, Rotate, Scale, Translate};

    #[test]
    fn empty_transforms_are_identity() {
        assert!(calculate_result_transform(&[]).is_identity(1e-6));
    }

    #[test]
    fn translates_into_the_fourth_column() {
        let result = calculate_result_transform(&[Translate::new([1.0, 2.0, 3.0]).into()]);
        assert_eq!(result.a4, 1.0);
        assert_eq!(result.b4, 2.0);
        assert_eq!(result.c4, 3.0);
        assert_eq!(result.d4, 1.0);
    }

    #[test]
    fn scales_the_diagonal() {
        let result = calculate_result_transform(&[Scale::new([2.0, 3.0, 4.0]).into()]);
        assert_eq!(result.a1, 2.0);
        assert_eq!(result.b2, 3.0);
        assert_eq!(result.c3, 4.0);
        assert_eq!(result.d4, 1.0);
    }

    #[test]
    fn rotates_90_degrees_around_z() {
        let result = calculate_result_transform(&[Rotate::new([0.0, 0.0, 1.0], 90.0).into()]);
        assert!((result.a1 - 0.0).abs() < 1e-6);
        assert!((result.a2 - -1.0).abs() < 1e-6);
        assert!((result.b1 - 1.0).abs() < 1e-6);
        assert!((result.b2 - 0.0).abs() < 1e-6);
    }

    #[test]
    fn normalizes_scaled_rotation_axis() {
        let scaled = calculate_result_transform(&[Rotate::new([0.0, 2.0, 0.0], 90.0).into()]);
        let unit = calculate_result_transform(&[Rotate::new([0.0, 1.0, 0.0], 90.0).into()]);
        let scaled_array: [AiReal; 16] = scaled.into();
        let unit_array: [AiReal; 16] = unit.into();
        for (scaled_value, unit_value) in scaled_array.iter().zip(unit_array.iter()) {
            assert!((scaled_value - unit_value).abs() < 1e-6);
        }
    }

    #[test]
    fn copies_matrix_values_in_assimp_order() {
        let values = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        let result = calculate_result_transform(&[Matrix::new(values).into()]);
        let as_array: [AiReal; 16] = result.into();
        assert_eq!(as_array, values);
    }

    #[test]
    fn look_at_builds_right_up_neg_dir_basis() {
        let result = calculate_result_transform(&[LookAt::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, -1.0],
            [0.0, 1.0, 0.0],
        )
        .into()]);
        assert!((result.a1 - 1.0).abs() < 1e-6);
        assert!((result.b2 - 1.0).abs() < 1e-6);
        assert!((result.c3 - 1.0).abs() < 1e-6);
        assert_eq!(result.a4, 0.0);
        assert_eq!(result.b4, 0.0);
        assert_eq!(result.c4, 0.0);
    }

    #[test]
    fn composes_transforms_left_to_right() {
        let result = calculate_result_transform(&[
            Translate::new([1.0, 0.0, 0.0]).into(),
            Scale::new([2.0, 1.0, 1.0]).into(),
        ]);
        assert_eq!(result.a1, 2.0);
        assert_eq!(result.a4, 1.0);
    }
}
