use std::{collections::HashMap, fs, io::BufReader, path::Path};

use enumflags2::BitFlags;
use gltf::{
    buffer, image,
    material::{NormalTexture, OcclusionTexture},
    texture::{self, WrappingMode},
    Document, Gltf, Mesh, Semantic,
};

use crate::{
    core::{
        error::AiReadError,
        import::AiImport,
        importer::AiImporter,
        importer_desc::{AiImporterDesc, AiImporterFlags},
    },
    structs::{
        base_types::AiReal,
        matkey::{
            self, _AI_MATKEY_MAPPINGMODE_U_BASE, _AI_MATKEY_MAPPINGMODE_V_BASE,
            _AI_MATKEY_TEXTURE_BASE,
        },
        scene::AiScene,
        AiColor3D, AiColor4D, AiMaterial, AiMesh, AiPrimitiveType, AiPropertyTypeInfo, AiTexel,
        AiTexture, AiTextureFormat, AiTextureMapMode, AiTextureType, AiVector3D,
    },
};

use super::gltf2_error::Gtlf2Error;

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
    fn texture_transform(&self) -> Option<gltf::texture::TextureTransform<'a>>;
}

impl<'a> ImportTexture<'a> for texture::Info<'a> {
    fn texture(&self) -> gltf::Texture<'a> {
        self.texture()
    }

    fn tex_coord(&self) -> u32 {
        self.tex_coord()
    }

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

    fn texture_transform(&self) -> Option<gltf::texture::TextureTransform<'a>> {
        None
    }
}

trait GetPointer {
    fn get_pointer<'a>(&self, buffers: &'a [buffer::Data]) -> Result<Vec<u8>, Gtlf2Error>;
}

impl<'b> GetPointer for gltf::Accessor<'b> {
    fn get_pointer<'a>(&self, buffers: &'a [buffer::Data]) -> Result<Vec<u8>, Gtlf2Error> {
        //Get Base Result
        //Result is guarenteed to have a length of size * count
        let mut result = if let Some(view) = self.view() {
            //Load Accessor Buffer
            let data_index = view.buffer().index();

            let start_index = self.offset() + view.offset();
            let count = self.count();
            let size = self.size();
            let stride = view.stride().unwrap_or(0);

            let end_index = start_index + (stride * (count - 1)) + (count * size);
            let data = buffers
                .get(data_index)
                .ok_or(Gtlf2Error::MissingBufferData)?;
            if end_index > data.len() {
                return Err(Gtlf2Error::ExceedsBounds);
            }

            //Copy Data into Result
            let mut result = Vec::new();
            result.reserve(count * size);
            let sliced = &data[start_index..end_index];
            for i in (0..sliced.len()).step_by(stride) {
                if i + size <= sliced.len() {
                    result.extend_from_slice(&sliced[i..i + size]);
                }
            }
            result
        } else {
            //Early Out as we must be Sparse if we don't have a view
            if self.sparse().is_none() {
                return Err(Gtlf2Error::BrokenSparseDataAccess);
            }
            //Fill up size * count with a bunch of zeroes
            let mut result = Vec::new();
            let count = self.count();
            let size = self.size();
            result.resize(count * size, 0u8);
            result
        };

        //Handle Sparse Data
        if let Some(sparse) = self.sparse() {
            //Load Index Data Buffer
            let index_data_index = sparse.indices().view().buffer().index();
            let index_data_start_index = sparse.indices().offset();
            let index_data_size = sparse.indices().index_type().size();
            let index_data_end_index = index_data_start_index + index_data_size * sparse.count();
            let index_data = buffers
                .get(index_data_index)
                .ok_or(Gtlf2Error::MissingBufferData)?;
            if index_data_end_index > index_data.len() {
                return Err(Gtlf2Error::ExceedsBounds);
            }

            //Load Value Data Buffer
            let values_data_index = sparse.values().view().buffer().index();
            let values_data_start_index = sparse.values().offset();
            let values_data_size = self.data_type().size();
            let values_data_end_index = values_data_start_index + values_data_size * sparse.count();
            let values_data = buffers
                .get(values_data_index)
                .ok_or(Gtlf2Error::MissingBufferData)?;
            if values_data_end_index > values_data.len() {
                return Err(Gtlf2Error::ExceedsBounds);
            }

            //Get Indices and Values
            let index_data_slice = &index_data[index_data_start_index..index_data_end_index]; //Should be index_data_size * sparse.count() length
            let indices: Vec<usize> = match sparse.indices().index_type() {
                gltf::accessor::sparse::IndexType::U8 => {
                    index_data_slice.iter().map(|&byte| byte as usize).collect()
                }
                gltf::accessor::sparse::IndexType::U16 => index_data_slice
                    .chunks_exact(2)
                    .map(|chunk| usize::from(u16::from_le_bytes([chunk[0], chunk[1]])))
                    .collect(),
                gltf::accessor::sparse::IndexType::U32 => index_data_slice
                    .chunks_exact(4)
                    .map(|chunk| {
                        u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as usize
                    })
                    .collect(),
            }; //Should be sparse.count() length due to chunking on index_data_size
            let values = &values_data[values_data_start_index..values_data_end_index]; //Should be values_data_size * sparse.count() length

            //Replace Indices/Values in result
            for i in 0..indices.len() {
                let index = values_data_size * indices[i]; //Map Index for Packed Data to Index for Unpacked Data

                //Get Value
                let start_index = values_data_size * i;
                let end_index = start_index + values_data_size;
                let value = &values[start_index..end_index]; //Get the next values_data_size from i

                result[index..index + values_data_size].copy_from_slice(value);
            }
        }
        Ok(result)
    }
}

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
        let path_ref = path.as_ref();
        let base = path_ref.parent().unwrap_or_else(|| Path::new("./"));
        let file_result =
            fs::File::open(path_ref).map_err(|x| AiReadError::FileOpenError(Box::new(x)))?;
        let reader = BufReader::new(file_result);

        //Load Gltf Info
        let Gltf { document, blob } =
            Gltf::from_reader(reader).map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        //@todo: Buffer Data loads all Buffer Data, it would be better to load on an "as-needed case".
        let buffer_data = gltf::import_buffers(&document, Some(base), blob)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        let (embedded_textures, embedded_tex_ids) =
            Gltf2Importer::import_embedded_textures(&document, Some(base), &buffer_data)?;
        let embedded_materials =
            Gltf2Importer::import_embedded_materials(&document, &embedded_tex_ids)?;

        Err(AiReadError::UnsupportedImageFormat("t".to_string(), "t".to_string()))
    }
}

