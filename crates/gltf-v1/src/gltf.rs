use std::path::Path;
use std::{fs, io, ops};

use crate::binary;
use crate::document::Document;
use crate::error::Result;

/// glTF JSON wrapper plus binary payload.
#[derive(Clone, Debug)]
pub struct Gltf {
    /// The glTF JSON wrapper.
    pub document: Document,

    /// The glTF binary payload in the case of binary glTF.
    pub blob: Option<Vec<u8>>,
}

impl Gltf {
    /// Convenience function that loads glTF from the file system.
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let gltf = Self::from_reader(reader)?;
        Ok(gltf)
    }
    /// Loads glTF from a reader without performing validation checks.
    pub fn from_reader_without_validation<R>(mut reader: R) -> Result<Self>
    where
        R: io::Read + io::Seek,
    {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        reader.seek(io::SeekFrom::Current(-4))?;
        let (json, blob): (json::Root, Option<Vec<u8>>);
        if magic.starts_with(b"glTF") {
            let mut glb = binary::Glb::from_reader(reader)?;
            // TODO: use `json::from_reader` instead of `json::from_slice`
            json = json::deserialize::from_slice(&glb.content)?;
            blob = glb.body.take().map(|x| x.into_owned());
        } else {
            json = json::deserialize::from_reader(reader)?;
            blob = None;
        };
        let document = Document::from_json_without_validation(json);
        Ok(Gltf { document, blob })
    }
    /// Loads glTF from a reader.
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: io::Read + io::Seek,
    {
        let gltf = Self::from_reader_without_validation(reader)?;
        gltf.document.validate()?;
        Ok(gltf)
    }

    pub fn from_slice_without_validation(slice: &[u8]) -> Result<Self> {
        let (json, blob): (json::Root, Option<Vec<u8>>);
        if slice.starts_with(b"glTF") {
            let mut glb = binary::Glb::from_slice(slice)?;
            json = json::deserialize::from_slice(&glb.content)?;
            blob = glb.body.take().map(|x| x.into_owned());
        } else {
            json = json::deserialize::from_slice(slice)?;
            blob = None;
        };
        let document = Document::from_json_without_validation(json);
        Ok(Gltf { document, blob })
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let gltf = Self::from_slice_without_validation(slice)?;
        gltf.document.validate()?;
        Ok(gltf)
    }
}

impl ops::Deref for Gltf {
    type Target = Document;
    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

impl ops::DerefMut for Gltf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.document
    }
}
