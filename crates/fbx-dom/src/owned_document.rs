//! Eager-owned snapshot of a loaded FBX [`crate::Document`]: typed object rows, templates, globals,
//! and the full connection graph in FBX object-id space.
//!
//! Building an [`OwnedDocument`] from a live [`crate::Document`] (classification, remap tables) is
//! left to a materializer; this module defines the storage shape only.

use crate::Document;
use crate::object::OwnedObject;
use crate::objects::{
    AnimationCurve, AnimationCurveNode, AnimationLayer, AnimationStack, BlendShape,
    BlendShapeChannel, Camera, CameraSwitcher, Cluster, LayeredTexture, Light, LimbNode,
    LineGeometry, Material, MeshGeometry, Model, NullNode, OwnedGlobalSettings, ShapeGeometry,
    Skin, Texture, Video,
};

/// Fully owned FBX DOM view: header, definitions, globals, connection graph, and typed object rows.
#[derive(Debug, Default, PartialEq)]
pub struct OwnedDocument {
    pub fbx_version: u32,
    pub creator: String,
    pub creation_date: [u32; 7],
    pub global_settings: OwnedGlobalSettings,

    pub models: Vec<Model>,
    pub mesh_geometries: Vec<MeshGeometry>,
    pub line_geometries: Vec<LineGeometry>,
    pub shape_geometries: Vec<ShapeGeometry>,
    pub cameras: Vec<Camera>,
    pub camera_switchers: Vec<CameraSwitcher>,
    pub lights: Vec<Light>,
    pub null_nodes: Vec<NullNode>,
    pub limb_nodes: Vec<LimbNode>,
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>,
    pub layered_textures: Vec<LayeredTexture>,
    pub videos: Vec<Video>,
    pub clusters: Vec<Cluster>,
    pub skins: Vec<Skin>,
    pub blend_shapes: Vec<BlendShape>,
    pub blend_shape_channels: Vec<BlendShapeChannel>,
    pub animation_stacks: Vec<AnimationStack>,
    pub animation_layers: Vec<AnimationLayer>,
    pub animation_curves: Vec<AnimationCurve>,
    pub animation_curve_nodes: Vec<AnimationCurveNode>,
    pub unknown_objects: Vec<OwnedObject>,
}

impl From<Document> for OwnedDocument {
    fn from(document: Document) -> Self {
        let global_settings = document.global_settings().into();
        Self {
            fbx_version: document.fbx_version,
            creator: document.creator,
            creation_date: document.creation_date,
            global_settings,
            models: todo!(),
            mesh_geometries: todo!(),
            line_geometries: todo!(),
            shape_geometries: todo!(),
            cameras: todo!(),
            camera_switchers: todo!(),
            lights: todo!(),
            null_nodes: todo!(),
            limb_nodes: todo!(),
            materials: todo!(),
            textures: todo!(),
            layered_textures: todo!(),
            videos: todo!(),
            clusters: todo!(),
            skins: todo!(),
            blend_shapes: todo!(),
            blend_shape_channels: todo!(),
            animation_stacks: todo!(),
            animation_layers: todo!(),
            animation_curves: todo!(),
            animation_curve_nodes: todo!(),
            unknown_objects: todo!(),
        }
    }
}
