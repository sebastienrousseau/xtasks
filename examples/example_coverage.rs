//! This crate provides tasks for generating code coverage reports.
use std::env;
use xtasks::tasks::coverage::coverage;

#[allow(clippy::redundant_pub_crate)]
pub(crate) fn main() {
    // Determine dev_mode based on an environment variable
    let dev_mode =
        env::var("DEV_MODE").map(|v| v == "true").unwrap_or(false);

    // Generate a standard HTML code coverage report
    if let Err(e) = coverage(dev_mode) {
        eprintln!(
            "Error generating standard code coverage report: {e:?}",
        );
    }
}
