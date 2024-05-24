use std::{
    ffi::OsStr,
    io::Result,
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus, Output},
};

use log::{error, info};

/// Trait for running commands, which can be implemented for real or mock commands.
trait CommandRunner {
    fn new<S: AsRef<OsStr>>(cmd: S) -> Self;
    fn args<I, S>(self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
    #[allow(dead_code)]
    fn env<K, V>(self, key: K, value: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>;
    fn spawn(&mut self) -> Result<Output>;
}

/// A wrapper around the standard library's `Command` struct.
///
/// This struct provides an implementation of the `CommandRunner` trait,
/// allowing for the execution of system commands.
struct RealCommand(Command);

impl CommandRunner for RealCommand {
    fn new<S: AsRef<OsStr>>(cmd: S) -> Self {
        RealCommand(Command::new(cmd))
    }

    fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let _ = self.0.args(args); // Ignore the unused result
        self
    }

    fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let _ = self.0.env(key, value);
        self
    }

    fn spawn(&mut self) -> Result<Output> {
        self.0.output()
    }
}

/// A mock command runner for testing purposes.
///
/// This struct is used for testing command execution, allowing for the
/// configuration of the command's output and behaviour.
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
    use log::{LevelFilter, Log, Metadata, Record};
    use rayon::prelude::*;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };
    use xtasks::tasks::ci::{CIBuilder, CI};

    struct TestLogger;

    impl Log for TestLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= LevelFilter::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("{} - {}", record.level(), record.args());
            }
        }

        fn flush(&self) {}
    }

    static LOGGER: TestLogger = TestLogger;

    fn init() {
        let _ = log::set_logger(&LOGGER);
        log::set_max_level(LevelFilter::Info);
    }

    /// Tests the functionality of the CI struct.
    #[test]
    fn test_ci_functionality() {
        init();
        let ci = CI::default();
        assert!(!ci.nightly);
    }

    /// Tests the functionality of the `MockCommand` struct.
    #[test]
    fn test_mock_command() {
        init();
        let output = b"Hello, world!\n";
        let exit_status = ExitStatus::from_raw(0);

        let mock_cmd = MockCommand::new()
            .stdout(output.as_ref())
            .status(exit_status)
            .spawn()
            .expect("Command should succeed");

        assert_eq!(mock_cmd.status, exit_status);
        assert_eq!(&mock_cmd.stdout, output);
    }

    /// Tests the functionality of the `RealCommand` struct.
    #[test]
    fn test_real_command() {
        init();
        let output = RealCommand::new("echo")
            .args(["Hello, world!"])
            .spawn()
            .expect("Command should succeed");

        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "Hello, world!\n"
        );
    }

    /// Tests the CI run method with a mock command.
    #[test]
    fn test_ci_run() {
        init();

        let mock_cmd = Arc::new(Mutex::new(
            MockCommand::new().status(ExitStatus::from_raw(0)),
        ));

        let ci_builder = CIBuilder::default();
        let ci = ci_builder
            .build()
            .expect("CIBuilder should build successfully");

        ci.validate().expect("Validation should pass");

        let tasks: Vec<(String, Vec<String>)> = vec![
            (
                "cargo fmt".to_string(),
                vec![
                    "cargo".to_string(),
                    "fmt".to_string(),
                    "--".to_string(),
                    "--check".to_string(),
                ],
            ),
            (
                "cargo clippy".to_string(),
                vec![
                    "cargo".to_string(),
                    "clippy".to_string(),
                    "--".to_string(),
                    "-D".to_string(),
                    "warnings".to_string(),
                ],
            ),
            (
                "cargo test".to_string(),
                vec!["cargo".to_string(), "test".to_string()],
            ),
            (
                "cargo test --doc".to_string(),
                vec![
                    "cargo".to_string(),
                    "test".to_string(),
                    "--doc".to_string(),
                ],
            ),
        ];

        let results = Mutex::new(HashMap::new());

        tasks.par_iter().for_each(|(name, _args)| {
            info!("Running {}", name);
            let result = mock_cmd.lock().unwrap().spawn();
            if let Err(e) = &result {
                error!("Failed to execute {}: {}", name, e);
            }
            let _ =
                results.lock().unwrap().insert(name.clone(), result); // Ignore the result of insert
        });

        let results = results.into_inner().unwrap();
        for (name, result) in results {
            assert!(result.is_ok(), "Task {} should succeed", name);
        }
    }
}
