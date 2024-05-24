use anyhow::{Context, Result as AnyResult};
use derive_builder::Builder;
use duct::cmd;
use log::{error, info};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Represents the configuration for a Continuous Integration (CI) run.
///
/// This struct encapsulates various settings that can be configured for a CI run,
/// such as whether to run with the nightly compiler or to enable all Clippy lints.
///
#[derive(
    Builder,
    Copy,
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

impl CI {
    /// Validates the CI configuration.
    ///
    /// This method ensures that the configuration settings are valid.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> AnyResult<()> {
        if self.nightly
            && !cmd!("rustup", "toolchain", "list")
                .read()?
                .contains("nightly")
        {
            return Err(anyhow::anyhow!(
                "Nightly toolchain is not installed"
            ));
        }
        Ok(())
    }
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
        let ci =
            self.build().context("Failed to build CI configuration")?;
        ci.validate()?;

        let tasks: Vec<(String, Vec<String>)> = vec![
            (
                "cargo fmt".to_string(),
                if ci.nightly {
                    vec![
                        "rustup".to_string(),
                        "run".to_string(),
                        "nightly".to_string(),
                        "cargo".to_string(),
                        "fmt".to_string(),
                        "--".to_string(),
                        "--check".to_string(),
                    ]
                } else {
                    vec![
                        "cargo".to_string(),
                        "fmt".to_string(),
                        "--".to_string(),
                        "--check".to_string(),
                    ]
                },
            ),
            (
                "cargo clippy".to_string(),
                if ci.clippy_max {
                    vec![
                        "cargo".to_string(),
                        "clippy".to_string(),
                        "--all-targets".to_string(),
                        "--all-features".to_string(),
                        "--".to_string(),
                        "-D".to_string(),
                        "warnings".to_string(),
                        "-W".to_string(),
                        "clippy::pedantic".to_string(),
                        "-W".to_string(),
                        "clippy::nursery".to_string(),
                    ]
                } else {
                    vec![
                        "cargo".to_string(),
                        "clippy".to_string(),
                        "--".to_string(),
                        "-D".to_string(),
                        "warnings".to_string(),
                    ]
                },
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

        tasks.par_iter().for_each(|(name, args)| {
            info!("Running {}", name);
            let result = cmd(&args[0], &args[1..]).run();
            if let Err(e) = &result {
                error!("Failed to execute {}: {}", name, e);
            }
            let _ =
                results.lock().unwrap().insert(name.clone(), result);
        });

        let results = results.into_inner().unwrap();
        for (name, result) in results {
            let _ = result
                .context(format!("Failed to execute {}", name))?;
        }

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
