use gltf_v1_derive::Validate;
use indexmap::IndexMap;

use crate::extensions;
use crate::validation::Validate;

use super::accessor::Accessor;
use super::animation::Animation;
use super::asset::Asset;
use super::buffer::{Buffer, BufferView};
use super::camera::Camera;
use super::common::StringIndex;
use super::image::Image;
use super::material::{Material, Technique};
use super::mesh::Mesh;
use super::node::Node;
use super::scene::Scene;
use super::shader::{Program, Shader};
use super::skin::Skin;
use super::texture::{Sampler, Texture};

pub trait Get<T> {
    fn get(&self, id: StringIndex<T>) -> Option<&T>;
}

pub struct UniqueKeyGenerator {
    prefix: String,
    counter: usize,
}

impl UniqueKeyGenerator {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            counter: 0,
        }
    }

    fn next_key<T>(&mut self, map: &IndexMap<String, T>) -> String {
        loop {
            let key = format!("{}{}", self.prefix, self.counter);
            self.counter += 1;
            if !map.contains_key(&key) {
                return key;
            }
        }
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct Root {
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub accessors: IndexMap<String, Accessor>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub animations: IndexMap<String, Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Asset>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub buffers: IndexMap<String, Buffer>,
    #[serde(default)]
    #[serde(rename = "bufferViews")]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub buffer_views: IndexMap<String, BufferView>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub cameras: IndexMap<String, Camera>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub images: IndexMap<String, Image>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub materials: IndexMap<String, Material>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub meshes: IndexMap<String, Mesh>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub nodes: IndexMap<String, Node>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub programs: IndexMap<String, Program>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub samplers: IndexMap<String, Sampler>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<StringIndex<Scene>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub scenes: IndexMap<String, Scene>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub shaders: IndexMap<String, Shader>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub skins: IndexMap<String, Skin>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub techniques: IndexMap<String, Technique>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub textures: IndexMap<String, Texture>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extensions_used: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::gltf::Root>,
}

impl Root {
    pub fn add_default_material(&mut self) {
        // Give all Materials with no Technique a Default Technique as the Default Material
        let materials_with_no_technique: Vec<(&String, &mut Material)> = self
            .materials
            .iter_mut()
            .filter(|x| x.1.technique == None)
            .collect();
        if materials_with_no_technique.is_empty() {
            return;
        }

        let mut generator = UniqueKeyGenerator::new("vertexShader");
        let vertex_shader_key = generator.next_key(&self.shaders);
        self.shaders
            .insert(vertex_shader_key.clone(), Shader::default_vertex_shader());
        let mut generator = UniqueKeyGenerator::new("fragmentShader");
        let fragment_shader_key = generator.next_key(&self.shaders);
        self.shaders.insert(
            fragment_shader_key.clone(),
            Shader::default_fragment_shader(),
        );
        let mut generator = UniqueKeyGenerator::new("program");
        let program_key = generator.next_key(&self.programs);
        self.programs.insert(
            program_key.clone(),
            Program::default_program(fragment_shader_key, vertex_shader_key),
        );
        let mut generator = UniqueKeyGenerator::new("technique");
        let technique_key = generator.next_key(&self.techniques);
        self.techniques.insert(
            technique_key.clone(),
            Technique::default_technique(program_key),
        );

        for (_, material) in materials_with_no_technique {
            material.technique = Some(StringIndex::new(technique_key.clone()))
        }
    }
}

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<'a> Get<$ty> for Root {
            fn get(&self, index: StringIndex<$ty>) -> Option<&$ty> {
                self.$field.get(index.value())
            }
        }
    };
}

impl_get!(Accessor, accessors);
impl_get!(Animation, animations);
impl_get!(Buffer, buffers);
impl_get!(BufferView, buffer_views);
impl_get!(Camera, cameras);
impl_get!(Image, images);
impl_get!(Material, materials);
impl_get!(Mesh, meshes);
impl_get!(Node, nodes);
impl_get!(Program, programs);
impl_get!(Sampler, samplers);
impl_get!(Scene, scenes);
impl_get!(Shader, shaders);
impl_get!(Skin, skins);
impl_get!(Texture, textures);
impl_get!(Technique, techniques);

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
    let gltf: Root = serde_json::from_str(data).unwrap();
    println!("{}", serde_json::to_string(&gltf).unwrap());
    assert_eq!(
        &Some("(C) Copyright Khronos Group".to_string()),
        &gltf.asset.unwrap().copyright
    );
}
