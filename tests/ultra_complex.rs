// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! Document 1 of the core's ultra-complex fixture through the shim's
//! `from_str` (document 2 carries a sequence as a mapping key, which the
//! `serde_yaml` contract refuses): merge keys at two depths resolved by
//! `apply_merge`, explicit tags and a block sequence project onto exactly
//! the expected JSON.

#![allow(missing_docs)]

use noyalib_serde_yaml as serde_yaml;

#[test]
fn document_one_projects_onto_the_expected_json() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ultra-complex");
    let yaml = std::fs::read_to_string(dir.join("valid.yaml")).unwrap();
    let doc1 = &yaml[..yaml.find("---\n# Document 2").expect("document 2")];
    let expected: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(dir.join("valid.json")).unwrap()).unwrap();
    // Like serde_yaml, the shim keeps `<<` until `apply_merge` resolves it.
    let mut value: serde_yaml::Value = serde_yaml::from_str(doc1).expect("document 1 parses");
    value.apply_merge().expect("merge keys resolve");
    assert_eq!(serde_json::to_value(value).unwrap(), expected[0]);
}
