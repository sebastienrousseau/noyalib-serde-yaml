//! The package-rename contract: code written against `serde_yaml`
//! 0.9's surface compiles and behaves against this crate unchanged.

// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

// In a migrated project this alias is what Cargo's
// `serde_yaml = { package = "noyalib-serde-yaml", … }` rename
// produces; spelling it here keeps the test bodies byte-identical
// to pre-migration code.
use noyalib_serde_yaml as serde_yaml;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Config {
    name: String,
    port: u16,
}

#[test]
fn typed_round_trip_is_byte_identical_pre_migration_code() {
    let cfg: Config = serde_yaml::from_str("name: noyalib\nport: 8080\n").unwrap();
    assert_eq!(cfg.port, 8080);
    let s = serde_yaml::to_string(&cfg).unwrap();
    let round: Config = serde_yaml::from_str(&s).unwrap();
    assert_eq!(cfg, round);
}

#[test]
fn value_and_error_surfaces_resolve() {
    let v: serde_yaml::Value = serde_yaml::from_str("a: [1, 2]\n").unwrap();
    assert_eq!(v["a"][0].as_i64(), Some(1));

    let err = serde_yaml::from_str::<serde_yaml::Value>("a: [unclosed").unwrap_err();
    let loc = err.location().expect("parse errors carry a location");
    assert!(loc.line() >= 1 && loc.column() >= 1);
    let _: usize = loc.index();
}

#[test]
fn behaves_like_serde_yaml_not_like_noyalib_defaults() {
    // The two spot checks migrants hit first; the full 18-case
    // contract suite lives in the core repository.
    let v: serde_json::Value = serde_yaml::from_str("a: &a {x: 1}\nb:\n  <<: *a\n").unwrap();
    assert!(v["b"].get("<<").is_some(), "merge key stays literal");
    let v: serde_json::Value = serde_yaml::from_str("n: 0123\n").unwrap();
    assert_eq!(v["n"].as_str(), Some("0123"), "leading zero stays a string");
}
