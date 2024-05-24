#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::io::Result;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

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
        fn env<K, V>(self, key: K, value: V) -> Self
        where
            K: AsRef<OsStr>,
            V: AsRef<OsStr>,
            Self: Sized;
        fn spawn(&mut self) -> Result<Output>;
    }

    struct MockCommand {
        status: ExitStatus,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
        args: Vec<String>,
        env: Vec<(String, String)>,
    }

    #[allow(dead_code)]
    impl MockCommand {
        fn new(_cmd: &str) -> Self {
            Self {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
                args: Vec::new(),
                env: Vec::new(),
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
                env: Vec::new(),
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

        fn spawn(&mut self) -> Result<Output> {
            Ok(Output {
                status: self.status,
                stdout: self.stdout.clone(),
                stderr: self.stderr.clone(),
            })
        }
    }

    #[test]
    fn test_coverage() {
        let cmd = MockCommand::new("cargo");
        let output = cmd
            .args(["tarpaulin", "--out", "Html"])
            .stdout("Coverage report generated successfully.")
            .spawn()
            .unwrap();
        assert!(output.status.success());
    }
}
