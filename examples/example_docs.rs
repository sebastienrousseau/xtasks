// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

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
