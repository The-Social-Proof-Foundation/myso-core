#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Unified PoC E2E: session refresh → oracle sync → post → grpc-sync attestation →
# on-chain event asserts → GraphQL cross-check → tip/reserve → discovery embed → mock claim.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - GraphQL :9125, fullnode :9000
#   - PoC API :8000 (`myso start --with-poc` or manual docker compose --profile app)
#
# Session: network.config/poc/poc-e2e-session.env
#
# Usage:
#   ./scripts/poc-e2e-runnable.sh                              # interactive menu
#   ASSUME_YES=1 ./scripts/poc-e2e-runnable.sh --run-all
#   ASSUME_YES=1 ./scripts/poc-e2e-runnable.sh --run-all --skip-discovery
#   ASSUME_YES=1 ./scripts/poc-e2e-runnable.sh --run-all --skip-claim
#   ./scripts/poc-e2e-runnable.sh --refresh-session

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/poc/poc-e2e-session.env"
export SOCIAL_SESSION_SAVE_PATH
export ASSUME_YES="${ASSUME_YES:-0}"
export SKIP_CONFIRM_RUN="${SKIP_CONFIRM_RUN:-1}"

# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/poc-oracle-common.sh
source "${SCRIPT_DIR}/lib/poc-oracle-common.sh"
# shellcheck source=lib/poc-oracle-http.sh
source "${SCRIPT_DIR}/lib/poc-oracle-http.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

RUN_MODE=''
SKIP_DISCOVERY=0
SKIP_CLAIM=0
POC_USE_DIRECT_MOVE="${POC_USE_DIRECT_MOVE:-0}"
export POC_USE_DIRECT_MOVE

ANALYZE_TX_DIGEST=''
POC_CONFIG_SYNC_DIGEST=''

usage() {
    sed -n '2,24p' "$0" | sed 's/^# \?//'
}

load_e2e_session() {
    social_load_session
}

invoke_poc_oracle_post() {
    local mode="$1"
    SOCIAL_SESSION_SAVE_PATH="$SOCIAL_SESSION_SAVE_PATH" \
        ASSUME_YES="$ASSUME_YES" SKIP_CONFIRM_RUN="$SKIP_CONFIRM_RUN" \
        POC_USE_DIRECT_MOVE="$POC_USE_DIRECT_MOVE" \
        "$SCRIPT_DIR/poc-oracle-post-runnable.sh" "$mode"
}

preflight_stack() {
    local rpc="${MYSO_RPC_URL:-http://127.0.0.1:9000}"
    curl -sf -X POST "$rpc" -H 'content-type: application/json' \
        -d '{"jsonrpc":"2.0","id":1,"method":"rpc.discover","params":[]}' >/dev/null 2>&1 || {
        echo "Fullnode not reachable at $rpc (JSON-RPC rpc.discover failed)" >&2
        return 1
    }
    poc_oracle_stack_ready || return 1
    log_step "Preflight OK (fullnode + PoC API + GraphQL)"
}

step_refresh_session() {
    log_step "Refreshing PoC E2E session from GraphQL (proof-of-creativity-runnable)"
    SOCIAL_SESSION_SAVE_PATH="$SOCIAL_SESSION_SAVE_PATH" \
        ASSUME_YES="$ASSUME_YES" \
        "$SCRIPT_DIR/proof-of-creativity-runnable.sh" --refresh-session
    log_step "Refreshing PoC oracle post extras (poc-oracle-post-runnable)"
    invoke_poc_oracle_post --refresh-session
    load_e2e_session
    log_step "Session refreshed (chain object IDs + PoC oracle extras)"
}

step_sync_oracle_and_env() {
    load_e2e_session
    poc_oracle_sync_worker_stack || return 1
}

