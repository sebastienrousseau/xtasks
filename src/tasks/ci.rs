use anyhow::{Context, Result as AnyResult};
use derive_builder::Builder;
use duct::cmd;
use serde::{Deserialize, Serialize};

/// Represents the configuration for a Continuous Integration (CI) run.
///
/// This struct encapsulates various settings that can be configured for a CI run,
/// such as whether to run with the nightly compiler or to enable all Clippy lints.
///
#[derive(
    Builder,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Serialize,
    Deserialize,
)]
#[builder(setter(into))]
pub struct CI {
    /// Determines whether to run the CI tasks with the nightly version of the Rust compiler.
    ///
    /// By default, this is set to `false`, meaning that the stable compiler will be used.
    ///
    #[builder(default = "false")]
    pub nightly: bool,

    /// Determines whether to enable all Clippy lints, including pedantic, nursery, and 2018-idioms.
    ///
    /// By default, this is set to `true`, meaning that all Clippy lints will be enabled.
    ///
    #[builder(default = "true")]
    pub clippy_max: bool,
}

impl CIBuilder {
    /// Executes the configured CI tasks.
    ///
    /// This method runs various cargo commands like `cargo fmt`, `cargo clippy`,
    /// and `cargo test`, with arguments determined by the CI configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the cargo commands fail to execute.
    pub fn run(&self) -> AnyResult<()> {
        let CI {
            nightly,
            clippy_max,
        } = self.build().context("Failed to build CI configuration")?;

        if nightly {
            cmd!(
                "rustup", "run", "nightly", "cargo", "fmt", "--",
                "--check"
            )
            .run()
            .context(
                "Failed to execute 'cargo fmt' with nightly compiler",
            )?;
        } else {
            cmd!("cargo", "fmt", "--", "--check")
                .run()
                .context("Failed to execute 'cargo fmt'")?;
        }

        if clippy_max {
            cmd!(
                "cargo",
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
                "-W",
                "clippy::pedantic",
                "-W",
                "clippy::nursery"
            )
            .run()
            .context("Failed to execute 'cargo clippy'")?;
        } else {
            cmd!("cargo", "clippy", "--", "-D", "warnings")
                .run()
                .context("Failed to execute 'cargo clippy'")?;
        }

        cmd!("cargo", "test")
            .run()
            .context("Failed to execute 'cargo test'")?;
        Ok(())
    }
}

/// Executes a sequence of typical Continuous Integration (CI) tasks.
///
/// This function will perform the following CI tasks in order:
/// 1. Format check using `cargo fmt`.
/// 2. Linting using `cargo clippy`.
/// 3. Run tests using `cargo test`.
/// 4. Run documentation tests using `cargo test --doc`.
///
/// The specific settings for these tasks, such as whether to use the nightly compiler or to enable all Clippy lints, are determined by the default configuration of the `CI` struct.
///
/// # Errors
///
/// This function will return an error if any of the CI tasks fail to execute. The specific task that failed will be included in the error message to aid in debugging.
pub fn ci() -> AnyResult<()> {
    CIBuilder::default().run()
}
