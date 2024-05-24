//! # Cargo `XTask`
//!
//! A collection of tasks to be executed with `cargo xtask`.
//!
//! ## Overview
//!
//! This module provides a comprehensive suite of tasks aimed at streamlining the development, testing, and maintenance of Rust projects. It leverages `cargo xtask`, a convention for creating and running custom cargo commands, enabling developers to extend Cargo's capabilities and integrate additional tooling and workflows directly into their build process.
//!
//! ## Features
//!
//! - **Documentation Generation**: Automate the creation of project documentation, ensuring consistency and completeness across all codebase components.
//!
//! - **Continuous Integration (CI) Tasks**: Implement a variety of CI tasks to validate code quality, run tests, and ensure the stability of the codebase.
//!
//! - **Dependency Analysis**: Analyze project dependencies for potential issues, outdated libraries, and opportunities for optimization.
//!
//! - **Development Workflow Enhancement**: Streamline the development workflow with tasks designed to automate repetitive tasks and improve efficiency.
//!
//! - **Customization**: Easily extend and customize tasks to suit the unique requirements of your project.
//!
//! ## Usage
//!
//! To use these tasks, you will need to have `cargo xtask` installed. Once installed, you can run tasks using the following command:
//!
//! ```sh
//! cargo xtask <task-name>
//! ```
//!
//! Replace `<task-name>` with the name of the task you wish to execute. Each task may have its own set of arguments and options, which can be discovered by running:
//!
//! ```sh
//! cargo xtask <task-name> --help
//! ```
//!
//! ## Contributing
//!
//! Contributions to enhance existing tasks or add new tasks are welcome. Please ensure that all new tasks are well-documented and include appropriate error handling to maintain the robustness of the tooling.
//!
//! ## License
//!
//! This collection of cargo xtasks is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See LICENSE-APACHE and LICENSE-MIT for details.

use crate::tasks::{
    bloat::{deps, format_analysis_results, time},
    coverage::coverage,
    docs::docs,
    powerset::powerset,
};
use anyhow::{Context, Result as AnyResult};
use clap::{Arg, ArgAction, ArgMatches, Command};
use duct::cmd;
use log::error;
use std::{
    env,
    fs::write,
    fs::{self, read_to_string},
    path::Path,
};

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

