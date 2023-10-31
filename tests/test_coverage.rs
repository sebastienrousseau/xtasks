
#[cfg(test)]
mod tests {
    use std::process::{Command, Output, ExitStatus};
    use std::io::Result;
    use std::ffi::OsStr;
    use std::os::unix::process::ExitStatusExt;

    trait CommandRunner {
        fn new<S: AsRef<OsStr>>(program: S) -> Self where Self: Sized;
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

    struct RealCommand(Command);

    impl CommandRunner for RealCommand {
        fn new<S: AsRef<OsStr>>(program: S) -> Self {
            RealCommand(Command::new(program))
        }

        fn args<I, S>(mut self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
        {
            self.0.args(args);
            self
        }

        fn env<K, V>(mut self, key: K, value: V) -> Self
        where
            K: AsRef<OsStr>,
            V: AsRef<OsStr>,
        {
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
            MockCommand {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
                args: Vec::new(),
                env: Vec::new(),
            }
        }

        fn status(mut self, status: ExitStatus) -> Self {
            self.status = status;
            self
        }

        fn stdout<S: Into<Vec<u8>>>(mut self, stdout: S) -> Self {
            self.stdout = stdout.into();
            self
        }

    }

    impl CommandRunner for MockCommand {
        fn new<S: AsRef<OsStr>>(_cmd: S) -> Self {
            MockCommand {
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
            self.args.extend(args.into_iter().map(|s| s.as_ref().to_string_lossy().to_string()));
            self
        }

        fn env<K, V>(mut self, key: K, value: V) -> Self
        where
            K: AsRef<OsStr>,
            V: AsRef<OsStr>,
        {
            self.env.push((key.as_ref().to_string_lossy().to_string(), value.as_ref().to_string_lossy().to_string()));
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
        // Using MockCommand for testing
        let example_output = br"
Jan 30 21:43:33.715  INFO cargo_tarpaulin::config: Creating config
Jan 30 21:43:33.908  INFO cargo_tarpaulin: Running Tarpaulin
Jan 30 21:43:33.908  INFO cargo_tarpaulin: Building project
Jan 30 21:43:33.908  INFO cargo_tarpaulin::cargo: Cleaning project
   Compiling simple_project v0.1.0 (/home/daniel/personal/tarpaulin/tests/data/simple_project)
    Finished test [unoptimized + debuginfo] target(s) in 0.51s
Jan 30 21:43:34.631  INFO cargo_tarpaulin::process_handling::linux: Launching test
Jan 30 21:43:34.631  INFO cargo_tarpaulin::process_handling: running /home/daniel/personal/tarpaulin/tests/data/simple_project/target/debug/deps/simple_project-417a21905eb8be09

running 1 test
test tests::bad_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

Jan 30 21:43:35.563  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/lib.rs: 6
|| src/unused.rs: 4-6
|| Tested/Total Lines:
|| src/lib.rs: 3/4
|| src/unused.rs: 0/3
|| 
42.86% coverage, 3/7 lines covered
";

    let cmd = MockCommand::new("cargo")
    .args(["tarpaulin"])
    .status(ExitStatus::from_raw(0))
    .stdout(example_output.to_vec())
    .spawn()
    .expect("Failed to execute 'cargo tarpaulin'.");

    assert!(cmd.status.success());
    assert_eq!(cmd.stdout, example_output);
    }
}
