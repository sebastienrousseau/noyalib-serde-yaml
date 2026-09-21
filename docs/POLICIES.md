<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Engineering policies

## Version and dependency policy

`noyalib-serde-yaml` releases in strict lockstep with `noyalib`. Version 0.0.46
pins the core at exactly `=0.0.46`. Release work uses `feat/v0.0.46`; each
subsequent iteration increments exactly 0.0.1.

## Minimum Rust version

The minimum supported Rust version is 1.86.0, declared in the manifest and
verified by CI. A floor increase is a breaking-axis change and requires a
changelog entry.

## Compatibility

The documented `serde_yaml` behaviour, error display and location shapes,
package-rename path, and exported names are public. Production manifests must
not contain a local path override. During `0.0.x`, the patch component is the
breaking axis.

Family-wide policies live in the core
[`POLICIES.md`](https://github.com/sebastienrousseau/noyalib/blob/main/docs/POLICIES.md).
