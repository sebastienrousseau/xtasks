<!-- SPDX-FileCopyrightText: 2026 xtasks -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# `xtasks` architecture

The map a contributor needs. Companion to
[`USER-GUIDE.md`](USER-GUIDE.md) and the rustdoc on
[docs.rs](https://docs.rs/xtasks).

## Layout

```
src/
├── lib.rs        # re-exports: macros, ops, tasks
├── main.rs       # thin binary; delegates to tasks::main
├── macros.rs     # command-running and logging macros
├── ops.rs        # filesystem and prompt helpers
├── tasks.rs      # CLI definition and dispatch
└── tasks/
    ├── bloat.rs      # dependency size, build timing
    ├── ci.rs         # the full local check suite
    ├── coverage.rs   # coverage report
    ├── docs.rs       # documentation build
    └── powerset.rs   # feature-combination checks
```

The split that matters: **`main.rs` holds nothing**. Argument parsing
and every task live in the library, so they can be exercised by tests
without spawning a process. The binary is nine lines.

## Dispatch

`tasks::main` collects arguments and hands them to `main_with_args`,
which builds a `clap::Command` with one subcommand per task. Separating
the two is what makes dispatch testable: a test calls
`main_with_args(&["xtask", "ci"])` directly.

Subcommands: `ci`, `coverage` (`--dev`), `powerset`, `docs`, `bloat`
(`--package`), `time` (`--package`), `vars`, `dev`, `install`.

## Everything shells out

`xtasks` is a task runner. Almost every task is a wrapper around a
`cargo` subcommand or another tool, run through the macros in
`macros.rs`:

| Macro | Purpose |
|---|---|
| `run_command!` | run a program, propagate failure |
| `run_std_command!` | as above, via `std::process` |
| `run_cargo_command!` | run a `cargo` subcommand |
| `macro_cargo_cmd!` | build a cargo invocation |
| `macro_execute_and_log!` | run and log the outcome |

`println!`, `print!` and `assert!` are shadowed deliberately so task
output is routed consistently rather than written straight to stdout.

**Consequence for tests.** A task's behaviour is mostly the behaviour of
the tool it calls, so a test that asserts a task *succeeds* is really
asserting the tool is installed and working. Those tests belong behind
an availability check — see [`TESTING.md`](TESTING.md).

## `ops`

Filesystem and prompt helpers shared by the tasks:

- `clean_files(pattern)` — glob-based removal
- `remove_file` / `remove_dir` / `exists`
- `copy_contents(from, to)`
- `confirm(question)` — interactive yes/no
- `root_dir()` — the workspace root, so tasks are location-independent

## CI

Delegates to the shared `rust-ci.yml` in
`sebastienrousseau/pipelines`, plus a nightly-toolchain test job that is
`continue-on-error`.

The inputs a caller passes must exist in the reusable workflow. Passing
one that does not is not a warning — the run fails at startup before any
job begins, and the pull request shows *no checks at all* rather than a
failure. See [ADR-0001](adr/0001-reusable-ci.md).
