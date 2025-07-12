<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> asset-importer-rs </h1>
<h3 align="center"> Assimp, but in Rust </h3>
<h5 align="center"> A safe, high-performance 3D asset import/export library </h5>

<p align="center"> 
  <img src="https://img.shields.io/badge/Version-0.1.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge" alt="Rust Version">
</p>

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :book: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project"> ➤ About The Project</a></li>
    <li><a href="#overview"> ➤ Overview</a></li>
    <li><a href="#project-structure"> ➤ Project Structure</a></li>
    <li><a href="#getting-started"> ➤ Getting Started</a></li>
    <li><a href="#supported-formats"> ➤ Supported Formats</a></li>
    <li><a href="#implementation-notes"> ➤ Implementation Notes</a></li>
    <li><a href="#key-differences"> ➤ Key Differences from Assimp</a></li>
    <li><a href="#roadmap"> ➤ Roadmap</a></li>
    <li><a href="#contributing"> ➤ Contributing</a></li>
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE PROJECT -->
<h2 id="about-the-project"> :pencil: About The Project</h2>

<p align="justify"> 
  asset-importer-rs is a Rust implementation of the popular Assimp library, providing safe and efficient 3D asset import and export functionality. Built with Rust's memory safety guarantees and performance characteristics, this library aims to be a modern, thread-safe alternative to the C++ Assimp library while maintaining compatibility with existing workflows.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- OVERVIEW -->
<h2 id="overview"> :cloud: Overview</h2>

<p align="justify"> 
  This project provides a comprehensive solution for loading, processing, and exporting 3D assets in various formats. The library is designed with a modular architecture, allowing for easy extension and customization. It supports both modern formats like glTF 2.0 and legacy formats like glTF 1.0, with plans to support additional formats including OBJ.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- PROJECT STRUCTURE -->
<h2 id="project-structure"> :floppy_disk: Project Structure</h2>

<h3>Core Crates</h3>
<ul>
  <li><b>asset-importer-rs-core</b> - Core import/export functionality and pipeline management</li>
  <li><b>asset-importer-rs-scene</b> - 3D scene data structures and types</li>
  <li><b>asset-importer-rs-gltf</b> - glTF 2.0 import/export support</li>
  <li><b>asset-importer-rs-gltf-v1</b> - glTF 1.0 import support (legacy)</li>
  <li><b>asset-importer-rs-obj</b> - OBJ format support (planned)</li>
</ul>

<h3>Supporting Crates</h3>
<ul>
  <li><b>gltf-v1</b> - glTF 1.0 specification implementation</li>
  <li><b>gltf-v1-json</b> - JSON serialization for glTF 1.0</li>
  <li><b>gltf-v1-derive</b> - Procedural macros for glTF 1.0 validation</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
asset-importer-rs-core = { path = "crates/asset-importer-rs-core" }
asset-importer-rs-scene = { path = "crates/asset-importer-rs-scene" }
asset-importer-rs-gltf = { path = "crates/asset-importer-rs-gltf" }

# For legacy glTF 1.0 support
asset-importer-rs-gltf-v1 = { path = "crates/asset-importer-rs-gltf-v1" }
</code></pre>

<p>Basic usage example:</p>

<pre><code>use asset_importer_rs_gltf::Gltf2Importer;

// Import a glTF file
let importer = Gltf2Importer::new();
let scene = importer.import_file("model.gltf")?;
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- SUPPORTED FORMATS -->
<h2 id="supported-formats"> :small_orange_diamond: Supported Formats</h2>

<h3>Currently Supported</h3>
<ul>
  <li><b>glTF 2.0</b> - Complete specification support with extensions</li>
  <li><b>glTF 1.0</b> - Legacy format support with KHR_materials_common</li>
</ul>

<h3>Planned Support</h3>
<ul>
  <li><b>OBJ</b> - Wavefront OBJ format</li>
  <li><b>FBX</b> - Autodesk FBX format</li>
  <li><b>3DS</b> - 3D Studio format</li>
  <li><b>DAE</b> - Collada format</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- IMPLEMENTATION NOTES -->
<h2 id="implementation-notes"> :small_orange_diamond: Implementation Notes</h2>

<h3>Implementation Plan 0.1.0</h3>

<p align="justify"> 
  The goal for version 0.1.0 is to produce a working rust-safe version of Assimp that provides GLTF and OBJ files formats. To minimize on potential unsafety, pointers, despite being obvious direct improvements to performance, will be eschewed in favor of base rust smart-types. The goal is to build a working model and slowly introduce unsafety for the sake of matching performance.
</p>

<p align="justify"> 
  As part of 0.1.0, a benchmark system should be set up to compare native assimp versus rust-assimp versions of the same command to begin to focus in on parity. Numbers should be reported and bounties assigned for performance improvements to particular regions.
</p>

<p align="justify"> 
  For the most part, there will be a default towards public external access, but ideally internals are overtime fully encapsulated.
</p>

<p align="justify"> 
  Specific Feature-Flag parity with Assimp will be considered as a future excursion with the exception of double precision as the means of testing feature flags.
</p>

<p align="justify"> 
  The active goal is to lay out the code and then optimize later. As part of this, a redocumentation phase will commence after 0.1.0 works to categorize a set of 'issues' and engage in providing documentation on the work.
</p>

<p align="justify"> 
  All documentation provided should have corresponding testing.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- KEY DIFFERENCES -->
<h2 id="key-differences"> :small_orange_diamond: Key Differences from Assimp</h2>

<p align="justify"> 
  This implementation differs from the original C++ Assimp library in several important ways:
</p>

<ul>
  <li><b>AiTexture</b> - Uses an enum for format rather than a text-based hint. This should reduce memory usage and result in higher stability in terms of options</li>
  <li><b>AiMaterial</b> - Encodes AiPropertyInfo differently in order to retain semantics, taking advantage of rust's enum system for types. This makes working with it easier</li>
  <li><b>AiNodes</b> - Are saved in an Arena instead of using a typical arena structure</li>
  <li><b>Memory Safety</b> - Leverages Rust's ownership system for automatic memory management</li>
  <li><b>Thread Safety</b> - Built with Rust's thread safety guarantees from the ground up</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ROADMAP -->
<h2 id="roadmap"> :small_orange_diamond: Roadmap</h2>

<h3>Version 0.1.0 (Current)</h3>
<ul>
  <li>✅ glTF 2.0 import/export support</li>
  <li>✅ glTF 1.0 import support</li>
  <li>🔄 OBJ format support</li>
  <li>🔄 Benchmark system setup</li>
  <li>🔄 Comprehensive testing suite</li>
</ul>

<h3>Future Versions</h3>
<ul>
  <li>Additional format support (FBX, 3DS, DAE)</li>
  <li>Performance optimizations</li>
  <li>Advanced post-processing features</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- CONTRIBUTING -->
<h2 id="contributing"> :small_orange_diamond: Contributing</h2>

<p align="justify"> 
  Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change. Check out the <a href="CONTRIBUTING.md">Contributing Guide</a> for more information.
</p>

<p>Please make sure to update tests as appropriate and ensure all documentation is up to date.</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- LICENSE -->
<h2 id="license"> :scroll: License</h2>

<p>This project is licensed under the MIT License - see the <a href="LICENSE">LICENSE</a> file for details.</p>

<p align="center"> 
  <strong>Copyright (c) 2024 Jackson Levitt</strong>
</p>

<p align="center">
  <a href="https://github.com/your-username/asset-importer-rs">
    <img src="https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white" alt="GitHub">
  </a>
  <a href="https://crates.io/crates/asset-importer-rs">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/asset-importer-rs">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>