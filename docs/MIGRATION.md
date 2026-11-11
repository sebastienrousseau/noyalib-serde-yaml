<!-- SPDX-FileCopyrightText: 2026 Noyalib -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Migrating from `serde_yaml`

Rename the dependency for code using the supported Serde entry points:

```toml
serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.61" }
```

This branch prepares an unpublished version. Use a published matching
version for registry installation until it ships.

Keep the `serde_yaml` import name, then run the application's tests. Check
[the compatibility contract](CONTRACT.md) for the captured behaviors and
remaining differences. In particular, review code that manipulates
`Value` or `Mapping`, uses composite keys, or compares diagnostics outside
the pinned classes. A package rename alone does not establish compatibility
for those uses.

The local `behavioural_contract` target tests eighteen captured cases and
additional shape and location checks. Its JSON-target custom-tag refusal
anchors the location at the tag. Error wording outside the pinned classes
can differ. The core's YAML test suite and differential fuzzer are separate
conformance evidence.

To roll back, restore the previous dependency declaration and lockfile,
then rerun the application's tests.

For YAML 1.2 resolution, CST editing, source spans, parser configuration or
alloc-only support, depend on `noyalib` directly. See the core's
[migration guide](https://github.com/sebastienrousseau/noyalib/blob/main/docs/MIGRATION-FROM-SERDE-YAML.md)
for the function mapping.
