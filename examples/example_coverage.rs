//! This crate provides tasks for generating code coverage reports.
use xtasks::tasks::coverage::coverage;

#[allow(clippy::redundant_pub_crate)]
pub(crate) fn main() {
    // Generate a standard HTML code coverage report
    if let Err(e) = coverage() {
        eprintln!(
            "Error generating standard code coverage report: {e:?}",
        );
    }
}
