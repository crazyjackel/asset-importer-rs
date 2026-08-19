use asset_importer_rs_scene::{AiCamera, AiVector3D};
use dae_parser::{Camera, LocalMap, Node, ProjectionType};

use crate::DaeImportError;

use super::DaeImporter;

impl DaeImporter {
    pub(crate) fn build_cameras_for_node(
        camera_map: &LocalMap<'_, Camera>,
        node: &Node,
        node_name: &str,
        cameras: &mut Vec<AiCamera>,
    ) -> Result<(), DaeImportError> {
        for instance in &node.instance_camera {
            let Some(src_camera) = camera_map.get(&instance.url) else {
                continue;
            };
            let mut camera = ai_camera_from(src_camera);
            // Camera's name corresponds to the node name in the scene graph
            camera.name = node_name.to_string();
            cameras.push(camera);
        }
        Ok(())
    }
}

fn ai_camera_from(src_camera: &Camera) -> AiCamera {
    let mut out = AiCamera {
        look_vec: AiVector3D::new(0.0, 0.0, -1.0),
        ..AiCamera::default()
    };

    match &src_camera.optics.ty {
        ProjectionType::Orthographic(ortho) => {
            out.near_plane = ortho.znear;
            out.far_plane = ortho.zfar;
            out.horizontal_fov = 0.0;
            if let Some(aspect) = ortho.aspect_ratio {
                out.aspect_ratio = aspect;
            }
            if let (Some(xmag), Some(ymag)) = (ortho.xmag, ortho.ymag) {
                if ymag != 0.0 {
                    out.aspect_ratio = xmag / ymag;
                }
            }
            if let Some(xmag) = ortho.xmag {
                out.orthographic_width = xmag;
            } else if let (Some(ymag), Some(aspect)) = (ortho.ymag, ortho.aspect_ratio) {
                out.orthographic_width = ymag * aspect;
            }
        }
        ProjectionType::Perspective(perspective) => {
            out.near_plane = perspective.znear;
            out.far_plane = perspective.zfar;
            if let Some(aspect) = perspective.aspect_ratio {
                out.aspect_ratio = aspect;
            }
            if let Some(hor_fov) = perspective.xfov {
                out.horizontal_fov = hor_fov.to_radians();
                if perspective.aspect_ratio.is_none()
                    && let Some(ver_fov) = perspective.yfov
                {
                    out.aspect_ratio =
                        (hor_fov.to_radians() * 0.5).tan() / (ver_fov.to_radians() * 0.5).tan();
                }
            } else if let (Some(aspect), Some(ver_fov)) =
                (perspective.aspect_ratio, perspective.yfov)
            {
                out.horizontal_fov = 2.0 * (aspect * (ver_fov.to_radians() * 0.5).tan()).atan();
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use dae_parser::Document;
    use std::str::FromStr;

    fn camera_document_with(camera_open: &str, optics: &str) -> Document {
        let xml = format!(
            r##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <created>1970-01-01T00:00:00Z</created>
    <modified>1970-01-01T00:00:00Z</modified>
  </asset>
  <library_cameras>
    {camera_open}
      <optics>
        <technique_common>
          {optics}
        </technique_common>
      </optics>
    </camera>
  </library_cameras>
  <library_visual_scenes>
    <visual_scene id="Scene">
      <node id="CamNode">
        <instance_camera url="#Cam"/>
      </node>
    </visual_scene>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Scene"/>
  </scene>
</COLLADA>"##
        );
        Document::from_str(&xml).expect("camera document should parse")
    }

    fn camera_document(optics: &str) -> Document {
        camera_document_with(r#"<camera id="Cam">"#, optics)
    }

    fn import_cameras(document: &Document) -> Vec<AiCamera> {
        let camera_map = document.local_map::<Camera>().expect("camera map");
        let node = document
            .get_visual_scene()
            .expect("visual scene")
            .nodes
            .first()
            .expect("root node");
        let mut cameras = Vec::new();
        DaeImporter::build_cameras_for_node(&camera_map, node, "CamNode", &mut cameras)
            .expect("cameras");
        cameras
    }

    #[test]
    fn perspective_xfov_is_converted_to_radians() {
        let document = camera_document(
            "<perspective><xfov>90</xfov><aspect_ratio>1.777</aspect_ratio><znear>0.1</znear><zfar>1000</zfar></perspective>",
        );
        let cameras = import_cameras(&document);
        assert_eq!(cameras.len(), 1);
        let camera = &cameras[0];
        assert_eq!(camera.name, "CamNode");
        assert_eq!(camera.look_vec, AiVector3D::new(0.0, 0.0, -1.0));
        assert!((camera.horizontal_fov - 90f32.to_radians()).abs() < 1e-5);
        assert!((camera.aspect_ratio - 1.777).abs() < 1e-5);
        assert_eq!(camera.near_plane, 0.1);
        assert_eq!(camera.far_plane, 1000.0);
    }

    #[test]
    fn perspective_yfov_and_aspect_compute_horizontal_fov() {
        let document = camera_document(
            "<perspective><yfov>60</yfov><aspect_ratio>2</aspect_ratio><znear>1</znear><zfar>10</zfar></perspective>",
        );
        let cameras = import_cameras(&document);
        let expected = 2.0 * (2.0 * (60f32.to_radians() * 0.5).tan()).atan();
        assert!((cameras[0].horizontal_fov - expected).abs() < 1e-5);
        assert_eq!(cameras[0].aspect_ratio, 2.0);
    }

    #[test]
    fn perspective_xfov_and_yfov_compute_aspect() {
        let document = camera_document(
            "<perspective><xfov>90</xfov><yfov>45</yfov><znear>1</znear><zfar>10</zfar></perspective>",
        );
        let cameras = import_cameras(&document);
        let expected = (90f32.to_radians() * 0.5).tan() / (45f32.to_radians() * 0.5).tan();
        assert!((cameras[0].aspect_ratio - expected).abs() < 1e-5);
    }

    #[test]
    fn orthographic_xmag_and_ymag_set_width_and_aspect() {
        let document = camera_document(
            "<orthographic><xmag>2</xmag><ymag>1</ymag><znear>0.1</znear><zfar>100</zfar></orthographic>",
        );
        let camera = &import_cameras(&document)[0];
        assert_eq!(camera.horizontal_fov, 0.0);
        assert_eq!(camera.orthographic_width, 2.0);
        assert_eq!(camera.aspect_ratio, 2.0);
        assert_eq!(camera.near_plane, 0.1);
        assert_eq!(camera.far_plane, 100.0);
    }

    #[test]
    fn orthographic_ymag_and_aspect_derive_width() {
        let document = camera_document(
            "<orthographic><ymag>3</ymag><aspect_ratio>1.5</aspect_ratio><znear>1</znear><zfar>10</zfar></orthographic>",
        );
        let camera = &import_cameras(&document)[0];
        assert_eq!(camera.horizontal_fov, 0.0);
        assert_eq!(camera.orthographic_width, 4.5);
        assert_eq!(camera.aspect_ratio, 1.5);
    }

    #[test]
    fn missing_camera_instance_is_skipped() {
        let xml = r##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <created>1970-01-01T00:00:00Z</created>
    <modified>1970-01-01T00:00:00Z</modified>
  </asset>
  <library_visual_scenes>
    <visual_scene id="Scene">
      <node id="CamNode">
        <instance_camera url="#Missing"/>
      </node>
    </visual_scene>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Scene"/>
  </scene>
</COLLADA>"##;
        let document = Document::from_str(xml).expect("document should parse");
        assert!(import_cameras(&document).is_empty());
    }
}
