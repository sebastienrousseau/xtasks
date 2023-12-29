// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use xtasks::tasks::powerset::{powerset, PowersetBuilder};

pub fn main() {
    // Example of creating a PowersetBuilder with a specific depth
    let builder = PowersetBuilder::new(3);
    // Running the powerset test with the specified builder configuration
    if let Err(e) = builder.run() {
        eprintln!(
            "Error running powerset test with custom configuration: {:?}",
            e
        );
    }

    // Example of performing a CI build with a default powerset of features
    if let Err(e) = powerset() {
        eprintln!(
            "Error performing CI build with powerset of features: {:?}",
            e
        );
    }
}
