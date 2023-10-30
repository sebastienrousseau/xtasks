// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//!
//! # XTasks
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
//! use xtaskops::ops::clean_files;
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
//!
//! ## License
//!
//! `xtasks` is distributed under the terms of both the MIT license and the Apache License (Version 2.0),
//! providing flexibility and ensuring open-source readiness. For more details, refer to the LICENSE-APACHE
//! and LICENSE-MIT files in the project repository.
//!
//! Enjoy automating your workflows with `xtasks`!
//!

#![allow(clippy::must_use_candidate)]
#![warn(missing_docs)]

pub mod macros;
pub mod ops;
pub mod tasks;

