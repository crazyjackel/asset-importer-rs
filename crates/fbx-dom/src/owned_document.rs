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
    LineGeometry, Material, MeshGeometry, Model, NullNode, OwnedGlobalSettings, ShapeGeometry, Skin,
    Texture, Video,
};
use crate::objects::ClassifiedFbxObject;

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
    pub unknown_geometries: Vec<OwnedObject>,

    pub cameras: Vec<Camera>,
    pub camera_switchers: Vec<CameraSwitcher>,
    pub lights: Vec<Light>,
    pub null_nodes: Vec<NullNode>,
    pub limb_nodes: Vec<LimbNode>,
    pub unknown_node_attributes: Vec<OwnedObject>,
    
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>,
    pub layered_textures: Vec<LayeredTexture>,
    pub videos: Vec<Video>,
    pub clusters: Vec<Cluster>,
    pub skins: Vec<Skin>,
    pub blend_shapes: Vec<BlendShape>,
    pub blend_shape_channels: Vec<BlendShapeChannel>,
    pub unknown_deformers: Vec<OwnedObject>,
    pub animation_stacks: Vec<AnimationStack>,
    pub animation_layers: Vec<AnimationLayer>,
    pub animation_curves: Vec<AnimationCurve>,
    pub animation_curve_nodes: Vec<AnimationCurveNode>,
    pub unknown_objects: Vec<OwnedObject>,
}

impl From<Document> for OwnedDocument {
    fn from(document: Document) -> Self {
        let mut models = Vec::new();
        let mut mesh_geometries = Vec::new();
        let mut line_geometries = Vec::new();
        let mut shape_geometries = Vec::new();
        let mut unknown_geometries = Vec::new();
        let mut cameras = Vec::new();
        let mut camera_switchers = Vec::new();
        let mut lights = Vec::new();
        let mut null_nodes = Vec::new();
        let mut limb_nodes = Vec::new();
        let mut unknown_node_attributes = Vec::new();
        let mut materials = Vec::new();
        let mut textures = Vec::new();
        let mut layered_textures = Vec::new();
        let mut videos = Vec::new();
        let mut clusters = Vec::new();
        let mut skins = Vec::new();
        let mut blend_shapes = Vec::new();
        let mut blend_shape_channels = Vec::new();
        let mut unknown_deformers = Vec::new();
        let mut animation_stacks = Vec::new();
        let mut animation_layers = Vec::new();
        let mut animation_curves = Vec::new();
        let mut animation_curve_nodes = Vec::new();
        let mut unknown_objects = Vec::new();

        for object in document.objects().flatten() {
            let owned: OwnedObject = object.into();
            let classified = match ClassifiedFbxObject::try_from(owned) {
                Ok(v) => v,
                Err(e) => {
                    unknown_objects.push(e.object);
                    continue;
                }
            };
            match classified {
                ClassifiedFbxObject::Model(v) => models.push(v),
                ClassifiedFbxObject::MeshGeometry(v) => mesh_geometries.push(v),
                ClassifiedFbxObject::LineGeometry(v) => line_geometries.push(v),
                ClassifiedFbxObject::ShapeGeometry(v) => shape_geometries.push(v),
                ClassifiedFbxObject::UnknownGeometry(v) => unknown_geometries.push(v),
                ClassifiedFbxObject::Camera(v) => cameras.push(v),
                ClassifiedFbxObject::CameraSwitcher(v) => camera_switchers.push(v),
                ClassifiedFbxObject::Light(v) => lights.push(v),
                ClassifiedFbxObject::NullNode(v) => null_nodes.push(v),
                ClassifiedFbxObject::LimbNode(v) => limb_nodes.push(v),
                ClassifiedFbxObject::UnknownNodeAttribute(v) => unknown_node_attributes.push(v),
                ClassifiedFbxObject::Material(v) => materials.push(v),
                ClassifiedFbxObject::Texture(v) => textures.push(v),
                ClassifiedFbxObject::LayeredTexture(v) => layered_textures.push(v),
                ClassifiedFbxObject::Video(v) => videos.push(v),
                ClassifiedFbxObject::Cluster(v) => clusters.push(v),
                ClassifiedFbxObject::Skin(v) => skins.push(v),
                ClassifiedFbxObject::BlendShape(v) => blend_shapes.push(v),
                ClassifiedFbxObject::BlendShapeChannel(v) => blend_shape_channels.push(v),
                ClassifiedFbxObject::UnknownDeformer(v) => unknown_deformers.push(v),
                ClassifiedFbxObject::AnimationStack(v) => animation_stacks.push(v),
                ClassifiedFbxObject::AnimationLayer(v) => animation_layers.push(v),
                ClassifiedFbxObject::AnimationCurve(v) => animation_curves.push(v),
                ClassifiedFbxObject::AnimationCurveNode(v) => animation_curve_nodes.push(v),
                ClassifiedFbxObject::Unknown(v) => unknown_objects.push(v),
            }
        }

        let global_settings = document.global_settings().into();
        Self {
            fbx_version: document.fbx_version,
            creator: document.creator,
            creation_date: document.creation_date,
            global_settings,
            models,
            mesh_geometries,
            line_geometries,
            shape_geometries,
            unknown_geometries,
            cameras,
            camera_switchers,
            lights,
            null_nodes,
            limb_nodes,
            unknown_node_attributes,
            materials,
            textures,
            layered_textures,
            videos,
            clusters,
            skins,
            blend_shapes,
            blend_shape_channels,
            unknown_deformers,
            animation_stacks,
            animation_layers,
            animation_curves,
            animation_curve_nodes,
            unknown_objects,
        }
    }
}
