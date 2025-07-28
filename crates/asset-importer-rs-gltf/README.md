<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> asset-importer-rs-gltf </h1>
<h3 align="center"> glTF 2.0 import and export functionality </h3>
<h5 align="center"> Complete glTF 2.0 specification support with extensions </h5>

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
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE CRATE -->
<h2 id="about-the-crate"> :pencil: About The Crate</h2>

<p align="justify"> 
  asset-importer-rs-gltf provides comprehensive glTF 2.0 import and export functionality for the asset-importer-rs project. This implementation supports the full glTF 2.0 specification including all major extensions, making it a complete solution for working with glTF assets in Rust applications.
</p>

<p align="justify">
  This crate provides the essential functionality for:
</p>

<ul>
  <li><b>glTF 2.0 Import</b> - Complete scene graph construction from glTF files</li>
  <li><b>glTF 2.0 Export</b> - Full scene serialization to glTF format</li>
  <li><b>Material System</b> - Comprehensive PBR material support with extensions</li>
  <li><b>Mesh Processing</b> - Geometry and mesh data handling</li>
  <li><b>Animation Support</b> - Keyframe and morph target animations</li>
  <li><b>Extension Support</b> - Extensive glTF extension compatibility</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Complete glTF 2.0 Support</b> - Full specification compliance with all major features</li>
  <li><b>Comprehensive Material System</b> - PBR materials with extensive extension support</li>
  <li><b>Mesh and Geometry Handling</b> - Complete mesh data processing and optimization</li>
  <li><b>Animation Framework</b> - Support for keyframe and morph target animations</li>
  <li><b>Camera and Light Support</b> - Complete camera and lighting system</li>
  <li><b>Texture Processing</b> - Advanced texture and image handling</li>
  <li><b>Node Hierarchy</b> - Full scene graph and node hierarchy support</li>
  <li><b>Extension Ecosystem</b> - Extensive glTF extension compatibility</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- SUPPORTED EXTENSIONS -->
<h2 id="supported-extensions"> :puzzle_piece: Supported Extensions</h2>

<p>The following glTF extensions are fully supported:</p>

<h3>Material Extensions</h3>
<ul>
  <li><b>KHR_texture_transform</b> - Texture coordinate transformations</li>
  <li><b>KHR_materials_unlit</b> - Unlit material support</li>
  <li><b>KHR_materials_transmission</b> - Transmission material properties</li>
  <li><b>KHR_materials_ior</b> - Index of refraction</li>
  <li><b>KHR_materials_volume</b> - Volume material properties</li>
  <li><b>KHR_materials_specular</b> - Specular material properties</li>
  <li><b>KHR_materials_pbrSpecularGlossiness</b> - Specular-glossiness workflow</li>
  <li><b>KHR_materials_emissive_strength</b> - Enhanced emissive materials</li>
</ul>

<h3>Lighting Extensions</h3>
<ul>
  <li><b>KHR_lights_punctual</b> - Point, spot, and directional lights</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core Import/Export</h3>
<ul>
  <li><b>Gltf2Importer</b> - Main glTF 2.0 import functionality</li>
  <li><b>Gltf2Exporter</b> - Main glTF 2.0 export functionality</li>
  <li><b>Gltf2ImportError</b> - Import error handling</li>
  <li><b>Gltf2ExportError</b> - Export error handling</li>
</ul>

<h3>Import Pipeline</h3>
<ul>
  <li><b>Scene Graph Construction</b> - Complete node hierarchy building</li>
  <li><b>Material Loading</b> - PBR material and texture processing</li>
  <li><b>Mesh Processing</b> - Geometry data import and optimization</li>
  <li><b>Animation Import</b> - Keyframe and morph target data</li>
  <li><b>Camera & Light Setup</b> - Camera and lighting configuration</li>
</ul>

<h3>Export Pipeline</h3>
<ul>
  <li><b>Scene Serialization</b> - Complete scene graph export</li>
  <li><b>Material Export</b> - PBR material and texture writing</li>
  <li><b>Mesh Writing</b> - Geometry data serialization</li>
  <li><b>Animation Export</b> - Animation data serialization</li>
  <li><b>Extension Support</b> - glTF extension compatibility</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
asset-importer-rs-gltf = "0.3.0"

# Or for development from source:
asset-importer-rs-gltf = { path = "../path/to/asset-importer-rs-gltf" }

# Enable specific extensions as needed
[features]
default = [
    "guess_mime_type",
    "extensions",
    "KHR_texture_transform",
    "KHR_materials_unlit",
    "KHR_materials_transmission",
    "KHR_materials_ior",
    "KHR_materials_volume",
    "KHR_materials_specular",
    "KHR_materials_pbrSpecularGlossiness",
    "KHR_materials_emissive_strength",
    "KHR_lights_punctual",
]
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic glTF import example:</p>

<pre><code>use asset_importer_rs_gltf::{
    Gltf2Importer,
    Gltf2ImportError,
};
use std::path::Path;

// Create an importer
let importer = Gltf2Importer::new();

// Import a glTF file
let scene = importer.import_file(Path::new("model.gltf"))?;

// Access scene data
println!("Scene has {} meshes", scene.meshes.len());
println!("Scene has {} materials", scene.materials.len());
</code></pre>

<p>Basic glTF export example:</p>

<pre><code>use asset_importer_rs_gltf::{
    Gltf2Exporter,
    Gltf2ExportError,
};
use asset_importer_rs_scene::AiScene;

// Create an exporter
let exporter = Gltf2Exporter::new();

// Export scene to glTF
let scene = AiScene::default(); // Your scene data
exporter.export_file(&scene, Path::new("output.gltf"))?;
</code></pre>

<p>Working with extensions:</p>

<pre><code>use asset_importer_rs_gltf::Gltf2Importer;

// Create importer with specific extension support
let mut importer = Gltf2Importer::new();
importer.enable_extension("KHR_materials_unlit")?;
importer.enable_extension("KHR_lights_punctual")?;

// Import with extension support
let scene = importer.import_file("model_with_extensions.gltf")?;
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ARCHITECTURE -->
<h2 id="architecture"> :small_orange_diamond: Architecture</h2>

<p align="justify"> 
  The glTF crate provides a complete implementation of the glTF 2.0 specification with a modular architecture that separates import and export concerns. The importer handles scene graph construction, material processing, and mesh optimization, while the exporter manages serialization, texture handling, and extension support.
</p>

<p align="justify">
  The crate integrates seamlessly with the asset-importer-rs ecosystem, providing glTF-specific implementations of the core import/export traits while maintaining compatibility with the broader 3D asset pipeline.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>gltf</b> - Core glTF parsing and validation</li>
  <li><b>asset-importer-rs-core</b> - Core import/export functionality</li>
  <li><b>asset-importer-rs-scene</b> - Scene data structures</li>
  <li><b>image</b> - Image processing and format support</li>
  <li><b>base64</b> - Base64 encoding/decoding</li>
  <li><b>serde_json</b> - JSON parsing and serialization</li>
  <li><b>urlencoding</b> - URI encoding/decoding</li>
  <li><b>bytemuck</b> - Safe type casting</li>
</ul>

<h3>Feature Dependencies</h3>
<ul>
  <li><b>gltf/extensions</b> - glTF extension support</li>
  <li><b>gltf/guess_mime_type</b> - MIME type detection</li>
</ul>

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
  <a href="https://crates.io/crates/asset-importer-rs-gltf">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/asset-importer-rs-gltf">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>
