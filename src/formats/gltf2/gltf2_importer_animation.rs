use gltf::{animation::Interpolation, buffer, Document};

use crate::{
    core::error::AiReadError,
    structs::{
        base_types::AiReal, AiAnimInterpolation, AiAnimation, AiMeshMorphAnim, AiMeshMorphKey,
        AiNodeAnim, AiQuatKey, AiQuaternion, AiVector3D, AiVectorKey,
    },
};

use super::{
    gltf2_importer::Gltf2Importer,
    gltf2_importer_mesh::{remap_data, GetPointer},
};

const MILLISECONDS_TO_SECONDS: f64 = 1000.0;

impl Gltf2Importer {
    pub(crate) fn import_animations(
        document: &Document,
        buffer_data: &[buffer::Data],
    ) -> Result<Vec<AiAnimation>, AiReadError> {
        let mut animations: Vec<AiAnimation> = Vec::new(); //Final Animations to return
        for animation in document.animations() {
            let mut ai_anim = AiAnimation {
                name: animation.name().unwrap_or("").to_string(),
                ticks_per_second: 1000.0,
                ..AiAnimation::default()
            };

            let asset_channels: Vec<gltf::animation::Channel<'_>> = animation.channels().collect();
            let mut duration = 0.0;
            for channel in asset_channels {
                let sampler = channel.sampler();
                let interpolation = sampler.interpolation();
                let input = sampler.input();
                let output = sampler.output();
                match channel.target().property() {
                    gltf::animation::Property::Translation => {
                        let mut ai_node_anim = AiNodeAnim::default();
                        let node = channel.target().node();
                        ai_node_anim.node_name = node
                            .name()
                            .unwrap_or(node.index().to_string().as_str())
                            .to_string();

                        let input_data = input
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let times = remap_data(None, input_data, 4, |chunk| {
                            f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                        });

                        let output_data = output
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let translation = remap_data(None, output_data, 12, |chunk| {
                            let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                as AiReal;
                            let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]])
                                as AiReal;
                            let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]])
                                as AiReal;
                            AiVector3D::new(x, y, z)
                        });
                        for i in 0..times.len() {
                            let time = times[i] as f64 * MILLISECONDS_TO_SECONDS;
                            if time > duration {
                                duration = time;
                            }
                            ai_node_anim.position_keys.push(AiVectorKey::new(
                                time,
                                translation[i],
                                match interpolation {
                                    gltf::animation::Interpolation::Linear => {
                                        AiAnimInterpolation::Linear
                                    }
                                    gltf::animation::Interpolation::Step => {
                                        AiAnimInterpolation::Step
                                    }
                                    gltf::animation::Interpolation::CubicSpline => {
                                        AiAnimInterpolation::CubicSpline
                                    }
                                },
                            ));
                        }
                        ai_anim.channels.push(ai_node_anim);
                    }
                    gltf::animation::Property::Rotation => {
                        let mut ai_node_anim = AiNodeAnim::default();
                        let node = channel.target().node();
                        ai_node_anim.node_name = node
                            .name()
                            .unwrap_or(node.index().to_string().as_str())
                            .to_string();

                        let input_data = input
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let times = remap_data(None, input_data, 4, |chunk| {
                            f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                        });

                        let output_data = output
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let rotation_opt = match output.data_type() {
                            gltf::accessor::DataType::I8 => {
                                Some(remap_data(None, output_data, 4, |chunk| {
                                    let x =
                                        (i8::from_le_bytes([chunk[0]]) as AiReal / 127.0).max(-1.0);
                                    let y =
                                        (i8::from_le_bytes([chunk[1]]) as AiReal / 127.0).max(-1.0);
                                    let z =
                                        (i8::from_le_bytes([chunk[2]]) as AiReal / 127.0).max(-1.0);
                                    let w =
                                        (i8::from_le_bytes([chunk[3]]) as AiReal / 127.0).max(-1.0);
                                    AiQuaternion::new(x, y, z, w)
                                }))
                            }
                            gltf::accessor::DataType::U8 => {
                                Some(remap_data(None, output_data, 4, |chunk| {
                                    let x = chunk[0] as AiReal / 255.0;
                                    let y = chunk[1] as AiReal / 255.0;
                                    let z = chunk[2] as AiReal / 255.0;
                                    let w = chunk[3] as AiReal / 255.0;
                                    AiQuaternion::new(x, y, z, w)
                                }))
                            }
                            gltf::accessor::DataType::I16 => {
                                Some(remap_data(None, output_data, 8, |chunk| {
                                    let x = (i16::from_le_bytes([chunk[0], chunk[1]]) as AiReal
                                        / 32767.0)
                                        .max(-1.0);
                                    let y = (i16::from_le_bytes([chunk[2], chunk[3]]) as AiReal
                                        / 32767.0)
                                        .max(-1.0);
                                    let z = (i16::from_le_bytes([chunk[4], chunk[5]]) as AiReal
                                        / 32767.0)
                                        .max(-1.0);
                                    let w = (i16::from_le_bytes([chunk[6], chunk[7]]) as AiReal
                                        / 32767.0)
                                        .max(-1.0);
                                    AiQuaternion::new(x, y, z, w)
                                }))
                            }
                            gltf::accessor::DataType::U16 => {
                                Some(remap_data(None, output_data, 8, |chunk| {
                                    let x = u16::from_le_bytes([chunk[0], chunk[1]]) as AiReal
                                        / 65535.0;
                                    let y = u16::from_le_bytes([chunk[2], chunk[3]]) as AiReal
                                        / 65535.0;
                                    let w = u16::from_le_bytes([chunk[4], chunk[5]]) as AiReal
                                        / 65535.0;
                                    let z = u16::from_le_bytes([chunk[6], chunk[7]]) as AiReal
                                        / 65535.0;
                                    AiQuaternion::new(x, y, z, w)
                                }))
                            }
                            gltf::accessor::DataType::F32 => {
                                Some(remap_data(None, output_data, 16, |chunk| {
                                    let x = f32::from_le_bytes([
                                        chunk[0], chunk[1], chunk[2], chunk[3],
                                    ]) as AiReal;
                                    let y = f32::from_le_bytes([
                                        chunk[4], chunk[5], chunk[6], chunk[7],
                                    ]) as AiReal;
                                    let z = f32::from_le_bytes([
                                        chunk[8], chunk[9], chunk[10], chunk[11],
                                    ]) as AiReal;
                                    let w = f32::from_le_bytes([
                                        chunk[12], chunk[13], chunk[14], chunk[15],
                                    ]) as AiReal;
                                    AiQuaternion::new(x, y, z, w)
                                }))
                            }
                            _ => None,
                        };

                        if let Some(rotation) = rotation_opt {
                            for i in 0..times.len() {
                                let time = times[i] as f64 * MILLISECONDS_TO_SECONDS;
                                if time > duration {
                                    duration = time;
                                }
                                ai_node_anim.rotation_keys.push(AiQuatKey::new(
                                    time,
                                    rotation[i],
                                    match interpolation {
                                        gltf::animation::Interpolation::Linear => {
                                            AiAnimInterpolation::Linear
                                        }
                                        gltf::animation::Interpolation::Step => {
                                            AiAnimInterpolation::Step
                                        }
                                        gltf::animation::Interpolation::CubicSpline => {
                                            AiAnimInterpolation::CubicSpline
                                        }
                                    },
                                ));
                            }
                        }
                        ai_anim.channels.push(ai_node_anim);
                    }
                    gltf::animation::Property::Scale => {
                        let mut ai_node_anim = AiNodeAnim::default();
                        let node = channel.target().node();
                        ai_node_anim.node_name = node
                            .name()
                            .unwrap_or(node.index().to_string().as_str())
                            .to_string();

                        let input_data = input
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let times = remap_data(None, input_data, 4, |chunk| {
                            f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                        });

                        let output_data = output
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let scale = remap_data(None, output_data, 12, |chunk| {
                            let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                as AiReal;
                            let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]])
                                as AiReal;
                            let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]])
                                as AiReal;
                            AiVector3D::new(x, y, z)
                        });
                        for i in 0..times.len() {
                            let time = times[i] as f64 * MILLISECONDS_TO_SECONDS;
                            if time > duration {
                                duration = time;
                            }
                            ai_node_anim.scaling_keys.push(AiVectorKey::new(
                                time,
                                scale[i],
                                match interpolation {
                                    gltf::animation::Interpolation::Linear => {
                                        AiAnimInterpolation::Linear
                                    }
                                    gltf::animation::Interpolation::Step => {
                                        AiAnimInterpolation::Step
                                    }
                                    gltf::animation::Interpolation::CubicSpline => {
                                        AiAnimInterpolation::CubicSpline
                                    }
                                },
                            ));
                        }
                        ai_anim.channels.push(ai_node_anim);
                    }
                    gltf::animation::Property::MorphTargetWeights => {
                        let mut ai_morph_anim = AiMeshMorphAnim::default();
                        let node = channel.target().node();
                        ai_morph_anim.name = node
                            .name()
                            .unwrap_or(node.index().to_string().as_str())
                            .to_string();

                        let input_data = input
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let times = remap_data(None, input_data, 4, |chunk| {
                            f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                        });

                        let output_data = output
                            .get_pointer(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        let values = match output.data_type() {
                            gltf::accessor::DataType::I8 => {
                                remap_data(None, output_data, 1, |chunk| {
                                    (i8::from_le_bytes([chunk[0]]) as AiReal / 127.0).max(-1.0)
                                })
                            }
                            gltf::accessor::DataType::U8 => {
                                remap_data(None, output_data, 1, |chunk| chunk[0] as AiReal / 255.0)
                            }
                            gltf::accessor::DataType::I16 => {
                                remap_data(None, output_data, 2, |chunk| {
                                    (i16::from_le_bytes([chunk[0], chunk[1]]) as AiReal / 32767.0)
                                        .max(-1.0)
                                })
                            }
                            gltf::accessor::DataType::U16 => {
                                remap_data(None, output_data, 2, |chunk| {
                                    u16::from_le_bytes([chunk[0], chunk[1]]) as AiReal / 65535.0
                                })
                            }
                            gltf::accessor::DataType::U32 | gltf::accessor::DataType::F32 => {
                                remap_data(None, output_data, 4, |chunk| {
                                    f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                        as AiReal
                                })
                            }
                        };
                        let stride = output.count() / times.len();
                        let num_morphs = if interpolation == Interpolation::CubicSpline {
                            stride - 2
                        } else {
                            stride
                        };
                        let offset = if interpolation == Interpolation::CubicSpline {
                            1
                        } else {
                            0
                        };
                        for (time_index, time_millis) in times.iter().enumerate() {
                            let mut mesh_morph_key = AiMeshMorphKey::default();
                            let time = *time_millis as f64 * MILLISECONDS_TO_SECONDS;
                            if time > duration {
                                duration = time;
                            }
                            mesh_morph_key.time = time;

                            let mut k = stride * time_index + offset;
                            for j in 0..num_morphs {
                                mesh_morph_key.values.push(j as u32);
                                mesh_morph_key.weights.push(if 0.0 > values[k] {
                                    0.0
                                } else {
                                    values[k] as f64
                                });
                                k += 1;
                            }
                            ai_morph_anim.keys.push(mesh_morph_key);
                        }

                        ai_anim.morph_channels.push(ai_morph_anim);
                    }
                }
            }
            ai_anim.duration = duration;

            animations.push(ai_anim);
        }
        Ok(animations)
    }
}
