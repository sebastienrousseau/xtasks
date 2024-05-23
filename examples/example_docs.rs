//! This crate provides tasks for generating and watching documentation.

use xtasks::tasks::docs::docs;
use xtasks::tasks::docs::ensure_cargo_watch_installed;

#[allow(clippy::redundant_pub_crate)]
pub(crate) fn main() {
    // Ensure that the cargo-watch tool is installed
    if let Err(e) = ensure_cargo_watch_installed() {
        eprintln!("Failed to ensure cargo-watch is installed: {e:?}");
        return;
    }

    // Generate and watch documentation for the current project
    if let Err(e) = docs() {
        eprintln!("Error analysing build times: {e:?}");
    }
}
