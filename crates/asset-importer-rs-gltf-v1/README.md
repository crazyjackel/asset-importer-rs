# asset-importer-rs-gltf-v1

A Rust crate providing glTF 1.0 import functionality for the `asset-importer-rs` project. This implementation supports the legacy glTF 1.0 specification, including the KHR_materials_common extension.

## Features

- Complete glTF 1.0 import support
- Legacy material system with KHR_materials_common
- Mesh and geometry handling
- Camera and light definitions
- Texture and image processing
- Node hierarchy and scene graph
- Asset management and loading

## Supported Extensions

The following glTF 1.0 extension is supported:

- `KHR_materials_common`: Legacy material system with common material types

## Dependencies

- `gltf-v1`: Core glTF 1.0 parsing and validation
- `asset-importer-rs-core`: Core import functionality
- `asset-importer-rs-scene`: Scene data structures
- `serde_json`: JSON parsing
- `bytemuck`: Safe type casting
- `enumflags2`: Flag-based enums

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
asset-importer-rs-gltf-v1 = { path = "../path/to/asset-importer-rs-gltf-v1" }

# Enable KHR_materials_common extension
[features]
default = ["KHR_materials_common"]
```

## Example

```rust
use asset_importer_rs_gltf_v1::{
    GltfImporter,
    GltfAsset,
};

// Import a glTF 1.0 file
let importer = GltfImporter::new();
let asset = GltfAsset::from_file("model.gltf")?;
let scene = importer.import(&asset)?;
```

## Implementation Details

The crate provides support for glTF 1.0 through several key components:

- **Import Pipeline**:
  - Scene graph construction
  - Material and texture loading
  - Mesh and geometry processing
  - Camera and light setup
  - Asset management

- **Asset Management**:
  - File loading and parsing
  - Resource management
  - Buffer handling
  - Image loading
  - Shader management

## Legacy Support

This crate is specifically designed for handling legacy glTF 1.0 files. For modern glTF 2.0 support, please use the `asset-importer-rs-gltf` crate instead.

## License

This project is part of the `asset-importer-rs` workspace and follows its licensing terms.
