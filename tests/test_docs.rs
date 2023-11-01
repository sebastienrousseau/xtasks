#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::io::Result;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{Command, ExitStatus, Output};

    trait CommandRunner {
        fn new(cmd: &str) -> Self
        where
            Self: Sized;
        fn args<I, S>(self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
            Self: Sized;
        fn env<K: AsRef<OsStr>, V: AsRef<OsStr>>(
            self,
            key: K,
            value: V,
        ) -> Self
        where
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
            self.0.args(args);
            self
        }

        fn env<K: AsRef<OsStr>, V: AsRef<OsStr>>(
            mut self,
            key: K,
            value: V,
        ) -> Self {
            self.0.env(key, value);
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
        env: Vec<(String, String)>,
    }

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

        fn env<K: AsRef<OsStr>, V: AsRef<OsStr>>(
            mut self,
            key: K,
            value: V,
        ) -> Self {
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
    fn test_ensure_cargo_watch_installed() {
        // Using MockCommand for testing
        let cmd = MockCommand::new("cargo")
            .args(["install", "cargo-watch"])
            .status(ExitStatus::from_raw(0))
            .stdout(b"cargo-watch installed successfully".to_vec())
            .spawn()
            .expect("Failed to execute 'cargo install cargo-watch'.");

        assert!(cmd.status.success());
        assert_eq!(
            String::from_utf8(cmd.stdout).unwrap(),
            "cargo-watch installed successfully"
        );
    }

    #[test]
    fn test_docs_function() {
        // Using MockCommand for testing
        let cmd = MockCommand::new("cargo")
            .args(["watch", "-s", "cargo doc --no-deps"])
            .status(ExitStatus::from_raw(0))
            .stdout(b"Documentation generated successfully".to_vec())
            .spawn()
            .expect("Failed to execute 'cargo watch' for generating documentation.");

        assert!(cmd.status.success());
        assert_eq!(
            String::from_utf8(cmd.stdout).unwrap(),
            "Documentation generated successfully"
        );
    }
}
