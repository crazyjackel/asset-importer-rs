# asset-importer-rs-dae

Collada (`.dae`) importer for [asset-importer-rs](https://github.com/crazyjackel/asset-importer-rs).

Import is stubbed: `DaeImporter` advertises `.dae` support and returns `DaeImportError::NotImplemented` until conversion work lands.

## Test assets (`tests/`)

| File | Source |
|------|--------|
| [`tests/cube.dae`](tests/cube.dae) | Example Collada cube from [wtsnz/cube.dae](https://gist.github.com/wtsnz/bfa11c40e04594b260255b5dc7956f26#file-cube-dae) (SceneKit Collada Exporter v1.0, 2018). |
