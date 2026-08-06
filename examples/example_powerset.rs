// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This file is both a standalone example and a module of
// `examples/example.rs`, which calls this `main`. The `pub` is therefore
// required, but from an example binary root the lint cannot see the
// cross-module use and reports it as unreachable.
#![allow(unreachable_pub)]

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
