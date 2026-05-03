//! FBX `AnimationCurveNode` — Assimp [`AnimationCurveNode`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXAnimation.cpp) / [`FBXDocument.h`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).
//!
//! Target object and animated property name are resolved via connections in Assimp; this wrapper
//! only exposes the property table on [`OwnedObject`].

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::Property;
use crate::{ObjectPropertyConnection, OwnedDocument, OwnedObject};

use super::{
    AnimationCurve, FbxObjectTag, FbxTypeMismatch, Model, NodeAttributeRef, fbx_object_tag,
};

#[derive(Debug, PartialEq)]
pub struct AnimationCurveNode(pub OwnedObject);

impl AnimationCurveNode {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }

    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.0.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.0.properties.get(name)
    }

    /// First valid OP target with a non-empty property name, mirroring Assimp's search order.
    fn get_target_property_connection(&self) -> Option<&ObjectPropertyConnection> {
        self.inner()
            .object_property_targets
            .iter()
            .find(|c| !c.property.is_empty())
    }

    pub fn get_target_model<'a>(&'a self, document: &'a OwnedDocument) -> Option<&'a Model> {
        let target = self.get_target_property_connection()?;
        document
            .models
            .iter()
            .find(|m| m.inner().object_index == target.dest)
    }

    pub fn get_target_node_attribute<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> Option<NodeAttributeRef<'a>> {
        let target = self.get_target_property_connection()?;
        if let Some(v) = document
            .cameras
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(NodeAttributeRef::Camera(v));
        }
        if let Some(v) = document
            .camera_switchers
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(NodeAttributeRef::CameraSwitcher(v));
        }
        if let Some(v) = document
            .lights
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(NodeAttributeRef::Light(v));
        }
        if let Some(v) = document
            .null_nodes
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(NodeAttributeRef::NullNode(v));
        }
        if let Some(v) = document
            .limb_nodes
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(NodeAttributeRef::LimbNode(v));
        }
        document
            .unknown_node_attributes
            .iter()
            .find(|x| x.object_index == target.dest)
            .map(NodeAttributeRef::Unknown)
    }

    pub fn get_target_deformer<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> Option<&'a OwnedObject> {
        let target = self.get_target_property_connection()?;
        if let Some(v) = document
            .clusters
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(v.inner());
        }
        if let Some(v) = document
            .skins
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(v.inner());
        }
        if let Some(v) = document
            .blend_shapes
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(v.inner());
        }
        if let Some(v) = document
            .blend_shape_channels
            .iter()
            .find(|x| x.inner().object_index == target.dest)
        {
            return Some(v.inner());
        }
        document
            .unknown_deformers
            .iter()
            .find(|x| x.object_index == target.dest)
    }

    /// Resolve incoming `AnimationCurve -> AnimationCurveNode` OP links keyed by property name.
    pub fn get_curves<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> HashMap<&'a str, &'a AnimationCurve> {
        let node_id = self.inner().object_index;
        let mut out = HashMap::new();
        for curve in &document.animation_curves {
            for conn in &curve.inner().object_property_targets {
                if conn.dest == node_id && !conn.property.is_empty() {
                    out.insert(conn.property.as_str(), curve);
                }
            }
        }
        out
    }
}

