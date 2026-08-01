#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo build --release --target wasm32-unknown-unknown --manifest-path "$root/runtime/Cargo.toml"

wasm-bindgen \
  --target web \
  --out-dir "$root/runtime/pkg" \
  "$root/runtime/target/wasm32-unknown-unknown/release/tsc_hk_runtime.wasm"
