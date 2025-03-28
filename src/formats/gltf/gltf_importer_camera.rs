use std::collections::HashMap;

use gltf_v1::{
    camera::{Camera, Projection},
    Document,
};

use crate::{
    core::error::AiReadError,
    structs::{AiCamera, AiVector3D},
};

use super::gltf_importer::GltfImporter;

pub struct ImportCameras(pub Vec<AiCamera>, pub HashMap<String, usize>);

impl GltfImporter {
    pub(crate) fn import_cameras(document: &Document) -> Result<ImportCameras, AiReadError> {
        let asset_cameras: Vec<Camera<'_>> = document.cameras().collect();
        let mut cameras: Vec<AiCamera> = Vec::with_capacity(asset_cameras.len());
        let mut camera_map: HashMap<String, usize> = HashMap::new();
        for camera in asset_cameras {
            let index = cameras.len();
            let name = camera
                .name()
                .map(|x| x.to_string())
                .unwrap_or(format!("{}", index));
            camera_map.insert(name.clone(), index);
            if camera_map.contains_key(&name) {
                return Err(AiReadError::FileFormatError(Box::new(
                    super::gltf_error::Error::DuplicateName,
                )));
            }
            camera_map.insert(name.clone(), index);
            let mut ai_camera = AiCamera {
                name,
                look_vec: AiVector3D::new(0.0, 0.0, -1.0),
                ..AiCamera::default()
            };
            match camera.projection() {
                Projection::Orthographic(orthographic) => {
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
                Projection::Perspective(perspective) => {
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
                    ai_camera.far_plane = perspective.zfar();
                }
            }
            cameras.push(ai_camera);
        }
        Ok(ImportCameras(cameras, camera_map))
    }
}

#[test]
fn test_gltf_camera_import() {
    let gltf_data = r#"{
            "cameras" : {
                "perpsective" :
                    {
                    "type": "perspective",
                    "perspective": {
                        "aspectRatio": 1.0,
                        "yfov": 0.7,
                        "zfar": 100,
                        "znear": 0.01
                    }
                },
                "orthographic": {
                "type": "orthographic",
                "orthographic": {
                    "xmag": 1.0,
                    "ymag": 1.0,
                    "zfar": 100,
                    "znear": 0.01
                }
                }
            }   
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let cameras = GltfImporter::import_cameras(&document).unwrap();
    assert_eq!(2, cameras.0.len());
}
