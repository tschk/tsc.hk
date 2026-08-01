#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

want="$(grep -A2 'name = "wasm-bindgen"' "$root/runtime/Cargo.lock" | grep '^version' | head -1 | sed 's/version = "\(.*\)"/\1/')"
have="$(wasm-bindgen --version 2>/dev/null | awk '{print $2}' || true)"
if [ "$have" != "$want" ]; then
  cargo install wasm-bindgen-cli --version "$want" --locked --force
fi

cargo build --release --target wasm32-unknown-unknown --manifest-path "$root/runtime/Cargo.toml"

wasm-bindgen \
  --target web \
  --out-dir "$root/runtime/pkg" \
  "$root/runtime/target/wasm32-unknown-unknown/release/tsc_hk_runtime.wasm"
