<!-- SPDX-FileCopyrightText: 2026 Noyalib -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# The `serde_yaml` compatibility contract

A Cargo package rename preserves the import name for the supported APIs.
Compatibility is defined by the tests below. Code using the archived
crate's complete API or value model needs a migration review.

## Tested scope

| Surface | Evidence | Boundary |
|---|---|---|
| Typed Serde parsing and serialization | `tests/drop_in.rs` | Derive-based configuration roundtrips |
| JSON-target resolution, aliases, merge keys and duplicate keys | `tests/behavioural_contract.rs` | Eighteen captured input/output and error cases |
| Error text and `location()` | The same contract target | Pinned error classes; column counts characters, index counts bytes |
| Tagged `Value` scalar, sequence and mapping | Extended contract tests | Shape and roundtrip preservation |
| Maximum unsigned integer and overflow | Extended contract tests | Value precision and error location; overflow wording can differ by target type |
| Binary-looking string keys | Extended contract test and fuzz regression | Serialization preserves the string key |

The eighteen-case corpus originated in an adopter's
[migration evaluation](https://github.com/Takazudo/zudo-front-builder/issues/2787).
Its expected JSON results, errors and locations were captured from
`serde_yaml 0.9.34+deprecated`. The facade now carries those cases directly
in `tests/behavioural_contract.rs`, alongside additional location and shape
checks. The core retains its corresponding contract suite.

```bash
cargo test --test behavioural_contract
cargo test --test drop_in
```

## Value model and limits

`Value`, `Mapping`, `Number` and related types come from noyalib. Mapping
keys are strings. Archived `serde_yaml::Value` can represent composite
mapping keys, so code depending on that representation requires changes.
An overflowing integer parsed into `Value` can also produce different
error wording from the archived implementation even when its location
matches. The JSON-target contract pins its own overflow diagnostic.

The facade uses noyalib's parser budgets. Spec-versus-libyaml acceptance
differences remain possible outside the captured cases. The core's
differential fuzzer compares the compatibility profile against the archive;
findings require a fix or a documented, justified difference.

## Build modes

This standalone facade requires `std`: its manifest explicitly enables
core `std` and `compat-serde-yaml`. Disabling the facade's default features
does not remove that dependency requirement.

Core `noyalib` exposes its compatibility profile separately in alloc-only
builds. Its bare-metal and no_std checks establish that separate contract.
The archived crate is used as a development oracle by the core fuzzer and
is absent from the facade's runtime dependency graph.
