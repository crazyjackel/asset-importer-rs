pub mod accessor;
pub mod animation;
pub mod asset;
pub mod buffer;
pub mod camera;
pub mod common;
pub mod gltf;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod path;
pub mod scene;
pub mod shader;
pub mod skin;
pub mod texture;
pub mod validation;

#[doc(inline)]
pub use accessor::Accessor;
#[doc(inline)]
pub use animation::Animation;
#[doc(inline)]
pub use asset::Asset;
#[doc(inline)]
pub use buffer::Buffer;
#[doc(inline)]
pub use buffer::BufferView;
#[doc(inline)]
pub use camera::Camera;
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use material::Material;
#[doc(inline)]
pub use material::Technique;
#[doc(inline)]
pub use mesh::Mesh;
#[doc(inline)]
pub use node::Node;
#[doc(inline)]
pub use scene::Scene;
#[doc(inline)]
pub use shader::Program;
#[doc(inline)]
pub use shader::Shader;
#[doc(inline)]
pub use skin::Skin;
#[doc(inline)]
pub use texture::Sampler;
#[doc(inline)]
pub use texture::Texture;

#[doc(inline)]
pub use self::common::StringIndex;
#[doc(inline)]
pub use self::gltf::Root;
#[doc(inline)]
pub use self::path::Path;
#[doc(inline)]
pub use self::validation::Error as ValidationError;

#[doc(inline)]
pub use serde_json::Error;
#[doc(inline)]
pub use serde_json::Value;

pub mod deserialize {
    pub use serde_json::{from_reader, from_slice, from_str, from_value};
}

pub mod serialize {
    pub use serde_json::{
        to_string, to_string_pretty, to_value, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
    };
}
pub extern crate indexmap as map;
