mod accessor;
mod animation;
mod asset;
mod buffer;
mod camera;
mod gltf;
mod image;
mod material;
mod mesh;
mod node;
mod root;
mod scene;
mod shader;
mod skin;
mod texture;
mod validation;

#[doc(inline)]
pub use accessor::Accessor;
#[doc(inline)]
pub use animation::Animation;
#[doc(inline)]
pub use asset::Asset;
#[doc(inline)]
pub use buffer::Buffer;
#[doc(inline)]
pub use camera::Camera;
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use material::Material;
#[doc(inline)]
pub use mesh::Mesh;
#[doc(inline)]
pub use node::Node;
#[doc(inline)]
pub use scene::Scene;
#[doc(inline)]
pub use skin::Skin;
#[doc(inline)]
pub use texture::Texture;

#[doc(inline)]
pub use self::gltf::GLTF;

pub mod deserialize {
    pub use serde_json::{from_reader, from_slice, from_str, from_value};
}

pub mod serialize {
    pub use serde_json::{
        to_string, to_string_pretty, to_value, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
    };
}