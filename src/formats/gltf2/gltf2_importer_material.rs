use std::collections::HashMap;

use gltf::{
    material::{NormalTexture, OcclusionTexture},
    texture::{self, WrappingMode},
    Document,
};

use crate::{
    core::error::AiReadError,
    structs::{
        matkey::{
            self, _AI_MATKEY_MAPPINGMODE_U_BASE, _AI_MATKEY_MAPPINGMODE_V_BASE,
            _AI_MATKEY_TEXTURE_BASE,
        },
        AiColor3D, AiColor4D, AiMaterial, AiPropertyTypeInfo, AiTextureMapMode, AiTextureType,
    },
};

use super::gltf2_importer::Gltf2Importer;

pub const AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE: AiTextureType =
    AiTextureType::Unknown;
pub const AI_MATKEY_GLTF_ALPHAMODE: &str = "$mat.gltf.alphaMode";
pub const AI_MATKEY_GLTF_ALPHACUTOFF: &str = "$mat.gltf.alphaCutoff";

pub const _AI_MATKEY_GLTF_MAPPINGNAME_BASE: &str = "$tex.mappingname";
pub const _AI_MATKEY_GLTF_MAPPINGID_BASE: &str = "$tex.mappingid";
pub const _AI_MATKEY_GLTF_MAPPINGFILTER_MAG_BASE: &str = "$tex.mappingfiltermag";
pub const _AI_MATKEY_GLTF_MAPPINGFILTER_MIN_BASE: &str = "$tex.mappingfiltermin";
pub const _AI_MATKEY_GLTF_SCALE_BASE: &str = "$tex.scale";
pub const _AI_MATKEY_GLTF_STRENGTH_BASE: &str = "$tex.strength";

trait ImportTexture<'a> {
    fn texture(&self) -> gltf::Texture<'a>;
    fn tex_coord(&self) -> u32;
    #[cfg(feature = "KHR_texture_transform")]
    fn texture_transform(&self) -> Option<gltf::texture::TextureTransform<'a>>;
}

impl<'a> ImportTexture<'a> for texture::Info<'a> {
    fn texture(&self) -> gltf::Texture<'a> {
        self.texture()
    }

    fn tex_coord(&self) -> u32 {
        self.tex_coord()
    }

    #[cfg(feature = "KHR_texture_transform")]
    fn texture_transform(&self) -> Option<gltf::texture::TextureTransform<'a>> {
        self.texture_transform()
    }
}

impl<'a> ImportTexture<'a> for NormalTexture<'a> {
    fn texture(&self) -> gltf::Texture<'a> {
        self.texture()
    }

    fn tex_coord(&self) -> u32 {
        self.tex_coord()
    }

    //@todo: When supported, update here: https://github.com/gltf-rs/gltf/pull/412
    #[cfg(feature = "KHR_texture_transform")]
    fn texture_transform(&self) -> Option<gltf::texture::TextureTransform<'a>> {
        None
    }
}
impl<'a> ImportTexture<'a> for OcclusionTexture<'a> {
    fn texture(&self) -> gltf::Texture<'a> {
        self.texture()
    }

    fn tex_coord(&self) -> u32 {
        self.tex_coord()
    }

    #[cfg(feature = "KHR_texture_transform")]
    fn texture_transform(&self) -> Option<gltf::texture::TextureTransform<'a>> {
        None
    }
}
impl Gltf2Importer {
    pub(crate) fn import_embedded_materials(
        document: &Document,
        embedded_tex_ids: &HashMap<usize, usize>,
    ) -> Result<Vec<AiMaterial>, AiReadError> {
        let mut materials: Vec<AiMaterial> = Vec::new();
        for material in document.materials() {
            let mut ai_material = AiMaterial::new();

            //Handle Name
            if let Some(name) = material.name() {
                ai_material.add_property(
                    matkey::AI_MATKEY_NAME,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(name.bytes().collect()),
                    0,
                );
            }

            //Handle PBR_ROUGHNESS
            handle_pbr_roughness(embedded_tex_ids, &material, &mut ai_material);

            //Handle Base Properties
            handle_base(embedded_tex_ids, &material, &mut ai_material);

            handle_specular(embedded_tex_ids, &material, &mut ai_material);
            handle_unlit(embedded_tex_ids, &material, &mut ai_material);

            //@todo Handle Sheen and Clearcoat once GLTF2 supports

            //handle_sheen(embedded_tex_ids, &material, &mut ai_material);
            //handle_clearcoat(embedded_tex_ids, &material, &mut ai_material);

            handle_transmission(embedded_tex_ids, &material, &mut ai_material);
            handle_volume(embedded_tex_ids, &material, &mut ai_material);
            handle_ior(embedded_tex_ids, &material, &mut ai_material);
            handle_emissive_strength(embedded_tex_ids, &material, &mut ai_material);

            materials.push(ai_material);
        }
        Ok(materials)
    }
}

