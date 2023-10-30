// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result as AnyResult};
use duct::cmd;
use std::process::Command;

/// Generates and watches documentation for the current project.
///
/// This function runs `cargo doc` in watch mode, automatically rebuilding
/// the documentation whenever changes are detected in the source code.
/// This is particularly useful during development.
///
/// # Examples
///
/// ```
/// # use anyhow::Result;
/// # fn run() -> Result<()> {
/// docs::docs()?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
///
/// # Errors
///
/// Returns an `anyhow::Error` if the `cargo watch` or `cargo doc` commands fail to execute.
/// The error will contain additional context about what went wrong to aid in debugging.
pub fn docs() -> AnyResult<()> {
    // Ensure that the necessary tools are installed
    ensure_cargo_watch_installed()?;

    // Execute the cargo watch command to build and watch the documentation
    cmd!("cargo", "watch", "-s", "cargo doc --no-deps")
        .run()
        .with_context(|| "Failed to execute 'cargo watch' for generating documentation.")?;

    Ok(())
}

/// Ensures that the `cargo-watch` tool is installed.
///
/// # Examples
///
/// ```
/// # use anyhow::Result;
/// # use docs::ensure_cargo_watch_installed;
/// # fn run() -> Result<()> {
/// ensure_cargo_watch_installed()?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
///
/// # Errors
///
/// Returns an `anyhow::Error` if the `cargo install cargo-watch` command fails to execute.
fn ensure_cargo_watch_installed() -> AnyResult<()> {
    let output = Command::new("cargo")
        .args(["install", "cargo-watch"])
        .output()
        .with_context(|| "Failed to install 'cargo-watch'.")?;

    if !output.status.success() {
        return Err(anyhow::Error::msg(
            "Failed to install 'cargo-watch'. Please ensure you have the necessary permissions.",
        ));
    }

    Ok(())
}
