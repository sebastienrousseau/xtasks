// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use xtasks::tasks::bloat::{deps, time};

pub fn main() {
    // Example usage of the `deps` function to analyze the dependencies of a package
    let package_name = "dtt";
    if let Err(e) = deps(package_name) {
        eprintln!("Error analysing dependencies: {:?}", e);
    }

    // Example usage of the `time` function to analyze the build times of a package
    if let Err(e) = time(package_name) {
        eprintln!("Error analysing build times: {:?}", e);
    }
}
