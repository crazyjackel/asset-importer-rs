use std::collections::HashMap;

use bytemuck::Pod;
use gltf_v1::{
    buffer::Data,
    json::{
        accessor::{ComponentType, Type},
        map::IndexMap,
        mesh::{PrimitiveMode, Semantic},
        validation::Checked,
    },
    Document,
};

use crate::{
    core::error::AiReadError,
    structs::{
        base_types::AiReal, AiColor4D, AiMesh, AiVector3D, AI_MAX_NUMBER_OF_COLORS_SETS,
        AI_MAX_NUMBER_OF_TEXTURECOORDS,
    },
};

use super::{error::Error, gltf_importer::GltfImporter};

use crate::structs::AiPrimitiveType;

pub(crate) trait ExtractData {
    fn extract_data<T>(&self, buffers: &IndexMap<String, Data>) -> Result<Vec<T>, Error>
    where
        T: Sized + Default + Pod;
}

impl ExtractData for gltf_v1::Accessor<'_> {
    fn extract_data<T>(&self, buffers: &IndexMap<String, Data>) -> Result<Vec<T>, Error>
    where
        T: Sized + Default + Pod,
    {
        let view = self.view();
        let data = buffers
            .get(view.buffer().index())
            .ok_or(Error::MissingBufferData)?;

        let num_components = self.accessor_type().get_num_components();
        let bytes_per_component = self.component_type().size();
        let count = self.count();

        let elem_size: usize = (num_components * bytes_per_component) as usize;
        let stride = self.stride().unwrap_or(elem_size);

        let target_elem_size = size_of::<T>();

        let start_index = self.offset() + view.offset();
        let end_index = start_index + (count - 1) * stride + elem_size; //The Last Element
        if end_index > data.len() {
            return Err(Error::ExceedsBounds);
        }
        let data_slice = &data[start_index..end_index];

        let mut result = Vec::with_capacity(count);
        if stride == elem_size && target_elem_size == elem_size {
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
        Ok(result)
    }
}

pub struct IndexSpan(pub u32, pub u32);

pub struct ImportMeshes(pub Vec<AiMesh>, pub HashMap<String, IndexSpan>);

