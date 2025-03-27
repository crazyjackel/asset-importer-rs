use bytemuck::Pod;
use gltf::{buffer, Document, Mesh, Semantic};

use crate::{
    core::error::AiReadError,
    structs::{
        base_types::AiReal, AiAnimMesh, AiColor4D, AiMesh, AiPrimitiveType, AiVector3D,
        AI_MAX_NUMBER_OF_COLORS_SETS, AI_MAX_NUMBER_OF_TEXTURECOORDS,
    },
};

use super::{gltf2_error::Gtlf2Error, gltf2_importer::Gltf2Importer};

pub(crate) trait ExtractData {
    fn extract_data<T>(
        &self,
        buffers: &[buffer::Data],
        vertex_remapping_table: Option<&Vec<usize>>,
    ) -> Result<Vec<T>, Gtlf2Error>
    where
        T: Sized + Default + Pod;
}

impl ExtractData for gltf::Accessor<'_> {
    fn extract_data<T>(
        &self,
        buffers: &[buffer::Data],
        remapping_indices: Option<&Vec<usize>>,
    ) -> Result<Vec<T>, Gtlf2Error>
    where
        T: Sized + Default + Pod,
    {
        let target_elem_size = size_of::<T>();
        let mut result = if let Some(view) = self.view() {
            //Get Buffer Pointer
            let data_index = view.buffer().index();
            let data = buffers
                .get(data_index)
                .ok_or(Gtlf2Error::MissingBufferData)?;

            let elem_size = self.size(); //how large each element is
            let count = self.count(); //how many elements there is
            let stride = match view.stride() { //how many bytes to move to get the next element
                Some(0) | None => elem_size,
                Some(s) => s,
            };

            if stride < elem_size {
                return Err(Gtlf2Error::InvalidStride);
            }

            if elem_size > target_elem_size {
                return Err(Gtlf2Error::SizeExceedsTarget);
            }

            //Get Slice of Data
            let start_index = self.offset() + view.offset();
            let end_index = start_index + (count - 1) * stride + elem_size; //The Last Element
            if end_index > data.len() {
                return Err(Gtlf2Error::ExceedsBounds);
            }
            let data_slice = &data[start_index..end_index];

            //Copy Data into Result
            let mut result = Vec::with_capacity(count);
            if let Some(remap) = remapping_indices {
                //bytemuck::from_bytes requires that slice matches size of T.
                if target_elem_size == elem_size {
                    for src_index in remap {
                        if src_index >= &count {
                            return Err(Gtlf2Error::ExceedsBounds);
                        }
                        let start = src_index * stride;
                        let end = start + elem_size;
                        let element = bytemuck::from_bytes::<T>(&data_slice[start..end]);
                        result.push(*element);
                    }
                } else {
                    for src_index in remap {
                        if src_index >= &count {
                            return Err(Gtlf2Error::ExceedsBounds);
                        }
                        let start = src_index * stride;
                        let end = start + elem_size;
                        let mut output: Vec<u8> = Vec::with_capacity(target_elem_size);
                        output.extend_from_slice(&data_slice[start..end]);
                        output.resize(target_elem_size, 0);
                        let element = bytemuck::from_bytes::<T>(&output);
                        result.push(*element);
                    }
                }
            } else if stride == elem_size && target_elem_size == elem_size {
                result.extend_from_slice(bytemuck::cast_slice::<u8, T>(data_slice));
            } else if target_elem_size == elem_size {
                for i in 0..count {
                    let start = i * stride;
                    let end = start + elem_size;
                    let element = bytemuck::from_bytes::<T>(&data_slice[start..end]);
                    result.push(*element);
                }
            } else {
                for i in 0..count {
                    let start = i * stride;
                    let end = start + elem_size;
                    let mut output: Vec<u8> = Vec::with_capacity(target_elem_size);
                    output.extend_from_slice(&data_slice[start..end]);
                    output.resize(target_elem_size, 0);
                    let element = bytemuck::from_bytes::<T>(&output);
                    result.push(*element);
                }
            }
            result
        } else {
            //Early Out as we must be Sparse if we don't have a view
            if self.sparse().is_none() {
                return Err(Gtlf2Error::BrokenSparseDataAccess);
            }
            let count = self.count();
            let mut result = Vec::with_capacity(count);
            result.resize(count, T::default());
            result
        };

        //Handle Sparse Data
        if let Some(sparse) = self.sparse() {
            //Load Index Data Buffer
            let index_data_index = sparse.indices().view().buffer().index();
            let index_data_start_index =
                sparse.indices().offset() + sparse.indices().view().offset();
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
            let values_data_start_index =
                sparse.values().offset() + sparse.values().view().offset();
            let values_data_size = self.data_type().size() * self.dimensions().multiplicity();
            let values_data_end_index = values_data_start_index + values_data_size * sparse.count();
            let values_data = buffers
                .get(values_data_index)
                .ok_or(Gtlf2Error::MissingBufferData)?;
            if values_data_end_index > values_data.len() {
                return Err(Gtlf2Error::ExceedsBounds);
            }

            //Get Indices and Values
            let index_data_slice = &index_data[index_data_start_index..index_data_end_index]; //Should be index_data_size * sparse.count() length
            let sparse_indices: Vec<usize> = match sparse.indices().index_type() {
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
            for (i, sparse_index) in sparse_indices.iter().enumerate() {
                // let index = values_data_size * sparse_index; //Map Index for Packed Data to Index for Unpacked Data

                //Get Value
                let start_index = values_data_size * i;
                let end_index = start_index + values_data_size;
                let value = &values[start_index..end_index]; //Get the next values_data_size from i

                result[*sparse_index] = *bytemuck::from_bytes::<T>(value);
            }
        }

        Ok(result)
    }
}

