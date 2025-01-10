# asset-importer-rs
Assimp, but in Rust


## Implementation Notes

### Implementation Plan 0.1.0

The goal for version 0.1.0 is to produce a working rust-safe version of Assimp that provides GLTF and OBJ files formats. To minimize on potential unsafety, pointers, despite being obvious direct improvements to performance, will be eschewed in favor of base rust smart-types. The goal is to build a working model and slowing introduce unsafety for the sake of matching performance.

As part of 0.1.0, a benchmark system should be set up to compare native assimp versus rust-assimp versions of the same command to begin to focus in on parity. Numbers should be reported and bounties assigned for performance improvements to particular regions.

For the most part, there will be a default towards public external access, but ideally internals are overtime fully encapsulated.

Specific Feature-Flag parity with Assimp will be considered as a future excursion with the exception of double precision as the means of testing feature flags.

The active goal is to lay out the code and then optimize later. As part of this, a redocumentation phase will commence after 0.1.0 works to categorize a set of 'issues' and engage in providing documentation on the work.

All documentation provided should have corresponding testing.

Some key differences:
- AiTexture will use an enum for format rather than a text-based hint. This should reduce memory usage and result in higher stability in terms of options
- AiMaterial encodes AiPropertyInfo differently in order to retain semantics, taking advantage of rust's enum system for types. This makes working with it easier.