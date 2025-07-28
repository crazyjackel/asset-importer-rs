<p align="center"> 
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</p>
<h1 align="center"> gltf-v1-derive </h1>
<h3 align="center"> Procedural macros for glTF 1.0 validation </h3>
<h5 align="center"> Derive macros for validation and serialization traits </h5>

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
    <li><a href="#implementation-details"> ➤ Implementation Details</a></li>
    <li><a href="#dependencies"> ➤ Dependencies</a></li>
    <li><a href="#internal-use"> ➤ Internal Use</a></li>
    <li><a href="#license"> ➤ License</a></li>
  </ol>
</details>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- ABOUT THE CRATE -->
<h2 id="about-the-crate"> :pencil: About The Crate</h2>

<p align="justify"> 
  gltf-v1-derive provides procedural macro support for glTF 1.0 validation and serialization. This crate is part of the asset-importer-rs project and provides the necessary derive macros for implementing validation and serialization traits, enabling automatic code generation for glTF 1.0 data structures.
</p>

<p align="justify">
  This crate provides the essential functionality for:
</p>

<ul>
  <li><b>Validation Derive</b> - Automatic implementation of validation traits</li>
  <li><b>Custom Validation Hooks</b> - Extensible validation through attributes</li>
  <li><b>Field Validation</b> - Automatic field validation generation</li>
  <li><b>Nested Structure Support</b> - Complex structure validation</li>
  <li><b>Path-based Error Reporting</b> - Detailed error location tracking</li>
  <li><b>Procedural Macro Support</b> - Rust code generation and parsing</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- FEATURES -->
<h2 id="features"> :cloud: Features</h2>

<ul>
  <li><b>Validate Derive Macro</b> - Automatic validation trait implementation</li>
  <li><b>Custom Validation Hooks</b> - Extensible validation through attributes</li>
  <li><b>Automatic Field Validation</b> - Generated validation code for each field</li>
  <li><b>Nested Structure Support</b> - Complex nested structure validation</li>
  <li><b>Path-based Error Reporting</b> - Detailed error location and context</li>
  <li><b>Procedural Macro Support</b> - Rust code parsing and generation</li>
  <li><b>String Case Conversion</b> - Automatic case conversion utilities</li>
  <li><b>Token Stream Generation</b> - Efficient code generation</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- MAIN COMPONENTS -->
<h2 id="main-components"> :floppy_disk: Main Components</h2>

<h3>Derive Macros</h3>
<ul>
  <li><b>Validate</b> - Main validation trait derive macro</li>
  <li><b>Custom Validation Hooks</b> - Attribute-based validation extension</li>
  <li><b>Field Validation</b> - Automatic field-level validation</li>
  <li><b>Nested Validation</b> - Complex structure validation support</li>
</ul>

<h3>Procedural Macro Features</h3>
<ul>
  <li><b>Code Parsing</b> - Rust code structure analysis</li>
  <li><b>Token Generation</b> - Efficient code generation</li>
  <li><b>Attribute Processing</b> - Custom attribute handling</li>
  <li><b>Error Reporting</b> - Compile-time error generation</li>
</ul>

<h3>Validation System</h3>
<ul>
  <li><b>Automatic Implementation</b> - Trait implementation generation</li>
  <li><b>Custom Hooks</b> - Extensible validation system</li>
  <li><b>Path Tracking</b> - Error location reporting</li>
  <li><b>Type Safety</b> - Compile-time validation checks</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- GETTING STARTED -->
<h2 id="getting-started"> :book: Getting Started</h2>

<p>Add the following to your <code>Cargo.toml</code>:</p>

<pre><code>[dependencies]
gltf-v1-derive = "0.3.0"

# Or for development from source:
gltf-v1-derive = { path = "../path/to/gltf-v1-derive" }
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- USAGE EXAMPLES -->
<h2 id="usage-examples"> :small_orange_diamond: Usage Examples</h2>

<p>Basic validation derive example:</p>

<pre><code>use gltf_v1_derive::Validate;

#[derive(Validate)]
struct MyStruct {
    field1: String,
    field2: i32,
}

