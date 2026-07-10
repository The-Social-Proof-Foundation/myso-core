#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# SPoT oracle E2E helper: refresh chain object IDs from GraphQL, boot oracle stack,
# run review → resolve pipeline with live CoinGecko evidence.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed; social-proof2 owns SpotOracleAdminCap
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126 (required for on-chain walkthrough)
#   - docker, curl, jq, cargo on PATH
#   - Live HTTP sources (CoinGecko etc.) OR SPOT_ORACLE_LIVE_SOURCES=false with stubs
#   - Discovery is NOT required (SPoT fetches trusted sources directly)
#
# Session: network.config/spot-oracle/spot-oracle-session.env
#
# Usage:
#   ./scripts/spot-oracle-runnable.sh                              # menu 1 prompts for your prediction
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-walkthrough  # non-interactive auto claim
#   ./scripts/spot-oracle-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all          # off-chain SQL seed
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all-onchain
#   ENABLE_INSURANCE_E2E=1 ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-walkthrough
#   ./scripts/spot-oracle-runnable.sh --run-lazy-payout              # Move unit test

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/spot-oracle/spot-oracle-session.env"
# shellcheck source=lib/spot-oracle-common.sh
source "${SCRIPT_DIR}/lib/spot-oracle-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

COMPOSE_FILE="$REPO_ROOT/crates/myso-spot-oracle/docker-compose.yml"
KEEP_STACK="${KEEP_STACK:-0}"
SVC_PID=""
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"
ONCHAIN_MODE=0
BETTOR_ADDRESS=''
BET_OPTION_ID=''
BET_AMOUNT_MIST=''
BET_TX_DIGEST=''
PAYOUT_TX_DIGEST=''
SPOT_MARKET_ID=''

SPOT_WALKTHROUGH_SESSION_KEYS=(
    "${SPOT_ORACLE_SESSION_KEYS[@]}"
    POST_ID CREATOR_ADDRESS SPOT_CLAIM_TEXT SPOT_CLAIM_ID SPOT_MARKET_ID
    BETTOR_ADDRESS BET_OPTION_ID BET_AMOUNT_MIST BET_TX_DIGEST PAYOUT_TX_DIGEST
    INSURANCE_CONFIG_ID INSURANCE_ROUTER_CONFIG_ID INSURANCE_BACKSTOP_ID INSURANCE_ADMIN_CAP_ID
    INSURANCE_VAULT_ID INSURANCE_POLICY_ID INSURANCE_BUY_TX INSURANCE_CLAIM_TX
)

save_walkthrough_session() {
    spot_oracle_map_session_to_oracle_env
    social_save_session "${SPOT_WALKTHROUGH_SESSION_KEYS[@]}"
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
    log_step "Starting spot-oracle postgres (fresh volume for E2E)"
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
    if [[ "$ONCHAIN_MODE" != "1" ]]; then
        spot_oracle_force_offchain_mode
    fi
    SPOT_ORACLE_ENABLED=false cargo run -p myso-spot-oracle &
    SVC_PID=$!
    wait_for_oracle_health || return 1
    kill "$SVC_PID" 2>/dev/null || true
    wait "$SVC_PID" 2>/dev/null || true
    SVC_PID=""
}

