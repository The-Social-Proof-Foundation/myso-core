#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export DISCOVERY_DATABASE_URL="${DISCOVERY_DATABASE_URL:-postgresql://poc:poc@127.0.0.1:5434/discovery}"
export RUST_LOG="${RUST_LOG:-info}"
cargo run -p myso-discovery-service -- "$@"
