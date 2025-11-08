use enumflags2::{BitFlags, bitflags};

/// Flags that describe the capabilities and characteristics of an importer.
///
/// These flags provide information about what an importer supports and how it behaves.
/// They can be used to filter and select appropriate importers for specific use cases.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::AiImporterFlags;
/// use enumflags2::BitFlags;
///
/// let flags = AiImporterFlags::SupportTextFlavor | AiImporterFlags::SupportBinaryFlavor;
/// assert!(flags.contains(AiImporterFlags::SupportTextFlavor));
/// ```
#[bitflags]
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AiImporterFlags {
    /// The importer supports text-based file formats.
    SupportTextFlavor = 0x01,
    /// The importer supports binary file formats.
    SupportBinaryFlavor = 0x02,
    /// The importer supports compressed file formats.
    SupportCompressedFlavor = 0x04,
    /// The importer has limited support for the format.
    LimitedSupport = 0x08,
    /// The importer is experimental and may have bugs or incomplete features.
    Experimental = 0x10,
}

/// Description of an importer's capabilities and metadata.
///
/// This struct contains comprehensive information about an importer, including
/// its name, author, supported file extensions, and capabilities flags.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::{AiImporterDesc, AiImporterFlags};
/// use enumflags2::BitFlags;
///
/// let desc = AiImporterDesc {
///     name: "My Importer".to_string(),
///     author: "John Doe".to_string(),
///     maintainer: "John Doe".to_string(),
///     comments: "A custom importer for my format".to_string(),
///     flags: BitFlags::empty(),
///     min_major: 1,
///     min_minor: 0,
///     max_major: 2,
///     max_minor: 0,
///     extensions: vec!["my".to_string(), "format".to_string()],
/// };
/// ```
#[derive(Debug)]
pub struct AiImporterDesc {
    /// The name of the importer.
    pub name: String,
    /// The original author of the importer.
    pub author: String,
    /// The current maintainer of the importer.
    pub maintainer: String,
    /// Additional comments or description about the importer.
    pub comments: String,
    /// Flags describing the importer's capabilities and characteristics.
    pub flags: BitFlags<AiImporterFlags>,
    /// Minimum supported major version of the file format.
    pub min_major: u32,
    /// Minimum supported minor version of the file format.
    pub min_minor: u32,
    /// Maximum supported major version of the file format.
    pub max_major: u32,
    /// Maximum supported minor version of the file format.
    pub max_minor: u32,
    /// List of file extensions supported by this importer.
    pub extensions: Vec<String>,
}
