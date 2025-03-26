pub extern crate gltf_v1_json as json;

extern crate image as image_crate;

pub mod accessor;
pub mod binary;
pub mod buffer;
pub mod camera;
pub mod document;
pub mod error;
pub mod gltf;
pub mod image;
mod import;
pub mod material;
mod math;
pub mod mesh;
pub mod node;
pub mod scene;
pub mod skin;
pub mod texture;

#[doc(inline)]
pub use self::import::import_buffers;
#[doc(inline)]
pub use self::import::import_images;

#[doc(inline)]
pub use self::accessor::Accessor;
#[doc(inline)]
pub use self::binary::Glb;
#[doc(inline)]
pub use self::buffer::Buffer;
#[doc(inline)]
pub use self::node::Node;
#[doc(inline)]
pub use error::Error as GLTF_Error;

#[doc(inline)]
pub use document::Document;
#[doc(inline)]
pub use gltf::Gltf;
