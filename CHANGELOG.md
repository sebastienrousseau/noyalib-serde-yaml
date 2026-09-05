<!-- SPDX-FileCopyrightText: 2026 Noyalib -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Changelog

All notable changes to `noyalib-serde-yaml` are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
and versions in strict lockstep with the
[`noyalib`](https://github.com/sebastienrousseau/noyalib) core crate
(ADR-0005) — see that repository's `CHANGELOG.md` for release-wide
notes.

## [Unreleased]

## [v0.0.33] - 2026-09-05

### Changed

- Lockstep release with noyalib 0.0.33: bracket-quoted path segments
  (core #389), located duplicate-key errors (core #393), and serializer
  fixes for tag-like keys, non-printable characters, and block scalars
  (core #381, #391, #392). No local code change.

## [v0.0.32] - 2026-09-03

### Changed

- Lockstep release with noyalib 0.0.32: block sequence spans report
  their full extent (core #375). No local behaviour change.

## [v0.0.31] - 2026-09-03

### Changed

- **Repository layout, Phase 1 of the family structure plan**:
  `doc/` renamed to `docs/`, `DEVELOPMENT.md` added as the developer
  entry point, `.editorconfig` / `.markdownlint.yaml` /
  `.codespellrc` land with a per-push `docs-lint` CI gate consuming
  the core repo's shared-docs-lint.yml.

## [v0.0.30] - 2026-09-02

### Changed

- Lockstep release with noyalib 0.0.30 (exact serde_yaml location
  parity: tagged/anchored node spans anchor at their properties;
  the `custom-explicit-tag` contract case now pins `1:8:7`). No
  satellite-local changes.

## [v0.0.29] - 2026-09-01

### Added

- **Initial release: the package-rename drop-in.** A re-export of
  `noyalib::compat::serde_yaml` under a package name Cargo can
  substitute for `serde_yaml`:

  ```toml
  serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.29" }
  ```

  Zero source changes, with behavioural parity pinned by the core's
  18-case `serde_yaml` contract suite (expectations captured live
  from `serde_yaml 0.9.34+deprecated`). Ships when the core's
  v0.0.29 ships; the `=0.0.29` pin resolves only against the
  published core, so CI on this repository is red by design until
  that release lands on crates.io.
