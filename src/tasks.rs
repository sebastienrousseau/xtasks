// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Cargo XTask
//!
//! A collection of tasks to be executed with `cargo xtask`.
//!
//! ## Overview
//!
//! This module provides a comprehensive suite of tasks aimed at streamlining the development,
//! testing, and maintenance of Rust projects. It leverages `cargo xtask`, a convention for
//! creating and running custom cargo commands, enabling developers to extend Cargo's
//! capabilities and integrate additional tooling and workflows directly into their build process.
//!
//! ## Features
//!
//! - **Documentation Generation**: Automate the creation of project documentation, ensuring
//!   consistency and completeness across all codebase components.
//!
//! - **Continuous Integration (CI) Tasks**: Implement a variety of CI tasks to validate code
//!   quality, run tests, and ensure the stability of the codebase.
//!
//! - **Dependency Analysis**: Analyze project dependencies for potential issues, outdated
//!   libraries, and opportunities for optimization.
//!
//! - **Development Workflow Enhancement**: Streamline the development workflow with tasks
//!   designed to automate repetitive tasks and improve efficiency.
//!
//! - **Customization**: Easily extend and customize tasks to suit the unique requirements of
//!   your project.
//!
//! ## Usage
//!
//! To use these tasks, you will need to have `cargo xtask` installed. Once installed, you can
//! run tasks using the following command:
//!
//! ```sh
//! cargo xtask <task-name>
//! ```
//!
//! Replace `<task-name>` with the name of the task you wish to execute. Each task may have its
//! own set of arguments and options, which can be discovered by running:
//!
//! ```sh
//! cargo xtask <task-name> --help
//! ```
//!
//! ## Contributing
//!
//! Contributions to enhance existing tasks or add new tasks are welcome. Please ensure that all
//! new tasks are well-documented and include appropriate error handling to maintain the
//! robustness of the tooling.
//!
//! ## License
//!
//! This collection of cargo xtasks is distributed under the terms of both the MIT license and
//! the Apache License (Version 2.0). See LICENSE-APACHE and LICENSE-MIT for details.

#[cfg(feature = "cli")]
use clap;

use clap::{Arg, Command};
use anyhow::{Context, Result as AnyResult};
// use derive_builder::Builder;
use duct::cmd;
use crate::tasks::{
    bloat::{bloat_deps, bloat_time},
    ci::ci,
    coverage::coverage,
    docs::docs,
    powerset::powerset,
};
mod bloat;
mod ci;
mod coverage;
mod docs;
mod powerset;

///
/// Watch changes and after every change: `cargo check`, followed by `cargo test`
/// If `cargo check` fails, tests will not run.
///
/// # Errors
/// Errors if the command failed
///
pub fn dev() -> AnyResult<()> {
    cmd!("cargo", "watch", "-x", "check", "-x", "test").run()?;
    Ok(())
}

///
/// Instal cargo tools
///
/// # Errors
/// Errors if one of the commands failed
///
pub fn install() -> AnyResult<()> {
    cmd!("cargo", "install", "cargo-watch").run()?;
    cmd!("cargo", "install", "cargo-hack").run()?;
    cmd!("cargo", "install", "cargo-bloat").run()?;
    cmd!("rustup", "component", "add", "llvm-tools-preview").run()?;
    cmd!("cargo", "install", "grcov").run()?;
    Ok(())
}

/// Set up a main for your xtask. Uses clap.
/// To customize, look at this function's source and copy it to your
/// own xtask project.
///
/// # Errors
///
/// This function will return an error if any command failed
pub fn main() -> AnyResult<()> {
    let cli = Command::new("xtask")
        .subcommand(
            Command::new("coverage").arg(
                Arg::new("dev")
                    .short('d')
                    .long("dev")
                    .help("generate an html report"),
            ),
        )
        .subcommand(Command::new("vars"))
        .subcommand(Command::new("ci"))
        .subcommand(Command::new("powerset"))
        .subcommand(
            Command::new("bloat-deps").arg(
                Arg::new("package")
                    .short('p')
                    .long("package")
                    .help("package to build")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("bloat-time").arg(
                Arg::new("package")
                    .short('p')
                    .long("package")
                    .help("package to build")
                    .required(true),
            ),
        )
        .subcommand(Command::new("docs"));
    let matches = cli.get_matches();

    let root = crate::ops::root_dir();
    let res = match matches.subcommand() {
        Some(("vars", _)) => {
            println!("root: {root:?}");
            Ok(())
        }
        Some(("ci", _)) => crate::tasks::ci(),
        Some(("coverage", _)) => coverage(matches.contains_id("dev")),
        Some(("docs", _)) => docs(),
        Some(("powerset", _)) => crate::tasks::powerset(),
        Some(("bloat-deps", sm)) => crate::tasks::bloat_deps(
            sm.get_one::<String>("package")
                .context("please provide a package with -p")?,
        ),
        Some(("bloat-time", sm)) => crate::tasks::bloat_time(
            sm.get_one::<String>("package")
                .context("please provide a package with -p")?,
        ),
        _ => unreachable!("unreachable branch"),
    };
    res
}

