# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-v0.4.0...asset-importer-rs-v0.4.1) - 2026-08-19

### Added

- stub out asset-importer-rs-dae

### Fixed

- cargo fmt
- source project cargo clippy fixes
- add ignore files

### Other

- Merge pull request #52 from crazyjackel/dependabot/github_actions/release-plz/action-0.5.131
- Merge pull request #83 from crazyjackel/dependabot/github_actions/actions/checkout-7.0.1
- Bump actions/checkout from 4.3.1 to 7.0.1
- Merge pull request #73 from martinfrances107/cargo-doc-workspace
- Restrict the GITHUB_TOKEN permissions.
- Merge pull request #55 from crazyjackel/dependabot/github_actions/Swatinem/rust-cache-6323deb102c322ba6fcbdcafc7e3dddab59af2b6
- Merge pull request #53 from crazyjackel/dependabot/github_actions/SonarSource/sonarqube-scan-action-8.2.1
- Bump actions/download-artifact from 4.3.0 to 8.0.1
- Bump criterion to 0.8.2.
- bumped actions/upload-artifact to version 7.
- release

## [0.4.0](https://github.com/crazyjackel/asset-importer-rs/compare/asset-importer-rs-v0.3.0...asset-importer-rs-v0.4.0) - 2026-05-05

### Fixed

- fix up mesh_geometry
- fix up objects

### Other

- Testing Release-Plz
- Added Logging for ignoring errors
- - Added CodeRabbit
- Adding Cargo.lock
- Added fbx-dom and fbxscii to the sonar merge
- formatting fix
- ready for review
- Added children fetching and improved mesh_geometry tests
- FBX DOM finished up, ready for more involved testing
- Add animation
- all fixed up
- Added animation fields to objects.
- update some objects
- - Material fixes
- video file reviewed and accepted
- base64 decode fbx-dom video
- starting to fix up objects
- Added Objects for FBX based on Assimp
- Added ability to access connections on objects
- Added regular FBX import
- Added fbxcel loading from tree into DOM.
- working on document
- begun developing API for reading fbx-dom
- - added global settings
- begin writing the fbxscii loader
- formatting fixes
- rename to amphithreatre and added document parsing
- Parser Improvements for Element Handle, beginning to write document
- setting up fbx dom
- Initialize Parser
- Tokenizer Fixes:
- Merge branch 'main' into jlevitt/fbx-parser
- Update
- fbxscii initial tokenizer written
- obj integration
- comment generation
- added post-processing
