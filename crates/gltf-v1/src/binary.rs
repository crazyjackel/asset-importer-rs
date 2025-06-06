use std::{
    borrow::Cow,
    fmt,
    io::{self, Read},
    mem,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub enum Error {
    /// Io error occured.
    Io(::std::io::Error),
    /// Unsupported version.
    Version(u32),
    /// Magic says that file is not glTF.
    Magic([u8; 4]),
    Length {
        /// length specified in GLB header.
        length: u32,
        /// Actual length of data read.
        length_read: usize,
    },
}

/// Binary glTF contents.
#[derive(Clone, Debug)]
pub struct Glb<'a> {
    /// The header section of the `.glb` file.
    pub header: Header,
    /// The Content section of the `.glb` file.
    pub content: Cow<'a, [u8]>,
    /// The optional body(BIN) section of the `.glb` file.
    pub body: Option<Cow<'a, [u8]>>,
}

/// The header section of a .glb file.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Header {
    /// Must be `b"glTF"`.
    pub magic: [u8; 4],
    /// Must be `1`.
    pub version: u32,
    /// Must match the length of the parent .glb file.
    pub length: u32,
    /// Must match the length of the content section of the GLB. Must be greater than 0
    pub content_length: u32,
    /// Must be `0` for JSON
    pub content_format: u32,
}

impl Header {
    fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, Error> {
        use self::Error::Io;
        let mut magic = [0; 4];
        reader.read_exact(&mut magic).map_err(Io)?;
        // We only validate magic as we don't care for version and length of
        // contents, the caller does.  Let them decide what to do next with
        // regard to version and length.
        if &magic == b"glTF" {
            Ok(Self {
                magic,
                version: reader.read_u32::<LittleEndian>().map_err(Io)?,
                length: reader.read_u32::<LittleEndian>().map_err(Io)?,
                content_length: reader.read_u32::<LittleEndian>().map_err(Io)?,
                content_format: reader.read_u32::<LittleEndian>().map_err(Io)?,
            })
        } else {
            Err(Error::Magic(magic))
        }
    }

    fn size_of() -> usize {
        20
    }
}

impl<'a> Glb<'a> {
    pub fn from_slice(mut data: &'a [u8]) -> Result<Self, crate::GLTF_Error> {
        let header = Header::from_reader(&mut data)
            .and_then(|header| {
                let contents_length = header.length as usize - Header::size_of();
                if contents_length <= data.len() {
                    Ok(header)
                } else {
                    Err(Error::Length {
                        length: contents_length as u32,
                        length_read: data.len(),
                    })
                }
            })
            .map_err(crate::GLTF_Error::Binary)?;
        match header.version {
            1 => {
                let content_len = header.content_length;
                let mut content_buf = vec![0; content_len as usize];
                if let Err(e) = (&mut data).read_exact(&mut content_buf).map_err(Error::Io) {
                    return Err(crate::GLTF_Error::Binary(e));
                }

                let body_len = header.length - Header::size_of() as u32 - content_len;
                let body = if body_len != 0 {
                    let mut body_buf = vec![0; body_len as usize];
                    if let Err(e) = (&mut data).read_exact(&mut body_buf).map_err(Error::Io) {
                        return Err(crate::GLTF_Error::Binary(e));
                    }
                    Some(body_buf)
                } else {
                    None
                };

                Ok(Glb {
                    header,
                    content: content_buf.into(),
                    body: body.map(|x| x.into()),
                })
            }
            x => Err(crate::GLTF_Error::Binary(Error::Version(x))),
        }
    }
    pub fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, crate::GLTF_Error> {
        let header = Header::from_reader(&mut reader).map_err(crate::GLTF_Error::Binary)?;
        match header.version {
            1 => {
                let content_len = header.content_length;
                let mut content_buf = vec![0; content_len as usize];
                if let Err(e) = reader.read_exact(&mut content_buf).map_err(Error::Io) {
                    return Err(crate::GLTF_Error::Binary(e));
                }

                let body_len = header.length - Header::size_of() as u32 - content_len;
                let body = if body_len != 0 {
                    let mut body_buf = vec![0; body_len as usize];
                    if let Err(e) = reader.read_exact(&mut body_buf).map_err(Error::Io) {
                        return Err(crate::GLTF_Error::Binary(e));
                    }
                    Some(body_buf)
                } else {
                    None
                };

                Ok(Glb {
                    header,
                    content: content_buf.to_vec().into(),
                    body: body.map(|x| x.to_vec()).map(|x| x.into()),
                })
            }
            x => Err(crate::GLTF_Error::Binary(Error::Version(x))),
        }
    }
    pub fn to_writer<W: io::Write>(&self, mut writer: W) -> Result<(), crate::GLTF_Error> {
        // Write GLB header
        {
            let magic = b"glTF";
            let version: u32 = 1;
            let mut length = mem::size_of::<Header>() + self.content.len();
            align_to_multiple_of_four(&mut length);
            if let Some(body) = self.body.as_ref() {
                length += body.len();
                align_to_multiple_of_four(&mut length);
            }

            writer.write_all(&magic[..])?;
            writer.write_u32::<LittleEndian>(version)?;
            writer.write_u32::<LittleEndian>(length as u32)?;
        }

        // Write JSON chunk header
        {
            let mut length = self.content.len();
            let format: u32 = 0;
            align_to_multiple_of_four(&mut length);
            let padding = length - self.content.len();

            writer.write_u32::<LittleEndian>(length as u32)?;
            writer.write_u32::<LittleEndian>(format)?;
            writer.write_all(&self.content)?;
            for _ in 0..padding {
                writer.write_u8(0x20)?;
            }
        }

        if let Some(body) = self.body.as_ref() {
            let mut length = body.len();
            align_to_multiple_of_four(&mut length);
            let padding = length - body.len();
            writer.write_all(body)?;
            for _ in 0..padding {
                writer.write_u8(0)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Error::Version(_) => "unsupported version",
                Error::Magic(_) => "not glTF magic",
                Error::Length { .. } => "could not completely read the object",
                Error::Io(ref e) => return e.fmt(f),
            }
        )
    }
}

impl ::std::error::Error for Error {}

fn align_to_multiple_of_four(n: &mut usize) {
    *n = (*n + 3) & !3;
}
