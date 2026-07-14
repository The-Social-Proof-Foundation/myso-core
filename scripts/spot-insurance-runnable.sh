#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper for insurance coverage on a live SPoT prediction market.
# Follows the SPoT oracle procedure: create post → review → market → bet →
# prepare insurance vault → buy coverage → resolve → claim insurance → claim payout.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed; social-proof2 owns SpotOracleAdminCap /
#     InsuranceAdminCap (or the funder wallet used below)
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126
#   - docker, curl, jq, cargo on PATH
#   - Live HTTP sources (CoinGecko etc.) OR SPOT_ORACLE_LIVE_SOURCES=false with stubs
#
# Session: network.config/spot-insurance/spot-insurance-session.env
#
# Usage:
#   ./scripts/spot-insurance-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/spot-insurance-runnable.sh --run-all
#   ./scripts/spot-insurance-runnable.sh   # interactive menu (prompts for claim)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/spot-insurance/spot-insurance-session.env"
# shellcheck source=lib/spot-oracle-common.sh
source "${SCRIPT_DIR}/lib/spot-oracle-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

COMPOSE_FILE="$REPO_ROOT/crates/myso-spot-oracle/docker-compose.yml"
KEEP_STACK="${KEEP_STACK:-0}"
SVC_PID=""
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"
ONCHAIN_MODE=1

BETTOR_ADDRESS=''
BET_OPTION_ID=''
BET_AMOUNT_MIST=''
BET_TX_DIGEST=''
PAYOUT_TX_DIGEST=''
SPOT_MARKET_ID=''
POST_ID=''
CREATOR_ADDRESS=''
INSURANCE_CONFIG_ID=''
INSURANCE_ROUTER_CONFIG_ID=''
INSURANCE_BACKSTOP_ID=''
INSURANCE_ADMIN_CAP_ID=''
INSURANCE_VAULT_ID=''
INSURANCE_POLICY_ID=''
INSURANCE_BUY_TX=''
INSURANCE_CLAIM_TX=''

SPOT_INSURANCE_SESSION_KEYS=(
    "${SPOT_ORACLE_SESSION_KEYS[@]}"
    POST_ID CREATOR_ADDRESS SPOT_CLAIM_TEXT SPOT_CLAIM_ID SPOT_MARKET_ID
    BETTOR_ADDRESS BET_OPTION_ID BET_AMOUNT_MIST BET_TX_DIGEST PAYOUT_TX_DIGEST
    INSURANCE_CONFIG_ID INSURANCE_ROUTER_CONFIG_ID INSURANCE_BACKSTOP_ID INSURANCE_ADMIN_CAP_ID
    INSURANCE_VAULT_ID INSURANCE_POLICY_ID INSURANCE_BUY_TX INSURANCE_CLAIM_TX
)

save_insurance_session() {
    spot_oracle_map_session_to_oracle_env
    social_save_session "${SPOT_INSURANCE_SESSION_KEYS[@]}"
}

usage() {
    sed -n '2,24p' "$0" | sed 's/^# \?//'
}

psql_exec() {
    docker compose -f "$COMPOSE_FILE" exec -T spot-oracle-postgres \
        psql -U spot -d spot_oracle -tAc "$1"
}

cleanup() {
    if [[ -n "$SVC_PID" ]]; then
        kill "$SVC_PID" 2>/dev/null || true
        wait "$SVC_PID" 2>/dev/null || true
    fi
    if [[ "$KEEP_STACK" != "1" ]]; then
        docker compose -f "$COMPOSE_FILE" down -v >/dev/null 2>&1 || true
    fi
}

start_postgres() {
    log_step "Starting spot-oracle postgres (fresh volume for insurance E2E)"
    if [[ "${KEEP_STACK:-0}" != "1" ]]; then
        docker compose -f "$COMPOSE_FILE" down -v >/dev/null 2>&1 || true
    fi
    docker compose -f "$COMPOSE_FILE" up -d spot-oracle-postgres
    local i
    for i in $(seq 1 60); do
        if docker compose -f "$COMPOSE_FILE" exec -T spot-oracle-postgres pg_isready -U spot -d spot_oracle >/dev/null 2>&1; then
            return 0
        fi
        sleep 1
    done
    echo "spot-oracle postgres did not become ready" >&2
    return 1
}

wait_for_oracle_health() {
    local listen="$SPOT_ORACLE_LISTEN" i
    for i in $(seq 1 120); do
        if curl -sf "http://${listen}/health" >/dev/null 2>&1; then
            return 0
        fi
        if [[ -n "$SVC_PID" ]] && ! kill -0 "$SVC_PID" 2>/dev/null; then
            echo "spot-oracle exited before becoming healthy" >&2
            return 1
        fi
        sleep 1
    done
    echo "spot-oracle did not become healthy" >&2
    return 1
}

