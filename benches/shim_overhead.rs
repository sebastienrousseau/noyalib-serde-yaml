// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! The cost of behavioural compatibility: the shim path (serde_yaml
//! profile) against noyalib's direct path on the same documents.
//! The shim's config disqualifies the streaming fast path by design
//! (merge keys must materialise), so the interesting number is how
//! small that gap is — run `cargo bench` and see.

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const SMALL: &str = "name: gateway\nport: 8443\ntags:\n  - edge\n  - tls\n";

fn medium() -> String {
    let mut s = String::from("services:\n");
    for i in 0..200 {
        s.push_str(&format!(
            "  - name: svc-{i}\n    port: {}\n    replicas: {}\n",
            8000 + i,
            i % 7 + 1
        ));
    }
    s
}

fn bench(c: &mut Criterion) {
    let med = medium();
    let mut g = c.benchmark_group("from_str_value");
    let _ = g.bench_function("shim/small", |b| {
        b.iter(|| {
            noyalib_serde_yaml::from_str::<noyalib_serde_yaml::Value>(black_box(SMALL)).unwrap()
        });
    });
    let _ = g.bench_function("direct/small", |b| {
        b.iter(|| noyalib::from_str::<noyalib::Value>(black_box(SMALL)).unwrap());
    });
    let _ = g.bench_function("shim/medium", |b| {
        b.iter(|| {
            noyalib_serde_yaml::from_str::<noyalib_serde_yaml::Value>(black_box(&med)).unwrap()
        });
    });
    let _ = g.bench_function("direct/medium", |b| {
        b.iter(|| noyalib::from_str::<noyalib::Value>(black_box(&med)).unwrap());
    });
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