/// Checks if the given command is already installed.
///
/// # Arguments
///
/// * `command` - A string slice that holds the name of the command to check.
///
/// # Returns
///
/// * `bool` - Returns `true` if the command is installed, otherwise `false`.
fn is_installed(command: &str) -> bool {
    if let Ok(paths) = env::var("PATH") {
        for path in paths.split(':') {
            let full_path = format!("{path}/{command}");
            if fs::metadata(full_path).is_ok() {
                return true;
            }
        }
    }
    false
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
pub fn install() -> Result<(), anyhow::Error> {
    let commands = [
        ("cargo", ["install", "cargo-watch"].as_ref()),
        ("cargo", ["install", "cargo-hack"].as_ref()),
        ("cargo", ["install", "cargo-tarpaulin"].as_ref()),
        ("cargo", ["install", "cargo-bloat"].as_ref()),
        (
            "rustup",
            ["component", "add", "llvm-tools-preview"].as_ref(),
        ),
        ("cargo", ["install", "grcov"].as_ref()),
    ];

    for (cmd_name, args) in &commands {
        if is_installed(args[1]) {
            println!("{} is already installed.", args[1]);
        } else {
            let result = cmd(*cmd_name, *args).run().context(format!(
                "Failed to run install command: {} {}",
                cmd_name,
                args.join(" ")
            ));

            // Handle the result properly
            if let Err(e) = result {
                eprintln!("{}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Parses the xtask configuration file.
///
/// This function reads and parses the `xtasks.toml` configuration file, returning the parsed
/// configuration as a `toml::Value`.
///
/// # Returns
///
/// * `AnyResult<toml::Value>`: A `toml::Value` representing the parsed configuration, or an `Err` variant encapsulating any error that occurs during parsing.
///
/// # Errors
///
/// This function will return an error if the `xtasks.toml` file cannot be read or parsed.
pub fn parse_config() -> AnyResult<Option<toml::Value>> {
    match read_to_string("xtasks.toml") {
        Ok(config_content) => {
            let config: toml::Value =
                toml::from_str(&config_content)
                    .context("Failed to parse xtasks.toml")?;
            Ok(Some(config))
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            println!("xtasks.toml not found. Proceeding without configuration.");
            Ok(None)
        }
        Err(err) => Err(err).context("Failed to read xtasks.toml"),
    }
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
#[allow(clippy::too_many_lines)]
pub fn main_with_args(args: &[String]) -> AnyResult<()> {
    let dry_run_arg = Arg::new("dry-run")
        .short('n')
        .long("dry-run")
        .action(ArgAction::SetTrue)
        .id("dry-run")
        .help("Show what would be done without doing it");

    let verbose_arg = Arg::new("verbose")
        .short('v')
        .long("verbose")
        .action(ArgAction::SetTrue)
        .id("verbose")
        .help("Show verbose output");

    let cli = Command::new("xtasks")
        .arg(verbose_arg)
        .subcommand(Command::new("benchmark").arg(dry_run_arg.clone()))
        .subcommand(
    Command::new("bloat-deps")
        .arg(dry_run_arg.clone())
        .arg(
            Arg::new("package")
                .short('p')
                .long("package")
                .value_name("PACKAGE")
                .help("The package to analyze for bloat")
                .required(true),
        ),
)
        .subcommand(
    Command::new("bloat-report")
        .arg(dry_run_arg.clone())
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("The output file to write formatted results to")
                .required(true),
        ),
)
        .subcommand(
    Command::new("bloat-time")
        .arg(dry_run_arg.clone())
        .arg(
            Arg::new("package")
                .short('p')
                .long("package")
                .value_name("PACKAGE")
                .help("The package to analyze for bloat")
                .required(true),
        ),
)
        .subcommand(Command::new("ci").arg(dry_run_arg.clone()))
        .subcommand(Command::new("clean").arg(dry_run_arg.clone()))
        .subcommand(Command::new("config").arg(dry_run_arg.clone()))
        .subcommand(
    Command::new("coverage")
        .arg(dry_run_arg.clone())
        .arg(
            Arg::new("dev")
                .long("dev")
                .action(ArgAction::SetTrue)
                .help("Include development dependencies in the coverage report"),
        ),
)
        .subcommand(Command::new("docs").arg(dry_run_arg.clone()))
        .subcommand(Command::new("format").arg(dry_run_arg.clone()))
        .subcommand(Command::new("init").arg(dry_run_arg.clone()))
        .subcommand(Command::new("install").arg(dry_run_arg.clone()))
        .subcommand(Command::new("lint").arg(dry_run_arg.clone()))
        .subcommand(Command::new("powerset").arg(dry_run_arg.clone()))
        .subcommand(Command::new("release").arg(dry_run_arg.clone()))
        .subcommand(Command::new("security").arg(dry_run_arg.clone()))
        .subcommand(Command::new("update").arg(dry_run_arg))
        .subcommand(Command::new("vars"));

    let matches = cli.get_matches_from(args);
    let verbose =
        matches.get_one::<bool>("verbose").copied().unwrap_or(false);

    if verbose {
        env_logger::Builder::new()
            .filter(None, log::LevelFilter::Info)
            .init();
    }

    match matches.subcommand() {
        Some(("benchmark", sub_matches)) => {
            handle_benchmark(sub_matches)
        }
        Some(("bloat-deps", sub_matches)) => {
            handle_bloat_deps(sub_matches)
        }
        Some(("bloat-report", sub_matches)) => {
            handle_bloat_report(sub_matches)
        }
        Some(("bloat-time", sub_matches)) => {
            handle_bloat_time(sub_matches)
        }
        Some(("ci", sub_matches)) => handle_ci(sub_matches),
        Some(("clean", sub_matches)) => handle_clean(sub_matches),
        Some(("config", sub_matches)) => handle_config(sub_matches),
        Some(("coverage", sub_matches)) => handle_coverage(sub_matches),
        Some(("docs", sub_matches)) => handle_docs(sub_matches),
        Some(("format", sub_matches)) => handle_format(sub_matches),
        Some(("init", sub_matches)) => handle_init(sub_matches),
        Some(("install", sub_matches)) => handle_install(sub_matches),
        Some(("lint", sub_matches)) => handle_lint(sub_matches),
        Some(("powerset", sub_matches)) => handle_powerset(sub_matches),
        Some(("release", sub_matches)) => handle_release(sub_matches),
        Some(("security", sub_matches)) => handle_security(sub_matches),
        Some(("update", sub_matches)) => handle_update(sub_matches),
        Some(("vars", _)) => {
            handle_vars();
            Ok(())
        }
        _ => {
            let error_msg = "Unrecognized subcommand";
            error!("{}", error_msg);
            Err(anyhow::Error::msg(error_msg))
        }
    }
}

/// Handles the 'install' subcommand.
///
/// This function installs the required tools and dependencies for the project.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the installation is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the installation process fails.
fn handle_install(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would install tools");
    } else {
        install().context("Failed to install tools")?;
    }
    Ok(())
}

/// Handles the `bloat-report` subcommand.
///
/// This function generates a detailed bloat report by running the `cargo bloat` command, processing its output, and formatting it into a human-readable table. The report can be printed to the console or written to a specified output file.
///
/// # Arguments
///
/// * `args` - A reference to `clap::ArgMatches` containing the command-line arguments.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the report is generated successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if:
/// - The `cargo bloat` command fails to run.
/// - The output of the `cargo bloat` command cannot be parsed.
/// - The formatted report cannot be written to the specified output file.
///
pub fn handle_bloat_report(args: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        args.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would generate bloat report");
    } else {
        let output = args.get_one::<String>("output");

        // Generate cargo bloat output
        let cargo_bloat_output =
            cmd!("cargo", "bloat", "--release", "--crates")
                .read()
                .context("Failed to run cargo bloat")?;

        // Print raw output for debugging
        // println!("cargo bloat output:\n{}", cargo_bloat_output);

        let raw_output =
            String::from_utf8(cargo_bloat_output.into_bytes())
                .context("Failed to parse cargo bloat output")?;

        // Preprocess the raw output to match the expected format
        let preprocessed_output: String = raw_output
            .lines()
            .skip_while(|line| {
                !line.starts_with(" File  .text     Size Crate")
            })
            .skip(1) // Skip the header line
            .take_while(|line| !line.trim().starts_with("56.7%")) // Take lines until we reach the total line
            .filter(|line| line.split_whitespace().count() >= 4) // Ensure there are enough parts in the line
            .map(|line| {
                let parts: Vec<&str> =
                    line.split_whitespace().collect();
                let crate_name = parts.last().unwrap();
                let size = parts[2];
                let percentage = parts[0];
                format!("{}, {}, {}", crate_name, size, percentage)
            })
            .collect::<Vec<String>>()
            .join("\n");

        // Print preprocessed output for debugging
        // println!("Preprocessed output:\n{}", preprocessed_output);

        let formatted_report =
            format_analysis_results(&preprocessed_output)?;

        if let Some(output_file) = output {
            write(output_file, formatted_report)
                .context("Failed to write output file")?;
        } else {
            println!("Bloat Report:\n{}", formatted_report);
        }
    }
    Ok(())
}

/// Handles the 'lint' subcommand.
///
/// This function runs the linter on the project's codebase.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the linting is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the linting process fails.
fn handle_lint(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would run linter");
    } else {
        run_linter().context("Failed to run linter")?;
    }
    Ok(())
}

/// Handles the 'format' subcommand.
///
/// This function formats the project's codebase according to the specified formatting rules.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the formatting is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the formatting process fails.
fn handle_format(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would format the code");
    } else {
        run_formatter().context("Failed to format code")?;
    }
    Ok(())
}

/// Handles the 'release' subcommand.
///
/// This function prepares the project for release by running necessary checks and builds.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the release preparation is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the release preparation process fails.
fn handle_release(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would prepare release");
    } else {
        prepare_release().context("Failed to prepare release")?;
    }
    Ok(())
}

/// Handles the 'benchmark' subcommand.
///
/// This function runs the project's benchmarks.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the benchmarks run successfully or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the benchmarking process fails.
fn handle_benchmark(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would run benchmarks");
    } else {
        run_benchmarks().context("Failed to run benchmarks")?;
    }
    Ok(())
}