seed_review_job() {
    log_step "Seeding market + ReviewPost job (BTC above \$1)"
    psql_exec "
    INSERT INTO markets (post_id, creator, claim_text, status, betting_options)
    SELECT '0xdeadbeef', '0xowner', 'Will BTC trade above \$1?', 'pending_review', '[\"Yes\",\"No\"]'::jsonb
    WHERE NOT EXISTS (SELECT 1 FROM markets WHERE post_id = '0xdeadbeef');
    "
    local market_id
    market_id="$(psql_exec "SELECT id::text FROM markets WHERE post_id = '0xdeadbeef' LIMIT 1")"
    market_id="${market_id// /}"
    psql_exec "
    INSERT INTO spot_jobs (job_type, market_id, priority_score, payload)
    VALUES (
      'ReviewPost',
      '${market_id}',
      100,
      '{\"post_id\":\"0xdeadbeef\",\"owner\":\"0xowner\",\"content\":\"Will BTC trade above \$1?\",\"created_at_ms\":1}'::jsonb
    );
    "
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

wait_for_evidence() {
    local post_filter="${1:-}"
    local evidence=0 hash='' i evidence_sql hash_sql
    if [[ -n "$post_filter" ]]; then
        evidence_sql="SELECT COUNT(*) FROM evidence e JOIN markets m ON m.id = e.market_id WHERE m.post_id = '${post_filter}'"
        hash_sql="SELECT e.content_hash FROM evidence e JOIN markets m ON m.id = e.market_id WHERE m.post_id = '${post_filter}' ORDER BY e.fetched_at DESC LIMIT 1"
    else
        evidence_sql="SELECT COUNT(*) FROM evidence"
        hash_sql="SELECT content_hash FROM evidence ORDER BY fetched_at DESC LIMIT 1"
    fi
    log_step "Waiting for evidence (after claim deadline, up to 5 min)..."
    for i in $(seq 1 150); do
        evidence="$(psql_exec "$evidence_sql" || echo 0)"
        evidence="${evidence// /}"
        if [[ -n "$evidence" && "$evidence" -gt 0 ]]; then
            hash="$(psql_exec "$hash_sql" || true)"
            hash="${hash// /}"
            EVIDENCE_COUNT="$evidence"
            SAMPLE_HASH="$hash"
            return 0
        fi
        sleep 2
    done
    echo "FAIL: no evidence rows after review+resolve pipeline" >&2
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

wait_for_reviews_and_evidence() {
    wait_for_accepted_review || return 1
    wait_for_evidence || return 1
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

walkthrough_ensure_platform() {
    if [[ -z "${PLATFORM_OBJECT_ID:-}" ]] || ! object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
        SOCIAL_RUN_ID="${SOCIAL_RUN_ID:-$(date +%s)}"
        log_step "No platform on localnet — creating test platform"
        create_test_platform || return 1
        save_walkthrough_session
    fi
}

walkthrough_preflight() {
    require_social_stack_for_onchain || return 1
    validate_onchain_oracle_key || return 1
    log_step "Preflight OK (GraphQL + social-server + oracle key; no Discovery)"
}

walkthrough_refresh_session_if_needed() {
    if [[ -z "${SPOT_CONFIG_ID:-}" || -z "${SPOT_REGISTRY_ID:-}" || -z "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        refresh_spot_oracle_session_from_graphql || return 1
        load_spot_oracle_session
    fi
}

run_market_walkthrough() {
    ONCHAIN_MODE=1
    trap cleanup EXIT

    spot_prompt_walkthrough_claim || return 1
    local walkthrough_claim="$SPOT_CLAIM_TEXT"
    load_spot_oracle_session
    SPOT_CLAIM_TEXT="$walkthrough_claim"
    export SPOT_CLAIM_TEXT
    save_walkthrough_session

    walkthrough_preflight || return 1
    walkthrough_refresh_session_if_needed || return 1

    start_postgres || return 1
    run_migrations_boot || return 1
    walkthrough_ensure_platform || return 1
    boot_onchain_oracle_workers || return 1

    log_step "Creating enable_spot=true post"
    SPOT_CLAIM_TEXT="$SPOT_CLAIM_TEXT" ASSUME_YES=1 \
        "$SCRIPT_DIR/spot-oracle-post-runnable.sh" --run-all || return 1
    load_spot_oracle_session
    [[ -n "${POST_ID:-}" ]] || {
        echo "FAIL: POST_ID missing after spot-oracle-post-runnable" >&2
        return 1
    }
    log_session_use "POST_ID" "$POST_ID"
    save_walkthrough_session

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
    save_walkthrough_session

    if [[ "${ENABLE_INSURANCE_E2E:-0}" == "1" ]]; then
        log_step "ENABLE_INSURANCE_E2E=1 — preparing vault + buying coverage"
        spot_insurance_e2e_prepare || return 1
        spot_insurance_buy_coverage "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" "$BET_OPTION_ID" || return 1
        save_walkthrough_session
    fi

    log_step "Waiting for oracle resolve (~1–3 min for price-threshold claims)..."
    wait_for_market_resolved "$POST_ID" || return 1

    if [[ "${ENABLE_INSURANCE_E2E:-0}" == "1" ]]; then
        spot_insurance_claim "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" || return 1
        spot_insurance_assert_duplicate_claim_fails "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" || return 1
        save_walkthrough_session
    fi

    if [[ "${ASSUME_YES:-0}" != "1" ]]; then
        read -r -p "Market resolved. Press Enter to claim payout (or Ctrl+C to skip)... " _
    fi
    spot_claim_payout "$BETTOR_ADDRESS" "$POST_ID" "$SPOT_MARKET_ID" || return 1
    save_walkthrough_session

    print_run_summary_header "SPoT Market Walkthrough — PASS"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Claim" "$SPOT_CLAIM_TEXT"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "SpotMarket" "$SPOT_MARKET_ID"
    print_run_summary_line "Bettor" "$BETTOR_ADDRESS"
    print_run_summary_line "Bet side" "option_id=${BET_OPTION_ID}"
    print_run_summary_line "Bet amount (MIST)" "$BET_AMOUNT_MIST"
    print_run_summary_line "Bet tx" "${BET_TX_DIGEST:-<none>}"
    print_run_summary_line "Payout tx" "${PAYOUT_TX_DIGEST:-<none>}"
    if [[ "${ENABLE_INSURANCE_E2E:-0}" == "1" ]]; then
        print_run_summary_line "Insurance policy" "${INSURANCE_POLICY_ID:-<none>}"
        print_run_summary_line "Insurance claim tx" "${INSURANCE_CLAIM_TX:-<none>}"
    fi
    print_run_summary_line "Market status" "${MARKET_STATUS:-resolved}"
    print_run_summary_footer
}

run_spot_move_tests() {
    local filter="$1"
    local pkg_dir="$REPO_ROOT/crates/myso-framework/packages/myso-social"
    log_step "Running Move tests (filter=$filter)"
    (cd "$pkg_dir" && myso move test --filter "$filter") || return 1
}

run_lazy_payout_e2e() {
    log_step "SPoT lazy payout: resolve retains escrow; claim_payout + claim_creator_payout are O(1)"
    run_spot_move_tests "test_lazy_claim_creator_payout" || return 1
    print_run_summary_header "SPoT Lazy Payout — PASS (Move)"
    print_run_summary_line "Scenario" "resolve → pending tables → lazy winner/creator claims"
    print_run_summary_footer
}

run_creator_fee_e2e() {
    log_step "SPoT creator fee: distinct pending entries per referring post"
    run_spot_move_tests "test_per_referrer_creator_fees" || return 1
    print_run_summary_header "SPoT Creator Fee — PASS (Move)"
    print_run_summary_line "Scenario" "two referrers → independent claim_creator_payout"
    print_run_summary_footer
}

run_expired_reclaim_e2e() {
    log_step "SPoT expired reclaim: past creator_claim_window_ms → reclaim_expired_creator_rewards"
    run_spot_move_tests "test_reclaim_expired_creator_rewards" || return 1
    print_run_summary_header "SPoT Expired Reclaim — PASS (Move)"
    print_run_summary_line "Scenario" "unclaimed creator rewards → ecosystem (+ platform remainder)"
    print_run_summary_footer
}

run_shared_claim_e2e() {
    log_step "SPoT shared claim: multiple posts link to one claim/market"
    run_spot_move_tests "test_shared_claim_multiple_posts" || return 1
    print_run_summary_header "SPoT Shared Claim — PASS (Move)"
    print_run_summary_line "Scenario" "link_post_to_spot_claim shares open market"
    print_run_summary_footer
}

run_router_only_e2e() {
    log_step "SPoT router-only: betting via unlinked post is rejected"
    run_spot_move_tests "test_router_rejects_unlinked_post" || return 1
    print_run_summary_header "SPoT Router-Only — PASS (Move)"
    print_run_summary_line "Scenario" "place_spot_bet_for_post rejects EPostNotLinked"
    print_run_summary_footer
}

run_ownership_transfer_e2e() {
    log_step "SPoT ownership: only settlement-recorded creator can claim_creator_payout"
    run_spot_move_tests "test_settlement_creator_only_can_claim" || return 1
    print_run_summary_header "SPoT Settlement Creator — PASS (Move)"
    print_run_summary_line "Scenario" "non-creator rejected at claim_creator_payout (ENotCreator)"
    print_run_summary_footer
}

run_offchain_e2e() {
    ONCHAIN_MODE=0
    trap cleanup EXIT
    load_spot_oracle_session
    if [[ -z "${SPOT_CONFIG_ID:-}" || -z "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        log_step "Off-chain E2E: chain object IDs unset (OK for SQL-seeded review pipeline)"
    fi
    start_postgres
    run_migrations_boot || return 1
    seed_review_job

    log_step "Starting spot-oracle workers (off-chain)"
    export_spot_oracle_env
    spot_oracle_force_offchain_mode
    SPOT_ORACLE_REVIEW_POLL_INTERVAL_SECS=5 \
    SPOT_ORACLE_SCHEDULER_POLL_INTERVAL_SECS=5 \
    cargo run -p myso-spot-oracle &
    SVC_PID=$!
    wait_for_oracle_health || return 1

    wait_for_accepted_review || return 1
    wait_for_market_active "0xdeadbeef" 0 || return 1
    wait_for_evidence || return 1
    wait_for_market_resolved "0xdeadbeef" || return 1

    print_run_summary_header "SPoT Oracle E2E — PASS (off-chain)"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "SpotConfig" "${SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID:-<unset>}"
    print_run_summary_line "SpotClaimRegistry" "${SPOT_ORACLE_REGISTRY_OBJECT_ID:-<unset>}"
    print_run_summary_line "Oracle admin cap" "${SPOT_ORACLE_ADMIN_CAP_OBJECT_ID:-<unset>}"
    print_run_summary_line "oracle_reviews (accepted)" "${ACCEPTED_COUNT:-$REVIEWS_COUNT}"
    print_run_summary_line "market status" "${MARKET_STATUS:-<unknown>}"
    print_run_summary_line "evidence rows" "$EVIDENCE_COUNT"
    print_run_summary_line "sample content_hash" "${SAMPLE_HASH:-<none>}"
    print_run_summary_footer
}

run_onchain_e2e() {
    ONCHAIN_MODE=1
    trap cleanup EXIT
    load_spot_oracle_session
    require_social_stack_for_onchain || return 1
    validate_onchain_oracle_key || return 1

    start_postgres
    run_migrations_boot || return 1
    boot_onchain_oracle_workers || return 1

    log_step "Creating enable_spot=true post via spot-oracle-post-runnable"
    ASSUME_YES=1 "$SCRIPT_DIR/spot-oracle-post-runnable.sh" --run-all || return 1
    load_spot_oracle_session
    if [[ -z "${POST_ID:-}" ]]; then
        # shellcheck disable=SC1090
        source "$SOCIAL_SESSION_SAVE_PATH"
    fi
    [[ -n "${POST_ID:-}" ]] || {
        echo "FAIL: POST_ID missing after spot-oracle-post-runnable" >&2
        return 1
    }
    log_session_use "POST_ID" "$POST_ID"

    wait_for_post_ingest "$POST_ID" || return 1

    wait_for_accepted_review "$POST_ID" || return 1
    wait_for_market_active "$POST_ID" 1 || return 1
    wait_for_evidence "$POST_ID" || return 1
    wait_for_market_resolved "$POST_ID" || return 1

    print_run_summary_header "SPoT Oracle E2E — PASS (on-chain path)"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "market status" "${MARKET_STATUS:-<unknown>}"
    print_run_summary_line "spot_claim_id" "${SPOT_CLAIM_ID:-<none>}"
    print_run_summary_line "spot_market_id" "${SPOT_MARKET_ID:-<none>}"
    print_run_summary_line "SpotConfig" "${SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID:-<unset>}"
    print_run_summary_line "SpotClaimRegistry" "${SPOT_ORACLE_REGISTRY_OBJECT_ID:-<unset>}"
    print_run_summary_line "oracle_reviews (accepted)" "${ACCEPTED_COUNT:-$REVIEWS_COUNT}"
    print_run_summary_line "evidence rows" "$EVIDENCE_COUNT"
    print_run_summary_line "sample content_hash" "${SAMPLE_HASH:-<none>}"
    print_run_summary_footer
}

show_menu() {
    echo ""
    echo "=== SPoT Oracle ==="
    echo " 1) Full market walkthrough — enter your prediction, then post → bet → resolve"
    echo " 2) Refresh session from GraphQL (requires indexer at \$GRAPHQL_URL)"
    echo " ---"
    echo " Advanced / developer"
    echo " a) Off-chain review pipeline (--run-all SQL seed)"
    echo " b) On-chain ingest only (--run-all-onchain, no bet)"
    echo " c) Move test: lazy payout"
    echo " d) Move test: creator fees"
    echo " e) Move test: expired reclaim"
    echo " f) Move test: shared claim"
    echo " g) Move test: router-only rejection"
    echo " h) Move test: settlement creator gate"
    echo " ?) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        1) run_market_walkthrough ;;
        2)
            refresh_spot_oracle_session_from_graphql && load_spot_oracle_session || {
                echo "FAIL: session refresh aborted; existing session file unchanged." >&2
            }
            ;;
        [Aa]) run_offchain_e2e ;;
        [Bb]) run_onchain_e2e ;;
        [Cc]) run_lazy_payout_e2e ;;
        [Dd]) run_creator_fee_e2e ;;
        [Ee]) run_expired_reclaim_e2e ;;
        [Ff]) run_shared_claim_e2e ;;
        [Gg]) run_router_only_e2e ;;
        [Hh]) run_ownership_transfer_e2e ;;
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
            -y) ASSUME_YES=1; shift ;;
            --refresh-session) RUN_MODE=refresh; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            --run-all-onchain) RUN_MODE=run_all_onchain; shift ;;
            --run-walkthrough) RUN_MODE=run_walkthrough; shift ;;
            --run-lazy-payout) RUN_MODE=run_lazy_payout; shift ;;
            --run-creator-fee) RUN_MODE=run_creator_fee; shift ;;
            --run-expired-reclaim) RUN_MODE=run_expired_reclaim; shift ;;
            --run-shared-claim) RUN_MODE=run_shared_claim; shift ;;
            --run-router-only) RUN_MODE=run_router_only; shift ;;
            --run-ownership-transfer) RUN_MODE=run_ownership_transfer; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    # SPOT_ORACLE_ONCHAIN=1 with --run-all selects the on-chain path.
    case "${SPOT_ORACLE_ONCHAIN:-0}" in
        1|true|TRUE|yes|YES)
            if [[ "${RUN_MODE:-}" == "run_all" || -z "${RUN_MODE:-}" ]]; then
                RUN_MODE=run_all_onchain
            fi
            ;;
    esac

    local claim_from_env=''
    # Walkthrough claim comes from the interactive prompt, not a stale session/env value.
    if [[ "${RUN_MODE:-}" != "run_walkthrough" && -n "${SPOT_CLAIM_TEXT:-}" ]]; then
        claim_from_env="$SPOT_CLAIM_TEXT"
    fi

    load_spot_oracle_session
    if [[ -n "$claim_from_env" ]]; then
        SPOT_CLAIM_TEXT="$claim_from_env"
        export SPOT_CLAIM_TEXT
    fi

    case "${RUN_MODE:-}" in
        refresh)
            refresh_spot_oracle_session_from_graphql || {
                echo "FAIL: could not refresh SPoT session from GraphQL ($GRAPHQL_URL)" >&2
                exit 1
            }
            load_spot_oracle_session
            exit 0
            ;;
        run_all) run_offchain_e2e ;;
        run_all_onchain) run_onchain_e2e ;;
        run_walkthrough) run_market_walkthrough ;;
        run_lazy_payout) run_lazy_payout_e2e ;;
        run_creator_fee) run_creator_fee_e2e ;;
        run_expired_reclaim) run_expired_reclaim_e2e ;;
        run_shared_claim) run_shared_claim_e2e ;;
        run_router_only) run_router_only_e2e ;;
        run_ownership_transfer) run_ownership_transfer_e2e ;;
        '')
            if [[ ! -t 0 ]]; then
                echo "No TTY — use: ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all" >&2
                exit 1
            fi
            show_menu
            ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
