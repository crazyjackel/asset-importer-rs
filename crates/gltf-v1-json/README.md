# gltf-v1-json

A Rust crate providing JSON serialization and deserialization for glTF 1.0 specification. This crate is part of the `asset-importer-rs` project and implements the complete glTF 1.0 JSON schema with validation support.

## Features

- Complete glTF 1.0 JSON schema implementation
- Serde-based serialization and deserialization
- Comprehensive validation system
- Support for all core glTF 1.0 components:
  - Accessors and Buffers
  - Animations and Skins
  - Cameras and Lights
  - Materials and Textures
  - Meshes and Nodes
  - Scenes and Assets
  - Shaders and Programs
- Extension support through feature flags
- Path-based error reporting

## Supported Extensions

The following glTF 1.0 extensions are supported through feature flags:

- `KHR_binary_glTF`: Binary buffer support
- `KHR_materials_common`: Common material types

## Dependencies

- `serde`: Serialization framework
- `serde_json`: JSON serialization
- `indexmap`: Indexed hash map with serialization support
- `gltf-v1-derive`: Validation derive macros

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gltf-v1-json = { path = "../path/to/gltf-v1-json" }

# Enable specific extensions
[features]
default = []
extensions = []
KHR_binary_glTF = []
KHR_materials_common = []
```

## Example

```rust
use gltf_v1_json::{
    Root,
    deserialize,
    serialize,
};

// Deserialize from JSON
let json_str = r#"{
    "asset": { "version": "1.0" },
    "scenes": [],
    "meshes": []
}"#;
let gltf: Root = deserialize::from_str(json_str)?;

// Serialize to JSON
let json = serialize::to_string_pretty(&gltf)?;
```

## Core Components

The crate provides comprehensive support for all glTF 1.0 components:

- **Asset**: Version and metadata information
- **Buffer**: Raw data storage
- **Accessor**: Buffer access and type information
- **Animation**: Keyframe animations
- **Camera**: Camera definitions
- **Material**: Material properties and techniques
- **Mesh**: Geometry data
- **Node**: Scene graph nodes
- **Scene**: Scene organization
- **Shader**: GLSL shader programs
- **Skin**: Skeletal animations
- **Texture**: Image and sampler definitions

## Validation

The crate includes a robust validation system:

- Automatic validation through derive macros
- Path-based error reporting
- Extension-specific validation
- Custom validation hooks

## License

This project is part of the `asset-importer-rs` workspace and follows its licensing terms.
