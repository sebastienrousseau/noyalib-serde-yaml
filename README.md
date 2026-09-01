<!-- SPDX-FileCopyrightText: 2026 Noyalib -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# noyalib-serde-yaml

Drop-in [`serde_yaml`](https://crates.io/crates/serde_yaml)
replacement backed by [noyalib](https://crates.io/crates/noyalib) —
**rename the package in `Cargo.toml`, change zero source lines**.

```toml
# Cargo.toml — the whole migration:
serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.29" }
```

Every `use serde_yaml::…` keeps compiling unchanged. And since
noyalib v0.0.29, it keeps **behaving** like `serde_yaml` 0.9 where
migrations actually break:

| serde_yaml 0.9 behaviour | this crate |
| --- | --- |
| `<<:` stays a literal key (alias value resolved) | ✅ identical |
| `0123` is a string, `0b11` is `3` | ✅ identical |
| `1e999` stays the string `"1e999"` | ✅ identical |
| `u64::MAX` keeps full precision; one past it errors `JSON number out of range` | ✅ identical |
| `[a, b]: v` errors `invalid type: sequence, expected a string key` at `1:1:0` | ✅ identical |
| alias bombs error `repetition limit exceeded` (jumps ≤ events × 100) | ✅ identical |
| libyaml error phrasing + end-of-input location convention | ✅ identical for the pinned classes |
| `Error::location()` → 1-based line/column, 0-based index | ✅ identical |

Pinned by an 18-case contract suite whose expectations were captured
**live from `serde_yaml 0.9.34+deprecated`** — values, error text,
and location pins (`noyalib` repo,
`crates/noyalib/tests/serde_yaml_contract.rs`). One documented
partial: a custom tag refused under `deserialize_any` anchors its
location at the value where upstream anchors at the tag; message and
refusal semantics are exact.

## Why move off `serde_yaml`

- `serde_yaml` 0.9 is archived (March 2024); advisories and spec
  fixes no longer flow into it.
- noyalib is pure Rust, `#![forbid(unsafe_code)]`, actively
  maintained, and passes 406/406 of the official YAML test suite.
- This crate never re-introduces the archived crate or its advisory
  chain — every type is noyalib-native under the `serde_yaml` names.
- When you outgrow drop-in: the same engine offers YAML 1.2 strict
  resolution, byte-exact lossless editing (`noyalib::cst`), source
  spans, schema validation, and more — one `use noyalib::…` away.

## Repository layout

| Path | What lives there |
| --- | --- |
| `src/lib.rs` | The whole crate: a re-export of `noyalib::compat::serde_yaml` |
| `tests/drop_in.rs` | Pre-migration `serde_yaml` code, compiled and run verbatim against this crate |
| `examples/drop_in.rs` | The rename migration end to end — `cargo run --example drop_in` |
| `examples/behavioural_parity.rs` | The upstream quirks, reproduced — `cargo run --example behavioural_parity` |
| `benches/shim_overhead.rs` | Shim path vs direct noyalib on the same documents — `cargo bench` |
| `doc/MIGRATION.md` | The one-line migration, its guarantees, and its edges |
| `doc/CONTRACT.md` | Where the 18-case behavioural contract comes from and how it is enforced |

## Developing against an unpublished core

The `=0.0.X` lockstep pin resolves against the *published* core, so
between releases run the suite via a path override:

```sh
cargo test --config 'patch.crates-io.noyalib.path="../noyalib/crates/noyalib"'
```

## Versioning

Releases in strict lockstep with `noyalib` at the identical `=0.0.X`
(ADR-0005). This crate is a re-export of
`noyalib::compat::serde_yaml`; all engineering happens in the core
repository.

## License

MIT OR Apache-2.0.
