<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/noyalib/v1/logos/noyalib.svg" alt="Noyalib logo" width="128" />
</p>

<h1 align="center">noyalib-serde-yaml</h1>

<p align="center">
  <strong>Drop-in <code>serde_yaml</code> replacement backed by
  noyalib — rename the package in Cargo.toml, change zero source
  lines, keep the behaviour your code depends on.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/noyalib-serde-yaml/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/noyalib-serde-yaml/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/noyalib-serde-yaml"><img src="https://img.shields.io/crates/v/noyalib-serde-yaml.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/noyalib-serde-yaml"><img src="https://img.shields.io/badge/docs.rs-noyalib--serde--yaml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://lib.rs/crates/noyalib-serde-yaml"><img src="https://img.shields.io/badge/lib.rs-noyalib-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/noyalib"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/noyalib?style=for-the-badge&label=OpenSSF%20Scorecard&logo=openssf" alt="OpenSSF Scorecard" /></a>
</p>

---

## Contents

- [Install](#install) — the one-line migration
- [Quick Start](#quick-start) — pre-migration code, unchanged
- [Why this approach?](#why-this-approach) — design rationale
- [Behavioural parity](#behavioural-parity) — what "drop-in" guarantees
- [Examples](#examples) — runnable demonstrations
- [Benchmarks](#benchmarks) — the cost of compatibility
- [Repository layout](#repository-layout)
- [Developing against an unpublished core](#developing-against-an-unpublished-core)
- [When not to use noyalib-serde-yaml](#when-not-to-use-noyalib-serde-yaml)
- [Documentation](#documentation)
- [License](#license)

---

## Install

```toml
# Cargo.toml — the whole migration:
serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.33" }
```

Every `use serde_yaml::…` in your codebase keeps compiling — Cargo's
package rename substitutes this crate under the old name, so the
diff is one manifest line.

**MSRV: Rust 1.86.0** — matching the noyalib core floor. Releases
ship in strict lockstep with the core at the identical `=0.0.X`
(ADR-0005); the exact pin is the compatibility contract.

## Quick Start

This is `serde_yaml` 0.9 code, byte for byte — only the `Cargo.toml`
line above has changed:

```rust
use serde_yaml::{from_str, to_string};

#[derive(serde::Serialize, serde::Deserialize)]
struct Config { name: String, port: u16 }

let cfg: Config = from_str("name: gateway\nport: 8443\n")?;
let out = to_string(&cfg)?;

let err = from_str::<serde_yaml::Value>("a: [unclosed").unwrap_err();
let loc = err.location().unwrap(); // 1-based line/column, 0-based index
```

## Why this approach?

`serde_yaml` 0.9 was archived in March 2024; advisories and spec
corrections no longer flow into it. Every successor asks you to edit
call sites — and then behaves differently anyway. This crate takes
the opposite contract on both counts:

- **Zero source changes.** The migration is a package rename in one
  manifest line, reversible with the same line.
- **Behaviour, pinned.** Since noyalib v0.0.29 the shim this crate
  re-exports is *behavioural*: it parses under the core's
  `serde_yaml_compat()` profile and renders errors upstream-style,
  held by an 18-case contract suite whose expectations were captured
  **live from `serde_yaml 0.9.34+deprecated`** — values, error
  `Display` text, and `location()` pins.
- **No dead branch.** The archived crate and its advisory chain are
  never dependencies — every type is noyalib-native under the
  `serde_yaml` names, pure Rust, `#![forbid(unsafe_code)]`.

## Behavioural parity

| `serde_yaml` 0.9 behaviour | this crate |
| :--- | :---: |
| `<<:` stays a literal key (alias value resolved) | ✅ |
| `0123` is a string, `0b11` is `3` | ✅ |
| `1e999` stays the string `"1e999"` | ✅ |
| `u64::MAX` keeps full precision; one past it errors `JSON number out of range` | ✅ |
| `[a, b]: v` errors `invalid type: sequence, expected a string key` at `1:1:0` | ✅ |
| alias bombs error `repetition limit exceeded` (jumps ≤ events × 100) | ✅ |
| libyaml error phrasing and end-of-input location convention | ✅ pinned classes |
| `Error::location()` → 1-based line/column, 0-based index | ✅ |

One documented partial: a custom tag refused under `deserialize_any`
anchors its location at the value where upstream anchors at the tag —
message and refusal semantics are exact. Full provenance and
enforcement: [`docs/CONTRACT.md`](docs/CONTRACT.md).

## Examples

```bash
cargo run --example drop_in             # the rename migration end to end
cargo run --example behavioural_parity  # every headline quirk, live
```

## Benchmarks

```bash
cargo bench   # shim path vs direct noyalib on the same documents
```

The shim's profile deliberately bypasses noyalib's streaming fast
path (merge keys must materialise), so the number worth watching is
how small that gap is on your documents.

## Repository layout

| Path | What lives there |
| :--- | :--- |
| `src/lib.rs` | The whole crate: a re-export of `noyalib::compat::serde_yaml` |
| `tests/drop_in.rs` | Pre-migration `serde_yaml` code, compiled and run verbatim |
| `examples/` | The migration and the parity quirks, runnable |
| `benches/shim_overhead.rs` | Shim path vs direct noyalib |
| `docs/MIGRATION.md` | The one-line migration, its guarantees, and its edges |
| `docs/CONTRACT.md` | Where the 18-case behavioural contract comes from |

## Developing against an unpublished core

Between lockstep releases the `=0.0.X` pin can point at a core
version crates.io does not have yet. During that window a
`[patch.crates-io]` section resolving it against the core's release
branch is committed so CI and local builds work; **the patch is
removed at release time** (the release validation refuses to tag
while it is present). To develop against your local checkout
instead:

```bash
cargo test --config 'patch.crates-io.noyalib.path="../noyalib/crates/noyalib"'
```

## When not to use noyalib-serde-yaml

- **Greenfield code.** Depend on [`noyalib`](https://crates.io/crates/noyalib)
  directly — YAML 1.2 strict resolution, byte-exact lossless editing
  (`noyalib::cst`), source spans, streaming deserialisation, and
  schema validation, without the legacy quirks this crate exists to
  reproduce.
- **You want the quirks gone.** The same engine minus the profile is
  one `use noyalib::…` away; migrate gradually with the core's
  [`compat-serde-yaml` feature](https://github.com/sebastienrousseau/noyalib/blob/main/docs/MIGRATION-FROM-SERDE-YAML.md).

## Documentation

The four entry points, identical across every repo in the family:

- **[User Manual](https://sebastienrousseau.github.io/noyalib/manual/)** — the rendered book: user guide, migrations, architecture, policies, ADRs
- **[API reference](https://docs.rs/noyalib-serde-yaml)** — rustdoc on docs.rs
- **[Developer docs](DEVELOPMENT.md)** — this repo's dev entry point, pointing at the family guide
- **[Ecosystem map](https://github.com/sebastienrousseau/noyalib/blob/main/docs/ECOSYSTEM.md)** — the six crates, the lockstep model, the scorecard

- **Migration guide** — [`docs/MIGRATION.md`](docs/MIGRATION.md), and the
  core repository's
  [`MIGRATION-FROM-SERDE-YAML.md`](https://github.com/sebastienrousseau/noyalib/blob/main/docs/MIGRATION-FROM-SERDE-YAML.md)
  for the function-by-function mapping.
- **Behavioural contract** — [`docs/CONTRACT.md`](docs/CONTRACT.md).
- **API reference** — [docs.rs/noyalib-serde-yaml](https://docs.rs/noyalib-serde-yaml).
- **Engineering policies** — the core repository's
  [`POLICIES.md`](https://github.com/sebastienrousseau/noyalib/blob/main/docs/POLICIES.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.
