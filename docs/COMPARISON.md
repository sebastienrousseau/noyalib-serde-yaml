<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# serde_yaml migration comparison

| Option | Source changes | Maintained engine | Legacy behaviour retained |
| :--- | :---: | :---: | :---: |
| `noyalib-serde-yaml` | No | Yes | Yes, by documented contract |
| Direct `noyalib` | Yes | Yes | Opt-in compatibility profile |
| Archived `serde_yaml` | No | No | Yes |

Use this facade when a reversible package rename and known legacy behaviour are
more important than adopting the native noyalib API immediately. Prefer direct
`noyalib` for greenfield code. The detailed behavioural boundary and captured
evidence are in [the contract](CONTRACT.md).
