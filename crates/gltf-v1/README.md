# gltf-v1

A Rust crate providing comprehensive support for glTF 1.0 specification, including binary GLB format support. This crate is part of the `asset-importer-rs` project and serves as the main implementation for glTF 1.0 file handling.

## Features

- Complete glTF 1.0 specification support
- Binary GLB format support
- Image loading and processing
- Buffer management
- Comprehensive error handling
- Mathematical utilities for 3D operations
- Support for all core glTF 1.0 components:
  - Accessors and Buffers
  - Animations and Skins
  - Cameras and Lights
  - Materials and Textures
  - Meshes and Nodes
  - Scenes and Assets

## Supported Extensions

The following glTF 1.0 extensions are supported through feature flags:

- `KHR_binary_glTF` (enabled by default): Binary buffer support
- `KHR_materials_common`: Common material types

## Dependencies

- `gltf-v1-json`: JSON schema implementation
- `image`: Image processing (jpeg, png, bmp, gif)
- `base64`: Base64 encoding/decoding
- `byteorder`: Binary data handling
- `urlencoding`: URI encoding/decoding
- `indexmap`: Indexed hash map support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gltf-v1 = { path = "../path/to/gltf-v1" }

# Enable specific extensions
[features]
default = ["KHR_binary_glTF"]
KHR_binary_glTF = []
KHR_materials_common = []
```

## Example

```rust
use gltf_v1::{
    Document,
    Gltf,
    import_buffers,
    import_images,
};

// Load a glTF file
let gltf = Gltf::open("model.gltf")?;
let document = Document::from_gltf(gltf)?;

// Import binary data
let buffers = import_buffers(&document, "model.bin")?;
let images = import_images(&document, "textures")?;
```

## Core Components

The crate provides comprehensive support for all glTF 1.0 components:

- **Document**: Main entry point for glTF file handling
- **Buffer**: Raw data storage and management
- **Accessor**: Buffer access and type information
- **Animation**: Keyframe animations
- **Camera**: Camera definitions and parameters
- **Material**: Material properties and techniques
- **Mesh**: Geometry data and primitives
- **Node**: Scene graph nodes and transformations
- **Scene**: Scene organization and hierarchy
- **Skin**: Skeletal animations and bindings
- **Texture**: Image and sampler definitions
- **Light**: Light source definitions
- **Math**: 3D mathematics utilities

## Binary Support

The crate includes comprehensive support for binary GLB files:

- Binary chunk parsing
- Buffer data extraction
- Image data handling
- Base64 and URI decoding
- Binary data validation

## Error Handling

The crate provides a robust error handling system:

- Detailed error types for each operation
- Path-based error reporting
- Binary format validation
- Resource loading errors
- Extension-specific errors

## License

This project is part of the `asset-importer-rs` workspace and follows its licensing terms.
