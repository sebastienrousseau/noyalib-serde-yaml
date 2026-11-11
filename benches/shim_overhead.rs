// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! Compare facade and core entry points with identical compatibility semantics.
//! Keep native-versus-compatibility profile costs in a separate group.

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
    let inputs = [("small", SMALL), ("medium", med.as_str())];
    let mut equivalent = c.benchmark_group("equivalent_compatibility");
    for (name, input) in inputs {
        let facade: noyalib_serde_yaml::Value = noyalib_serde_yaml::from_str(input).unwrap();
        let core: noyalib::Value = noyalib::compat::serde_yaml::from_str(input).unwrap();
        assert_eq!(facade, core);
        let _ = equivalent.bench_function(format!("facade/{name}"), |b| {
            b.iter(|| {
                noyalib_serde_yaml::from_str::<noyalib_serde_yaml::Value>(black_box(input)).unwrap()
            });
        });
        let _ = equivalent.bench_function(format!("core/{name}"), |b| {
            b.iter(|| {
                noyalib::compat::serde_yaml::from_str::<noyalib::Value>(black_box(input)).unwrap()
            });
        });
    }
    equivalent.finish();

    let mut profiles = c.benchmark_group("resolver_profiles");
    for (name, input) in inputs {
        let _ = profiles.bench_function(format!("compatibility/{name}"), |b| {
            b.iter(|| {
                noyalib_serde_yaml::from_str::<noyalib_serde_yaml::Value>(black_box(input)).unwrap()
            });
        });
        let _ = profiles.bench_function(format!("native/{name}"), |b| {
            b.iter(|| noyalib::from_str::<noyalib::Value>(black_box(input)).unwrap());
        });
    }
    profiles.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
