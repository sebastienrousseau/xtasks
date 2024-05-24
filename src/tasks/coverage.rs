use anyhow::Result as AnyResult;
use duct::cmd;
use log::info;

/// Generates a code coverage report for the current project.
///
/// # Parameters
///
/// * `dev` - If `true`, generates an HTML report for easier viewing and analysis.
///
/// # Errors
///
/// Returns an error if the `cargo tarpaulin` command fails to execute.
///
pub fn coverage() -> AnyResult<()> {
    info!("Starting coverage generation");

    let _ = cmd!("cargo", "tarpaulin", "--out", "Html").run()?;

    info!("Coverage report generated successfully.");

    Ok(())
}
