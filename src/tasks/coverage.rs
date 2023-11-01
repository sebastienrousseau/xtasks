use anyhow::{Context, Result as AnyResult};
use duct::cmd;

/// Generates a code coverage report for the current project.
///
/// # Parameters
///
/// * `dev` - If `true`, generates an HTML report for easier viewing and analysis.
///
/// # Errors
///
/// Returns an error if the `cargo tarpaulin` command fails to execute.
pub fn coverage(dev: bool) -> AnyResult<()> {
    let coverage_cmd = if dev {
        cmd!("cargo", "tarpaulin", "--out", "Html", "--dev")
    } else {
        cmd!("cargo", "tarpaulin", "--out", "Html")
    };

    coverage_cmd.run().context(
        "Failed to execute 'cargo tarpaulin' for code coverage",
    )?;
    Ok(())
}
