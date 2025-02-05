use gltf::{
    Camera, Document,
};

use crate::{
    core::error::AiReadError,
    structs::{AiCamera, AiVector3D},
};

use super::gltf2_importer::Gltf2Importer;

impl Gltf2Importer {
    pub(crate) fn import_cameras(document: &Document) -> Result<Vec<AiCamera>, AiReadError> {
        let asset_cameras: Vec<Camera<'_>> = document.cameras().collect();
        let mut cameras: Vec<AiCamera> = Vec::new(); //Final Meshes to return
        cameras.reserve(asset_cameras.len());
        for camera in asset_cameras {
            let mut ai_camera = AiCamera::default();
            ai_camera.name = camera.name().unwrap_or("").to_string();
            ai_camera.look_vec = AiVector3D::new(0.0, 0.0, -1.0);
            match camera.projection() {
                gltf::camera::Projection::Orthographic(orthographic) => {
                    ai_camera.aspect_ratio = if orthographic.ymag() == 0.0 {
                        1.0
                    } else {
                        orthographic.xmag() / orthographic.ymag()
                    };
                    ai_camera.horizontal_fov = 0.0;
                    ai_camera.near_plane = orthographic.znear();
                    ai_camera.far_plane = orthographic.zfar();
                    ai_camera.orthographic_width = orthographic.xmag();
                }
                gltf::camera::Projection::Perspective(perspective) => {
                    ai_camera.aspect_ratio = perspective.aspect_ratio().unwrap_or(0.0);
                    ai_camera.horizontal_fov = 2.0
                        * f32::atan(
                            f32::tan(perspective.yfov() * 0.5)
                                * (if ai_camera.aspect_ratio == 0.0 {
                                    1.0
                                } else {
                                    ai_camera.aspect_ratio
                                }),
                        );
                    ai_camera.near_plane = perspective.znear();
                    ai_camera.far_plane = perspective.zfar().unwrap_or(100.0)
                }
            }
            cameras.push(ai_camera);
        }
        Ok(cameras)
    }
}

#[test]
fn test_camera_import() {
    let gltf_data = r#"{
            "cameras" : [
                {
                "type": "perspective",
                "perspective": {
                    "aspectRatio": 1.0,
                    "yfov": 0.7,
                    "zfar": 100,
                    "znear": 0.01
                }
                },
                {
                "type": "orthographic",
                "orthographic": {
                    "xmag": 1.0,
                    "ymag": 1.0,
                    "zfar": 100,
                    "znear": 0.01
                }
                }
            ],
            "asset" : {
                "version" : "2.0"
            }
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let cameras = Gltf2Importer::import_cameras(&document).unwrap();
    assert_eq!(2, cameras.len());
}
