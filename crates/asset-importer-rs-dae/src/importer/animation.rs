use std::collections::HashSet;

use asset_importer_rs_scene::AiAnimation;
use dae_parser::{AnimationClip, Channel, Document, LocalMap};

use crate::DaeImportError;

use super::DaeImporter;

/// Assimp `Collada::AnimationChannel`.
#[derive(Clone, Debug, Default, PartialEq)]
struct AnimationChannel {
    target: String,
    #[allow(dead_code)]
    source_times: String,
    #[allow(dead_code)]
    source_values: String,
    #[allow(dead_code)]
    in_tan_values: String,
    #[allow(dead_code)]
    out_tan_values: String,
    #[allow(dead_code)]
    interpolation_values: String,
}

impl From<&Channel> for AnimationChannel {
    fn from(channel: &Channel) -> Self {
        Self {
            target: channel.target.0.clone(),
            ..Default::default()
        }
    }
}

impl DaeImporter {
    pub(crate) fn import_animations(
        &self,
        document: &Document,
    ) -> Result<Vec<AiAnimation>, DaeImportError> {
        // Collect maps from document.
        let anim_map = document
            .local_map::<dae_parser::Animation>()
            .map_err(DaeImportError::FileFormatError)?;
        let clip_map = document
            .local_map::<AnimationClip>()
            .map_err(DaeImportError::FileFormatError)?;

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
            if !src.channel.is_empty()
                && let Some(anim) = create_animation(src, &name, &anim_map)
            {
                anims.push(anim);
            }
        }

        // When processing the clips, we may have created duplicate animations with the same name.
        // We need to combine them into a single animation.
        combine_single_channel_ai_anims(&mut anims);
        Ok(anims)
    }
}

/// Sampling, matrix decompose, rotate subsample, and morph-weights are not implemented yet.
fn create_animation(
    _src: &dae_parser::Animation,
    _name: &str,
    _anim_map: &LocalMap<'_, dae_parser::Animation>,
) -> Option<AiAnimation> {
    None
}

fn combine_single_channel_ai_anims(anims: &mut Vec<AiAnimation>) {
    let mut delete_indices = Vec::new();
    for a in 0..anims.len() {
        // Skip if the animation has more than one channel.
        if anims[a].channels.len() != 1 {
            continue;
        }

        // Collect all animations with the same duration and ticks per second.
        let duration = anims[a].duration;
        let ticks_per_second = anims[a].ticks_per_second;
        let mut collected = Vec::new();
        for b in a + 1..anims.len() {
            if anims[b].channels.len() == 1
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
        let clip_map = document
            .local_map::<AnimationClip>()
            .expect("clip map");
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
    fn maps_channel_target() {
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
        let channel = AnimationChannel::from(&src.channel[0]);
        assert_eq!(channel.target, "Root/translate");
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
    fn import_animations_is_empty_while_create_is_stubbed() {
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
            .import_animations(&document)
            .expect("import");
        assert!(anims.is_empty());
    }
}
