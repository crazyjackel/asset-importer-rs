use gltf::{buffer, Document, Mesh, Semantic};

use crate::{core::error::AiReadError, structs::{base_types::AiReal, AiAnimMesh, AiColor4D, AiMesh, AiPrimitiveType, AiVector3D, AI_MAX_NUMBER_OF_COLORS_SETS, AI_MAX_NUMBER_OF_TEXTURECOORDS}};

use super::{gltf2_error::Gtlf2Error, gltf2_importer::Gltf2Importer};

pub(crate) trait GetPointer {
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

impl Gltf2Importer{
    pub(crate)  fn import_meshes<'a>(
        document: &'a Document,
        buffer_data: &'a [buffer::Data],
        last_material_index: usize
    ) -> Result<(Vec<AiMesh>, Vec<u32>, Vec<Vec<u32>>), AiReadError> {
        let asset_meshes: Vec<Mesh<'_>> = document.meshes().collect();

        //Maps Document Mesh Index to Offset. Lets us add all primitives to a Node as Meshes
        //GLTF2 only allows one mesh per node and uses primitives for multiple groups, whilst Assimp has many meshes per Node
        let mut mesh_offsets: Vec<u32> = Vec::new();
        mesh_offsets.reserve(asset_meshes.len() + 1);
        let mut cumulative_meshes = 0;
        for mesh in asset_meshes.iter() {
            mesh_offsets.push(cumulative_meshes);
            cumulative_meshes += mesh.primitives().len() as u32;
        }
        mesh_offsets.push(cumulative_meshes); // add a last element so we can always do mesh_offsets[n+1] - mesh_offsets[n]

        let mut meshes: Vec<AiMesh> = Vec::new(); //Final Meshes to return
        let mut vertex_remapping_tables: Vec<Vec<u32>> = Vec::new(); //For Each Mesh, how do we remap their indices. Is needed when building Nodes

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
                let mut vertex_remapping_table: Option<&Vec<u32>> = None;
                if let Some(indices) = primitive.indices() {
                    use_index_buffer = true; //used to remember if we did this or not
                    let count = indices.count();

                    //recycle data structures
                    index_buffer.resize(count, 0);
                    reverse_mapping_indices.clear();
                    
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
                            index_buffer[i] = index_usize;
                            continue;
                        }
                        if index_usize >= reverse_mapping_indices.len() {
                            reverse_mapping_indices.resize(index_usize + 1, u32::MAX);
                        }
                        if reverse_mapping_indices[index_usize] == u32::MAX {
                            reverse_mapping_indices[index_usize] = vertex_remap_table.len() as u32;
                            vertex_remap_table.push(index);
                        }
                        index_buffer[i] = reverse_mapping_indices[index_usize] as usize;
                    }
                    vertex_remapping_table = Some(vertex_remap_table);
                }

                //Construct Mesh
                let mut ai_mesh = AiMesh::default();
                
                //Set Name
                ai_mesh.name = mesh
                    .name()
                    .map(|x| x.to_string())
                    .unwrap_or(mesh.index().to_string());
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
                    //Position Accessor must be VEC3 of type float (f32), thus they must chunk by 3 * 4 = 12
                    let data = attr_position
                        .get_pointer(&buffer_data)
                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                    debug_assert!(
                        data.len()  == attr_position.count() * 12, 
                        "Position Accessor must be VEC3 of type float (f32), the resultant pointer should therefore always be 12x as big");

                    ai_mesh.vertices = remap_data(vertex_remapping_table, data, 12, |chunk|{                 
                        let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as AiReal;
                        let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) as AiReal;
                        let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) as AiReal;
                        AiVector3D::new(x, y, z)
                    });
                }
                
                //Handle Normals, Tangents, and Bitangents
                let mut tangent_weights: Vec<f32> = Vec::new();
                if let Some((_,attr_normals)) = primitive.attributes().find(|x: &(Semantic, gltf::Accessor<'_>)| x.0 == Semantic::Normals){
                    if attr_normals.count() != num_all_vertices{
                        println!("Normal count in mesh \"{}\" does not match the vertex count, normals ignored.", ai_mesh.name);
                    }
                    else{
                        let data = attr_normals
                            .get_pointer(&buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                        debug_assert!(
                        data.len()  == attr_normals.count() * 12, 
                        "Normal Accessor must be VEC3 of type float (f32), the resultant pointer should therefore always be 12x as big");

                        //Get Vertices
                        ai_mesh.normals = remap_data(vertex_remapping_table, data, 12, |chunk|{              
                            let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as AiReal;
                            let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) as AiReal;
                            let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) as AiReal;
                            AiVector3D::new(x, y, z)
                        });


                        // only extract tangents if normals are present
                        if let Some((_,attr_tangents)) = primitive.attributes().find(|x: &(Semantic, gltf::Accessor<'_>)| x.0 == Semantic::Tangents){
                            if attr_tangents.count() != num_all_vertices{
                                println!("Tangent count in mesh \"{}\" does not match the vertex count, tangents ignored.", ai_mesh.name);
                            }else{
                                let data = attr_tangents
                                    .get_pointer(&buffer_data)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                                debug_assert!(
                                    data.len() == attr_normals.count() * 16, 
                                    "Normal Accessor must be VEC4 of type float (f32), the resultant pointer should therefore always be 16x as big");

                                let tangents = remap_data(vertex_remapping_table, data, 16, |chunk|{              
                                    let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as AiReal;
                                    let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) as AiReal;
                                    let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) as AiReal;
                                    let w = f32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]) as AiReal;
                                    (x,y,z,w)
                                });

                                ai_mesh.tangents.resize(tangents.len(), AiVector3D::default());
                                ai_mesh.bi_tangents.resize(tangents.len(), AiVector3D::default());
                                for i in 0..tangents.len(){
                                    let (x,y,z,w) = tangents[i];
                                    ai_mesh.tangents[i] = AiVector3D::new(x, y, z);
                                    ai_mesh.bi_tangents[i] = (ai_mesh.normals[i] ^ AiVector3D::new(x, y, z)) * w;
                                    tangent_weights.push(w);
                                }
                                
                            }
                        }
                    
                    }
                }

                //Handle Colors
                let colors : Vec<(gltf::Accessor<'_>, u32)> = primitive.attributes().filter_map(|x| match x.0 {
                    Semantic::Colors(n) if n < AI_MAX_NUMBER_OF_COLORS_SETS as u32 => Some((x.1, n)),
                    _ => None
                }).collect();
                for (attr_color, index) in colors{
                    let data = attr_color
                        .get_pointer(&buffer_data)
                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                    ai_mesh.colors[index as usize] = match attr_color.dimensions(){
                        gltf::accessor::Dimensions::Vec3 => match attr_color.data_type(){
                            gltf::accessor::DataType::U8 => Some(remap_data(vertex_remapping_table, data, 3, |chunk|{
                                AiColor4D::new(chunk[0] as f32 / 255.0, chunk[1] as f32 / 255.0, chunk[2] as f32 / 255.0, 1.0)
                            })),
                            gltf::accessor::DataType::U16 => Some(remap_data(vertex_remapping_table, data, 6, |chunk|{
                                let r = u16::from_le_bytes([chunk[0], chunk[1]]);
                                let g = u16::from_le_bytes([chunk[2], chunk[3]]);
                                let b = u16::from_le_bytes([chunk[4], chunk[5]]);
                                AiColor4D::new(r as f32 / 65535.0, g as f32 / 65535.0, b as f32 / 65535.0, 1.0)
                            })),
                            gltf::accessor::DataType::F32 => Some(remap_data(vertex_remapping_table, data, 12, |chunk|{
                                let r = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                                let g = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                                let b = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]);
                                AiColor4D::new(r,g,b, 1.0)
                            })),
                            _ => None
                        },
                        gltf::accessor::Dimensions::Vec4 => match attr_color.data_type(){
                            gltf::accessor::DataType::U8 => Some(remap_data(vertex_remapping_table, data, 4, |chunk|{
                                AiColor4D::new(chunk[0] as f32 / 255.0, chunk[1] as f32 / 255.0, chunk[2] as f32 / 255.0, chunk[3] as f32 / 255.0)
                            })),
                            gltf::accessor::DataType::U16 => Some(remap_data(vertex_remapping_table, data, 8, |chunk|{
                                let r = u16::from_le_bytes([chunk[0], chunk[1]]);
                                let g = u16::from_le_bytes([chunk[2], chunk[3]]);
                                let b = u16::from_le_bytes([chunk[4], chunk[5]]);
                                let a = u16::from_le_bytes([chunk[6], chunk[7]]);
                                AiColor4D::new(r as f32 / 65535.0, g as f32 / 65535.0, b as f32 / 65535.0, a as f32 / 65535.0)
                            })),
                            gltf::accessor::DataType::F32 => Some(remap_data(vertex_remapping_table, data, 16, |chunk|{
                                let r = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                                let g = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                                let b = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]);
                                let a = f32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]);
                                AiColor4D::new(r,g,b, a)
                            })),
                            _ => None
                        },
                        _ => None
                    }
                }
                
                //Handle Textures
                let texcoords : Vec<(gltf::Accessor<'_>, u32)> = primitive.attributes().filter_map(|x| match x.0 {
                    Semantic::TexCoords(n) if n < AI_MAX_NUMBER_OF_TEXTURECOORDS as u32 => Some((x.1, n)),
                    _ => None
                }).collect();
                for (attr_texcoords, index) in texcoords{
                    let data = attr_texcoords
                        .get_pointer(&buffer_data)
                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                    ai_mesh.texture_coords[index as usize] = match attr_texcoords.data_type(){
                        gltf::accessor::DataType::U8 => Some(remap_data(vertex_remapping_table, data, 2, |chunk|{
                            let u = chunk[0] as f32 / 255.0;
                            let v = 1.0 - (chunk[1] as f32 / 255.0);
                            AiVector3D::new(u, v, 0.0)
                        })),
                        gltf::accessor::DataType::U16 => Some(remap_data(vertex_remapping_table, data, 4, |chunk|{
                            let u = u16::from_le_bytes([chunk[0], chunk[1]])as f32 / 65535.0;
                            let v = 1.0 - (u16::from_le_bytes([chunk[2], chunk[3]])as f32 / 65535.0);
                            AiVector3D::new(u, v, 0.0)
                        })),
                        gltf::accessor::DataType::F32 => Some(remap_data(vertex_remapping_table, data, 8, |chunk|{
                            let u = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                            let v = 1.0 - f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                            AiVector3D::new(u, v, 0.0)
                        })),
                        _ => None
                    };
                }

                //Handle AnimMeshes
                //ai_mesh.anim_meshes should be an empty vec here
                if let Some(weights) = mesh.weights(){
                    let targets : Vec<gltf::mesh::MorphTarget<'_>> = primitive.morph_targets().collect();
                    if targets.len() == weights.len(){
                        for i in 0..targets.len(){
                            let weight =  weights[i];
                            let target = &targets[i];
                            let mut anim_mesh = AiAnimMesh::default();

                            //handle name
                            //@todo: names are defined via an extra value in mesh.extra.targetNames, it is safe to go without a name for now.

                            //handle weight
                            anim_mesh.weight = weight;

                            //handle position
                            if let Some(positions) = target.positions(){
                                if positions.count() == num_all_vertices{
                                    let data = positions
                                        .get_pointer(&buffer_data)
                                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                                    let offsets = remap_data(vertex_remapping_table, data, 12, |chunk|{                 
                                        let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as AiReal;
                                        let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) as AiReal;
                                        let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) as AiReal;
                                        AiVector3D::new(x, y, z)
                                    });

                                    if offsets.len() == ai_mesh.vertices.len(){
                                        anim_mesh.vertices = ai_mesh.vertices.clone();
                                        for i in 0..offsets.len(){
                                            anim_mesh.vertices[i] += offsets[i];
                                        }
                                    }
                                }
                            }

                            //handle normals
                            if let Some(normals) = target.normals(){
                                if normals.count() == num_all_vertices{
                                    let data = normals
                                    .get_pointer(&buffer_data)
                                    .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                                    let offsets = remap_data(vertex_remapping_table, data, 12, |chunk|{              
                                        let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as AiReal;
                                        let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) as AiReal;
                                        let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) as AiReal;
                                        AiVector3D::new(x, y, z)
                                    });

                                    if offsets.len() == ai_mesh.normals.len(){
                                        anim_mesh.normals = ai_mesh.normals.clone();
                                        for i in 0..offsets.len(){
                                            anim_mesh.normals[i] += offsets[i];
                                        }
                                    }
                                }
                            }
                            //handle tangents
                            if let Some(tangents) = target.tangents(){
                                if tangents.count() == num_all_vertices && !anim_mesh.normals.is_empty(){
                                    let offset_data = tangents
                                        .get_pointer(&buffer_data)
                                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;

                                    let tangents_offsets= remap_data(vertex_remapping_table, offset_data, 12, |chunk|{              
                                        let x: AiReal = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as AiReal;
                                        let y: AiReal = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) as AiReal;
                                        let z: AiReal = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) as AiReal;
                                        (x,y,z)
                                    });

                                    if tangents_offsets.len() == ai_mesh.tangents.len(){
                                        anim_mesh.tangents = ai_mesh.tangents.clone();
                                        anim_mesh.bi_tangents = Vec::new();
                                        anim_mesh.bi_tangents.resize(ai_mesh.tangents.len(), Default::default());
                                        for i in 0..tangents_offsets.len(){
                                            let offset = tangents_offsets[i];
                                            anim_mesh.tangents[i] += AiVector3D::new(offset.0, offset.1, offset.2);
                                            anim_mesh.bi_tangents[i] = (anim_mesh.normals[i] ^ anim_mesh.tangents[i]) * tangent_weights[i]; //Saved Tangent Weights to prevent need for re-extraction
                                        }
                                    }
                                }
                            }
                            //we do not handle colors nor texture coords as gltf 2.0 does not provide that information, it should be assumed that info doesn't changed
                        }
                    }
                }

                //Handle Faces
                ai_mesh.faces = if use_index_buffer{
                    match primitive.mode(){
                        gltf::mesh::Mode::Points => {
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            for i in 0..(index_buffer.len()){
                                let a = index_buffer[i];
                                if a >= num_all_vertices{
                                    continue;
                                }
                                vec.push(vec![index_buffer[i]]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::Lines => {
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            for i in 0..(index_buffer.len() / 2) {
                                let a = index_buffer[2*i];
                                let b = index_buffer[2*i + 1];
                                if a >= num_all_vertices || b >= num_all_vertices{
                                    continue;
                                }
                                vec.push(vec![a,b]);
                            }
                            vec
                        },
                        gltf::mesh::Mode::LineLoop | gltf::mesh::Mode::LineStrip => {
                            //Indices represent a path, in the case of a loop, it comes back around
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            for i in 0..(index_buffer.len()-1){
                                let a = index_buffer[i];
                                let b = index_buffer[i + 1];
                                if a >= num_all_vertices || b >= num_all_vertices{
                                    continue;
                                }
                                vec.push(vec![a,b]);
                            }
                            if primitive.mode() == gltf::mesh::Mode::LineLoop{
                                let a = index_buffer[index_buffer.len() - 1];
                                let b = index_buffer[0];
                                if a < num_all_vertices && b < num_all_vertices{
                                    vec.push(vec![a,b]);    
                                }
                            }
                            vec
                        },
                        gltf::mesh::Mode::Triangles =>{
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            for i in 0..(index_buffer.len() / 3) {
                                let a = index_buffer[3*i];
                                let b = index_buffer[3*i + 1];
                                let c = index_buffer[3*i + 2];
                                if a >= num_all_vertices || b >= num_all_vertices || c >= num_all_vertices{
                                    continue;
                                }
                                vec.push(vec![a,b,c ]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::TriangleStrip => {
                            let mut vec : Vec<Vec<usize>> = Vec::new(); //Indices are strips of triangles
                            for i in 0..(index_buffer.len()-2){
                                if (i + 1) % 2 == 0 {
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    let a = index_buffer[i];
                                    let b = index_buffer[i + 1];
                                    let c = index_buffer[i + 2];
                                    if a >= num_all_vertices || b >= num_all_vertices || c >= num_all_vertices{
                                        continue;
                                    }
                                    vec.push(vec![b,a,c]);
                                } else {
                                    // For odd n, vertices n, n+1, and n+2 define triangle n
                                    let a = index_buffer[i];
                                    let b = index_buffer[i + 1];
                                    let c = index_buffer[i + 2];
                                    if a >= num_all_vertices || b >= num_all_vertices || c >= num_all_vertices{
                                        continue;
                                    }
                                    vec.push(vec![a,b,c]);
                                }
                            }
                            vec
                        },
                        gltf::mesh::Mode::TriangleFan =>{
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            let a = index_buffer[0];
                            let b = index_buffer[1];
                            let c = index_buffer[2];
                            if a < num_all_vertices && b < num_all_vertices && c < num_all_vertices{
                                vec.push(vec![a,b,c]);
                                for i in 1..(index_buffer.len()-2){
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    let d = index_buffer[i + 1];
                                    let e = index_buffer[i + 2];
                                    if d >= num_all_vertices || e >= num_all_vertices{
                                        continue;
                                    }
                                    vec.push(vec![a , d, e ]);
                                }
                            }
                            vec
                        },
                    }
                }else{
                    match primitive.mode(){
                        gltf::mesh::Mode::Points => {
                            (0..ai_mesh.vertices.len()).map(|x| vec![x]).collect() //Indices represent points
                        }
                        gltf::mesh::Mode::Lines => {
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            for i in 0..(ai_mesh.vertices.len() / 2) {
                                vec.push(vec![2*i,2*i+1]);
                            }
                            vec
                        },
                        gltf::mesh::Mode::LineLoop | gltf::mesh::Mode::LineStrip => {
                            //Indices represent a path, in the case of a loop, it comes back around
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            for i in 0..(ai_mesh.vertices.len()-1){
                                vec.push(vec![i,i+1]);
                            }
                            if primitive.mode() == gltf::mesh::Mode::LineLoop{
                                vec.push(vec![ai_mesh.vertices.len()-1, 0]);
                            }
                            vec
                        },
                        gltf::mesh::Mode::Triangles =>{
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            for i in 0..(ai_mesh.vertices.len() / 3) {
                                vec.push(vec![i,i+1,i+2]);
                            }
                            vec
                        }
                        gltf::mesh::Mode::TriangleStrip => {
                            let mut vec : Vec<Vec<usize>> = Vec::new(); //Indices are strips of triangles
                            for i in 0..(ai_mesh.vertices.len()-2){
                                if (i + 1) % 2 == 0 {
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    vec.push(vec![i+1, i, i+2]);
                                } else {
                                    // For odd n, vertices n, n+1, and n+2 define triangle n
                                    vec.push(vec![i, i+1, i+2]);
                                }
                            }
                            vec
                        },
                        gltf::mesh::Mode::TriangleFan =>{
                            let mut vec : Vec<Vec<usize>> = Vec::new();
                            vec.push(vec![index_buffer[0], index_buffer[1], index_buffer[2]]);
                            for i in 1..(index_buffer.len()-2){
                                // For even n, vertices n + 1, n, and n + 2 define triangle n
                                vec.push(vec![index_buffer[0] ,index_buffer[i + 1] , index_buffer[i+2] ]);
                            }
                            vec
                        },
                    }
                };

                //Handle Material
                ai_mesh.material_index = primitive.material().index().unwrap_or(last_material_index) as u32;
            
                meshes.push(ai_mesh);
            }
        }
        Ok((meshes, mesh_offsets, vertex_remapping_tables))
    }
}


pub(crate) fn remap_data<B,F>(vertex_remapping_table: Option<&Vec<u32>>, data: Vec<u8>, chunk_size: usize, f: F) 
-> Vec<B> where F: FnMut(&[u8]) -> B, B:Clone + Default
    {
    let vertices= if let Some(remap) = vertex_remapping_table {
        //If we have Remap, prepare the vertices and then chunk them in
        let mut vertices: Vec<B> = Vec::new();
        vertices.resize(data.len() / chunk_size, B::default());
        for (index, chunk) in data
            .chunks_exact(chunk_size)
            .map(f)
            .enumerate()
        {
            vertices[remap[index] as usize] = chunk;
        }
        vertices
    } else {
        //Vertices are already sorted, no remap neccessary. Only happens when we have no Indices
        data.chunks_exact(chunk_size)
            .map(f)
            .collect()
    };
    vertices
}
