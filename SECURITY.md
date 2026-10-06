# Security Policy

## Security Philosophy

`asset-importer-rs` is designed to give downstream developers a safer way to process and convert 3D asset files.

A core goal of the project is to preserve Rust's memory-safety guarantees so that applications can confidently process untrusted assets on servers without relying on memory-unsafe parsers.

The project therefore treats memory safety, malformed input handling, and denial-of-service resistance as important security concerns.

## Supported Versions

Security fixes are generally applied to:

| Version | Supported |
| --- | --- |
| Latest release | ✅ |
| `main` | ✅ |
| Older releases | ❌ |

## Reporting a Vulnerability

Please do not open a public issue for security vulnerabilities.

Use GitHub's **Private Vulnerability Reporting** feature under the repository's **Security** tab.

When possible, include:

- A description of the issue
- Steps to reproduce it
- The affected file format or crate
- A minimal example file
- The expected security impact

## Security Scope

Relevant security issues include:

- Memory-safety violations
- Invalid or unchecked memory access
- Integer overflows affecting parsing or allocation
- Excessive memory allocation
- Excessive CPU usage
- Stack exhaustion or unbounded recursion
- Panics caused by malformed input
- Path traversal or unintended filesystem access
- Dependency vulnerabilities that affect asset processing

Malformed files should generally result in a recoverable error rather than a crash or memory-safety violation.

## Server-Side Use

A major use case for `asset-importer-rs` is server-side conversion of user-supplied assets.

Applications should still enforce reasonable limits on:

- File size
- Memory usage
- Parsing time
- Object and vertex counts
- Nesting depth

Memory safety does not eliminate denial-of-service risks from hostile input.

## Safe Harbor

Good-faith security research and responsible disclosure are welcome.

Please avoid disrupting systems, accessing data that is not yours, or publicly disclosing exploitable details before a reasonable remediation period has passed.