<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/noyalib/v1/logos/noyalib.svg" alt="noyalib-serde-yaml logo" width="128" />
</p>

<h1 align="center">noyalib-serde-yaml</h1>

<p align="center">
  Drop-in <code>serde_yaml</code> compatibility backed by the maintained noyalib engine.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/noyalib-serde-yaml/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/noyalib-serde-yaml/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/noyalib-serde-yaml"><img src="https://img.shields.io/crates/v/noyalib-serde-yaml.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Registry" /></a>
  <a href="https://docs.rs/noyalib-serde-yaml"><img src="https://img.shields.io/badge/docs.rs-noyalib--serde--yaml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/noyalib-serde-yaml"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/noyalib-serde-yaml?style=for-the-badge&label=OpenSSF%20Scorecard&logo=openssf" alt="OpenSSF Scorecard" /></a>
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg?style=for-the-badge" alt="License: Apache-2.0 OR MIT" /></a>
  <a href="https://github.com/sebastienrousseau/noyalib-serde-yaml/blob/main/docs/POLICIES.md"><img src="https://img.shields.io/badge/MSRV-1.86.0-93450a.svg?style=for-the-badge&logo=rust" alt="MSRV 1.86.0" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — one Cargo package rename
- [Requirements](#requirements) — toolchain floor and lockstep core
- [Quick Start](#quick-start) — existing serde_yaml code unchanged

**The noyalib-serde-yaml ecosystem**

- [The noyalib-serde-yaml ecosystem](#the-noyalib-serde-yaml-ecosystem) — compatibility and core relationships

**Library reference**

- [Capabilities at a glance](#capabilities-at-a-glance) — the current surface by theme
- [Ecosystem comparison](#ecosystem-comparison) — short matrix; full table at [`docs/COMPARISON.md`](docs/COMPARISON.md)
- [Benchmarks](#benchmarks) — shim overhead; full method at [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md)
- [Features](#features) — compatibility surface
- [Configuration](#configuration) — dependency and local-core setup
- [Examples](#examples) — runnable parity demonstrations

**Operational**

- [When not to use noyalib-serde-yaml](#when-not-to-use-noyalib-serde-yaml) — limitations
- [Development](#development) — make targets, contract tests, CI
- [Security](#security) — guarantees and compliance
- [Documentation](#documentation) — all reference docs
- [Stability guarantees](#stability-guarantees) — compatibility, SemVer, and toolchain discipline
- [License](#license)

---

## Install

### As a Rust library

```toml
[dependencies]
serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.49" }
```

Cargo exposes the dependency under the `serde_yaml` crate name, so existing
`use serde_yaml::...` imports remain unchanged.

## Requirements

- Rust **1.86.0 or newer**.
- The crate pins `noyalib` at exactly `=0.0.49` and releases in lockstep.
- Existing code should target the documented `serde_yaml` 0.9 compatibility
  contract rather than undocumented implementation details.

| Surface | Minimum toolchain | Enforcement |
| :--- | :---: | :--- |
| Library | Rust 1.86.0 | manifest and MSRV CI |
| Tests and benchmarks | Rust 1.86.0 | all-target CI |

## Quick Start

```rust
use serde_yaml::{from_str, to_string};

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    name: String,
    port: u16,
}

let config: Config = from_str("name: gateway\nport: 8443\n")?;
let output = to_string(&config)?;
```

Only the dependency declaration changes during migration.

## The noyalib-serde-yaml ecosystem

| Component | Purpose |
| :--- | :--- |
| `noyalib-serde-yaml` | Package-renamed compatibility facade |
| [`noyalib`](https://github.com/sebastienrousseau/noyalib) | Maintained parser and serializer implementation |
| `serde_yaml` 0.9 | Behavioural reference for the compatibility contract |
| [`noya-cli`](https://github.com/sebastienrousseau/noya-cli) | Command-line migration and validation tools |

## Capabilities at a glance

| Area | Capability | Status |
| :--- | :--- | :--- |
| Migration | Package rename without source edits | Supported |
| Serde | `from_*`, `to_*`, `Value`, `Mapping`, and error surface | Compatible |
| Behaviour | 18-case captured parity contract | CI-gated |
| Safety | Pure Rust with `unsafe` forbidden | Enforced |
| Conformance | Shared YAML test-suite projection | CI-gated |

## Ecosystem comparison

| Option | Source changes | Maintained engine | Legacy quirks retained |
| :--- | :---: | :---: | :---: |
| **noyalib-serde-yaml** | No | Yes | Yes, by contract |
| Direct `noyalib` | Yes | Yes | Opt-in profile only |
| Archived `serde_yaml` | No | No | Yes |

See [`docs/COMPARISON.md`](docs/COMPARISON.md) for the migration trade-offs.

## Benchmarks

The `shim_overhead` Criterion harness compares the compatibility profile with a
direct noyalib path on the same documents.

```bash
cargo bench --bench shim_overhead
```

See [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md) for methodology.

## Features

- Package rename under the `serde_yaml` crate name.
- Familiar value, mapping, serializer, deserializer, and error APIs.
- Captured upstream error text and location behaviour for documented classes.
- Merge-key and scalar-resolution compatibility profile.
- No dependency on the archived `serde_yaml` implementation.

## Configuration

Production manifests must use the exact lockstep version shown above. To test
against an unpublished local core without editing the manifest:

```bash
cargo test --config 'patch.crates-io.noyalib.path="../noyalib/crates/noyalib"'
```

Do not commit a path override to a release branch.

## Examples

- [`drop_in.rs`](examples/drop_in.rs): package-rename migration.
- [`behavioural_parity.rs`](examples/behavioural_parity.rs): headline quirks.
- [`docs/MIGRATION.md`](docs/MIGRATION.md): migration steps and rollback.
- [`docs/CONTRACT.md`](docs/CONTRACT.md): evidence behind behavioural parity.

## When not to use noyalib-serde-yaml

- Use `noyalib` directly for greenfield applications that do not require legacy
  `serde_yaml` behaviour.
- Do not choose this facade when the goal is to remove the legacy resolver and
  error-shape quirks immediately.
- Audit code that depends on undocumented `serde_yaml` internals before
  migrating.

The [detailed README reference](docs/README-REFERENCE.md) retains the parity
table, repository layout, and conformance discussion.

## Development

```bash
make
make test
make clippy
make fmt
cargo test --test behavioural_contract
```

CI runs the compatibility contract, drop-in tests, shared YAML suite, rustdoc,
dependency policy, formatting, and linting. See [`DEVELOPMENT.md`](DEVELOPMENT.md).

## Security

Report vulnerabilities through [`SECURITY.md`](SECURITY.md). The crate forbids
`unsafe` code and inherits noyalib's parser limits and dependency review. The
archived `serde_yaml` crate is not present in the runtime dependency graph.

## Documentation

- [User Manual](https://sebastienrousseau.github.io/noyalib-serde-yaml/manual/)
- [API reference](https://docs.rs/noyalib-serde-yaml)
- [Migration guide](docs/MIGRATION.md)
- [Compatibility contract](docs/CONTRACT.md)
- [Developer documentation](DEVELOPMENT.md)
- [Engineering policies](docs/POLICIES.md)
- [Compliance grade](docs/COMPLIANCE-GRADE.md)
- [Detailed README reference](docs/README-REFERENCE.md)

## Stability guarantees

- During `0.0.x`, the patch component is the breaking-change axis.
- The documented serde_yaml compatibility behaviour is a public contract.
- The exact noyalib pin and satellite version move together.
- The MSRV may rise only on the breaking axis with a changelog explanation.

## License

Licensed under either [Apache License 2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
