# Changelog

Notable changes to `xtasks`. Earlier releases are described on
the [GitHub releases page](https://github.com/sebastienrousseau/xtasks/releases).

## [0.0.6] - 2026-11-07

v0.0.5 was tagged but never published: its release workflow failed on
the two Windows GNU builds, because `ubuntu-latest` moved to Ubuntu
26.04, where the `cross` 0.2.5 image for those targets fails. Those two
jobs now run on `ubuntu-24.04`. Nothing else differs from v0.0.5.

## [0.0.5] - 2026-11-06

### Deprecated

- **`xtasks` is deprecated and unmaintained.** This is the final
  release; the GitHub repository is archived after it. Nothing on
  crates.io depends on the crate. Use a `Makefile` or `justfile` in
  your own repository, or a local `cargo xtask` crate in your
  workspace. `Cargo.toml` declares
  `[badges] maintenance = { status = "deprecated" }` and the README
  and crate documentation open with the notice.

### Changed

- `rlg` 0.0.16 (from #80).
- CI and release workflows default `GITHUB_TOKEN` to `contents: read`;
  only the release job asks for `contents: write`. This clears the six
  open `actions/missing-workflow-permissions` code-scanning alerts.
- The release workflow titles the draft release `xtasks <version>`.

### Removed

- `document.yml`, a documentation workflow that could never run (its
  job required a push to `main`, which it did not trigger on) and
  targeted a GitHub Pages site the repository does not have.

## [0.0.4] - 2026-11-01

### Changed

- **Breaking: the MSRV is now Rust 1.89.0** (was 1.71.1), one floor
  shared with the maintainer's other Rust crates. `rust-version` in
  `Cargo.toml` is the only place xtasks states it.
- The `cargo bloat` error paths log through `inspect_err` instead of a
  `map_err` that returned the error unchanged, as clippy asks once the
  floor allows it.
- The documentation workflow passes the gh-pages commit identity as
  `user_name` and `user_email`, the inputs
  peaceiris/actions-gh-pages v4 defines; the old `commit_user_*` and
  `clean` inputs were ignored. The action is pinned to its v4.1.0
  commit.