impl Gltf2Importer {
    fn import_meshes<'a>(
        document: &'a Document,
        buffer_data: &'a [buffer::Data],
    ) -> Result<Vec<AiMesh<'a>>, AiReadError> {
        let asset_meshes: Vec<Mesh<'_>> = document.meshes().collect();

        //Maps Document Mesh Index to Offset. Lets us add all primitives to a Node as Meshes
        //GLTF2 only allows one mesh per node and uses primitives for multiple groups, whilst Assimp has many
        let mut mesh_offsets: Vec<u32> = Vec::new();
        mesh_offsets.reserve(asset_meshes.len() + 1);
        let mut cumulative_meshes = 0;
        for mesh in asset_meshes.iter() {
            mesh_offsets.push(cumulative_meshes);
            cumulative_meshes += mesh.primitives().len() as u32;
        }
        mesh_offsets.push(cumulative_meshes); // add a last element so we can always do mesh_offsets[n+1] - mesh_offsets[n]

        let mut meshes: Vec<AiMesh> = Vec::new(); //Final Meshes
        let mut vertex_remapping_tables: Vec<Vec<u32>> = Vec::new(); //For Each Mesh, how do we map indices to positions

        let mut reverse_mapping_indices: Vec<u32> = Vec::new(); //Indices
        let mut index_buffer: Vec<u32> = Vec::new(); //Maps vertices back to original indices.
        meshes.reserve(cumulative_meshes as usize);
        vertex_remapping_tables.resize(cumulative_meshes as usize, Vec::new());
        for mesh in asset_meshes.iter() {
            for (p, primitive) in mesh.primitives().enumerate() {
                //Get Accessor Count on how many Vertices exist and need to be copied
                let mut num_all_vertices: usize = 0;
                for (sem, acc) in primitive.attributes() {
                    if sem == Semantic::Positions {
                        num_all_vertices = acc.count();
                    }
                }

                //GLTF Files can use an Index Buffer, wherein their vertices are indexed. These Indexes are even used for Weight Mapping.
                //
                let mut use_index_buffer = false;
                let mut vertex_remapping_table: Option<&Vec<u32>> = None;
                if let Some(indices) = primitive.indices() {
                    use_index_buffer = true;
                    let count = indices.count();
                    let vertex_remap_table = &mut vertex_remapping_tables[meshes.len()];
                    vertex_remap_table.reserve(count / 3);

                    let index_data = indices
                        .get_pointer(buffer_data)
                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?; //Returns bytes equal with length equal to indices.count() * size

                    for i in 0..count {
                        let index = u32::from_le_bytes([
                            index_data[i * 4],
                            index_data[i * 4 + 1],
                            index_data[i * 4 + 2],
                            index_data[i * 4 + 3],
                        ]);
                        let index_usize: usize = index as usize;
                        if index_usize >= num_all_vertices {
                            index_buffer[i] = index;
                            continue;
                        }
                        if index_usize >= reverse_mapping_indices.len() {
                            reverse_mapping_indices.resize(index_usize + 1, 0);
                        }
                        if (reverse_mapping_indices[index_usize] == 0) {
                            reverse_mapping_indices[index_usize] = vertex_remap_table.len() as u32;
                            vertex_remap_table.push(index);
                        }
                        index_buffer[i] = reverse_mapping_indices[index_usize];
                    }
                    vertex_remapping_table = Some(vertex_remap_table);
                }

                let mut ai_mesh = AiMesh::default();
                ai_mesh.name = mesh
                    .name()
                    .map(|x| x.to_string())
                    .unwrap_or(mesh.index().to_string());

                if mesh.primitives().len() > 1 {
                    ai_mesh.name = format!("{}-{}", ai_mesh.name, p);
                }

                match primitive.mode() {
                    gltf::mesh::Mode::Points => ai_mesh.primitive_types |= AiPrimitiveType::Point,
                    gltf::mesh::Mode::Lines
                    | gltf::mesh::Mode::LineLoop
                    | gltf::mesh::Mode::LineStrip => {
                        ai_mesh.primitive_types |= AiPrimitiveType::Line
                    }
                    gltf::mesh::Mode::Triangles
                    | gltf::mesh::Mode::TriangleStrip
                    | gltf::mesh::Mode::TriangleFan => {
                        ai_mesh.primitive_types |= AiPrimitiveType::Triangle
                    }
                }

                if let Some((_, attr_position)) =
                    primitive.attributes().find(|x| x.0 == Semantic::Positions)
                {
                    //Position Accessor must be VEC3 of type float (f32), thus they must chunk by 3 * 4 = 12
                    let data = attr_position
                        .get_pointer(&buffer_data)
                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                    debug_assert!(
                        data.len()  == attr_position.count() * 12, 
                        "Position Accessor must be VEC3 of type float (f32), the resultant pointer should therefore always be 12x as big");

                    //Get Vertices
                    ai_mesh.vertices = if let Some(remap) = vertex_remapping_table {
                        //If we have Remap, prepare the vertices and then chunk them in
                        let mut vertices: Vec<AiVector3D> = Vec::new();
                        vertices.resize(attr_position.count(), AiVector3D::default());
                        for (index, chunk) in data
                            .chunks_exact(12)
                            .map(|chunk| {
                                let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                    as AiReal;
                                let y = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                    as AiReal;
                                let z = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                    as AiReal;
                                AiVector3D::new(x, y, z)
                            })
                            .enumerate()
                        {
                            vertices[remap[index] as usize] = chunk;
                        }
                        vertices
                    } else {
                        //Vertices are already sorted, no remap neccessary. Only happens when we have no Indices
                        data.chunks_exact(12)
                            .map(|chunk| {
                                let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                    as AiReal;
                                let y = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                    as AiReal;
                                let z = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                                    as AiReal;
                                AiVector3D::new(x, y, z)
                            })
                            .collect()
                    }
                }
                meshes.push(ai_mesh);
            }
        }
        Ok(meshes)
    }

    fn import_embedded_materials(
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
    mut ai_material: &mut AiMaterial,
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

    // Handle Metallic Roughness Texture
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
            AI_MATKEY_COLOR_DIFFUSE,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(
                bytemuck::bytes_of(&AiColor4D::from(specular.diffuse_factor())).to_vec(),
            ),
            0,
        );
        ai_material.add_property(
            AI_MATKEY_COLOR_SPECULAR,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(
                bytemuck::bytes_of(&AiColor3D::from(specular.specular_factor())).to_vec(),
            ),
            0,
        );
        let shininess = specular.glossiness_factor() * 1000.0;
        ai_material.add_property(
            AI_MATKEY_SHININESS,
            Some(AiTextureType::None),
            AiPropertyTypeInfo::Binary(shininess.to_le_bytes().to_vec()),
            0,
        );
        ai_material.add_property(
            AI_MATKEY_GLOSSINESS_FACTOR,
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
    ai_material.add_property(
        AI_MATKEY_SHADING_MODEL,
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
