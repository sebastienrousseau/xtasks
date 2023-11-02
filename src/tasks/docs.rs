// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{run_command, run_std_command, tasks::cmd};
use anyhow::{Context, Result as AnyResult};
use std::process::Command;

/// Generates and watches documentation for the current project.
///
/// This function runs `cargo doc` in watch mode, automatically rebuilding
/// the documentation whenever changes are detected in the source code.
/// This is particularly useful during development.
///
/// # Errors
///
/// Returns an `anyhow::Error` if the `cargo watch` or `cargo doc` commands fail to execute.
/// The error will contain additional context about what went wrong to aid in debugging.
pub fn docs() -> AnyResult<()> {
    // Ensure that the necessary tools are installed
    ensure_cargo_watch_installed()?;

    // Execute the cargo watch command to build and watch the documentation
    run_command!(
        cmd!("cargo", "watch", "-s", "cargo doc --no-deps"),
        "Failed to execute 'cargo watch' for generating documentation"
    );

    Ok(())
}

/// Ensures that the `cargo-watch` tool is installed.
///
/// # Errors
///
/// Returns an `anyhow::Error` if the `cargo install cargo-watch` command fails to execute.
pub fn ensure_cargo_watch_installed() -> AnyResult<()> {
    run_std_command!(
        Command::new("cargo").args(["install", "cargo-watch"]),
        "Failed to install 'cargo-watch'"
    );
    Ok(())
}