fn convert_map_mode(wrap_mode: WrappingMode) -> AiTextureMapMode {
    match wrap_mode {
        WrappingMode::ClampToEdge => AiTextureMapMode::Clamp,
        WrappingMode::MirroredRepeat => AiTextureMapMode::Mirror,
        WrappingMode::Repeat => AiTextureMapMode::Wrap,
    }
}

fn import_texture_property<'a, T: ImportTexture<'a>>(
    texture_info_ref: &Option<T>,
    ai_material: &mut AiMaterial,
    embedded_tex_ids: &HashMap<usize, usize>,
    texture_type: AiTextureType,
    texture_index: u32,
) {
    if let Some(texture_info) = texture_info_ref {
        let texture = texture_info.texture();
        let source = texture.source();

        //Get Uri from Source
        let uri = match &embedded_tex_ids.get(&source.index()) {
            Some(str) => format!("*{}", str),
            None => format!("*{}", source.index()),
        };

        ai_material.add_property(
            matkey::_AI_MATKEY_TEXTURE_BASE,
            Some(texture_type),
            AiPropertyTypeInfo::Binary(uri.bytes().collect()),
            texture_index,
        );

        let uv_index = texture_info.tex_coord();
        ai_material.add_property(
            matkey::_AI_MATKEY_UVWSRC_BASE,
            Some(texture_type),
            AiPropertyTypeInfo::Binary(uv_index.to_le_bytes().to_vec()),
            texture_index,
        );

        //Handle Texture Transform
        handle_texture_transform(texture_info, ai_material, texture_type, texture_index);

        let sampler = texture.sampler();
        if let Some(id) = sampler.index() {
            let name = sampler.name().unwrap_or("");
            ai_material.add_property(
                _AI_MATKEY_GLTF_MAPPINGNAME_BASE,
                Some(texture_type),
                AiPropertyTypeInfo::Binary(name.bytes().collect()),
                texture_index,
            );
            ai_material.add_property(
                _AI_MATKEY_GLTF_MAPPINGID_BASE,
                Some(texture_type),
                AiPropertyTypeInfo::Binary((id as u32).to_le_bytes().to_vec()),
                texture_index,
            );

            let map_mode = convert_map_mode(sampler.wrap_s());
            ai_material.add_property(
                _AI_MATKEY_MAPPINGMODE_U_BASE,
                Some(texture_type),
                AiPropertyTypeInfo::Binary(vec![(map_mode as u8)]),
                texture_index,
            );
            let map_mode = convert_map_mode(sampler.wrap_t());
            ai_material.add_property(
                _AI_MATKEY_MAPPINGMODE_V_BASE,
                Some(texture_type),
                AiPropertyTypeInfo::Binary(vec![(map_mode as u8)]),
                texture_index,
            );

            if let Some(mag_filter) = sampler.mag_filter() {
                ai_material.add_property(
                    _AI_MATKEY_GLTF_MAPPINGFILTER_MAG_BASE,
                    Some(texture_type),
                    AiPropertyTypeInfo::Binary(vec![(mag_filter as u8)]),
                    texture_index,
                );
            }
            if let Some(min_filter) = sampler.min_filter() {
                ai_material.add_property(
                    _AI_MATKEY_GLTF_MAPPINGFILTER_MIN_BASE,
                    Some(texture_type),
                    AiPropertyTypeInfo::Binary(vec![(min_filter as u8)]),
                    texture_index,
                );
            }
        } else {
            let map_mode = convert_map_mode(sampler.wrap_s());
            ai_material.add_property(
                _AI_MATKEY_MAPPINGMODE_U_BASE,
                Some(texture_type),
                AiPropertyTypeInfo::Binary(vec![(map_mode as u8)]),
                texture_index,
            );

            let map_mode = convert_map_mode(sampler.wrap_t());
            ai_material.add_property(
                _AI_MATKEY_MAPPINGMODE_V_BASE,
                Some(texture_type),
                AiPropertyTypeInfo::Binary(vec![(map_mode as u8)]),
                texture_index,
            );
        }
    }
}

