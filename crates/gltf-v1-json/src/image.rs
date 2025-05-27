use gltf_v1_derive::Validate;
use serde_derive::{Deserialize, Serialize};

use crate::extensions;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Image {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::image::Image>,
}

#[test]
fn test_image_deserialize() {
    let data = r#"{
            "name": "user-defined image name",
            "uri" : "image.png",
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }
        }"#;
    let image: Result<Image, _> = serde_json::from_str(data);
    let image_unwrap = image.unwrap();
    println!("{}", serde_json::to_string(&image_unwrap).unwrap());
    assert_eq!(
        Some("user-defined image name".to_string()),
        image_unwrap.name
    );
}
