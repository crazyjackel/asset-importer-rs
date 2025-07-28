<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> gltf-v1-json </h1>
<h3 align="center"> glTF 1.0 JSON serialization and deserialization </h3>
<h5 align="center"> Complete glTF 1.0 JSON schema implementation with validation </h5>

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
    <li><a href="#validation"> ➤ Validation</a></li>
    <li><a href="#dependencies"> ➤ Dependencies</a></li>
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE CRATE -->
<h2 id="about-the-crate"> :pencil: About The Crate</h2>

<p align="justify"> 
  gltf-v1-json provides JSON serialization and deserialization for the glTF 1.0 specification. This crate is part of the asset-importer-rs project and implements the complete glTF 1.0 JSON schema with comprehensive validation support, enabling robust parsing and generation of glTF 1.0 files.
</p>

<p align="justify">
  This crate provides the essential functionality for:
</p>

<ul>
  <li><b>JSON Serialization</b> - Complete glTF 1.0 to JSON conversion</li>
  <li><b>JSON Deserialization</b> - JSON to glTF 1.0 structure parsing</li>
  <li><b>Schema Validation</b> - Comprehensive validation system</li>
  <li><b>Extension Support</b> - glTF 1.0 extension compatibility</li>
  <li><b>Error Reporting</b> - Path-based error reporting and validation</li>
  <li><b>Type Safety</b> - Strongly typed glTF 1.0 data structures</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Complete glTF 1.0 Schema</b> - Full JSON schema implementation</li>
  <li><b>Serde Integration</b> - Seamless serialization and deserialization</li>
  <li><b>Comprehensive Validation</b> - Automatic validation through derive macros</li>
  <li><b>Extension Support</b> - Feature flag-based extension compatibility</li>
  <li><b>Path-based Error Reporting</b> - Detailed error location and context</li>
  <li><b>Type Safety</b> - Strongly typed data structures</li>
  <li><b>Custom Validation Hooks</b> - Extensible validation system</li>
  <li><b>Indexed Collections</b> - Efficient hash map support with serialization</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- SUPPORTED EXTENSIONS -->
<h2 id="supported-extensions"> :puzzle_piece: Supported Extensions</h2>

<p>The following glTF 1.0 extensions are supported through feature flags:</p>

<h3>Core Extensions</h3>
<ul>
  <li><b>KHR_binary_glTF</b> - Binary buffer support</li>
  <li><b>KHR_materials_common</b> - Common material types</li>
</ul>

<h3>Feature Flags</h3>
<ul>
  <li><b>extensions</b> - Enable all extension support</li>
  <li><b>extras</b> - Enable extras field support</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core glTF Components</h3>
<ul>
  <li><b>Root</b> - Main glTF document structure</li>
  <li><b>Asset</b> - Version and metadata information</li>
  <li><b>Buffer</b> - Raw data storage</li>
  <li><b>Accessor</b> - Buffer access and type information</li>
  <li><b>Animation</b> - Keyframe animations</li>
  <li><b>Camera</b> - Camera definitions</li>
  <li><b>Material</b> - Material properties and techniques</li>
  <li><b>Mesh</b> - Geometry data</li>
  <li><b>Node</b> - Scene graph nodes</li>
  <li><b>Scene</b> - Scene organization</li>
  <li><b>Shader</b> - GLSL shader programs</li>
  <li><b>Skin</b> - Skeletal animations</li>
  <li><b>Texture</b> - Image and sampler definitions</li>
</ul>

<h3>Serialization Functions</h3>
<ul>
  <li><b>deserialize</b> - JSON to glTF structure conversion</li>
  <li><b>serialize</b> - glTF structure to JSON conversion</li>
  <li><b>from_str</b> - String-based deserialization</li>
  <li><b>to_string_pretty</b> - Pretty-printed JSON output</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
gltf-v1-json = "0.3.0"

# Or for development from source:
gltf-v1-json = { path = "../path/to/gltf-v1-json" }

# Enable specific extensions
[features]
default = []
extensions = []
extras = []
KHR_binary_glTF = []
KHR_materials_common = []
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic JSON deserialization example:</p>

<pre><code>use gltf_v1_json::{
    Root,
    deserialize,
    serialize,
};

// Deserialize from JSON string
let json_str = r#"{
    "asset": { "version": "1.0" },
    "scenes": [],
    "meshes": []
}"#;
let gltf: Root = deserialize::from_str(json_str)?;

// Access glTF data
println!("glTF version: {}", gltf.asset.version);
println!("Number of scenes: {}", gltf.scenes.len());
</code></pre>

<p>JSON serialization example:</p>

<pre><code>use gltf_v1_json::{Root, serialize};

// Create a glTF structure
let mut gltf = Root::default();
gltf.asset.version = "1.0".to_string();

// Serialize to JSON
let json = serialize::to_string_pretty(&gltf)?;
println!("Generated JSON:\n{}", json);
</code></pre>

<p>Working with extensions:</p>

<pre><code>use gltf_v1_json::{Root, deserialize};

// Deserialize with extension support
let json_with_extensions = r#"{
    "asset": { "version": "1.0" },
    "extensionsUsed": ["KHR_materials_common"],
    "materials": [{
        "name": "material1",
        "extensions": {
            "KHR_materials_common": {
                "technique": "BLINN",
                "values": {
                    "ambient": [0.1, 0.1, 0.1],
                    "diffuse": [0.8, 0.8, 0.8]
                }
            }
        }
    }]
}"#;

let gltf: Root = deserialize::from_str(json_with_extensions)?;
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- VALIDATION -->
<h2 id="validation"> :warning: Validation</h2>

<p align="justify"> 
  The crate includes a robust validation system designed to ensure glTF 1.0 specification compliance and provide detailed error reporting for debugging and development.
</p>

<h3>Validation Features</h3>
<ul>
  <li><b>Automatic Validation</b> - Validation through derive macros</li>
  <li><b>Path-based Error Reporting</b> - Detailed error location information</li>
  <li><b>Extension-specific Validation</b> - Extension compatibility checking</li>
  <li><b>Custom Validation Hooks</b> - Extensible validation system</li>
  <li><b>Schema Compliance</b> - glTF 1.0 specification validation</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>serde</b> - Serialization framework</li>
  <li><b>serde_json</b> - JSON serialization</li>
  <li><b>serde_derive</b> - Serialization derive macros</li>
  <li><b>indexmap</b> - Indexed hash map with serialization support</li>
  <li><b>gltf-v1-derive</b> - Validation derive macros</li>
</ul>

<h3>Feature Dependencies</h3>
<ul>
  <li><b>indexmap/serde</b> - Serialization support for indexed maps</li>
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
  <a href="https://crates.io/crates/gltf-v1-json">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/gltf-v1-json">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>
