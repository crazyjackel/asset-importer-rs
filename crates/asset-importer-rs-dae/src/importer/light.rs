use asset_importer_rs_scene::{AiColor3D, AiLight, AiLightSourceType, AiVector3D};
use dae_parser::{Light, LightKind, LocalMap, Node};

use crate::DaeImportError;

use super::DaeImporter;

/// Sentinel used by Assimp for unset FCOLLADA cone extras.
const LIGHT_ANGLE_NOT_SET: f32 = 1e9;

impl DaeImporter {
    pub(crate) fn build_lights_for_node(
        light_map: &LocalMap<'_, Light>,
        node: &Node,
        node_name: &str,
        lights: &mut Vec<AiLight>,
    ) -> Result<(), DaeImportError> {
        for instance in &node.instance_light {
            let Some(src_light) = light_map.get(&instance.url) else {
                continue;
            };
            let mut light = ai_light_from(src_light);
            light.name = node_name.to_string();
            lights.push(light);
        }
        Ok(())
    }
}

fn ai_light_from(src_light: &Light) -> AiLight {
    let extras = LightExtras::from(src_light);
    let mut out = AiLight {
        direction: AiVector3D::new(0.0, 0.0, -1.0),
        ..AiLight::default()
    };

    match &src_light.kind {
        LightKind::Ambient(ambient) => {
            let color = AiColor3D::new(
                ambient.color[0] * extras.intensity,
                ambient.color[1] * extras.intensity,
                ambient.color[2] * extras.intensity,
            );
            out.source_type = AiLightSourceType::Ambient;
            out.attenuation = 1.0;
            out.attenuation_linear = 0.0;
            out.attenuation_quadratic = 0.0;
            out.ambient_color = color;
            out.diffuse_color = AiColor3D::new(0.0, 0.0, 0.0);
            out.specular_color = AiColor3D::new(0.0, 0.0, 0.0);
        }
        LightKind::Directional(directional) => {
            let color = AiColor3D::new(
                directional.color[0] * extras.intensity,
                directional.color[1] * extras.intensity,
                directional.color[2] * extras.intensity,
            );
            out.source_type = AiLightSourceType::Directional;
            out.attenuation = 1.0;
            out.attenuation_linear = 0.0;
            out.attenuation_quadratic = 0.0;
            out.diffuse_color = color;
            out.specular_color = color;
            out.ambient_color = AiColor3D::new(0.0, 0.0, 0.0);
        }
        LightKind::Point(point) => {
            let color = AiColor3D::new(
                point.color[0] * extras.intensity,
                point.color[1] * extras.intensity,
                point.color[2] * extras.intensity,
            );
            out.source_type = AiLightSourceType::Point;
            out.attenuation = point.constant_attenuation;
            out.attenuation_linear = point.linear_attenuation;
            out.attenuation_quadratic = point.quadratic_attenuation;
            out.diffuse_color = color;
            out.specular_color = color;
            out.ambient_color = AiColor3D::new(0.0, 0.0, 0.0);
        }
        LightKind::Spot(spot) => {
            let color = AiColor3D::new(
                spot.color[0] * extras.intensity,
                spot.color[1] * extras.intensity,
                spot.color[2] * extras.intensity,
            );
            out.source_type = AiLightSourceType::Spot;
            out.attenuation = spot.constant_attenuation;
            out.attenuation_linear = spot.linear_attenuation;
            out.attenuation_quadratic = spot.quadratic_attenuation;
            out.diffuse_color = color;
            out.specular_color = color;
            out.ambient_color = AiColor3D::new(0.0, 0.0, 0.0);
            let falloff_angle = extras.falloff_angle.unwrap_or(spot.falloff_angle);
            out.inner_cone_angle = falloff_angle.to_radians();
            if extras.outer_angle >= LIGHT_ANGLE_NOT_SET * (1.0 - 1e-6) {
                if extras.penumbra_angle >= LIGHT_ANGLE_NOT_SET * (1.0 - 1e-6) {
                    let f = if spot.falloff_exponent != 0.0 {
                        1.0 / spot.falloff_exponent
                    } else {
                        1.0
                    };
                    out.outer_cone_angle = 0.1f32.powf(f).acos() + out.inner_cone_angle;
                } else {
                    out.outer_cone_angle =
                        out.inner_cone_angle + extras.penumbra_angle.to_radians();
                    if out.outer_cone_angle < out.inner_cone_angle {
                        std::mem::swap(&mut out.inner_cone_angle, &mut out.outer_cone_angle);
                    }
                }
            } else {
                out.outer_cone_angle = extras.outer_angle.to_radians();
            }
        }
    }

    out
}

