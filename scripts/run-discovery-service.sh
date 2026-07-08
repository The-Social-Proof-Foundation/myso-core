#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Run myso-discovery-service locally from the repo root.
#
# Session: network.config/discovery/discovery-session.env
#
# Usage:
#   ./scripts/run-discovery-service.sh
#   ./scripts/run-discovery-service.sh --refresh-session   # via discovery-runnable.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

DISCOVERY_SESSION_FILE="$REPO_ROOT/network.config/discovery/discovery-session.env"
if [[ -f "$DISCOVERY_SESSION_FILE" ]]; then
  # shellcheck disable=SC1090
  source "$DISCOVERY_SESSION_FILE"
fi

export DISCOVERY_DATABASE_URL="${DISCOVERY_DATABASE_URL:-postgresql://poc:poc@127.0.0.1:5434/discovery}"
export DISCOVERY_SOURCES_CONFIG="${DISCOVERY_SOURCES_CONFIG:-crates/myso-discovery-service/config/discovery/sources.localnet.yaml}"
export DISCOVERY_EMBED_ENABLED="${DISCOVERY_EMBED_ENABLED:-false}"
export RUST_LOG="${RUST_LOG:-info}"

exec cargo run -p myso-discovery-service -- "$@"
