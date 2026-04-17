# FBX-DOM

A Document Model for Reading in FBX.

Supports 7300 ASCII

## Binary ingress spike (`fbxcel-loader-spike`)

Primary binary ingress is `fbxcel` **tree API** (`tree::any::AnyTree`) for now.

- We parse binary bytes into an `AnyTree`.
- We adapt tree nodes into existing `Document` fields (header, definitions, globals, objects, connections).
- The `AnyTree` is treated as an **ephemeral adapter input** and is dropped after
  `Document` materialization, so we avoid keeping both a long-lived `fbxcel` tree
  and a second binary DOM copy in memory.

Pull-parser remains useful as an internal building block (and can be revisited for
streaming hot paths), but the canonical ingress in `fbx-dom` is tree-first while
the shape of `Document` is stabilized.


How to Write:

FBX Element ->

u64 id,
usize ref Document
usize Element Index
