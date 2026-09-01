// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! Drop-in `serde_yaml` replacement backed by
//! [noyalib](https://docs.rs/noyalib) — **rename the package in
//! `Cargo.toml`, change zero source lines**.
//!
//! ```toml
//! # Cargo.toml — the whole migration:
//! serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.29" }
//! ```
//!
//! Every `use serde_yaml::…` in the codebase keeps compiling
//! unchanged, and — since noyalib v0.0.29 made the shim
//! *behavioural* — keeps behaving like `serde_yaml` 0.9 where it
//! matters: `<<` merge keys stay literal entries, `0123` stays a
//! string and `0b11` is 3, a literal `1e999` stays a string,
//! `u64::MAX` keeps full precision and one past it refuses as
//! `JSON number out of range`, non-scalar keys refuse as
//! `invalid type: sequence, expected a string key`, alias expansion
//! is budgeted with upstream's `repetition limit exceeded`, and
//! parse errors carry libyaml's phrasing and end-of-input location
//! convention. The 18-case contract suite pinning all of this —
//! with expectations captured live from `serde_yaml 0.9.34` — ships
//! in the core repository (`tests/serde_yaml_contract.rs`).
//!
//! Unlike the archived `serde_yaml` 0.9 this surface is maintained,
//! pure-Rust with `#![forbid(unsafe_code)]`, and never re-introduces
//! the unmaintained crate or its advisory chain as a dependency.
//!
//! # What this crate is
//!
//! A re-export of [`noyalib::compat::serde_yaml`] under a package
//! name that Cargo's `package =` rename can substitute for
//! `serde_yaml`. All engineering lives in noyalib; this crate pins
//! the core at the identical `=0.0.X` and adds nothing else.
//!
//! # Examples
//!
//! ```
//! // In a migrated project this line reads
//! // `use serde_yaml::{from_str, to_string};` — unchanged from
//! // before the migration.
//! use noyalib_serde_yaml::{from_str, to_string};
//!
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
//! struct Config {
//!     name: String,
//!     port: u16,
//! }
//!
//! let cfg: Config = from_str("name: noyalib\nport: 8080\n").unwrap();
//! assert_eq!(cfg.port, 8080);
//! let back = to_string(&cfg).unwrap();
//! let round: Config = from_str(&back).unwrap();
//! assert_eq!(cfg, round);
//! ```

#![forbid(unsafe_code)]

pub use noyalib::compat::serde_yaml::*;
