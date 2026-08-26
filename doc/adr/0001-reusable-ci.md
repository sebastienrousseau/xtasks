<!-- SPDX-FileCopyrightText: 2026 xtasks -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# ADR-0001 — CI delegates to a shared reusable workflow

**Status:** accepted

## Context

`xtasks` had four workflows — audit, check, lint, test — duplicating
setup that also existed in every sibling repository. Changing the Rust
version or adding a step meant editing the same YAML in many places.

## Decision

`ci.yml` calls
`sebastienrousseau/pipelines/.github/workflows/rust-ci.yml@main`, plus a
local nightly job marked `continue-on-error`.

## Consequences

Shared improvements arrive without edits here. The cost is a coupling
that fails in an unusually quiet way.

**An undefined input is fatal and near-silent.** `ci.yml` passed
`coverage-threshold: 80`, which `rust-ci.yml` does not define — it has
`coverage-exclude` and `coverage-exclude-packages`, but no threshold.
The run failed at startup before any job began, so pull requests showed
**no checks at all** rather than a failing check. CI was dead for over
a day, and a dependency bump merged with zero checks while reporting
`CLEAN`, because "no checks" and "all checks passed" look the same from
a mergeability API.

Two habits follow:

- Treat a pull request with **zero** checks as a failure to
  investigate, not a pass.
- When changing the inputs passed to a reusable workflow, read that
  workflow's `workflow_call.inputs` rather than assuming, since
  `actionlint` cannot see across repositories and reports the file as
  clean.

Pinning `@main` means upstream changes land without review here. That
is the intended trade for a task runner, but it is why an input
mismatch can appear without any local change.

## Alternatives considered

**Keep local workflows.** No cross-repository coupling, at the cost of
maintaining the same YAML in a dozen repositories.

**Pin the reusable workflow to a tag.** Removes surprise upstream
changes, but needs a bump in every consumer for every improvement.
Worth revisiting if a second startup failure occurs.
