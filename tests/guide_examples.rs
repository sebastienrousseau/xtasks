// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compile-check for the examples in `doc/USER-GUIDE.md`.
//!
//! Documentation examples drift silently: nothing fails when a
//! signature changes underneath a code block in a Markdown file. This
//! only type-checks the calls — it does not run the tasks, which shell
//! out to external tools.

#![allow(unused_imports, dead_code, unreachable_code)]

use xtasks::ops::{
    clean_files, confirm, copy_contents, exists, remove_dir,
    remove_file, root_dir,
};
use xtasks::tasks::{
    bloat, ci, coverage, docs, main_with_args, powerset,
};

/// Type-checks the task entry points without invoking them.
#[allow(clippy::diverging_sub_expression)]
fn _task_signatures() -> Result<(), anyhow::Error> {
    if false {
        ci::ci()?;
        coverage::coverage(false)?;
        powerset::powerset()?;
        docs::docs()?;
        bloat::deps("serde")?;
        bloat::time("serde")?;
        let args = vec!["xtask".to_string(), "ci".to_string()];
        main_with_args(&args)?;
    }
    Ok(())
}

/// Type-checks the `ops` helpers without touching the filesystem.
fn _ops_signatures() -> Result<(), anyhow::Error> {
    if false {
        let _root = root_dir();
        clean_files("target/debug/*.d")?;
        if exists("build") {
            remove_dir("build")?;
        }
        let _bytes = copy_contents("templates", "out", true)?;
        if confirm("Delete the cache?")? {
            remove_file("cache.bin")?;
        }
    }
    Ok(())
}

#[test]
fn guide_examples_typecheck() {
    // Compiling this file is the assertion; root_dir is cheap and real.
    assert!(
        root_dir().is_absolute() || root_dir().components().count() > 0
    );
}
