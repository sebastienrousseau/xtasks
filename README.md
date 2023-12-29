<!-- markdownlint-disable MD033 MD041 -->

<img
  align="right"
  alt="Logo of XTasks"
  height="261"
  src="https://kura.pro/xtasks/images/logos/xtasks.webp"
  title="Logo of XTasks"
  width="261"
  />

<!-- markdownlint-enable MD033 MD041 -->

# XTasks

Essential tools and tasks for Rust projects using the xtask pattern, simplifying
common build and development workflows.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Banner of XTasks][banner]

[![Made With Rust][made-with-rust-badge]][14] [![Crates.io][crates-badge]][8] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][9] [![License][license-badge]][3] [![Codecov][codecov-badge]][15]

• [Website][1] • [Documentation][9] • [Report Bug][4] • [Request Feature][4] • [Contributing Guidelines][5]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Table of Contents

- [XTasks](#xtasks)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Features](#features)
    - [1. Enhanced File Operations (`ops.rs`)](#1-enhanced-file-operations-opsrs)
    - [2. XTasks Library Introduction (`lib.rs`)](#2-xtasks-library-introduction-librs)
    - [3. Convenient Macros (`macros.rs`)](#3-convenient-macros-macrosrs)
    - [4. Cargo XTask Integration (`tasks.rs`)](#4-cargo-xtask-integration-tasksrs)
    - [5. Binary Entry Point (`main.rs`)](#5-binary-entry-point-mainrs)
    - [6. Cargo Build Configuration (`powerset.rs`)](#6-cargo-build-configuration-powersetrs)
    - [7. Dependency Analysis (`bloat.rs`)](#7-dependency-analysis-bloatrs)
    - [8. CI Configuration Management (`ci.rs`)](#8-ci-configuration-management-cirs)
    - [9. Dynamic Documentation Generation (`docs.rs`)](#9-dynamic-documentation-generation-docsrs)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Usage](#usage)
  - [Running the Tests](#running-the-tests)
  - [Examples](#examples)
    - [File Operations Example](#file-operations-example)
    - [Macro Usage Example](#macro-usage-example)
    - [CI Configuration Example](#ci-configuration-example)
  - [Semantic Versioning Policy](#semantic-versioning-policy)
  - [License](#license)
  - [Contribution](#contribution)
  - [Acknowledgements](#acknowledgements)

![divider][divider]

## Overview

`XTasks` is a comprehensive Rust library designed to facilitate common
operations and tasks in projects that adhere to the xtask pattern. This pattern
is prevalent in the Rust ecosystem, where it is used to define custom build,
test, and deployment scripts within a project’s workspace.

## Features

### 1. Enhanced File Operations (`ops.rs`)

- **Filesystem Utilities**: Provides functions for common filesystem operations like file copying and removal.
- **Glob Pattern File Removal**: Supports removing files based on glob patterns, aiding in cleaning up temporary or generated files.

### 2. XTasks Library Introduction (`lib.rs`)

- Simplifies build and development workflows for Rust projects using the xtask pattern.

### 3. Convenient Macros (`macros.rs`)

- **Enhanced Print Macros**: Redefined `println!` and `print!` macros for formatted console output.
- **Assertion Macros**: Custom macros for asserting conditions in the code, enhancing readability and functionality.

### 4. Cargo XTask Integration (`tasks.rs`)

- **Streamlined Development Workflows**: Provides tasks for development, testing, and maintenance of Rust projects.
- **Documentation Automation**: Facilitates automatic generation of project documentation.
- **Continuous Integration Support**: Implements tasks for CI to ensure code quality and stability.

### 5. Binary Entry Point (`main.rs`)

- Central entry point for the `XTasks` binary, integrating various components of the project.

### 6. Cargo Build Configuration (`powerset.rs`)

- **Powerset Feature Management**: Manages combinations of features for cargo build runs, allowing depth specification for feature combinations.

### 7. Dependency Analysis (`bloat.rs`)

- Analyzes dependencies to identify their impact on build size, aiding in build optimization.

### 8. CI Configuration Management (`ci.rs`)

- **Flexible CI Settings**: Configures settings for CI runs, including compiler version choices and Clippy lint options.

### 9. Dynamic Documentation Generation (`docs.rs`)

- Automatically generates and updates documentation in response to source code changes.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You will need Rust and Cargo installed on your system. If you don't have them installed, you can install them from the official Rust website.

### Installation

To use `XTasks`, add it as a dependency to your `Cargo.toml` file:

```shell
[dependencies]
xtasks = "0.0.2"
```

Then, in your `lib.rs` or `main.rs` file, add:

```shell
extern crate xtasks;
```

## Usage

You can use `XTasks` to simplify and standardize the process of building,
testing, and deploying your Rust projects. For more detailed usage examples,
please refer to the library's documentation.

## Running the Tests

To run the tests, use the following command:

```shell
cargo test
```

## Examples

This section provides basic examples to demonstrate how to use some of the key features of XTasks in your Rust projects.

### File Operations Example

Using the enhanced file operations from `ops.rs`:

```rust
// Assuming you've included the `xtasks` crate
use xtasks::file_operations;

fn main() {
    // Copying a file
    file_operations::copy("path/to/source.file", "path/to/destination.file").unwrap();

    // Removing files based on a glob pattern
    file_operations::remove_files("path/to/temporary/*.tmp").unwrap();
}
```

### Macro Usage Example

Implementing the custom println! and assert! macros:

```rust
// Importing macros from xtasks
use xtasks::{println, assert};

fn main() {
    // Using the enhanced println! macro
    println!("This is a formatted {} message", "output");

    // Using a custom assert! macro
    assert!(2 + 2 == 4, "Math error: 2 + 2 should equal 4");
}
```

### CI Configuration Example

Configuring and running CI tasks with ci.rs:

```rust
use xtasks::ci::CI;

fn main() {
    let ci_config = CI::builder()
        .nightly(true)
        .enable_clippy_lints(true)
        .build()
        .unwrap();

    ci_config.run();
}
```

## Semantic Versioning Policy

For transparency into our release cycle and in striving to maintain backward compatibility, XTasks follows [semantic versioning][7].

## License

The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

- [Apache License, Version 2.0][2]
- [MIT license][3]

## Contribution

We welcome all people who want to contribute. Please see the [contributing instructions][5] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must adhere to the [Rust's Code of Conduct][16].

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements

A big thank you to all the awesome contributors of [xtasks][6] for their help and support.

A special thank you goes to the [Rust Reddit][13] community for providing a lot of useful suggestions on how to improve this project.

[1]: https://xtasks.pro "xtasks Website"
[2]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[3]: http://opensource.org/licenses/MIT "MIT license"
[4]: https://github.com/sebastienrousseau/xtasks/issues "Issues"
[5]: https://github.com/sebastienrousseau/xtasks/blob/main/CONTRIBUTING.md "Contributing"
[6]: https://github.com/sebastienrousseau/xtasks/graphs/contributors "Contributors"
[7]: http://semver.org/ "Semantic Versioning"
[8]: https://crates.io/crates/xtasks "Crate.io"
[9]: https://docs.rs/crate/xtasks/ "Docs.rs"
[10]: https://lib.rs/crates/xtasks "Lib.rs"
[11]: https://github.com/sebastienrousseau/xtasks/actions "Actions"
[12]: https://github.github.com/gfm/ "GitHub Flavoured Markdown"
[13]: https://www.reddit.com/r/rust/ "Rust Reddit"
[14]: https://www.rust-lang.org/learn/get-started "Rust"
[15]: https://codecov.io/github/sebastienrousseau/xtasks?branch=main "Codecov"
[16]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[17]: https://forge.rust-lang.org/release/platform-support.html "Rust Platform Support"

[banner]: https://kura.pro/xtasks/images/titles/title-xtasks.webp "Banner of xtasks"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/xtasks?style=for-the-badge&token=wAcpid8YEt 'Codecov'

[crates-badge]: https://img.shields.io/crates/v/xtasks.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/xtasks.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/xtasks.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