struct LightExtras {
    intensity: f32,
    outer_angle: f32,
    penumbra_angle: f32,
    falloff_angle: Option<f32>,
}

impl Default for LightExtras {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            outer_angle: LIGHT_ANGLE_NOT_SET,
            penumbra_angle: LIGHT_ANGLE_NOT_SET,
            falloff_angle: None,
        }
    }
}

impl From<&Light> for LightExtras {
    fn from(src_light: &Light) -> Self {
        let mut extras = Self::default();
        for extra in &src_light.extra {
            for technique in &extra.technique {
                for child in technique.element.children() {
                    let Ok(value) = child.text().trim().parse::<f32>() else {
                        continue;
                    };
                    match child.name() {
                        "intensity" => extras.intensity = value,
                        "outer_cone" | "falloff" | "decay_falloff" => extras.outer_angle = value,
                        "penumbra_angle" => extras.penumbra_angle = value,
                        "hotspot_beam" => extras.falloff_angle = Some(value),
                        _ => {}
                    }
                }
            }
        }
        extras
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dae_parser::Document;
    use std::str::FromStr;

    fn light_document(light_body: &str) -> Document {
        let xml = format!(
            r##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <created>1970-01-01T00:00:00Z</created>
    <modified>1970-01-01T00:00:00Z</modified>
  </asset>
  <library_lights>
    <light id="Lamp">
      {light_body}
    </light>
  </library_lights>
  <library_visual_scenes>
    <visual_scene id="Scene">
      <node id="LampNode">
        <instance_light url="#Lamp"/>
      </node>
    </visual_scene>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Scene"/>
  </scene>
</COLLADA>"##
        );
        Document::from_str(&xml).expect("light document should parse")
    }

    fn import_lights(document: &Document) -> Vec<AiLight> {
        let light_map = document.local_map::<Light>().expect("light map");
        let node = document
            .get_visual_scene()
            .expect("visual scene")
            .nodes
            .first()
            .expect("root node");
        let mut lights = Vec::new();
        DaeImporter::build_lights_for_node(&light_map, node, "LampNode", &mut lights)
            .expect("lights");
        lights
    }

    #[test]
    fn ambient_light_uses_ambient_color_only() {
        let document = light_document(
            "<technique_common><ambient><color>0.2 0.4 0.6</color></ambient></technique_common>",
        );
        let lights = import_lights(&document);
        assert_eq!(lights.len(), 1);
        let light = &lights[0];
        assert_eq!(light.name, "LampNode");
        assert_eq!(light.source_type, AiLightSourceType::Ambient);
        assert_eq!(light.direction, AiVector3D::new(0.0, 0.0, -1.0));
        assert_eq!(light.ambient_color, AiColor3D::new(0.2, 0.4, 0.6));
        assert_eq!(light.diffuse_color, AiColor3D::new(0.0, 0.0, 0.0));
        assert_eq!(light.specular_color, AiColor3D::new(0.0, 0.0, 0.0));
        assert_eq!(light.attenuation, 1.0);
        assert_eq!(light.attenuation_linear, 0.0);
        assert_eq!(light.attenuation_quadratic, 0.0);
    }

    #[test]
    fn point_light_copies_attenuation_and_punctual_colors() {
        let document = light_document(
            "<technique_common><point><color>1 0 0</color><constant_attenuation>1</constant_attenuation><linear_attenuation>0.1</linear_attenuation><quadratic_attenuation>0.01</quadratic_attenuation></point></technique_common>",
        );
        let light = &import_lights(&document)[0];
        assert_eq!(light.source_type, AiLightSourceType::Point);
        assert_eq!(light.diffuse_color, AiColor3D::new(1.0, 0.0, 0.0));
        assert_eq!(light.specular_color, AiColor3D::new(1.0, 0.0, 0.0));
        assert_eq!(light.ambient_color, AiColor3D::new(0.0, 0.0, 0.0));
        assert_eq!(light.attenuation, 1.0);
        assert_eq!(light.attenuation_linear, 0.1);
        assert_eq!(light.attenuation_quadratic, 0.01);
    }

    #[test]
    fn intensity_scales_light_color() {
        let document = light_document(
            r#"<technique_common><directional><color>1 1 1</color></directional></technique_common>
            <extra><technique profile="FCOLLADA"><intensity>0.5</intensity></technique></extra>"#,
        );
        let light = &import_lights(&document)[0];
        assert_eq!(light.source_type, AiLightSourceType::Directional);
        assert_eq!(light.diffuse_color, AiColor3D::new(0.5, 0.5, 0.5));
        assert_eq!(light.specular_color, AiColor3D::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn spot_falloff_exponent_estimates_outer_cone() {
        let document = light_document(
            "<technique_common><spot><color>1 1 1</color><falloff_angle>45</falloff_angle><falloff_exponent>0</falloff_exponent></spot></technique_common>",
        );
        let light = &import_lights(&document)[0];
        assert_eq!(light.source_type, AiLightSourceType::Spot);
        assert!((light.inner_cone_angle - 45f32.to_radians()).abs() < 1e-5);
        let expected_outer = 0.1f32.acos() + 45f32.to_radians();
        assert!((light.outer_cone_angle - expected_outer).abs() < 1e-5);
    }

    #[test]
    fn spot_outer_cone_extra_is_converted_to_radians() {
        let document = light_document(
            r#"<technique_common><spot><color>1 1 1</color><falloff_angle>20</falloff_angle></spot></technique_common>
            <extra><technique profile="FCOLLADA"><outer_cone>40</outer_cone></technique></extra>"#,
        );
        let light = &import_lights(&document)[0];
        assert!((light.inner_cone_angle - 20f32.to_radians()).abs() < 1e-5);
        assert!((light.outer_cone_angle - 40f32.to_radians()).abs() < 1e-5);
    }

    #[test]
    fn spot_penumbra_swaps_if_outer_is_smaller() {
        let document = light_document(
            r#"<technique_common><spot><color>1 1 1</color><falloff_angle>40</falloff_angle></spot></technique_common>
            <extra><technique profile="FCOLLADA"><penumbra_angle>-10</penumbra_angle></technique></extra>"#,
        );
        let light = &import_lights(&document)[0];
        assert!((light.inner_cone_angle - 30f32.to_radians()).abs() < 1e-5);
        assert!((light.outer_cone_angle - 40f32.to_radians()).abs() < 1e-5);
    }

    #[test]
    fn missing_light_instance_is_skipped() {
        let xml = r##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <created>1970-01-01T00:00:00Z</created>
    <modified>1970-01-01T00:00:00Z</modified>
  </asset>
  <library_visual_scenes>
    <visual_scene id="Scene">
      <node id="LampNode">
        <instance_light url="#Missing"/>
      </node>
    </visual_scene>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Scene"/>
  </scene>
</COLLADA>"##;
        let document = Document::from_str(xml).expect("document should parse");
        assert!(import_lights(&document).is_empty());
    }
}
