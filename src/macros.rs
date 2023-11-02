// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! # Macros
//!
//! This module provides convenient macros for common operations in the library.

/// Prints a formatted message to the console with a newline.
///
/// # Parameters
///
/// * `args`: The format string and values to print.
///
/// # Examples
///
/// ```rust
/// println!("This is a {} message", "formatted");
/// ```
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        std::println!($($arg)*);
    };
}

/// Prints a formatted message to the console without a newline.
///
/// # Parameters
///
/// * `args`: The format string and values to print.
///
/// # Examples
///
/// ```rust
/// print!("This is a {} message", "formatted");
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        std::print!($($arg)*);
    };
}

/// Macros related to asserting conditions in the code.
///
/// Asserts that a condition is true, and panics with a formatted message if it is not.
///
/// # Parameters
///
/// * `$cond`: The condition to assert.
/// * `args`: The format string and values for the panic message if the assertion fails.
///
/// # Examples
///
/// ```rust
/// assert!(1 + 1 == 2, "Math is broken!");
/// ```
#[macro_export]
macro_rules! assert {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            std::panic!("Assertion failed: {}", std::format!($($arg)*));
        }
    };
}

/// Custom logging macro for various log levels and formats.
///
/// # Parameters
///
/// * `$level`: The log level of the message.
/// * `$component`: The component where the log is coming from.
/// * `$description`: A description of the log message.
/// * `$format`: The format of the log message.
///
#[macro_export]
macro_rules! macro_log_info {
    ($level:expr, $component:expr, $description:expr, $format:expr) => {{
        use dtt::DateTime;
        use vrd::Random;
        use $crate::loggers::{Log, LogFormat, LogLevel};

        // Get the current date and time in ISO 8601 format.
        let date = DateTime::new();
        let iso = date.iso_8601;

        // Create a new random number generator
        let mut rng = Random::default();
        let session_id = rng.rand().to_string();

        let log = Log::new(
            &session_id,
            &iso,
            $level,
            $component,
            $description,
            $format,
        );
        let _ = log.log();
        log // Return the Log instance
    }};
}

/// Macros related to executing shell commands.
///
/// Executes a shell command, logs the start and completion of the operation, and handles any errors that occur.
///
/// # Parameters
///
/// * `$command`: The shell command to execute.
/// * `$package`: The name of the package the command is being run on.
/// * `$operation`: A description of the operation being performed.
/// * `$start_message`: The log message to be displayed at the start of the operation.
/// * `$complete_message`: The log message to be displayed upon successful completion of the operation.
/// * `$error_message`: The log message to be displayed in case of an error.
///
/// # Returns
///
/// Returns a `Result<(), anyhow::Error>` indicating the success or failure of the operation.
///
#[macro_export]
macro_rules! macro_execute_and_log {
    ($command:expr, $package:expr, $operation:expr, $start_message:expr, $complete_message:expr, $error_message:expr) => {{
        use anyhow::{Context, Result as AnyResult};
        use $crate::loggers::{LogFormat, LogLevel};
        use $crate::macro_log_info;

        macro_log_info!(
            LogLevel::INFO,
            $operation,
            $start_message,
            LogFormat::CLF
        );

        $command
            .run()
            .map(|_| ())
            .map_err(|err| {
                macro_log_info!(
                    LogLevel::ERROR,
                    $operation,
                    $error_message,
                    LogFormat::CLF
                );
                err
            })
            .with_context(|| {
                format!(
                    "Failed to execute '{}' for {} on package '{}'",
                    stringify!($command),
                    $operation,
                    $package
                )
            })?;

        macro_log_info!(
            LogLevel::INFO,
            $operation,
            $complete_message,
            LogFormat::CLF
        );
        Ok(())
    }};
}

/// Executes a cargo command with optional arguments and error handling.
///
/// This macro simplifies the execution of cargo commands, handling optional arguments based on the CI configuration,
/// and providing a context for any errors that occur.
///
/// # Parameters
///
/// * `command`: The base cargo command to execute (e.g., "fmt", "clippy", "test").
/// * `nightly`: A boolean indicating whether to run the command with the nightly compiler.
/// * `args`: Additional arguments to pass to the cargo command.
/// * `error_message`: The error message to display if the command fails to execute.
///
#[macro_export]
macro_rules! macro_cargo_cmd {
    ($command:expr, $nightly:expr, $args:expr, $error_message:expr) => {{
        use duct::cmd;
        use anyhow::Context;

        let mut command = if $nightly {
            cmd!("rustup", "run", "nightly", "cargo", $command, $($args),*)
        } else {
            cmd!("cargo", $command, $($args),*)
        };

        command.run().context($error_message)
    }};
}

/// Executes a command and provides context for any potential errors.
///
/// This macro simplifies the process of running a command and handling
/// any errors that may occur, by attaching a provided context message
/// to the resulting error. This makes error messages more informative
/// and helps in diagnosing issues more quickly.
///
/// # Parameters
///
/// * `$cmd`: The command to be executed. This should be an expression
///   that evaluates to a type implementing the `duct::cmd` interface.
/// * `$context`: A string expression providing context for the command.
///   This message will be attached to any errors that occur during the
///   execution of the command.
///
/// # Errors
///
/// If the command fails to execute, an error is returned with the
/// attached context message.
///
/// # Note
///
/// This macro requires the `duct` crate for command execution and
/// the `anyhow` crate for error handling. Ensure that these crates
/// are included in your project's dependencies and properly imported
/// in your code.
#[macro_export]
macro_rules! run_command {
    ($cmd:expr, $context:expr) => {
        $cmd.run().context($context)?
    };
}

/// Executes a standard command and provides context for any potential errors.
///
/// This macro simplifies the process of running a command using `std::process::Command`
/// and handling any errors that may occur, by attaching a provided context message
/// to the resulting error. This makes error messages more informative
/// and helps in diagnosing issues more quickly.
///
/// # Parameters
///
/// * `$cmd`: The command to be executed. This should be an expression
///   that evaluates to a type implementing the `std::process::Command` interface.
/// * `$context`: A string expression providing context for the command.
///   This message will be attached to any errors that occur during the
///   execution of the command.
///
/// # Errors
///
/// If the command fails to execute, or if the command returns a non-zero exit status,
/// an error is returned with the attached context message and the command's output.
///
#[macro_export]
macro_rules! run_std_command {
    ($cmd:expr, $context:expr) => {
        let output = $cmd.output().with_context(|| $context)?;
        if !output.status.success() {
            return Err(anyhow::Error::msg(format!(
                "{}: {:?}",
                $context, output
            )));
        }
    };
}

/// Executes a cargo command and provides context for any potential errors.
///
/// This macro is a convenience wrapper around `run_std_command`, specifically
/// tailored for executing cargo commands. It ensures consistent error handling
/// and provides informative error messages.
///
/// # Parameters
///
/// * `$args`: An expression that evaluates to an iterator of arguments for the cargo command.
/// * `$context`: A string expression providing context for the command.
///
/// # Errors
///
/// If the cargo command fails to execute, or if the command returns a non-zero exit status,
/// an error is returned with the attached context message and the command's output.
///
#[macro_export]
macro_rules! run_cargo_command {
    ($args:expr, $context:expr) => {
        run_std_command!(Command::new("cargo").args($args), $context)
    };
}
