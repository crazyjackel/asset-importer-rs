# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.2](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-scene-v0.4.1...asset-importer-rs-scene-v0.4.2) - 2026-09-08

### Added

- update clippy linting
- added triangulation post-processing

### Fixed

- gltf issues
- clippy fixes related to double_precision, if chains, extra into(), bad asserts, as_chunks vs. chunks_exact

### Other

- improved docstrings and memory-sharing for scratchpad
- lint and clippy changes

## [0.4.1](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-scene-v0.4.0...asset-importer-rs-scene-v0.4.1) - 2026-09-06

### Added

- added `node.rs` parsing
- added material loading

### Fixed

- coderabbit issues
- missing root name, better naming for nodes, matrix rotation normalization in construction
- cargo fmt
- cargo clippy --fix --lib -p asset-importer-rs-scene
- added type definition for coercion

### Other

- Now uniformly wrapping tests in a mod tests.

## [0.4.0](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-scene-v0.3.0...asset-importer-rs-scene-v0.4.0) - 2026-05-05

### Fixed

- fix identity matrix being incorrect

### Other

- Merge branch 'crazyjackel:main' into uv_flipping
- test fixes for new workflows
- added post-processing
