use std::collections::HashMap;

use gltf::{
    animation::{Interpolation, Property},
    json::{
        Animation, Index, Root,
        animation::{Channel, Sampler, Target},
        validation::Checked,
    },
};

use asset_importer_rs_scene::{AiQuaternion, AiScene, AiVector3D};

use crate::exporter::error::Gltf2ExportError;

use super::{
    exporter::{Gltf2Exporter, generate_unique_name},
    mesh::AccessorExporter,
};

impl Gltf2Exporter {
    pub(crate) fn export_animations(
        &self,
        scene: &AiScene,
        root: &mut Root,
        buffer_data: &mut Vec<u8>,
        unique_names_map: &mut HashMap<String, u32>,
    ) -> Result<(), Gltf2ExportError> {
        for anim in &scene.animations {
            let mut channels: Vec<Channel> = Vec::new();
            let mut samplers: Vec<Sampler> = Vec::new();

            for channel in &anim.channels {
                let anim_node = root
                    .nodes
                    .iter()
                    .enumerate()
                    .find(|(_, x)| x.name == Some(channel.node_name.clone()))
                    .map(|(x, _)| x);
                if let Some(index) = anim_node {
                    //handle position keys
                    if !channel.position_keys.is_empty() {
                        let num_key_frames = channel.position_keys.len();
                        let mut times: Vec<f32> = Vec::with_capacity(num_key_frames);
                        let mut values: Vec<AiVector3D> = Vec::with_capacity(num_key_frames);
                        for (i, key) in channel.position_keys.iter().enumerate() {
                            times[i] = (key.time / anim.ticks_per_second) as f32;
                            values[i] = key.value;
                        }
                        let input = AccessorExporter::export_real(root, buffer_data, &times);
                        let output = AccessorExporter::export_vector_3d(root, buffer_data, &values);
                        if input.is_none() || output.is_none() {
                            continue;
                        }
                        let sampler = Sampler {
                            interpolation: Checked::Valid(Interpolation::Linear),
                            input: root.push(input.unwrap()),
                            output: root.push(output.unwrap()),
                            extensions: Default::default(),
                            extras: Default::default(),
                        };

                        samplers.push(sampler);
                        let channel = Channel {
                            target: Target {
                                node: Index::new(index as u32),
                                path: Checked::Valid(Property::Translation),
                                extensions: Default::default(),
                                extras: Default::default(),
                            },
                            sampler: Index::new((samplers.len() - 1) as u32),
                            extensions: Default::default(),
                            extras: Default::default(),
                        };
                        channels.push(channel);
                    }

                    //handle rotation keys
                    if !channel.rotation_keys.is_empty() {
                        let num_key_frames = channel.rotation_keys.len();
                        let mut times: Vec<f32> = Vec::with_capacity(num_key_frames);
                        let mut values: Vec<AiQuaternion> = Vec::with_capacity(num_key_frames);
                        for (i, key) in channel.rotation_keys.iter().enumerate() {
                            times[i] = (key.time / anim.ticks_per_second) as f32;
                            values[i] = key.value;
                        }
                        let input = AccessorExporter::export_real(root, buffer_data, &times);
                        let output =
                            AccessorExporter::export_quaternion(root, buffer_data, &values);
                        if input.is_none() || output.is_none() {
                            continue;
                        }
                        let sampler = Sampler {
                            interpolation: Checked::Valid(Interpolation::Linear),
                            input: root.push(input.unwrap()),
                            output: root.push(output.unwrap()),
                            extensions: Default::default(),
                            extras: Default::default(),
                        };

                        samplers.push(sampler);
                        let channel = Channel {
                            target: Target {
                                node: Index::new(index as u32),
                                path: Checked::Valid(Property::Rotation),
                                extensions: Default::default(),
                                extras: Default::default(),
                            },
                            sampler: Index::new((samplers.len() - 1) as u32),
                            extensions: Default::default(),
                            extras: Default::default(),
                        };
                        channels.push(channel);
                    }

                    //handle scaling keys
                    if !channel.scaling_keys.is_empty() {
                        let num_key_frames = channel.scaling_keys.len();
                        let mut times: Vec<f32> = Vec::with_capacity(num_key_frames);
                        let mut values: Vec<AiVector3D> = Vec::with_capacity(num_key_frames);
                        for (i, key) in channel.scaling_keys.iter().enumerate() {
                            times[i] = (key.time / anim.ticks_per_second) as f32;
                            values[i] = key.value;
                        }
                        let input = AccessorExporter::export_real(root, buffer_data, &times);
                        let output = AccessorExporter::export_vector_3d(root, buffer_data, &values);
                        if input.is_none() || output.is_none() {
                            continue;
                        }
                        let sampler = Sampler {
                            interpolation: Checked::Valid(Interpolation::Linear),
                            input: root.push(input.unwrap()),
                            output: root.push(output.unwrap()),
                            extensions: Default::default(),
                            extras: Default::default(),
                        };

                        samplers.push(sampler);
                        let channel = Channel {
                            target: Target {
                                node: Index::new(index as u32),
                                path: Checked::Valid(Property::Scale),
                                extensions: Default::default(),
                                extras: Default::default(),
                            },
                            sampler: Index::new((samplers.len() - 1) as u32),
                            extensions: Default::default(),
                            extras: Default::default(),
                        };
                        channels.push(channel);
                    }
                }
            }
            let animation = Animation {
                name: Some(generate_unique_name(&anim.name, unique_names_map)),
                channels,
                samplers,
                extensions: Default::default(),
                extras: Default::default(),
            };
            root.animations.push(animation);
        }
        Ok(())
    }
}
