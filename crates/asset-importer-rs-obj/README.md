# Asset Importer Obj

## Notes

Why tObj over obj-rs? tObj did the splitting out of models and materials for easy parsing whereas obj-rs kept them together.

However, tObj should be replaced with a custom-loader at some point as tObj is not efficient for our use case. There is plenty of optimizations that can be gained from directly parsing, such as:
1. Not directly converting parsed Points, Lines, Triangles, Quads, and Polygons into face_arities, instead leave the handling to asset-importer-rs
2. Better Memory-Streaming by loading one Model at a time, doing work on it, and then moving on. (We lose out on Vec capacity benefits, but probably worth it)
3. Better Missing or Rare Attribute handling that tObj doesn't handle.

In general, it is better to write-up our own parser into our format, but getting things to work comes first.