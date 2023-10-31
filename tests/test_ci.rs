use std::ffi::OsStr;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Output, ExitStatus};
use std::io::Result;

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
    fn new() -> Self {
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
        MockCommand::new()
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

#[cfg(test)]
mod tests {
    use super::*;
    use xtasks::tasks::ci::CI;

    #[test]
    fn test_ci_functionality() {
        let ci = CI::default();
        assert_eq!(ci.nightly, false);
        assert_eq!(ci.clippy_max, false);
    }

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
