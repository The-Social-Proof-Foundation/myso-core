#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Supervisor helpers for scripts/localnet-bridge-up.sh (long-running, no flags).

if [[ -n "${_LOCALNET_BRIDGE_STACK_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_LOCALNET_BRIDGE_STACK_SOURCED=1

: "${REPO_ROOT:?REPO_ROOT required}"
: "${MYSO_CONFIG_DIR:=$REPO_ROOT/network.config/localnet-bridge}"
: "${BRIDGE_DIR:=$REPO_ROOT/network.config/bridge}"
: "${STACK_DIR:=$BRIDGE_DIR/stack}"

readonly STACK_DEFAULT_RPC='http://127.0.0.1:9000'
readonly STACK_DEFAULT_GRAPHQL='http://127.0.0.1:9125/graphql'
readonly STACK_DEFAULT_ANVIL='http://127.0.0.1:8545'
readonly STACK_DEFAULT_MYSO_START_FLAGS='--with-faucet --with-indexer --with-graphql=9125'

MYSO_RPC_URL="${MYSO_RPC_URL:-$STACK_DEFAULT_RPC}"
GRAPHQL_URL="${GRAPHQL_URL:-$STACK_DEFAULT_GRAPHQL}"
ETH_RPC_URL="${ETH_RPC_URL:-$STACK_DEFAULT_ANVIL}"
MYSO_START_FLAGS="${MYSO_START_FLAGS:-$STACK_DEFAULT_MYSO_START_FLAGS}"

STACK_LAST_CHAIN=''
STACK_NEEDS_FRESH_BOOTSTRAP=0
STACK_BOOTSTRAP_PHASE=0
STACK_PHASE_FAILS=0
STACK_PHASE_FAIL_CAP=2
STACK_TICK_N=0
STACK_NODE_RELOADED_FOR_TOKENS=0

stack_pid_file() {
    printf '%s/%s.pid' "$STACK_DIR" "$1"
}

stack_log_file() {
    printf '%s/%s.log' "$STACK_DIR" "$1"
}

stack_is_running() {
    local name="$1" pidf pid
    pidf="$(stack_pid_file "$name")"
    [[ -f "$pidf" ]] || return 1
    pid="$(cat "$pidf" 2>/dev/null)" || return 1
    [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null
}

stack_stop_one() {
    local name="$1" pidf pid
    pidf="$(stack_pid_file "$name")"
    [[ -f "$pidf" ]] || return 0
    pid="$(cat "$pidf" 2>/dev/null)" || true
    rm -f "$pidf"
    [[ -n "$pid" ]] && kill "$pid" 2>/dev/null || true
}

stack_rpc_up() {
    curl -sf --connect-timeout 2 --max-time 5 "$MYSO_RPC_URL" \
        -H 'Content-Type: application/json' \
        -d '{"jsonrpc":"2.0","id":1,"method":"myso_getLatestMySoSystemState","params":[]}' \
        >/dev/null 2>&1
}

stack_wait_rpc() {
    local i max="${1:-180}"
    for ((i = 1; i <= max; i++)); do
        stack_rpc_up && return 0
        [[ $((i % 10)) -eq 0 ]] && log_wait_progress "MySo RPC" "$i" "$max"
        sleep 1
    done
    return 1
}

stack_graphql_up() {
    graphql_is_reachable "$GRAPHQL_URL"
}

stack_wait_graphql() {
    local i max="${1:-180}"
    for ((i = 1; i <= max; i++)); do
        stack_graphql_up && return 0
        [[ $((i % 10)) -eq 0 ]] && log_wait_progress "GraphQL" "$i" "$max"
        sleep 1
    done
    return 1
}

stack_anvil_up() {
    bridge_eth_rpc_reachable "$ETH_RPC_URL"
}

stack_ensure_anvil() {
    local anvil_bin logf
    stack_anvil_up && return 0
    if stack_is_running anvil; then
        stack_wait_anvil 15 && return 0
        stack_stop_one anvil
    fi
    anvil_bin="$(bridge_resolve_anvil || true)"
    [[ -n "$anvil_bin" ]] || {
        echo "anvil not found (install Foundry)" >&2
        return 1
    }
    mkdir -p "$STACK_DIR"
    logf="$(stack_log_file anvil)"
    log_step "Starting anvil → $logf"
    nohup "$anvil_bin" --port 8545 --block-time 1 >"$logf" 2>&1 &
    echo "$!" >"$(stack_pid_file anvil)"
    stack_wait_anvil 30 || true
}

stack_wait_anvil() {
    local i max="${1:-60}"
    for ((i = 1; i <= max; i++)); do
        stack_anvil_up && return 0
        sleep 1
    done
    return 1
}

stack_ensure_myso() {
    local logf
    stack_rpc_up && return 0
    if stack_is_running myso; then
        stack_wait_rpc 60 && return 0
        log_step "myso process died — restarting"
        stack_stop_one myso
    fi
    mkdir -p "$STACK_DIR"
    logf="$(stack_log_file myso)"
    log_step "Starting myso (release) → $logf"
    # shellcheck disable=SC2206
    (
        cd "$REPO_ROOT"
        export MYSO_CONFIG_DIR
        # shellcheck disable=SC2086
        nohup cargo run --release -p myso -- start --network.config "$MYSO_CONFIG_DIR" $MYSO_START_FLAGS >"$logf" 2>&1 &
        echo $! >"$(stack_pid_file myso)"
    )
    stack_wait_rpc 300 || true
}

stack_ensure_genesis_if_needed() {
    [[ -f "$MYSO_CONFIG_DIR/genesis.blob" && -f "$MYSO_CONFIG_DIR/network.yaml" ]] && return 0
    log_step "No genesis config — creating one"
    stack_fresh_genesis
    STACK_NEEDS_FRESH_BOOTSTRAP=1
}

stack_fresh_genesis() {
    log_step "Regenerating genesis for $MYSO_CONFIG_DIR"
    rm -rf \
        "$MYSO_CONFIG_DIR/authorities_db" \
        "$MYSO_CONFIG_DIR/consensus_db" \
        "$MYSO_CONFIG_DIR/full_node_db" \
        "$MYSO_CONFIG_DIR/data_ingestion" \
        "$MYSO_CONFIG_DIR/consistent_store" \
        "$MYSO_CONFIG_DIR/faucet.wal"
    myso genesis \
        --working-dir "$MYSO_CONFIG_DIR" \
        --committee-size 1 \
        --with-faucet \
        --epoch-duration-ms 60000 \
        --force
    bridge_load_session 2>/dev/null || true
    bridge_reset_fresh_chain_state
    rm -rf "$BRIDGE_DIR/client-db" "$BRIDGE_DIR/token-packages"
    rm -f "$REPO_ROOT/network.config/orderbook/orderbook-session.env"
    STACK_NEEDS_FRESH_BOOTSTRAP=1
    STACK_LAST_CHAIN=''
}

# Returns 0 if chain id changed (caller should run fresh bootstrap).
stack_sync_chain_session() {
    local live saved
    bridge_load_session 2>/dev/null || true
    bridge_apply_defaults
    live="$(bridge_graphql_chain_identifier 2>/dev/null || bridge_live_chain_identifier 2>/dev/null || true)"
    [[ -n "$live" ]] || return 1

    saved="${MYSO_CHAIN_IDENTIFIER:-}"
    if [[ -n "$STACK_LAST_CHAIN" && "$STACK_LAST_CHAIN" != "$live" ]]; then
        log_step "Chain changed ($STACK_LAST_CHAIN → $live)"
        bridge_reset_fresh_chain_state
        rm -f "$REPO_ROOT/network.config/orderbook/orderbook-session.env"
        STACK_NEEDS_FRESH_BOOTSTRAP=1
        STACK_LAST_CHAIN="$live"
        bridge_sync_session_with_chain "$live"
        bridge_save_session
        return 0
    fi

    if [[ -n "$saved" && "$saved" != "$live" ]]; then
        log_step "New chain vs session ($saved → $live)"
        bridge_reset_fresh_chain_state
        rm -f "$REPO_ROOT/network.config/orderbook/orderbook-session.env"
        STACK_NEEDS_FRESH_BOOTSTRAP=1
    fi

    STACK_LAST_CHAIN="$live"
    bridge_sync_session_with_chain "$live"
    bridge_save_session
    return 1
}

stack_handle_new_chain() {
    stack_stop_one bridge-node
    STACK_BOOTSTRAP_PHASE=0
}

stack_social_bootstrap_claimed() {
    local json
    stack_graphql_up || return 1
    json="$(graphql_post '{ caps: objects(filter: { type: "0x2::package::PackagePublishingAdminCap" }, last: 1) { nodes { address } } }' '{}' 2>/dev/null)" || return 1
    [[ "$(printf '%s' "$json" | jq -r '.data.caps.nodes | length' 2>/dev/null)" -gt 0 ]]
}

stack_run_social_bootstrap() {
    stack_social_bootstrap_claimed && return 0
    log_step "Running ./scripts/bootstrap.sh (admin caps)"
    (
        cd "$REPO_ROOT"
        export MYSO_CONFIG_DIR GRAPHQL_URL
        ./scripts/bootstrap.sh
    ) || echo "bootstrap.sh failed — will retry" >&2
}

# Bash 3.2 + set -u treats empty "${arr[@]}" as unbound. Never use arrays for optional args.
stack_run_bridge_bootstrap() {
    local logf rc=0
    logf="$(stack_log_file bootstrap)"
    mkdir -p "$STACK_DIR"
    (
        cd "$REPO_ROOT"
        export MYSO_CONFIG_DIR MYSO_RPC_URL GRAPHQL_URL ETH_RPC_URL
        if [[ $# -gt 0 ]]; then
            ./scripts/localnet-bridge-bootstrap.sh "$@"
        else
            ./scripts/localnet-bridge-bootstrap.sh
        fi
    ) >>"$logf" 2>&1 || rc=$?
    if [[ "$rc" -ne 0 ]]; then
        echo "bridge bootstrap failed (exit $rc) — last 40 lines:" >&2
        tail -n 40 "$logf" >&2 || true
        return "$rc"
    fi
    return 0
}

stack_tick_bootstrap() {
    STACK_TICK_N=$((STACK_TICK_N + 1))
    stack_run_social_bootstrap
    bridge_load_session 2>/dev/null || true
    bridge_apply_defaults

    case "${STACK_BOOTSTRAP_PHASE:-0}" in
        0)
            log_step "Bootstrap phase: native MYSO + committee + EVM"
            if [[ "${STACK_NEEDS_FRESH_BOOTSTRAP:-0}" == 1 ]]; then
                stack_run_bridge_bootstrap --fresh-chain --skip-governance || true
            else
                stack_run_bridge_bootstrap --skip-governance || true
            fi
            STACK_NEEDS_FRESH_BOOTSTRAP=0
            bridge_load_session 2>/dev/null || true
            bridge_apply_defaults
            # Pending or seated — do not re-init; go publish tokens.
            if bridge_committee_has_seat; then
                STACK_BOOTSTRAP_PHASE=2
            else
                STACK_BOOTSTRAP_PHASE=2
                log_step "No committee seat yet — still continuing to token publish"
            fi
            ;;
        1)
            STACK_BOOTSTRAP_PHASE=2
            ;;
        2)
            if [[ "${STACK_PHASE_FAILS:-0}" -ge "${STACK_PHASE_FAIL_CAP:-2}" ]]; then
                if [[ $((STACK_TICK_N % 4)) -ne 0 ]]; then
                    log_step "Phase 2 failed ${STACK_PHASE_FAILS} times — retrying every 4 ticks (see $(stack_log_file bootstrap))"
                    return 0
                fi
                log_step "Retrying phase 2 after failures (tick ${STACK_TICK_N})"
            fi
            log_step "Bootstrap phase: tokens + governance (committee pending is ok)"
            if bridge_committee_node_ready; then
                stack_start_bridge_node || true
            else
                log_step "Committee pending epoch — publishing tokens without starting the node"
            fi
            if stack_run_bridge_bootstrap; then
                STACK_PHASE_FAILS=0
            else
                STACK_PHASE_FAILS=$((STACK_PHASE_FAILS + 1))
            fi
            bridge_load_session 2>/dev/null || true
            bridge_apply_defaults
            if [[ -n "${BRIDGE_BTC_TYPE:-}" && -n "${BRIDGE_ETH_TYPE:-}" \
                && -n "${BRIDGE_USDC_TYPE:-}" && -n "${BRIDGE_USDT_TYPE:-}" ]]; then
                if [[ "${TOKENS_REGISTERED_MYSO:-}" == 1 ]]; then
                    STACK_BOOTSTRAP_PHASE=3
                    STACK_PHASE_FAILS=0
                    log_step "Bridge bootstrap complete"
                elif stack_bridge_authority_up; then
                    if [[ "${STACK_NODE_RELOADED_FOR_TOKENS:-0}" != 1 ]]; then
                        log_step "Reloading bridge node so approved-governance-actions match published tokens"
                        stack_stop_one bridge-node
                        STACK_NODE_RELOADED_FOR_TOKENS=1
                        stack_start_bridge_node || true
                    fi
                    log_step "Tokens published; governance not applied yet — will retry once node stays up"
                else
                    log_step "Tokens published; node not up — supervisor will start it when committee is seated"
                fi
            fi
            ;;
        3)
            if [[ "${TOKENS_REGISTERED_MYSO:-}" != 1 ]] && bridge_committee_node_ready && stack_bridge_authority_up; then
                STACK_BOOTSTRAP_PHASE=2
                STACK_PHASE_FAILS=0
            fi
            ;;
    esac
}

stack_build_bridge_node() {
    [[ -x "$REPO_ROOT/target/debug/myso-bridge-node" ]] \
        || [[ -x "$REPO_ROOT/target/release/myso-bridge-node" ]] && return 0
    log_step "Building myso-bridge-node (one-time)"
    (cd "$REPO_ROOT" && cargo build -p myso-bridge --bin myso-bridge-node) \
        >>"$(stack_log_file bridge-node-build)" 2>&1 || true
}

stack_bridge_node_bin() {
    if [[ -x "$REPO_ROOT/target/debug/myso-bridge-node" ]]; then
        printf '%s/target/debug/myso-bridge-node' "$REPO_ROOT"
    elif [[ -x "$REPO_ROOT/target/release/myso-bridge-node" ]]; then
        printf '%s/target/release/myso-bridge-node' "$REPO_ROOT"
    else
        bridge_node_bin 2>/dev/null || printf '%s/target/debug/myso-bridge-node' "$REPO_ROOT"
    fi
}

stack_bridge_authority_up() {
    bridge_node_reachable "${BRIDGE_AUTHORITY_URL:-http://127.0.0.1:${BRIDGE_NODE_PORT:-19291}}"
}

stack_start_bridge_node() {
    local bin logf cfg
    bridge_load_session 2>/dev/null || true
    bridge_apply_defaults
    if ! bridge_committee_node_ready; then
        return 1
    fi
    cfg="${BRIDGE_NODE_CONFIG_PATH:-$BRIDGE_DIR/bridge-node.yaml}"
    [[ -f "$cfg" ]] || return 1
    stack_bridge_authority_up && return 0
    if stack_is_running bridge-node; then
        sleep 2
        stack_bridge_authority_up && return 0
        stack_stop_one bridge-node
    fi
    stack_build_bridge_node
    bin="$(stack_bridge_node_bin)"
    mkdir -p "$STACK_DIR"
    logf="$(stack_log_file bridge-node)"
    log_step "Starting bridge node → $logf"
    nohup "$bin" --config-path "$cfg" >"$logf" 2>&1 &
    echo "$!" >"$(stack_pid_file bridge-node)"
    local i
    for ((i = 1; i <= 30; i++)); do
        stack_bridge_authority_up && return 0
        sleep 1
    done
    return 1
}

stack_ensure_bridge_node() {
    [[ "${STACK_BOOTSTRAP_PHASE:-0}" -ge 2 ]] || return 0
    bridge_committee_node_ready || return 0
    if stack_bridge_authority_up; then
        stack_is_running bridge-node || return 0
        return 0
    fi
    if stack_is_running bridge-node; then
        sleep 2
        stack_bridge_authority_up && return 0
        log_step "Bridge node crashed — restarting"
        stack_stop_one bridge-node
    fi
    stack_start_bridge_node || true
}

stack_stop_all() {
    stack_stop_one bridge-node
    stack_stop_one anvil
    stack_stop_one myso
}

stack_print_status_line() {
    local chain total phase rpc gql anvil node
    chain="$(bridge_graphql_chain_identifier 2>/dev/null || true)"
    chain="${chain:-${STACK_LAST_CHAIN:-?}}"
    total="$(bridge_committee_total_voting_power 2>/dev/null || echo '?')"
    phase="${STACK_BOOTSTRAP_PHASE:-0}"
    rpc='down'; stack_rpc_up && rpc='up'
    gql='down'; stack_graphql_up && gql='up'
    anvil='down'; stack_anvil_up && anvil='up'
    node='down'; stack_bridge_authority_up && node='up'
    printf '[bridge-stack] chain=%s phase=%s committee=%s/10000 rpc=%s gql=%s anvil=%s node=%s (Ctrl+C to stop)\n' \
        "$chain" "$phase" "$total" "$rpc" "$gql" "$anvil" "$node" >&2
}
