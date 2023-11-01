// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//!
//! # XTasks
//!
//! [![XTasks Logo](https://kura.pro/xtasks/images/banners/banner-xtasks.webp)][00]
//!
//! ## Essential tools and tasks for Rust projects using the xtask pattern, simplifying common build and development workflows.
//!
//! [![Crates.io](https://img.shields.io/crates/v/xtasks.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/xtasks "Crates.io")
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.19-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/xtasks "Lib.rs")
//! [![License](https://img.shields.io/crates/l/xtasks.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](https://opensource.org/license/apache-2-0/ "MIT or Apache License, Version 2.0")
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org "Rust")
//!
//! `xtasks` is a comprehensive Rust library designed to facilitate common operations and tasks
//! in projects that adhere to the `xtask` pattern. This pattern is prevalent in the Rust ecosystem,
//! where it is used to define custom build, test, and deployment scripts within a project’s workspace.
//!
//! ## Overview
//!
//! The library aims to simplify and standardize the process of handling frequent project-related tasks,
//! offering a range of functionalities from basic file operations to intricate continuous integration
//! workflows and documentation generation. It provides both low-level building blocks and high-level
//! abstractions to cater to different use cases and levels of complexity.
//!
//! ## Modules
//!
//! - **`ops`**: This module contains fundamental building block operations such as file manipulation,
//!   confirmation prompts, and command execution. It serves as the foundation for more complex tasks.
//!
//! - **`tasks`**: Here, you'll find higher-level functionalities and default implementations for common
//!   project tasks, streamlining processes like code coverage analysis, CI/CD workflows, and more.
//!
//! - **`macros`**: This module offers a collection of convenient macros designed to expedite common
//!   operations, reducing boilerplate and enhancing code readability.
//!
//! ## Features
//!
//! - **Simplicity**: Provides a user-friendly API, making it easy for developers to integrate `xtasks`
//!   into their projects and start automating their workflows.
//!
//! - **Extensibility**: Designed with extensibility in mind, allowing developers to easily add new tasks
//!   and customize existing ones to meet the unique needs of their projects.
//!
//! - **Robustness**: Incorporates thorough error handling and validation, ensuring that tasks are executed
//!   reliably and any issues are promptly reported.
//!
//! - **Integration**: Seamlessly integrates with existing Rust tooling and conventions, ensuring a smooth
//!   development experience.
//!
//! ## Usage
//!
//! To integrate `xtasks` into your project, add it as a dependency in your `Cargo.toml` file. Once added,
//! you can leverage the various functionalities and tasks provided by the library. Here’s an example on
//! how to use a function from the `ops` module:
//!
//! ```rust
//! use xtasks::ops::clean_files;
//!
//! # fn run() -> anyhow::Result<()> {
//! clean_files("target/debug/*.tmp")?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! ## Contributing
//!
//! Contributions to `xtasks` are warmly welcomed. Whether it’s enhancing existing features, fixing bugs,
//! or implementing new functionalities, your input is valuable. Please ensure that all contributions are
//! accompanied by comprehensive documentation and tests, adhering to the project’s coding standards.
//! Please see [CONTRIBUTING.md][03] for details on how to contribute.
//!
//! ## License
//!
//! `xtasks` is distributed under the terms of both the MIT license and the Apache License (Version 2.0),
//! providing flexibility and ensuring open-source readiness.
//!
//! - [Apache License, Version 2.0][04]
//! - [MIT license][05]
//!
//! Enjoy automating your workflows with `xtasks`!
//!
//! [00]: https://xtasks.pro "XTasks"
//! [01]: https://github.github.com/gfm/ "GitHub Flavoured Markdown"
//! [02]: https://www.rust-lang.org/ "Rust"
//! [03]: https://xtasks.pro/contribute/index.html "Contribute to XTasks"
//! [04]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
//! [05]: http://opensource.org/licenses/MIT "MIT license"

#![allow(clippy::must_use_candidate)]
#![deny(dead_code)]
#![deny(rustc::existing_doc_keyword)]
#![doc(
    html_favicon_url = "https://kura.pro/xtasks/images/favicon.ico",
    html_logo_url = "https://kura.pro/xtasks/images/logos/xtasks.webp",
    html_root_url = "https://docs.rs/xtasks"
)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(unreachable_pub)]
#![forbid(unsafe_code)]
#![crate_name = "xtasks"]
#![crate_type = "lib"]

/// The `loggers` module contains functions for logging.
pub mod loggers;
/// The `macros` module offers a collection of convenient macros designed to expedite common operations,
/// reducing boilerplate and enhancing code readability.
pub mod macros;
/// The `ops` module contains fundamental building block operations such as file manipulation,
/// confirmation prompts, and command execution. It serves as the foundation for more complex tasks.
pub mod ops;
/// The `tasks` module contains higher-level functionalities and default implementations for common
/// project tasks, streamlining processes like code coverage analysis, CI/CD workflows, and more.
pub mod tasks;
