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

/// # `macro_log_info` Macro
#[macro_export]
macro_rules! macro_log_info {
    ($level:expr, $component:expr, $description:expr, $format:expr) => {
        {
            use $crate::loggers::{Log, LogLevel, LogFormat};
            use dtt::DateTime;
            use vrd::Random;

            // Get the current date and time in ISO 8601 format.
            let date = DateTime::new();
            let iso = date.iso_8601;

            // Create a new random number generator
            let mut rng = Random::default();
            let session_id = rng.rand().to_string();

            let log = Log::new(&session_id, &iso, $level, $component, $description, $format);
            let _ = log.log();
            log // Return the Log instance
        }
    };
}
