use std::collections::{HashMap, HashSet};

use asset_importer_rs_scene::{
    AiAnimInterpolation, AiAnimation, AiMatrix4x4, AiMeshMorphAnim, AiMeshMorphKey, AiNodeAnim,
    AiNodeTree, AiQuatKey, AiReal, AiVector3D, AiVectorKey,
};
use dae_parser::{
    Animation, AnimationClip, ArrayElement, Document, LocalMaps, Sampler, Semantic, Source,
};

use crate::DaeImportError;

use super::DaeImporter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TargetComponent {
    Whole,
    X,
    Y,
    Z,
    Angle,
    Index(usize),
    Matrix(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AnimationTarget<'a> {
    node: &'a str,
    property: &'a str,
    component: TargetComponent,
}

impl<'a> TryFrom<&'a str> for AnimationTarget<'a> {
    type Error = ();

    fn try_from(target: &'a str) -> Result<Self, Self::Error> {
        // Collada addresses animation targets as `node/property`, with optional
        // component selection:
        // - `node/translate` applies the complete sampled value.
        // - `node/translate.X` selects X, Y, Z, or ANGLE.
        // - `node/morph-weights(2)` selects one indexed array element.
        // - `node/matrix(2)(3)` selects one matrix element by row and column.
        // Targets with missing or additional path segments are unsupported.
        let (node, selector) = target.split_once('/').ok_or(())?;
        if node.is_empty() || selector.is_empty() || selector.contains('/') {
            return Err(());
        }

        let (property, component) = match (selector.find('('), selector.split_once('.')) {
            // Indexed array element or matrix element.
            (Some(open), _) => {
                let property = &selector[..open];
                let (first, rest) = selector[open + 1..].split_once(')').ok_or(())?;
                let first = first.parse::<usize>().map_err(|_| ())?;
                match rest {
                    "" => (property, TargetComponent::Index(first)),
                    rest => {
                        let rest = rest.strip_prefix('(').ok_or(())?;
                        let (second, rest) = rest.split_once(')').ok_or(())?;
                        let second = second.parse::<usize>().map_err(|_| ())?;
                        if !rest.is_empty() || first >= 4 || second >= 4 {
                            return Err(());
                        }
                        (property, TargetComponent::Matrix(second * 4 + first))
                    }
                }
            }
            // Single component: `property.X`, `.Y`, `.Z`, or `.ANGLE`.
            (None, Some((property, component))) => {
                let component = match component {
                    "X" => TargetComponent::X,
                    "Y" => TargetComponent::Y,
                    "Z" => TargetComponent::Z,
                    "ANGLE" => TargetComponent::Angle,
                    _ => return Err(()),
                };
                (property, component)
            }
            // Whole property: `translate`, `rotate`, `scale`, etc.
            (None, None) => (selector, TargetComponent::Whole),
        };
        if property.is_empty() {
            return Err(());
        }

        Ok(Self {
            node,
            property,
            component,
        })
    }
}

#[allow(dead_code)]
struct ChannelEntry<'a> {
    target: AnimationTarget<'a>,
    node_index: usize,
    sampler: &'a Sampler,
    time_source: &'a Source,
    value_source: &'a Source,
}

#[allow(dead_code)]
struct NodeChannelEntries<'a> {
    entries: Vec<ChannelEntry<'a>>,
    start_time: f32,
    end_time: f32,
}

impl DaeImporter {
    pub(crate) fn import_animations(
        &self,
        document: &Document,
        nodes: &AiNodeTree,
        node_index_map: &HashMap<String, usize>,
    ) -> Result<Vec<AiAnimation>, DaeImportError> {
        // Collect maps from document.
        let maps = LocalMaps::default()
            .set::<Animation>()
            .set::<AnimationClip>()
            .set::<Sampler>()
            .set::<Source>()
            .collect(document);
        let anim_map = maps.get_map::<Animation>().expect("animation map enabled");
        let clip_map = maps
            .get_map::<AnimationClip>()
            .expect("animation clip map enabled");

        // Seed the stack with animations from library.
        let mut stack: Vec<(&dae_parser::Animation, String)> = Vec::new();
        if clip_map.0.is_empty() {
            // No clips, we only need to seed the animations instead of their views
            for lib in document.library_iter::<dae_parser::Animation>() {
                for anim in &lib.items {
                    stack.push((anim, String::new()));
                }
            }
        } else {
            // Clips, we need to seed the animations of their views
            for (index, clip) in clip_map.0.values().enumerate() {
                let parent_name = clip
                    .name
                    .as_ref()
                    .filter(|s| !s.is_empty())
                    .cloned()
                    .or_else(|| clip.id.clone())
                    .unwrap_or_else(|| format!("animation_{index}"));
                for instance in &clip.instance_animation {
                    if let Some(anim) = anim_map.get(&instance.url) {
                        stack.push((anim, parent_name.clone()));
                    }
                }
            }
        }

        // Process the stack to create animations.
        let mut anims = Vec::new();
        while let Some((src, mut name)) = stack.pop() {
            // Generate the name of the animation.
            let local_name = src
                .name
                .as_deref()
                .filter(|s| !s.is_empty())
                .unwrap_or("animation");
            if !name.is_empty() {
                name.push('_');
            }
            name.push_str(local_name);

            // Recursively process the children.
            for child in &src.children {
                stack.push((child, name.clone()));
            }

            // Create the animation if it has channels.
            if !src.channel.is_empty() {
                if let Some(anim) = create_animation(src, &name, nodes, node_index_map, &maps)? {
                    anims.push(anim);
                }
            }
        }

        // When processing the clips, we may have created duplicate animations with the same name.
        // We need to combine them into a single animation.
        combine_single_channel_ai_anims(&mut anims);
        Ok(anims)
    }
}