/// Handles the 'security' subcommand.
///
/// This function runs security checks on the project's dependencies and codebase.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the security checks run successfully or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the security checking process fails.
fn handle_security(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would run security checks");
    } else {
        run_security_checks()
            .context("Failed to run security checks")?;
    }
    Ok(())
}

/// Handles the 'config' subcommand.
///
/// This function manages the project's configuration.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the configuration management is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the configuration management process fails.
fn handle_config(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would manage config");
    } else {
        println!("Managing config...");
        match manage_config() {
            Ok(()) => println!("Config management succeeded"),
            Err(e) => {
                eprintln!("Config management failed: {e:?}");
                eprintln!("Backtrace: {}", e.backtrace());
                return Err(e);
            }
        }
    }
    Ok(())
}

/// Handles the 'ci' subcommand.
///
/// This function runs the project's continuous integration tasks.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the CI tasks run successfully or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the CI process fails.
fn handle_ci(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would run CI");
    } else {
        println!("Running CI...");
        match ci::ci() {
            Ok(()) => println!("CI completed successfully"),
            Err(e) => {
                eprintln!("CI failed: {e:?}");
                eprintln!("Backtrace: {}", e.backtrace());
                return Err(e).context("Failed to run CI");
            }
        }
    }
    Ok(())
}

