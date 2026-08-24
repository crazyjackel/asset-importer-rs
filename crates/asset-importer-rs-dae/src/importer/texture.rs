use std::collections::{HashMap, HashSet};

use asset_importer_rs_scene::{
    AiMaterial, AiPropertyTypeInfo, AiTexel, AiTexture, AiTextureFormat, AiTextureMapMode,
    AiTextureType, AiUvTransform,
    matkey::{
        _AI_MATKEY_MAPPINGMODE_U_BASE, _AI_MATKEY_MAPPINGMODE_V_BASE, _AI_MATKEY_TEXBLEND_BASE,
        _AI_MATKEY_TEXOP_BASE, _AI_MATKEY_TEXTURE_BASE, _AI_MATKEY_UVTRANSFORM_BASE,
        _AI_MATKEY_UVWSRC_BASE,
    },
};
use dae_parser::{
    Blinn, ColorParam, ConstantFx, Document, Effect, Extra, Image, ImageParam, ImageSource,
    Lambert, LocalMap, Material, Phong, ProfileCommon, Shader, SurfaceInit, Texture, Url, WrapMode,
};

use crate::DaeImportError;

use super::{DaeImporter, material::material_key};

const TEX_OP_MULTIPLY: u8 = 0;
const TEX_OP_ADD: u8 = 1;
const TEX_OP_SUBTRACT: u8 = 2;

#[derive(Clone, Debug)]
struct EffectSampler {
    name: String,
    uv_channel: String,
    uv_id: Option<u32>,
    wrap_u: AiTextureMapMode,
    wrap_v: AiTextureMapMode,
    transform: AiUvTransform,
    op: u8,
    weighting: f32,
}

impl Default for EffectSampler {
    fn default() -> Self {
        Self {
            name: String::new(),
            uv_channel: String::new(),
            uv_id: None,
            wrap_u: AiTextureMapMode::Wrap,
            wrap_v: AiTextureMapMode::Wrap,
            transform: AiUvTransform::default(),
            op: TEX_OP_MULTIPLY,
            weighting: 1.0,
        }
    }
}

impl EffectSampler {
    fn from_color(effect: &Effect, profile: &ProfileCommon, param: Option<&ColorParam>) -> Self {
        param
            .and_then(ColorParam::as_texture)
            .map(|texture| Self::from_texture(effect, profile, texture))
            .unwrap_or_default()
    }

    fn from_texture(effect: &Effect, profile: &ProfileCommon, texture: &Texture) -> Self {
        let mut sampler = Self {
            name: texture.texture.clone(),
            uv_channel: texture.texcoord.clone(),
            ..Self::default()
        };
        if let Some(param) = profile.get_param(effect, &texture.texture)
            && let Some(sampler2d) = param.ty.as_sampler2d()
        {
            sampler.wrap_u = match sampler2d.wrap_s {
                WrapMode::Wrap => AiTextureMapMode::Wrap,
                WrapMode::Mirror => AiTextureMapMode::Mirror,
                WrapMode::Clamp | WrapMode::Border | WrapMode::None => AiTextureMapMode::Clamp,
            };
            sampler.wrap_v = match sampler2d.wrap_t {
                WrapMode::Wrap => AiTextureMapMode::Wrap,
                WrapMode::Mirror => AiTextureMapMode::Mirror,
                WrapMode::Clamp | WrapMode::Border | WrapMode::None => AiTextureMapMode::Clamp,
            };
        }
        if let Some(extra) = texture.extra.as_deref() {
            apply_texture_extras(&mut sampler, extra);
        }
        sampler
    }
}

impl DaeImporter {
    pub(crate) fn import_textures(
        &self,
        document: &Document,
        effect_map: &LocalMap<'_, Effect>,
        materials: &mut [AiMaterial],
        material_index_map: &HashMap<String, usize>,
        material_uv_map: &HashMap<usize, HashMap<String, u32>>,
    ) -> Result<Vec<AiTexture>, DaeImportError> {
        let image_map = document
            .local_map::<Image>()
            .map_err(DaeImportError::FileFormatError)?;
        let mut textures = Vec::new();
        let mut embedded_ids = HashMap::new();

        for library in document.library_iter::<Material>() {
            for (index, material) in library.items.iter().enumerate() {
                let name = material_key(material, index);
                let Some(&material_index) = material_index_map.get(&name) else {
                    continue;
                };
                let Some(ai_material) = materials.get_mut(material_index) else {
                    continue;
                };
                let Some(effect) = effect_map.get(&material.instance_effect.url) else {
                    continue;
                };
                let Some(profile) = effect.get_common_profile() else {
                    continue;
                };

                let uv_sets = material_uv_map.get(&material_index);
                for (mut sampler, texture_type) in collect_profile_samplers(effect, profile) {
                    let filename = find_filename_for_effect_texture(
                        effect,
                        profile,
                        &image_map,
                        &sampler.name,
                        &mut textures,
                        &mut embedded_ids,
                    )?;
                    if let Some(&id) =
                        uv_sets.and_then(|channels| channels.get(&sampler.uv_channel))
                    {
                        sampler.uv_id = Some(id);
                    }
                    add_texture(ai_material, &sampler, texture_type, 0, &filename);
                }
            }
        }

        Ok(textures)
    }
}