/// Morph-weight animation is not implemented yet.
fn create_animation(
    src: &Animation,
    name: &str,
    nodes: &AiNodeTree,
    node_index_map: &HashMap<String, usize>,
    maps: &LocalMaps<'_>,
) -> Result<Option<AiAnimation>, DaeImportError> {
    // Resolve all channel entries before grouping them by scene node.
    let mut entries = Vec::new();
    for channel in &src.channel {
        // Parse the target of the channel.
        let Ok(target) = AnimationTarget::try_from(channel.target.0.as_str()) else {
            continue;
        };
        let Some(&node_index) = node_index_map.get(target.node) else {
            continue;
        };
        if nodes.arena.get(node_index).is_none() {
            continue;
        }
        // Get the sampler for the channel.
        let Some(sampler) = maps.get(&channel.source) else {
            continue;
        };
        // Get the time input for the channel.
        let Some(time_input) = sampler
            .inputs
            .iter()
            .find(|input| input.semantic == Semantic::Input)
        else {
            continue;
        };
        // Get the value input for the channel.
        let Some(value_input) = sampler
            .inputs
            .iter()
            .find(|input| input.semantic == Semantic::Output)
        else {
            continue;
        };
        let (Some(time_source), Some(value_source)) = (
            maps.get_raw::<Source>(&time_input.source),
            maps.get_raw::<Source>(&value_input.source),
        ) else {
            continue;
        };
        entries.push(ChannelEntry {
            target,
            node_index,
            sampler,
            time_source,
            value_source,
        });
    }

    if entries.is_empty() {
        return Ok(None);
    }

    // Collect the time and value data for each source.
    let mut source_data: HashMap<&str, Vec<f32>> = HashMap::new();
    let mut entries_by_node: HashMap<usize, NodeChannelEntries<'_>> = HashMap::new();
    'entries: for entry in entries {
        if entry.time_source.accessor.count != entry.value_source.accessor.count {
            return Err(DaeImportError::InvalidAnimation(format!(
                "time/value count mismatch for channel '{}/{}': {} != {}",
                entry.target.node,
                entry.target.property,
                entry.time_source.accessor.count,
                entry.value_source.accessor.count,
            )));
        }
        // Get the time source ID.
        let time_source = entry.time_source;
        let Some(time_source_id) = time_source.id.as_deref() else {
            continue;
        };
        if time_source.accessor.count == 0 || time_source.accessor.stride == 0 {
            continue;
        }
        let mut start_time = f32::INFINITY;
        let mut end_time = f32::NEG_INFINITY;
        if let Some(times) = source_data.get(time_source_id) {
            for time in times.iter().step_by(time_source.accessor.stride) {
                start_time = start_time.min(*time);
                end_time = end_time.max(*time);
            }
        } else {
            let Some(ArrayElement::Float(data)) = &time_source.array else {
                continue;
            };
            let mut times =
                Vec::with_capacity(time_source.accessor.count * time_source.accessor.stride);
            for index in 0..time_source.accessor.count {
                let start = time_source.accessor.offset + index * time_source.accessor.stride;
                let Some(time) = data.get(start..start + time_source.accessor.stride) else {
                    continue 'entries;
                };
                start_time = start_time.min(time[0]);
                end_time = end_time.max(time[0]);
                times.extend_from_slice(time);
            }
            source_data.insert(time_source_id, times);
        }

        let value_source = entry.value_source;
        let Some(value_source_id) = value_source.id.as_deref() else {
            continue;
        };
        if value_source.accessor.stride == 0 {
            continue;
        }
        if !source_data.contains_key(value_source_id) {
            let Some(ArrayElement::Float(data)) = &value_source.array else {
                continue;
            };
            let mut values =
                Vec::with_capacity(value_source.accessor.count * value_source.accessor.stride);
            for index in 0..value_source.accessor.count {
                let start = value_source.accessor.offset + index * value_source.accessor.stride;
                let Some(value) = data.get(start..start + value_source.accessor.stride) else {
                    continue 'entries;
                };
                values.extend_from_slice(value);
            }
            source_data.insert(value_source_id, values);
        }

        let group = entries_by_node
            .entry(entry.node_index)
            .or_insert_with(|| NodeChannelEntries {
                entries: Vec::new(),
                start_time,
                end_time,
            });
        group.start_time = group.start_time.min(start_time);
        group.end_time = group.end_time.max(end_time);
        group.entries.push(entry);
    }

    if entries_by_node.is_empty() {
        return Ok(None);
    }

    let mut node_anims = Vec::with_capacity(entries_by_node.len());
    let mut morph_anims = Vec::new();
    let mut duration = 0.0_f64;
    for (node_index, group) in entries_by_node {
        let Some(node) = nodes.arena.get(node_index) else {
            continue;
        };

        // Evaluate at every unique key time used by any channel for this node.
        let mut evaluation_times = Vec::new();
        for entry in &group.entries {
            let Some(time_source_id) = entry.time_source.id.as_deref() else {
                continue;
            };
            let Some(times) = source_data.get(time_source_id) else {
                continue;
            };
            for index in 0..entry.time_source.accessor.count {
                evaluation_times.push(times[index * entry.time_source.accessor.stride]);
            }

            // Axis-angle interpolation can take the long path through a rotation. Add
            // intermediate samples so consecutive quaternion keys remain below 180 degrees.
            if entry.target.component == TargetComponent::Angle {
                let Some(value_source_id) = entry.value_source.id.as_deref() else {
                    continue;
                };
                let Some(values) = source_data.get(value_source_id) else {
                    continue;
                };
                for index in 1..entry.time_source.accessor.count {
                    let previous_angle = values[(index - 1) * entry.value_source.accessor.stride];
                    let angle = values[index * entry.value_source.accessor.stride];
                    let delta = (angle - previous_angle).abs();
                    if delta < 180.0 {
                        continue;
                    }

                    // Add intermediate samples so consecutive quaternion keys remain below 180 degrees.
                    let previous_time = times[(index - 1) * entry.time_source.accessor.stride];
                    let time = times[index * entry.time_source.accessor.stride];
                    let sample_count = (delta / 90.0).floor() as usize;
                    for sample in 1..sample_count {
                        evaluation_times.push(
                            previous_time
                                + (time - previous_time) * sample as f32 / sample_count as f32,
                        );
                    }
                }
            }
        }
        evaluation_times.sort_unstable_by(f32::total_cmp);
        evaluation_times.dedup();
        if evaluation_times.is_empty() {
            continue;
        }
        duration = duration.max(evaluation_times[evaluation_times.len() - 1] as f64 * 1000.0);

        let mut position_keys = Vec::with_capacity(evaluation_times.len());
        let mut rotation_keys = Vec::with_capacity(evaluation_times.len());
        let mut scaling_keys = Vec::with_capacity(evaluation_times.len());
        let mut morph_keys = Vec::with_capacity(evaluation_times.len());
        for time in evaluation_times {
            let mut matrix = node.transformation.clone();
            let mut weights = Vec::new();
            let mut has_transforms = false;

            for entry in &group.entries {
                let (Some(time_source_id), Some(value_source_id)) = (
                    entry.time_source.id.as_deref(),
                    entry.value_source.id.as_deref(),
                ) else {
                    continue;
                };
                let (Some(times), Some(values)) = (
                    source_data.get(time_source_id),
                    source_data.get(value_source_id),
                ) else {
                    continue;
                };

                // Find the first key at or after the evaluation time.
                let mut post_index = 0;
                while post_index < entry.time_source.accessor.count
                    && times[post_index * entry.time_source.accessor.stride] < time
                {
                    post_index += 1;
                }
                post_index = post_index.min(entry.time_source.accessor.count - 1);
                let post_time = times[post_index * entry.time_source.accessor.stride];
                let value_stride = entry.value_source.accessor.stride;
                let value_start = post_index * value_stride;
                let mut sampled_values = values[value_start..value_start + value_stride].to_vec();

                // Linearly interpolate between the surrounding keys.
                if post_time > time && post_index > 0 {
                    let pre_index = post_index - 1;
                    let pre_time = times[pre_index * entry.time_source.accessor.stride];
                    let factor = (time - pre_time) / (post_time - pre_time);
                    let pre_start = pre_index * value_stride;
                    for component in 0..value_stride {
                        let pre_value = values[pre_start + component];
                        sampled_values[component] =
                            pre_value + (sampled_values[component] - pre_value) * factor;
                    }
                }

                let property = entry.target.property.to_ascii_lowercase();
                if property.contains("morph-weights") {
                    weights.push(sampled_values[0] as f64);
                    continue;
                }

                has_transforms = true;
                match property.as_str() {
                    property if property.contains("matrix") || property == "transform" => {
                        // Set the matrix in its entirety.
                        let mut elements: [AiReal; 16] = matrix.clone().into();
                        match entry.target.component {
                            TargetComponent::Whole if sampled_values.len() >= 16 => {
                                for (element, value) in
                                    elements.iter_mut().zip(&sampled_values[..16])
                                {
                                    *element = *value as AiReal;
                                }
                            }
                            TargetComponent::Matrix(index) => {
                                elements[index] = sampled_values[0] as AiReal;
                            }
                            _ => continue,
                        }
                        matrix = AiMatrix4x4::from(elements);
                    }
                    property
                        if property.contains("translate")
                            || property.contains("translation")
                            || property.contains("location") =>
                    {
                        // Set the translation in its entirety.
                        match entry.target.component {
                            TargetComponent::Whole if sampled_values.len() >= 3 => {
                                matrix.a4 = sampled_values[0] as AiReal;
                                matrix.b4 = sampled_values[1] as AiReal;
                                matrix.c4 = sampled_values[2] as AiReal;
                            }
                            TargetComponent::X => matrix.a4 = sampled_values[0] as AiReal,
                            TargetComponent::Y => matrix.b4 = sampled_values[0] as AiReal,
                            TargetComponent::Z => matrix.c4 = sampled_values[0] as AiReal,
                            _ => continue,
                        }
                    }
                    property if property.contains("scale") => {
                        // Scale the matrix in its entirety.
                        let current_scale = matrix.decompose().scale;
                        let mut scale = current_scale;
                        match entry.target.component {
                            TargetComponent::Whole if sampled_values.len() >= 3 => {
                                scale.x = sampled_values[0] as AiReal;
                                scale.y = sampled_values[1] as AiReal;
                                scale.z = sampled_values[2] as AiReal;
                            }
                            TargetComponent::X => scale.x = sampled_values[0] as AiReal,
                            TargetComponent::Y => scale.y = sampled_values[0] as AiReal,
                            TargetComponent::Z => scale.z = sampled_values[0] as AiReal,
                            _ => continue,
                        }

                        for (current, desired, column) in [
                            (current_scale.x, scale.x, [0usize, 4, 8]),
                            (current_scale.y, scale.y, [1usize, 5, 9]),
                            (current_scale.z, scale.z, [2usize, 6, 10]),
                        ] {
                            let mut elements: [AiReal; 16] = matrix.clone().into();
                            if current != 0.0 {
                                let factor = desired / current;
                                for index in column {
                                    elements[index] *= factor;
                                }
                            } else {
                                for index in column {
                                    elements[index] = 0.0;
                                }
                                elements[column[0]] = desired;
                            }
                            matrix = AiMatrix4x4::from(elements);
                        }
                    }
                    property if property.contains("rotate") || property.contains("rotation") => {
                        let (axis, angle) = match entry.target.component {
                            TargetComponent::Whole if sampled_values.len() >= 4 => (
                                AiVector3D::new(
                                    sampled_values[0] as AiReal,
                                    sampled_values[1] as AiReal,
                                    sampled_values[2] as AiReal,
                                ),
                                sampled_values[3],
                            ),
                            TargetComponent::Angle if property.contains('x') => {
                                (AiVector3D::new(1.0, 0.0, 0.0), sampled_values[0])
                            }
                            TargetComponent::Angle if property.contains('y') => {
                                (AiVector3D::new(0.0, 1.0, 0.0), sampled_values[0])
                            }
                            TargetComponent::Angle if property.contains('z') => {
                                (AiVector3D::new(0.0, 0.0, 1.0), sampled_values[0])
                            }
                            _ => continue,
                        };
                        let translation = matrix.decompose().translation;
                        let scale = matrix.decompose().scale;
                        matrix = AiMatrix4x4::rotation((angle as AiReal).to_radians(), &axis);
                        matrix.a1 *= scale.x;
                        matrix.b1 *= scale.x;
                        matrix.c1 *= scale.x;
                        matrix.a2 *= scale.y;
                        matrix.b2 *= scale.y;
                        matrix.c2 *= scale.y;
                        matrix.a3 *= scale.z;
                        matrix.b3 *= scale.z;
                        matrix.c3 *= scale.z;
                        matrix.a4 = translation.x;
                        matrix.b4 = translation.y;
                        matrix.c4 = translation.z;
                    }
                    _ => continue,
                }
            }

            let key_time = time as f64 * 1000.0;
            if has_transforms {
                let decomposed = matrix.decompose();
                position_keys.push(AiVectorKey::new(
                    key_time,
                    decomposed.translation,
                    AiAnimInterpolation::Linear,
                ));
                rotation_keys.push(AiQuatKey::new(
                    key_time,
                    decomposed.rotation,
                    AiAnimInterpolation::Linear,
                ));
                scaling_keys.push(AiVectorKey::new(
                    key_time,
                    decomposed.scale,
                    AiAnimInterpolation::Linear,
                ));
            }
            if !weights.is_empty() {
                morph_keys.push(AiMeshMorphKey {
                    time: key_time,
                    values: (0..weights.len() as u32).collect(),
                    weights,
                });
            }
        }

        if !position_keys.is_empty() {
            node_anims.push(AiNodeAnim {
                node_name: node.name.clone(),
                position_keys,
                rotation_keys,
                scaling_keys,
                ..AiNodeAnim::default()
            });
        }
        if !morph_keys.is_empty() {
            morph_anims.push(AiMeshMorphAnim {
                name: node.name.clone(),
                keys: morph_keys,
            });
        }
    }

    if node_anims.is_empty() && morph_anims.is_empty() {
        return Ok(None);
    }

    Ok(Some(AiAnimation {
        name: name.to_string(),
        duration,
        ticks_per_second: 1000.0,
        channels: node_anims,
        morph_channels: morph_anims,
        ..AiAnimation::default()
    }))
}