impl GltfImporter {
    pub(crate) fn import_meshes(
        document: &Document,
        buffer_data: &IndexMap<String, Data>,
        material_index_map: &HashMap<String, usize>,
    ) -> Result<ImportMeshes, AiReadError> {
        let mut meshes: Vec<AiMesh> = Vec::new();
        let mut mesh_offsets: HashMap<String, IndexSpan> = HashMap::new();

        let mut k: u32 = 0;
        for mesh in document.meshes() {
            let span = mesh.primitives().len() as u32;
            mesh_offsets.insert(mesh.index().to_string(), IndexSpan(k, span));
            k += span;
            for (index, primitive) in mesh.primitives().enumerate() {
                let mut ai_mesh = AiMesh {
                    name: mesh
                        .name()
                        .map(|x| x.to_string())
                        .unwrap_or(mesh.index().to_string()),
                    ..AiMesh::default()
                };
                if mesh.primitives().len() > 1 {
                    ai_mesh.name = format!("{}-{}", ai_mesh.name, index);
                }
                match primitive.mode() {
                    PrimitiveMode::Points => ai_mesh.primitive_types |= AiPrimitiveType::Point,
                    PrimitiveMode::Lines | PrimitiveMode::LineLoop | PrimitiveMode::LineStrip => {
                        ai_mesh.primitive_types |= AiPrimitiveType::Line
                    }
                    PrimitiveMode::Triangles
                    | PrimitiveMode::TriangleStrip
                    | PrimitiveMode::TriangleFan => {
                        ai_mesh.primitive_types |= AiPrimitiveType::Triangle
                    }
                }

                let mut num_all_vertices: usize = 0;
                //handle positions
                if let Some(positions) = primitive.get(&gltf_v1::json::mesh::Semantic::Positions) {
                    num_all_vertices = positions.count();
                    let data: Vec<[f32; 3]> = positions
                        .extract_data(buffer_data)
                        .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                    ai_mesh.vertices = data
                        .iter()
                        .map(|x| AiVector3D::new(x[0] as AiReal, x[1] as AiReal, x[2] as AiReal))
                        .collect();
                }

                //handle normals
                if let Some(normals) = primitive.get(&gltf_v1::json::mesh::Semantic::Normals) {
                    if normals.count() != num_all_vertices {
                        println!("Normal count in mesh \"{}\" does not match the vertex count, normals ignored.", ai_mesh.name);
                    } else {
                        let data: Vec<[f32; 3]> = normals
                            .extract_data(buffer_data)
                            .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                        ai_mesh.normals = data
                            .iter()
                            .map(|x| {
                                AiVector3D::new(x[0] as AiReal, x[1] as AiReal, x[2] as AiReal)
                            })
                            .collect();
                    }
                }

                //Handle Colors
                let colors: Vec<(gltf_v1::Accessor<'_>, u32)> = primitive
                    .attributes()
                    .filter_map(|x| match x.0 {
                        Checked::Valid(Semantic::Colors(n))
                            if n < AI_MAX_NUMBER_OF_COLORS_SETS as u32 =>
                        {
                            Some((x.1, n))
                        }
                        _ => None,
                    })
                    .collect();
                for (attr_color, index) in colors {
                    ai_mesh.colors[index as usize] = match attr_color.accessor_type() {
                        Type::VEC3 => match attr_color.component_type() {
                            ComponentType::UnsignedByte => {
                                let data: Vec<[u8; 3]> = attr_color
                                    .extract_data(buffer_data)
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
                            ComponentType::UnsignedShort => {
                                let data: Vec<[u16; 3]> = attr_color
                                    .extract_data(buffer_data)
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
                            ComponentType::Float => {
                                let data: Vec<[f32; 3]> = attr_color
                                    .extract_data(buffer_data)
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
                        Type::VEC4 => match attr_color.component_type() {
                            ComponentType::UnsignedByte => {
                                let data: Vec<[u8; 4]> = attr_color
                                    .extract_data(buffer_data)
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
                            ComponentType::UnsignedShort => {
                                let data: Vec<[u16; 4]> = attr_color
                                    .extract_data(buffer_data)
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
                            ComponentType::Float => {
                                let data: Vec<[f32; 4]> = attr_color
                                    .extract_data(buffer_data)
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

                //handle texcoords
                let texcoords: Vec<(gltf_v1::Accessor<'_>, u32)> = primitive
                    .attributes()
                    .filter_map(|x| match x.0 {
                        Checked::Valid(Semantic::TexCoords(n))
                            if n < AI_MAX_NUMBER_OF_TEXTURECOORDS as u32 =>
                        {
                            Some((x.1, n))
                        }
                        _ => None,
                    })
                    .collect();
                for (attr_texcoords, index) in texcoords {
                    ai_mesh.texture_coords[index as usize] = match attr_texcoords.component_type() {
                        ComponentType::Byte | ComponentType::UnsignedByte => {
                            let data: Vec<[u8; 2]> = attr_texcoords
                                .extract_data(buffer_data)
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
                        ComponentType::Short | ComponentType::UnsignedShort => {
                            let data: Vec<[u16; 2]> = attr_texcoords
                                .extract_data(buffer_data)
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
                        ComponentType::Float => {
                            let data: Vec<[f32; 2]> = attr_texcoords
                                .extract_data(buffer_data)
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

                if let Some(indices) = primitive.indices() {
                    let index_data: Vec<usize> = match indices.component_type() {
                        ComponentType::UnsignedByte | ComponentType::Byte => {
                            let index_data: Vec<u8> = indices
                                .extract_data(buffer_data)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            index_data.into_iter().map(|x| x as usize).collect()
                        }
                        ComponentType::UnsignedShort | ComponentType::Short => {
                            let index_data: Vec<u16> = indices
                                .extract_data(buffer_data)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            index_data.into_iter().map(|x| x as usize).collect()
                        }
                        ComponentType::UnsignedInt | ComponentType::Float => {
                            let index_data: Vec<usize> = indices
                                .extract_data(buffer_data)
                                .map_err(|err| AiReadError::FileFormatError(Box::new(err)))?;
                            index_data
                        }
                    };

                    ai_mesh.faces = match primitive.mode() {
                        PrimitiveMode::Points => {
                            let num_faces = index_data.len();
                            let mut vec: Vec<Vec<usize>> = Vec::with_capacity(num_faces);
                            for a in index_data {
                                if a >= num_all_vertices {
                                    continue;
                                }
                                vec.push(vec![a]);
                            }
                            vec
                        }
                        PrimitiveMode::Lines => {
                            let num_faces = index_data.len() / 2;
                            let mut vec: Vec<Vec<usize>> = Vec::with_capacity(num_faces);
                            for i in 0..num_faces {
                                let a = index_data[2 * i];
                                let b = index_data[2 * i + 1];
                                if a >= num_all_vertices || b >= num_all_vertices {
                                    continue;
                                }
                                vec.push(vec![a, b]);
                            }
                            vec
                        }
                        PrimitiveMode::LineLoop | PrimitiveMode::LineStrip => {
                            //Indices represent a path, in the case of a loop, it comes back around
                            let num_faces = index_data.len() - 1;
                            let is_loop = primitive.mode() == PrimitiveMode::LineLoop;
                            let add = if is_loop { 1 } else { 0 };
                            let mut vec: Vec<Vec<usize>> = Vec::with_capacity(num_faces + add);
                            for i in 0..num_faces {
                                let a = index_data[i];
                                let b = index_data[i + 1];
                                if a >= num_all_vertices || b >= num_all_vertices {
                                    continue;
                                }
                                vec.push(vec![a, b]);
                            }
                            if is_loop {
                                let a = index_data[index_data.len() - 1];
                                let b = index_data[0];
                                if a < num_all_vertices && b < num_all_vertices {
                                    vec.push(vec![a, b]);
                                }
                            }
                            vec
                        }
                        PrimitiveMode::Triangles => {
                            let num_faces = index_data.len() / 3;
                            let mut vec: Vec<Vec<usize>> = Vec::with_capacity(num_faces);
                            for i in 0..num_faces {
                                let a = index_data[3 * i];
                                let b = index_data[3 * i + 1];
                                let c = index_data[3 * i + 2];
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
                        PrimitiveMode::TriangleStrip => {
                            let num_faces = index_data.len() - 2;
                            let mut vec: Vec<Vec<usize>> = Vec::with_capacity(num_faces); //Indices are strips of triangles
                            for i in 0..num_faces {
                                if (i + 1) % 2 == 0 {
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    let a = index_data[i];
                                    let b = index_data[i + 1];
                                    let c = index_data[i + 2];
                                    if a >= num_all_vertices
                                        || b >= num_all_vertices
                                        || c >= num_all_vertices
                                    {
                                        continue;
                                    }
                                    vec.push(vec![b, a, c]);
                                } else {
                                    // For odd n, vertices n, n+1, and n+2 define triangle n
                                    let a = index_data[i];
                                    let b = index_data[i + 1];
                                    let c = index_data[i + 2];
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
                        PrimitiveMode::TriangleFan => {
                            let num_faces = index_data.len() - 2;
                            let mut vec: Vec<Vec<usize>> = Vec::with_capacity(num_faces);
                            let a = index_data[0];
                            let b = index_data[1];
                            let c = index_data[2];
                            if a < num_all_vertices && b < num_all_vertices && c < num_all_vertices
                            {
                                vec.push(vec![a, b, c]);
                                for i in 1..num_faces {
                                    // For even n, vertices n + 1, n, and n + 2 define triangle n
                                    let d = index_data[i + 1];
                                    let e = index_data[i + 2];
                                    if d >= num_all_vertices || e >= num_all_vertices {
                                        continue;
                                    }
                                    vec.push(vec![a, d, e]);
                                }
                            }
                            vec
                        }
                    }
                }
                if let Some(index) = material_index_map.get(primitive.material().index()) {
                    ai_mesh.material_index = *index as u32;
                }
                meshes.push(ai_mesh);
            }
        }
        Ok(ImportMeshes(meshes, mesh_offsets))
    }
}
