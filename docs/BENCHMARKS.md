<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Benchmarks

The `shim_overhead` Criterion harness separates two questions:

- `equivalent_compatibility` compares the facade with
  `noyalib::compat::serde_yaml`, using the same values, inputs and profile.
  Setup asserts equal results. The facade re-exports that core entry point.
- `resolver_profiles` compares compatibility and native defaults. Those
  profiles have different behavior, so this group measures a configuration
  tradeoff rather than facade overhead.

```bash
cargo bench --bench shim_overhead
```

Use `cargo bench --bench shim_overhead -- --test` to check the cases without
collecting timing estimates. Record the CPU, operating system, Rust version,
commit, corpus and command with measured results. Run timing experiments on
a quiet host, separate from fuzzing and parallel builds. No timing budget is
inferred from shared-runner results.
