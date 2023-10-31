// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result as AnyResult};
use duct::cmd;

use crate::macro_log_info;

/// Analyses the dependencies of the current project to find which ones contribute most to the build size.
///
/// # Parameters
///
/// * `package` - The name of the package to analyze.
///
/// # Errors
///
/// Returns an error if the `cargo bloat` command fails to execute. This could happen if the specified package
/// is not found, or if `cargo bloat` is not installed.
pub fn bloat_deps(package: &str) -> AnyResult<()> {
    macro_log_info!(LogLevel::INFO, "bloat_deps", "Starting dependency analysis", LogFormat::CLF);

    cmd!("cargo", "bloat", "-p", package, "--crates")
        .run()
        .map(|_| ())  // Convert Ok(Output) to Ok(())
        .map_err(|err| {
            // Log the error and then return it
            macro_log_info!(LogLevel::ERROR, "bloat_deps", "Dependency analysis failed", LogFormat::CLF);
            err
        })
        .with_context(|| format!("Failed to execute 'cargo bloat' for dependency analysis on package '{}'", package))?;

    macro_log_info!(LogLevel::INFO, "bloat_deps", "Dependency analysis completed", LogFormat::CLF);
    Ok(())
}

/// Analyses the build times of dependencies in the current project.
///
/// # Parameters
///
/// * `package` - The name of the package to analyze.
///
/// # Errors
///
/// Returns an error if the `cargo bloat` command fails to execute. This could be due to a variety of reasons,
/// such as the package not being found, or `cargo bloat` not being installed.
pub fn bloat_time(package: &str) -> AnyResult<()> {
    macro_log_info!(LogLevel::INFO, "bloat_time", "Starting build time analysis", LogFormat::CLF);

    cmd!("cargo", "bloat", "-p", package, "--time")
        .run()
        .map(|_| ())  // Convert Ok(Output) to Ok(())
        .map_err(|err| {
            // Log the error and then return it
            macro_log_info!(LogLevel::ERROR, "bloat_time", "Build time analysis failed", LogFormat::CLF);
            err
        })
        .with_context(|| format!("Failed to execute 'cargo bloat' for build time analysis on package '{}'", package))?;

    macro_log_info!(LogLevel::INFO, "bloat_time", "Build time analysis completed", LogFormat::CLF);
    Ok(())
}
