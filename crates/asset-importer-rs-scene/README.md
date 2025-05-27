# assimp-rs-scene

A Rust crate providing data structures and types for working with 3D scene data, inspired by the Assimp library. This crate is part of the `asset-importer-rs` project.

## Features

- Complete set of 3D scene data structures
- Support for animations and keyframes
- Material and texture handling
- Camera and light definitions
- Mesh and bone structures
- Vector and matrix mathematics
- Color and quaternion utilities

## Main Components

- **Scene**: Core scene graph structure with node hierarchy
- **Mesh**: Geometry data including vertices, faces, and bone weights
- **Material**: Material properties and texture mappings
- **Animation**: Keyframe animations and morph targets
- **Camera**: Camera definitions and parameters
- **Light**: Various light source types and properties
- **Texture**: Texture data and format handling

## Dependencies

- `bytemuck`: For safe type casting
- `enumflags2`: For flag-based enums
- `num_enum`: For numeric enums
- `image`: For image processing

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
assimp-rs-scene = { path = "../path/to/assimp-rs-scene" }
```

## Example

```rust
use asset_importer_rs_scene::{
    AiScene,
    AiMesh,
    AiMaterial,
    AiVector3D,
    AiMatrix4x4,
};

// Create and manipulate scene data
```

## License

This project is part of the `asset-importer-rs` workspace and follows its licensing terms.
