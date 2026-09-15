#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Convenience wrapper for bridge bootstrap against network.config/localnet-bridge.
# Does not start/stop myso or sidecars — only runs bridge-bootstrap.sh with the
# right MYSO_CONFIG_DIR and network yaml.
#
# Usage (after myso start is healthy):
#   ./scripts/localnet-bridge-bootstrap.sh --fresh-chain --skip-evm
#   ./scripts/localnet-bridge-bootstrap.sh --skip-evm --start-bridge-node

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

export MYSO_CONFIG_DIR="${MYSO_CONFIG_DIR:-$REPO_ROOT/network.config/localnet-bridge}"
export BRIDGE_NETWORK_YAML="${BRIDGE_NETWORK_YAML:-$MYSO_CONFIG_DIR/network.yaml}"
export MYSO_RPC_URL="${MYSO_RPC_URL:-http://127.0.0.1:9000}"
export GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"

exec "${SCRIPT_DIR}/bridge-bootstrap.sh" "$@"
