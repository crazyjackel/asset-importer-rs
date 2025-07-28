<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> asset-importer-rs-obj </h1>
<h3 align="center"> OBJ file import functionality </h3>
<h5 align="center"> Wavefront OBJ format support with material handling </h5>

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
    <li><a href="#main-components"> ➤ Main Components</a></li>
    <li><a href="#getting-started"> ➤ Getting Started</a></li>
    <li><a href="#usage-examples"> ➤ Usage Examples</a></li>
    <li><a href="#architecture"> ➤ Architecture</a></li>
    <li><a href="#dependencies"> ➤ Dependencies</a></li>
    <li><a href="#implementation-notes"> ➤ Implementation Notes</a></li>
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE CRATE -->
<h2 id="about-the-crate"> :pencil: About The Crate</h2>

<p align="justify"> 
  asset-importer-rs-obj provides Wavefront OBJ file import functionality for the asset-importer-rs project. This implementation supports the OBJ format with material library (MTL) handling, making it a complete solution for importing OBJ assets into the asset-importer-rs ecosystem.
</p>

<p align="justify">
  This crate provides the essential functionality for:
</p>

<ul>
  <li><b>OBJ File Import</b> - Complete scene graph construction from OBJ files</li>
  <li><b>Material Library Support</b> - MTL file parsing and material handling</li>
  <li><b>Geometry Processing</b> - Vertex, normal, and texture coordinate handling</li>
  <li><b>Face Data</b> - Support for triangles, quads, and polygons</li>
  <li><b>Texture Support</b> - Material texture and image handling</li>
  <li><b>Multiple Objects</b> - Support for multiple objects within a single OBJ file</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Complete OBJ Support</b> - Full Wavefront OBJ specification compliance</li>
  <li><b>Material Library Integration</b> - MTL file parsing and material application</li>
  <li><b>Geometry Processing</b> - Vertex, normal, and texture coordinate handling</li>
  <li><b>Face Type Support</b> - Triangles, quads, and polygon face handling</li>
  <li><b>Texture Mapping</b> - Material texture and image support</li>
  <li><b>Multiple Objects</b> - Support for multiple objects in single files</li>
  <li><b>Error Handling</b> - Comprehensive error reporting and recovery</li>
  <li><b>Memory Efficient</b> - Optimized parsing and memory management</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core Import</h3>
<ul>
  <li><b>ObjImporter</b> - Main OBJ import functionality</li>
  <li><b>ObjImportError</b> - Import error handling and reporting</li>
</ul>

<h3>Import Pipeline</h3>
<ul>
  <li><b>File Parsing</b> - OBJ and MTL file parsing</li>
  <li><b>Geometry Processing</b> - Vertex, normal, and texture coordinate handling</li>
  <li><b>Material Loading</b> - MTL file parsing and material creation</li>
  <li><b>Face Construction</b> - Triangle, quad, and polygon face building</li>
  <li><b>Object Separation</b> - Multiple object handling within files</li>
  <li><b>Texture Management</b> - Material texture and image loading</li>
</ul>

<h3>Supported OBJ Elements</h3>
<ul>
  <li><b>Vertices (v)</b> - 3D vertex positions</li>
  <li><b>Texture Coordinates (vt)</b> - 2D texture coordinates</li>
  <li><b>Normals (vn)</b> - Vertex normal vectors</li>
  <li><b>Faces (f)</b> - Face definitions with vertex indices</li>
  <li><b>Objects (o)</b> - Object group definitions</li>
  <li><b>Material Libraries (mtllib)</b> - Material file references</li>
  <li><b>Material Applications (usemtl)</b> - Material assignments</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
asset-importer-rs-obj = "0.3.0"

# Or for development from source:
asset-importer-rs-obj = { path = "../path/to/asset-importer-rs-obj" }
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic OBJ import example:</p>

