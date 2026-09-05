<!-- SPDX-FileCopyrightText: 2026 Noyalib -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Architecture

`noyalib-serde-yaml` is deliberately the smallest crate in the family.
Its whole source is one line:

```rust
pub use noyalib::compat::serde_yaml::*;
```

Everything else is contract.

## What the re-export gives you

`noyalib::compat::serde_yaml` is the core library's behavioural
`serde_yaml` 0.9 shim: the same function names, types, and error
phrasing, pinned by an 18-case contract suite captured live from
`serde_yaml 0.9.34` and kept in the core repository
(`tests/serde_yaml_contract.rs`). Publishing that module under a
package name lets Cargo's `package =` rename substitute it for
`serde_yaml` without a single source change downstream.

## Why a separate crate

A dependency rename can only target a crate on the registry with the
right name. The core cannot be renamed, so a one-line crate is the
smallest artefact that makes the migration a Cargo.toml edit. Keeping
it empty of logic means there is nothing here to drift from the core.

## Features

The crate enables exactly `std` and `compat-serde-yaml` on the core,
with default features off, so it pulls nothing the shim does not need.

## Testing

`tests/drop_in.rs` exercises the surface as a renamed dependency would
see it. `docs/CONTRACT.md` records the behavioural guarantees. Two
libFuzzer targets in `fuzz/` feed arbitrary input through the shim's
`from_str` and a `to_string` round trip, and CI replays the seed corpus
on every push. The README's Rust block is compiled in CI against this
crate under the rename.

## Lockstep

The crate pins `noyalib` at the identical `=0.0.X` and releases with
it (core ADR-0005). The exact pin is the compatibility contract: the
behaviour a user gets is the behaviour that version of the core tested.
