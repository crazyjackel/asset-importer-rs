# Profile Target

Generates flamegraphs, for a given gtlf(version2) files

For example, see profile.sh

```bash
cargo flamegraph --bin flamegraph_target -- -f ~/Avocado/glTF/Avocado.gltf
````

This allow gltf files with a long load time to be debugged.

# Future Work

At the moment this only supports Gltf2Importer
