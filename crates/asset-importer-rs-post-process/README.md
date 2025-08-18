<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> asset-importer-rs-post-process </h1>
<h3 align="center"> Post-processing module for 3D asset transformations </h3>
<h5 align="center"> Comprehensive asset optimization and validation pipeline </h5>

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
    <li><a href="#post-processing-steps"> ➤ Post-Processing Steps</a></li>
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
  asset-importer-rs-post-process provides a comprehensive suite of post-processing steps for 3D assets after import. This module implements various transformations, optimizations, and validations that can be applied to 3D data to improve performance, compatibility, and quality.
</p>

<p align="justify">
  This crate provides the essential functionality for:
</p>

<ul>
  <li><b>Asset Optimization</b> - Mesh optimization, cache locality improvements, and performance enhancements</li>
  <li><b>Data Validation</b> - Structure validation, degenerate detection, and data integrity checks</li>
  <li><b>Geometry Processing</b> - Triangulation, normal generation, and vertex optimization</li>
  <li><b>Material Processing</b> - Material optimization and texture embedding</li>
  <li><b>Coordinate System</b> - Handedness conversion and coordinate transformations</li>
  <li><b>Animation Support</b> - Bone weight limiting and armature data population</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Feature Flag System</b> - Configurable post-processing steps to reduce compile time and binary size</li>
  <li><b>Comprehensive Coverage</b> - 32 different post-processing steps covering all major asset transformations</li>
  <li><b>Flexible Architecture</b> - Individual step selection or complete feature sets</li>
  <li><b>Performance Optimized</b> - Only compile the steps you actually need</li>
  <li><b>Extensible Design</b> - Easy to add new post-processing steps</li>
  <li><b>Error Handling</b> - Comprehensive error handling for each processing step</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- POST-PROCESSING STEPS -->
<h2 id="post-processing-steps"> :gear: Post-Processing Steps</h2>

<h3>Default Features</h3>
<p>The following post-processing steps are enabled by default:</p>

<ul>
  <li><b>calc-tangent-spaces</b> - Calculate tangent spaces for meshes</li>
  <li><b>join-identical-vertices</b> - Join identical vertices in meshes</li>
  <li><b>triangulate</b> - Triangulate all meshes</li>
  <li><b>gen-normals</b> - Generate normals for meshes</li>
  <li><b>gen-smooth-normals</b> - Generate smooth normals for meshes</li>
  <li><b>validate-data-structure</b> - Validate data structure</li>
  <li><b>remove-redundant-materials</b> - Remove redundant materials</li>
  <li><b>find-degenerates</b> - Find degenerate triangles</li>
  <li><b>find-invalid-data</b> - Find invalid data</li>
  <li><b>optimize-meshes</b> - Optimize meshes</li>
  <li><b>optimize-graph</b> - Optimize scene graph</li>
</ul>

<h3>All Features</h3>
<p>To enable all post-processing steps, use the <code>all</code> feature:</p>

<pre><code>[dependencies]
asset-importer-rs-post-process = { version = "0.3.0", features = ["all"] }
</code></pre>

<p>This includes all 32 post-processing steps:</p>

<h4>Geometry Processing</h4>
<ul>
  <li><b>make-left-handed</b> - Make meshes left-handed</li>
  <li><b>split-large-meshes</b> - Split large meshes</li>
  <li><b>pre-transform-vertices</b> - Pre-transform vertices</li>
  <li><b>flip-winding-order</b> - Flip winding order</li>
  <li><b>split-by-bone-count</b> - Split by bone count</li>
</ul>

<h4>Material and Texture Processing</h4>
<ul>
  <li><b>remove-component</b> - Remove specific components</li>
  <li><b>embed-textures</b> - Embed textures</li>
  <li><b>gen-uv-coords</b> - Generate UV coordinates</li>
  <li><b>transform-uv-coords</b> - Transform UV coordinates</li>
  <li><b>flip-uvs</b> - Flip UVs</li>
</ul>

<h4>Animation and Skeletal Processing</h4>
<ul>
  <li><b>limit-bone-weights</b> - Limit bone weights</li>
  <li><b>populate-armature-data</b> - Populate armature data</li>
  <li><b>debone</b> - Debone meshes</li>
</ul>

<h4>Optimization and Performance</h4>
<ul>
  <li><b>improve-cache-locality</b> - Improve cache locality</li>
  <li><b>sort-by-p-type</b> - Sort by primitive type</li>
  <li><b>find-instances</b> - Find instances</li>
  <li><b>global-scale</b> - Apply global scale</li>
</ul>

<h4>Normal Processing</h4>
<ul>
  <li><b>fix-infacing-normals</b> - Fix infacing normals</li>
  <li><b>force-gen-normals</b> - Force generate normals</li>
  <li><b>drop-normals</b> - Drop normals</li>
</ul>

<h4>Utility Features</h4>
<ul>
  <li><b>gen-bounding-boxes</b> - Generate bounding boxes</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core Architecture</h3>
<ul>
  <li><b>AiPostProcess Trait</b> - Defines the interface for all post-processing steps</li>
  <li><b>AiPostProcessSteps Enum</b> - Bitflags enum for step selection and combination</li>
  <li><b>Step Implementations</b> - Individual implementations for each processing step</li>
  <li><b>Feature System</b> - Conditional compilation based on feature flags</li>
</ul>

<h3>Module Structure</h3>
<ul>
  <li><b>steps/</b> - Directory containing all post-processing step implementations</li>
  <li><b>mod.rs</b> - Module declarations and re-exports with feature flags</li>
  <li><b>Individual Step Files</b> - One file per post-processing step for maintainability</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<h3>Default Features</h3>
<pre><code>[dependencies]
asset-importer-rs-post-process = "0.3.0"

