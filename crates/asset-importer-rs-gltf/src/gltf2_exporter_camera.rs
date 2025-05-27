use std::collections::HashMap;

use gltf::json::{
    Camera, Root,
    camera::{Orthographic, Perspective, Type},
    validation::Checked,
};

use asset_importer_rs_core::AiExportError;
use asset_importer_rs_scene::AiScene;

use super::gltf2_exporter::{Gltf2Exporter, generate_unique_name};

impl Gltf2Exporter {
    pub(crate) fn export_cameras(
        &self,
        scene: &AiScene,
        root: &mut Root,
        unique_names_map: &mut HashMap<String, u32>,
    ) -> Result<HashMap<String, usize>, AiExportError> {
        let mut camera_name_to_index: HashMap<String, usize> = HashMap::new();
        for (index, ai_camera) in scene.cameras.iter().enumerate() {
            let orthographic = if ai_camera.orthographic_width > 0.0 {
                Some(Orthographic {
                    xmag: ai_camera.orthographic_width,
                    ymag: if ai_camera.aspect_ratio != 0.0 {
                        (1.0 / ai_camera.aspect_ratio) * ai_camera.orthographic_width
                    } else {
                        1.0
                    },
                    zfar: ai_camera.far_plane,
                    znear: ai_camera.near_plane,
                    extensions: Default::default(),
                    extras: Default::default(),
                })
            } else {
                None
            };

            let perspective = if orthographic.is_some() {
                None
            } else {
                // hfov = 2 * atan(tan(yfov * 0.5) * aspect_ratio)
                // yfov = 2 * atan(tan(hfov * 0.5) / aspect_ratio)
                let yfov = 2.0
                    * f32::atan(
                        f32::tan(ai_camera.horizontal_fov * 0.5)
                            / (if ai_camera.aspect_ratio == 0.0 {
                                1.0
                            } else {
                                ai_camera.aspect_ratio
                            }),
                    );

                Some(Perspective {
                    aspect_ratio: if ai_camera.aspect_ratio != 0.0 {
                        Some(ai_camera.aspect_ratio)
                    } else {
                        None
                    },
                    yfov,
                    zfar: Some(ai_camera.far_plane),
                    znear: ai_camera.near_plane,
                    extensions: Default::default(),
                    extras: Default::default(),
                })
            };
            let is_some = orthographic.is_some();
            let name = generate_unique_name(&ai_camera.name, unique_names_map);
            camera_name_to_index.insert(name.clone(), index);
            let camera = Camera {
                name: Some(name),
                orthographic,
                perspective,
                type_: Checked::Valid(if is_some {
                    Type::Orthographic
                } else {
                    Type::Perspective
                }),
                extensions: Default::default(),
                extras: Default::default(),
            };
            root.cameras.push(camera);
        }
        Ok(camera_name_to_index)
    }
}
