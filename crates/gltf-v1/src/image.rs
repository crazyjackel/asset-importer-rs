use std::ops::Deref;

use image_crate::DynamicImage;

use crate::GLTF_Error;
use crate::error::Result;
use crate::{buffer, document::Document};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Format {
    /// Red only.
    R8,

    /// Red, green.
    R8G8,

    /// Red, green, blue.
    R8G8B8,

    /// Red, green, blue, alpha.
    R8G8B8A8,

    /// Red only (16 bits).
    R16,

    /// Red, green (16 bits).
    R16G16,

    /// Red, green, blue (16 bits).
    R16G16B16,

    /// Red, green, blue, alpha (16 bits).
    R16G16B16A16,

    /// Red, green, blue (32 bits float)
    R32G32B32FLOAT,

    /// Red, green, blue, alpha (32 bits float)
    R32G32B32A32FLOAT,
}
#[derive(Clone, Debug)]
pub enum Source<'a> {
    Uri(&'a str),

    /// Image data is contained in a buffer view.
    View {
        /// The buffer view containing the encoded image data.
        view: buffer::View<'a>,

        /// The image data MIME type.
        json: &'a json::extensions::image::BinaryImage,
    },
}

/// Image data used to create a texture.
#[derive(Clone, Debug)]
pub struct Image<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

#[derive(Clone, Debug)]
pub struct Data {
    /// The image pixel data (8 bits per channel).
    pub pixels: Vec<u8>,

    /// The image pixel data format.
    pub format: Format,

    /// The image width in pixels.
    pub width: u32,

    /// The image height in pixels.
    pub height: u32,
}
/// An `Iterator` that visits every accessor in a glTF asset.
#[derive(Clone, Debug)]
pub struct Images<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Image>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl ExactSizeIterator for Images<'_> {}
impl<'a> Iterator for Images<'a> {
    type Item = Image<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Image::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Image::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Image::new(self.document, index, json))
    }
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::image::Image,
    ) -> Self {
        Self {
            document,
            index,
            json,
        }
    }
    pub fn index(&self) -> &str {
        self.index
    }

    /// Returns the image data source.
    pub fn source(&self) -> Source<'a> {
        #[cfg(feature = "KHR_binary_glTF")]
        if let Some(image_extensions) = &self.json.extensions {
            if let Some(binary) = &image_extensions.khr_binary_gltf {
                let view = self
                    .document
                    .views()
                    .find(|x| x.index() == binary.buffer_view.value())
                    .unwrap();
                return Source::View { view, json: binary };
            }
        }
        let uri = self.json.uri.deref();
        Source::Uri(uri)
    }

    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }
}

impl<'a> Source<'a> {
    pub fn mime_type_format(uri: &'a str) -> Option<image_crate::ImageFormat> {
        match uri.rsplit('.').next() {
            Some("png") => Some(image_crate::ImageFormat::Png),
            Some("jpg") | Some("jpeg") => Some(image_crate::ImageFormat::Jpeg),
            Some("gif") => Some(image_crate::ImageFormat::Gif),
            Some("bmp") => Some(image_crate::ImageFormat::Bmp),
            _ => None,
        }
    }

    pub fn mime_type(&self) -> Option<&'a str> {
        match self {
            Source::Uri(uri) => {
                let format = Source::mime_type_format(uri);
                format.map(|x| x.to_mime_type())
            }
            Source::View { view: _, json } => Some(json.mime_type.as_str()),
        }
    }
}

impl Data {
    /// Note: We don't implement `From<DynamicImage>` since we don't want
    /// to expose such functionality to the user.
    pub(crate) fn new(image: DynamicImage) -> Result<Self> {
        use image_crate::GenericImageView;
        let format = match image {
            DynamicImage::ImageLuma8(_) => Format::R8,
            DynamicImage::ImageLumaA8(_) => Format::R8G8,
            DynamicImage::ImageRgb8(_) => Format::R8G8B8,
            DynamicImage::ImageRgba8(_) => Format::R8G8B8A8,
            DynamicImage::ImageLuma16(_) => Format::R16,
            DynamicImage::ImageLumaA16(_) => Format::R16G16,
            DynamicImage::ImageRgb16(_) => Format::R16G16B16,
            DynamicImage::ImageRgba16(_) => Format::R16G16B16A16,
            DynamicImage::ImageRgb32F(_) => Format::R32G32B32FLOAT,
            DynamicImage::ImageRgba32F(_) => Format::R32G32B32A32FLOAT,
            image => return Err(GLTF_Error::UnsupportedImageFormat(image)),
        };
        let (width, height) = image.dimensions();
        let pixels = image.into_bytes();
        Ok(Data {
            format,
            width,
            height,
            pixels,
        })
    }
}
