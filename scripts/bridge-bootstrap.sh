#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Attach-only localnet bootstrap for the native MySo bridge.
#
# Default: never start, stop, or kill running services. Submits txs, writes
# configs under network.config/bridge/, and prints how to start the bridge node.
#
# Prerequisites (you already run these):
#   - myso fullnode RPC (MYSO_RPC_URL, default http://127.0.0.1:9000)
#   - optional GraphQL (GRAPHQL_URL, default http://127.0.0.1:9125/graphql)
#   - optional anvil (ETH_RPC_URL, default http://127.0.0.1:8545)
#   - optional bridge node for governance / --demo-transfer
#   - tools: myso, myso-bridge, jq, curl, python3 (+ forge/cast/anvil for EVM)
#   - active wallet with >= 50M MYSO for one-time bootstrap_native_myso
#
# Session: network.config/bridge/bridge-session.env
# Foreign BTC/ETH types here are canonical — orderbook-bootstrap.sh reuses them.
#
# Usage:
#   ./scripts/bridge-bootstrap.sh
#   ./scripts/bridge-bootstrap.sh --skip-evm
#   ./scripts/bridge-bootstrap.sh --skip-governance
#   ./scripts/bridge-bootstrap.sh --configs-only
#   ./scripts/bridge-bootstrap.sh --start-anvil
#   ./scripts/bridge-bootstrap.sh --start-bridge-node
#   ./scripts/bridge-bootstrap.sh --refresh-session
#   ./scripts/bridge-bootstrap.sh --fresh-chain
#   ./scripts/bridge-bootstrap.sh --demo-transfer
#   ASSUME_YES=1 ./scripts/bridge-bootstrap.sh
#
# Opt-in helpers bind 18545 (anvil) and 19291 (bridge node), not 8545/8080.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="${BRIDGE_SESSION_SAVE_PATH:-$REPO_ROOT/network.config/bridge/bridge-session.env}"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/bridge-bootstrap-common.sh
source "${SCRIPT_DIR}/lib/bridge-bootstrap-common.sh"

SKIP_CONFIRM_RUN=1
ASSUME_YES="${ASSUME_YES:-1}"
DO_REFRESH=0
DO_FRESH_CHAIN=0
SKIP_EVM=0
SKIP_GOVERNANCE=0
CONFIGS_ONLY=0
START_ANVIL=0
START_BRIDGE_NODE=0
DEMO_TRANSFER=0

usage() {
    sed -n '2,35p' "$0" | sed 's/^# \?//'
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help)
            usage
            exit 0
            ;;
        --refresh-session)
            DO_REFRESH=1
            shift
            ;;
        --fresh-chain)
            DO_FRESH_CHAIN=1
            shift
            ;;
        --skip-evm)
            SKIP_EVM=1
            shift
            ;;
        --skip-governance)
            SKIP_GOVERNANCE=1
            shift
            ;;
        --configs-only)
            CONFIGS_ONLY=1
            SKIP_GOVERNANCE=1
            shift
            ;;
        --start-anvil)
            START_ANVIL=1
            shift
            ;;
        --start-bridge-node)
            START_BRIDGE_NODE=1
            shift
            ;;
        --demo-transfer)
            DEMO_TRANSFER=1
            shift
            ;;
        -y|--yes)
            ASSUME_YES=1
            shift
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

mkdir -p "$BRIDGE_DIR"
bridge_apply_defaults

if [[ "$DO_FRESH_CHAIN" == 1 ]]; then
    bridge_reset_fresh_chain_state
fi

bridge_load_session

if [[ "$DO_REFRESH" == 1 ]]; then
    log_step "Refresh requested — keys and generated yaml will be replaced after confirm"
    ASSUME_YES="${ASSUME_YES:-0}"
    SKIP_CONFIRM_RUN=0
    if ! confirm_run; then
        echo "Aborted --refresh-session" >&2
        exit 1
    fi
    SKIP_CONFIRM_RUN=1
    ASSUME_YES=1
fi

bridge_preflight
bridge_sync_session_with_chain
bridge_refresh_session_from_graphql || {
    echo "GraphQL admin-cap refresh failed; token publish may fail without ./scripts/bootstrap.sh" >&2
}
bridge_save_session
bridge_ensure_ports
bridge_ensure_keys "$DO_REFRESH"

if [[ "$CONFIGS_ONLY" == 1 ]]; then
    bridge_write_configs
    bridge_save_session
    bridge_print_node_start
    bridge_print_summary
    exit 0
fi

if ! bridge_bootstrap_native_myso; then
    echo "Native MYSO bootstrap failed; continuing with committee and tokens." >&2
fi
bridge_save_session

if [[ "$START_ANVIL" == 1 ]]; then
    bridge_start_anvil_opt_in
fi

if [[ "$SKIP_EVM" == 1 ]]; then
    log_step "Skipping EVM deploy (--skip-evm)"
else
    bridge_deploy_evm
fi

bridge_write_configs
bridge_fund_address "$BRIDGE_AUTHORITY_MYSO_ADDRESS" "$BRIDGE_AUTHORITY_MIN_MIST" || true
bridge_fund_address "$BRIDGE_CLIENT_MYSO_ADDRESS" "$BRIDGE_AUTHORITY_MIN_MIST" || true
bridge_save_session

bridge_register_committee
bridge_save_session
bridge_wait_committee_finalized || true
bridge_write_configs
bridge_save_session

if [[ "$START_BRIDGE_NODE" == 1 ]]; then
    if bridge_committee_node_ready; then
        bridge_start_node_opt_in
    else
        log_step "Committee seat pending — not starting the node yet"
        bridge_print_node_start
    fi
else
    bridge_print_node_start
fi

if [[ "$SKIP_GOVERNANCE" == 1 ]]; then
    log_step "Skipping token publish / governance (--skip-governance)"
else
    # Token publish does not need a seated committee or a running node.
    bridge_register_myso_tokens
    bridge_write_configs
    bridge_save_session
    if bridge_committee_node_ready && bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        bridge_governance_add_tokens_myso || true
        bridge_governance_add_tokens_evm || true
        bridge_save_session
    else
        log_step "Tokens published (or reused). Governance waits until the committee is seated and the node is up."
    fi
fi

if [[ "$DEMO_TRANSFER" == 1 ]]; then
    bridge_demo_transfer
fi

bridge_print_summary