fn import_texture_property_occlusion(
    texture_info_ref: &Option<OcclusionTexture>,
    ai_material: &mut AiMaterial,
    embedded_tex_ids: &HashMap<usize, usize>,
    texture_type: AiTextureType,
    texture_index: u32,
) {
    import_texture_property(
        texture_info_ref,
        ai_material,
        embedded_tex_ids,
        texture_type,
        texture_index,
    );

    if let Some(texture_info) = texture_info_ref {
        let strength = texture_info.strength();
        let texture_strength_key = format!("{}.strength", _AI_MATKEY_TEXTURE_BASE);
        ai_material.add_property(
            &texture_strength_key,
            Some(texture_type),
            AiPropertyTypeInfo::Binary(strength.to_le_bytes().to_vec()),
            texture_index,
        );
    }
}
fn import_texture_property_normals(
    texture_info_ref: &Option<NormalTexture>,
    ai_material: &mut AiMaterial,
    embedded_tex_ids: &HashMap<usize, usize>,
    texture_type: AiTextureType,
    texture_index: u32,
) {
    import_texture_property(
        texture_info_ref,
        ai_material,
        embedded_tex_ids,
        texture_type,
        texture_index,
    );

    if let Some(texture_info) = texture_info_ref {
        let scale = texture_info.scale();
        ai_material.add_property(
            _AI_MATKEY_GLTF_SCALE_BASE,
            Some(texture_type),
            AiPropertyTypeInfo::Binary(scale.to_le_bytes().to_vec()),
            texture_index,
        );
    }
}

#[cfg(not(feature = "KHR_texture_transform"))]
fn handle_texture_transform<'a, T: ImportTexture<'a>>(
    _texture_info: &T,
    _ai_material: &mut AiMaterial,
    _texture_type: AiTextureType,
    _texture_index: u32,
) {
}
#[cfg(feature = "KHR_texture_transform")]
fn handle_texture_transform<'a, T: ImportTexture<'a>>(
    texture_info: &T,
    ai_material: &mut AiMaterial,
    texture_type: AiTextureType,
    texture_index: u32,
) {
    use crate::structs::{base_types::AiReal, AiUvTransform, AiVector2D};

    if let Some(transform) = texture_info.texture_transform() {
        let scale = transform.scale();
        let rotation = transform.rotation();
        let translation = transform.offset();
        let rcos = f32::cos(-rotation);
        let rsin = f32::sin(-rotation);
        let offset_x = (0.5 * scale[0]) * (-rcos + rsin + 1.0) + translation[0];
        let offset_y = ((0.5 * scale[1]) * (rsin + rcos - 1.0)) + 1.0 - scale[1] - translation[1];
        let transform = AiUvTransform {
            scaling: AiVector2D {
                x: scale[0] as AiReal,
                y: scale[1] as AiReal,
            },
            translation: AiVector2D {
                x: offset_x as AiReal,
                y: offset_y as AiReal,
            },
            rotation: rotation as AiReal,
        };
        ai_material.add_property(
            matkey::_AI_MATKEY_UVTRANSFORM_BASE,
            Some(texture_type),
            AiPropertyTypeInfo::Binary(bytemuck::bytes_of(&transform).to_vec()),
            texture_index,
        );
    }
}
fn handle_pbr_roughness(
    embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    let pbr_metallic_roughness = material.pbr_metallic_roughness();

    // Set Assimp DIFFUSE and BASE COLOR to the pbrMetallicRoughness base color and texture for backwards compatibility
    // Technically should not load any pbrMetallicRoughness if extensionsRequired contains KHR_materials_pbrSpecularGlossiness
    ai_material.add_property(
        matkey::AI_MATKEY_COLOR_DIFFUSE,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(
            bytemuck::bytes_of(&AiColor4D::from(pbr_metallic_roughness.base_color_factor()))
                .to_vec(),
        ),
        0,
    );
    ai_material.add_property(
        matkey::AI_MATKEY_BASE_COLOR,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(
            bytemuck::bytes_of(&AiColor4D::from(pbr_metallic_roughness.base_color_factor()))
                .to_vec(),
        ),
        0,
    );

    //Handle Base Color Texture
    import_texture_property(
        &pbr_metallic_roughness.base_color_texture(),
        ai_material,
        embedded_tex_ids,
        AiTextureType::Diffuse,
        0,
    );
    import_texture_property(
        &pbr_metallic_roughness.base_color_texture(),
        ai_material,
        embedded_tex_ids,
        AiTextureType::BaseColor,
        0,
    );

    // Handle Metallic Roughness Texture
    // Keep AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE for backwards compatibility
    import_texture_property(
        &pbr_metallic_roughness.metallic_roughness_texture(),
        ai_material,
        embedded_tex_ids,
        AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE,
        0,
    );
    import_texture_property(
        &pbr_metallic_roughness.metallic_roughness_texture(),
        ai_material,
        embedded_tex_ids,
        AiTextureType::Metalness,
        0,
    );
    import_texture_property(
        &pbr_metallic_roughness.metallic_roughness_texture(),
        ai_material,
        embedded_tex_ids,
        AiTextureType::DiffuseRoughness,
        0,
    );

    //Handle Metallic, Roughness, Shininess, and Opacity for PBR
    ai_material.add_property(
        matkey::AI_MATKEY_METALLIC_FACTOR,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(
            pbr_metallic_roughness
                .metallic_factor()
                .to_le_bytes()
                .to_vec(),
        ),
        0,
    );
    ai_material.add_property(
        matkey::AI_MATKEY_ROUGHNESS_FACTOR,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(
            pbr_metallic_roughness
                .roughness_factor()
                .to_le_bytes()
                .to_vec(),
        ),
        0,
    );
    ai_material.add_property(
        matkey::AI_MATKEY_SHININESS,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(
            ((1.0 - pbr_metallic_roughness.roughness_factor()) * 1000.0)
                .to_le_bytes()
                .to_vec(),
        ),
        0,
    );
    ai_material.add_property(
        matkey::AI_MATKEY_OPACITY,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(
            pbr_metallic_roughness.base_color_factor()[3]
                .to_le_bytes()
                .to_vec(),
        ),
        0,
    );
}

