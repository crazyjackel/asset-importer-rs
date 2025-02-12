# Contributing guide

Thank you for your interest in contributing to 'asset-importer-rs.' Your time and effort are truly valued. Do not hesitate to help — what is unknown to one is known among many. With your contribution, we aim to gather and share the knowledge of many.

## Contribution opportunities

 * Create or discuss issues for bugs, features, and ergonomics
 * Improve the crate documentation.
 * Add testings to match to the spec.
 * Participate in code reviews.
 * Submit PRs to fix acknowledged issues

## Code Guidelines

Code Guidelines are ever-evolving and difficult to ascertain, being for the most part discretionary. Here're some important details to give a good idea though on what direction we wish to take code:

 * We follow the [Rust API guidelines](https://github.com/rust-lang-nursery/api-guidelines).
 * Format code using rustfmt
 * Modules (refering to built-in asset importers and their respective code) should have a distinctive prefix appended to all files names.
 * Features Flags should be minimal and prefixed by respective module flags. They should be grouped together into minimal, default, and extra variants of the module.
 * test function names should begin with either 'test_' or 'external_'. External requires that the tester brings in extra exterior files. The name should be followed by the module name.

## Code of Conduct

Be genuine and be professional; The Project Maintainers commit to upholding Free of Opinion, but if you decide to express your opinion by cursing, insulting, or degrading other contributors then expect consequence.

If there is need for moderation regarding the project, both public or private, contact one of the Project Maintainers.