// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `xtaskops` is a collection of building block operations such as copy, remove, confirm, and more
//! for use in Rust project management tasks.
//!
//! This module provides utility functions that abstract over common filesystem operations, 
//! making it easier to perform tasks like cleaning up generated files, copying directory contents, 
//! and prompting the user for confirmation.
//!
//! # Examples
//!
//! ```
//! use xtaskops::ops::clean_files;
//!
//! fn main() -> anyhow::Result<()> {
//!     clean_files("target/debug/*.tmp")?;
//!     Ok(())
//! }
//! ```

use anyhow::{Result as AnyResult, Error as AnyError};
use dialoguer::{theme::ColorfulTheme, Confirm};
use fs_extra as fsx;
use fsx::dir::CopyOptions;
use glob::glob;
use std::path::{Path, PathBuf};

// Re-exporting cmd from duct for convenience.
pub use duct::cmd;

/// Removes files matching a given glob pattern.
///
/// This function searches for files that match the provided glob pattern and removes them,
/// which is useful for cleaning up temporary or generated files in a project.
///
/// # Parameters
///
/// - `pattern`: The glob pattern used to find files to remove.
///
/// # Returns
///
/// A `Result` that is `Ok` if all files were successfully removed, or an `Err` wrapping an `anyhow::Error`
/// if an error occurred.
///
/// # Errors
///
/// This function will return an error in the following situations:
/// - If the glob pattern is invalid.
/// - If any of the files matching the glob pattern cannot be removed.
///
/// # Examples
///
/// ```
/// use xtaskops::ops::clean_files;
///
/// # fn run() -> anyhow::Result<()> {
/// clean_files("target/debug/*.tmp")?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
pub fn clean_files(pattern: &str) -> AnyResult<()> {
    let files = glob(pattern)
        .map_err(AnyError::new)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(AnyError::new)?;

    files.into_iter().try_for_each(remove_file)
}

/// Removes a single file.
///
/// # Parameters
///
/// - `path`: The path of the file to remove.
///
/// # Returns
///
/// A `Result` that is `Ok` if the file was successfully removed, or an `Err` wrapping an `anyhow::Error`
/// if an error occurred.
///
/// # Errors
///
/// This function will return an error if the file removal fails.
pub fn remove_file<P>(path: P) -> AnyResult<()>
where
    P: AsRef<Path>,
{
    fsx::file::remove(&path).map_err(AnyError::new)
}

/// Removes a directory along with its contents.
///
/// # Parameters
///
/// - `path`: The path of the directory to remove.
///
/// # Returns
///
/// A `Result` that is `Ok` if the directory was successfully removed, or an `Err` wrapping an `anyhow::Error`
/// if an error occurred.
///
/// # Errors
///
/// This function will return an error if the directory removal fails.
pub fn remove_dir<P>(path: P) -> AnyResult<()>
where
    P: AsRef<Path>,
{
    fsx::dir::remove(&path).map_err(AnyError::new)
}

/// Checks if a given path exists.
///
/// # Parameters
///
/// - `path`: The path to check.
///
/// # Returns
///
/// `true` if the path exists, `false` otherwise.
pub fn exists<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    Path::exists(path.as_ref())
}

/// Copies the entire contents of a folder to another location.
///
/// # Parameters
///
/// - `from`: The source directory path.
/// - `to`: The destination directory path.
/// - `overwrite`: A boolean indicating whether to overwrite existing files in the destination.
///
/// # Returns
///
/// A `Result` that is `Ok(u64)` representing the total number of bytes copied, or an `Err` wrapping
/// an `anyhow::Error` if an error occurred.
///
/// # Errors
///
/// This function will return an error if any file operation fails.
pub fn copy_contents<P, Q>(from: P, to: Q, overwrite: bool) -> AnyResult<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut opts = CopyOptions::new();
    opts.content_only = true;
    opts.overwrite = overwrite;
    fsx::dir::copy(&from, &to, &opts).map_err(AnyError::new)
}

/// Prompts the user to confirm an action.
///
/// # Parameters
///
/// - `question`: The question to present to the user.
///
/// # Returns
///
/// A `Result` that is `Ok(bool)` representing the user's confirmation (true if confirmed, false otherwise),
/// or an `Err` wrapping an `anyhow::Error` if an input interaction fails.
///
/// # Errors
///
/// This function will return an error if the input interaction fails.
pub fn confirm(question: &str) -> AnyResult<bool> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .interact()
        .map_err(AnyError::new)
}

/// Retrieves the root directory of the cargo project.
///
/// This function assumes that it is called from a binary located in the same cargo workspace,
/// and it will return the path to the workspace root.
///
/// # Returns
///
/// A `PathBuf` representing the root directory of the cargo project.
pub fn root_dir() -> PathBuf {
    let mut xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.pop();
    xtask_dir
}
