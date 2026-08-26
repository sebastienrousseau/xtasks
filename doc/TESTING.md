<!-- SPDX-FileCopyrightText: 2026 xtasks -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Testing strategy

## The problem this crate has

`xtasks` runs other programs. Most of what a task does is decided by
the tool it shells out to, which means a test asserting that a task
succeeds is often asserting that a tool is installed, on PATH, and
working — a property of the machine, not of this crate.

That is not hypothetical. `test_deps_with_real_command` asserted
`deps("clap").is_ok()`, and failed on any machine without `cargo bloat`
installed, and on machines whose cargo aliases shadow the subcommand.

So the rule here is: **test the parts you control, and gate the parts
you do not.**

| Layer | What it covers |
|---|---|
| Unit tests | argument parsing, path handling, macro expansion |
| Integration tests | `main_with_args` dispatch, `ops` against a tempdir |
| Tool-dependent tests | skipped unless the tool is available |

## Running

```sh
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

## Gating a tool-dependent test

```rust
fn cargo_bloat_available() -> bool {
    std::process::Command::new("cargo")
        .args(["bloat", "--version"])
        .output()
        .is_ok_and(|o| o.status.success())
}

#[test]
fn deps_runs() {
    if !cargo_bloat_available() {
        eprintln!("skipping: `cargo bloat` is not available");
        return;
    }
    // ...
}
```

Skipping loudly beats failing misleadingly. A red suite that means
"you have not installed an optional tool" trains people to ignore red.

## clippy stops at the first failing target

Worth knowing, because it hides work. A single error in the binary
crate prevented the examples and integration tests from being linted at
all; fixing it surfaced fifteen further violations that had been
invisible for as long as the first one existed.

If you fix a lint and a wave of new ones appears, they were already
there.

## Matching CI's toolchain

CI installs `stable`. If your shell pins `RUSTUP_TOOLCHAIN`, local runs
can disagree with CI and a clean local clippy means nothing. Use a
fresh target directory too — a cached pass is indistinguishable from a
real one:

```sh
RUSTUP_TOOLCHAIN= CARGO_TARGET_DIR=/tmp/check \
  cargo +stable clippy --workspace --all-targets --all-features -- -D warnings
```
