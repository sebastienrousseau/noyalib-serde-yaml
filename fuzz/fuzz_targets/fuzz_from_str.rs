// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! The shim's `from_str` must never panic on any input: every failure
//! is an `Err` carrying serde_yaml 0.9's phrasing and a location.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = core::str::from_utf8(data) else { return };
    if let Err(e) = serde_yaml::from_str::<serde_yaml::Value>(s) {
        // Every error renders and carries a location the way the
        // contract suite pins it; neither may panic.
        let _ = e.to_string();
        let _ = e.location();
    }
});
