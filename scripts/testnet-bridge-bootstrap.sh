#!/usr/bin/env bash
# Testnet bridge planning is read-only by default. See bridge/TESTNET_BRIDGE.md.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$SCRIPT_DIR/lib/testnet_bridge.py" "$@"