run_migrations_boot() {
    log_step "Running DB migrations (spot-oracle boot, workers disabled)"
    export_spot_oracle_env
    SPOT_ORACLE_ENABLED=false cargo run -p myso-spot-oracle &
    SVC_PID=$!
    wait_for_oracle_health || return 1
    kill "$SVC_PID" 2>/dev/null || true
    wait "$SVC_PID" 2>/dev/null || true
    SVC_PID=""
}

boot_onchain_oracle_workers() {
    log_step "Starting spot-oracle workers (on-chain)"
    export_spot_oracle_env
    SPOT_ORACLE_REVIEW_POLL_INTERVAL_SECS=5 \
    SPOT_ORACLE_SCHEDULER_POLL_INTERVAL_SECS=5 \
    cargo run -p myso-spot-oracle &
    SVC_PID=$!
    wait_for_oracle_health || return 1
}

wait_for_post_ingest() {
    local post_id="$1"
    local i found=0
    log_step "Waiting for checkpoint ingest to ingest ${post_id}..."
    for i in $(seq 1 90); do
        found="$(psql_exec "SELECT COUNT(*) FROM markets WHERE post_id = '${post_id}'" || echo 0)"
        found="${found// /}"
        if [[ -n "$found" && "$found" -gt 0 ]]; then
            return 0
        fi
        sleep 2
    done
    echo "FAIL: market for POST_ID=${post_id} not ingested (is SubscribeCheckpoints streaming and enable_spot=true?)" >&2
    return 1
}

wait_for_accepted_review() {
    local post_filter="${1:-}"
    local reviews=0 accepted=0 reason='' i review_sql accepted_sql
    if [[ -n "$post_filter" ]]; then
        review_sql="SELECT COUNT(*) FROM oracle_reviews WHERE post_id = '${post_filter}'"
        accepted_sql="SELECT COUNT(*) FROM oracle_reviews WHERE post_id = '${post_filter}' AND decision = 'accepted'"
    else
        review_sql="SELECT COUNT(*) FROM oracle_reviews"
        accepted_sql="SELECT COUNT(*) FROM oracle_reviews WHERE decision = 'accepted'"
    fi
    log_step "Waiting for accepted oracle_reviews${post_filter:+ for $post_filter} (up to 3 min)..."
    for i in $(seq 1 90); do
        reviews="$(psql_exec "$review_sql" || echo 0)"
        reviews="${reviews// /}"
        accepted="$(psql_exec "$accepted_sql" || echo 0)"
        accepted="${accepted// /}"
        if [[ -n "$accepted" && "$accepted" -gt 0 ]]; then
            REVIEWS_COUNT="$reviews"
            ACCEPTED_COUNT="$accepted"
            return 0
        fi
        if [[ -n "$post_filter" ]]; then
            reason="$(psql_exec "SELECT reject_reason FROM oracle_reviews WHERE post_id = '${post_filter}' AND decision = 'rejected' ORDER BY created_at DESC LIMIT 1" || true)"
            reason="${reason// /}"
            if [[ "$reason" == "missing_deadline" ]]; then
                echo "FAIL: claim rejected — add when the claim should be evaluated (e.g. 'by the end of tomorrow' or 'before July 31, 2027')." >&2
                return 1
            fi
        fi
        sleep 2
    done
    echo "FAIL: no accepted oracle_reviews after pipeline (total=${reviews:-0})" >&2
    return 1
}

wait_for_market_active() {
    local post_id="$1"
    local require_spot_id="${2:-0}"
    local status='' market_obj='' i
    log_step "Waiting for market active${require_spot_id:+ + spot_market_object_id} (POST_ID=$post_id)..."
    for i in $(seq 1 90); do
        status="$(psql_exec "SELECT status FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
        status="${status// /}"
        market_obj="$(psql_exec "SELECT COALESCE(spot_market_object_id, '') FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
        market_obj="${market_obj// /}"
        if [[ "$status" == "waiting" || "$status" == "active" || "$status" == "resolving" || "$status" == "resolved" ]]; then
            if [[ "$require_spot_id" != "1" || -n "$market_obj" ]]; then
                MARKET_STATUS="$status"
                SPOT_MARKET_ID="${market_obj:-$SPOT_MARKET_ID}"
                return 0
            fi
        fi
        sleep 2
    done
    echo "FAIL: market not active (status='${status:-}' spot_market_object_id='${market_obj:-}')" >&2
    return 1
}

