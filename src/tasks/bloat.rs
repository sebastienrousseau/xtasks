// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result as AnyResult};
use duct::cmd;

use dtt::DateTime;
// rlg 0.0.11 removed the `macro_log!` macro and moved LogFormat and
// LogLevel out of the crate root. `Log::build` is the replacement, and
// it assigns its own session id, so vrd is no longer needed here.
use rlg::log::Log;
use rlg::log_format::LogFormat;
use rlg::log_level::LogLevel;

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
    // dtt 0.0.11 replaced the `iso_8601` field with `format_rfc3339()`,
    // which is fallible. Format once per call and reuse.
    let timestamp = date.format_rfc3339().unwrap_or_default();
    let log = Log {
        format: LogFormat::CLF,
        ..Log::build(LogLevel::INFO, "Starting dependency analysis")
            .time(&timestamp)
            .component("Dependencies")
    };
    drop(log);

    cmd!("cargo", "bloat", "-p", package, "--crates")
        .run()
        .map(|_| ())
        .map_err(|err| {
            // Log the error and then return it
            let log = Log {
            format: LogFormat::CLF,
            ..Log::build(LogLevel::ERROR, "Dependency analysis failed")
                .time(&timestamp)
                .component("Dependencies")
        };
            drop(log);
            err
        })
        .with_context(|| format!("Failed to execute 'cargo bloat' for dependency analysis on package '{package}'"))?;
    let log = Log {
        format: LogFormat::CLF,
        ..Log::build(LogLevel::ERROR, "Dependency analysis completed")
            .time(&timestamp)
            .component("Dependencies")
    };
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
    // dtt 0.0.11 replaced the `iso_8601` field with `format_rfc3339()`,
    // which is fallible. Format once per call and reuse.
    let timestamp = date.format_rfc3339().unwrap_or_default();
    let log = Log {
        format: LogFormat::CLF,
        ..Log::build(LogLevel::ERROR, "Starting build time analysis")
            .time(&timestamp)
            .component("Time Analysis")
    };
    drop(log);

    cmd!("cargo", "bloat", "-p", package, "--time")
        .run()
        .map(|_| ())  // Convert Ok(Output) to Ok(())
        .map_err(|err| {
            // Log the error and then return it
            let log = Log {
            format: LogFormat::CLF,
            ..Log::build(LogLevel::ERROR, "Build time analysis failed")
                .time(&timestamp)
                .component("Time Analysis")
        };
            drop(log);
            err
        })
        .with_context(|| format!("Failed to execute 'cargo bloat' for build time analysis on package '{package}'"))?;
    let log = Log {
        format: LogFormat::CLF,
        ..Log::build(LogLevel::ERROR, "Build time analysis completed")
            .time(&timestamp)
            .component("Time Analysis")
    };
    drop(log);
    Ok(())
}
