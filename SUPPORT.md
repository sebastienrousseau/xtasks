<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Support

## Where to look first

- [`doc/USER-GUIDE.md`](doc/USER-GUIDE.md) — every task and helper, with
  examples that the test suite compiles.
- [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) — how dispatch works and
  why tasks shell out.
- [docs.rs/xtasks](https://docs.rs/xtasks) — exact signatures.

## Before opening an issue

Most reports come down to a missing tool. Tasks wrap external programs:
`bloat` and `time` need `cargo-bloat`, `coverage` needs a coverage
driver, `docs` uses `cargo-watch` in development mode. `xtask install`
installs what the tasks expect.

A second common cause is a shell alias shadowing a cargo subcommand. If
`cargo bloat` works but `xtask bloat` does not, check your cargo
aliases — an alias named after a subcommand shadows the real one and
fails with a recursion error.

## Reporting a bug

Include the command you ran, the output, and `xtask vars`, which prints
the resolved environment.

## Security

Do not use issues for vulnerabilities. See [`SECURITY.md`](SECURITY.md).
