# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-post-process-v0.4.0...asset-importer-rs-post-process-v0.4.1) - 2026-08-11

### Fixed

- post-process fixes

## [0.4.0](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-post-process-v0.3.0...asset-importer-rs-post-process-v0.4.0) - 2026-05-05

### Other

- more changes from review. adjusted apply_flip_material from suggestion. Revert AiUvTransform translation back to AiVector2D.
- Made changes for PR Review on UV-Flip Post-Process feature. Changed AiUVTransform to have AiVector3D instead of AiVector2D for support of 3D texture mapping uv transforms.
- Finished refactoring AiMaterialProperty and associated code. Added prototype for flipping uvs post-processing.
- same as before
- flip_uvs_feature, need to add more tests.
- added post-processing

### Removed

- removed UvFlipVariant::Z and fixed error processing code.
