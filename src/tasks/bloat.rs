use std::collections::HashMap;

use anyhow::{anyhow, Context, Result as AnyResult};
use dtt::DateTime;
use duct::cmd;
use rlg::{log_format::LogFormat, log_level::LogLevel, macro_log};
use vrd::random::Random;

/// Analyses the dependencies of the current project to find which ones contribute the most to the build size.
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
/// - If the package build fails and the error message indicates that the package was not found, returns an error with the message "Package '<package>' not found".
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
            "Dependency Analysis",
            "Package name cannot be empty",
            &LogFormat::CLF
        );
        drop(log);
        return Err(anyhow::anyhow!("Package name cannot be empty"));
    }

    if !package.chars().all(|c| c.is_alphanumeric() || c == '-') {
        let log = macro_log!(
            &Random::default().int(0, 1_000_000_000).to_string(),
            &date.iso_8601,
            &LogLevel::ERROR,
            "Dependency Analysis",
            "Package name contains invalid characters",
            &LogFormat::CLF
        );
        drop(log);
        return Err(anyhow::anyhow!(
            "Package name contains invalid characters"
        ));
    }

    // Attempt to build the package
    let build_result = cmd!("cargo", "build", "-p", package).run();
    if let Err(err) = build_result {
        if err.to_string().contains("could not find") {
            let log = macro_log!(
                &Random::default().int(0, 1_000_000_000).to_string(),
                &date.iso_8601,
                &LogLevel::ERROR,
                "Dependency Analysis",
                format!("Package '{}' not found", package).as_str(),
                &LogFormat::CLF
            );
            drop(log);
            return Err(anyhow::anyhow!(format!(
                "Package '{}' not found",
                package
            )));
        } else {
            let log = macro_log!(
                &Random::default().int(0, 1_000_000_000).to_string(),
                &date.iso_8601,
                &LogLevel::ERROR,
                "Dependency Analysis",
                "Package build failed",
                &LogFormat::CLF
            );
            drop(log);
            return Err(err).context("Package build failed");
        }
    }

    // Perform dependency analysis
    let analysis_result =
        cmd!("cargo", "bloat", "-p", package, "--crates").run();
    if let Err(err) = analysis_result {
        let log = macro_log!(
            &Random::default().int(0, 1_000_000_000).to_string(),
            &date.iso_8601,
            &LogLevel::ERROR,
            "Dependency Analysis",
            "Dependency analysis failed",
            &LogFormat::CLF
        );
        drop(log);
        return Err(err).context("Dependency analysis failed");
    }

    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::INFO,
        "Dependency Analysis",
        "Dependency analysis completed",
        &LogFormat::CLF
    );
    drop(log);
    Ok(())
}

/// Analyses the build times of the current project's dependencies.
///
/// This function takes a package name as input and performs a build time analysis using `cargo bloat`.
/// It first logs an informational message indicating that the build time analysis is starting.
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
        &LogLevel::INFO,
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
        .with_context(|| format!("Failed to execute 'cargo bloat' for build time analysis on package '{}'", package))?;

    let log = macro_log!(
        &Random::default().int(0, 1_000_000_000).to_string(),
        &date.iso_8601,
        &LogLevel::INFO,
        "Time Analysis",
        "Build time analysis completed",
        &LogFormat::CLF
    );
    drop(log);
    Ok(())
}

/// Formats the result of the cargo bloat analysis.
///
/// This function takes the raw output of a cargo bloat command, parses it,
/// and formats it into a more readable structure.
///
/// # Arguments
///
/// * `raw_output` - The raw string output from the cargo bloat command.
///
/// # Returns
///
/// Returns a formatted string that is easier to read and understand.
///
/// # Errors
///
/// Returns an error if the parsing fails or the input data is malformed.
pub fn format_analysis_results(raw_output: &str) -> AnyResult<String> {
    let mut formatted_output = String::new();
    let mut results = HashMap::new();

    // Assume each line of the output is a new entry and parse it
    for line in raw_output.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid line format: {}", line));
        }

        // Collect the data into a hashmap for further processing
        let _ = results.insert(
            parts[0].trim().to_string(),
            (parts[1].trim().to_string(), parts[2].trim().to_string()),
        );
    }

    // Format the results into a readable string
    formatted_output.push_str("Dependency Analysis Results:\n");
    for (key, (value1, value2)) in results {
        formatted_output.push_str(&format!(
            "{}: Size = {}, Time = {}\n",
            key, value1, value2
        ));
    }

    Ok(formatted_output)
}
