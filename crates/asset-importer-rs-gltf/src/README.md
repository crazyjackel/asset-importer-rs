# GLTF 2.0 

## Implementation Notes

### 0.1.0

Initial Implementation works on the basis of the GLTF 2.0 Library. The GLTF 2.0 Library will be less effective for loading and memory compared to the C++ Assimp implementation due to it doing full-loading for importing data. The JSON portion of GLTF 2.0 looks very effective from a quick scan given that rust's json loading in similar to rapidjson in performance with serde being relatively quick. Honestly though, you cannot tell without doing benchmarking. GLTF 2.0 needs a looking at for a better custom solution to get performance up that considers lazy-loading at least when it comes to memory. I would estimate -- without benchmarking -- that it should be near-par with assimp if that were done, but who knows?

Gltf-RS is missing a couple features:
1. mesh.extras.targetNames should be made available when extras is enabled as giving morph targets names is common.
2. Sheen and Clearcoat Materials are not JSON defined.

Assimp implementation differences:
1. Assimp loads buffers at the time of accessing via accessor, instead of all at once. This makes our implementation more memory intensive as we have to allocate memory for every buffer instead of just the buffers we need.
2. We save tagents weights when loading in meshes, using up memory instead of reloading the tangents, trading off memory for ease.