// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use xtasks::tasks::bloat::deps;

    /// Tests the `deps` function with a valid package name.
    /// This test expects the function to complete successfully.
    #[test]
    fn test_deps_with_real_command() {
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
}
