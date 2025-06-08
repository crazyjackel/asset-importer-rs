use std::collections::{HashMap, HashSet, VecDeque};

use asset_importer_rs_scene::{AiReal, AiScene};
use gltf_v1::json::{Node, Root, StringIndex};

use crate::{
    GltfExporter,
    exporter::{error::Error, generate_unique_name},
};

impl GltfExporter {
    pub(crate) fn export_nodes(
        &self,
        scene: &AiScene,
        root: &mut Root,
        config_epsilon: f32,
    ) -> Result<HashMap<usize, String>, Error> {
        if scene.nodes.arena.is_empty() {
            return Ok(HashMap::new());
        }
        let mut unique_names_map: HashMap<String, u32> = HashMap::new();
        let mut mesh_index_map = HashMap::new();
        let mut queue: VecDeque<(usize, Option<String>)> = VecDeque::new();
        let mut checked_nodes: HashSet<usize> = HashSet::new();
        queue.push_back((scene.nodes.root.unwrap_or(0), None));
        while let Some((node_index, parent_name)) = queue.pop_front() {
            if checked_nodes.contains(&node_index) {
                continue;
            }
            checked_nodes.insert(node_index);
            let ai_node = &scene.nodes.arena[node_index];
            let mut node = Node::default();
            let base_name = if ai_node.name.is_empty() {
                "node"
            } else {
                &ai_node.name
            };
            node.name = Some(generate_unique_name(base_name, &mut unique_names_map));
            if let Some(parent_name) = parent_name {
                if let Some(parent) = root.nodes.get_mut(&parent_name) {
                    parent
                        .children
                        .push(StringIndex::new(node.name.clone().unwrap()));
                }
            }
            if !ai_node.transformation.is_identity(config_epsilon as AiReal) {
                node.matrix = Some(ai_node.transformation.clone().into());
            }
            for mesh_index in &ai_node.mesh_indexes {
                let ai_mesh = &scene.meshes[*mesh_index];
                let base_name = if ai_mesh.name.is_empty() {
                    "mesh"
                } else {
                    &ai_mesh.name
                };
                let unique_name = generate_unique_name(base_name, &mut unique_names_map);
                mesh_index_map.insert(*mesh_index, unique_name.clone());
                node.meshes.push(StringIndex::new(unique_name));
            }
            for child_index in &ai_node.children {
                queue.push_back((*child_index, node.name.clone()));
            }
            root.nodes.insert(node.name.clone().unwrap(), node);
        }
        Ok(mesh_index_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asset_importer_rs_scene::{AiMatrix4x4, AiMesh, AiNode, AiScene, AiVector3D};

    #[test]
    fn test_export_empty_scene() {
        let scene = AiScene::default();
        let mut root = Root::default();
        let exporter = GltfExporter::default();

        let result = exporter.export_nodes(&scene, &mut root, 0.0001);
        assert!(result.is_ok());
        let mesh_index_map = result.unwrap();
        assert!(mesh_index_map.is_empty());
        assert!(root.nodes.is_empty());
    }

    #[test]
    fn test_export_single_node() {
        let mut scene = AiScene::default();
        let node = AiNode {
            name: "test_node".to_string(),
            transformation: AiMatrix4x4::identity(),
            mesh_indexes: vec![],
            children: vec![],
            parent: None,
            metadata: Default::default(),
        };
        scene.nodes.arena.push(node);
        scene.nodes.root = Some(0);

        let mut root = Root::default();
        let exporter = GltfExporter::default();

        let result = exporter.export_nodes(&scene, &mut root, 0.0001);
        assert!(result.is_ok());
        let mesh_index_map = result.unwrap();
        assert!(mesh_index_map.is_empty());
        assert_eq!(root.nodes.len(), 1);
        assert!(root.nodes.contains_key("test_node"));
    }

    #[test]
    fn test_export_node_with_mesh() {
        let mut scene = AiScene::default();
        let mesh = AiMesh {
            name: "test_mesh".to_string(),
            vertices: vec![AiVector3D::new(0.0, 0.0, 0.0)],
            ..Default::default()
        };
        scene.meshes.push(mesh);

        let node = AiNode {
            name: "test_node".to_string(),
            transformation: AiMatrix4x4::identity(),
            mesh_indexes: vec![0],
            children: vec![],
            parent: None,
            metadata: Default::default(),
        };
        scene.nodes.arena.push(node);
        scene.nodes.root = Some(0);

        let mut root = Root::default();
        let exporter = GltfExporter::default();

        let result = exporter.export_nodes(&scene, &mut root, 0.0001);
        assert!(result.is_ok());
        let mesh_index_map = result.unwrap();
        assert_eq!(mesh_index_map.len(), 1);
        assert!(mesh_index_map.contains_key(&0));
        assert_eq!(root.nodes.len(), 1);

        let exported_node = root.nodes.get("test_node").unwrap();
        assert_eq!(exported_node.meshes.len(), 1);
    }

    #[test]
    fn test_export_node_hierarchy() {
        let mut scene = AiScene::default();

        // Create parent node
        let parent_node = AiNode {
            name: "parent".to_string(),
            transformation: AiMatrix4x4::identity(),
            mesh_indexes: vec![],
            children: vec![1], // Points to child node
            parent: None,
            metadata: Default::default(),
        };
        scene.nodes.arena.push(parent_node);

        // Create child node
        let child_node = AiNode {
            name: "child".to_string(),
            transformation: AiMatrix4x4::identity(),
            mesh_indexes: vec![],
            children: vec![],
            parent: Some(0),
            metadata: Default::default(),
        };
        scene.nodes.arena.push(child_node);

        scene.nodes.root = Some(0);

        let mut root = Root::default();
        let exporter = GltfExporter::default();

        let result = exporter.export_nodes(&scene, &mut root, 0.0001);
        assert!(result.is_ok());
        assert_eq!(root.nodes.len(), 2);

        let parent = root.nodes.get("parent").unwrap();
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].to_string(), "child");
    }

