//! `xtasks` is a collection of building block operations such as copy, remove, confirm, and more
//! for use in Rust project management tasks.
//!
//! This module provides utility functions that abstract over common filesystem operations,
//! making it easier to perform tasks like cleaning up generated files, copying directory contents,
//! and confirming user actions.

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use dialoguer::{theme::ColorfulTheme, Confirm};
use fs_extra as fsx;
use fsx::dir::CopyOptions;
use glob::glob;
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};

// Re-exporting cmd from duct for convenience.
pub use duct::cmd;

/// Removes a single file.
///
/// This function attempts to remove a file located at the specified path.
/// If the file does not exist, it returns an error.
///
/// # Parameters
///
/// - `path`: A generic parameter that implements `AsRef<Path>`, representing the path of the file to remove.
///
/// # Returns
///
/// A `Result` that is `Ok` if the file was successfully removed, or an `Err` wrapping an `anyhow::Error`
/// if an error occurred.
///
/// # Errors
///
/// This function will return an error if the file does not exist or cannot be removed.
pub fn remove_file<P>(path: P) -> AnyResult<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.exists() {
        if path.is_file() {
            fs::remove_file(path).with_context(|| {
                format!("Failed to remove file: {path:?}")
            })
        } else {
            Err(AnyError::msg(format!("Path is not a file: {path:?}")))
        }
    } else {
        Err(AnyError::msg(format!("File does not exist: {path:?}")))
    }
}

/// Checks if a file exists at the given path.
///
/// # Parameters
///
/// - `path`: A generic parameter that implements `AsRef<Path>`, representing the path to check.
///
/// # Returns
///
/// A boolean indicating whether the file exists.
pub fn file_exists<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref().exists()
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
pub fn copy_contents<P, Q>(
    from: P,
    to: Q,
    overwrite: bool,
) -> AnyResult<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut opts = CopyOptions::new();
    opts.content_only = true;
    opts.overwrite = overwrite;

    fsx::dir::copy(&from, &to, &opts).with_context(|| {
        format!(
            "Failed to copy contents from {:?} to {:?}",
            from.as_ref(),
            to.as_ref()
        )
    })
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
        .with_context(|| {
            format!(
                "Failed to get confirmation for question: {question}"
            )
        })
        .map_err(|e| {
            error!("Error during confirmation: {}", e);
            e
        })
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

/// Removes files and directories matching a given glob pattern.
///
/// This function searches for files and directories that match the provided glob pattern and removes them,
/// which is useful for cleaning up temporary or generated files in a project.
///
/// # Parameters
///
/// - `pattern`: The glob pattern used to find files and directories to remove.
///
/// # Returns
///
/// A `Result` that is `Ok` if all files and directories were successfully removed, or an `Err` wrapping an `anyhow::Error`
/// if an error occurred.
///
/// # Errors
///
/// This function will return an error in the following situations:
/// - If the glob pattern is invalid.
/// - If any of the files or directories matching the glob pattern cannot be removed.
pub fn clean_files(pattern: &str) -> AnyResult<()> {
    let entries = glob(pattern)
        .with_context(|| format!("Invalid glob pattern: {pattern}"))?
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| {
            format!("Failed to read glob pattern: {pattern}")
        })?;

    for entry in entries {
        if entry.is_file() {
            match fs::remove_file(&entry) {
                Ok(()) => {
                    info!("Successfully removed file: {:?}", entry);
                }
                Err(e) => {
                    error!("Failed to remove file {:?}: {}", entry, e);
                }
            }
        } else if entry.is_dir() {
            match fs::remove_dir_all(&entry) {
                Ok(()) => {
                    info!(
                        "Successfully removed directory: {:?}",
                        entry
                    );
                }
                Err(e) => error!(
                    "Failed to remove directory {:?}: {}",
                    entry, e
                ),
            }
        }
    }
    Ok(())
}

/// Removes a directory, even if it is not empty.
///
/// # Parameters
///
/// - `path`: A generic parameter that implements `AsRef<Path>`, representing the path of the directory to remove.
///
/// # Returns
///
/// A `Result` that is `Ok` if the directory was successfully removed, or an `Err` wrapping an `anyhow::Error`
/// if an error occurred.
///
/// # Errors
///
/// This function will return an error if the directory does not exist or cannot be removed.
pub fn remove_dir<P>(path: P) -> AnyResult<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.exists() {
        fs::remove_dir_all(path).with_context(|| {
            format!("Failed to remove directory: {path:?}")
        })
    } else {
        Err(AnyError::msg(format!(
            "Directory does not exist: {path:?}"
        )))
    }
}