wait_for_market_resolved() {
    local post_id="$1"
    local status='' i
    log_step "Waiting for market resolved (POST_ID=$post_id, up to 5 min)..."
    for i in $(seq 1 150); do
        status="$(psql_exec "SELECT status FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
        status="${status// /}"
        if [[ "$status" == "resolved" ]]; then
            MARKET_STATUS="$status"
            return 0
        fi
        sleep 2
    done
    echo "FAIL: market not resolved (status='${status:-}')" >&2
    return 1
}

walkthrough_ensure_platform() {
    if [[ -z "${PLATFORM_OBJECT_ID:-}" ]] || ! object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
        SOCIAL_RUN_ID="${SOCIAL_RUN_ID:-$(date +%s)}"
        log_step "No platform on localnet — creating test platform"
        create_test_platform || return 1
        save_insurance_session
    fi
}

walkthrough_preflight() {
    require_social_stack_for_onchain || return 1
    validate_onchain_oracle_key || return 1
    log_step "Preflight OK (GraphQL + social-server + oracle key)"
}

walkthrough_refresh_session_if_needed() {
    if [[ -z "${SPOT_CONFIG_ID:-}" || -z "${SPOT_REGISTRY_ID:-}" || -z "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        refresh_spot_oracle_session_from_graphql || return 1
        load_spot_oracle_session
    fi
}

refresh_spot_insurance_session_from_graphql() {
    refresh_spot_oracle_session_from_graphql || return 1
    load_spot_oracle_session
    spot_insurance_refresh_ids || return 1
    save_insurance_session
}

run_insurance_walkthrough() {
    ONCHAIN_MODE=1
    trap cleanup EXIT

    spot_prompt_walkthrough_claim || return 1
    local walkthrough_claim="$SPOT_CLAIM_TEXT"
    load_spot_oracle_session
    SPOT_CLAIM_TEXT="$walkthrough_claim"
    export SPOT_CLAIM_TEXT

    # Clear per-run insurance / bet IDs so a prior session cannot skip vault create.
    INSURANCE_VAULT_ID=''
    INSURANCE_POLICY_ID=''
    INSURANCE_BUY_TX=''
    INSURANCE_CLAIM_TX=''
    BET_TX_DIGEST=''
    PAYOUT_TX_DIGEST=''
    save_insurance_session

    walkthrough_preflight || return 1
    walkthrough_refresh_session_if_needed || return 1
    spot_insurance_refresh_ids || return 1
    save_insurance_session

    start_postgres || return 1
    run_migrations_boot || return 1
    walkthrough_ensure_platform || return 1
    boot_onchain_oracle_workers || return 1

    log_step "Creating enable_spot=true post"
    SPOT_CLAIM_TEXT="$SPOT_CLAIM_TEXT" ASSUME_YES=1 \
        "$SCRIPT_DIR/spot-oracle-post-runnable.sh" --run-all || return 1
    # Post runnable persists to spot-oracle-session.env (its own SOCIAL_SESSION_SAVE_PATH).
    local spot_session="$REPO_ROOT/network.config/spot-oracle/spot-oracle-session.env"
    [[ -f "$spot_session" ]] || {
        echo "FAIL: missing spot-oracle session after post create: $spot_session" >&2
        return 1
    }
    POST_ID="$(
        # shellcheck disable=SC1090
        source "$spot_session"
        printf '%s' "${POST_ID:-}"
    )"
    CREATOR_ADDRESS="$(
        # shellcheck disable=SC1090
        source "$spot_session"
        printf '%s' "${CREATOR_ADDRESS:-}"
    )"
    [[ -n "$POST_ID" ]] || {
        echo "FAIL: POST_ID missing after spot-oracle-post-runnable" >&2
        return 1
    }
    POST_ID="$(normalize_hex_id "$POST_ID")"
    [[ -n "$CREATOR_ADDRESS" ]] && CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "CREATOR_ADDRESS" "${CREATOR_ADDRESS:-}"
    save_insurance_session

    wait_for_post_ingest "$POST_ID" || return 1
    wait_for_accepted_review "$POST_ID" || return 1
    wait_for_market_active "$POST_ID" 1 || return 1

    SPOT_MARKET_ID="$(spot_resolve_market_id "$POST_ID" "${SPOT_MARKET_ID:-}")" || return 1
    log_session_use "SPOT_MARKET_ID" "$SPOT_MARKET_ID"

    local options_json
    options_json="$(psql_exec "SELECT betting_options::text FROM markets WHERE post_id = '${POST_ID}' LIMIT 1" || true)"
    options_json="${options_json// /}"
    [[ -n "$options_json" && "$options_json" != "[]" ]] || options_json='["Yes","No"]'
    log_step "Market active — POST_ID=$POST_ID market=$SPOT_MARKET_ID options=${options_json}"

    spot_prompt_bet_side "$options_json" || return 1
    spot_prompt_bet_amount_mist

    BETTOR_ADDRESS="${BETTOR_ADDRESS:-${CREATOR_ADDRESS:-}}"
    [[ -n "$BETTOR_ADDRESS" ]] || BETTOR_ADDRESS="$(resolve_myso_active_address)" || return 1
    BETTOR_ADDRESS="$(normalize_hex_id "$BETTOR_ADDRESS")"
    log_session_use "BETTOR_ADDRESS" "$BETTOR_ADDRESS"

    spot_place_bet_for_post "$BETTOR_ADDRESS" "$POST_ID" "$SPOT_MARKET_ID" "$BET_OPTION_ID" "$BET_AMOUNT_MIST" || return 1
    save_insurance_session

    log_step "Preparing insurance vault + buying coverage"
    spot_insurance_e2e_prepare || return 1
    spot_insurance_buy_coverage "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" "$BET_OPTION_ID" || return 1
    save_insurance_session

    log_step "Waiting for oracle resolve (~1–3 min for price-threshold claims)..."
    wait_for_market_resolved "$POST_ID" || return 1

    spot_insurance_claim "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" || return 1
    spot_insurance_assert_duplicate_claim_fails "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" || return 1
    save_insurance_session

    if [[ "${ASSUME_YES:-0}" != "1" ]]; then
        read -r -p "Market resolved. Press Enter to claim SPoT payout (or Ctrl+C to skip)... " _
    fi
    spot_claim_payout "$BETTOR_ADDRESS" "$POST_ID" "$SPOT_MARKET_ID" || return 1
    save_insurance_session

    print_run_summary_header "SPoT Insurance E2E — PASS"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Claim" "$SPOT_CLAIM_TEXT"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "SpotMarket" "$SPOT_MARKET_ID"
    print_run_summary_line "Bettor" "$BETTOR_ADDRESS"
    print_run_summary_line "Bet side" "option_id=${BET_OPTION_ID}"
    print_run_summary_line "Bet amount (MIST)" "$BET_AMOUNT_MIST"
    print_run_summary_line "Bet tx" "${BET_TX_DIGEST:-<none>}"
    print_run_summary_line "Insurance vault" "${INSURANCE_VAULT_ID:-<none>}"
    print_run_summary_line "Insurance policy" "${INSURANCE_POLICY_ID:-<none>}"
    print_run_summary_line "Insurance buy tx" "${INSURANCE_BUY_TX:-<none>}"
    print_run_summary_line "Insurance claim tx" "${INSURANCE_CLAIM_TX:-<none>}"
    print_run_summary_line "SPoT payout tx" "${PAYOUT_TX_DIGEST:-<none>}"
    print_run_summary_line "Market status" "${MARKET_STATUS:-resolved}"
    print_run_summary_line "Outcome" "SPOT market created, coverage purchased, market resolved, insurance claimed (duplicate rejected), SPoT payout claimed"
    print_run_summary_footer
}

