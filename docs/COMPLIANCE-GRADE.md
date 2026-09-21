<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Repository standard compliance grade

Assessment date: 2026-09-20. Source rubric:
`/Users/seb/Code/REPO-STANDARD.md`. This is a conservative static assessment of
checked-in evidence, not a substitute for successful CI runs.

## Result

**14/24 signals (58%). README template: pass. Strict candidate tier: L1.**

| Category | Signals | Summary |
| :--- | :---: | :--- |
| Identity and README | 3/3 | Canonical structure and compatibility policy present |
| Documentation | 2/3 | Manual, architecture, migration, and contract docs present |
| Build and install UX | 1/3 | Native Cargo build present; CLI-generation requirements are not applicable evidence |
| Releases and binaries | 2/3 | Registry automation present; complete L3 evidence remains open |
| Packaging | 1/3 | Registry distribution only; wider tracking and reproducibility are open |
| CI quality gates | 2/3 | Matrix and coverage signals present |
| Supply chain | 2/3 | Foundation controls present; advisory-audit detection remains open |
| Community | 1/3 | Foundation governance present; L2 template and docs-lint signals are open |

The cumulative tier remains L1. Priority work is distribution tracking,
reproducible-build evidence, explicit advisory-audit CI, and audit-visible
community templates and markdown lint enforcement.
