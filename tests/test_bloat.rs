// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration tests for the `bloat` task.

#[cfg(test)]
mod tests {
    use xtasks::tasks::bloat::{deps, time};

    /// Reports whether `cargo bloat` is callable in this environment.
    ///
    /// `deps` shells out to it, so without it the test below is
    /// measuring the machine rather than this crate.
    fn cargo_bloat_available() -> bool {
        std::process::Command::new("cargo")
            .args(["bloat", "--version"])
            .output()
            .is_ok_and(|o| o.status.success())
    }

    /// Tests the `deps` function with a valid package name.
    ///
    /// Skipped when `cargo bloat` is not installed. The subcommand is an
    /// optional developer tool, and a missing tool is an environment
    /// fact rather than a defect in `deps` -- asserting success
    /// unconditionally made this test fail on any machine without it,
    /// and on any machine whose cargo aliases shadow the subcommand.
    #[test]
    fn test_deps_with_real_command() {
        if !cargo_bloat_available() {
            eprintln!("skipping: `cargo bloat` is not available");
            return;
        }
        let package = "clap";
        let result = deps(package);
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
    }

    /// Tests the `deps` function with a nonexistent package name.
    /// This scenario should result in an error, as the specified package does not exist.
    #[test]
    fn test_deps_with_nonexistent_package() {
        let package = "this_package_should_not_exist";
        let result = deps(package);
        assert!(result.is_err(), "Expected Err, got {result:?}");
    }

    /// Tests the `deps` function with an empty package name.
    /// An empty package name is not valid, and the function is expected to return an error.
    #[test]
    fn test_deps_with_empty_package_name() {
        let package = "";
        let result = deps(package);
        assert!(result.is_err(), "Expected Err, got {result:?}");
    }

    /// Tests the `deps` function with a package name containing special characters.
    /// The behaviour in this case depends on how `cargo bloat` handles special characters,
    /// but typically, this would result in an error.
    #[test]
    fn test_deps_with_special_characters() {
        let package = "!@#$%^&*()";
        let result = deps(package);
        assert!(result.is_err(), "Expected Err, got {result:?}");
    }

    /// Tests the `deps` function with a package name that includes spaces.
    /// Package names with spaces are not valid in Rust, so this should result in an error.
    #[test]
    fn test_deps_with_spaces() {
        let package = "package with spaces";
        let result = deps(package);
        assert!(result.is_err(), "Expected Err, got {result:?}");
    }

    /// Tests that a failing `deps` run keeps the underlying error.
    ///
    /// The failure branch logs through `inspect_err`, which must hand
    /// the command's error on unchanged: the result carries the
    /// context naming the package and, beneath it, the I/O error the
    /// command returned.
    #[test]
    fn test_deps_error_keeps_context_and_cause() {
        let package = "this_package_should_not_exist";
        let err =
            deps(package).expect_err("a missing package must fail");
        let message = err.to_string();
        assert!(
            message.contains("dependency analysis")
                && message.contains(package),
            "unexpected context: {message}"
        );
        assert!(
            err.root_cause().downcast_ref::<std::io::Error>().is_some(),
            "the command's error was replaced: {err:?}"
        );
    }

    /// Tests the `time` function with a nonexistent package name.
    ///
    /// Whether `cargo bloat` is missing or the package is unknown, the
    /// command fails, and `time` must return that failure with its
    /// context and cause rather than swallow it.
    #[test]
    fn test_time_with_nonexistent_package() {
        let package = "this_package_should_not_exist";
        let err =
            time(package).expect_err("a missing package must fail");
        let message = err.to_string();
        assert!(
            message.contains("build time analysis")
                && message.contains(package),
            "unexpected context: {message}"
        );
        assert!(
            err.root_cause().downcast_ref::<std::io::Error>().is_some(),
            "the command's error was replaced: {err:?}"
        );
    }

    /// Tests the `time` function with an empty package name.
    #[test]
    fn test_time_with_empty_package_name() {
        let result = time("");
        assert!(result.is_err(), "Expected Err, got {result:?}");
    }
}
