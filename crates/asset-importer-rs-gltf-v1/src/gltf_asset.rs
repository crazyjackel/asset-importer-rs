use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{Error, Read},
};

use animation::Animation;
use buffer::Buffer;
use camera::Camera;
use light::Light;
use material::Material;
use mesh::Mesh;
use serde_json::Value;

use super::common::{MAT4, VEC3, VEC4};

#[derive(Debug)]
struct GLBHeader {
    magic: [u8; 4],
    version: u32,
    length: u32,
    scene_length: u32,
    scene_format: u32,
}

#[derive(Debug)]
enum PrimitiveMode {
    Points,
    Lines,
    Line_Loop,
    Line_Strip,
    Triangles,
    Triangle_Strip,
    Triangle_Fan,
}

#[derive(Debug)]
enum ComponentType {
    Byte = 5120,
    Unsigned_Byte = 5121,
    SHORT = 5122,
    UNSIGNED_SHORT = 5123,
    UNSIGNED_INT = 5125,
    FLOAT = 5126,
}

impl ComponentType {
    const fn size(&self) -> u32 {
        match self {
            ComponentType::Byte | ComponentType::Unsigned_Byte => 1,
            ComponentType::SHORT | ComponentType::UNSIGNED_SHORT => 2,
            ComponentType::UNSIGNED_INT | ComponentType::FLOAT => 4,
        }
    }
}

#[derive(Debug)]
enum BufferViewTarget {
    None = 0,
    ArrayBuffer = 32962,
    ElementArrayBuffer = 34963,
}

#[derive(Debug)]
enum SamplerMagFilter {
    Nearest = 9728,
    Linear = 9729,
}

#[derive(Debug)]
enum SamplerMinFilter {
    Nearest = 9728,
    Linear = 9729,
    NearestMipmapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
}

#[derive(Debug)]
enum SamplerWrap {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    Repeat = 10497,
}

#[derive(Debug)]
enum TextureFormat {
    ALPHA = 6406,
    RGB = 6407,
    RGBA = 6408,
    LUMINANCE = 6409,
    LuminanceAlpha = 6410,
}

#[derive(Debug)]
enum TextureTarget {
    Texture2d = 3553,
}

#[derive(Debug)]
enum TextureType {
    UnsignedByte = 5121,
    UnsignedShort5_6_5 = 33635,
    UnsignedShort4_4_4_4 = 32819,
    UnsignedShort5_5_5_1 = 32820,
}

pub mod attrib_type {
    #[derive(Debug)]
    pub enum Value {
        SCALAR,
        VEC2,
        VEC3,
        VEC4,
        MAT2,
        MAT3,
        MAT4,
    }

    impl Value {
        pub const fn get_num_components(&self) -> u32 {
            match self {
                Value::SCALAR => 1,
                Value::VEC2 => 2,
                Value::VEC3 => 3,
                Value::VEC4 => 4,
                Value::MAT2 => 4,
                Value::MAT3 => 9,
                Value::MAT4 => 16,
            }
        }
    }
    pub fn from_str(str: &str) -> Value {
        match str {
            "SCALAR" => Value::SCALAR,
            "VEC2" => Value::VEC2,
            "VEC3" => Value::VEC3,
            "VEC4" => Value::VEC4,
            "MAT2" => Value::MAT2,
            "MAT3" => Value::MAT3,
            "MAT4" => Value::MAT4,
            _ => Value::SCALAR,
        }
    }
}

#[derive(Debug)]
struct Accessor<'a> {
    id: String,
    name: String,
    buffer_view: &'a BufferView<'a>,
    byte_offset: u32,
    byte_stride: u32,
    component_type: ComponentType,
    count: u32,
    attribute_type: attrib_type::Value,
    max: Vec<f64>,
    min: Vec<f64>,
}

impl<'a> Default for Accessor<'a> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            buffer_view: Default::default(),
            byte_offset: Default::default(),
            byte_stride: Default::default(),
            component_type: Default::default(),
            count: Default::default(),
            attribute_type: Default::default(),
            max: Default::default(),
            min: Default::default(),
        }
    }
}

pub mod buffer {
    #[derive(Debug)]
    pub enum BufferType {
        ArrayBuffer,
        Text,
    }

    #[derive(Debug)]
    pub struct SEncodedRegion<'a> {
        offset: usize,
        length: usize,
        decoded_data: &'a [u8],
        decoded_length: usize,
        id: String,
    }
    #[derive(Debug)]
    pub struct Buffer<'a> {
        pub length: usize,
        pub buffer_type: BufferType,
        pub current_region: &'a SEncodedRegion<'a>,
        data: &'a [u8],
        is_special: bool,
        capacity: usize,
        regions: Vec<&'a SEncodedRegion<'a>>,
    }

    impl<'a> Default for Buffer<'a> {
        fn default() -> Self {
            Self {
                length: Default::default(),
                buffer_type: Default::default(),
                current_region: Default::default(),
                data: Default::default(),
                is_special: Default::default(),
                capacity: Default::default(),
                regions: Default::default(),
            }
        }
    }
}

