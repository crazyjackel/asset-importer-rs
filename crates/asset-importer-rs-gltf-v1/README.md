<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> asset-importer-rs-gltf-v1 </h1>
<h3 align="center"> glTF 1.0 import and export functionality </h3>
<h5 align="center"> Legacy glTF 1.0 specification support with KHR_materials_common </h5>

<p align="center"> 
  <img src="https://img.shields.io/badge/Version-0.3.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge" alt="Rust Version">
</p>

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :book: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-crate"> ➤ About The Crate</a></li>
    <li><a href="#features"> ➤ Features</a></li>
    <li><a href="#supported-extensions"> ➤ Supported Extensions</a></li>
    <li><a href="#main-components"> ➤ Main Components</a></li>
    <li><a href="#getting-started"> ➤ Getting Started</a></li>
    <li><a href="#usage-examples"> ➤ Usage Examples</a></li>
    <li><a href="#architecture"> ➤ Architecture</a></li>
    <li><a href="#dependencies"> ➤ Dependencies</a></li>
    <li><a href="#legacy-support"> ➤ Legacy Support</a></li>
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE CRATE -->
<h2 id="about-the-crate"> :pencil: About The Crate</h2>

<p align="justify"> 
  asset-importer-rs-gltf-v1 provides glTF 1.0 import and export functionality for the asset-importer-rs project. This implementation supports the legacy glTF 1.0 specification, including the KHR_materials_common extension, making it essential for handling older glTF assets and maintaining backward compatibility.
</p>

<p align="justify">
  This crate provides the essential functionality for:
</p>

<ul>
  <li><b>glTF 1.0 Import</b> - Complete scene graph construction from legacy glTF files</li>
  <li><b>glTF 1.0 Export</b> - Full scene serialization to glTF 1.0 format</li>
  <li><b>Legacy Material System</b> - KHR_materials_common extension support</li>
  <li><b>Mesh Processing</b> - Geometry and mesh data handling</li>
  <li><b>Asset Management</b> - File loading, parsing, and resource management</li>
  <li><b>Backward Compatibility</b> - Support for older glTF 1.0 assets</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Complete glTF 1.0 Support</b> - Full legacy specification compliance</li>
  <li><b>Legacy Material System</b> - KHR_materials_common extension support</li>
  <li><b>Mesh and Geometry Handling</b> - Complete mesh data processing</li>
  <li><b>Camera and Light Support</b> - Legacy camera and lighting system</li>
  <li><b>Texture Processing</b> - Texture and image handling</li>
  <li><b>Node Hierarchy</b> - Scene graph and node hierarchy support</li>
  <li><b>Asset Management</b> - File loading, parsing, and resource management</li>
  <li><b>Buffer Handling</b> - Binary buffer and shader management</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- SUPPORTED EXTENSIONS -->
<h2 id="supported-extensions"> :puzzle_piece: Supported Extensions</h2>

<p>The following glTF 1.0 extension is fully supported:</p>

<h3>Material Extensions</h3>
<ul>
  <li><b>KHR_materials_common</b> - Legacy material system with common material types</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core Import/Export</h3>
<ul>
  <li><b>GltfImporter</b> - Main glTF 1.0 import functionality</li>
  <li><b>GltfExporter</b> - Main glTF 1.0 export functionality</li>
  <li><b>GLTFImportError</b> - Import error handling</li>
  <li><b>GltfExportError</b> - Export error handling</li>
</ul>

<h3>Import Pipeline</h3>
<ul>
  <li><b>Scene Graph Construction</b> - Complete node hierarchy building</li>
  <li><b>Material Loading</b> - KHR_materials_common material processing</li>
  <li><b>Mesh Processing</b> - Geometry data import and optimization</li>
  <li><b>Camera & Light Setup</b> - Legacy camera and lighting configuration</li>
  <li><b>Asset Management</b> - File loading and resource management</li>
</ul>

<h3>Export Pipeline</h3>
<ul>
  <li><b>Scene Serialization</b> - Complete scene graph export</li>
  <li><b>Material Export</b> - Legacy material and texture writing</li>
  <li><b>Mesh Writing</b> - Geometry data serialization</li>
  <li><b>Buffer Management</b> - Binary buffer and shader handling</li>
  <li><b>Extension Support</b> - glTF 1.0 extension compatibility</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
