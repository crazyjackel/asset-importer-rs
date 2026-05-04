//! FBX `Model` objects — Assimp [`Model`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXModel.cpp) / [`FBXDocument.h`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedDocument, OwnedObject, Property};

use super::{
    AttrExtractorExt, FbxObjectTag, FbxTypeMismatch, Material, ModelGeometryRef, NodeAttributeRef,
    fbx_object_tag,
};

const ATTR_SHADING: &str = "Shading";
const ATTR_CULLING: &str = "Culling";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelRotationOrder {
    EulerXYZ = 0,
    EulerXZY = 1,
    EulerYZX = 2,
    EulerYXZ = 3,
    EulerZXY = 4,
    EulerZYX = 5,
    SphericXYZ = 6,
}

impl ModelRotationOrder {
    fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::EulerXZY,
            2 => Self::EulerYZX,
            3 => Self::EulerYXZ,
            4 => Self::EulerZXY,
            5 => Self::EulerZYX,
            6 => Self::SphericXYZ,
            _ => Self::EulerXYZ,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTransformInheritance {
    RrSs = 0,
    RSrs = 1,
    Rrs = 2,
}

impl ModelTransformInheritance {
    fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::RSrs,
            2 => Self::Rrs,
            _ => Self::RrSs,
        }
    }
}

/// Typed wrapper for a scene graph model / transform node (`Model::*` except unsupported effectors).
#[derive(Debug, PartialEq)]
pub struct Model {
    object: OwnedObject,
    pub shading: String,
    pub culling: String,
}