// The Validate trait is automatically implemented
let my_struct = MyStruct {
    field1: "test".to_string(),
    field2: 42,
};

// Validation can be called automatically
my_struct.validate(&root, &path, &mut report)?;
</code></pre>

<p>Custom validation hook example:</p>

<pre><code>use gltf_v1_derive::Validate;

#[derive(Validate)]
#[gltf(validate_hook = "custom_validate")]
struct CustomStruct {
    field1: String,
    field2: i32,
}

fn custom_validate(
    this: &CustomStruct,
    root: &Root,
    path: impl Fn() -> Path,
    report: &mut impl FnMut(&dyn Fn() -> Path, Error),
) {
    // Custom validation logic
    if this.field2 < 0 {
        report(&path, Error::InvalidValue("field2 must be positive"));
    }
}
</code></pre>

<p>Nested structure validation:</p>

<pre><code>use gltf_v1_derive::Validate;

#[derive(Validate)]
struct ParentStruct {
    child: ChildStruct,
    name: String,
}

#[derive(Validate)]
struct ChildStruct {
    value: f32,
    #[gltf(validate_hook = "validate_range")]
    range: i32,
}

fn validate_range(
    this: &i32,
    _root: &Root,
    path: impl Fn() -> Path,
    report: &mut impl FnMut(&dyn Fn() -> Path, Error),
) {
    if *this < 0 || *this > 100 {
        report(&path, Error::InvalidValue("range must be between 0 and 100"));
    }
}
</code></pre>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- IMPLEMENTATION DETAILS -->
<h2 id="implementation-details"> :gear: Implementation Details</h2>

<p align="justify"> 
  The crate provides comprehensive procedural macro support for glTF 1.0 validation and serialization, enabling automatic code generation and trait implementation.
</p>

<h3>Validation Derive Features</h3>
<ul>
  <li><b>Automatic Trait Implementation</b> - Generates Validate trait implementations</li>
  <li><b>Field-level Validation</b> - Automatic validation code for each field</li>
  <li><b>Custom Validation Hooks</b> - Extensible validation through attributes</li>
  <li><b>Nested Structure Handling</b> - Complex structure validation support</li>
  <li><b>Path-based Error Reporting</b> - Detailed error location tracking</li>
</ul>

<h3>Procedural Macro Capabilities</h3>
<ul>
  <li><b>Rust Code Parsing</b> - AST analysis and structure understanding</li>
  <li><b>Token Stream Generation</b> - Efficient code generation</li>
  <li><b>Attribute Processing</b> - Custom attribute handling and validation</li>
  <li><b>Compile-time Validation</b> - Early error detection and reporting</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- DEPENDENCIES -->
<h2 id="dependencies"> :small_orange_diamond: Dependencies</h2>

<h3>Core Dependencies</h3>
<ul>
  <li><b>proc-macro2</b> - Procedural macro support</li>
  <li><b>quote</b> - Token stream generation</li>
  <li><b>syn</b> - Rust code parsing</li>
  <li><b>inflections</b> - String case conversion utilities</li>
</ul>

<h3>Procedural Macro Support</h3>
<ul>
  <li><b>proc-macro = true</b> - Enables procedural macro functionality</li>
</ul>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/cloudy.png)

<!-- INTERNAL USE -->
<h2 id="internal-use"> :warning: Internal Use</h2>

<p align="justify"> 
  This crate is primarily intended for internal use within the <code>asset-importer-rs</code> project, specifically for the glTF 1.0 implementation. It provides the necessary derive macros for implementing validation and serialization traits that are used throughout the glTF 1.0 ecosystem.
</p>

<p align="justify">
  While the crate can be used independently, it is designed and optimized for use within the asset-importer-rs workspace and may have dependencies on internal types and traits specific to the project.
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
  <a href="https://crates.io/crates/gltf-v1-derive">
    <img src="https://img.shields.io/badge/Crates.io-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io">
  </a>
  <a href="https://docs.rs/gltf-v1-derive">
    <img src="https://img.shields.io/badge/Docs.rs-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Docs.rs">
  </a>
</p>
