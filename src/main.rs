// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This line is a copyright notice. It's important for intellectual property rights and
// indicates the year of creation and the owner of the source code (in this case, xtasks).
// The SPDX identifier specifies the license under which this code is distributed,
// which in this case is either Apache-2.0 or MIT.

// The `extern crate` statement is used to link a crate to this binary. In Rust 2018 edition
// and later, this is often optional due to automatic crate linking. However, including it
// explicitly can make dependencies clearer. Here, it links the `xtasks` crate, presumably
// a library crate part of the same project.
extern crate xtasks;

// The `main` function is the entry point of the Rust binary. The `fn main() -> Result<(), E>`
// signature is a common pattern for binaries that may return an error. Here, `E` is specified
// as `anyhow::Error`, which means the function can return any error that implements the
// `std::error::Error` trait, providing flexibility in error handling.
fn main() -> Result<(), anyhow::Error> {
    // This line calls a function `main` within the `tasks` module of the `xtasks` crate.
    // This is the primary functionality of this binary. The `xtasks::tasks::main()` function
    // is expected to perform the main operations of this binary and return a `Result`.
    // If it returns an `Err`, that error will propagate out of this `main` function.
    xtasks::tasks::main()
}
