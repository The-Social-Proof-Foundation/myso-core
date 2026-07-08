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
#   - Social-server at http://127.0.0.1:9126
#   - docker, curl, jq, cargo on PATH
#
# Session: network.config/spot-oracle/spot-oracle-session.env
#
# Usage:
#   ./scripts/spot-oracle-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all
#   ./scripts/spot-oracle-runnable.sh   # interactive menu

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

usage() {
    sed -n '2,20p' "$0" | sed 's/^# \?//'
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
    spot_oracle_force_offchain_mode
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
    VALUES ('0xdeadbeef', '0xowner', 'Will BTC trade above \$1?', 'pending_review', '[\"Yes\",\"No\"]'::jsonb)
    ON CONFLICT (post_id) DO NOTHING;
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

run_offchain_e2e() {
    trap cleanup EXIT
    load_spot_oracle_session
    if [[ -z "${SPOT_CONFIG_ID:-}" || -z "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        log_step "Off-chain E2E: chain object IDs unset (OK for SQL-seeded review pipeline)"
    fi
    start_postgres
    run_migrations_boot || return 1
    seed_review_job

    log_step "Starting spot-oracle workers"
    export_spot_oracle_env
    spot_oracle_force_offchain_mode
    SPOT_ORACLE_REVIEW_POLL_INTERVAL_SECS=5 \
    SPOT_ORACLE_SCHEDULER_POLL_INTERVAL_SECS=5 \
    cargo run -p myso-spot-oracle &
    SVC_PID=$!
    wait_for_oracle_health || return 1

    local reviews=0 evidence=0 hash='' i
    log_step "Waiting for review pipeline (up to 3 min)..."
    for i in $(seq 1 90); do
        reviews="$(psql_exec "SELECT COUNT(*) FROM oracle_reviews" || echo 0)"
        reviews="${reviews// /}"
        if [[ -n "$reviews" && "$reviews" -gt 0 ]]; then
            break
        fi
        sleep 2
    done
    if [[ -z "$reviews" || "$reviews" -eq 0 ]]; then
        echo "FAIL: no oracle_reviews after pipeline" >&2
        return 1
    fi

    log_step "Waiting for resolve + evidence (maturity ~1 min, up to 3 min)..."
    for i in $(seq 1 90); do
        evidence="$(psql_exec "SELECT COUNT(*) FROM evidence" || echo 0)"
        evidence="${evidence// /}"
        if [[ -n "$evidence" && "$evidence" -gt 0 ]]; then
            hash="$(psql_exec "SELECT content_hash FROM evidence ORDER BY fetched_at DESC LIMIT 1" || true)"
            hash="${hash// /}"
            break
        fi
        sleep 2
    done
    if [[ -z "$evidence" || "$evidence" -eq 0 ]]; then
        echo "FAIL: no evidence rows after review+resolve pipeline" >&2
        return 1
    fi

    print_run_summary_header "SPoT Oracle E2E — PASS"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "SpotConfig" "${SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID:-<unset>}"
    print_run_summary_line "Oracle admin cap" "${SPOT_ORACLE_ADMIN_CAP_OBJECT_ID:-<unset>}"
    print_run_summary_line "oracle_reviews" "$reviews"
    print_run_summary_line "evidence rows" "$evidence"
    print_run_summary_line "sample content_hash" "${hash:-<none>}"
    print_run_summary_footer
}

show_menu() {
    echo ""
    echo "=== SPoT Oracle E2E Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Run off-chain E2E (--run-all)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) refresh_spot_oracle_session_from_graphql; load_spot_oracle_session ;;
        1) run_offchain_e2e ;;
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

    load_spot_oracle_session

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
