// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! # Macros
//!
//! This module provides convenient macros for common operations in the library.

/// Prints a formatted message to the console with a newline.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        std::println!($($arg)*);
    };
}

/// Prints a formatted message to the console without a newline.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        std::print!($($arg)*);
    };
}

/// Asserts that a condition is true, and panics with a formatted message if it is not.
#[macro_export]
macro_rules! assert {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            std::panic!("Assertion failed: {}", std::format!($($arg)*));
        }
    };
}
