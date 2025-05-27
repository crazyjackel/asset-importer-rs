# asset-importer-rs-gltf

A Rust crate providing glTF 2.0 import and export functionality for the `asset-importer-rs` project. This implementation supports the full glTF 2.0 specification including all major extensions.

## Features

- Complete glTF 2.0 import/export support
- Comprehensive material system with PBR support
- Mesh and geometry handling
- Animation support
- Camera and light definitions
- Texture and image processing
- Node hierarchy and scene graph
- Extensive extension support

## Supported Extensions

The following glTF extensions are supported:

- `KHR_texture_transform`: Texture coordinate transformations
- `KHR_materials_unlit`: Unlit material support
- `KHR_materials_transmission`: Transmission material properties
- `KHR_materials_ior`: Index of refraction
- `KHR_materials_volume`: Volume material properties
- `KHR_materials_specular`: Specular material properties
- `KHR_materials_pbrSpecularGlossiness`: Specular-glossiness workflow
- `KHR_materials_emissive_strength`: Enhanced emissive materials
- `KHR_lights_punctual`: Point, spot, and directional lights

## Dependencies

- `gltf`: Core glTF parsing and validation
- `asset-importer-rs-core`: Core import/export functionality
- `asset-importer-rs-scene`: Scene data structures
- `image`: Image processing and format support
- `base64`: Base64 encoding/decoding
- `serde_json`: JSON parsing and serialization
- `urlencoding`: URI encoding/decoding
- `bytemuck`: Safe type casting

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
asset-importer-rs-gltf = { path = "../path/to/asset-importer-rs-gltf" }

# Enable specific extensions as needed
[features]
default = [
    "guess_mime_type",
    "extensions",
    "KHR_texture_transform",
    "KHR_materials_unlit",
    # ... other extensions
]
```

## Example

```rust
use asset_importer_rs_gltf::{
    Gltf2Importer,
    Gltf2Exporter,
};

// Import a glTF file
let importer = Gltf2Importer::new();
let scene = importer.import_file("model.gltf")?;

// Export to glTF
let exporter = Gltf2Exporter::new();
exporter.export_file(&scene, "output.gltf")?;
```

## Implementation Details

The crate provides comprehensive support for glTF 2.0 through several key components:

- **Import Pipeline**:
  - Scene graph construction
  - Material and texture loading
  - Mesh and geometry processing
  - Animation data import
  - Camera and light setup

- **Export Pipeline**:
  - Scene serialization
  - Material and texture export
  - Mesh and geometry writing
  - Animation data export
  - Camera and light configuration

## License

This project is part of the `asset-importer-rs` workspace and follows its licensing terms.