/// Handles the 'update' subcommand.
///
/// This function updates the project's dependencies.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the dependency update is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the dependency update process fails.
fn handle_update(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would update dependencies");
    } else {
        update_dependencies()
            .context("Failed to update dependencies")?;
    }
    Ok(())
}

/// Handles the 'clean' subcommand.
///
/// This function cleans the project's build artifacts and generated files.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the cleaning process is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the cleaning process fails.
fn handle_clean(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would clean project");
    } else {
        clean_project().context("Failed to clean project")?;
    }
    Ok(())
}

/// Handles the 'init' subcommand.
///
/// This function initializes a new project.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the project initialization is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the project initialization process fails.
fn handle_init(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would initialize project");
    } else {
        initialize_project().context("Failed to initialize project")?;
    }
    Ok(())
}

/// Handles the 'vars' subcommand.
///
/// This function prints the project's root directory path.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the root directory path is successfully printed.
fn handle_vars() {
    let root = crate::ops::root_dir();
    println!("root: {root:?}");
}

/// Handles the 'coverage' subcommand.
///
/// This function generates a code coverage report for the project.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the coverage report is successfully generated or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the coverage report generation process fails.
fn handle_coverage(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    let dev_mode =
        matches.get_one::<bool>("dev").copied().unwrap_or(false); // Extract the `dev` parameter from matches

    if dry_run {
        println!("Would generate coverage report");
        Ok(())
    } else {
        coverage(dev_mode)
    }
}

/// Handles the 'docs' subcommand.
///
/// This function generates the project's documentation.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the documentation is successfully generated or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the documentation generation process fails.
fn handle_docs(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would generate documentation");
    } else {
        docs()?;
    }
    Ok(())
}

/// Handles the 'powerset' subcommand.
///
/// This function generates the powerset of the project's dependencies.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the powerset is successfully generated or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the powerset generation process fails.
fn handle_powerset(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would generate powerset");
    } else {
        powerset()?;
    }
    Ok(())
}

/// Handles the 'bloat-deps' subcommand.
///
/// This function analyzes the project's dependencies for bloat.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the dependency bloat analysis is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the dependency bloat analysis process fails or if the required 'package'
/// argument is missing.
fn handle_bloat_deps(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would analyze dependencies for bloat");
    } else {
        let package = matches
            .get_one::<String>("package")
            .context("Please provide a package with -p")?;
        deps(package, matches)?;
    }
    Ok(())
}

/// Handles the 'bloat-time' subcommand.
///
/// This function analyzes the project's build time for bloat.
///
/// # Arguments
///
/// * `matches`: An `ArgMatches` struct representing the command-line arguments for this subcommand.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the build time bloat analysis is successful or a dry run is performed,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the build time bloat analysis process fails or if the required 'package'
/// argument is missing.
fn handle_bloat_time(matches: &ArgMatches) -> AnyResult<()> {
    let dry_run =
        matches.get_one::<bool>("dry-run").copied().unwrap_or(false);
    if dry_run {
        println!("Would analyze build time for bloat");
    } else {
        let package = matches
            .get_one::<String>("package")
            .context("Please provide a package with -p")?;
        time(package, matches)?;
    }
    Ok(())
}

