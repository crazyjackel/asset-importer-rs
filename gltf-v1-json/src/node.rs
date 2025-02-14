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
    #[serde(skip_serializing_if = "matrix_is_default", default = "default_matrix")]
    pub matrix: [f32; 16],
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meshes: Vec<StringIndex<Mesh>>,
    #[serde(
        skip_serializing_if = "rotation_is_default",
        default = "default_rotation"
    )]
    pub rotation: [f32; 4],
    #[serde(skip_serializing_if = "scale_is_default", default = "default_scale")]
    pub scale: [f32; 3],
    #[serde(
        skip_serializing_if = "translation_is_default",
        default = "default_translation"
    )]
    pub translation: [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

fn translation_is_default(value: &[f32; 3]) -> bool {
    value[0] == 0.0 && value[1] == 0.0 && value[2] == 0.0
}

fn default_translation() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn scale_is_default(value: &[f32; 3]) -> bool {
    value[0] == 1.0 && value[1] == 1.0 && value[2] == 1.0
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn rotation_is_default(value: &[f32; 4]) -> bool {
    value[0] == 0.0 && value[1] == 0.0 && value[2] == 0.0 && value[3] == 1.0
}

fn default_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn matrix_is_default(value: &[f32; 16]) -> bool {
    value[0] == 1.0
        && value[1] == 0.0
        && value[2] == 0.0
        && value[3] == 0.0
        && value[4] == 0.0
        && value[5] == 1.0
        && value[6] == 0.0
        && value[7] == 0.0
        && value[8] == 0.0
        && value[9] == 0.0
        && value[10] == 1.0
        && value[11] == 0.0
        && value[12] == 0.0
        && value[13] == 0.0
        && value[14] == 0.0
        && value[15] == 1.0
}

fn default_matrix() -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
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
