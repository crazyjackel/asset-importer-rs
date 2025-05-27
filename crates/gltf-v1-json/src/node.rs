use gltf_v1_derive::Validate;
use serde_derive::{Deserialize, Serialize};

use super::{camera::Camera, common::StringIndex, mesh::Mesh, skin::Skin};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<StringIndex<Camera>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<StringIndex<Node>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skeletons: Vec<StringIndex<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<StringIndex<Skin>>,
    #[serde(rename = "jointName", skip_serializing_if = "Option::is_none")]
    pub joint_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f32; 16]>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meshes: Vec<StringIndex<Mesh>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