fn combine_single_channel_ai_anims(anims: &mut Vec<AiAnimation>) {
    let mut delete_indices = Vec::new();
    for a in 0..anims.len() {
        // Skip if the animation has more than one channel.
        if anims[a].channels.len() != 1 || !anims[a].morph_channels.is_empty() {
            continue;
        }

        // Collect all animations with the same duration and ticks per second.
        let duration = anims[a].duration;
        let ticks_per_second = anims[a].ticks_per_second;
        let mut collected = Vec::new();
        for b in a + 1..anims.len() {
            if anims[b].channels.len() == 1
                && anims[b].morph_channels.is_empty()
                && anims[b].duration == duration
                && anims[b].ticks_per_second == ticks_per_second
            {
                collected.push(b);
            }
        }

        // Check if the animations have the same target node.
        let mut targets = HashSet::new();
        targets.insert(anims[a].channels[0].node_name.clone());
        let mut different_channels = true;
        for &index in &collected {
            if !targets.insert(anims[index].channels[0].node_name.clone()) {
                different_channels = false;
                break;
            }
        }
        if !different_channels || collected.is_empty() {
            continue;
        }

        // Combine the animations into a single animation.
        let mut combined = AiAnimation {
            name: format!("combinedAnim_{a}"),
            duration,
            ticks_per_second,
            channels: Vec::with_capacity(collected.len() + 1),
            ..AiAnimation::default()
        };
        // Add the channels from the original animation.
        // We take the channels so as to skip them on loop.
        combined
            .channels
            .extend(std::mem::take(&mut anims[a].channels));
        for &index in &collected {
            combined
                .channels
                .extend(std::mem::take(&mut anims[index].channels));
        }
        anims[a] = combined;
        // Mark the collected animations for deletion.
        delete_indices.extend(collected);
    }

    // Delete the collected animations.
    delete_indices.sort_unstable();
    for index in delete_indices.into_iter().rev() {
        anims.remove(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asset_importer_rs_scene::AiNodeAnim;
    use std::str::FromStr;

    fn document_with(body: &str) -> Document {
        let xml = format!(
            r##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <created>1970-01-01T00:00:00Z</created>
    <modified>1970-01-01T00:00:00Z</modified>
  </asset>
  {body}
</COLLADA>"##
        );
        Document::from_str(&xml).expect("document should parse")
    }

    fn walk_names(document: &Document) -> Vec<String> {
        let anim_map = document
            .local_map::<dae_parser::Animation>()
            .expect("animation map");
        let clip_map = document.local_map::<AnimationClip>().expect("clip map");
        let mut names = Vec::new();
        let mut stack: Vec<(&dae_parser::Animation, String)> = Vec::new();
        if clip_map.0.is_empty() {
            for lib in document.library_iter::<dae_parser::Animation>() {
                for anim in &lib.items {
                    stack.push((anim, String::new()));
                }
            }
        } else {
            for (index, clip) in clip_map.0.values().enumerate() {
                let parent_name = clip
                    .name
                    .as_ref()
                    .filter(|s| !s.is_empty())
                    .cloned()
                    .or_else(|| clip.id.clone())
                    .unwrap_or_else(|| format!("animation_{index}"));
                for instance in &clip.instance_animation {
                    if let Some(anim) = anim_map.get(&instance.url) {
                        stack.push((anim, parent_name.clone()));
                    }
                }
            }
        }
        while let Some((src, mut name)) = stack.pop() {
            let local_name = src
                .name
                .as_deref()
                .filter(|s| !s.is_empty())
                .unwrap_or("animation");
            if !name.is_empty() {
                name.push('_');
            }
            name.push_str(local_name);
            for child in &src.children {
                stack.push((child, name.clone()));
            }
            if !src.channel.is_empty() {
                names.push(name);
            }
        }
        names
    }

    fn sampler_sources(id: &str, times: &str, values: &str) -> String {
        let times_array = format!("{times}-array");
        let times_src = format!("#{times}");
        let times_array_src = format!("#{times_array}");
        let values_array = format!("{values}-array");
        let values_src = format!("#{values}");
        let values_array_src = format!("#{values_array}");
        let interp = format!("{id}-interp");
        let interp_array = format!("{interp}-array");
        let interp_src = format!("#{interp}");
        let interp_array_src = format!("#{interp_array}");
        format!(
            r#"
      <source id="{times}">
        <float_array id="{times_array}" count="2">0 1</float_array>
        <technique_common>
          <accessor source="{times_array_src}" count="2">
            <param name="TIME" type="float"/>
          </accessor>
        </technique_common>
      </source>
      <source id="{values}">
        <float_array id="{values_array}" count="6">0 0 0 1 0 0</float_array>
        <technique_common>
          <accessor source="{values_array_src}" count="2" stride="3">
            <param name="X" type="float"/>
            <param name="Y" type="float"/>
            <param name="Z" type="float"/>
          </accessor>
        </technique_common>
      </source>
      <source id="{interp}">
        <Name_array id="{interp_array}" count="2">LINEAR LINEAR</Name_array>
        <technique_common>
          <accessor source="{interp_array_src}" count="2">
            <param name="INTERPOLATION" type="Name"/>
          </accessor>
        </technique_common>
      </source>
      <sampler id="{id}">
        <input semantic="INPUT" source="{times_src}"/>
        <input semantic="OUTPUT" source="{values_src}"/>
        <input semantic="INTERPOLATION" source="{interp_src}"/>
      </sampler>"#
        )
    }

    fn node_anim(node_name: &str) -> AiNodeAnim {
        AiNodeAnim {
            node_name: node_name.to_string(),
            ..AiNodeAnim::default()
        }
    }

    fn single_channel_ai(name: &str, node_name: &str, duration: f64) -> AiAnimation {
        AiAnimation {
            name: name.to_string(),
            duration,
            ticks_per_second: 1000.0,
            channels: vec![node_anim(node_name)],
            ..AiAnimation::default()
        }
    }

    #[test]
    fn parses_channel_target() {
        let channel_src = "#samp";
        let channel_target = "Root/translate";
        let document = document_with(&format!(
            r#"
  <library_animations>
    <animation id="move" name="Move">
      {sources}
      <channel source="{channel_src}" target="{channel_target}"/>
    </animation>
  </library_animations>"#,
            sources = sampler_sources("samp", "times", "values")
        ));
        let src = document
            .library_iter::<dae_parser::Animation>()
            .next()
            .unwrap()
            .items
            .first()
            .unwrap();
        let target = AnimationTarget::try_from(src.channel[0].target.0.as_str()).unwrap();
        assert_eq!(
            target,
            AnimationTarget {
                node: "Root",
                property: "translate",
                component: TargetComponent::Whole,
            }
        );
    }

    #[test]
    fn parses_channel_target_components() {
        assert_eq!(
            AnimationTarget::try_from("Root/location.X")
                .unwrap()
                .component,
            TargetComponent::X
        );
        assert_eq!(
            AnimationTarget::try_from("Root/rotation.ANGLE")
                .unwrap()
                .component,
            TargetComponent::Angle
        );
        assert_eq!(
            AnimationTarget::try_from("Root/matrix(2)(3)")
                .unwrap()
                .component,
            TargetComponent::Matrix(14)
        );
        assert_eq!(
            AnimationTarget::try_from("Root/morph-weights(2)")
                .unwrap()
                .component,
            TargetComponent::Index(2)
        );
        assert!(AnimationTarget::try_from("Root/location.W").is_err());
    }

    #[test]
    fn prefixes_nested_animation_names_during_store() {
        let channel_src = "#samp";
        let channel_target = "Root/translate";
        let document = document_with(&format!(
            r#"
  <library_animations>
    <animation name="Parent">
      <animation name="Child">
        {sources}
        <channel source="{channel_src}" target="{channel_target}"/>
      </animation>
    </animation>
  </library_animations>"#,
            sources = sampler_sources("samp", "times", "values")
        ));
        assert_eq!(walk_names(&document), vec!["Parent_Child"]);
    }

    #[test]
    fn clips_seed_from_clip_map_and_keep_nested_children() {
        let samp_a = "#samp_a";
        let samp_b = "#samp_b";
        let target_a = "Root/translate";
        let target_b = "Child/translate";
        let clip_a = "#move_a";
        let document = document_with(&format!(
            r#"
  <library_animations>
    <animation id="move_a" name="MoveA">
      <animation name="Nested">
        {b}
        <channel source="{samp_b}" target="{target_b}"/>
      </animation>
      {a}
      <channel source="{samp_a}" target="{target_a}"/>
    </animation>
  </library_animations>
  <library_animation_clips>
    <animation_clip id="clip0" name="Clip">
      <instance_animation url="{clip_a}"/>
    </animation_clip>
  </library_animation_clips>"#,
            a = sampler_sources("samp_a", "times_a", "values_a"),
            b = sampler_sources("samp_b", "times_b", "values_b")
        ));
        let names = walk_names(&document);
        assert!(names.contains(&"Clip_MoveA".to_string()));
        assert!(names.contains(&"Clip_MoveA_Nested".to_string()));
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn combines_single_channel_ai_animations_with_distinct_nodes() {
        let mut anims = vec![
            single_channel_ai("one", "Root", 1000.0),
            single_channel_ai("two", "Child", 1000.0),
        ];
        combine_single_channel_ai_anims(&mut anims);
        assert_eq!(anims.len(), 1);
        assert_eq!(anims[0].name, "combinedAnim_0");
        assert_eq!(anims[0].duration, 1000.0);
        assert_eq!(anims[0].ticks_per_second, 1000.0);
        assert_eq!(
            anims[0]
                .channels
                .iter()
                .map(|channel| channel.node_name.as_str())
                .collect::<Vec<_>>(),
            vec!["Root", "Child"]
        );
    }

    #[test]
    fn deletes_collected_animations_after_combining_all_groups() {
        let mut anims = vec![
            single_channel_ai("one", "Root", 1000.0),
            single_channel_ai("two", "Arm", 2000.0),
            single_channel_ai("three", "Leg", 2000.0),
            single_channel_ai("four", "Child", 1000.0),
        ];
        combine_single_channel_ai_anims(&mut anims);
        assert_eq!(anims.len(), 2);
        assert_eq!(anims[0].name, "combinedAnim_0");
        assert_eq!(anims[1].name, "combinedAnim_1");
    }

    #[test]
    fn does_not_combine_ai_animations_that_share_a_node() {
        let mut anims = vec![
            single_channel_ai("one", "Root", 1000.0),
            single_channel_ai("two", "Root", 1000.0),
        ];
        combine_single_channel_ai_anims(&mut anims);
        assert_eq!(anims.len(), 2);
    }

    #[test]
    fn ignores_animation_channels_with_unknown_scene_nodes() {
        let channel_src = "#samp";
        let channel_target = "Root/translate";
        let document = document_with(&format!(
            r#"
  <library_animations>
    <animation name="Move">
      {sources}
      <channel source="{channel_src}" target="{channel_target}"/>
    </animation>
  </library_animations>"#,
            sources = sampler_sources("samp", "times", "values")
        ));
        let anims = DaeImporter::new()
            .import_animations(&document, &AiNodeTree::default(), &HashMap::new())
            .expect("import");
        assert!(anims.is_empty());
    }

    #[test]
    fn creates_node_animation_keys_from_translation_channel() {
        let document = document_with(&format!(
            r##"
  <library_animations>
    <animation name="Move">
      {sources}
      <channel source="#samp" target="Root/translate"/>
    </animation>
  </library_animations>"##,
            sources = sampler_sources("samp", "times", "values")
        ));
        let mut nodes = AiNodeTree::with_root();
        nodes.arena[0].name = "Root".to_string();

        let anims = DaeImporter::new()
            .import_animations(&document, &nodes, &HashMap::from([("Root".to_string(), 0)]))
            .expect("import");

        assert_eq!(anims.len(), 1);
        assert_eq!(anims[0].name, "Move");
        assert_eq!(anims[0].duration, 1000.0);
        assert_eq!(anims[0].ticks_per_second, 1000.0);
        assert_eq!(anims[0].channels.len(), 1);
        assert_eq!(anims[0].channels[0].node_name, "Root");
        assert_eq!(anims[0].channels[0].position_keys.len(), 2);
        assert_eq!(anims[0].channels[0].position_keys[0].time, 0.0);
        assert_eq!(anims[0].channels[0].position_keys[1].time, 1000.0);
        assert_eq!(anims[0].channels[0].position_keys[0].value.x, 0.0);
        assert_eq!(anims[0].channels[0].position_keys[1].value.x, 1.0);
    }

    #[test]
    fn creates_scaling_keys_from_scale_channel() {
        let sources = sampler_sources("samp", "times", "values")
            .replace(">0 0 0 1 0 0</float_array>", ">1 1 1 2 3 4</float_array>");
        let document = document_with(&format!(
            r##"
  <library_animations>
    <animation name="Scale">
      {sources}
      <channel source="#samp" target="Root/scale"/>
    </animation>
  </library_animations>"##
        ));
        let mut nodes = AiNodeTree::with_root();
        nodes.arena[0].name = "Root".to_string();

        let anims = DaeImporter::new()
            .import_animations(&document, &nodes, &HashMap::from([("Root".to_string(), 0)]))
            .expect("import");
        let scaling_keys = &anims[0].channels[0].scaling_keys;

        assert_eq!(scaling_keys.len(), 2);
        assert_eq!(scaling_keys[0].value, AiVector3D::new(1.0, 1.0, 1.0));
        assert_eq!(scaling_keys[1].value, AiVector3D::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn interpolates_channels_at_other_channel_key_times() {
        let translate = sampler_sources("translate", "translate-times", "translate-values");
        let scale = sampler_sources("scale", "scale-times", "scale-values")
            .replace(">0 1</float_array>", ">0.5 1</float_array>")
            .replace(">0 0 0 1 0 0</float_array>", ">1 1 1 2 2 2</float_array>");
        let (translate_sources, translate_sampler) =
            translate.split_at(translate.find("<sampler").expect("translate sampler"));
        let (scale_sources, scale_sampler) =
            scale.split_at(scale.find("<sampler").expect("scale sampler"));
        let document = document_with(&format!(
            r##"
  <library_animations>
    <animation name="MoveAndScale">
      {translate_sources}
      {scale_sources}
      {translate_sampler}
      {scale_sampler}
      <channel source="#translate" target="Root/translate"/>
      <channel source="#scale" target="Root/scale"/>
    </animation>
  </library_animations>"##
        ));
        let mut nodes = AiNodeTree::with_root();
        nodes.arena[0].name = "Root".to_string();

        let anims = DaeImporter::new()
            .import_animations(&document, &nodes, &HashMap::from([("Root".to_string(), 0)]))
            .expect("import");
        let position_keys = &anims[0].channels[0].position_keys;

        assert_eq!(position_keys.len(), 3);
        assert_eq!(position_keys[1].time, 500.0);
        assert_eq!(position_keys[1].value.x, 0.5);
    }

    #[test]
    fn creates_morph_animation_keys() {
        let document = document_with(&format!(
            r##"
  <library_animations>
    <animation name="Morph">
      {sources}
      <channel source="#samp" target="Face/morph-weights(0)"/>
    </animation>
  </library_animations>"##,
            sources = sampler_sources("samp", "times", "values")
        ));
        let mut nodes = AiNodeTree::with_root();
        nodes.arena[0].name = "Face".to_string();

        let anims = DaeImporter::new()
            .import_animations(&document, &nodes, &HashMap::from([("Face".to_string(), 0)]))
            .expect("import");

        assert_eq!(anims.len(), 1);
        assert!(anims[0].channels.is_empty());
        assert_eq!(anims[0].morph_channels.len(), 1);
        assert_eq!(anims[0].morph_channels[0].name, "Face");
        assert_eq!(anims[0].morph_channels[0].keys.len(), 2);
        assert_eq!(anims[0].morph_channels[0].keys[0].time, 0.0);
        assert_eq!(anims[0].morph_channels[0].keys[0].values, vec![0]);
        assert_eq!(anims[0].morph_channels[0].keys[0].weights, vec![0.0]);
        assert_eq!(anims[0].morph_channels[0].keys[1].time, 1000.0);
        assert_eq!(anims[0].morph_channels[0].keys[1].weights, vec![1.0]);
    }

    #[test]
    fn rejects_mismatched_time_and_value_counts() {
        let sources = sampler_sources("samp", "times", "values").replacen(
            r#"count="2" stride="3""#,
            r#"count="1" stride="3""#,
            1,
        );
        let document = document_with(&format!(
            r##"
  <library_animations>
    <animation name="Move">
      {sources}
      <channel source="#samp" target="Root/translate"/>
    </animation>
  </library_animations>"##
        ));
        let result = DaeImporter::new().import_animations(
            &document,
            &AiNodeTree::with_root(),
            &HashMap::from([("Root".to_string(), 0)]),
        );

        assert!(matches!(
            result,
            Err(DaeImportError::InvalidAnimation(detail))
                if detail.contains("2 != 1")
        ));
    }
}
