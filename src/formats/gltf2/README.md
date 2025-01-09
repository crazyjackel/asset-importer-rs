# GLTF 2.0 

## Implementation Notes

### 0.1.0

Initial Implementation works on the basis of the GLTF 2.0 Library. The GLTF 2.0 Library will be less effective for loading and memory compared to the C++ Assimp implementation due to it doing full-loading for importing data. The JSON portion of GLTF 2.0 looks very effective from a quick scan given that rust's json loading in similar to rapidjson in performance with serde being relatively quick. Honestly though, you cannot tell without doing benchmarking. GLTF 2.0 needs a looking at for a better custom solution to get performance up that considers lazy-loading at least when it comes to memory. I would estimate -- without benchmarking -- that it should be near-par with assimp if that were done, but who knows?