impl TryFrom<OwnedObject> for AnimationCurveNode {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::AnimationCurveNode => Ok(AnimationCurveNode(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "AnimationCurveNode".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{
        ANIMATION_CURVE_CLASS_NAME, ANIMATION_CURVE_NODE_CLASS_NAME,
        ANIMATION_CURVE_NODE_TYPE_NAME, ANIMATION_CURVE_TYPE_NAME, DEFORMER_TYPE_NAME,
        MODEL_TYPE_NAME, NodeAttributeRef,
    };
    use crate::{ObjectPropertyConnection, OwnedDocument, OwnedObject, Property};

    use super::AnimationCurveNode;

    #[test]
    fn property_accessors() {
        let mut properties = HashMap::new();
        properties.insert("d|Visibility".to_string(), Property::Float(1.0));
        let o = OwnedObject {
            object_index: 2,
            name: "AnimCurveNode::T".into(),
            type_name: ANIMATION_CURVE_NODE_TYPE_NAME.into(),
            class_name: ANIMATION_CURVE_NODE_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let n = AnimationCurveNode::try_from(o).unwrap();
        assert_eq!(n.property("d|Visibility"), Some(&Property::Float(1.0)));
    }

    #[test]
    fn resolves_target_model_and_curves() {
        let node = AnimationCurveNode::try_from(OwnedObject {
            object_index: 3000,
            name: "AnimCurveNode::T".into(),
            type_name: ANIMATION_CURVE_NODE_TYPE_NAME.into(),
            class_name: ANIMATION_CURVE_NODE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![ObjectPropertyConnection {
                dest: 3001,
                property: "Lcl Translation".into(),
            }],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let model = crate::objects::Model::try_from(OwnedObject {
            object_index: 3001,
            name: "Model::Node".into(),
            type_name: MODEL_TYPE_NAME.into(),
            class_name: "Mesh".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let curve = crate::objects::AnimationCurve::try_from(OwnedObject {
            object_index: 3002,
            name: "AnimCurve::X".into(),
            type_name: ANIMATION_CURVE_TYPE_NAME.into(),
            class_name: ANIMATION_CURVE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::from([
                ("KeyTime".to_string(), {
                    use fbxscii::{ElementAttribute, LeafAttribute};
                    ElementAttribute::Leaf(Box::new(LeafAttribute {
                        key: String::new(),
                        tokens: vec!["0,1".into()],
                    }))
                }),
                ("KeyValueFloat".to_string(), {
                    use fbxscii::{ElementAttribute, LeafAttribute};
                    ElementAttribute::Leaf(Box::new(LeafAttribute {
                        key: String::new(),
                        tokens: vec!["0,1".into()],
                    }))
                }),
            ]),
            connected_object_ids: vec![],
            object_property_targets: vec![ObjectPropertyConnection {
                dest: 3000,
                property: "d|X".into(),
            }],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let mut owned = OwnedDocument::default();
        owned.models = vec![model];
        owned.animation_curves = vec![curve];

        assert_eq!(
            node.get_target_model(&owned)
                .map(|m| m.inner().object_index),
            Some(3001)
        );
        let curves = node.get_curves(&owned);
        assert_eq!(curves.len(), 1);
        assert_eq!(
            curves.get("d|X").map(|curve| curve.inner().object_index),
            Some(3002)
        );
    }

    #[test]
    fn resolves_target_node_attribute_ref() {
        let node = AnimationCurveNode::try_from(OwnedObject {
            object_index: 3100,
            name: "AnimCurveNode::T".into(),
            type_name: ANIMATION_CURVE_NODE_TYPE_NAME.into(),
            class_name: ANIMATION_CURVE_NODE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![ObjectPropertyConnection {
                dest: 3101,
                property: "d|Visibility".into(),
            }],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let camera = crate::objects::Camera::try_from(OwnedObject {
            object_index: 3101,
            name: "NodeAttribute::Cam".into(),
            type_name: "NodeAttribute".into(),
            class_name: "Camera".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let mut owned = OwnedDocument::default();
        owned.cameras = vec![camera];
        let target = node.get_target_node_attribute(&owned).unwrap();
        assert!(matches!(target, NodeAttributeRef::Camera(_)));
    }

    #[test]
    fn resolves_target_deformer() {
        let node = AnimationCurveNode::try_from(OwnedObject {
            object_index: 3200,
            name: "AnimCurveNode::T".into(),
            type_name: ANIMATION_CURVE_NODE_TYPE_NAME.into(),
            class_name: ANIMATION_CURVE_NODE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![ObjectPropertyConnection {
                dest: 3201,
                property: "DeformPercent".into(),
            }],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let mut owned = OwnedDocument::default();
        owned.unknown_deformers = vec![OwnedObject {
            object_index: 3201,
            name: "Deformer::Custom".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: "CustomDeformer".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        }];

        let target = node.get_target_deformer(&owned).unwrap();
        assert_eq!(target.object_index, 3201);
    }
}