show_menu() {
    echo ""
    echo "=== SPoT Insurance E2E Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Run insurance walkthrough (--run-all)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0)
            refresh_spot_insurance_session_from_graphql && load_spot_oracle_session || {
                echo "FAIL: session refresh aborted; existing session file unchanged." >&2
            }
            ;;
        1) run_insurance_walkthrough ;;
        [Hh]) usage ;;
        [Qq]) exit 0 ;;
        *) echo "Invalid choice" ;;
    esac
    show_menu
}

main() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h) usage; exit 0 ;;
            -y) ASSUME_YES=1; shift ;;
            --refresh-session) RUN_MODE=refresh; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    local claim_from_env=''
    if [[ "${RUN_MODE:-}" != "run_all" && -n "${SPOT_CLAIM_TEXT:-}" ]]; then
        claim_from_env="$SPOT_CLAIM_TEXT"
    fi

    load_spot_oracle_session
    if [[ -n "$claim_from_env" ]]; then
        SPOT_CLAIM_TEXT="$claim_from_env"
        export SPOT_CLAIM_TEXT
    fi

    case "${RUN_MODE:-}" in
        refresh)
            refresh_spot_insurance_session_from_graphql || {
                echo "FAIL: could not refresh SPoT/insurance session from GraphQL ($GRAPHQL_URL)" >&2
                exit 1
            }
            load_spot_oracle_session
            exit 0
            ;;
        run_all) run_insurance_walkthrough ;;
        '')
            if [[ ! -t 0 ]]; then
                echo "No TTY — use: ASSUME_YES=1 ./scripts/spot-insurance-runnable.sh --run-all" >&2
                exit 1
            fi
            show_menu
            ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
