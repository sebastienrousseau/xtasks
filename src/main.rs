// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Command-line entry point for `xtasks`.
//!
//! A thin wrapper around [`xtasks::tasks::main`]. Argument parsing and
//! every task implementation live in the library crate, so they can be
//! exercised by tests without spawning a process.

/// Runs the task selected on the command line.
///
/// # Errors
///
/// Propagates whatever [`xtasks::tasks::main`] returns: an unknown task
/// name, a failure inside the task itself, or a failure of a tool it
/// shells out to.
fn main() -> Result<(), anyhow::Error> {
    xtasks::tasks::main()
}
