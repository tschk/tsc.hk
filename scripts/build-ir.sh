#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo run --release --manifest-path "$root/ir-gen/Cargo.toml" -- \
  "$root/index.crepus" \
  "$root/src/generated/view-ir.json"
