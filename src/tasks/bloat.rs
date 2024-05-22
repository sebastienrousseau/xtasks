// Copyright © 2023-2024 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result as AnyResult};
use duct::cmd;

use dtt::DateTime;
use rlg::{log_format::LogFormat, log_level::LogLevel, macro_log};
use vrd::random::Random;

/// Analyzes the dependencies of the current project to find which ones contribute the most to the build size.
///
/// This function takes a package name as input and performs a dependency analysis using `cargo bloat`.
/// It first checks if the package name is valid and logs appropriate error messages if it's empty or contains invalid characters.
/// It then attempts to build the package using `cargo build` and captures the build output.
/// If the build succeeds, it proceeds with the dependency analysis using `cargo bloat -p <package> --crates`.
/// If the analysis fails, it logs an error message and returns the error.
/// If the package build fails, it checks the error message and returns an appropriate error.
/// Finally, it logs an informational message indicating that the dependency analysis has completed.
///
/// # Arguments
///
/// * `package` - The name of the package to analyze.
///
/// # Returns
///
/// Returns `Ok(())` if the dependency analysis completes successfully.
///
/// # Errors
///
/// Returns an error in the following cases:
/// - If the package name is empty, returns an error with the message "Package name cannot be empty".
/// - If the package name contains invalid characters, returns an error with the message "Package name contains invalid characters".
/// - If the package build fails and the error message indicates that the package was not found, returns an error with the message "Package '&lt;package&gt;' not found".
/// - If the package build fails due to other reasons, returns an error with the original error message and additional context.
/// - If the dependency analysis fails, returns the original error.
pub fn deps(package: &str) -> AnyResult<()> {
    let date = DateTime::new();

    // Check if the package name is valid
    if package.is_empty() {
        let log = macro_log!(
            &Random::default().int(0, 1_000_000_000).to_string(),
            &date.iso_8601,
            &LogLevel::ERROR,
            "Dependencies",
            "Package name cannot be empty",
            &LogFormat::CLF
        );
        drop(log);
        return Err(anyhow::Error::msg("Package name cannot be empty"));
    }

    if !package
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        let log = macro_log!(
            &Random::default().int(0, 1_000_000_000).to_string(),
            &date.iso_8601,
            &LogLevel::ERROR,
            "Dependencies",
            "Package name contains invalid characters",
            &LogFormat::CLF
        );
        drop(log);
        return Err(anyhow::Error::msg(
            "Package name contains invalid characters",
        ));
    }

    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::INFO,
        "Dependencies",
        "Starting dependency analysis",
        &LogFormat::CLF
    );
    drop(log);

    let build_result = cmd!("cargo", "build", "--package", package)
        .stderr_to_stdout()
        .run();

    match build_result {
        Ok(_) => {
            // Package build succeeded, proceed with dependency analysis
            cmd!("cargo", "bloat", "-p", package, "--crates")
                .run()
                .map(|_| ())
                .map_err(|err| {
                    // Log the error and then return it
                    let log = macro_log!(
                        &Random::default().int(0, 1_000_000_000).to_string(),
                        &date.iso_8601,
                        &LogLevel::ERROR,
                        "Dependencies",
                        "Dependency analysis failed",
                        &LogFormat::CLF
                    );
                    drop(log);
                    err
                })
                .with_context(|| {
                    format!("Failed to execute 'cargo bloat' for dependency analysis on package '{package}'")
                })?;
        }
        Err(err) => {
            // Package build failed
            let log = macro_log!(
                &Random::default().int(0, 1_000_000_000).to_string(),
                &date.iso_8601,
                &LogLevel::ERROR,
                "Dependencies",
                &format!("Failed to build package '{package}'"),
                &LogFormat::CLF
            );
            drop(log);
            return Err(anyhow::Error::new(err).context(format!(
                "Failed to build package '{package}'"
            )));
        }
    }

    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::INFO,
        "Dependencies",
        "Dependency analysis completed",
        &LogFormat::CLF
    );
    drop(log);

    Ok(())
}

/// Analyzes the build times of dependencies in the current project.
///
/// This function takes a package name as input and performs a build time analysis using `cargo bloat`.
/// It logs an informational message indicating that the build time analysis is starting.
/// It then executes the `cargo bloat -p <package> --time` command to analyze the build times of dependencies.
/// If the analysis succeeds, it returns `Ok(())`.
/// If the analysis fails, it logs an error message and returns the error.
/// Finally, it logs an informational message indicating that the build time analysis has completed.
///
/// # Arguments
///
/// * `package` - The name of the package to analyze.
///
/// # Returns
///
/// Returns `Ok(())` if the build time analysis completes successfully.
///
/// # Errors
///
/// Returns an error if the `cargo bloat` command fails to execute. This could be due to various reasons,
/// such as the package not being found or `cargo bloat` not being installed.
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
