#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::io::Result;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    #[allow(dead_code)]
    trait CommandRunner {
        fn new<S: AsRef<OsStr>>(program: S) -> Self
        where
            Self: Sized;
        fn args<I, S>(self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
            Self: Sized;
        fn run(&mut self) -> Result<()>;
    }

    struct MockCommand {
        status: ExitStatus,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
        args: Vec<String>,
    }

    #[allow(dead_code)]
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
        fn new<S: AsRef<OsStr>>(_cmd: S) -> Self {
            Self {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
                args: Vec::new(),
            }
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

        fn run(&mut self) -> Result<()> {
            if self.status.success() {
                println!("{}", String::from_utf8_lossy(&self.stdout));
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Command failed: {}",
                        String::from_utf8_lossy(&self.stderr)
                    ),
                ))
            }
        }
    }

    #[test]
    fn test_coverage_success() {
        let mut cmd = MockCommand::new("cargo")
            .args(["tarpaulin", "--out", "Html"])
            .stdout("Coverage report generated successfully.");

        let result = cmd.run();
        assert!(result.is_ok());
    }

    #[test]
    fn test_coverage_failure() {
        let mut cmd = MockCommand::new("cargo")
            .args(["tarpaulin", "--out", "Html"])
            .status(ExitStatus::from_raw(1))
            .stderr("Failed to generate coverage report.");

        let result = cmd.run();
        assert!(result.is_err());
    }
}