fn collect_profile_samplers(
    effect: &Effect,
    profile: &ProfileCommon,
) -> Vec<(EffectSampler, AiTextureType)> {
    let mut slots = profile
        .technique
        .data
        .shaders
        .first()
        .map(|shader| match shader {
            Shader::Constant(shader) => samplers_from_constant(shader, effect, profile),
            Shader::Lambert(shader) => samplers_from_lambert(shader, effect, profile),
            Shader::Blinn(shader) => samplers_from_blinn(shader, effect, profile),
            Shader::Phong(shader) => samplers_from_phong(shader, effect, profile),
        })
        .unwrap_or_default();

    let extras = effect
        .extra
        .iter()
        .chain(profile.extra.iter())
        .chain(profile.technique.extra.iter());
    if let Some(bump) = bump_sampler_from_extras(effect, profile, extras) {
        slots.push((bump, AiTextureType::Normals));
    }
    slots
}

fn samplers_from_constant(
    shader: &ConstantFx,
    effect: &Effect,
    profile: &ProfileCommon,
) -> Vec<(EffectSampler, AiTextureType)> {
    samplers_from_slots(
        effect,
        profile,
        &[
            (shader.emission.as_deref(), AiTextureType::Emissive),
            (shader.transparent.as_deref(), AiTextureType::Opacity),
            (shader.reflective.as_deref(), AiTextureType::Reflection),
        ],
    )
}

fn samplers_from_lambert(
    shader: &Lambert,
    effect: &Effect,
    profile: &ProfileCommon,
) -> Vec<(EffectSampler, AiTextureType)> {
    samplers_from_slots(
        effect,
        profile,
        &[
            (shader.ambient.as_deref(), AiTextureType::Lightmap),
            (shader.emission.as_deref(), AiTextureType::Emissive),
            (shader.diffuse.as_deref(), AiTextureType::Diffuse),
            (shader.transparent.as_deref(), AiTextureType::Opacity),
            (shader.reflective.as_deref(), AiTextureType::Reflection),
        ],
    )
}

fn samplers_from_blinn(
    shader: &Blinn,
    effect: &Effect,
    profile: &ProfileCommon,
) -> Vec<(EffectSampler, AiTextureType)> {
    samplers_from_slots(
        effect,
        profile,
        &[
            (shader.ambient.as_deref(), AiTextureType::Lightmap),
            (shader.emission.as_deref(), AiTextureType::Emissive),
            (shader.specular.as_deref(), AiTextureType::Specular),
            (shader.diffuse.as_deref(), AiTextureType::Diffuse),
            (shader.transparent.as_deref(), AiTextureType::Opacity),
            (shader.reflective.as_deref(), AiTextureType::Reflection),
        ],
    )
}

fn samplers_from_phong(
    shader: &Phong,
    effect: &Effect,
    profile: &ProfileCommon,
) -> Vec<(EffectSampler, AiTextureType)> {
    samplers_from_slots(
        effect,
        profile,
        &[
            (shader.ambient.as_deref(), AiTextureType::Lightmap),
            (shader.emission.as_deref(), AiTextureType::Emissive),
            (shader.specular.as_deref(), AiTextureType::Specular),
            (shader.diffuse.as_deref(), AiTextureType::Diffuse),
            (shader.transparent.as_deref(), AiTextureType::Opacity),
            (shader.reflective.as_deref(), AiTextureType::Reflection),
        ],
    )
}

fn samplers_from_slots(
    effect: &Effect,
    profile: &ProfileCommon,
    slots: &[(Option<&ColorParam>, AiTextureType)],
) -> Vec<(EffectSampler, AiTextureType)> {
    let mut out = Vec::new();
    for &(param, texture_type) in slots {
        let sampler = EffectSampler::from_color(effect, profile, param);
        if !sampler.name.is_empty() {
            out.push((sampler, texture_type));
        }
    }
    out
}

