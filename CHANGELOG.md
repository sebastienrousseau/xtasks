# Changelog

Notable changes to `xtasks`. Earlier releases are described on
the [GitHub releases page](https://github.com/sebastienrousseau/xtasks/releases).

## [0.0.4] - Unreleased

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
