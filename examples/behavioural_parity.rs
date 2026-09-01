// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! The behavioural half of "drop-in": the quirks `serde_yaml` 0.9
//! migrants depend on, reproduced — not noyalib's spec-strict
//! defaults. The full 18-case contract suite lives in the core
//! repository (`tests/serde_yaml_contract.rs`).
//!
//! Run: `cargo run --example behavioural_parity`

use noyalib_serde_yaml as serde_yaml;

fn main() {
    // `<<` merge keys stay literal entries (alias value resolved) —
    // serde_yaml never implemented the merge.
    let v: serde_json::Value =
        serde_yaml::from_str("base: &b {x: 1}\npost:\n  <<: *b\n  y: 2\n").unwrap();
    println!("merge key : {}", serde_json::to_string(&v["post"]).unwrap());

    // Leading-zero integers stay strings; 1.1 binary resolves.
    let v: serde_json::Value = serde_yaml::from_str("old: 0123\nbin: 0b11\n").unwrap();
    println!("numbers   : old={:?} bin={:?}", v["old"], v["bin"]);

    // A literal float overflow stays the string it was written as.
    let v: serde_json::Value = serde_yaml::from_str("overflow: 1e999\n").unwrap();
    println!("overflow  : {:?}", v["overflow"]);

    // u64::MAX keeps full precision; one past it refuses like upstream.
    let v: serde_json::Value = serde_yaml::from_str("max: 18446744073709551615\n").unwrap();
    println!("u64::MAX  : {}", v["max"]);
    let err =
        serde_yaml::from_str::<serde_json::Value>("over: 18446744073709551616\n").unwrap_err();
    println!("past MAX  : {err}");

    // Alias bombs fail with upstream's exact wording.
    let bomb = "l: &l x\na: &a [*l, *l, *l, *l, *l, *l, *l, *l, *l, *l]\n\
                b: &b [*a, *a, *a, *a, *a, *a, *a, *a, *a, *a]\n\
                c: &c [*b, *b, *b, *b, *b, *b, *b, *b, *b, *b]\n\
                d: &d [*c, *c, *c, *c, *c, *c, *c, *c, *c, *c]\nroot: *d\n";
    let err = serde_yaml::from_str::<serde_json::Value>(bomb).unwrap_err();
    println!("bomb      : {err}");
}
