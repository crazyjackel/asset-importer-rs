use std::{
    collections::HashMap,
    io::{self, Read, Seek},
    path::Path,
};

use gltf::{Document, buffer};

use asset_importer_rs_scene::{AiTexel, AiTexture, AiTextureFormat};

use image::ImageFormat;

use crate::importer::error::Gltf2ImportError;

use super::importer::Gltf2Importer;

impl Gltf2Importer {
    pub fn from_source<R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        source: gltf::image::Source<'_>,
        base: Option<&Path>,
        loader: &F,
        buffer_data: &[buffer::Data],
    ) -> Result<gltf::image::Data, gltf::Error> {
        #[cfg(feature = "guess_mime_type")]
        let guess_format = |encoded_image: &[u8]| match image::guess_format(encoded_image) {
            Ok(ImageFormat::Png) => Some(ImageFormat::Png),
            Ok(ImageFormat::Jpeg) => Some(ImageFormat::Jpeg),
            Ok(ImageFormat::WebP) => Some(ImageFormat::WebP),
            _ => None,
        };
        #[cfg(not(feature = "guess_mime_type"))]
        let guess_format = |_encoded_image: &[u8]| None;

        let decoded_image = match source {
            gltf::image::Source::Uri { uri, mime_type } => {
                if uri.contains(':') {
                    if let Some(rest) = uri.strip_prefix("data:") {
                        // Inline base64-encoded data
                        let mut it = rest.split(";base64,");
                        let (annoying_case, base64) = match (it.next(), it.next()) {
                            (match0_opt, Some(match1)) => Ok((match0_opt, match1)),
                            (Some(match0), _) => Ok((None, match0)),
                            _ => Err(gltf::Error::UnsupportedScheme),
                        }?;
                        let encoded_image = base64::decode(base64).map_err(gltf::Error::Base64)?;

                        // Determine image format either by MIME type or guessing
                        let encoded_format = match annoying_case {
                            Some("image/png") => image::ImageFormat::Png,
                            Some("image/jpeg") => image::ImageFormat::Jpeg,
                            _ => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(gltf::Error::UnsupportedImageEncoding),
                            },
                        };
                        image::load_from_memory_with_format(&encoded_image, encoded_format)?
                    } else if let Some(rest) =
                        uri.strip_prefix("file://").or(uri.strip_prefix("file:"))
                    {
                        if base.is_none() {
                            return Err(gltf::Error::ExternalReferenceInSliceImport);
                        }

                        // Read the encoded image data from the file
                        let mut encoded_image = Vec::new();
                        loader(rest.as_ref())?.read_to_end(&mut encoded_image)?;

                        // Guess or determine the format based on MIME type or file extension
                        let encoded_format = match mime_type {
                            Some("image/png") => image::ImageFormat::Png,
                            Some("image/jpeg") => image::ImageFormat::Jpeg,
                            Some(_) => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(gltf::Error::UnsupportedImageEncoding),
                            },
                            None => match uri.rsplit('.').next() {
                                Some("png") => image::ImageFormat::Png,
                                Some("jpg") | Some("jpeg") => image::ImageFormat::Jpeg,
                                _ => match guess_format(&encoded_image) {
                                    Some(format) => format,
                                    None => return Err(gltf::Error::UnsupportedImageEncoding),
                                },
                            },
                        };
                        image::load_from_memory_with_format(&encoded_image, encoded_format)?
                    } else {
                        // Unsupported URI scheme
                        return Err(gltf::Error::UnsupportedScheme);
                    }
                } else {
                    // Handle relative URI references
                    let encoding = urlencoding::decode(uri).unwrap();
                    if base.is_none() {
                        return Err(gltf::Error::ExternalReferenceInSliceImport);
                    }
                    // Load image from relative path
                    let mut encoded_image = Vec::new();
                    loader(base.unwrap().join(&*encoding).as_ref())?
                        .read_to_end(&mut encoded_image)?;

                    // Identify format by MIME type or extension
                    let encoded_format = match mime_type {
                        Some("image/png") => image::ImageFormat::Png,
                        Some("image/jpeg") => image::ImageFormat::Jpeg,
                        Some(_) => match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(gltf::Error::UnsupportedImageEncoding),
                        },
                        None => match uri.rsplit('.').next() {
                            Some("png") => image::ImageFormat::Png,
                            Some("jpg") | Some("jpeg") => image::ImageFormat::Jpeg,
                            _ => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(gltf::Error::UnsupportedImageEncoding),
                            },
                        },
                    };
                    image::load_from_memory_with_format(&encoded_image, encoded_format)?
                }
            }
            gltf::image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let encoded_image = &parent_buffer_data[begin..end];

                // Identify format based on MIME type or guessing
                let encoded_format = match mime_type {
                    "image/png" => image::ImageFormat::Png,
                    "image/jpeg" => image::ImageFormat::Jpeg,
                    _ => match guess_format(encoded_image) {
                        Some(format) => format,
                        None => return Err(gltf::Error::UnsupportedImageEncoding),
                    },
                };
                image::load_from_memory_with_format(encoded_image, encoded_format)?
            }
        };

        use image::GenericImageView;
        let format = match decoded_image {
            image::DynamicImage::ImageLuma8(_) => gltf::image::Format::R8,
            image::DynamicImage::ImageLumaA8(_) => gltf::image::Format::R8G8,
            image::DynamicImage::ImageRgb8(_) => gltf::image::Format::R8G8B8,
            image::DynamicImage::ImageRgba8(_) => gltf::image::Format::R8G8B8A8,
            image::DynamicImage::ImageLuma16(_) => gltf::image::Format::R16,
            image::DynamicImage::ImageLumaA16(_) => gltf::image::Format::R16G16,
            image::DynamicImage::ImageRgb16(_) => gltf::image::Format::R16G16B16,
            image::DynamicImage::ImageRgba16(_) => gltf::image::Format::R16G16B16A16,
            image::DynamicImage::ImageRgb32F(_) => gltf::image::Format::R32G32B32FLOAT,
            image::DynamicImage::ImageRgba32F(_) => gltf::image::Format::R32G32B32A32FLOAT,
            image => return Err(gltf::Error::UnsupportedImageFormat(image)),
        };
        let (width, height) = decoded_image.dimensions();
        let pixels = decoded_image.into_bytes();
        Ok(gltf::image::Data {
            format,
            width,
            height,
            pixels,
        })
    }

    pub(crate) fn import_embedded_textures<R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        document: &Document,
        base: Option<&Path>,
        loader: &F,
        buffer_data: &[buffer::Data],
    ) -> Result<(Vec<AiTexture>, HashMap<usize, usize>), Gltf2ImportError> {
        let mut textures: Vec<AiTexture> = Vec::new();
        let mut embedded_tex_ids: HashMap<usize, usize> = HashMap::new();
        for image in document.images() {
            let source = image.source();
            let (filename, mime_type) = match source {
                gltf::image::Source::View { view: _, mime_type } => {
                    (image.index().to_string(), Some(mime_type))
                }
                gltf::image::Source::Uri { uri, mime_type } => (
                    if let Some(pos) = uri.find('.') {
                        &uri[..pos]
                    } else {
                        uri
                    }
                    .to_string(),
                    mime_type,
                ),
            };

            //@todo: add better mime_type predictions
            //format is used for exporting from texels
            let format: AiTextureFormat = match mime_type {
                Some(mime_type_str) => match mime_type_str {
                    "image/jpeg" => AiTextureFormat::JPEG,
                    "image/png" => AiTextureFormat::PNG,
                    "image/webp" => AiTextureFormat::WEBP,
                    _ => AiTextureFormat::Unknown,
                },
                None => AiTextureFormat::Unknown,
            };

            let data: gltf::image::Data =
                Gltf2Importer::from_source(image.source(), base, &loader, buffer_data)
                    .map_err(Gltf2ImportError::FileFormatError)?;

            let texels: Vec<AiTexel> = match data.format {
                gltf::image::Format::R8 => data
                    .pixels
                    .into_iter()
                    .map(|r| AiTexel::new(r, r, r, 255))
                    .collect(),
                gltf::image::Format::R8G8 => data
                    .pixels
                    .chunks_exact(2)
                    .map(|x| AiTexel::new(x[0], x[1], 0, 255))
                    .collect(),
                gltf::image::Format::R8G8B8 => data
                    .pixels
                    .chunks_exact(3)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], 255))
                    .collect(),
                gltf::image::Format::R8G8B8A8 => data
                    .pixels
                    .chunks_exact(4)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[1], chunk[2], chunk[3]))
                    .collect(),
                gltf::image::Format::R16 => {
                    data.pixels
                        .chunks_exact(2)
                        .map(|chunk| {
                            let r = chunk[0]; // Take the most significant byte
                            AiTexel::new(r, r, r, 255)
                        })
                        .collect()
                }
                gltf::image::Format::R16G16 => data
                    .pixels
                    .chunks_exact(4)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], 0, 255))
                    .collect(),
                gltf::image::Format::R16G16B16 => data
                    .pixels
                    .chunks_exact(6)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], 255))
                    .collect(),
                gltf::image::Format::R16G16B16A16 => data
                    .pixels
                    .chunks_exact(8)
                    .map(|chunk| AiTexel::new(chunk[0], chunk[2], chunk[4], chunk[6]))
                    .collect(),
                gltf::image::Format::R32G32B32FLOAT => data
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
                gltf::image::Format::R32G32B32A32FLOAT => data
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

#[cfg(test)]
mod tests {
    use super::*;

    use asset_importer_rs_core::default_file_loader;
    #[test]
    fn test_gltf2_texture_import() {
        let binding = std::env::current_dir().expect("Failed to get the current executable path");
        let exe_path = binding.as_path();

        let gltf_data = r#"{
            "asset": {
                "version": "2.0"
            },
            "images": [
                {
                    "uri": "tests/textures/Unicode❤♻Texture.png"
                }
            ],
            "textures": [
                {
                    "source": 0
                }
            ]
        }"#;
        let scene = serde_json::from_str(gltf_data).unwrap();
        let document = Document::from_json_without_validation(scene);
        let (embedded_textures, _tex_ids) = Gltf2Importer::import_embedded_textures(
            &document,
            Some(exe_path),
            &default_file_loader,
            &[],
        )
        .unwrap();
        assert_eq!(1, embedded_textures.len());
    }
}
