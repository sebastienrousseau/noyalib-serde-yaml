// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! What the shim writes, the shim reads back: `to_string` output must
//! re-parse, and for values without NaN it must re-parse to itself.
#![no_main]

use libfuzzer_sys::fuzz_target;
use serde_yaml::Value;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = core::str::from_utf8(data) else { return };
    let Ok(value) = serde_yaml::from_str::<Value>(s) else { return };
    let Ok(out) = serde_yaml::to_string(&value) else { return };
    let back: Value = serde_yaml::from_str(&out).expect("emitted YAML must re-parse");
    // NaN never compares equal to itself; everything else must.
    if !out.to_ascii_lowercase().contains("nan") {
        assert_eq!(back, value, "round trip drift:\n{out}");
    }
});
