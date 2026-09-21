<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Benchmarks

The `shim_overhead` Criterion harness compares the compatibility profile with a
direct noyalib path using the same documents.

```bash
cargo bench --bench shim_overhead
```

Record the CPU, operating system, Rust version, commit, input corpus, and command
with results. The compatibility profile may materialize data required by legacy
merge-key semantics, so compare equivalent behaviour. CI builds benchmark
targets but does not gate releases on shared-runner timing.