impl Gltf2Importer {
    pub(crate) fn import_meshes<'a>(
        document: &'a Document,
        buffer_data: &'a [buffer::Data],
        last_material_index: usize,
    ) -> Result<(Vec<AiMesh>, Vec<u32>, Vec<Vec<usize>>), AiReadError> {
        let asset_meshes: Vec<Mesh<'_>> = document.meshes().collect();

        //Maps Document Mesh Index to Offset. Lets us add all primitives to a Node as Meshes
        //GLTF2 only allows one mesh per node and uses primitives for multiple groups, whilst Assimp has many meshes per Node
        let mut mesh_offsets: Vec<u32> = Vec::with_capacity(asset_meshes.len() + 1);
        let mut cumulative_meshes = 0;
        for mesh in asset_meshes.iter() {
            mesh_offsets.push(cumulative_meshes);
            cumulative_meshes += mesh.primitives().len() as u32;
        }
        mesh_offsets.push(cumulative_meshes); // add a last element so we can always do mesh_offsets[n+1] - mesh_offsets[n]

        let mut meshes: Vec<AiMesh> = Vec::new(); //Final Meshes to return
        let mut vertex_remapping_tables: Vec<Vec<usize>> = Vec::new(); //For Each Mesh, how do we remap their indices. Is needed when building Nodes

        let mut reverse_mapping_indices: Vec<u32> = Vec::new(); //Indices
        let mut index_buffer: Vec<usize> = Vec::new(); //Maps vertices back to original indices.

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

                //If Primitive has Indices, build up a Remapping Table
                let mut use_index_buffer = false;
                let mut vertex_remapping_table: Option<&Vec<usize>> = None;
                if let Some(indices) = primitive.indices() {
                    use_index_buffer = true; //used to remember if we did this or not
                    let count = indices.count();

                    //recycle data structures
                    index_buffer.resize(count, 0);
                    reverse_mapping_indices.clear();

                    let vertex_remap_table = &mut vertex_remapping_tables[meshes.len()];
                    vertex_remap_table.reserve(count / 3);

                    let index_data: Vec<u32> = match indices.data_type() {
                        gltf::accessor::DataType::I8 | gltf::accessor::DataType::U8 => {
                            let index_data: Vec<u8> = indices
                                .extract_data(buffer_data, None)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            index_data.into_iter().map(|x| x as u32).collect()
                        }
                        gltf::accessor::DataType::I16 | gltf::accessor::DataType::U16 => {
                            let index_data: Vec<u16> = indices
                                .extract_data(buffer_data, None)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            index_data.into_iter().map(|x| x as u32).collect()
                        }
                        gltf::accessor::DataType::U32 | gltf::accessor::DataType::F32 => {
                            let index_data: Vec<u32> = indices
                                .extract_data(buffer_data, None)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            index_data
                        }
                    };

                    for i in 0..count {
                        let index = index_data[i];
                        let index_usize: usize = index as usize;
                        if index_usize >= num_all_vertices {
                            index_buffer[i] = index_usize;
                            continue;
                        }
                        if index_usize >= reverse_mapping_indices.len() {
                            reverse_mapping_indices.resize(index_usize + 1, u32::MAX);
                        }
                        if reverse_mapping_indices[index_usize] == u32::MAX {
                            reverse_mapping_indices[index_usize] = vertex_remap_table.len() as u32;
                            vertex_remap_table.push(index_usize);
                        }
                        index_buffer[i] = reverse_mapping_indices[index_usize] as usize;
                    }
                    vertex_remapping_table = Some(vertex_remap_table);
                }

                //Construct Mesh
                let mut ai_mesh = AiMesh {
                    name: mesh
                        .name()
                        .map(|x| x.to_string())
                        .unwrap_or(mesh.index().to_string()),
                    ..AiMesh::default()
                };

                if mesh.primitives().len() > 1 {
                    ai_mesh.name = format!("{}-{}", ai_mesh.name, p);
                }

                //Set Primitive Types
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

                //Handle Attribute Position
                if let Some((_, attr_position)) =
                    primitive.attributes().find(|x| x.0 == Semantic::Positions)
                {
                    let data: Vec<[f32; 3]> = attr_position
                        .extract_data(buffer_data, vertex_remapping_table)
                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                    ai_mesh.vertices = data
                        .iter()
                        .map(|x| AiVector3D::new(x[0] as AiReal, x[1] as AiReal, x[2] as AiReal))
                        .collect();
                }

                //Handle Normals, Tangents, and Bitangents
                let mut tangent_weights: Vec<AiReal> = Vec::new();
                if let Some((_, attr_normals)) = primitive
                    .attributes()
                    .find(|x: &(Semantic, gltf::Accessor<'_>)| x.0 == Semantic::Normals)
                {
                    if attr_normals.count() != num_all_vertices {
                        println!("Normal count in mesh \"{}\" does not match the vertex count, normals ignored.", ai_mesh.name);
                    } else {
                        let data: Vec<[f32; 3]> = attr_normals
                            .extract_data(buffer_data, vertex_remapping_table)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                        ai_mesh.normals = data
                            .iter()
                            .map(|x| {
                                AiVector3D::new(x[0] as AiReal, x[1] as AiReal, x[2] as AiReal)
                            })
                            .collect();

                        // only extract tangents if normals are present
                        if let Some((_, attr_tangents)) = primitive
                            .attributes()
                            .find(|x: &(Semantic, gltf::Accessor<'_>)| x.0 == Semantic::Tangents)
                        {
                            if attr_tangents.count() != num_all_vertices {
                                println!("Tangent count in mesh \"{}\" does not match the vertex count, tangents ignored.", ai_mesh.name);
                            } else {
                                let data: Vec<[f32; 4]> = attr_tangents
                                    .extract_data(buffer_data, vertex_remapping_table)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                                ai_mesh.tangents.resize(data.len(), AiVector3D::default());
                                ai_mesh
                                    .bi_tangents
                                    .resize(data.len(), AiVector3D::default());
                                for (i, tangent) in data.iter().enumerate() {
                                    let x = tangent[0] as AiReal;
                                    let y = tangent[1] as AiReal;
                                    let z = tangent[2] as AiReal;
                                    let w = tangent[3] as AiReal;
                                    ai_mesh.tangents[i] = AiVector3D::new(x, y, z);
                                    ai_mesh.bi_tangents[i] =
                                        (ai_mesh.normals[i] ^ AiVector3D::new(x, y, z)) * w;
                                    tangent_weights.push(w);
                                }
                            }
                        }
                    }
                }

                //Handle Colors
                let colors: Vec<(gltf::Accessor<'_>, u32)> = primitive
                    .attributes()
                    .filter_map(|x| match x.0 {
                        Semantic::Colors(n) if n < AI_MAX_NUMBER_OF_COLORS_SETS as u32 => {
                            Some((x.1, n))
                        }
                        _ => None,
                    })
                    .collect();
                for (attr_color, index) in colors {
                    ai_mesh.colors[index as usize] = match attr_color.dimensions() {
                        gltf::accessor::Dimensions::Vec3 => match attr_color.data_type() {
                            gltf::accessor::DataType::U8 => {
                                let data: Vec<[u8; 3]> = attr_color
                                    .extract_data(buffer_data, vertex_remapping_table)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                                Some(
                                    data.iter()
                                        .map(|chunk| {
                                            AiColor4D::new(
                                                chunk[0] as f32 / 255.0,
                                                chunk[1] as f32 / 255.0,
                                                chunk[2] as f32 / 255.0,
                                                1.0,
                                            )
                                        })
                                        .collect(),
                                )
                            }
                            gltf::accessor::DataType::U16 => {
                                let data: Vec<[u16; 3]> = attr_color
                                    .extract_data(buffer_data, vertex_remapping_table)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                                Some(
                                    data.iter()
                                        .map(|chunk| {
                                            AiColor4D::new(
                                                chunk[0] as f32 / 65535.0,
                                                chunk[1] as f32 / 65535.0,
                                                chunk[2] as f32 / 65535.0,
                                                1.0,
                                            )
                                        })
                                        .collect(),
                                )
                            }
                            gltf::accessor::DataType::F32 => {
                                let data: Vec<[f32; 3]> = attr_color
                                    .extract_data(buffer_data, vertex_remapping_table)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                                Some(
                                    data.iter()
                                        .map(|chunk| {
                                            AiColor4D::new(chunk[0], chunk[1], chunk[2], 1.0)
                                        })
                                        .collect(),
                                )
                            }
                            _ => None,
                        },
                        gltf::accessor::Dimensions::Vec4 => match attr_color.data_type() {
                            gltf::accessor::DataType::U8 => {
                                let data: Vec<[u8; 4]> = attr_color
                                    .extract_data(buffer_data, vertex_remapping_table)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                                Some(
                                    data.iter()
                                        .map(|chunk| {
                                            AiColor4D::new(
                                                chunk[0] as f32 / 255.0,
                                                chunk[1] as f32 / 255.0,
                                                chunk[2] as f32 / 255.0,
                                                chunk[3] as f32 / 255.0,
                                            )
                                        })
                                        .collect(),
                                )
                            }
                            gltf::accessor::DataType::U16 => {
                                let data: Vec<[u16; 4]> = attr_color
                                    .extract_data(buffer_data, vertex_remapping_table)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                                Some(
                                    data.iter()
                                        .map(|chunk| {
                                            AiColor4D::new(
                                                chunk[0] as f32 / 65535.0,
                                                chunk[1] as f32 / 65535.0,
                                                chunk[2] as f32 / 65535.0,
                                                chunk[3] as f32 / 65535.0,
                                            )
                                        })
                                        .collect(),
                                )
                            }
                            gltf::accessor::DataType::F32 => {
                                let data: Vec<[f32; 4]> = attr_color
                                    .extract_data(buffer_data, vertex_remapping_table)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                                Some(
                                    data.iter()
                                        .map(|chunk| {
                                            AiColor4D::new(chunk[0], chunk[1], chunk[2], chunk[3])
                                        })
                                        .collect(),
                                )
                            }
                            _ => None,
                        },
                        _ => None,
                    }
                }

                //Handle Textures
                let texcoords: Vec<(gltf::Accessor<'_>, u32)> = primitive
                    .attributes()
                    .filter_map(|x| match x.0 {
                        Semantic::TexCoords(n) if n < AI_MAX_NUMBER_OF_TEXTURECOORDS as u32 => {
                            Some((x.1, n))
                        }
                        _ => None,
                    })
                    .collect();
                for (attr_texcoords, index) in texcoords {
                    ai_mesh.texture_coords[index as usize] = match attr_texcoords.data_type() {
                        gltf::accessor::DataType::U8 => {
                            let data: Vec<[u8; 2]> = attr_texcoords
                                .extract_data(buffer_data, vertex_remapping_table)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            Some(
                                data.iter()
                                    .map(|chunk| {
                                        let u = (chunk[0] as f32 / 255.0) as AiReal;
                                        let v = (1.0 - (chunk[1] as f32 / 255.0)) as AiReal;
                                        AiVector3D::new(u, v, 0.0)
                                    })
                                    .collect(),
                            )
                        }
                        gltf::accessor::DataType::U16 => {
                            let data: Vec<[u16; 2]> = attr_texcoords
                                .extract_data(buffer_data, vertex_remapping_table)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            Some(
                                data.iter()
                                    .map(|chunk| {
                                        let u = (chunk[0] as f32 / 65535.0) as AiReal;
                                        let v = (1.0 - (chunk[1] as f32 / 65535.0)) as AiReal;
                                        AiVector3D::new(u, v, 0.0)
                                    })
                                    .collect(),
                            )
                        }
                        gltf::accessor::DataType::F32 => {
                            let data: Vec<[f32; 2]> = attr_texcoords
                                .extract_data(buffer_data, vertex_remapping_table)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            Some(
                                data.iter()
                                    .map(|chunk| {
                                        let u = chunk[0] as AiReal;
                                        let v = 1.0 - chunk[1] as AiReal;
                                        AiVector3D::new(u, v, 0.0)
                                    })
                                    .collect(),
                            )
                        }
                        _ => None,
                    };
                }

                //Handle AnimMeshes
                //ai_mesh.anim_meshes should be an empty vec here
                if let Some(weights) = mesh.weights() {
                    let targets: Vec<gltf::mesh::MorphTarget<'_>> =
                        primitive.morph_targets().collect();
                    if targets.len() == weights.len() {
                        for i in 0..targets.len() {
                            let weight = weights[i];
                            let target = &targets[i];
                            //handle name
                            //@todo: names are defined via an extra value in mesh.extra.targetNames, it is safe to go without a name for now.
                            let mut anim_mesh = AiAnimMesh {
                                weight,
                                ..AiAnimMesh::default()
                            };

                            //handle position
                            if let Some(positions) = target.positions() {
                                if positions.count() == num_all_vertices {
                                    let data: Vec<[f32; 3]> = positions
                                        .extract_data(buffer_data, vertex_remapping_table)
                                        .map_err(|err| {
                                            AiReadError::FileFormatError(Box::new(err))
                                        })?;
                                    let offsets: Vec<AiVector3D> = data
                                        .iter()
                                        .map(|x| {
                                            AiVector3D::new(
                                                x[0] as AiReal,
                                                x[1] as AiReal,
                                                x[2] as AiReal,
                                            )
                                        })
                                        .collect();

                                    if offsets.len() == ai_mesh.vertices.len() {
                                        anim_mesh.vertices = ai_mesh.vertices.clone();
                                        for (i, offset) in offsets.iter().enumerate() {
                                            anim_mesh.vertices[i] += *offset;
                                        }
                                    }
                                }
                            }

                            //handle normals
                            if let Some(normals) = target.normals() {
                                if normals.count() == num_all_vertices {
                                    let data: Vec<[f32; 3]> = normals
                                        .extract_data(buffer_data, vertex_remapping_table)
                                        .map_err(|err| {
                                            AiReadError::FileFormatError(Box::new(err))
                                        })?;
                                    let offsets: Vec<AiVector3D> = data
                                        .iter()
                                        .map(|x| {
                                            AiVector3D::new(
                                                x[0] as AiReal,
                                                x[1] as AiReal,
                                                x[2] as AiReal,
                                            )
                                        })
                                        .collect();

                                    if offsets.len() == ai_mesh.normals.len() {
                                        anim_mesh.normals = ai_mesh.normals.clone();
                                        for (i, offset) in offsets.iter().enumerate() {
                                            anim_mesh.normals[i] += *offset;
                                        }
                                    }
                                }
                            }
                            //handle tangents
                            if let Some(tangents) = target.tangents() {
                                if tangents.count() == num_all_vertices
                                    && !anim_mesh.normals.is_empty()
                                {
                                    let tangents_offsets: Vec<[f32; 3]> = tangents
                                        .extract_data(buffer_data, vertex_remapping_table)
                                        .map_err(|err| {
                                            AiReadError::FileFormatError(Box::new(err))
                                        })?;

                                    if tangents_offsets.len() == ai_mesh.tangents.len() {
                                        anim_mesh.tangents = ai_mesh.tangents.clone();
                                        anim_mesh.bi_tangents = Vec::new();
                                        anim_mesh
                                            .bi_tangents
                                            .resize(ai_mesh.tangents.len(), Default::default());
                                        for i in 0..tangents_offsets.len() {
                                            let offset = tangents_offsets[i];
                                            anim_mesh.tangents[i] += AiVector3D::new(
                                                offset[0] as AiReal,
                                                offset[1] as AiReal,
                                                offset[2] as AiReal,
                                            );
                                            anim_mesh.bi_tangents[i] = (anim_mesh.normals[i]
                                                ^ anim_mesh.tangents[i])
                                                * tangent_weights[i]; //Saved Tangent Weights to prevent need for re-extraction
                                        }
                                    }
                                }
                            }
                            //we do not handle colors nor texture coords as gltf 2.0 does not provide that information, it should be assumed that info doesn't changed
                        }
                    }
                }

                //Handle Faces
                ai_mesh.faces = if use_index_buffer {
                    match primitive.mode() {
                        gltf::mesh::Mode::Points => {
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            for a in index_buffer.iter() {
                                if a >= &num_all_vertices {
                                    continue;
                                }
                                vec.push(vec![*a]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::Lines => {
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            for i in 0..(index_buffer.len() / 2) {
                                let a = index_buffer[2 * i];
                                let b = index_buffer[2 * i + 1];
                                if a >= num_all_vertices || b >= num_all_vertices {
                                    continue;
                                }
                                vec.push(vec![a, b]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::LineLoop | gltf::mesh::Mode::LineStrip => {
                            //Indices represent a path, in the case of a loop, it comes back around
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            for i in 0..(index_buffer.len() - 1) {
                                let a = index_buffer[i];
                                let b = index_buffer[i + 1];
                                if a >= num_all_vertices || b >= num_all_vertices {
                                    continue;
                                }
                                vec.push(vec![a, b]);
                            }
                            if primitive.mode() == gltf::mesh::Mode::LineLoop {
                                let a = index_buffer[index_buffer.len() - 1];
                                let b = index_buffer[0];
                                if a < num_all_vertices && b < num_all_vertices {
                                    vec.push(vec![a, b]);
                                }
                            }
                            vec
                        }
                        gltf::mesh::Mode::Triangles => {
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            for i in 0..(index_buffer.len() / 3) {
                                let a = index_buffer[3 * i];
                                let b = index_buffer[3 * i + 1];
                                let c = index_buffer[3 * i + 2];
                                if a >= num_all_vertices
                                    || b >= num_all_vertices
                                    || c >= num_all_vertices
                                {
                                    continue;
                                }
                                vec.push(vec![a, b, c]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::TriangleStrip => {
                            let mut vec: Vec<Vec<usize>> = Vec::new(); //Indices are strips of triangles
                            for i in 0..(index_buffer.len() - 2) {
                                if (i + 1) % 2 == 0 {
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    let a = index_buffer[i];
                                    let b = index_buffer[i + 1];
                                    let c = index_buffer[i + 2];
                                    if a >= num_all_vertices
                                        || b >= num_all_vertices
                                        || c >= num_all_vertices
                                    {
                                        continue;
                                    }
                                    vec.push(vec![b, a, c]);
                                } else {
                                    // For odd n, vertices n, n+1, and n+2 define triangle n
                                    let a = index_buffer[i];
                                    let b = index_buffer[i + 1];
                                    let c = index_buffer[i + 2];
                                    if a >= num_all_vertices
                                        || b >= num_all_vertices
                                        || c >= num_all_vertices
                                    {
                                        continue;
                                    }
                                    vec.push(vec![a, b, c]);
                                }
                            }
                            vec
                        }
                        gltf::mesh::Mode::TriangleFan => {
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            let a = index_buffer[0];
                            let b = index_buffer[1];
                            let c = index_buffer[2];
                            if a < num_all_vertices && b < num_all_vertices && c < num_all_vertices
                            {
                                vec.push(vec![a, b, c]);
                                for i in 1..(index_buffer.len() - 2) {
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    let d = index_buffer[i + 1];
                                    let e = index_buffer[i + 2];
                                    if d >= num_all_vertices || e >= num_all_vertices {
                                        continue;
                                    }
                                    vec.push(vec![a, d, e]);
                                }
                            }
                            vec
                        }
                    }
                } else {
                    match primitive.mode() {
                        gltf::mesh::Mode::Points => {
                            (0..ai_mesh.vertices.len()).map(|x| vec![x]).collect()
                            //Indices represent points
                        }
                        gltf::mesh::Mode::Lines => {
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            for i in 0..(ai_mesh.vertices.len() / 2) {
                                vec.push(vec![2 * i, 2 * i + 1]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::LineLoop | gltf::mesh::Mode::LineStrip => {
                            //Indices represent a path, in the case of a loop, it comes back around
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            for i in 0..(ai_mesh.vertices.len() - 1) {
                                vec.push(vec![i, i + 1]);
                            }
                            if primitive.mode() == gltf::mesh::Mode::LineLoop {
                                vec.push(vec![ai_mesh.vertices.len() - 1, 0]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::Triangles => {
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            for i in 0..(ai_mesh.vertices.len() / 3) {
                                vec.push(vec![i, i + 1, i + 2]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::TriangleStrip => {
                            let mut vec: Vec<Vec<usize>> = Vec::new(); //Indices are strips of triangles
                            for i in 0..(ai_mesh.vertices.len() - 2) {
                                if (i + 1) % 2 == 0 {
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    vec.push(vec![i + 1, i, i + 2]);
                                } else {
                                    // For odd n, vertices n, n+1, and n+2 define triangle n
                                    vec.push(vec![i, i + 1, i + 2]);
                                }
                            }
                            vec
                        }
                        gltf::mesh::Mode::TriangleFan => {
                            let mut vec: Vec<Vec<usize>> = Vec::new();
                            vec.push(vec![index_buffer[0], index_buffer[1], index_buffer[2]]);
                            for i in 1..(index_buffer.len() - 2) {
                                // For even n, vertices n + 1, n, and n + 2 define triangle n
                                vec.push(vec![
                                    index_buffer[0],
                                    index_buffer[i + 1],
                                    index_buffer[i + 2],
                                ]);
                            }
                            vec
                        }
                    }
                };

                //Handle Material
                ai_mesh.material_index =
                    primitive.material().index().unwrap_or(last_material_index) as u32;

                meshes.push(ai_mesh);
            }
        }
        Ok((meshes, mesh_offsets, vertex_remapping_tables))
    }
}
