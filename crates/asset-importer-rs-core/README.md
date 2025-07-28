<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> asset-importer-rs-core </h1>
<h3 align="center"> Core functionality for 3D asset import/export </h3>
<h5 align="center"> The foundation of the asset-importer-rs ecosystem </h5>

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
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE CRATE -->
<h2 id="about-the-crate"> :pencil: About The Crate</h2>

<p align="justify"> 
  asset-importer-rs-core is the foundational crate of the asset-importer-rs project, providing the core functionality for importing and exporting 3D assets. Inspired by the Assimp library, this crate establishes the fundamental building blocks that all format-specific importers and exporters rely upon.
</p>

<p align="justify">
  This crate provides the essential infrastructure for:
</p>

<ul>
  <li><b>Import Pipeline</b> - Defines the AiImporter trait and related interfaces for loading 3D assets</li>
  <li><b>Export Pipeline</b> - Defines the AiExport trait and related interfaces for writing 3D assets</li>
  <li><b>Post-Processing</b> - Defines post-processing steps and operations for asset transformations</li>
  <li><b>Configuration</b> - Provides configuration constants for import/export behavior</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Flexible Asset System</b> - Configurable import/export pipelines for various 3D formats</li>
  <li><b>Post-Processing Capabilities</b> - Built-in support for asset transformations and optimizations</li>
  <li><b>Error Handling</b> - Comprehensive error handling and reporting system</li>
  <li><b>Format Support</b> - Extensible architecture supporting multiple file formats</li>
  <li><b>Configuration Management</b> - Fine-grained control over import/export operations</li>
  <li><b>Extensible Architecture</b> - Easy to extend with new format support</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core Modules</h3>
<ul>
  <li><b>Import System</b> - Defines the AiImporter trait for format-specific importers</li>
  <li><b>Export System</b> - Defines the AiExport trait for format-specific exporters</li>
  <li><b>Post-Processing</b> - Defines AiPostProcessSteps enum for asset transformations</li>
  <li><b>Configuration</b> - Provides configuration constants for import/export settings</li>
  <li><b>Importer Metadata</b> - AiImporterDesc for format-specific importer descriptions</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
asset-importer-rs-core = "0.2.0"

# Or for development from source:
asset-importer-rs-core = { path = "../path/to/asset-importer-rs-core" }
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic usage example:</p>

<pre><code>use asset_importer_rs_core::{
    AiImporter,
    AiImporterInfo
};
use std::path::Path;

// Define a custom importer
struct MyImporter;

impl AiImporterInfo for MyImporter {
    fn info(&self) -> asset_importer_rs_core::AiImporterDesc {
        // Implementation details...
        unimplemented!()
    }
}

impl AiImporter for MyImporter {
    type Error = AiReadError;
    
    fn can_read_dyn(&self, path: &Path, loader: &dyn Fn(&Path) -> std::io::Result<Box<dyn asset_importer_rs_core::ReadSeek>>) -> bool {
        // Check if this importer can handle the file
        unimplemented!()
    }
    
    fn read_file_dyn(&self, path: &Path, loader: &dyn Fn(&Path) -> std::io::Result<Box<dyn asset_importer_rs_core::ReadSeek>>) -> Result<asset_importer_rs_scene::AiScene, Self::Error> {
        // Import the file
        unimplemented!()
    }
}
</code></pre>

<p>Using post-processing steps:</p>

<pre><code>use asset_importer_rs_core::AiPostProcessSteps;
use enumflags2::BitFlags;

// Define post-processing operations
let post_process_steps = BitFlags::from(
    AiPostProcessSteps::Triangulate | 
    AiPostProcessSteps::GenNormals |
    AiPostProcessSteps::CalcTangentSpaces
);
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ARCHITECTURE -->
<h2 id="architecture"> :small_orange_diamond: Architecture</h2>

<p align="justify"> 
  The core crate provides the fundamental building blocks for asset import/export operations. It establishes the interfaces and abstractions that all format-specific implementations must follow, ensuring consistency and interoperability across the entire asset-importer-rs ecosystem.
</p>


![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>enumflags2</b> - For flag-based enums and bit manipulation in post-processing</li>
  <li><b>asset-importer-rs-scene</b> - For scene data structures and types</li>
</ul>

<h3>Standard Library Dependencies</h3>
<ul>
  <li><b>std::io</b> - For I/O operations and error handling</li>
  <li><b>std::path</b> - For path handling</li>
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
  <a href="https://crates.io/crates/asset-importer-rs-core">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/asset-importer-rs-core">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>
