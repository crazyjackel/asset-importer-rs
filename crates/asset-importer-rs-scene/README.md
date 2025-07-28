<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> asset-importer-rs-scene </h1>
<h3 align="center"> 3D scene data structures and types </h3>
<h5 align="center"> Complete scene graph representation for 3D assets </h5>

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
  asset-importer-rs-scene provides comprehensive data structures and types for working with 3D scene data, inspired by the Assimp library. This crate is a core component of the asset-importer-rs project, offering a complete representation of 3D scenes including geometry, materials, animations, and more.
</p>

<p align="justify">
  This crate provides the essential data structures for:
</p>

<ul>
  <li><b>Scene Graph</b> - Complete node hierarchy and scene organization</li>
  <li><b>Geometry Data</b> - Mesh structures with vertices, faces, and bone weights</li>
  <li><b>Material System</b> - Material properties and texture mappings</li>
  <li><b>Animation Support</b> - Keyframe animations and morph targets</li>
  <li><b>Camera & Lighting</b> - Camera definitions and various light source types</li>
  <li><b>Mathematics</b> - Vector, matrix, and quaternion utilities</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Complete Scene Representation</b> - Full 3D scene graph with node hierarchy</li>
  <li><b>Rich Geometry Support</b> - Comprehensive mesh data including vertices, faces, and bone weights</li>
  <li><b>Material System</b> - Flexible material properties and texture handling</li>
  <li><b>Animation Framework</b> - Support for keyframe animations and morph targets</li>
  <li><b>Camera & Lighting</b> - Various camera types and light source definitions</li>
  <li><b>Mathematics Utilities</b> - Vector, matrix, and quaternion operations</li>
  <li><b>Color & Texture Support</b> - Color utilities and texture data handling</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Core Data Structures</h3>
<ul>
  <li><b>AiScene</b> - Core scene graph structure with node hierarchy</li>
  <li><b>AiNode</b> - Scene graph node with transformation and children</li>
  <li><b>AiMesh</b> - Geometry data including vertices, faces, and bone weights</li>
  <li><b>AiMaterial</b> - Material properties and texture mappings</li>
  <li><b>AiAnimation</b> - Keyframe animations and morph targets</li>
  <li><b>AiCamera</b> - Camera definitions and parameters</li>
  <li><b>AiLight</b> - Various light source types and properties</li>
  <li><b>AiTexture</b> - Texture data and format handling</li>
  <li><b>AiMetadata</b> - Custom metadata storage for scene objects</li>
</ul>

<h3>Animation & Keyframes</h3>
<ul>
  <li><b>AiNodeAnim</b> - Node animation with position, rotation, and scale keys</li>
  <li><b>AiMeshMorphAnim</b> - Morph target animations</li>
  <li><b>AiQuatKey/AiVectorKey</b> - Keyframe data for quaternions and vectors</li>
  <li><b>AiAnimInterpolation</b> - Interpolation modes for animations</li>
</ul>

<h3>Mathematics & Utilities</h3>
<ul>
  <li><b>AiVector2D/AiVector3D</b> - 2D and 3D vector operations</li>
  <li><b>AiMatrix4x4</b> - 4x4 matrix transformations</li>
  <li><b>AiQuaternion</b> - Quaternion rotations</li>
  <li><b>AiColor3D/AiColor4D</b> - Color representation</li>
</ul>

<h3>Mesh & Geometry</h3>
<ul>
  <li><b>AiBone</b> - Bone data for skeletal animations</li>
  <li><b>AiFace</b> - Face/primitive data</li>
  <li><b>AiVertexWeight</b> - Vertex weight data for bones</li>
  <li><b>AiAnimMesh</b> - Animated mesh data</li>
  <li><b>AiPrimitiveType</b> - Primitive type definitions</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
asset-importer-rs-scene = "0.3.0"

# Or for development from source:
asset-importer-rs-scene = { path = "../path/to/asset-importer-rs-scene" }
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic scene creation example:</p>

<pre><code>use asset_importer_rs_scene::{
    AiScene,
    AiMesh,
    AiMaterial,
    AiVector3D,
    AiMatrix4x4,
    AiNode,
};

// Create a new scene
let mut scene = AiScene::default();

// Add a mesh to the scene
let mesh = AiMesh::default();
scene.meshes.push(mesh);

// Create a material
let material = AiMaterial::default();
scene.materials.push(material);

// Create a node hierarchy
let root_node = AiNode::default();
scene.root_node = Some(Box::new(root_node));
</code></pre>

<p>Working with vectors and matrices:</p>

<pre><code>use asset_importer_rs_scene::{
    AiVector3D,
    AiMatrix4x4,
    AiQuaternion,
};

// Create vectors
let position = AiVector3D::new(1.0, 2.0, 3.0);
let normal = AiVector3D::new(0.0, 1.0, 0.0);

// Create transformation matrix
let transform = AiMatrix4x4::identity();

// Create quaternion rotation (w, x, y, z order)
let rotation = AiQuaternion::new(1.0, 0.0, 0.0, 0.0);
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ARCHITECTURE -->
<h2 id="architecture"> :small_orange_diamond: Architecture</h2>

<p align="justify"> 
  The scene crate provides a comprehensive data model for representing 3D scenes, materials, and animations. It serves as the foundation for all asset import/export operations, ensuring that 3D data can be consistently represented and manipulated across different file formats and applications.
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>bytemuck</b> - For safe type casting and memory operations</li>
  <li><b>enumflags2</b> - For flag-based enums and bit manipulation</li>
  <li><b>num_enum</b> - For numeric enums and conversions</li>
  <li><b>image</b> - For image processing and texture handling</li>
</ul>

<h3>Standard Library Dependencies</h3>
<ul>
  <li><b>std::collections</b> - For data structures and collections</li>
  <li><b>std::ops</b> - For mathematical operations</li>
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
  <a href="https://crates.io/crates/asset-importer-rs-scene">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/asset-importer-rs-scene">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>
