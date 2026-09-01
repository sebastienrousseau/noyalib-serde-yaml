// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! The package-rename migration, demonstrated: this file is written
//! exactly as it would have been against `serde_yaml` 0.9 — only the
//! `use` alias below stands in for what Cargo's
//! `serde_yaml = { package = "noyalib-serde-yaml", … }` rename does.
//!
//! Run: `cargo run --example drop_in`

use noyalib_serde_yaml as serde_yaml;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Service {
    name: String,
    port: u16,
    tags: Vec<String>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let yaml = "name: gateway\nport: 8443\ntags:\n  - edge\n  - tls\n";

    // Typed deserialisation — unchanged from serde_yaml.
    let svc: Service = serde_yaml::from_str(yaml)?;
    println!("typed     : {svc:?}");

    // Untyped Value access — unchanged.
    let v: serde_yaml::Value = serde_yaml::from_str(yaml)?;
    println!("value     : port = {:?}", v["port"].as_i64());

    // Serialisation — unchanged.
    let out = serde_yaml::to_string(&svc)?;
    println!("emitted   :\n{out}");

    // Error locations — the exact serde_yaml surface.
    let err = serde_yaml::from_str::<serde_yaml::Value>("a: [unclosed").unwrap_err();
    if let Some(loc) = err.location() {
        println!(
            "diagnostic: {err} (line {}, column {}, byte {})",
            loc.line(),
            loc.column(),
            loc.index()
        );
    }
    Ok(())
}
