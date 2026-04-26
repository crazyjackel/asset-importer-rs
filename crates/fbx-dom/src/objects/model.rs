//! FBX `Model` objects — Assimp [`Model`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXModel.cpp) / [`FBXDocument.h`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{AttrExtractorExt, FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

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
}

impl TryFrom<OwnedObject> for Model {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::Model) {
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

    use crate::objects::{MODEL_TYPE_NAME, Model, ModelRotationOrder, ModelTransformInheritance};
    use crate::OwnedObject;
    use crate::Property;

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
}