fn handle_base(
    embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    import_texture_property_normals(
        &material.normal_texture(),
        ai_material,
        embedded_tex_ids,
        AiTextureType::Normals,
        0,
    );
    import_texture_property_occlusion(
        &material.occlusion_texture(),
        ai_material,
        embedded_tex_ids,
        AiTextureType::Lightmap,
        0,
    );
    import_texture_property(
        &material.emissive_texture(),
        ai_material,
        embedded_tex_ids,
        AiTextureType::Emissive,
        0,
    );

    ai_material.add_property(
        matkey::AI_MATKEY_TWOSIDED,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(vec![material.double_sided() as u8]),
        0,
    );
    ai_material.add_property(
        matkey::AI_MATKEY_COLOR_EMISSIVE,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(
            bytemuck::bytes_of(&AiColor3D::from(material.emissive_factor())).to_vec(),
        ),
        0,
    );

    //This should always succeed
    if let Ok(alpha_mode) = serde_json::to_string(&material.alpha_mode()) {
        ai_material.add_property(
            AI_MATKEY_GLTF_ALPHAMODE,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(alpha_mode.bytes().collect()),
            0,
        );
    }

    if let Some(alpha_cutoff) = material.alpha_cutoff() {
        ai_material.add_property(
            AI_MATKEY_GLTF_ALPHACUTOFF,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(alpha_cutoff.to_le_bytes().to_vec()),
            0,
        );
    }
}

#[cfg(not(any(
    feature = "KHR_materials_specular",
    feature = "KHR_materials_pbrSpecularGlossiness"
)))]
fn handle_specular(
    _embedded_tex_ids: &HashMap<usize, usize>,
    _material: &gltf::Material<'_>,
    _ai_material: &mut AiMaterial,
) {
}
#[cfg(feature = "KHR_materials_specular")]
fn handle_specular(
    embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    use crate::structs::matkey::{AI_MATKEY_COLOR_SPECULAR, AI_MATKEY_SPECULAR_FACTOR};

    if let Some(specular) = material.specular() {
        ai_material.add_property(
            AI_MATKEY_COLOR_SPECULAR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(
                bytemuck::bytes_of(&AiColor3D::from(specular.specular_color_factor())).to_vec(),
            ),
            0,
        );
        ai_material.add_property(
            AI_MATKEY_SPECULAR_FACTOR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(specular.specular_factor().to_le_bytes().to_vec()),
            0,
        );
        import_texture_property(
            &specular.specular_texture(),
            ai_material,
            embedded_tex_ids,
            AiTextureType::Specular,
            0,
        );
        import_texture_property(
            &specular.specular_color_texture(),
            ai_material,
            embedded_tex_ids,
            AiTextureType::Specular,
            1,
        );
    }
}
#[cfg(all(
    feature = "KHR_materials_pbrSpecularGlossiness",
    not(feature = "KHR_materials_specular")
))]
fn handle_specular(
    embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    if let Some(specular) = material.pbr_specular_glossiness() {
        ai_material.add_property(
            matkey::AI_MATKEY_COLOR_DIFFUSE,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(
                bytemuck::bytes_of(&AiColor4D::from(specular.diffuse_factor())).to_vec(),
            ),
            0,
        );
        ai_material.add_property(
            matkey::AI_MATKEY_COLOR_SPECULAR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(
                bytemuck::bytes_of(&AiColor3D::from(specular.specular_factor())).to_vec(),
            ),
            0,
        );
        let shininess = specular.glossiness_factor() * 1000.0;
        ai_material.add_property(
            matkey::AI_MATKEY_SHININESS,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(shininess.to_le_bytes().to_vec()),
            0,
        );
        ai_material.add_property(
            matkey::AI_MATKEY_GLOSSINESS_FACTOR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(specular.glossiness_factor().to_le_bytes().to_vec()),
            0,
        );

        import_texture_property(
            &specular.diffuse_texture(),
            ai_material,
            embedded_tex_ids,
            AiTextureType::Diffuse,
            0,
        );
        import_texture_property(
            &specular.specular_glossiness_texture(),
            ai_material,
            embedded_tex_ids,
            AiTextureType::Specular,
            0,
        );
    }
}

