// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result as AnyResult};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents the configuration for generating a powerset of features for cargo build runs.
///
/// A powerset in this context refers to all possible combinations of features that can be enabled
/// or disabled for a cargo build.
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
pub struct Powerset {
    /// Specifies the depth of the powerset.
    ///
    /// The depth determines how many features will be considered for each combination in the powerset.
    /// For example, a depth of 2 would consider all combinations of 2 features at a time.
    ///
    /// Valid values are integers greater than or equal to 1.
    ///
    /// By default, this is set to 2.
    #[builder(default = "2")]
    pub depth: i32,

    /// Determines whether to exclude build runs with no default features enabled.
    ///
    /// Setting this to `true` means that build runs where no default features are enabled will be excluded
    /// from the powerset. This can be useful if you know that your project requires at least one feature to be
    /// enabled for successful compilation.
    ///
    /// By default, this is set to `false`.
    #[builder(default = "false")]
    pub exclude_no_default_features: bool,
}

impl PowersetBuilder {
    /// Builds and runs a powerset test.
    ///
    /// This function will execute `cargo hack clippy`, `cargo hack test`, and `cargo hack test --doc`
    /// with various feature combinations based on the powerset configuration.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the `cargo` commands fail to execute.
    pub fn run(&self) -> AnyResult<()> {
        let t = self
            .build()
            .context("Failed to build Powerset configuration")?;
        let depth = t.depth.to_string();
        let mut common_args = vec![
            "--workspace",
            "--exclude",
            "xtask",
            "--feature-powerset",
            "--depth",
            &depth,
        ];
        if t.exclude_no_default_features {
            common_args.push("--exclude-no-default-features");
        }

        let mut clippy_args = common_args.clone();
        clippy_args.extend(["--", "-D", "warnings"]);
        std::process::Command::new("cargo")
            .args(["hack", "clippy"])
            .args(&clippy_args)
            .status()
            .context("Failed to execute 'cargo hack clippy'")?;

        let mut test_args = common_args.clone();
        test_args.push("test");
        std::process::Command::new("cargo")
            .args(["hack"])
            .args(&test_args)
            .status()
            .context("Failed to execute 'cargo hack test'")?;

        let mut doc_test_args = common_args;
        doc_test_args.extend(["test", "--doc"]);
        std::process::Command::new("cargo")
            .args(["hack"])
            .args(&doc_test_args)
            .status()
            .context("Failed to execute 'cargo hack test --doc'")?;

        Ok(())
    }

    /// Creates a new `PowersetBuilder` instance with a specified depth.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth of the powerset.
    ///
    /// # Examples
    ///
    /// ```
    /// # use xtasks::tasks::powerset::PowersetBuilder;
    /// let builder = PowersetBuilder::new(3);
    /// ```
    pub fn new(depth: i32) -> Self {
        let mut builder = PowersetBuilder::default();
        builder.depth(depth);
        builder
    }
}

///
/// Perform a CI build with powerset of features
///
/// # Errors
/// Errors if one of the commands failed
///
pub fn powerset() -> AnyResult<()> {
    PowersetBuilder::default().run()
}
