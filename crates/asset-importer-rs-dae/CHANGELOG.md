# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.2](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-dae-v0.4.1...asset-importer-rs-dae-v0.4.2) - 2026-09-08

### Added

- update clippy linting

### Fixed

- clippy fixes related to double_precision, if chains, extra into(), bad asserts, as_chunks vs. chunks_exact

## [0.4.1](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-dae-v0.4.0...asset-importer-rs-dae-v0.4.1) - 2026-09-06

### Added

- dae importer
- url_decoding used instead of adhoc untested code
- simplified the resolution of the index to better handle none values
- Added Textures
- handle lights
- camera parsing
- materials is small and quick for iteration, thereby material for symbol should be fine and we should have consistency.
- instance-cycle protection
- updated mesh parsing from nodes to work
- remove unused effect parameters
- added `node.rs` parsing
- added material loading
- began building `build_materials`
- first load and stubbed data
- moved into `importer.rs` and did magic check
- stub out asset-importer-rs-dae

### Fixed

- cargo fmt
- added image to handle multiple formats for data and load texels for embedded images
- added visitation bounds for current
- resolve_material_index fixed to option
- camera and mesh done
- format fix
- coderabbit issues
- scene mesh name map fix
- quick bounds checking
- cargo fmt
- missing root name, better naming for nodes, matrix rotation normalization in construction
- lint and clippy fixes
- import change

### Added

- Stub `DaeImporter` with `.dae` detection and `NotImplemented` import
