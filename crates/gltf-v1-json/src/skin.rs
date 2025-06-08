use gltf_v1_derive::Validate;
use serde_derive::{Deserialize, Serialize};

use super::{accessor::Accessor, common::StringIndex, node::Node};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Skin {
    #[serde(
        rename = "bindShapeMatrix",
        skip_serializing_if = "matrix_is_default",
        default = "default_matrix"
    )]
    pub bind_shape_matrix: [f32; 16],
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: StringIndex<Accessor>,
    #[serde(default)]
    #[serde(rename = "jointNames")]
    pub joint_names: Vec<StringIndex<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
fn test_skin_deserialize() {
    let data = r#"{
            "bindShapeMatrix": [
                1,
                0,
                0,
                0,
                0,
                1,
                0,
                0,
                0,
                0,
                1,
                0,
                0,
                0,
                0,
                1
            ],
            "inverseBindMatrices": "accessor_id",
            "jointNames": [
                "joint_name",
                "another_joint_name"
            ],
            "name": "user-defined skin name",
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let skin: Result<Skin, _> = serde_json::from_str(data);
    let skin_unwrap = skin.unwrap();
    println!("{}", serde_json::to_string(&skin_unwrap).unwrap());
    assert_eq!(Some("user-defined skin name".to_string()), skin_unwrap.name);
}
