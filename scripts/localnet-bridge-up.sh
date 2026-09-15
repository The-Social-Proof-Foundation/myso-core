#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Localnet bridge supervisor — one command, runs until Ctrl+C.
#
#   ./scripts/localnet-bridge-up.sh
#
# Starts myso + anvil + bridge node, syncs chain via GraphQL, bootstraps idempotently,
# and keeps watching until you cancel. New chain vs same chain is detected automatically.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

export MYSO_CONFIG_DIR="${MYSO_CONFIG_DIR:-$REPO_ROOT/network.config/localnet-bridge}"
export MYSO_RPC_URL="${MYSO_RPC_URL:-http://127.0.0.1:9000}"
export GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
export ETH_RPC_URL="${ETH_RPC_URL:-http://127.0.0.1:8545}"

SOCIAL_SESSION_SAVE_PATH="${BRIDGE_SESSION_SAVE_PATH:-$REPO_ROOT/network.config/bridge/bridge-session.env}"

# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/bridge-bootstrap-common.sh
source "${SCRIPT_DIR}/lib/bridge-bootstrap-common.sh"
# shellcheck source=lib/localnet-bridge-stack-common.sh
source "${SCRIPT_DIR}/lib/localnet-bridge-stack-common.sh"

SKIP_CONFIRM_RUN=1
ASSUME_YES=1

STACK_SHUTDOWN=0
STACK_TICK_SECS="${STACK_TICK_SECS:-15}"

stack_on_signal() {
    [[ "$STACK_SHUTDOWN" == 1 ]] && exit 0
    STACK_SHUTDOWN=1
    echo "" >&2
    log_step "Shutting down (Ctrl+C) — stopping myso, anvil, bridge node"
    stack_stop_all
    exit 0
}

trap stack_on_signal INT TERM

mkdir -p "$STACK_DIR" "$BRIDGE_DIR"

log_step "Localnet bridge supervisor — runs until Ctrl+C"
log_step "Logs: $STACK_DIR/"

# ── Initial bring-up ──────────────────────────────────────────────────────────
stack_ensure_genesis_if_needed || true
stack_ensure_myso
stack_ensure_anvil
stack_wait_graphql 300 || true

# ── Supervisory loop ──────────────────────────────────────────────────────────
while [[ "$STACK_SHUTDOWN" == 0 ]]; do
    stack_ensure_myso
    stack_ensure_anvil

    if stack_graphql_up; then
        if stack_sync_chain_session; then
            stack_handle_new_chain
        fi
        stack_tick_bootstrap
    elif stack_rpc_up; then
        log_step "RPC up, GraphQL not yet — waiting for indexer"
    fi

    stack_ensure_bridge_node
    stack_print_status_line

    for _ in $(seq 1 "$STACK_TICK_SECS"); do
        [[ "$STACK_SHUTDOWN" == 1 ]] && break
        sleep 1
    done
done