#[cfg(not(feature = "KHR_materials_unlit"))]
fn handle_unlit(
    _embedded_tex_ids: &HashMap<usize, usize>,
    _material: &gltf::Material<'_>,
    _ai_material: &mut AiMaterial,
) {
    use crate::structs::{matkey::AI_MATKEY_SHADING_MODEL, AiShadingMode};
    _ai_material.add_property(
        matkey::AI_MATKEY_SHADING_MODEL,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(vec![AiShadingMode::PBR as u8]),
        0,
    );
}
#[cfg(feature = "KHR_materials_unlit")]
fn handle_unlit(
    _embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    use crate::structs::{matkey::AI_MATKEY_SHADING_MODEL, AiShadingMode};

    ai_material.add_property(
        "$mat.gltf.unlit",
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(vec![material.unlit() as u8]),
        0,
    );
    ai_material.add_property(
        AI_MATKEY_SHADING_MODEL,
        Some(AiTextureType::None),
        AiPropertyTypeInfo::Binary(vec![AiShadingMode::Unlit as u8]),
        0,
    );
}

#[cfg(not(feature = "KHR_materials_transmission"))]
fn handle_transmission(
    _embedded_tex_ids: &HashMap<usize, usize>,
    _material: &gltf::Material<'_>,
    _ai_material: &mut AiMaterial,
) {
}
#[cfg(feature = "KHR_materials_transmission")]
fn handle_transmission(
    embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    use crate::structs::matkey::{AI_MATKEY_TRANSMISSION_FACTOR, AI_MATKEY_TRANSMISSION_TEXTURE};

    if let Some(transmission) = material.transmission() {
        ai_material.add_property(
            AI_MATKEY_TRANSMISSION_FACTOR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(transmission.transmission_factor().to_le_bytes().to_vec()),
            0,
        );
        import_texture_property(
            &transmission.transmission_texture(),
            ai_material,
            embedded_tex_ids,
            AI_MATKEY_TRANSMISSION_TEXTURE,
            0,
        );
    }
}

#[cfg(not(feature = "KHR_materials_volume"))]
fn handle_volume(
    _embedded_tex_ids: &HashMap<usize, usize>,
    _material: &gltf::Material<'_>,
    _ai_material: &mut AiMaterial,
) {
}
#[cfg(feature = "KHR_materials_volume")]
fn handle_volume(
    embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    use crate::structs::matkey::{
        AI_MATKEY_VOLUME_ATTENUATION_COLOR, AI_MATKEY_VOLUME_ATTENUATION_DISTANCE,
        AI_MATKEY_VOLUME_THICKNESS_FACTOR, AI_MATKEY_VOLUME_THICKNESS_TEXTURE,
    };

    if let Some(volume) = material.volume() {
        ai_material.add_property(
            AI_MATKEY_VOLUME_THICKNESS_FACTOR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(volume.thickness_factor().to_le_bytes().to_vec()),
            0,
        );
        import_texture_property(
            &volume.thickness_texture(),
            ai_material,
            embedded_tex_ids,
            AI_MATKEY_VOLUME_THICKNESS_TEXTURE,
            0,
        );
        ai_material.add_property(
            AI_MATKEY_VOLUME_ATTENUATION_DISTANCE,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(volume.attenuation_distance().to_le_bytes().to_vec()),
            0,
        );
        ai_material.add_property(
            AI_MATKEY_VOLUME_ATTENUATION_COLOR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(
                bytemuck::bytes_of(&AiColor3D::from(volume.attenuation_color())).to_vec(),
            ),
            0,
        );
    }
}

