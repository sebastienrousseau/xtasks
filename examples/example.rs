// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT


/// This is a module for bloat example task.
mod example_bloat;
/// This is a module for ci example task.
mod example_ci;
/// This is a module for coverage example task.
mod example_coverage;
/// This is a module for docs example task.
mod example_docs;
/// This is a module for powerset example task.
mod example_powerset;

fn main() {
    // Run bloat example task.
    example_bloat::main();

    // Run ci example task.
    let _ = example_ci::main();

    // Run coverage example task.
    example_coverage::main();

    // Run docs example task.
    example_docs::main();

    // Run powerset example task.
    example_powerset::main();
}
