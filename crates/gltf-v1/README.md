<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> gltf-v1 </h1>
<h3 align="center"> glTF 1.0 specification implementation </h3>
<h5 align="center"> Complete glTF 1.0 support with binary GLB format </h5>

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
    <li><a href="#binary-support"> ➤ Binary Support</a></li>
    <li><a href="#error-handling"> ➤ Error Handling</a></li>
    <li><a href="#dependencies"> ➤ Dependencies</a></li>
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE CRATE -->
<h2 id="about-the-crate"> :pencil: About The Crate</h2>

<p align="justify"> 
  gltf-v1 provides comprehensive support for the glTF 1.0 specification, including binary GLB format support. This crate is part of the asset-importer-rs project and serves as the main implementation for glTF 1.0 file handling, offering complete parsing, validation, and manipulation capabilities.
</p>

<p align="justify">
  This crate provides the essential functionality for:
</p>

<ul>
  <li><b>glTF 1.0 Parsing</b> - Complete specification compliance and validation</li>
  <li><b>Binary GLB Support</b> - Binary format parsing and data extraction</li>
  <li><b>Image Processing</b> - Image loading and format support</li>
  <li><b>Buffer Management</b> - Raw data storage and access</li>
  <li><b>Mathematical Utilities</b> - 3D mathematics and transformations</li>
  <li><b>Extension Support</b> - glTF 1.0 extension compatibility</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Complete glTF 1.0 Support</b> - Full specification compliance with all core components</li>
  <li><b>Binary GLB Format</b> - Comprehensive binary format support</li>
  <li><b>Image Processing</b> - Multi-format image loading (JPEG, PNG, BMP, GIF)</li>
  <li><b>Buffer Management</b> - Efficient buffer data handling and access</li>
  <li><b>Comprehensive Error Handling</b> - Detailed error types and reporting</li>
  <li><b>Mathematical Utilities</b> - 3D mathematics for transformations</li>
  <li><b>Extension Ecosystem</b> - Support for glTF 1.0 extensions</li>
  <li><b>Resource Loading</b> - External resource and URI handling</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- SUPPORTED EXTENSIONS -->
<h2 id="supported-extensions"> :puzzle_piece: Supported Extensions</h2>

<p>The following glTF 1.0 extensions are supported through feature flags:</p>

<h3>Core Extensions</h3>
<ul>
  <li><b>KHR_binary_glTF</b> - Binary buffer support (enabled by default)</li>
  <li><b>KHR_materials_common</b> - Common material types</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core glTF Components</h3>
<ul>
  <li><b>Document</b> - Main entry point for glTF file handling</li>
  <li><b>Buffer</b> - Raw data storage and management</li>
  <li><b>Accessor</b> - Buffer access and type information</li>
  <li><b>Animation</b> - Keyframe animations</li>
  <li><b>Camera</b> - Camera definitions and parameters</li>
  <li><b>Material</b> - Material properties and techniques</li>
  <li><b>Mesh</b> - Geometry data and primitives</li>
  <li><b>Node</b> - Scene graph nodes and transformations</li>
  <li><b>Scene</b> - Scene organization and hierarchy</li>
  <li><b>Skin</b> - Skeletal animations and bindings</li>
  <li><b>Texture</b> - Image and sampler definitions</li>
  <li><b>Light</b> - Light source definitions</li>
</ul>

<h3>Utilities and Support</h3>
<ul>
  <li><b>Math</b> - 3D mathematics utilities</li>
  <li><b>Binary Parsing</b> - GLB format support</li>
  <li><b>Image Loading</b> - Multi-format image support</li>
  <li><b>Error Handling</b> - Comprehensive error system</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
gltf-v1 = "0.3.0"

# Or for development from source:
gltf-v1 = { path = "../path/to/gltf-v1" }

# Enable specific extensions
[features]
default = ["KHR_binary_glTF"]
KHR_binary_glTF = []
KHR_materials_common = []
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic glTF 1.0 loading example:</p>

<pre><code>use gltf_v1::{
    Document,
    Gltf,
    import_buffers,
    import_images,
};

// Load a glTF file
let gltf = Gltf::open("model.gltf")?;
let document = Document::from_gltf(gltf)?;

// Import binary data
let buffers = import_buffers(&document, "model.bin")?;
let images = import_images(&document, "textures")?;

// Access scene data
println!("Document has {} scenes", document.scenes.len());
println!("Document has {} meshes", document.meshes.len());
</code></pre>

<p>Working with binary GLB files:</p>

<pre><code>use gltf_v1::{Document, Gltf};

// Load a binary GLB file
let gltf = Gltf::open("model.glb")?;
let document = Document::from_gltf(gltf)?;

// Access embedded binary data
for buffer in &document.buffers {
    println!("Buffer: {:?}", buffer);
}
</code></pre>

<p>Accessing mesh and material data:</p>

<pre><code>use gltf_v1::Document;

let document = Document::from_gltf(gltf)?;

// Access mesh data
for mesh in &document.meshes {
    println!("Mesh: {:?}", mesh.name);
    for primitive in &mesh.primitives {
        println!("  Primitive: {:?}", primitive);
    }
}

// Access material data
for material in &document.materials {
    println!("Material: {:?}", material.name);
    if let Some(technique) = &material.technique {
        println!("  Technique: {:?}", technique);
    }
}
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- BINARY SUPPORT -->
<h2 id="binary-support"> :floppy_disk: Binary Support</h2>

<p align="justify"> 
  The crate includes comprehensive support for binary GLB files, providing efficient parsing and data extraction capabilities for embedded binary data.
</p>

<h3>Binary Features</h3>
<ul>
  <li><b>Binary Chunk Parsing</b> - GLB format chunk extraction</li>
  <li><b>Buffer Data Extraction</b> - Embedded buffer data handling</li>
  <li><b>Image Data Handling</b> - Binary image data processing</li>
  <li><b>Base64 and URI Decoding</b> - Multiple data encoding support</li>
  <li><b>Binary Data Validation</b> - Format integrity checking</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ERROR HANDLING -->
<h2 id="error-handling"> :warning: Error Handling</h2>

<p align="justify"> 
  The crate provides a robust error handling system designed to provide detailed information about parsing and validation issues.
</p>

<h3>Error Types</h3>
<ul>
  <li><b>Detailed Error Types</b> - Specific error types for each operation</li>
  <li><b>Path-based Reporting</b> - Error location and context information</li>
  <li><b>Binary Format Validation</b> - GLB format integrity checking</li>
  <li><b>Resource Loading Errors</b> - External resource access issues</li>
  <li><b>Extension-specific Errors</b> - Extension validation and compatibility</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>gltf-v1-json</b> - JSON schema implementation</li>
  <li><b>image</b> - Image processing (JPEG, PNG, BMP, GIF)</li>
  <li><b>base64</b> - Base64 encoding/decoding</li>
  <li><b>byteorder</b> - Binary data handling</li>
  <li><b>urlencoding</b> - URI encoding/decoding</li>
  <li><b>indexmap</b> - Indexed hash map support</li>
</ul>

<h3>Feature Dependencies</h3>
<ul>
  <li><b>gltf-v1-json/KHR_binary_glTF</b> - Binary format support</li>
  <li><b>gltf-v1-json/KHR_materials_common</b> - Material extension support</li>
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
  <a href="https://crates.io/crates/gltf-v1">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/gltf-v1">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>
