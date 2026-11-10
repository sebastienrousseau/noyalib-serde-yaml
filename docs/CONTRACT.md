<!-- SPDX-FileCopyrightText: 2026 Noyalib -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# The `serde_yaml` behavioural contract

"Drop-in replacement" is a behavioural claim, so it is held by a
test suite, not a README sentence.

## Provenance

The corpus began life as an *evaluation harness built by a
prospective adopter* to decide whether noyalib could replace
`serde_yaml` in their build
([zudo-front-builder#2787](https://github.com/Takazudo/zudo-front-builder/issues/2787)).
noyalib 0.0.28 diverged on 11 of its 18 cases and was rejected —
the most rigorous public specification of what `serde_yaml`
compatibility actually means. noyalib v0.0.29 vendors that corpus as
its own contract suite
(`crates/noyalib/tests/integrations/serde_yaml_contract.rs`), with every
expectation captured **live from `serde_yaml 0.9.34+deprecated`**:
the JSON value produced, the error `Display` string, and the
`location()` line/column/index. All 18 pass.

## Enforcement

- The suite runs in the core's `cargo test --all-features` — every
  PR, every platform in the test matrix.
- A differential fuzzer (`fuzz_serde_yaml_compat`) runs the shim
  against the real archived `serde_yaml` on arbitrary inputs,
  hunting for divergences nobody has written down yet; confirmed
  legitimate differences graduate into its documented allowlist,
  everything else is a bug.
- This standalone facade requires `std`: its manifest explicitly enables
  core `std` and `compat-serde-yaml`. Disabling the facade's default
  features does not disable that dependency requirement.
- Core `noyalib` exposes its compatibility profile separately in alloc-only
  builds. The core owns that no_std implementation and its target matrix;
  it is not evidence that this standalone facade is a no_std crate.