step_create_post() {
    log_step "Creating PoC+SPT post (poc-oracle-post-runnable --run-all)"
    invoke_poc_oracle_post --run-all
    load_e2e_session
    require_session_fields POST_ID LAST_TX_DIGEST RESERVATION_POOL_ID || return 1
    log_step "Post created POST_ID=$POST_ID pool=$RESERVATION_POOL_ID digest=$LAST_TX_DIGEST"
}

step_wait_worker_attestation() {
    local digest
    log_step "Waiting for grpc-sync + oracle worker attestation on post $POST_ID"
    digest="$(wait_for_poc_post_attested "$POST_ID" "${POC_ORACLE_ATTESTATION_TIMEOUT_SEC:-180}")" || return 1
    ANALYZE_TX_DIGEST="$digest"
    log_step "Worker attested post=$POST_ID analyze_tx=$ANALYZE_TX_DIGEST"
}

step_assert_analyze_chain_events() {
    [[ -n "$ANALYZE_TX_DIGEST" ]] || {
        echo "Missing ANALYZE_TX_DIGEST" >&2
        return 1
    }
    wait_for_tx_finalized "$ANALYZE_TX_DIGEST" || return 1
    assert_poc_scenario_events analyze_original "$ANALYZE_TX_DIGEST" || return 1
    log_step "Chain events OK for analyze tx $ANALYZE_TX_DIGEST"
}

step_graphql_cross_check() {
    local resp
    resp="$(gql_cross_check_post_poc "$POST_ID")" || true
    if [[ -n "$resp" ]]; then
        log_step "GraphQL post snapshot: $(echo "$resp" | jq -c '.data.post // empty')"
    fi
}

step_tip_and_reserve() {
    log_step "Tip post (vault deposit path)"
    invoke_poc_oracle_post --tip-post
    load_e2e_session
    require_session_fields LAST_TIP_TX_DIGEST || return 1
    log_step "Tip tx=$LAST_TIP_TX_DIGEST events verified by poc-oracle-post-runnable"

    log_step "SPT reserve into post"
    invoke_poc_oracle_post --reserve-post
    load_e2e_session
    require_session_fields LAST_RESERVE_TX_DIGEST || return 1
    log_step "Reserve tx=$LAST_RESERVE_TX_DIGEST events verified by poc-oracle-post-runnable"
}

step_discovery_embed() {
    log_step "Off-network discovery embed leg (self-contained PoC discovery-worker)"
    POC_ORACLE_URL="${POC_ORACLE_URL:-http://127.0.0.1:8000}" \
        MYSO_POC_REPO="${MYSO_POC_REPO:-$REPO_ROOT/../proof-of-creativity}" \
        "$SCRIPT_DIR/discovery-poc-runnable.sh"
}

step_username_provision_and_claim() {
    log_step "Username provision + mock claim (proof-of-creativity-runnable --username-flow)"
    SOCIAL_SESSION_SAVE_PATH="$SOCIAL_SESSION_SAVE_PATH" \
        ASSUME_YES="$ASSUME_YES" SKIP_CONFIRM_RUN="$SKIP_CONFIRM_RUN" \
        "$SCRIPT_DIR/proof-of-creativity-runnable.sh" --username-flow
    load_e2e_session
}

print_e2e_summary() {
    print_run_summary_header "PoC E2E — full loop completed"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Oracle address" "${POC_DEFAULT_ORACLE_ADDRESS:-}"
    print_run_summary_line "Config sync digest" "${POC_CONFIG_SYNC_DIGEST:-<skipped>}"
    print_run_summary_line "Post" "$(normalize_hex_id "${POST_ID:-}")"
    print_run_summary_line "Reservation pool" "$(normalize_hex_id "${RESERVATION_POOL_ID:-}")"
    print_run_summary_line "Create digest" "${LAST_TX_DIGEST:-}"
    print_run_summary_line "Analyze digest (worker)" "${ANALYZE_TX_DIGEST:-}"
    print_run_summary_line "Tip digest" "${LAST_TIP_TX_DIGEST:-}"
    print_run_summary_line "Reserve digest" "${LAST_RESERVE_TX_DIGEST:-}"
    if [[ -n "${POC_UB_LAST_BENEFICIARY_ID:-}" ]]; then
        print_run_summary_line "Username beneficiary" "${POC_UB_LAST_BENEFICIARY_ID}"
        print_run_summary_line "Claim digest" "${POC_ORACLE_LAST_CLAIM_TX:-}"
    fi
    print_run_summary_line "Chain verification" "Move events asserted via myso client tx-block (GraphQL cross-check only)"
    print_run_summary_footer
}

