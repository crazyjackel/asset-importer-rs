use std::{collections::HashMap, fs, io::BufReader, path::Path};

use enumflags2::BitFlags;
use gltf::{
    buffer, image,
    texture::{self, WrappingMode},
    Document, Gltf,
};

use crate::{
    core::{
        error::AiReadError,
        import::AiImport,
        importer::AiImporter,
        importer_desc::{AiImporterDesc, AiImporterFlags},
    },
    structs::{
        matkey::{self, _AI_MATKEY_MAPPINGMODE_U_BASE, _AI_MATKEY_MAPPINGMODE_V_BASE}, scene::AiScene, AiColor4D, AiMaterial, AiPropertyTypeInfo, AiTexel, AiTexture, AiTextureFormat, AiTextureMapMode, AiTextureType
    },
};

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

#[derive(Debug)]
pub struct Gltf2Importer;

impl AiImport for Gltf2Importer {
    fn info(&self) -> AiImporterDesc {
        AiImporterDesc {
            name: "glTF2 Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: Default::default(),
            flags: BitFlags::from(
                AiImporterFlags::SupportBinaryFlavor
                    | AiImporterFlags::LimitedSupport
                    | AiImporterFlags::SupportTextFlavor
                    | AiImporterFlags::Experimental,
            ),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["gltf".to_string(), "glb".to_string(), "vrm".to_string()],
        }
    }

    fn can_read<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        //Match Extension Guard Clause
        match path.as_ref().extension() {
            None => {
                return false;
            }
            Some(os_str) => match os_str.to_str() {
                Some("gltf") => {}
                Some("glb") => {}
                Some("vrm") => {}
                Some(_) | None => return false,
            },
        };
        //Check if File can be Opened
        let file_result = fs::File::open(path);
        if file_result.is_err() {
            return false;
        }

        //Attempt to Read JSON
        let file = file_result.unwrap();
        let reader = BufReader::new(file);
        let gltf = Gltf::from_reader(reader);

        //If Result is Good, we can Read
        !gltf.is_err()
    }

    fn read_file<P>(&self, importer: &mut AiImporter, path: P) -> Result<AiScene, AiReadError>
    where
        P: AsRef<std::path::Path>,
    {
        //Collect File Info
        let base = path.as_ref().parent().unwrap_or_else(|| Path::new("./"));
        let file_result =
            fs::File::open(path).map_err(|x| AiReadError::FileOpenError(Box::new(x)))?;
        let reader = BufReader::new(file_result);

        //Load Gltf Info
        let Gltf { document, blob } =
            Gltf::from_reader(reader).map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        //@todo: Buffer Data loads all Buffer Data, it would be better to load on an "as-needed case".
        let buffer_data = gltf::import_buffers(&document, Some(base), blob)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        let (embedded_textures, embedded_tex_ids) =
            Gltf2Importer::import_embedded_textures(&document, Some(base), &buffer_data)?;

        Ok(AiScene {})
    }
}

impl Gltf2Importer {
    fn import_embedded_materials(
        document: &Document,
        embedded_tex_ids: &HashMap<usize, usize>,
    ) -> Result<Vec<AiMaterial>, AiReadError> {
        let mut materials: Vec<AiMaterial> = Vec::new();
        for material in document.materials() {
            let mut ai_material = AiMaterial::new();

            //Name
            if let Some(name) = material.name() {
                ai_material.add_property(
                    matkey::AI_MATKEY_NAME,
                    Some(AiTextureType::None),
                    AiPropertyTypeInfo::Binary(name.bytes().collect()),
                    0,
                );
            }

            // PBR_METALLIC_ROUGHNESS
            let pbr_metallic_roughness = material.pbr_metallic_roughness();
            // Set Assimp DIFFUSE and BASE COLOR to the pbrMetallicRoughness base color and texture for backwards compatibility
            // Technically should not load any pbrMetallicRoughness if extensionsRequired contains KHR_materials_pbrSpecularGlossiness
            ai_material.add_property(
                matkey::AI_MATKEY_COLOR_DIFFUSE,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(bytemuck::bytes_of(&AiColor4D::from(pbr_metallic_roughness.base_color_factor())).to_vec()),
                0,
            );
            ai_material.add_property(
                matkey::AI_MATKEY_BASE_COLOR,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(bytemuck::bytes_of(&AiColor4D::from(pbr_metallic_roughness.base_color_factor())).to_vec()),
                0,
            );

            import_texture_property(
                &pbr_metallic_roughness.base_color_texture(),
                &mut ai_material,
                embedded_tex_ids,
                AiTextureType::Diffuse,
                0,
            );
            import_texture_property(
                &pbr_metallic_roughness.base_color_texture(),
                &mut ai_material,
                embedded_tex_ids,
                AiTextureType::BaseColor,
                0,
            );

            // Keep AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE for backwards compatibility
            import_texture_property(
                &pbr_metallic_roughness.metallic_roughness_texture(),
                &mut ai_material,
                embedded_tex_ids,
                AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE,
                0,
            );
            import_texture_property(
                &pbr_metallic_roughness.metallic_roughness_texture(),
                &mut ai_material,
                embedded_tex_ids,
                AiTextureType::Metalness,
                0,
            );
            import_texture_property(
                &pbr_metallic_roughness.metallic_roughness_texture(),
                &mut ai_material,
                embedded_tex_ids,
                AiTextureType::DiffuseRoughness,
                0,
            );
            ai_material.add_property(
                matkey::AI_MATKEY_METALLIC_FACTOR,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(pbr_metallic_roughness.metallic_factor().to_le_bytes().to_vec()),
                0,
            );
            ai_material.add_property(
                matkey::AI_MATKEY_ROUGHNESS_FACTOR,
                Some(AiTextureType::None),
                AiPropertyTypeInfo::Binary(pbr_metallic_roughness.roughness_factor().to_le_bytes().to_vec()),
                0,
            );
        }
        Ok(materials)
    }

