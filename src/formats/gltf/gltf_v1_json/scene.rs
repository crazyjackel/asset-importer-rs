use serde_derive::{Deserialize, Serialize};

use super::node::Node;
use super::root::StringIndex;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scene {
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<StringIndex<Node>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
}

#[test]
fn test_scene_deserialize() {
    let data = r#"{
            "name": "user-defined scene name",
            "nodes": [
                "mesh_node_id",
                "camera_node_id"
            ],
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }
        }"#;
    let scene: Result<Scene, _> = serde_json::from_str(data);
    let scene_unwrap = scene.unwrap();
    println!("{}", serde_json::to_string(&scene_unwrap).unwrap());
    assert_eq!(
        Some("user-defined scene name".to_string()),
        scene_unwrap.name
    );
}
