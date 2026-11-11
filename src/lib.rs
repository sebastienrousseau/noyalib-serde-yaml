// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Noyalib. All rights reserved.

//! `serde_yaml` migration facade backed by
//! [noyalib](https://docs.rs/noyalib). Supported Serde entry points can
//! retain their import name through a Cargo package rename.
//!
//! ```toml
//! serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.61" }
//! ```
//!
//! The captured contract covers JSON-target scalar resolution, merge
//! keys, errors and locations. It does not promise every archived
//! `serde_yaml` API or every diagnostic. `Value` and `Mapping` use
//! noyalib's model, including string mapping keys. Check the repository's
//! `docs/CONTRACT.md` before migrating code that depends on these types.
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
#![deny(missing_docs)]

pub use noyalib::compat::serde_yaml::*;