fn bump_sampler_from_extras<'a>(
    effect: &Effect,
    profile: &ProfileCommon,
    extras: impl IntoIterator<Item = &'a Extra>,
) -> Option<EffectSampler> {
    for extra in extras {
        for technique in &extra.technique {
            for child in technique.element.children() {
                if !child.name().eq_ignore_ascii_case("bump") {
                    continue;
                }
                for nested in child.children() {
                    if nested.name() != "texture" {
                        continue;
                    }
                    let Some(sampler_name) = nested.attr("texture") else {
                        continue;
                    };
                    let texture =
                        Texture::new(sampler_name, nested.attr("texcoord").unwrap_or_default());
                    return Some(EffectSampler::from_texture(effect, profile, &texture));
                }
            }
        }
    }
    None
}

fn apply_texture_extras(sampler: &mut EffectSampler, extra: &Extra) {
    for technique in &extra.technique {
        for child in technique.element.children() {
            let name = child.name().to_ascii_lowercase();
            let text = child.text();
            let text = text.trim();
            let flag = text == "1" || text.eq_ignore_ascii_case("true");
            match name.as_str() {
                "wrapu" => {
                    sampler.wrap_u = if flag {
                        AiTextureMapMode::Wrap
                    } else {
                        AiTextureMapMode::Clamp
                    };
                }
                "wrapv" => {
                    sampler.wrap_v = if flag {
                        AiTextureMapMode::Wrap
                    } else {
                        AiTextureMapMode::Clamp
                    };
                }
                "mirroru" => {
                    if flag {
                        sampler.wrap_u = AiTextureMapMode::Mirror;
                    }
                }
                "mirrorv" => {
                    if flag {
                        sampler.wrap_v = AiTextureMapMode::Mirror;
                    }
                }
                "offsetu" => {
                    if let Ok(value) = text.parse::<f32>() {
                        sampler.transform.translation.x = value;
                    }
                }
                "offsetv" => {
                    if let Ok(value) = text.parse::<f32>() {
                        sampler.transform.translation.y = value;
                    }
                }
                "repeatu" => {
                    if let Ok(value) = text.parse::<f32>() {
                        sampler.transform.scaling.x = value;
                    }
                }
                "repeatv" => {
                    if let Ok(value) = text.parse::<f32>() {
                        sampler.transform.scaling.y = value;
                    }
                }
                "rotateuv" => {
                    if let Ok(value) = text.parse::<f32>() {
                        sampler.transform.rotation = value;
                    }
                }
                "weighting" | "amount" => {
                    if let Ok(value) = text.parse::<f32>() {
                        sampler.weighting = value;
                    }
                }
                "blend_mode" => {
                    sampler.op = match text.to_ascii_uppercase().as_str() {
                        "ADD" => TEX_OP_ADD,
                        "SUBTRACT" => TEX_OP_SUBTRACT,
                        _ => TEX_OP_MULTIPLY,
                    };
                }
                _ => {}
            }
        }
    }
}

fn add_texture(
    material: &mut AiMaterial,
    sampler: &EffectSampler,
    texture_type: AiTextureType,
    index: u32,
    filename: &str,
) {
    material.add_property(
        _AI_MATKEY_TEXTURE_BASE,
        Some(texture_type),
        AiPropertyTypeInfo::Binary,
        index,
        filename.bytes().collect(),
    );
    material.add_property(
        _AI_MATKEY_MAPPINGMODE_U_BASE,
        Some(texture_type),
        AiPropertyTypeInfo::Binary,
        index,
        vec![sampler.wrap_u as u8],
    );
    material.add_property(
        _AI_MATKEY_MAPPINGMODE_V_BASE,
        Some(texture_type),
        AiPropertyTypeInfo::Binary,
        index,
        vec![sampler.wrap_v as u8],
    );
    material.add_property(
        _AI_MATKEY_UVTRANSFORM_BASE,
        Some(texture_type),
        AiPropertyTypeInfo::Binary,
        index,
        bytemuck::bytes_of(&sampler.transform).to_vec(),
    );
    material.add_property(
        _AI_MATKEY_TEXOP_BASE,
        Some(texture_type),
        AiPropertyTypeInfo::Binary,
        index,
        vec![sampler.op],
    );
    material.add_property(
        _AI_MATKEY_TEXBLEND_BASE,
        Some(texture_type),
        AiPropertyTypeInfo::Binary,
        index,
        sampler.weighting.to_le_bytes().to_vec(),
    );
    let uv_source = if let Some(id) = sampler.uv_id {
        id as i32
    } else {
        let mut digits = String::new();
        for ch in sampler.uv_channel.chars() {
            if ch.is_ascii_digit() {
                digits.push(ch);
            } else if !digits.is_empty() {
                break;
            }
        }
        digits.parse().unwrap_or(0)
    };
    material.add_property(
        _AI_MATKEY_UVWSRC_BASE,
        Some(texture_type),
        AiPropertyTypeInfo::Binary,
        index,
        uv_source.to_le_bytes().to_vec(),
    );
}

