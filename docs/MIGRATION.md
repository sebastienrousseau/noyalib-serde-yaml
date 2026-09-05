<!-- SPDX-FileCopyrightText: 2026 Noyalib -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Migrating from `serde_yaml` — the one-line version

```toml
# Cargo.toml — the whole migration:
serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.33" }
```

Every `use serde_yaml::…` in your codebase keeps compiling and
behaving. That is the entire recipe; the rest of this page is what
"behaving" means and where the edges are.

## What is guaranteed

The shim parses under the core's `ParserConfig::serde_yaml_compat()`
profile and renders errors upstream-style. The 18-case contract
suite in the core repository
(`crates/noyalib/tests/serde_yaml_contract.rs`) pins — with
expectations captured live from `serde_yaml 0.9.34+deprecated` —
values, error `Display` text, and `location()` line/column/index for:
anchors and aliases, merge keys, scalar and composite keys, YAML 1.1
boolean spellings, octal/binary/sexagesimal numbers, null and date
scalars, Unicode/CRLF/emoji, malformed-input locations, built-in and
custom tags, duplicate keys, non-finite and overflowing numbers,
integer boundaries, and alias resource limits.

## The edges

- **Custom tags under `deserialize_any`** refuse with upstream's
  message, but anchor the location at the value rather than the tag.
- **Error wording beyond the pinned classes** may use noyalib's own
  phrasing — upstream's message set is unbounded; the classes real
  code pins (per the contract corpus) are matched.
- **Spec-vs-libyaml acceptance gaps** exist in both directions
  (libyaml rejects spec-legal anchor names like `&\L`; empty
  implicit keys; etc.). noyalib follows the spec — it passes 406/406
  of the official YAML test suite.

## When you outgrow drop-in

The same engine offers YAML 1.2 strict resolution, byte-exact
lossless editing (`noyalib::cst`), source spans, streaming
deserialisation, and schema validation. Depend on `noyalib` directly
(optionally with `features = ["compat-serde-yaml"]` to keep the shim
during a gradual migration), and see the core's
[`docs/MIGRATION-FROM-SERDE-YAML.md`](https://github.com/sebastienrousseau/noyalib/blob/main/docs/MIGRATION-FROM-SERDE-YAML.md)
for the full function-by-function mapping.
