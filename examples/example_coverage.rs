// Copyright © 2023-2024 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![warn(missing_docs)]

//! This crate provides tasks for generating code coverage reports.
use xtasks::tasks::coverage::coverage;

#[allow(clippy::redundant_pub_crate)]
pub(crate) fn main() {
    // Generate a development-specific HTML code coverage report
    if let Err(e) = coverage(true) {
        eprintln!(
            "Error generating development code coverage report: {e:?}"
        );
    }

    // Generate a standard HTML code coverage report
    if let Err(e) = coverage(false) {
        eprintln!(
            "Error generating standard code coverage report: {e:?}",
        );
    }
}
