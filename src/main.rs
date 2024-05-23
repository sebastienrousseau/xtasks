//! This is the main function of the binary. It calls the `main` function within the `tasks` module of the `xtasks` crate.
//!
//! This is the primary functionality of the binary. The `xtasks::tasks::main()` function is expected to perform the main operations of the binary and return a `Result`.
//!
//! If it returns an `Err`, that error will propagate out of this `main` function.
//!
//! # Arguments
//!
//! This function does not take any arguments.
//!
//! # Return Value
//!
//! This function returns a `Result` containing either `()` (no error) or an `anyhow::Error` (an error occurred).
//!
//! # Panics
//!
//! This function does not panic.
//!
//! # Examples
//!
//! ```rust
//! # use anyhow::Result;
//! # use xtasks::tasks;
//! # fn main() -> Result<()> {
//! #     tasks::main()
//! # }
//! ```

fn main() -> Result<(), anyhow::Error> {
    // This line calls a function `main` within the `tasks` module of the `xtasks` crate.
    // This is the primary functionality of this binary. The `xtasks::tasks::main()` function
    // is expected to perform the main operations of this binary and return a `Result`.
    // If it returns an `Err`, that error will propagate out of this `main` function.
    xtasks::tasks::main()
}