#[cfg(not(feature = "KHR_materials_ior"))]
fn handle_ior(
    _embedded_tex_ids: &HashMap<usize, usize>,
    _material: &gltf::Material<'_>,
    _ai_material: &mut AiMaterial,
) {
}
#[cfg(feature = "KHR_materials_ior")]
fn handle_ior(
    _embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    use crate::structs::matkey::AI_MATKEY_REFRACTI;

    if let Some(ior) = material.ior() {
        ai_material.add_property(
            AI_MATKEY_REFRACTI,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(ior.to_le_bytes().to_vec()),
            0,
        );
    }
}

#[cfg(not(feature = "KHR_materials_emissive_strength"))]
fn handle_emissive_strength(
    _embedded_tex_ids: &HashMap<usize, usize>,
    _material: &gltf::Material<'_>,
    _ai_material: &mut AiMaterial,
) {
}
#[cfg(feature = "KHR_materials_emissive_strength")]
fn handle_emissive_strength(
    _embedded_tex_ids: &HashMap<usize, usize>,
    material: &gltf::Material<'_>,
    ai_material: &mut AiMaterial,
) {
    use crate::structs::matkey::AI_MATKEY_EMISSIVE_INTENSITY;
    if let Some(emissive_strength) = material.emissive_strength() {
        ai_material.add_property(
            AI_MATKEY_EMISSIVE_INTENSITY,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(emissive_strength.to_le_bytes().to_vec()),
            0,
        );
    }
}

#[test]
fn test_material_import() {
    let gltf_data = r#"{
            "asset": {
                "generator": "COLLADA2GLTF",
                "version": "2.0"
            },
            "scene": 0,
            "scenes": [
                {
                    "nodes": [
                        0
                    ]
                }
            ],
            "nodes": [
                {
                    "children": [
                        1
                    ],
                    "matrix": [
                        1.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        -1.0,
                        0.0,
                        0.0,
                        1.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        1.0
                    ]
                },
                {
                    "mesh": 0
                }
            ],
            "meshes": [
                {
                    "primitives": [
                        {
                            "attributes": {
                                "NORMAL": 1,
                                "POSITION": 2
                            },
                            "indices": 0,
                            "mode": 4,
                            "material": 0
                        }
                    ],
                    "name": "Mesh"
                }
            ],
            "accessors": [
                {
                    "bufferView": 0,
                    "byteOffset": 0,
                    "componentType": 5123,
                    "count": 36,
                    "max": [
                        23
                    ],
                    "min": [
                        0
                    ],
                    "type": "SCALAR"
                },
                {
                    "bufferView": 1,
                    "byteOffset": 0,
                    "componentType": 5126,
                    "count": 24,
                    "max": [
                        1.0,
                        1.0,
                        1.0
                    ],
                    "min": [
                        -1.0,
                        -1.0,
                        -1.0
                    ],
                    "type": "VEC3"
                },
                {
                    "bufferView": 1,
                    "byteOffset": 288,
                    "componentType": 5126,
                    "count": 24,
                    "max": [
                        0.5,
                        0.5,
                        0.5
                    ],
                    "min": [
                        -0.5,
                        -0.5,
                        -0.5
                    ],
                    "type": "VEC3"
                }
            ],
            "materials": [
                {
                    "pbrMetallicRoughness": {
                        "baseColorFactor": [
                            0.800000011920929,
                            0.0,
                            0.0,
                            1.0
                        ],
                        "metallicFactor": 0.0
                    },
                    "name": "Red"
                }
            ],
            "bufferViews": [
                {
                    "buffer": 0,
                    "byteOffset": 576,
                    "byteLength": 72,
                    "target": 34963
                },
                {
                    "buffer": 0,
                    "byteOffset": 0,
                    "byteLength": 576,
                    "byteStride": 12,
                    "target": 34962
                }
            ],
            "buffers": [
                {
                    "byteLength": 648,
                    "uri": "Box0.bin"
                }
            ]
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let materials = Gltf2Importer::import_embedded_materials(&document, &HashMap::new()).unwrap();
    assert_eq!(1, materials.len())
}