# Or for development from source:
asset-importer-rs-post-process = { path = "../path/to/asset-importer-rs-post-process" }
</code></pre>

<h3>All Features</h3>
<pre><code>[dependencies]
asset-importer-rs-post-process = { version = "0.3.0", features = ["all"] }
</code></pre>

<h3>Custom Feature Selection</h3>
<pre><code>[dependencies]
asset-importer-rs-post-process = { version = "0.3.0", default-features = false, features = ["triangulate", "gen-normals"] }
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<h3>Basic Usage (Default Features)</h3>

<pre><code>use asset_importer_rs_post_process::{
    AiPostProcess, AiPostProcessSteps, CalcTangentSpaces, Triangulate
};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

// Create post-processing steps
let mut calc_tangents = CalcTangentSpaces;
let mut triangulate = Triangulate;

// Define which steps to apply
let steps = AiPostProcessSteps::CalcTangentSpaces | AiPostProcessSteps::Triangulate;

// Apply post-processing to a scene
let mut scene = AiScene::new();

if calc_tangents.prepare(steps) {
    calc_tangents.process(&mut scene)?;
}

if triangulate.prepare(steps) {
    triangulate.process(&mut scene)?;
}
</code></pre>

<h3>Using All Features</h3>

<pre><code>use asset_importer_rs_post_process::{
    AiPostProcess, AiPostProcessSteps, 
    CalcTangentSpaces, Triangulate, GenNormals, OptimizeMeshes,
    MakeLeftHanded, FlipUVs, GlobalScale
};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

// All post-processing steps are available with the "all" feature
let mut calc_tangents = CalcTangentSpaces;
let mut triangulate = Triangulate;
let mut gen_normals = GenNormals;
let mut optimize_meshes = OptimizeMeshes;
let mut make_left_handed = MakeLeftHanded;
let mut flip_uvs = FlipUVs;
let mut global_scale = GlobalScale;

// Define which steps to apply
let steps = AiPostProcessSteps::CalcTangentSpaces 
    | AiPostProcessSteps::Triangulate 
    | AiPostProcessSteps::GenNormals
    | AiPostProcessSteps::OptimizeMeshes
    | AiPostProcessSteps::MakeLeftHanded
    | AiPostProcessSteps::FlipUVs
    | AiPostProcessSteps::GlobalScale;

let mut scene = AiScene::new();

// Apply all enabled steps
if calc_tangents.prepare(steps) {
    calc_tangents.process(&mut scene)?;
}
if triangulate.prepare(steps) {
    triangulate.process(&mut scene)?;
}
if gen_normals.prepare(steps) {
    gen_normals.process(&mut scene)?;
}
if optimize_meshes.prepare(steps) {
    optimize_meshes.process(&mut scene)?;
}
if make_left_handed.prepare(steps) {
    make_left_handed.process(&mut scene)?;
}
if flip_uvs.prepare(steps) {
    flip_uvs.process(&mut scene)?;
}
if global_scale.prepare(steps) {
    global_scale.process(&mut scene)?;
}
</code></pre>

<h3>Custom Feature Groups</h3>

<pre><code>// In your Cargo.toml
[dependencies]
asset-importer-rs-post-process = { version = "0.3.0", features = ["optimization"] }

[features]
optimization = [
    "asset-importer-rs-post-process/optimize-meshes",
    "asset-importer-rs-post-process/optimize-graph",
    "asset-importer-rs-post-process/improve-cache-locality"
]
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ARCHITECTURE -->
<h2 id="architecture"> :building_construction: Architecture</h2>

<h3>Feature Flag System</h3>
<p>The post-process crate uses a sophisticated feature flag system that allows users to include only the post-processing steps they need. This provides several benefits:</p>

<ul>
  <li><b>Compile Time Optimization</b> - Only compile the steps you actually use</li>
  <li><b>Binary Size Reduction</b> - Smaller binaries when only specific features are enabled</li>
  <li><b>Flexibility</b> - Choose exactly which post-processing capabilities you want</li>
  <li><b>Default Features</b> - Sensible defaults for common use cases</li>
</ul>

<h3>Step Implementation Pattern</h3>
<p>Each post-processing step follows a consistent pattern:</p>

<pre><code>pub struct StepName;

impl AiPostProcess for StepName {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::StepName)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // Implementation here
        Ok(())
    }
}
</code></pre>

<h3>Module Organization</h3>
<p>The crate is organized into individual files for each post-processing step, making it easy to:</p>

<ul>
  <li><b>Maintain</b> - Each step is isolated and can be modified independently</li>
  <li><b>Extend</b> - New steps can be added by creating new files</li>
  <li><b>Test</b> - Individual steps can be tested in isolation</li>
  <li><b>Document</b> - Each step has its own documentation and examples</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :link: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>asset-importer-rs-core</b> - Core traits and types for the post-processing system</li>
  <li><b>asset-importer-rs-scene</b> - Scene data structures and manipulation</li>
  <li><b>enumflags2</b> - Bit flags support for step selection and combination</li>
</ul>

<h3>Optional Dependencies</h3>
<p>Additional dependencies may be required depending on which features are enabled. The feature flag system ensures that only necessary dependencies are included.</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- LICENSE -->
<h2 id="license"> :scroll: License</h2>

<p>This project is licensed under either of</p>

<ul>
  <li>Apache License, Version 2.0, (<a href="LICENSE-APACHE">LICENSE-APACHE</a> or <a href="https://www.apache.org/licenses/LICENSE-2.0">https://www.apache.org/licenses/LICENSE-2.0</a>)</li>
  <li>MIT license (<a href="LICENSE-MIT">LICENSE-MIT</a> or <a href="https://opensource.org/licenses/MIT">https://opensource.org/licenses/MIT</a>)</li>
</ul>

<p>at your option.</p> 