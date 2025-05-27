use gltf_v1_derive::Validate;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct AssetProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Asset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    #[serde(rename = "premultipliedAlpha")]
    pub premultiplied_alpha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<AssetProfile>,
    pub version: String,
}

#[test]
fn test_asset_deserialize() {
    let data = r#"{
        "copyright" : "(C) Copyright Khronos Group",
        "generator" : "collada2gltf@042d7d2a3782aaf6d86961d052fc53bea8b3e424",
        "premultipliedAlpha" : true,
        "profile" : {
            "api" : "WebGL",
            "version" : "1.0.3",
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }  
        },
        "version" : "1.0",
        "extensions" : {
           "extension_name" : {
              "extension specific" : "value"
           }
        },
        "extras" : {
            "Application specific" : "The extra object can contain any properties."
        }  
    }"#;
    let asset: Result<Asset, _> = serde_json::from_str(data);
    let asset_unwrap = asset.unwrap();
    println!("{}", serde_json::to_string(&asset_unwrap).unwrap());
    assert_eq!(
        Some("(C) Copyright Khronos Group".to_string()),
        asset_unwrap.copyright
    );
}
