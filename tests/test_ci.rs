// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    ffi::OsStr,
    io::Result,
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus, Output},
};

/// A trait defining a set of methods for running system commands.
///
/// This trait abstracts the functionality to run system commands,
/// providing methods to configure and execute them.
trait CommandRunner {
    /// Creates a new command runner instance to run a given program.
    ///
    /// # Examples
    ///
    /// ```
    /// let cmd_runner = CommandRunner::new("ls");
    /// ```
    ///
    /// # Parameters
    ///
    /// - `program`: The program to run.
    ///
    /// # Returns
    ///
    /// A new instance of the implementing type.
    fn new<S: AsRef<OsStr>>(program: S) -> Self
    where
        Self: Sized;

    /// Adds arguments to the command to be run.
    ///
    /// # Examples
    ///
    /// ```
    /// let cmd_runner = CommandRunner::new("ls").args(&["-l", "-a"]);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `args`: An iterator of arguments to pass to the command.
    ///
    /// # Returns
    ///
    /// The command runner instance with the added arguments.
    fn args<I, S>(self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
        Self: Sized;

    /// Adds an environment variable to the command.
    ///
    /// # Examples
    ///
    /// ```
    /// let cmd_runner = CommandRunner::new("printenv")
    ///     .env("KEY", "value");
    /// ```
    ///
    /// # Parameters
    ///
    /// - `key`: The environment variable key.
    /// - `value`: The environment variable value.
    ///
    /// # Returns
    ///
    /// The command runner instance with the added environment variable.
    fn env<K, V>(self, key: K, value: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
        Self: Sized;

    /// Executes the command, returning the output.
    ///
    /// # Returns
    ///
    /// A `Result` containing the command's output or an error.
    fn spawn(&mut self) -> Result<Output>;
}

/// A wrapper around the standard library's `Command` struct.
///
/// This struct provides an implementation of the `CommandRunner` trait,
/// allowing for the execution of system commands.
struct RealCommand(Command);

impl CommandRunner for RealCommand {
    /// Creates a new `RealCommand` instance to run a given program.
    ///
    /// # Parameters
    ///
    /// - `program`: The program to run.
    ///
    /// # Returns
    ///
    /// A new `RealCommand` instance.
    fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self(Command::new(program))
    }

    /// Adds arguments to the command to be run.
    ///
    /// # Parameters
    ///
    /// - `args`: An iterator of arguments to pass to the command.
    ///
    /// # Returns
    ///
    /// The `RealCommand` instance with the added arguments.
    fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    /// Adds an environment variable to the command.
    ///
    /// # Parameters
    ///
    /// - `key`: The environment variable key.
    /// - `value`: The environment variable value.
    ///
    /// # Returns
    ///
    /// The `RealCommand` instance with the added environment variable.
    fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.env(key, value);
        self
    }

    /// Executes the command, returning the output.
    ///
    /// # Returns
    ///
    /// A `Result` containing the command's output or an error.
    fn spawn(&mut self) -> Result<Output> {
        self.0.output()
    }
}

/// A mock command runner for testing purposes.
///
/// This struct is used for testing command execution, allowing for the configuration of
/// the command's output and behaviour.
struct MockCommand {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    args: Vec<String>,
    env: Vec<(String, String)>,
}

impl MockCommand {
    /// Creates a new `MockCommand` instance with default values.
    ///
    /// # Returns
    ///
    /// A new `MockCommand` instance.
    fn new() -> Self {
        Self {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
            args: Vec::new(),
            env: Vec::new(),
        }
    }

    /// Sets the exit status for the mock command.
    ///
    /// # Parameters
    ///
    /// - `status`: The exit status to set.
    ///
    /// # Returns
    ///
    /// The `MockCommand` instance with the updated exit status.
    const fn status(mut self, status: ExitStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the standard output for the mock command.
    ///
    /// # Parameters
    ///
    /// - `stdout`: The data to set as standard output.
    ///
    /// # Returns
    ///
    /// The `MockCommand` instance with the updated standard output.
    fn stdout<S: Into<Vec<u8>>>(mut self, stdout: S) -> Self {
        self.stdout = stdout.into();
        self
    }
}

impl CommandRunner for MockCommand {
    /// Creates a new `MockCommand` instance, ignoring the provided command.
    ///
    /// # Returns
    ///
    /// A new `MockCommand` instance.
    fn new<S: AsRef<OsStr>>(_cmd: S) -> Self {
        Self::new()
    }

    /// Adds arguments to the mock command.
    ///
    /// # Returns
    ///
    /// The `MockCommand` instance with the added arguments.
    fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args.extend(
            args.into_iter()
                .map(|s| s.as_ref().to_string_lossy().to_string()),
        );
        self
    }

    /// Adds an environment variable to the mock command.
    ///
    /// # Returns
    ///
    /// The `MockCommand` instance with the added environment variable.
    fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.env.push((
            key.as_ref().to_string_lossy().to_string(),
            value.as_ref().to_string_lossy().to_string(),
        ));
        self
    }

    /// Simulates the execution of the mock command, returning the configured output.
    ///
    /// # Returns
    ///
    /// A `Result` containing the command's output or an error.
    fn spawn(&mut self) -> Result<Output> {
        Ok(Output {
            status: self.status,
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xtasks::tasks::ci::CI;

    /// Tests the functionality of the CI struct.
    #[test]
    fn test_ci_functionality() {
        let ci = CI::default();
        assert!(!ci.nightly);
        assert!(!ci.clippy_max);
    }

    /// Tests the functionality of the `MockCommand` struct.
    #[test]
    fn test_mock_command() {
        let output = b"Hello, world!\n";
        let exit_status = ExitStatus::from_raw(0);

        let mock_cmd = MockCommand::new()
            .stdout(*output)
            .status(exit_status)
            .spawn()
            .expect("Command should succeed");

        assert_eq!(mock_cmd.status, exit_status);
        assert_eq!(&mock_cmd.stdout, output);
    }
}
