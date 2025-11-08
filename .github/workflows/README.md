# GitHub Workflows

This directory contains CI/CD workflows for the project.

## Workflows

### `cargo.yml` - Base Tests
Runs standard Rust quality checks:
- Documentation tests
- Clippy linting
- Code formatting (rustfmt)
- Semantic versioning checks

**Triggers:** Push and pull requests to `main` branch

### `benchmark.yml` - Benchmarks
Runs performance benchmarks:
- Executes cargo benchmarks
- Stores results and tracks performance over time
- Alerts on performance regressions (>120% threshold)

**Triggers:** Push to `main` branch only

### `core.yml` - Core Crate Tests
Tests the `asset-importer-rs-core` crate:
- Builds and tests the core crate
- Triggers when core or scene crates change

**Triggers:** Push to `main` branch, or pull requests to `main` when core/scene crates change

### `scene.yml` - Scene Crate Tests
Tests the `asset-importer-rs-scene` crate:
- Builds and tests the scene crate
- Triggers when scene crate changes

**Triggers:** Push to `main` branch, or pull requests to `main` when scene crate changes

### `post-process.yml` - Post-Process Crate Tests
Tests the `asset-importer-rs-post-process` crate:
- Builds and tests the post-process crate
- Triggers when post-process, core, or scene crates change

**Triggers:** Push to `main` branch, or pull requests to `main` when post-process/core/scene crates change

### `format-gltf.yml` - GLTF 2.0 Format Tests
Tests GLTF 2.0 import/export functionality:
- Tests with sample assets from KhronosGroup/glTF-Sample-Assets
- Tests with different feature combinations (default, minimal, extras)
- Tests the gltf crate directly

**Triggers:** Push to `main` branch, or pull requests to `main` when GLTF 2.0 crate or dependencies change

### `format-gltf-v1.yml` - GLTF V1 Format Tests
Tests GLTF V1 format import/export:
- Tests the gltf-v1 format crate
- Runs integration tests

**Triggers:** Push to `main` branch, or pull requests to `main` when GLTF V1 format crate or dependencies change

### `format-obj.yml` - OBJ Format Tests
Tests OBJ format import/export:
- Tests the obj format crate
- Runs integration tests

**Triggers:** Push to `main` branch, or pull requests to `main` when OBJ format crate or dependencies change

### `dep-gltf-v1.yml` - GLTF V1 Library Tests
Tests the `gltf-v1` library crate:
- Builds and tests the gltf-v1 library
- Triggers when gltf-v1 or gltf-v1-json crates change

**Triggers:** Push to `main` branch, or pull requests to `main` when gltf-v1 library or dependencies change

### `dep-gltf-v1-json.yml` - GLTF V1 JSON Tests
Tests the `gltf-v1-json` crate:
- Builds and tests the gltf-v1-json crate
- Triggers when gltf-v1-json or gltf-v1-derive crates change

**Triggers:** Push to `main` branch, or pull requests to `main` when gltf-v1-json or gltf-v1-derive crates change

### `gltf-v1-derive.yml` - GLTF V1 Derive Tests
Tests the `gltf-v1-derive` crate:
- Builds and tests the gltf-v1-derive proc-macro crate
- Triggers when gltf-v1-derive crate changes

**Triggers:** Push to `main` branch, or pull requests to `main` when gltf-v1-derive crate changes