#[derive(Debug)]
struct BufferView<'a> {
    id: String,
    name: String,
    buffer: &'a buffer::Buffer<'a>,
    offset: usize,
    length: usize,
    target: BufferViewTarget,
}

pub mod camera {
    #[derive(Debug)]
    pub struct Perspective {
        aspect_ratio: f32,
        y_fov: f32,
        z_far: f32,
        z_near: f32,
    }

    #[derive(Debug)]
    pub struct Ortographic {
        x_mag: f32,
        y_mag: f32,
        z_far: f32,
        z_near: f32,
    }

    #[derive(Debug)]
    pub enum CameraType {
        Perspective(Perspective),
        Orthographic(Ortographic),
    }
    #[derive(Debug)]
    pub struct Camera {
        id: String,
        name: String,
        camera_type: CameraType,
    }
}

#[derive(Debug)]
struct Image<'a> {
    pub uri: String,
    pub buffer_view: &'a BufferView<'a>,
    pub mime_type: String,
    pub width: i32,
    pub height: i32,
    data: Vec<u8>,
}

#[derive(Debug)]
struct TexProperty<'a> {
    texture: &'a Texture<'a>,
    color: VEC4,
}

mod material {
    use super::TexProperty;

    #[derive(Debug)]
    enum Technique {
        Undefined,
        Blinn,
        Phong,
        Lambert,
        Constant,
    }
    #[derive(Debug)]
    pub struct Material<'a> {
        id: String,
        name: String,
        ambient: TexProperty<'a>,
        diffuse: TexProperty<'a>,
        specular: TexProperty<'a>,
        emission: TexProperty<'a>,
        doubled_side: bool,
        transparent: bool,
        transparency: f32,
        shininess: f32,
        technique: Technique,
    }
}

mod mesh {
    use super::{Accessor, PrimitiveMode, material::Material};

    type AccessorList<'a> = Vec<&'a Accessor<'a>>;

    #[derive(Debug)]
    struct PrimitiveAttributes<'a> {
        position: AccessorList<'a>,
        normal: AccessorList<'a>,
        texcoord: AccessorList<'a>,
        color: AccessorList<'a>,
        joint: AccessorList<'a>,
        joint_matrix: AccessorList<'a>,
        weight: AccessorList<'a>,
    }

    #[derive(Debug)]
    struct Primitive<'a> {
        mode: PrimitiveMode,
        attributes: PrimitiveAttributes<'a>,
        indices: &'a Accessor<'a>,
        material: &'a Material<'a>,
    }

    #[derive(Debug)]
    enum SExtension {
        Unknown,
    }
    #[derive(Debug)]
    pub struct Mesh<'a> {
        id: String,
        name: String,
        primitives: Vec<Primitive<'a>>,
        extensions: Vec<SExtension>,
    }
}

#[derive(Debug)]
struct Node<'a> {
    id: String,
    name: String,
    children: Vec<&'a Node<'a>>,
    meshes: Vec<&'a mesh::Mesh<'a>>,
    matrix: Option<MAT4>,
    translation: Option<VEC3>,
    rotation: Option<VEC4>,
    scale: Option<VEC3>,
    camera: &'a Camera,
    light: &'a Light,
    skeletons: Vec<&'a Node<'a>>,
    skin: &'a Skin<'a>,
    joint_name: String,
    parent: &'a Node<'a>,
}

#[derive(Debug)]
struct Program {
    id: String,
    name: String,
}

#[derive(Debug)]
struct Sampler {
    id: String,
    name: String,
    mag_filter: SamplerMagFilter,
    min_filter: SamplerMinFilter,
    wrap_s: SamplerWrap,
    wrap_t: SamplerWrap,
}

#[derive(Debug)]
struct Scene<'a> {
    nodes: Vec<&'a Node<'a>>,
}

#[derive(Debug)]
struct Shader {
    id: String,
    name: String,
}

#[derive(Debug)]
struct Skin<'a> {
    id: String,
    name: String,
    bind_shape_matrix: Option<MAT4>,
    inverse_bind_matrices: &'a Accessor<'a>,
    joint_names: Vec<&'a Node<'a>>,
}

mod technique {
    #[derive(Debug)]
    struct Parameters;
    #[derive(Debug)]
    struct States;
    #[derive(Debug)]
    struct Functions;
    #[derive(Debug)]
    struct Technique {
        id: String,
        name: String,
    }
}

