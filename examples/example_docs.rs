// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This file is both a standalone example and a module of
// `examples/example.rs`, which calls this `main`. The `pub` is therefore
// required, but from an example binary root the lint cannot see the
// cross-module use and reports it as unreachable.
#![allow(unreachable_pub)]

use xtasks::tasks::docs::docs;
use xtasks::tasks::docs::ensure_cargo_watch_installed;

pub fn main() {
    // Ensure that the cargo-watch tool is installed
    if let Err(e) = ensure_cargo_watch_installed() {
        eprintln!("Failed to ensure cargo-watch is installed: {:?}", e);
        return;
    }

    // Generate and watch documentation for the current project
    if let Err(e) = docs() {
        eprintln!(
            "Error generating and watching documentation: {:?}",
            e
        );
    }
}