impl Model {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }

    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.object.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.object.properties.get(name)
    }

    pub fn shading(&self) -> &str {
        &self.shading
    }

    pub fn culling(&self) -> &str {
        &self.culling
    }

    pub fn quaternion_interpolate(&self) -> i32 {
        match self.property("QuaternionInterpolate") {
            Some(Property::Int(v)) => *v,
            _ => 0,
        }
    }
    pub fn rotation_offset(&self) -> [f32; 3] {
        match self.property("RotationOffset") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn rotation_pivot(&self) -> [f32; 3] {
        match self.property("RotationPivot") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn scaling_offset(&self) -> [f32; 3] {
        match self.property("ScalingOffset") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn scaling_pivot(&self) -> [f32; 3] {
        match self.property("ScalingPivot") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn translation_active(&self) -> bool {
        match self.property("TranslationActive") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn translation_min(&self) -> [f32; 3] {
        match self.property("TranslationMin") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn translation_max(&self) -> [f32; 3] {
        match self.property("TranslationMax") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn translation_min_x(&self) -> bool {
        match self.property("TranslationMinX") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn translation_max_x(&self) -> bool {
        match self.property("TranslationMaxX") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn translation_min_y(&self) -> bool {
        match self.property("TranslationMinY") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn translation_max_y(&self) -> bool {
        match self.property("TranslationMaxY") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn translation_min_z(&self) -> bool {
        match self.property("TranslationMinZ") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn translation_max_z(&self) -> bool {
        match self.property("TranslationMaxZ") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_order(&self) -> ModelRotationOrder {
        match self.property("RotationOrder") {
            Some(Property::Int(v)) => ModelRotationOrder::from_i32(*v),
            _ => ModelRotationOrder::EulerXYZ,
        }
    }
    pub fn rotation_space_for_limit_only(&self) -> bool {
        match self.property("RotationSpaceForLimitOnly") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_stiffness_x(&self) -> f32 {
        match self.property("RotationStiffnessX") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn rotation_stiffness_y(&self) -> f32 {
        match self.property("RotationStiffnessY") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn rotation_stiffness_z(&self) -> f32 {
        match self.property("RotationStiffnessZ") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn axis_len(&self) -> f32 {
        match self.property("AxisLen") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn pre_rotation(&self) -> [f32; 3] {
        match self.property("PreRotation") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn post_rotation(&self) -> [f32; 3] {
        match self.property("PostRotation") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn rotation_active(&self) -> bool {
        match self.property("RotationActive") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_min(&self) -> [f32; 3] {
        match self.property("RotationMin") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn rotation_max(&self) -> [f32; 3] {
        match self.property("RotationMax") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn rotation_min_x(&self) -> bool {
        match self.property("RotationMinX") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_max_x(&self) -> bool {
        match self.property("RotationMaxX") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_min_y(&self) -> bool {
        match self.property("RotationMinY") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_max_y(&self) -> bool {
        match self.property("RotationMaxY") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_min_z(&self) -> bool {
        match self.property("RotationMinZ") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn rotation_max_z(&self) -> bool {
        match self.property("RotationMaxZ") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn inherit_type(&self) -> ModelTransformInheritance {
        match self.property("InheritType") {
            Some(Property::Int(v)) => ModelTransformInheritance::from_i32(*v),
            _ => ModelTransformInheritance::RrSs,
        }
    }
    pub fn scaling_active(&self) -> bool {
        match self.property("ScalingActive") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn scaling_min(&self) -> [f32; 3] {
        match self.property("ScalingMin") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn scaling_max(&self) -> [f32; 3] {
        match self.property("ScalingMax") {
            Some(Property::Vec3(v)) => *v,
            _ => [1.0, 1.0, 1.0],
        }
    }
    pub fn scaling_min_x(&self) -> bool {
        match self.property("ScalingMinX") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn scaling_max_x(&self) -> bool {
        match self.property("ScalingMaxX") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn scaling_min_y(&self) -> bool {
        match self.property("ScalingMinY") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn scaling_max_y(&self) -> bool {
        match self.property("ScalingMaxY") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn scaling_min_z(&self) -> bool {
        match self.property("ScalingMinZ") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn scaling_max_z(&self) -> bool {
        match self.property("ScalingMaxZ") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn geometric_translation(&self) -> [f32; 3] {
        match self.property("GeometricTranslation") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn geometric_rotation(&self) -> [f32; 3] {
        match self.property("GeometricRotation") {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }
    pub fn geometric_scaling(&self) -> [f32; 3] {
        match self.property("GeometricScaling") {
            Some(Property::Vec3(v)) => *v,
            _ => [1.0, 1.0, 1.0],
        }
    }
    pub fn min_damp_range_x(&self) -> f32 {
        match self.property("MinDampRangeX") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn min_damp_range_y(&self) -> f32 {
        match self.property("MinDampRangeY") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn min_damp_range_z(&self) -> f32 {
        match self.property("MinDampRangeZ") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn max_damp_range_x(&self) -> f32 {
        match self.property("MaxDampRangeX") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn max_damp_range_y(&self) -> f32 {
        match self.property("MaxDampRangeY") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn max_damp_range_z(&self) -> f32 {
        match self.property("MaxDampRangeZ") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn min_damp_strength_x(&self) -> f32 {
        match self.property("MinDampStrengthX") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn min_damp_strength_y(&self) -> f32 {
        match self.property("MinDampStrengthY") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn min_damp_strength_z(&self) -> f32 {
        match self.property("MinDampStrengthZ") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn max_damp_strength_x(&self) -> f32 {
        match self.property("MaxDampStrengthX") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn max_damp_strength_y(&self) -> f32 {
        match self.property("MaxDampStrengthY") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn max_damp_strength_z(&self) -> f32 {
        match self.property("MaxDampStrengthZ") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn preferred_angle_x(&self) -> f32 {
        match self.property("PreferredAngleX") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn preferred_angle_y(&self) -> f32 {
        match self.property("PreferredAngleY") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn preferred_angle_z(&self) -> f32 {
        match self.property("PreferredAngleZ") {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
    pub fn show(&self) -> bool {
        match self.property("Show") {
            Some(Property::Bool(v)) => *v,
            _ => true,
        }
    }
    pub fn lod_box(&self) -> bool {
        match self.property("LODBox") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
    pub fn freeze(&self) -> bool {
        match self.property("Freeze") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }

    /// Incoming object–object (`OO`) links from [`Material`] sources whose `connected_object_ids`
    /// contain this model’s id — Assimp [`Model::ResolveLinks`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXModel.cpp) material branch (`OP` links are excluded).
    ///
    /// Use `Material::get_textures` / `Material::get_layered_textures` on each entry for texture data.
    pub fn connected_materials<'a>(&'a self, document: &'a OwnedDocument) -> Vec<&'a Material> {
        let id = self.object.object_index;
        document
            .materials
            .iter()
            .filter(|m| m.inner().connected_object_ids.contains(&id))
            .collect()
    }

    /// Incoming `OO` links from `Geometry` sources (mesh, line, shape, or unknown geometry class).
    pub fn connected_geometries<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> Vec<ModelGeometryRef<'a>> {
        let id = self.object.object_index;
        let mut out = Vec::new();
        for g in &document.mesh_geometries {
            if g.inner().connected_object_ids.contains(&id) {
                out.push(ModelGeometryRef::Mesh(g));
            }
        }
        for g in &document.line_geometries {
            if g.inner().connected_object_ids.contains(&id) {
                out.push(ModelGeometryRef::Line(g));
            }
        }
        for g in &document.shape_geometries {
            if g.inner().connected_object_ids.contains(&id) {
                out.push(ModelGeometryRef::Shape(g));
            }
        }
        for o in &document.unknown_geometries {
            if o.connected_object_ids.contains(&id) {
                out.push(ModelGeometryRef::Unknown(o));
            }
        }
        out
    }

    /// Incoming `OO` links from `NodeAttribute` sources, same discriminant shape as
    /// `AnimationCurveNode::get_target_node_attribute`.
    pub fn connected_node_attributes<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> Vec<NodeAttributeRef<'a>> {
        let id = self.object.object_index;
        let mut out = Vec::new();
        for v in &document.cameras {
            if v.inner().connected_object_ids.contains(&id) {
                out.push(NodeAttributeRef::Camera(v));
            }
        }
        for v in &document.camera_switchers {
            if v.inner().connected_object_ids.contains(&id) {
                out.push(NodeAttributeRef::CameraSwitcher(v));
            }
        }
        for v in &document.lights {
            if v.inner().connected_object_ids.contains(&id) {
                out.push(NodeAttributeRef::Light(v));
            }
        }
        for v in &document.null_nodes {
            if v.inner().connected_object_ids.contains(&id) {
                out.push(NodeAttributeRef::NullNode(v));
            }
        }
        for v in &document.limb_nodes {
            if v.inner().connected_object_ids.contains(&id) {
                out.push(NodeAttributeRef::LimbNode(v));
            }
        }
        for o in &document.unknown_node_attributes {
            if o.connected_object_ids.contains(&id) {
                out.push(NodeAttributeRef::Unknown(o));
            }
        }
        out
    }
}

impl TryFrom<OwnedObject> for Model {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != FbxObjectTag::Model {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "Model".to_string()));
        }

        let shading = o
            .attributes
            .optional_token_case_insensitive(ATTR_SHADING)
            .ok()
            .flatten()
            .map(ToString::to_string)
            .unwrap_or_else(|| "Y".to_string());
        let culling = o
            .attributes
            .optional_token_case_insensitive(ATTR_CULLING)
            .ok()
            .flatten()
            .map(ToString::to_string)
            .unwrap_or_default();

        Ok(Model {
            object: o,
            shading,
            culling,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{ElementAttribute, LeafAttribute};

    use crate::objects::{
        GEOMETRY_TYPE_NAME, MATERIAL_CLASS_NAME, MATERIAL_TYPE_NAME, MODEL_TYPE_NAME, Model,
        ModelGeometryRef, ModelRotationOrder, ModelTransformInheritance,
        NODE_ATTRIBUTE_LIGHT_CLASS_NAME, NODE_ATTRIBUTE_TYPE_NAME, NodeAttributeRef,
        TEXTURE_CLASS_NAME, TEXTURE_TYPE_NAME,
    };
    use crate::{ObjectPropertyConnection, OwnedDocument, OwnedObject, Property};

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    #[test]
    fn extracts_shading_and_culling_and_properties() {
        let mut attrs = HashMap::new();
        attrs.insert("Shading".into(), leaf(&["Phong"]));
        attrs.insert("Culling".into(), leaf(&["CullingOff"]));
        let mut props = HashMap::new();
        props.insert("Show".into(), Property::Bool(false));
        props.insert("RotationOrder".into(), Property::Int(5));
        props.insert("InheritType".into(), Property::Int(2));
        let o = OwnedObject {
            object_index: 100,
            name: "Model::A".into(),
            type_name: MODEL_TYPE_NAME.into(),
            class_name: "Mesh".into(),
            properties: props,
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let m = Model::try_from(o).unwrap();
        assert_eq!(m.shading(), "Phong");
        assert_eq!(m.culling(), "CullingOff");
        assert_eq!(m.show(), false);
        assert_eq!(m.rotation_order(), ModelRotationOrder::EulerZYX);
        assert_eq!(m.inherit_type(), ModelTransformInheritance::Rrs);
    }

    #[test]
    fn defaults_match_assimp_header() {
        let o = OwnedObject {
            object_index: 101,
            name: "Model::B".into(),
            type_name: MODEL_TYPE_NAME.into(),
            class_name: "Mesh".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let m = Model::try_from(o).unwrap();
        assert_eq!(m.shading(), "Y");
        assert_eq!(m.culling(), "");
        assert_eq!(m.scaling_max(), [1.0, 1.0, 1.0]);
        assert_eq!(m.geometric_scaling(), [1.0, 1.0, 1.0]);
        assert_eq!(m.show(), true);
        assert_eq!(m.lod_box(), false);
        assert_eq!(m.freeze(), false);
        assert_eq!(m.rotation_order(), ModelRotationOrder::EulerXYZ);
        assert_eq!(m.inherit_type(), ModelTransformInheritance::RrSs);
    }

    #[test]
    fn model_typed_property_getters_cover_most_fields() {
        let props: HashMap<String, Property> = HashMap::from([
            ("QuaternionInterpolate".into(), Property::Int(1)),
            ("RotationOffset".into(), Property::Vec3([0.1, 0.2, 0.3])),
            ("RotationPivot".into(), Property::Vec3([0.4, 0.5, 0.6])),
            ("ScalingOffset".into(), Property::Vec3([0.7, 0.8, 0.9])),
            ("ScalingPivot".into(), Property::Vec3([1.0, 1.1, 1.2])),
            ("TranslationActive".into(), Property::Bool(true)),
            ("TranslationMin".into(), Property::Vec3([1.0, 2.0, 3.0])),
            ("TranslationMax".into(), Property::Vec3([4.0, 5.0, 6.0])),
            ("TranslationMinX".into(), Property::Bool(true)),
            ("TranslationMaxX".into(), Property::Bool(false)),
            ("TranslationMinY".into(), Property::Bool(false)),
            ("TranslationMaxY".into(), Property::Bool(true)),
            ("TranslationMinZ".into(), Property::Bool(true)),
            ("TranslationMaxZ".into(), Property::Bool(false)),
            ("RotationOrder".into(), Property::Int(6)),
            ("RotationSpaceForLimitOnly".into(), Property::Bool(true)),
            ("RotationStiffnessX".into(), Property::Float(0.11)),
            ("RotationStiffnessY".into(), Property::Float(0.22)),
            ("RotationStiffnessZ".into(), Property::Float(0.33)),
            ("AxisLen".into(), Property::Float(9.5)),
            ("PreRotation".into(), Property::Vec3([10.0, 20.0, 30.0])),
            ("PostRotation".into(), Property::Vec3([40.0, 50.0, 60.0])),
            ("RotationActive".into(), Property::Bool(true)),
            ("RotationMin".into(), Property::Vec3([-1.0, -2.0, -3.0])),
            ("RotationMax".into(), Property::Vec3([1.0, 2.0, 3.0])),
            ("RotationMinX".into(), Property::Bool(true)),
            ("RotationMaxX".into(), Property::Bool(false)),
            ("RotationMinY".into(), Property::Bool(false)),
            ("RotationMaxY".into(), Property::Bool(true)),
            ("RotationMinZ".into(), Property::Bool(true)),
            ("RotationMaxZ".into(), Property::Bool(false)),
            ("InheritType".into(), Property::Int(1)),
            ("ScalingActive".into(), Property::Bool(true)),
            ("ScalingMin".into(), Property::Vec3([0.5, 0.6, 0.7])),
            ("ScalingMax".into(), Property::Vec3([2.0, 2.5, 3.0])),
            ("ScalingMinX".into(), Property::Bool(true)),
            ("ScalingMaxX".into(), Property::Bool(false)),
            ("ScalingMinY".into(), Property::Bool(false)),
            ("ScalingMaxY".into(), Property::Bool(true)),
            ("ScalingMinZ".into(), Property::Bool(true)),
            ("ScalingMaxZ".into(), Property::Bool(false)),
            ("GeometricTranslation".into(), Property::Vec3([7.0, 8.0, 9.0])),
            ("GeometricRotation".into(), Property::Vec3([0.01, 0.02, 0.03])),
            ("GeometricScaling".into(), Property::Vec3([1.5, 2.5, 3.5])),
            ("MinDampRangeX".into(), Property::Float(0.1)),
            ("MinDampRangeY".into(), Property::Float(0.2)),
            ("MinDampRangeZ".into(), Property::Float(0.3)),
            ("MaxDampRangeX".into(), Property::Float(0.4)),
            ("MaxDampRangeY".into(), Property::Float(0.5)),
            ("MaxDampRangeZ".into(), Property::Float(0.6)),
            ("MinDampStrengthX".into(), Property::Float(0.7)),
            ("MinDampStrengthY".into(), Property::Float(0.8)),
            ("MinDampStrengthZ".into(), Property::Float(0.9)),
            ("MaxDampStrengthX".into(), Property::Float(1.1)),
            ("MaxDampStrengthY".into(), Property::Float(1.2)),
            ("MaxDampStrengthZ".into(), Property::Float(1.3)),
            ("PreferredAngleX".into(), Property::Float(2.1)),
            ("PreferredAngleY".into(), Property::Float(2.2)),
            ("PreferredAngleZ".into(), Property::Float(2.3)),
            ("Show".into(), Property::Bool(false)),
            ("LODBox".into(), Property::Bool(true)),
            ("Freeze".into(), Property::Bool(true)),
        ]);

        let o = OwnedObject {
            object_index: 600,
            name: "Model::Props".into(),
            type_name: MODEL_TYPE_NAME.into(),
            class_name: "Mesh".into(),
            properties: props,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };

        let m = Model::try_from(o).unwrap();
        assert_eq!(m.inner().object_index, 600);
        assert!(m.property("QuaternionInterpolate").is_some());
        assert!(m.property("missing").is_none());
        assert_eq!(m.properties().len(), 62);

        assert_eq!(m.quaternion_interpolate(), 1);
        assert_eq!(m.rotation_offset(), [0.1, 0.2, 0.3]);
        assert_eq!(m.rotation_pivot(), [0.4, 0.5, 0.6]);
        assert_eq!(m.scaling_offset(), [0.7, 0.8, 0.9]);
        assert_eq!(m.scaling_pivot(), [1.0, 1.1, 1.2]);
        assert_eq!(m.translation_active(), true);
        assert_eq!(m.translation_min(), [1.0, 2.0, 3.0]);
        assert_eq!(m.translation_max(), [4.0, 5.0, 6.0]);
        assert_eq!(m.translation_min_x(), true);
        assert_eq!(m.translation_max_x(), false);
        assert_eq!(m.translation_min_y(), false);
        assert_eq!(m.translation_max_y(), true);
        assert_eq!(m.translation_min_z(), true);
        assert_eq!(m.translation_max_z(), false);
        assert_eq!(m.rotation_order(), ModelRotationOrder::SphericXYZ);
        assert_eq!(m.rotation_space_for_limit_only(), true);
        assert_eq!(m.rotation_stiffness_x(), 0.11);
        assert_eq!(m.rotation_stiffness_y(), 0.22);
        assert_eq!(m.rotation_stiffness_z(), 0.33);
        assert_eq!(m.axis_len(), 9.5);
        assert_eq!(m.pre_rotation(), [10.0, 20.0, 30.0]);
        assert_eq!(m.post_rotation(), [40.0, 50.0, 60.0]);
        assert_eq!(m.rotation_active(), true);
        assert_eq!(m.rotation_min(), [-1.0, -2.0, -3.0]);
        assert_eq!(m.rotation_max(), [1.0, 2.0, 3.0]);
        assert_eq!(m.rotation_min_x(), true);
        assert_eq!(m.rotation_max_x(), false);
        assert_eq!(m.rotation_min_y(), false);
        assert_eq!(m.rotation_max_y(), true);
        assert_eq!(m.rotation_min_z(), true);
        assert_eq!(m.rotation_max_z(), false);
        assert_eq!(m.inherit_type(), ModelTransformInheritance::RSrs);
        assert_eq!(m.scaling_active(), true);
        assert_eq!(m.scaling_min(), [0.5, 0.6, 0.7]);
        assert_eq!(m.scaling_max(), [2.0, 2.5, 3.0]);
        assert_eq!(m.scaling_min_x(), true);
        assert_eq!(m.scaling_max_x(), false);
        assert_eq!(m.scaling_min_y(), false);
        assert_eq!(m.scaling_max_y(), true);
        assert_eq!(m.scaling_min_z(), true);
        assert_eq!(m.scaling_max_z(), false);
        assert_eq!(m.geometric_translation(), [7.0, 8.0, 9.0]);
        assert_eq!(m.geometric_rotation(), [0.01, 0.02, 0.03]);
        assert_eq!(m.geometric_scaling(), [1.5, 2.5, 3.5]);
        assert_eq!(m.min_damp_range_x(), 0.1);
        assert_eq!(m.min_damp_range_y(), 0.2);
        assert_eq!(m.min_damp_range_z(), 0.3);
        assert_eq!(m.max_damp_range_x(), 0.4);
        assert_eq!(m.max_damp_range_y(), 0.5);
        assert_eq!(m.max_damp_range_z(), 0.6);
        assert_eq!(m.min_damp_strength_x(), 0.7);
        assert_eq!(m.min_damp_strength_y(), 0.8);
        assert_eq!(m.min_damp_strength_z(), 0.9);
        assert_eq!(m.max_damp_strength_x(), 1.1);
        assert_eq!(m.max_damp_strength_y(), 1.2);
        assert_eq!(m.max_damp_strength_z(), 1.3);
        assert_eq!(m.preferred_angle_x(), 2.1);
        assert_eq!(m.preferred_angle_y(), 2.2);
        assert_eq!(m.preferred_angle_z(), 2.3);
        assert_eq!(m.show(), false);
        assert_eq!(m.lod_box(), true);
        assert_eq!(m.freeze(), true);

        let inner = m.into_inner();
        assert_eq!(inner.object_index, 600);
        assert_eq!(inner.name, "Model::Props");
    }

    #[test]
    fn resolves_incoming_material_geometry_and_node_attribute_oo_links() {
        let model = Model::try_from(OwnedObject {
            object_index: 500,
            name: "Model::Root".into(),
            type_name: MODEL_TYPE_NAME.into(),
            class_name: "Mesh".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let material = crate::objects::Material::try_from(OwnedObject {
            object_index: 501,
            name: "Material::M".into(),
            type_name: MATERIAL_TYPE_NAME.into(),
            class_name: MATERIAL_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::from([
                (
                    "ShadingModel".to_string(),
                    ElementAttribute::Leaf(Box::new(LeafAttribute {
                        key: "ShadingModel".into(),
                        tokens: vec!["Phong".into()],
                    })),
                ),
                (
                    "MultiLayer".to_string(),
                    ElementAttribute::Leaf(Box::new(LeafAttribute {
                        key: "MultiLayer".into(),
                        tokens: vec!["0".into()],
                    })),
                ),
            ]),
            connected_object_ids: vec![500],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let unknown_geo = OwnedObject {
            object_index: 502,
            name: "Geometry::Custom".into(),
            type_name: GEOMETRY_TYPE_NAME.into(),
            class_name: "CustomMesh".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![500],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };

        let light = crate::objects::Light::try_from(OwnedObject {
            object_index: 503,
            name: "NodeAttribute::L".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_LIGHT_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![500],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let texture = crate::objects::Texture::try_from(OwnedObject {
            object_index: 504,
            name: "Texture::D".into(),
            type_name: TEXTURE_TYPE_NAME.into(),
            class_name: TEXTURE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![ObjectPropertyConnection {
                dest: 501,
                property: "DiffuseColor".into(),
            }],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let mut doc = OwnedDocument::default();
        doc.models = vec![model];
        doc.materials = vec![material];
        doc.unknown_geometries = vec![unknown_geo];
        doc.lights = vec![light];
        doc.textures = vec![texture];

        let model = &doc.models[0];
        let mats = model.connected_materials(&doc);
        assert_eq!(mats.len(), 1);
        assert_eq!(mats[0].inner().object_index, 501);

        let geos = model.connected_geometries(&doc);
        assert_eq!(geos.len(), 1);
        assert!(matches!(geos[0], ModelGeometryRef::Unknown(_)));
        assert_eq!(geos[0].inner().object_index, 502);

        let attrs = model.connected_node_attributes(&doc);
        assert_eq!(attrs.len(), 1);
        assert!(matches!(attrs[0], NodeAttributeRef::Light(_)));
        assert_eq!(attrs[0].inner().object_index, 503);

        let tex = mats[0].get_textures(&doc);
        assert_eq!(
            tex.get("DiffuseColor").map(|t| t.inner().object_index),
            Some(504)
        );
    }
}
