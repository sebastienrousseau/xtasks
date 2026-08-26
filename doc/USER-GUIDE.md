<!-- SPDX-FileCopyrightText: 2026 xtasks -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# `xtasks` user guide

`xtasks` is a task runner for Rust workspaces: a set of `cargo`
wrappers behind one binary, plus the filesystem helpers those tasks
need.

## Contents

1. [Install](#1-install)
2. [Tasks](#2-tasks)
3. [Using it as a library](#3-using-it-as-a-library)
4. [Filesystem helpers](#4-filesystem-helpers)
5. [Macros](#5-macros)

## 1. Install

```sh
cargo install xtasks
```

Or as a dependency, if you are building your own runner on top:

```toml
[dependencies]
xtasks = "0.0.2"
```

## 2. Tasks

```sh
xtask ci                      # the full local check suite
xtask coverage                # coverage report
xtask coverage --dev          # coverage in development mode
xtask powerset                # check every feature combination
xtask docs                    # build the API documentation
xtask bloat --package NAME    # dependency size analysis
xtask time --package NAME     # build timing
xtask vars                    # print the resolved environment
xtask dev                     # development loop
xtask install                 # install the tools the tasks need
```

Most tasks wrap an external tool. `bloat` and `time` need
`cargo-bloat`; `coverage` needs a coverage driver; `docs` uses
`cargo-watch` in development mode and will offer to install it. If a
tool is missing the task fails with the tool's own error, which is
usually clearer than anything this crate could substitute.

## 3. Using it as a library

Every task is a plain function, so a workspace can build its own runner
without going through the CLI:

```rust
use xtasks::tasks::{bloat, ci, coverage, docs, powerset};

ci::ci()?;                        // run the check suite
coverage::coverage(false)?;       // false = not dev mode
powerset::powerset()?;
docs::docs()?;
bloat::deps("serde")?;            // dependency sizes
bloat::time("serde")?;            // build timing
# Ok::<(), anyhow::Error>(())
```

Dispatch is testable without spawning a process:

```rust
use xtasks::tasks::main_with_args;

let args = vec!["xtask".to_string(), "ci".to_string()];
main_with_args(&args)?;
# Ok::<(), anyhow::Error>(())
```

That separation is deliberate — `main.rs` contains nothing but a call
into the library.

## 4. Filesystem helpers

```rust
use xtasks::ops::{clean_files, confirm, copy_contents, exists, remove_dir, remove_file, root_dir};

let root = root_dir();            // workspace root, wherever you ran from
clean_files("target/debug/*.d")?; // glob removal
if exists("build") {
    remove_dir("build")?;
}
copy_contents("templates", "out", true)?;   // true = overwrite

if confirm("Delete the cache?")? {
    remove_file("cache.bin")?;
}
# Ok::<(), anyhow::Error>(())
```

`root_dir()` is what makes tasks location-independent: they operate on
the workspace, not on the current directory.

`copy_contents` copies the *contents* of a directory rather than the
directory itself, takes an `overwrite` flag, and returns the number of
bytes copied.

`clean_files` takes a glob and **fails if the pattern matches a
directory** — it removes files only. That is intentional, so a stray
pattern cannot delete a tree.

## 5. Macros

Task output is routed through the crate's macros rather than written
straight to stdout, so a runner can capture it:

| Macro | Purpose |
|---|---|
| `run_command!` | run a program, propagate failure |
| `run_std_command!` | as above, via `std::process` |
| `run_cargo_command!` | run a `cargo` subcommand |
| `macro_cargo_cmd!` | build a cargo invocation |
| `macro_execute_and_log!` | run and log the outcome |

`println!`, `print!` and `assert!` are shadowed on purpose. If you are
writing a task and output is not appearing where you expect, that is
why.
