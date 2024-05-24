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
pub fn coverage(dev: bool) -> AnyResult<()> {
    info!("Starting coverage generation. Dev mode: {}", dev);

    let coverage_cmd = if dev {
        cmd!("cargo", "tarpaulin", "--out", "Html", "--dev")
    } else {
        cmd!("cargo", "tarpaulin", "--out", "Html")
    };

    let output = coverage_cmd.stderr_to_stdout().unchecked().run()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to execute 'cargo tarpaulin': {}\nOutput: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    info!("Coverage report generated successfully.");
    Ok(())
}