#[derive(Debug)]
struct Texture<'a> {
    id: String,
    name: String,
    sampler: &'a Sampler,
    source: &'a Image<'a>,
}

mod light {
    use crate::formats::gltf::common::VEC4;

    #[derive(Debug)]
    pub enum LightType {
        Undefined,
        Ambient,
        Directional,
        Point,
        Spot,
    }
    #[derive(Debug)]
    pub struct Light {
        id: String,
        name: String,
        light_type: LightType,
        color: VEC4,
        distance: f32,
        constant_attenuation: f32,
        linear_attenuation: f32,
        quadratic_attenuation: f32,
        fall_off_angle: f32,
        fall_off_exponent: f32,
    }
}

mod animation {
    use super::{Accessor, Node};

    #[derive(Debug)]
    struct AnimSampler {
        id: String,
        input: String,
        interpolation: String,
        output: String,
    }

    #[derive(Debug)]
    struct AnimTarget<'a> {
        id: &'a Node<'a>,
        path: String,
    }

    #[derive(Debug)]
    struct AnimChannel<'a> {
        sampler: String,
        target: AnimTarget<'a>,
    }

    #[derive(Debug)]
    struct AnimParameters<'a> {
        time: &'a Accessor<'a>,
        rotation: &'a Accessor<'a>,
        scale: &'a Accessor<'a>,
        translation: &'a Accessor<'a>,
    }
    #[derive(Debug)]
    pub struct Animation<'a> {
        id: String,
        name: String,
        channels: Vec<AnimChannel<'a>>,
        parameters: AnimParameters<'a>,
        samplers: Vec<AnimSampler>,
    }
}

trait LazyDictBase<'a>: Debug + Sized {}

#[derive(Debug)]
struct LazyDict<'a, T> {
    id: String,
    extension_id: String,
    objs: Vec<&'a T>,
    id_index_map: HashMap<String, usize>,
    json: &'a serde_json::Value,
    asset: &'a mut Asset<'a>,
}

impl<'a, T: Debug> LazyDictBase<'a> for LazyDict<'a, T> {}

#[derive(Debug)]
struct AssetMetadataProfile {
    api: String,
    version: String,
}

#[derive(Debug)]
struct AssetMetadata {
    copyright: String,
    generator: String,
    premultiplied_alpha: bool,
    profile: AssetMetadataProfile,
    version: String,
}

#[derive(Debug)]
struct AssetExtensions {
    khf_binary_gltf: bool,
    khf_materials_common: bool,
}
#[derive(Debug)]
struct Asset<'a> {
    current_asset_dir: String,
    scene_length: usize,
    body_offset: usize,
    body_length: usize,
    used_ids: HashMap<String, usize>,
    body_buffer: &'a Buffer<'a>,
    extensions_used: AssetExtensions,
    asset_metadata: AssetMetadata,
    accessors: LazyDict<'a, Accessor<'a>>,
    animations: LazyDict<'a, Animation<'a>>,
    buffers: LazyDict<'a, Buffer<'a>>,
    buffer_views: LazyDict<'a, BufferView<'a>>,
    cameras: LazyDict<'a, Camera>,
    images: LazyDict<'a, Image<'a>>,
    materials: LazyDict<'a, Material<'a>>,
    meshes: LazyDict<'a, Mesh<'a>>,
    nodes: LazyDict<'a, Node<'a>>,
    samplers: LazyDict<'a, Sampler>,
    scenes: LazyDict<'a, Scene<'a>>,
    skins: LazyDict<'a, Skin<'a>>,
    textures: LazyDict<'a, Texture<'a>>,
    scene: &'a Scene<'a>,
}

impl<'a> Default for Asset<'a> {
    fn default() -> Self {
        Self {
            current_asset_dir: Default::default(),
            scene_length: Default::default(),
            body_offset: Default::default(),
            body_length: Default::default(),
            used_ids: Default::default(),
            body_buffer: Default::default(),
            extensions_used: Default::default(),
            asset_metadata: Default::default(),
            accessors: Default::default(),
            animations: Default::default(),
            buffers: Default::default(),
            buffer_views: Default::default(),
            cameras: Default::default(),
            images: Default::default(),
            materials: Default::default(),
            meshes: Default::default(),
            nodes: Default::default(),
            samplers: Default::default(),
            scenes: Default::default(),
            skins: Default::default(),
            textures: Default::default(),
            scene: Default::default(),
        }
    }
}

impl<'a> Asset<'a> {
    pub fn load_asset(filepath: &str, is_binary: bool) -> Result<Self, Error> {
        let file = File::open(filepath)?;
        let (scene_length, body_length): (u64, u64) = if is_binary {
            (0, 0)
        } else {
            (file.metadata()?.len(), 0)
        };

        Ok(Default::default())
    }

    fn read_accessor(&mut self, obj: &Value) {}
}
