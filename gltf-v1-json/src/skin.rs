use serde_derive::{Deserialize, Serialize};

use super::{accessor::Accessor, node::Node, root::StringIndex};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skin {
    #[serde(rename = "bindShapeMatrix", skip_serializing_if = "Option::is_none")]
    bind_shape_matrix: Option<[f32; 16]>,
    #[serde(rename = "inverseBindMatrices")]
    inverse_bind_matrices: StringIndex<Accessor>,
    #[serde(rename = "jointNames")]
    joint_names: Vec<StringIndex<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
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