asset-importer-rs-gltf-v1 = "0.3.0"

# Or for development from source:
asset-importer-rs-gltf-v1 = { path = "../path/to/asset-importer-rs-gltf-v1" }

# Enable KHR_materials_common extension
[features]
default = ["KHR_materials_common"]
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic glTF 1.0 import example:</p>

<pre><code>use asset_importer_rs_gltf_v1::{
    GltfImporter,
    GLTFImportError,
};
use std::path::Path;

// Create an importer
let importer = GltfImporter::new();

// Import a glTF 1.0 file
let scene = importer.import_file(Path::new("legacy_model.gltf"))?;

// Access scene data
println!("Scene has {} meshes", scene.meshes.len());
println!("Scene has {} materials", scene.materials.len());
</code></pre>

<p>Basic glTF 1.0 export example:</p>

<pre><code>use asset_importer_rs_gltf_v1::{
    GltfExporter,
    GltfExportError,
};
use asset_importer_rs_scene::AiScene;

// Create an exporter
let exporter = GltfExporter::new();

// Export scene to glTF 1.0
let scene = AiScene::default(); // Your scene data
exporter.export_file(&scene, Path::new("output_v1.gltf"))?;
</code></pre>

<p>Working with KHR_materials_common:</p>

<pre><code>use asset_importer_rs_gltf_v1::GltfImporter;

// Create importer with KHR_materials_common support
let importer = GltfImporter::new();

// Import with legacy material support
let scene = importer.import_file("model_with_common_materials.gltf")?;

// Access legacy material data
for material in &scene.materials {
    // Handle KHR_materials_common properties
    println!("Material: {:?}", material);
}
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ARCHITECTURE -->
<h2 id="architecture"> :small_orange_diamond: Architecture</h2>

<p align="justify"> 
  The glTF v1 crate provides a complete implementation of the legacy glTF 1.0 specification with a focus on backward compatibility and legacy asset support. The importer handles scene graph construction, legacy material processing, and mesh optimization, while the exporter manages serialization, texture handling, and extension support for the older format.
</p>

<p align="justify">
  The crate integrates seamlessly with the asset-importer-rs ecosystem, providing glTF 1.0-specific implementations of the core import/export traits while maintaining compatibility with the broader 3D asset pipeline and ensuring legacy assets can be properly handled.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>gltf-v1</b> - Core glTF 1.0 parsing and validation</li>
  <li><b>asset-importer-rs-core</b> - Core import/export functionality</li>
  <li><b>asset-importer-rs-scene</b> - Scene data structures</li>
  <li><b>serde_json</b> - JSON parsing and serialization</li>
  <li><b>bytemuck</b> - Safe type casting</li>
  <li><b>enumflags2</b> - Flag-based enums</li>
  <li><b>base64</b> - Base64 encoding/decoding</li>
</ul>

<h3>Feature Dependencies</h3>
<ul>
  <li><b>gltf-v1/KHR_materials_common</b> - Legacy material extension support</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- LEGACY SUPPORT -->
<h2 id="legacy-support"> :warning: Legacy Support</h2>

<p align="justify"> 
  This crate is specifically designed for handling legacy glTF 1.0 files and maintaining backward compatibility with older assets. For modern glTF 2.0 support with the latest features and extensions, please use the <code>asset-importer-rs-gltf</code> crate instead.
</p>

<p align="justify">
  The glTF 1.0 specification has been superseded by glTF 2.0, but this crate ensures that legacy assets can still be properly imported, processed, and exported within the asset-importer-rs ecosystem.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- LICENSE -->
<h2 id="license"> :scroll: License</h2>

<p>This project is part of the <code>asset-importer-rs</code> workspace and follows its licensing terms. See the main project <a href="../../LICENSE">LICENSE</a> file for details.</p>

<p align="center"> 
  <strong>Copyright (c) 2024 Jackson Levitt</strong>
</p>

<p align="center">
  <a href="https://github.com/crazyjackel/asset-importer-rs">
    <img src="https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white" alt="GitHub">
  </a>
  <a href="https://crates.io/crates/asset-importer-rs-gltf-v1">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/asset-importer-rs-gltf-v1">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>
