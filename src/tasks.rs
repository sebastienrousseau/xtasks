// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Cargo `XTask`
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

use crate::tasks::{
    bloat::{deps, time},
    ci::ci,
    coverage::coverage,
    docs::docs,
    powerset::powerset,
};
use anyhow::{Context, Result as AnyResult};
use clap::{Arg, Command};
use duct::cmd;
use std::env;

/// Analyses the dependencies of the current project to find which ones contribute most to the build size.
pub mod bloat;

/// Implements a variety of CI tasks to validate code quality, run tests, and ensure the stability of the codebase.
pub mod ci;

/// Automate the creation of project documentation, ensuring consistency and completeness across all codebase components.
pub mod coverage;

/// Streamline the development workflow with tasks designed to automate repetitive tasks and improve efficiency.
pub mod docs;

/// Easily extend and customize tasks to suit the unique requirements of your project.
pub mod powerset;

/// Runs a specified command with `watch`, `-x check`, and `-x test` arguments.
///
/// This function is intended to be used for development purposes, enabling live
/// reloading and automatic execution of checks and tests upon code changes.
///
/// # Arguments
///
/// * `command`: The command to run, typically "cargo".
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the command runs successfully, or an `Err` variant
///   encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the external command fails to run, or if any other
/// error occurs during execution.
pub fn dev_with_command(command: &str) -> AnyResult<()> {
    cmd!(command, "watch", "-x", "check", "-x", "test").run()?;
    Ok(())
}

/// Convenience function to run the `cargo watch` command with `-x check` and `-x test` arguments.
///
/// This function is a wrapper around `dev_with_command`, providing a simpler interface for the
/// common use case of running `cargo watch`.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the command runs successfully, or an `Err` variant
///   encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the `cargo watch` command fails to run, or if any other
/// error occurs during execution.
pub fn dev() -> AnyResult<()> {
    dev_with_command("cargo")
}

/// Installs various cargo tools and Rust components required for development.
///
/// This function executes a series of commands to install `cargo-watch`, `cargo-hack`,
/// `cargo-bloat`, and `grcov`. It also adds the `llvm-tools-preview` component via `rustup`.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if all commands run successfully, or an `Err` variant
///   encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if any of the installation commands fail to run,
/// or if any other error occurs during execution.
pub fn install() -> AnyResult<()> {
    cmd!("cargo", "install", "cargo-watch").run()?;
    cmd!("cargo", "install", "cargo-hack").run()?;
    cmd!("cargo", "install", "cargo-bloat").run()?;
    cmd!("rustup", "component", "add", "llvm-tools-preview").run()?;
    cmd!("cargo", "install", "grcov").run()?;
    Ok(())
}

/// Sets up the main command-line interface for your xtask project and executes
/// the specified subcommands.
///
/// This function configures and executes various subcommands using `clap`. The available subcommands
/// include `coverage`, `vars`, `ci`, `powerset`, `bloat-deps`, `bloat-time`, and `docs`.
///
/// # Arguments
///
/// * `args`: A slice of strings representing the command-line arguments.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the executed subcommand (if any) runs successfully,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if:
/// - Any subcommand fails to run.
/// - Required arguments for a subcommand are missing.
/// - There is a problem in setting up or executing the command-line interface.
pub fn main_with_args(args: &[String]) -> AnyResult<()> {
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
    let matches = cli.get_matches_from(args);
    println!("Received subcommand: {:?}", matches.subcommand());

    let res = match matches.subcommand() {
        Some(("vars", _)) => {
            let root = crate::ops::root_dir();
            println!("root: {root:?}");
            Ok(())
        }
        Some(("ci", _)) | None => crate::tasks::ci(),
        Some(("coverage", matches)) => {
            coverage(matches.contains_id("dev"))
        }
        Some(("docs", _)) => docs(),
        Some(("powerset", _)) => powerset(),
        Some(("bloat-deps", sm)) => deps(
            sm.get_one::<String>("package")
                .context("please provide a package with -p")?,
        ),
        Some(("bloat-time", sm)) => time(
            sm.get_one::<String>("package")
                .context("please provide a package with -p")?,
        ),
        _ => {
            eprintln!("Error: Unrecognized subcommand");
            Err(anyhow::Error::msg("Unrecognized subcommand"))
        }
    };
    res
}

/// The main entry point of the application.
///
/// This function collects command-line arguments and passes them to `main_with_args` for
/// further processing and execution of the appropriate subcommands.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the application runs successfully,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will propagate any errors returned by `main_with_args`.
pub fn main() -> AnyResult<()> {
    let args: Vec<String> = env::args().collect();
    main_with_args(&args)
}
