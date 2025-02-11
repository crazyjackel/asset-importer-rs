use serde_derive::{Deserialize, Serialize};

use super::{camera::Camera, mesh::Mesh, root::StringIndex, skin::Skin};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    camera: Option<StringIndex<Camera>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<StringIndex<Node>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skeletons: Option<Vec<StringIndex<Node>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skin: Option<StringIndex<Skin>>,
    #[serde(rename = "jointName", skip_serializing_if = "Option::is_none")]
    joint_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matrix: Option<[f32; 16]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meshes: Option<Vec<StringIndex<Mesh>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<[f32; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    translation: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[test]
fn test_node_deserialize() {
    let data = r#"{
            "children": [],
            "matrix": [ 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0 ],
            "meshes": [
                "mesh_id"
            ],
            "name": "user-defined name of meshes node",
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let node: Result<Node, _> = serde_json::from_str(data);
    let node_unwrap = node.unwrap();
    println!("{}", serde_json::to_string(&node_unwrap).unwrap());
    assert_eq!(
        Some("user-defined name of meshes node".to_string()),
        node_unwrap.name
    );
}
