<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Security policy

## Reporting a vulnerability

Report privately through the repository's **Security → Report a
vulnerability** page. Do not open a public issue.

## Supported versions

Pre-1.0. Only the latest `0.0.x` receives fixes.

## Threat model, honestly stated

`xtasks` is a developer task runner. It **executes external programs by
design** — `cargo` and its subcommands — and is intended to be run by a
developer on their own workspace, not as a service or on untrusted
input.

That shapes what counts as a vulnerability here:

- **In scope:** command injection through a task argument, a task
  writing outside the workspace root, following a symlink out of a
  directory it was asked to clean, a dependency advisory.
- **Not in scope:** the fact that tasks run external programs, or that
  a task fails when a tool it wraps is missing.

`clean_files` refuses to remove directories, so a stray glob cannot
delete a tree. If you find a way past that, it is in scope.

## Posture

- `cargo audit` runs in CI; an advisory blocks a release.
- Publishing uses crates.io Trusted Publishing, so no long-lived
  registry credential exists in the repository.
