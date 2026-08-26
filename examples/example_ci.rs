// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This file is both a standalone example and a module of
// `examples/example.rs`, which calls this `main`. The `pub` is therefore
// required, but from an example binary root the lint cannot see the
// cross-module use and reports it as unreachable.

//! Example: the `ci` task, which runs the full local check suite.
#![allow(unreachable_pub)]

use anyhow::Context;
use xtasks::tasks::ci::CIBuilder;

/// Demonstrates the `ci` task, which runs the full local check suite.
pub fn main() -> anyhow::Result<()> {
    // Create a CI builder with custom settings
    let ci_builder = CIBuilder::default()
        .nightly(true) // Set to use the nightly compiler
        .clippy_max(false) // Disable maximum Clippy lints
        .build()
        .context("Failed to build custom CI configuration")?;

    // Execute CI tasks with the custom configuration
    // Assuming `run` is a method of `CIBuilder` before `build` is called
    CIBuilder::default()
        .nightly(ci_builder.nightly)
        .clippy_max(ci_builder.clippy_max)
        .run()
        .context("Failed to run CI tasks")?;

    Ok(())
}
