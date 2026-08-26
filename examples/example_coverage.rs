// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This file is both a standalone example and a module of
// `examples/example.rs`, which calls this `main`. The `pub` is therefore
// required, but from an example binary root the lint cannot see the
// cross-module use and reports it as unreachable.

//! Example: the `coverage` task, which produces a coverage report.
#![allow(unreachable_pub)]

use xtasks::tasks::coverage::coverage;

/// Demonstrates the `coverage` task, which produces a coverage report.
pub fn main() {
    // Generate a development-specific HTML code coverage report
    if let Err(e) = coverage(true) {
        eprintln!(
            "Error generating development code coverage report: {:?}",
            e
        );
    }

    // Generate a standard HTML code coverage report
    if let Err(e) = coverage(false) {
        eprintln!(
            "Error generating standard code coverage report: {:?}",
            e
        );
    }
}