fn find_filename_for_effect_texture(
    effect: &Effect,
    profile: &ProfileCommon,
    image_map: &LocalMap<'_, Image>,
    name: &str,
    textures: &mut Vec<AiTexture>,
    embedded_ids: &mut HashMap<String, usize>,
) -> Result<String, DaeImportError> {
    let mut current = name.to_string();
    let mut visited = HashSet::new();
    while visited.insert(current.clone()) {
        let Some(param) = profile.get_param(effect, &current) else {
            break;
        };
        if let Some(sampler) = param.ty.as_sampler2d() {
            current = sampler.source.val.clone();
            continue;
        }
        if let Some(surface) = param.ty.as_surface() {
            if let SurfaceInit::From { image, .. } = &surface.init {
                current = image.val.clone();
                continue;
            }
        }
        break;
    }

    let image = image_map.get_str(&current).or_else(|| {
        effect
            .image
            .iter()
            .chain(profile.image.iter())
            .find(|image| image.id.as_deref() == Some(current.as_str()))
            .or_else(|| {
                profile
                    .technique
                    .data
                    .image_param
                    .iter()
                    .find_map(|param| match param {
                        ImageParam::Image(image)
                            if image.id.as_deref() == Some(current.as_str()) =>
                        {
                            Some(image)
                        }
                        _ => None,
                    })
            })
    });
    let Some(image) = image else {
        return Ok(format!("{current}.jpg"));
    };

    match &image.source {
        ImageSource::InitFrom(url) => {
            let raw = match url {
                Url::Fragment(fragment) => fragment.as_str(),
                Url::Other(other) => other.as_str(),
            };
            let trimmed = raw.trim();
            let without_file = trimmed
                .strip_prefix("file://")
                .or_else(|| trimmed.strip_prefix("FILE://"))
                .unwrap_or(trimmed);
            let decoded = urlencoding::decode(without_file)
                .map(|s| s.into_owned())
                .unwrap_or_else(|_| without_file.to_string());
            if decoded.is_empty() {
                return Err(DaeImportError::InvalidTexture(format!(
                    "image '{}' has no data or file reference",
                    image.id.as_deref().unwrap_or(&current)
                )));
            }
            Ok(decoded)
        }
        ImageSource::Data(data) => {
            let key = image
                .id
                .clone()
                .or_else(|| image.name.clone())
                .unwrap_or_else(|| current.clone());
            if let Some(&index) = embedded_ids.get(&key) {
                return Ok(format!("*{index}"));
            }
            let index = textures.len();
            embedded_ids.insert(key.clone(), index);
            let format = match image
                .format
                .as_deref()
                .map(|value| value.to_ascii_lowercase())
                .as_deref()
            {
                Some("png") => AiTextureFormat::PNG,
                Some("jpg" | "jpeg") => AiTextureFormat::JPEG,
                Some("bmp") => AiTextureFormat::BMP,
                Some("gif") => AiTextureFormat::GIF,
                Some("webp") => AiTextureFormat::WEBP,
                _ => AiTextureFormat::Unknown,
            };
            let rgba = match format {
                AiTextureFormat::Unknown => image::load_from_memory(data),
                hint => image::load_from_memory_with_format(data, hint.into())
                    .or_else(|_| image::load_from_memory(data)),
            }
            .map_err(|err| {
                DaeImportError::InvalidTexture(format!(
                    "image '{}' is not a valid embedded image: {err}",
                    image.id.as_deref().unwrap_or(&current)
                ))
            })?
            .to_rgba8();
            textures.push(AiTexture {
                filename: image
                    .name
                    .clone()
                    .or_else(|| image.id.clone())
                    .unwrap_or(key),
                width: rgba.width(),
                height: rgba.height(),
                ach_format_hint: format,
                texel: rgba.pixels().map(|pixel| AiTexel::from(pixel.0)).collect(),
            });
            Ok(format!("*{index}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn collada_with(body: &str) -> Document {
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

    fn textured_effect(diffuse: &str, extra: &str) -> String {
        format!(
            r##"
  <library_effects>
    <effect id="fx">
      <profile_COMMON>
        <newparam sid="surface">
          <surface type="2D">
            <init_from>diffuse-image</init_from>
          </surface>
        </newparam>
        <newparam sid="sampler">
          <sampler2D>
            <source>surface</source>
            <wrap_s>CLAMP</wrap_s>
            <wrap_t>WRAP</wrap_t>
          </sampler2D>
        </newparam>
        <technique sid="common">
          <phong>
            {diffuse}
          </phong>
          {extra}
        </technique>
      </profile_COMMON>
    </effect>
  </library_effects>
  <library_materials>
    <material id="mat" name="Mat">
      <instance_effect url="#fx"/>
    </material>
  </library_materials>"##
        )
    }

    const DIFFUSE_TEX: &str =
        r#"<diffuse><texture texture="sampler" texcoord="UVSET0"/></diffuse>"#;

    fn library_image(attrs: &str, children: &str) -> String {
        format!(
            r#"
  <library_images>
    <image id="diffuse-image"{attrs}>
      {children}
    </image>
  </library_images>"#
        )
    }

    fn import_phong(image: &str, diffuse: &str, extra: &str) -> (Vec<AiMaterial>, Vec<AiTexture>) {
        import_document(&collada_with(&format!(
            "{image}{}",
            textured_effect(diffuse, extra)
        )))
    }

    fn import_document(document: &Document) -> (Vec<AiMaterial>, Vec<AiTexture>) {
        let effect_map = document.local_map::<Effect>().expect("effect map");
        let importer = DaeImporter::new();
        let (mut materials, material_index_map) = importer
            .import_materials(document, &effect_map)
            .expect("materials");
        let material_uv_map = document
            .get_visual_scene()
            .map(|visual_scene| {
                let material_map = document.local_map::<Material>().expect("material map");
                importer
                    .import_nodes(document, visual_scene, &material_map, &material_index_map)
                    .expect("nodes")
                    .material_uv_map
            })
            .unwrap_or_default();
        let textures = importer
            .import_textures(
                document,
                &effect_map,
                &mut materials,
                &material_index_map,
                &material_uv_map,
            )
            .expect("textures");
        (materials, textures)
    }

    fn tex_file(material: &AiMaterial, texture_type: AiTextureType) -> String {
        material
            .get_property_ai_str(_AI_MATKEY_TEXTURE_BASE, Some(texture_type), 0)
            .expect("texture file")
            .expect("utf8")
    }

    fn assert_wrap(material: &AiMaterial, u: AiTextureMapMode, v: AiTextureMapMode) {
        assert_eq!(
            material
                .get_property_byte(
                    _AI_MATKEY_MAPPINGMODE_U_BASE,
                    Some(AiTextureType::Diffuse),
                    0
                )
                .expect("wrap u"),
            u as u8
        );
        assert_eq!(
            material
                .get_property_byte(
                    _AI_MATKEY_MAPPINGMODE_V_BASE,
                    Some(AiTextureType::Diffuse),
                    0
                )
                .expect("wrap v"),
            v as u8
        );
    }

    fn uvwsrc(material: &AiMaterial, texture_type: AiTextureType) -> i32 {
        let property = material
            .get_property(_AI_MATKEY_UVWSRC_BASE, Some(texture_type), 0)
            .expect("uvwsrc");
        i32::from_le_bytes(property.data.as_slice().try_into().expect("i32"))
    }

    #[test]
    fn diffuse_init_from_sets_path_and_wrap() {
        let (materials, textures) = import_phong(
            &library_image("", "<init_from>textures/diffuse.png</init_from>"),
            DIFFUSE_TEX,
            "",
        );
        assert!(textures.is_empty());
        assert_eq!(
            tex_file(&materials[0], AiTextureType::Diffuse),
            "textures/diffuse.png"
        );
        assert_wrap(
            &materials[0],
            AiTextureMapMode::Clamp,
            AiTextureMapMode::Wrap,
        );
        assert_eq!(uvwsrc(&materials[0], AiTextureType::Diffuse), 0);
    }

    #[test]
    fn cyclic_param_resolution_does_not_loop() {
        let document = collada_with(
            r##"
  <library_effects>
    <effect id="fx">
      <profile_COMMON>
        <newparam sid="surface">
          <surface type="2D">
            <init_from>sampler</init_from>
          </surface>
        </newparam>
        <newparam sid="sampler">
          <sampler2D>
            <source>surface</source>
          </sampler2D>
        </newparam>
        <technique sid="common">
          <phong>
            <diffuse><texture texture="sampler" texcoord="UVSET0"/></diffuse>
          </phong>
        </technique>
      </profile_COMMON>
    </effect>
  </library_effects>
  <library_materials>
    <material id="mat" name="Mat">
      <instance_effect url="#fx"/>
    </material>
  </library_materials>"##,
        );
        let (materials, _) = import_document(&document);
        assert_eq!(
            tex_file(&materials[0], AiTextureType::Diffuse),
            "sampler.jpg"
        );
    }

    #[test]
    fn unresolved_image_falls_back_to_jpg() {
        let (materials, _) = import_phong(
            "",
            r#"<diffuse><texture texture="missing" texcoord="UVSET0"/></diffuse>"#,
            "",
        );
        assert_eq!(
            tex_file(&materials[0], AiTextureType::Diffuse),
            "missing.jpg"
        );
    }

    #[test]
    fn embedded_data_image_uses_star_index() {
        // 1x1 RGBA PNG (black, opaque)
        const PNG: &str = "89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c4890000000a49444154789c63000100000500010d0a2db40000000049454e44ae426082";
        let (materials, textures) = import_phong(
            &library_image(
                r#" name="embedded" format="PNG""#,
                &format!("<data>{PNG}</data>"),
            ),
            DIFFUSE_TEX,
            "",
        );
        assert_eq!(tex_file(&materials[0], AiTextureType::Diffuse), "*0");
        assert_eq!(textures.len(), 1);
        assert_eq!(textures[0].filename, "embedded");
        assert_eq!(textures[0].width, 1);
        assert_eq!(textures[0].height, 1);
        assert_eq!(textures[0].ach_format_hint, AiTextureFormat::PNG);
        assert_eq!(textures[0].texel.len(), 1);
        let exported = textures[0]
            .export(&[AiTextureFormat::PNG])
            .expect("export decoded texels");
        assert!(!exported.data.is_empty());
    }

    #[test]
    fn bump_extra_maps_to_normals() {
        let (materials, _) = import_phong(
            &library_image("", "<init_from>textures/bump.png</init_from>"),
            "",
            r#"<extra>
            <technique profile="FCOLLADA">
              <bump>
                <texture texture="sampler" texcoord="UVSET0"/>
              </bump>
            </technique>
          </extra>"#,
        );
        assert_eq!(
            tex_file(&materials[0], AiTextureType::Normals),
            "textures/bump.png"
        );
    }

    #[test]
    fn import_textures_sets_uvwsrc_from_bind_vertex_input() {
        let document = collada_with(&format!(
            r##"
  {}
  {}
  <library_geometries>
    <geometry id="mesh">
      <mesh>
        <source id="pos">
          <float_array id="pos-array" count="9">0 0 0 1 0 0 0 1 0</float_array>
          <technique_common>
            <accessor source="#pos-array" count="3" stride="3">
              <param name="X" type="float"/>
              <param name="Y" type="float"/>
              <param name="Z" type="float"/>
            </accessor>
          </technique_common>
        </source>
        <vertices id="verts">
          <input semantic="POSITION" source="#pos"/>
        </vertices>
        <triangles count="1" material="s">
          <input semantic="VERTEX" source="#verts" offset="0"/>
          <p>0 1 2</p>
        </triangles>
      </mesh>
    </geometry>
  </library_geometries>
  <library_visual_scenes>
    <visual_scene id="Scene">
      <node>
        <instance_geometry url="#mesh">
          <bind_material>
            <technique_common>
              <instance_material symbol="s" target="#mat">
                <bind_vertex_input semantic="UVSET0" input_semantic="TEXCOORD" input_set="1"/>
              </instance_material>
            </technique_common>
          </bind_material>
        </instance_geometry>
      </node>
    </visual_scene>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Scene"/>
  </scene>"##,
            library_image("", "<init_from>textures/diffuse.png</init_from>"),
            textured_effect(DIFFUSE_TEX, "")
        ));
        let (materials, _) = import_document(&document);
        assert_eq!(uvwsrc(&materials[0], AiTextureType::Diffuse), 1);
    }
}
