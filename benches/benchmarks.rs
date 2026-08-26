// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Criterion benchmarks.
//!
//! Only the parts this crate actually controls are measured. Every task
//! shells out to `cargo` or another tool, so benchmarking a task would
//! time that tool and the machine it runs on, producing a precise
//! number that says nothing about this code.
//!
//! What is left is argument dispatch and the filesystem helpers.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use tempfile::tempdir;
use xtasks::ops::{exists, root_dir};

fn ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("ops");

    let _ = group.bench_function("root_dir", |b| {
        b.iter(|| black_box(root_dir()));
    });

    let dir = tempdir().expect("tempdir");
    let present = dir.path().to_path_buf();
    let absent = dir.path().join("does-not-exist");

    let _ = group.bench_function("exists/hit", |b| {
        b.iter(|| black_box(exists(black_box(&present))));
    });
    let _ = group.bench_function("exists/miss", |b| {
        b.iter(|| black_box(exists(black_box(&absent))));
    });

    group.finish();
}

criterion_group!(benches, ops);
criterion_main!(benches);
