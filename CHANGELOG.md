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
