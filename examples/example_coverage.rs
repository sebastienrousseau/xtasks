// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use xtasks::tasks::coverage::coverage;

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
