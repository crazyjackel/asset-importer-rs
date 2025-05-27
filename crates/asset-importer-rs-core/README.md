# asset-importer-rs-core

A Rust crate providing the core functionality for importing and exporting 3D assets, inspired by the Assimp library. This crate is part of the `asset-importer-rs` project.

## Features

- Flexible asset import/export system
- Configurable import/export pipelines
- Post-processing capabilities
- Error handling and reporting
- Support for multiple file formats
- Extensible importer/exporter architecture

## Main Components

- **Importer**: Core import functionality and format support
- **Exporter**: Asset export capabilities
- **PostProcess**: Post-processing operations for imported assets
- **Config**: Configuration management for import/export operations
- **Error**: Comprehensive error handling system
- **ImporterDesc**: Format-specific importer descriptions and capabilities

## Dependencies

- `enumflags2`: For flag-based enums
- `thiserror`: For error handling
- `serde`: For serialization support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
asset-importer-rs-core = { path = "../path/to/asset-importer-rs-core" }
```

## Example

```rust
use asset_importer_rs_core::{
    AiImporter,
    AiExporter,
    AiPostProcess,
    AiConfig,
};

// Configure and use the import/export system
```

## Architecture

The core crate provides the fundamental building blocks for asset import/export:

- **Import Pipeline**: Handles the loading and parsing of 3D assets
- **Export Pipeline**: Manages the writing and serialization of assets
- **Post-Processing**: Applies transformations and optimizations to imported data
- **Configuration**: Controls import/export behavior and format-specific settings

## License

This project is part of the `asset-importer-rs` workspace and follows its licensing terms.
