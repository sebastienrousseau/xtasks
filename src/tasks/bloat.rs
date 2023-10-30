// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result as AnyResult};
use duct::cmd;

/// Analyses the dependencies of the current project to find which ones contribute most to the build size.
///
/// # Parameters
///
/// * `package` - The name of the package to analyze.
///
/// # Examples
///
/// ```
/// # use anyhow::Result;
/// # fn run() -> Result<()> {
/// bloat_deps("my_package")?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the `cargo bloat` command fails to execute.
pub fn bloat_deps(package: &str) -> AnyResult<()> {
    cmd!("cargo", "bloat", "-p", package, "--crates")
        .run()
        .with_context(|| format!("Failed to execute 'cargo bloat' for dependency analysis on package '{}'", package))?;
    Ok(())
}

/// Analyses the build times of dependencies in the current project.
///
/// # Parameters
///
/// * `package` - The name of the package to analyze.
///
/// # Examples
///
/// ```
/// # use anyhow::Result;
/// # fn run() -> Result<()> {
/// bloat_time("my_package")?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the `cargo bloat` command fails to execute.
pub fn bloat_time(package: &str) -> AnyResult<()> {
    cmd!("cargo", "bloat", "-p", package, "--time")
        .run()
        .with_context(|| format!("Failed to execute 'cargo bloat' for build time analysis on package '{}'", package))?;
    Ok(())
}
