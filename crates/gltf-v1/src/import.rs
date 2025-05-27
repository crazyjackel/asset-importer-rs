use indexmap::IndexMap;

use crate::{buffer, document::Document, error::Result, image, GLTF_Error};
use std::{borrow::Cow, fs, io, path::Path};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Scheme<'a> {
    Data(Option<&'a str>, &'a str),
    File(&'a str),
    Relative(Cow<'a, str>),
    Unsupported,
}

impl<'a> Scheme<'a> {
    fn parse(uri: &str) -> Scheme<'_> {
        if uri.contains(':') {
            if let Some(rest) = uri.strip_prefix("data:") {
                let mut it = rest.split(";base64,");

                match (it.next(), it.next()) {
                    (match0_opt, Some(match1)) => Scheme::Data(match0_opt, match1),
                    (Some(match0), _) => Scheme::Data(None, match0),
                    _ => Scheme::Unsupported,
                }
            } else if let Some(rest) = uri.strip_prefix("file://") {
                Scheme::File(rest)
            } else if let Some(rest) = uri.strip_prefix("file:") {
                Scheme::File(rest)
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative(urlencoding::decode(uri).unwrap())
        }
    }

    fn read(base: Option<&Path>, uri: &str) -> Result<Vec<u8>> {
        match Scheme::parse(uri) {
            // The path may be unused in the Scheme::Data case
            // Example: "uri" : "data:application/octet-stream;base64,wsVHPgA...."
            Scheme::Data(_, base64) => base64::decode(base64).map_err(GLTF_Error::Base64),
            Scheme::File(path) if base.is_some() => read_to_end(path),
            Scheme::Relative(path) if base.is_some() => read_to_end(base.unwrap().join(&*path)),
            Scheme::Unsupported => Err(GLTF_Error::UnsupportedScheme),
            _ => Err(GLTF_Error::ExternalReferenceInSliceImport),
        }
    }
}

fn read_to_end<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    use io::Read;
    let file = fs::File::open(path.as_ref()).map_err(GLTF_Error::Io)?;
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file.  Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    let length = file.metadata().map(|x| x.len() + 1).unwrap_or(0);
    let mut reader = io::BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data).map_err(GLTF_Error::Io)?;
    Ok(data)
}

impl buffer::Data {
    pub fn from_source(source: buffer::Source<'_>, base: Option<&Path>) -> Result<Self> {
        Self::from_source_and_blob(source, base, &mut None)
    }

    pub fn from_source_and_blob(
        source: buffer::Source<'_>,
        base: Option<&Path>,
        blob: &mut Option<Vec<u8>>,
    ) -> Result<Self> {
        let mut data = match source {
            buffer::Source::Uri(uri) => Scheme::read(base, uri),
            buffer::Source::Bin => blob.take().ok_or(GLTF_Error::MissingBlob),
        }?;
        while data.len() % 4 != 0 {
            data.push(0);
        }
        Ok(buffer::Data(data))
    }
}

pub fn import_buffers(
    document: &Document,
    base: Option<&Path>,
    mut blob: Option<Vec<u8>>,
) -> Result<IndexMap<String, buffer::Data>> {
    let mut buffers = IndexMap::new();
    for buffer in document.buffers() {
        let index = buffer.index();
        let data = buffer::Data::from_source_and_blob(buffer.source(), base, &mut blob)?;
        if data.len() < buffer.length() {
            return Err(GLTF_Error::BufferLength {
                buffer: buffer.index().to_string(),
                expected: buffer.length(),
                actual: data.len(),
            });
        }
        buffers.insert(index.to_string(), data);
    }
    Ok(buffers)
}

pub fn import_images(
    document: &Document,
    base: Option<&Path>,
    buffer_data: &IndexMap<String, buffer::Data>,
) -> Result<IndexMap<String, image::Data>> {
    let mut images = IndexMap::new();
    for image in document.images() {
        let index = image.index();
        let data = image::Data::from_source(image.source(), base, buffer_data)?;
        images.insert(index.to_string(), data);
    }
    Ok(images)
}

impl image::Data {
    pub fn from_source(
        source: image::Source<'_>,
        base: Option<&Path>,
        buffer_data: &IndexMap<String, buffer::Data>,
    ) -> Result<Self> {
        let guess_format = |encoded_image: &[u8]| match image_crate::guess_format(encoded_image) {
            Ok(image_crate::ImageFormat::Png) => Some(image_crate::ImageFormat::Png),
            Ok(image_crate::ImageFormat::Jpeg) => Some(image_crate::ImageFormat::Jpeg),
            Ok(image_crate::ImageFormat::Bmp) => Some(image_crate::ImageFormat::Bmp),
            Ok(image_crate::ImageFormat::Gif) => Some(image_crate::ImageFormat::Gif),
            _ => None,
        };
        let decoded_image = match source {
            image::Source::Uri(uri) if base.is_some() => match Scheme::parse(uri) {
                Scheme::Data(Some(annoying_case), base64) => {
                    let encoded_image = base64::decode(base64).map_err(GLTF_Error::Base64)?;
                    let encoded_format = match annoying_case {
                        "image/png" => image_crate::ImageFormat::Png,
                        "image/jpeg" => image_crate::ImageFormat::Jpeg,
                        "image/bmp" => image_crate::ImageFormat::Bmp,
                        "image/gif" => image_crate::ImageFormat::Gif,
                        _ => match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(GLTF_Error::UnsupportedImageEncoding),
                        },
                    };
                    image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?
                }
                Scheme::Unsupported => return Err(GLTF_Error::UnsupportedScheme),
                _ => {
                    let encoded_image = Scheme::read(base, uri)?;
                    let encoded_format = image::Source::mime_type_format(uri).unwrap_or(
                        match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(GLTF_Error::UnsupportedImageEncoding),
                        },
                    );
                    image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?
                }
            },
            image::Source::View { view, json } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let encoded_image = &parent_buffer_data[begin..end];
                let encoded_format = match json.mime_type.as_str() {
                    "image/png" => image_crate::ImageFormat::Png,
                    "image/jpeg" => image_crate::ImageFormat::Jpeg,
                    "image/bmp" => image_crate::ImageFormat::Bmp,
                    "image/gif" => image_crate::ImageFormat::Gif,
                    _ => match guess_format(&encoded_image) {
                        Some(format) => format,
                        None => return Err(GLTF_Error::UnsupportedImageEncoding),
                    },
                };
                image_crate::load_from_memory_with_format(encoded_image, encoded_format)?
            }
            _ => return Err(GLTF_Error::ExternalReferenceInSliceImport),
        };
        image::Data::new(decoded_image)
    }
}
