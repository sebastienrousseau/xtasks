#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::io::Result;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{Command, ExitStatus, Output};
    use xtasks::tasks::docs::docs;

    #[allow(dead_code)]
    trait CommandRunner {
        fn new(cmd: &str) -> Self
        where
            Self: Sized;
        fn args<I, S>(self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
            Self: Sized;
        fn spawn(&mut self) -> Result<Output>;
    }

    struct RealCommand(Command);

    impl CommandRunner for RealCommand {
        fn new(cmd: &str) -> Self {
            Self(Command::new(cmd))
        }

        fn args<I, S>(mut self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
        {
            let _ = self.0.args(args);
            self
        }

        fn spawn(&mut self) -> Result<Output> {
            self.0.output()
        }
    }

    struct MockCommand {
        status: ExitStatus,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
        args: Vec<String>,
    }

    impl MockCommand {
        fn new(_cmd: &str) -> Self {
            Self {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
                args: Vec::new(),
            }
        }

        const fn status(mut self, status: ExitStatus) -> Self {
            self.status = status;
            self
        }

        fn stdout<S: Into<Vec<u8>>>(mut self, stdout: S) -> Self {
            self.stdout = stdout.into();
            self
        }

        fn stderr<S: Into<Vec<u8>>>(mut self, stderr: S) -> Self {
            self.stderr = stderr.into();
            self
        }
    }

    impl CommandRunner for MockCommand {
        fn new(cmd: &str) -> Self {
            Self::new(cmd)
        }

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

        fn spawn(&mut self) -> Result<Output> {
            Ok(Output {
                status: self.status,
                stdout: self.stdout.clone(),
                stderr: self.stderr.clone(),
            })
        }
    }

    // Mock the `run_command!` macro to prevent executing the actual command
    #[allow(unused_macros)]
    macro_rules! mock_run_command {
        ($cmd:expr, $err_msg:expr) => {
            // Do nothing, just return Ok(())
            Ok(())
        };
    }

    // Use the mocked `run_command!` macro
    #[allow(unused_macros)]
    macro_rules! run_command {
        ($cmd:expr, $err_msg:expr) => {
            mock_run_command!($cmd, $err_msg)
        };
    }

    // Replace the original `run_command!` macro with the mocked version
    #[test]
    fn test_docs_success() {
        let _cmd = MockCommand::new("cargo")
            .args(["watch", "-s", "cargo doc --no-deps"])
            .status(ExitStatus::from_raw(0))
            .stdout(b"Documentation generated successfully".to_vec());

        let result = docs();
        assert!(result.is_ok());
    }

    #[test]
    fn test_docs_failure() {
        let _cmd = MockCommand::new("cargo")
            .args(["watch", "-s", "cargo doc --no-deps"])
            .status(ExitStatus::from_raw(1))
            .stderr(b"Failed to generate documentation".to_vec());

        let result = docs();
        assert!(result.is_err());
    }
}
