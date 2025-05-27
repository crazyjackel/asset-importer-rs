# gltf-v1-derive

A Rust procedural macro crate that provides derive macros for glTF 1.0 validation and serialization. This crate is part of the `asset-importer-rs` project and is used internally by the glTF 1.0 implementation.

## Features

- `#[derive(Validate)]` macro for implementing validation traits
- Custom validation hooks through attributes
- Automatic field validation generation
- Support for nested structure validation
- Path-based error reporting

## Dependencies

- `proc-macro2`: Procedural macro support
- `quote`: Token stream generation
- `syn`: Rust code parsing
- `inflections`: String case conversion utilities

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gltf-v1-derive = { path = "../path/to/gltf-v1-derive" }
```

## Example

```rust
use gltf_v1_derive::Validate;

#[derive(Validate)]
struct MyStruct {
    field1: String,
    field2: i32,
}

// With custom validation hook
#[derive(Validate)]
#[gltf(validate_hook = "custom_validate")]
struct CustomStruct {
    field1: String,
    field2: i32,
}

fn custom_validate(
    this: &CustomStruct,
    root: &Root,
    path: impl Fn() -> Path,
    report: &mut impl FnMut(&dyn Fn() -> Path, Error),
) {
    // Custom validation logic
}
```

## Implementation Details

The crate provides procedural macros for:

- **Validation Derive**: Automatically implements the `Validate` trait for structs
  - Generates validation code for each field
  - Supports custom validation hooks
  - Handles nested structure validation
  - Provides path-based error reporting

## Internal Use

This crate is primarily intended for internal use within the `asset-importer-rs` project, specifically for the glTF 1.0 implementation. It provides the necessary derive macros for implementing validation and serialization traits.

## License

This project is part of the `asset-importer-rs` workspace and follows its licensing terms.
