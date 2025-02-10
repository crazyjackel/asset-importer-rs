use std::collections::BTreeMap;

use super::accessor::Accessor;
use super::animation::Animation;
use super::asset::Asset;
use super::buffer::{Buffer, BufferView};
use super::camera::Camera;
use super::image::Image;
use super::material::{Material, Technique};
use super::mesh::Mesh;
use super::node::Node;
use super::root::StringIndex;
use super::scene::Scene;
use super::shader::{Program, Shader};
use super::skin::Skin;
use super::texture::{Sampler, Texture};

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct GLTF {
    #[serde(skip_serializing_if = "Option::is_none")]
    accessors: Option<BTreeMap<String, Accessor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animations: Option<BTreeMap<String, Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffers: Option<BTreeMap<String, Buffer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_views: Option<BTreeMap<String, BufferView>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cameras: Option<BTreeMap<String, Camera>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<BTreeMap<String, Image>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    materials: Option<BTreeMap<String, Material>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meshes: Option<BTreeMap<String, Mesh>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<BTreeMap<String, Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    programs: Option<BTreeMap<String, Program>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samplers: Option<BTreeMap<String, Sampler>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene: Option<StringIndex<Scene>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scenes: Option<BTreeMap<String, Scene>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shaders: Option<BTreeMap<String, Shader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skins: Option<BTreeMap<String, Skin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    techniques: Option<BTreeMap<String, Technique>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    textures: Option<BTreeMap<String, Texture>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions_used: Option<Vec<String>>,
}

#[test]
fn test_gltf_deserialize() {
    let data = r#"{
    "asset" : {
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
    }
}"#;
    let gltf: GLTF = serde_json::from_str(data).unwrap();
    println!("{}", serde_json::to_string(&gltf).unwrap());
    assert_eq!(
        &Some("(C) Copyright Khronos Group".to_string()),
        &gltf.asset.unwrap().copyright
    );
}