run_poc_e2e_all() {
    preflight_stack || return 1
    step_refresh_session || return 1
    step_sync_oracle_and_env || return 1
    step_create_post || return 1

    if [[ "$POC_USE_DIRECT_MOVE" == "1" ]]; then
        log_step "POC_USE_DIRECT_MOVE=1 — triggering HTTP/e2e analyze instead of worker wait"
        sync_poc_config_oracle_on_chain "$POC_DEFAULT_ORACLE_ADDRESS" || return 1
        ANALYZE_TX_DIGEST="$(poc_oracle_analyze_post "$POST_ID" 1 50 none 0 false false 0)" || return 1
    else
        step_wait_worker_attestation || return 1
    fi

    step_assert_analyze_chain_events || return 1
    step_graphql_cross_check || true
    step_tip_and_reserve || return 1

    if [[ "$SKIP_DISCOVERY" != "1" ]]; then
        step_discovery_embed || return 1
    else
        log_step "Skipping discovery embed (--skip-discovery)"
    fi

    if [[ "$SKIP_CLAIM" != "1" ]]; then
        step_username_provision_and_claim || return 1
    else
        log_step "Skipping username claim (--skip-claim)"
    fi

    print_e2e_summary
}

show_menu() {
    echo ""
    echo "=== PoC E2E ==="
    echo " 1) Full E2E loop (recommended)"
    echo " 2) Refresh session from GraphQL"
    echo " ---"
    echo " Advanced"
    echo " a) Full E2E — skip discovery embed"
    echo " b) Full E2E — skip username claim"
    echo " c) Full E2E — direct-move analyze (no worker wait)"
    echo " ?) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        1) run_poc_e2e_all ;;
        2) step_refresh_session ;;
        [Aa])
            SKIP_DISCOVERY=1
            run_poc_e2e_all
            SKIP_DISCOVERY=0
            ;;
        [Bb])
            SKIP_CLAIM=1
            run_poc_e2e_all
            SKIP_CLAIM=0
            ;;
        [Cc])
            POC_USE_DIRECT_MOVE=1
            export POC_USE_DIRECT_MOVE
            run_poc_e2e_all
            POC_USE_DIRECT_MOVE=0
            export POC_USE_DIRECT_MOVE
            ;;
        \?) usage ;;
        [Qq]) exit 0 ;;
        *) echo "Invalid choice" ;;
    esac
    show_menu
}

main() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h) usage; exit 0 ;;
            -y) ASSUME_YES=1; export ASSUME_YES; shift ;;
            --refresh-session) RUN_MODE=refresh; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            --skip-discovery) SKIP_DISCOVERY=1; shift ;;
            --skip-claim) SKIP_CLAIM=1; shift ;;
            --direct-move) POC_USE_DIRECT_MOVE=1; export POC_USE_DIRECT_MOVE; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    load_e2e_session

    case "${RUN_MODE:-}" in
        refresh) step_refresh_session; exit 0 ;;
        run_all) run_poc_e2e_all; exit 0 ;;
        '')
            if [[ ! -t 0 ]]; then
                echo "No TTY — use: ASSUME_YES=1 ./scripts/poc-e2e-runnable.sh --run-all" >&2
                exit 1
            fi
            show_menu
            ;;
        *) echo "Unknown run mode: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
