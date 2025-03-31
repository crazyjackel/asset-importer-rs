pub mod gltf;
pub mod image;
pub mod light;
pub mod material;
pub mod node;

pub const ENABLED_EXTENSIONS: &[&str] = &[
    #[cfg(feature = "KHR_binary_glTF")]
    "KHR_binary_glTF",
    #[cfg(feature = "KHR_materials_common")]
    "KHR_materials_common",
];

pub const SUPPORTED_EXTENSIONS: &[&str] = &["KHR_binary_glTF", "KHR_materials_common"];

#[doc(inline)]
pub use light::Light;