/// Runs the linter using `cargo clippy` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the linter runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn run_linter() -> AnyResult<()> {
    let output =
        cmd!("cargo", "clippy", "--all-targets", "--all-features")
            .run()
            .expect("Failed to run linter");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Linter failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Runs the formatter using `cargo fmt` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the formatter runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn run_formatter() -> AnyResult<()> {
    let output = cmd!("cargo", "fmt", "--all")
        .run()
        .expect("Failed to run formatter");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Formatter failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Runs the checker using `cargo check` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the checker runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn run_checker() -> AnyResult<()> {
    let output = cmd!("cargo", "check", "--all", "--release")
        .run()
        .expect("Failed to run checker");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Checker failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Runs the build using `cargo build` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the build runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn run_build() -> AnyResult<()> {
    let output = cmd!("cargo", "build", "--all", "--release")
        .run()
        .expect("Failed to build");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Build failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Prepares the project for release by running necessary checks and builds.
///
/// This function executes the `cargo check` and `cargo build` commands with the `--release` flag
/// to ensure the project is ready for release.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the release preparation process is successful,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the release preparation process fails.
fn prepare_release() -> AnyResult<()> {
    run_checker().context("Failed to run release checks")?;
    run_build().context("Failed to build release artifacts")?;
    Ok(())
}

/// Runs the benchmarks using `cargo bench` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the benchmarks run successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn execute_benchmarks() -> AnyResult<()> {
    let output = cmd!("cargo", "bench", "--all")
        .run()
        .expect("Failed to run benchmarks");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Benchmarks failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Runs the project's benchmarks.
///
/// This function executes the `cargo bench` command to run the project's benchmarks.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the benchmarking process is successful,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the benchmarking process fails.
fn run_benchmarks() -> AnyResult<()> {
    execute_benchmarks().context("Failed to run benchmarks")?;
    Ok(())
}

/// Runs the security audit using `cargo audit` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the security audit runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn execute_security_audit() -> AnyResult<()> {
    let output = cmd!("cargo", "audit")
        .run()
        .expect("Failed to run security audit");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Security audit failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Runs security checks on the project's dependencies and codebase.
///
/// This function executes the `cargo audit` command to perform a security audit of the project's dependencies.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the security checking process is successful,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the security checking process fails.
fn run_security_checks() -> AnyResult<()> {
    execute_security_audit().context("Failed to run security audit")?;
    Ok(())
}

/// Manages the project's configuration.
///
/// This function reads and parses the project's configuration file (`xtasks.toml`) and prints its contents.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the configuration management process is successful,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the configuration file cannot be read or parsed.
fn manage_config() -> AnyResult<()> {
    match parse_config()? {
        Some(config) => {
            println!("Configuration: {config:#?}");
        }
        None => {
            println!("No configuration found.");
        }
    }
    Ok(())
}

/// Runs the dependency update using `cargo update` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the dependency update runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn execute_update_dependencies() -> AnyResult<()> {
    let output = cmd!("cargo", "update")
        .run()
        .expect("Failed to update dependencies");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Dependency update failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Updates the project's dependencies.
///
/// This function executes the `cargo update` command to update the project's dependencies to their latest compatible versions.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the dependency update process is successful,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the dependency update process fails.
fn update_dependencies() -> AnyResult<()> {
    execute_update_dependencies()
        .context("Failed to update dependencies")?;
    Ok(())
}

/// Runs the clean operation using `cargo clean` and checks for successful execution.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the clean operation runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn execute_clean_project() -> AnyResult<()> {
    let output = cmd!("cargo", "clean")
        .run()
        .expect("Failed to clean project");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Clean operation failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Cleans the project's build artifacts and generated files.
///
/// This function executes the `cargo clean` command to remove the project's build artifacts and generated files.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the cleaning process is successful,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the cleaning process fails.
fn clean_project() -> AnyResult<()> {
    execute_clean_project().context("Failed to clean project")?;
    Ok(())
}

/// Runs the project initialization using `cargo new` and checks for successful execution.
///
/// # Arguments
///
/// * `project_dir` - The directory name for the new project.
///
/// # Returns
///
/// * `AnyResult<()>` - An `Ok(())` variant if the project initialization runs successfully or
///   a `Result::Err` variant encapsulating any error that occurs during execution.
fn execute_initialize_project(project_dir: &str) -> AnyResult<()> {
    let output = cmd!("cargo", "new", "--bin", project_dir)
        .run()
        .expect("Failed to initialize new project");
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Project initialization failed with exit code: {:?}",
            output.status
        ))
    }
}

/// Initializes a new project.
///
/// This function creates a new directory for the project and initializes a new Rust binary project within it using the `cargo new` command.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the project initialization process is successful,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will return an error if the project initialization process fails.
fn initialize_project() -> AnyResult<()> {
    let project_dir = "new_project";

    // Check if the project directory already exists
    if Path::new(project_dir).exists() {
        // Remove the existing directory and its contents
        fs::remove_dir_all(project_dir).context(format!(
            "Failed to remove existing directory: {project_dir}"
        ))?;
    }

    // Initialize a new project
    execute_initialize_project(project_dir)
        .context("Failed to initialize new project")?;

    Ok(())
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
