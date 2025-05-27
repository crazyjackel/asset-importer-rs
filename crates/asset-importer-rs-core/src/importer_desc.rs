use enumflags2::{BitFlags, bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AiImporterFlags {
    SupportTextFlavor = 0x01,
    SupportBinaryFlavor = 0x02,
    SupportCompressedFlavor = 0x04,
    LimitedSupport = 0x08,
    Experimental = 0x10,
}

#[derive(Debug)]
pub struct AiImporterDesc {
    pub name: String,
    pub author: String,
    pub maintainer: String,
    pub comments: String,
    pub flags: BitFlags<AiImporterFlags>,
    pub min_major: u32,
    pub min_minor: u32,
    pub max_major: u32,
    pub max_minor: u32,
    pub extensions: Vec<String>,
}