<pre><code>use asset_importer_rs_obj::{
    ObjImporter,
    ObjImportError,
};
use std::path::Path;

// Create an importer
let importer = ObjImporter::new();

// Import an OBJ file
let scene = importer.import_file(Path::new("model.obj"))?;

// Access scene data
println!("Scene has {} meshes", scene.meshes.len());
println!("Scene has {} materials", scene.materials.len());

// Access mesh data
for mesh in &scene.meshes {
    println!("Mesh has {} vertices", mesh.vertices.len());
    println!("Mesh has {} faces", mesh.faces.len());
}
</code></pre>

<p>Importing with material support:</p>

<pre><code>use asset_importer_rs_obj::ObjImporter;

// Create an importer
let importer = ObjImporter::new();

// Import OBJ with MTL file
let scene = importer.import_file("model_with_materials.obj")?;

// Access material data
for material in &scene.materials {
    println!("Material: {:?}", material.name);
    // Access material properties like diffuse, specular, etc.
}
</code></pre>

<p>Error handling example:</p>

<pre><code>use asset_importer_rs_obj::{ObjImporter, ObjImportError};

let importer = ObjImporter::new();

match importer.import_file("model.obj") {
    Ok(scene) => {
        println!("Successfully imported OBJ file");
        // Process scene data
    }
    Err(ObjImportError::FileNotFound) => {
        eprintln!("OBJ file not found");
    }
    Err(ObjImportError::ParseError(msg)) => {
        eprintln!("Parse error: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {:?}", e);
    }
}
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ARCHITECTURE -->
<h2 id="architecture"> :small_orange_diamond: Architecture</h2>

<p align="justify"> 
  The OBJ crate provides a complete implementation of the Wavefront OBJ format with a focus on efficient parsing and material handling. The importer handles file parsing, geometry processing, material loading, and scene construction, integrating seamlessly with the asset-importer-rs ecosystem.
</p>

<p align="justify">
  The crate uses the tObj library for initial parsing but provides optimized handling for the asset-importer-rs pipeline, ensuring efficient memory usage and proper integration with the broader 3D asset ecosystem.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>tobj</b> - Core OBJ parsing and validation</li>
  <li><b>asset-importer-rs-core</b> - Core import functionality</li>
  <li><b>asset-importer-rs-scene</b> - Scene data structures</li>
  <li><b>bytemuck</b> - Safe type casting</li>
  <li><b>enumflags2</b> - Flag-based enums</li>
  <li><b>image</b> - Image processing and format support</li>
</ul>

<h3>Standard Library Dependencies</h3>
<ul>
  <li><b>std::io</b> - File I/O operations</li>
  <li><b>std::path</b> - Path handling</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- IMPLEMENTATION NOTES -->
<h2 id="implementation-notes"> :warning: Implementation Notes</h2>

<p align="justify"> 
  <strong>Why tObj over obj-rs?</strong> tObj provides better separation of models and materials for easy parsing, whereas obj-rs keeps them together. This separation aligns better with the asset-importer-rs architecture.
</p>

<p align="justify">
  <strong>Future Improvements:</strong> While tObj serves as a good foundation, there are several optimizations that could be implemented with a custom parser:
</p>

<ul>
  <li><b>Direct Parsing</b> - Avoid converting parsed Points, Lines, Triangles, Quads, and Polygons into face_arities, instead let asset-importer-rs handle the conversion</li>
  <li><b>Memory Streaming</b> - Load one Model at a time, process it, then move on (losing Vec capacity benefits but gaining memory efficiency)</li>
  <li><b>Better Attribute Handling</b> - Improved handling of missing or rare attributes that tObj doesn't handle well</li>
</ul>

<p align="justify">
  In general, writing a custom parser would be more efficient for our use case, but getting things to work comes first. The current implementation provides a solid foundation for OBJ import functionality.
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
  <a href="https://crates.io/crates/asset-importer-rs-obj">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/asset-importer-rs-obj">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>