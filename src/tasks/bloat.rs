// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result as AnyResult};
use duct::cmd;

use dtt::DateTime;
use rlg::{macro_log, LogFormat, LogLevel};
use vrd::Random;

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
pub fn deps(package: &str) -> AnyResult<()> {
    let date = DateTime::new();
    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::INFO,
        "Dependencies",
        "Starting dependency analysis",
        &LogFormat::CLF
    );
    drop(log);

    cmd!("cargo", "bloat", "-p", package, "--crates")
        .run()
        .map(|_| ())  // Convert Ok(Output) to Ok(())
        .map_err(|err| {
            // Log the error and then return it
            let log = macro_log!(
                &Random::default().int(0, 1_000_000_000).to_string(),
                &date.iso_8601,
                &LogLevel::ERROR,
                "Dependencies",
                "Dependency analysis failed",
                &LogFormat::CLF);
            drop(log);
            err
        })
        .with_context(|| format!("Failed to execute 'cargo bloat' for dependency analysis on package '{package}'"))?;
    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::ERROR,
        "Dependencies",
        "Dependency analysis completed",
        &LogFormat::CLF
    );
    drop(log);
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
pub fn time(package: &str) -> AnyResult<()> {
    let date = DateTime::new();
    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::ERROR,
        "Time Analysis",
        "Starting build time analysis",
        &LogFormat::CLF
    );
    drop(log);

    cmd!("cargo", "bloat", "-p", package, "--time")
        .run()
        .map(|_| ())  // Convert Ok(Output) to Ok(())
        .map_err(|err| {
            // Log the error and then return it
            let log = macro_log!(
                &Random::default().int(0, 1_000_000_000).to_string(),
                &date.iso_8601,
                &LogLevel::ERROR,
                "Time Analysis",
                "Build time analysis failed",
                &LogFormat::CLF);
            drop(log);
            err
        })
        .with_context(|| format!("Failed to execute 'cargo bloat' for build time analysis on package '{package}'"))?;
    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::ERROR,
        "Time Analysis",
        "Build time analysis completed",
        &LogFormat::CLF
    );
    drop(log);
    Ok(())
}
