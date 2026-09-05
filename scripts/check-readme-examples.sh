#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Noyalib
# SPDX-License-Identifier: MIT OR Apache-2.0
#
# Compile every ```rust block in the README against this crate, the
# way a user who followed the package-rename instruction would use it:
# `serde_yaml` resolves to noyalib-serde-yaml. A block without its own
# `fn main` is wrapped in one that returns a boxed error, so `?` works.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
README="${README:-README.md}"
[ -f "$README" ] || { echo "no $README" >&2; exit 1; }
SCRATCH="${CARGO_TARGET_DIR:-target}/readme-doctest-scratch"
rm -rf "$SCRATCH"; mkdir -p "$SCRATCH/src"
HERE="$(pwd)"
cat > "$SCRATCH/Cargo.toml" <<TOML
[package]
name = "readme-doctest-scratch"
version = "0.0.0"
edition = "2021"
publish = false
[dependencies]
serde_yaml = { package = "noyalib-serde-yaml", path = "$HERE" }
serde = { version = "1.0", features = ["derive"] }
[workspace]
TOML
# A pre-release branch resolves the core through a [patch.crates-io]
# git source; patches apply only at a workspace root, so the scratch
# crate carries the same section until the cut removes it.
if command grep -q '^\[patch.crates-io\]' Cargo.toml; then
  sed -n '/^\[patch.crates-io\]/,$p' Cargo.toml >> "$SCRATCH/Cargo.toml"
fi
n=0; fail=0; block=""; inblk=0; start=0; line_no=0
compile() {
  local body="$1" at="$2"; n=$((n + 1))
  local src
  if command grep -q '^fn main' <<< "$body"; then src="$body"
  else src="fn main() -> Result<(), Box<dyn std::error::Error>> {
$body
    Ok(())
}"; fi
  printf '%s\n' "$src" > "$SCRATCH/src/main.rs"
  local out
  if out=$(cargo build --manifest-path "$SCRATCH/Cargo.toml" --quiet 2>&1); then
    printf '  [ OK  ] block #%d @ %s:%d\n' "$n" "$README" "$at"
  else
    printf '  [FAIL ] block #%d @ %s:%d\n%s\n%s\n' "$n" "$README" "$at" "$src" "$out" >&2
    fail=$((fail + 1))
  fi
}
while IFS= read -r line; do
  line_no=$((line_no + 1))
  if [ "$inblk" -eq 0 ]; then
    if [ "$line" = '```rust' ]; then inblk=1; block=""; start=$line_no; fi
  elif [ "$line" = '```' ]; then
    compile "$block" "$start"; inblk=0; block=""
  else
    block="$block
$line"
  fi
done < "$README"
[ "$n" -gt 0 ] || { echo "  [FAIL] no rust blocks found: this check would be vacuous" >&2; exit 1; }
if [ "$fail" -gt 0 ]; then echo "── $fail of $n README block(s) failed to compile ──" >&2; exit 1; fi
echo "── All $n README block(s) compile clean ──"
