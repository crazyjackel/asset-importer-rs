use std::collections::HashMap;

use asset_importer_rs_scene::AiScene;
use gltf_v1::json::{
    Animation, Root, StringIndex,
    animation::{
        AnimationChannel, AnimationChannelTarget, AnimationPath, AnimationSampler,
        SamplerInterpolation,
    },
    map::IndexMap,
    validation::Checked,
};

use crate::exporter::{
    GltfExporter,
    error::Error,
    generate_unique_name,
    mesh::{export_data, export_float, export_vector_3d, export_vector_4d},
};

impl GltfExporter {
    pub fn export_animations(
        &self,
        scene: &AiScene,
        root: &mut Root,
        body_buffer_data: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let mut unique_names_map = HashMap::new();
        for i in 0..scene.animations.len() {
            let animation = &scene.animations[i];
            let mut base_animation_name = animation.name.clone();
            if base_animation_name.is_empty() {
                base_animation_name = "anim".to_string();
            }

            for j in 0..animation.channels.len() {
                let channel = &animation.channels[j];
                let animation_name = generate_unique_name(
                    format!("{}_{}", base_animation_name, j).as_str(),
                    &mut unique_names_map,
                );

                let mut parameters = IndexMap::new();
                let ticks = animation.ticks_per_second;

                let num_keyframes = channel
                    .position_keys
                    .len()
                    .max(channel.rotation_keys.len())
                    .max(channel.scaling_keys.len());

                if !channel.position_keys.is_empty() {
                    let mut time_data = Vec::with_capacity(num_keyframes);
                    for i in 0..num_keyframes {
                        let frame_index = i * channel.position_keys.len() / num_keyframes;
                        time_data.push((channel.position_keys[frame_index].time / ticks) as f32);
                    }
                    let accessor =
                        export_float(root, body_buffer_data, &time_data, &mut unique_names_map);
                    parameters.insert("TIME".to_string(), accessor);

                    let mut translation_data = Vec::with_capacity(num_keyframes);
                    for i in 0..num_keyframes {
                        let frame_index = i * channel.position_keys.len() / num_keyframes;
                        translation_data.push(channel.position_keys[frame_index].value);
                    }
                    let accessor = export_vector_3d(
                        root,
                        body_buffer_data,
                        &translation_data,
                        &mut unique_names_map,
                    );
                    parameters.insert("translation".to_string(), accessor);
                }

                if !channel.scaling_keys.is_empty() {
                    let mut scaling_data = Vec::with_capacity(num_keyframes);
                    for i in 0..num_keyframes {
                        let frame_index = i * channel.scaling_keys.len() / num_keyframes;
                        scaling_data.push(channel.scaling_keys[frame_index].value);
                    }
                    let accessor = export_vector_3d(
                        root,
                        body_buffer_data,
                        &scaling_data,
                        &mut unique_names_map,
                    );
                    parameters.insert("scale".to_string(), accessor);
                }

                if !channel.rotation_keys.is_empty() {
                    let mut rotation_data = Vec::with_capacity(num_keyframes);
                    for i in 0..num_keyframes {
                        let frame_index = i * channel.rotation_keys.len() / num_keyframes;
                        rotation_data.push(channel.rotation_keys[frame_index].value);
                    }
                    let accessor = export_vector_4d(
                        root,
                        body_buffer_data,
                        &rotation_data,
                        &mut unique_names_map,
                    );
                    parameters.insert("rotation".to_string(), accessor);
                }

                let mut channels = vec![];
                let mut samplers = IndexMap::new();
                for k in 0..3 {
                    let (channel_type, channel_size, channel_path) = match k {
                        0 => (
                            "translation",
                            channel.position_keys.len(),
                            AnimationPath::Translation,
                        ),
                        1 => (
                            "rotation",
                            channel.rotation_keys.len(),
                            AnimationPath::Rotation,
                        ),
                        2 => ("scale", channel.scaling_keys.len(), AnimationPath::Scale),
                        _ => unreachable!(),
                    };

                    if channel_size < 1 {
                        continue;
                    }

                    let sampler_name = format!("{}_{}", animation_name, channel_type);

                    let sampler = AnimationSampler {
                        input: "TIME".to_string(),
                        output: channel_type.to_string(),
                        interpolation: Some(Checked::Valid(SamplerInterpolation::Linear)),
                    };

                    let channel = AnimationChannel {
                        sampler: StringIndex::new(sampler_name.clone()),
                        target: AnimationChannelTarget {
                            id: StringIndex::new(channel.node_name.clone()),
                            path: Checked::Valid(channel_path),
                        },
                    };

                    channels.push(channel);
                    samplers.insert(sampler_name, sampler);
                }

                let animation = Animation {
                    parameters,
                    channels,
                    samplers,
                    name: Some(animation_name.clone()),
                };

                root.animations.insert(animation_name, animation);
            }
        }
        Ok(())
    }
}