    #[test]
    fn test_export_node_with_transformation() {
        // test this ...

        let mut scene = AiScene::default();
        let mut transformation = AiMatrix4x4::identity();
        transformation.a4 = 1.0; // Set translation
        transformation.b4 = 2.0;
        transformation.c4 = 3.0;

        let node = AiNode {
            name: "test_node".to_string(),
            transformation,
            mesh_indexes: vec![],
            children: vec![],
            parent: None,
            metadata: Default::default(),
        };
        scene.nodes.arena.push(node);
        scene.nodes.root = Some(0);

        let mut root = Root::default();
        let exporter = GltfExporter::default();

        let result = exporter.export_nodes(&scene, &mut root, 0.0001);
        assert!(result.is_ok());
        assert_eq!(root.nodes.len(), 1);

        let exported_node = root.nodes.get("test_node").unwrap();
        assert!(exported_node.matrix.is_some());
        let matrix = exported_node.matrix.as_ref().unwrap();
        assert_eq!(matrix[3], 1.0); // Translation X
        assert_eq!(matrix[7], 2.0); // Translation Y
        assert_eq!(matrix[11], 3.0); // Translation Z
    }

    #[test]
    fn test_export_duplicate_names() {
        let mut scene = AiScene::default();

        // Create two nodes with the same name
        let node1 = AiNode {
            name: "duplicate".to_string(),
            transformation: AiMatrix4x4::identity(),
            mesh_indexes: vec![],
            children: vec![1],
            parent: None,
            metadata: Default::default(),
        };
        let node2 = AiNode {
            name: "duplicate".to_string(),
            transformation: AiMatrix4x4::identity(),
            mesh_indexes: vec![],
            children: vec![],
            parent: None,
            metadata: Default::default(),
        };
        scene.nodes.arena.push(node1);
        scene.nodes.arena.push(node2);
        scene.nodes.root = Some(0);

        let mut root = Root::default();
        let exporter = GltfExporter::default();

        let result = exporter.export_nodes(&scene, &mut root, 0.0001);
        assert!(result.is_ok());
        assert_eq!(root.nodes.len(), 2);

        // Check that the names are unique
        let names: Vec<_> = root.nodes.keys().collect();
        assert_ne!(names[0], names[1]);
        assert!(names[0].starts_with("duplicate"));
        assert!(names[1].starts_with("duplicate"));
    }
}