    fn import_embedded_textures(
        document: &Document,
        base: Option<&Path>,
        buffer_data: &[buffer::Data],
    ) -> Result<(Vec<AiTexture>, HashMap<usize, usize>), AiReadError> {
        let mut textures: Vec<AiTexture> = Vec::new();
        let mut embedded_tex_ids: HashMap<usize, usize> = HashMap::new();
        for image in document.images() {
            let source = image.source();
            let (filename, mime_type) = match source {
                image::Source::View { view: _, mime_type } => {
                    (image.index().to_string(), Some(mime_type))
                }
                image::Source::Uri { uri, mime_type } => (
                    if let Some(pos) = uri.find('.') {
                        &uri[..pos]
                    } else {
                        uri
                    }
                    .to_string(),
                    mime_type,
                ),
            };
            let format = match mime_type {
                Some(mime_type_str) => match mime_type_str {
                    "image/jpeg" => AiTextureFormat::JPEG,
                    "image/png" => AiTextureFormat::Png,
                    _ => AiTextureFormat::Unknown,
                },
                None => AiTextureFormat::Unknown,
            };

            let data: image::Data = image::Data::from_source(image.source(), base, buffer_data)
                .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

            let texels: Vec<AiTexel> = match data.format {
                image::Format::R8 => data
                    .pixels
                    .into_iter()
                    .map(|r| AiTexel::new(r, r, r, 255))
                    .collect(),
                image::Format::R8G8 => data
                    .pixels
                    .chunks_exact(2)
                    .map(|x| AiTexel::new(x[0], x[1], 0, 255))
                    .collect(),
                image::Format::R8G8B8 => data
                    .pixels
                    .chunks_exact(3)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], 255))
                    .collect(),
                image::Format::R8G8B8A8 => data
                    .pixels
                    .chunks_exact(4)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], chunk[3]))
                    .collect(),
                image::Format::R16 => {
                    data.pixels
                        .chunks_exact(2)
                        .map(|chunk| {
                            let r = chunk[0]; // Take the most significant byte
                            AiTexel::new(r, r, r, 255)
                        })
                        .collect()
                }
                image::Format::R16G16 => data
                    .pixels
                    .chunks_exact(4)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], 0, 255))
                    .collect(),
                image::Format::R16G16B16 => data
                    .pixels
                    .chunks_exact(6)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], 255))
                    .collect(),
                image::Format::R16G16B16A16 => data
                    .pixels
                    .chunks_exact(8)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], chunk[6]))
                    .collect(),
                image::Format::R32G32B32FLOAT => data
                    .pixels
                    .chunks_exact(12)
                    .map(|chunk| {
                        let r = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
                        let g = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
                        let b = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
                        AiTexel::new(
                            (r.clamp(0.0, 1.0) * 255.0) as u8,
                            (g.clamp(0.0, 1.0) * 255.0) as u8,
                            (b.clamp(0.0, 1.0) * 255.0) as u8,
                            255,
                        )
                    })
                    .collect(),
                image::Format::R32G32B32A32FLOAT => data
                    .pixels
                    .chunks_exact(16)
                    .map(|chunk| {
                        let r = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
                        let g = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
                        let b = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
                        let a = f32::from_le_bytes(chunk[12..16].try_into().unwrap());
                        AiTexel::new(
                            (r.clamp(0.0, 1.0) * 255.0) as u8,
                            (g.clamp(0.0, 1.0) * 255.0) as u8,
                            (b.clamp(0.0, 1.0) * 255.0) as u8,
                            (a.clamp(0.0, 1.0) * 255.0) as u8,
                        )
                    })
                    .collect(),
            };

            textures.push(AiTexture::new(
                filename,
                data.width,
                data.height,
                format,
                texels,
            ));
            embedded_tex_ids.insert(image.index(), textures.len() - 1);
        }
        Ok((textures, embedded_tex_ids))
    }
}

fn import_texture_property(
    texture_info_ref: &Option<texture::Info<'_>>,
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
            Some(str) => format!("*{}", str.to_string()),
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
                AiPropertyTypeInfo::Binary(id.to_le_bytes().to_vec()),
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

fn convert_map_mode(wrap_mode: WrappingMode) -> AiTextureMapMode {
    match wrap_mode {
        WrappingMode::ClampToEdge => AiTextureMapMode::Clamp,
        WrappingMode::MirroredRepeat => AiTextureMapMode::Mirror,
        WrappingMode::Repeat => AiTextureMapMode::Wrap,
    }
}

#[cfg(not(feature = "KHR_texture_transform"))]
fn handle_texture_transform2(
    _texture_info: &gltf::texture::Info<'_>,
    _ai_material: &mut AiMaterial,
    _texture_type: AiTextureType,
    _texture_index: u32,
) {
}
#[cfg(feature = "KHR_texture_transform")]
fn handle_texture_transform(
    texture_info: &gltf::texture::Info<'_>,
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
