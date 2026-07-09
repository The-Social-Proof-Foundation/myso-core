#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Discovery ↔ PoC embed E2E. Proves creative media crawl → embed → indexed
# lifecycle when the proof-of-creativity API is running with matching
# DISCOVERY_EMBED_SECRET. Uses sources.media.localnet.yaml (manual_curated
# public images), not factual RSS/JSON.
#
# Prerequisites:
#   - PoC API (default :8000; override DISCOVERY_EMBED_ENDPOINT for :8001)
#   - Matching DISCOVERY_EMBED_SECRET on both sides
#   - docker, curl, cargo on PATH
#
# Session: network.config/discovery/discovery-session.env
#
# Usage:
#   DISCOVERY_EMBED_SECRET=devsecret ./scripts/discovery-poc-runnable.sh
#   DISCOVERY_EMBED_ENDPOINT=http://127.0.0.1:8001/internal/discovery/embed \
#     DISCOVERY_EMBED_SECRET=devsecret ./scripts/discovery-poc-runnable.sh
#   KEEP_STACK=1 ./scripts/discovery-poc-runnable.sh
#   ./scripts/discovery-poc-runnable.sh --refresh-session

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

COMPOSE_FILE="$REPO_ROOT/crates/myso-discovery-service/docker-compose.yml"
DISCOVERY_SESSION_FILE="$REPO_ROOT/network.config/discovery/discovery-session.env"

PG_PORT="${DISCOVERY_PG_PORT:-5434}"
PG_USER="${DISCOVERY_PG_USER:-poc}"
PG_PASSWORD="${DISCOVERY_PG_PASSWORD:-poc}"
PG_DB="${DISCOVERY_PG_DB:-discovery}"
DB_URL="postgresql://${PG_USER}:${PG_PASSWORD}@127.0.0.1:${PG_PORT}/${PG_DB}"
SOURCES_CONFIG="${DISCOVERY_SOURCES_CONFIG:-crates/myso-discovery-service/config/discovery/sources.media.localnet.yaml}"
POLL_INTERVAL="${DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS:-30}"
LISTEN="${DISCOVERY_LISTEN:-127.0.0.1:8096}"
# Avoid clash with `myso start` metrics on :9186.
METRICS="${DISCOVERY_METRICS_ADDRESS:-127.0.0.1:9286}"
EMBED_ENDPOINT="${DISCOVERY_EMBED_ENDPOINT:-http://127.0.0.1:8000/internal/discovery/embed}"
EMBED_SECRET="${DISCOVERY_EMBED_SECRET:-}"
ADMIN_SECRET="${DISCOVERY_ADMIN_SECRET:-local-discovery-admin}"
KEEP_STACK="${KEEP_STACK:-0}"
WE_STARTED_PG=0
SVC_PID=""
# Required for sources.media.localnet.yaml (manual_curated adapter).
export DISCOVERY_USE_MANUAL_CURATED="${DISCOVERY_USE_MANUAL_CURATED:-1}"

log() { echo ">>> $*" >&2; }

psql_exec() {
  docker compose -f "$COMPOSE_FILE" exec -T discovery-postgres \
    psql -U "$PG_USER" -d "$PG_DB" -tAc "$1"
}

require_poc() {
  local base="${EMBED_ENDPOINT%/internal/discovery/embed}"
  base="${base:-http://127.0.0.1:8000}"
  if [[ -z "$EMBED_SECRET" ]]; then
    echo "FAIL: DISCOVERY_EMBED_SECRET is required for discovery-poc-runnable" >&2
    echo "Export the same secret configured on the PoC API" >&2
    return 1
  fi
  if ! curl -sf --max-time 5 "${base}/health" >/dev/null 2>&1 \
    && ! curl -sf --max-time 5 "${base}/" >/dev/null 2>&1; then
    echo "FAIL: PoC unreachable at ${base}" >&2
    echo "From proof-of-creativity: docker compose --profile app up postgres redis api oracle-worker" >&2
    return 1
  fi
  log "PoC reachable at ${base}"
}

start_postgres() {
  if [[ "${REUSE_DB:-0}" == "1" ]] \
    && docker compose -f "$COMPOSE_FILE" ps discovery-postgres 2>/dev/null | grep -q "running\|healthy"; then
    log "REUSE_DB=1 — reusing running discovery postgres"
    return 0
  fi
  log "Starting discovery postgres (fresh volume for E2E)"
  docker compose -f "$COMPOSE_FILE" down -v >/dev/null 2>&1 || true
  docker compose -f "$COMPOSE_FILE" up -d discovery-postgres
  WE_STARTED_PG=1
  local i
  for i in $(seq 1 60); do
    if docker compose -f "$COMPOSE_FILE" exec -T discovery-postgres pg_isready -U "$PG_USER" -d "$PG_DB" >/dev/null 2>&1; then
      return 0
    fi
    sleep 1
  done
  echo "discovery postgres did not become ready" >&2
  return 1
}

run_discovery_service() {
  log "Running myso-discovery-service with embed enabled (media corpus)"
  DISCOVERY_DATABASE_URL="$DB_URL" \
  DISCOVERY_SOURCES_CONFIG="$SOURCES_CONFIG" \
  DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS="$POLL_INTERVAL" \
  DISCOVERY_WORKER_CONCURRENCY="${DISCOVERY_WORKER_CONCURRENCY:-2}" \
  DISCOVERY_LISTEN="$LISTEN" \
  DISCOVERY_METRICS_ADDRESS="$METRICS" \
  DISCOVERY_ENABLED=true \
  DISCOVERY_EMBED_ENABLED=true \
  DISCOVERY_EMBED_ENDPOINT="$EMBED_ENDPOINT" \
  DISCOVERY_EMBED_SECRET="$EMBED_SECRET" \
  DISCOVERY_ADMIN_SECRET="$ADMIN_SECRET" \
  DISCOVERY_USE_MANUAL_CURATED="${DISCOVERY_USE_MANUAL_CURATED}" \
  RUST_LOG="${RUST_LOG:-info}" \
  cargo run -p myso-discovery-service &
  SVC_PID=$!
}

wait_for_health() {
  local i
  for i in $(seq 1 180); do
    if curl -sf "http://${LISTEN}/health" >/dev/null 2>&1; then
      log "discovery healthy at http://${LISTEN}"
      return 0
    fi
    if ! kill -0 "$SVC_PID" 2>/dev/null; then
      echo "discovery service exited early" >&2
      return 1
    fi
    sleep 1
  done
  echo "discovery service did not become healthy" >&2
  return 1
}

assert_indexed() {
  log "Waiting for indexed assets (poll ${POLL_INTERVAL}s + embed)..."
  sleep "$POLL_INTERVAL"
  local i count indexed kind
  for i in $(seq 1 60); do
    count="$(psql_exec "SELECT COUNT(*) FROM discovery_assets" || echo 0)"
    count="${count// /}"
    indexed="$(psql_exec "SELECT COUNT(*) FROM discovery_assets WHERE lifecycle_state = 'indexed'" || echo 0)"
    indexed="${indexed// /}"
    if [[ -n "$indexed" && "$indexed" -gt 0 ]]; then
      kind="$(psql_exec "SELECT content_kind FROM discovery_assets WHERE lifecycle_state = 'indexed' ORDER BY updated_at DESC LIMIT 1" || true)"
      kind="${kind// /}"
      kind="${kind//$'\n'/}"
      if [[ "$kind" != "media" ]]; then
        echo "FAIL: expected content_kind=media for PoC corpus, got '${kind}'" >&2
        return 1
      fi
      log "discovery_assets=$count indexed=$indexed content_kind=$kind"
      log "PASS: embed path produced indexed discovery_assets"
      return 0
    fi
    sleep 5
  done
  echo "FAIL: no indexed assets (check PoC embed endpoint / DISCOVERY_EMBED_SECRET / RUST_LOG)" >&2
  echo "  total assets=${count:-0} indexed=${indexed:-0}" >&2
  return 1
}

simulate_lifecycle_callback() {
  local asset_id
  asset_id="$(psql_exec "SELECT id::text FROM discovery_assets WHERE lifecycle_state = 'indexed' ORDER BY updated_at DESC LIMIT 1" || true)"
  asset_id="${asset_id// /}"
  if [[ -z "$asset_id" ]]; then
    echo "FAIL: no indexed asset for lifecycle callback" >&2
    return 1
  fi
  log "POST /internal/lifecycle match_detected for $asset_id"
  curl -sf -X POST "http://${LISTEN}/internal/lifecycle" \
    -H 'content-type: application/json' \
    -d "{\"discovery_asset_id\":\"${asset_id}\",\"event\":\"match_detected\"}" >/dev/null
  local state
  state="$(psql_exec "SELECT lifecycle_state FROM discovery_assets WHERE id = '${asset_id}'" || true)"
  state="${state// /}"
  if [[ "$state" != "matched" ]]; then
    echo "FAIL: expected lifecycle_state=matched got '${state}'" >&2
    return 1
  fi
  log "PASS: lifecycle callback advanced asset to matched"
}

assert_metrics() {
  if curl -sf "http://${METRICS}/metrics" 2>/dev/null | grep -q 'discovery_'; then
    log "PASS: metrics scrapeable at http://${METRICS}/metrics"
  else
    log "WARN: metrics not found at ${METRICS} (non-fatal)"
  fi
}

cleanup() {
  local rc=$?
  if [[ -n "$SVC_PID" ]]; then
    kill "$SVC_PID" 2>/dev/null || true
    wait "$SVC_PID" 2>/dev/null || true
  fi
  if [[ "$KEEP_STACK" == "1" ]]; then
    log "KEEP_STACK=1 — leaving postgres running"
    return
  fi
  if [[ "$WE_STARTED_PG" == "1" ]]; then
    docker compose -f "$COMPOSE_FILE" down -v >/dev/null 2>&1 || true
  fi
  return "$rc"
}

refresh_session() {
  mkdir -p "$(dirname "$DISCOVERY_SESSION_FILE")"
  {
    echo "# Discovery↔PoC session — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf '%s=%q\n' DISCOVERY_DATABASE_URL "$DB_URL"
    printf '%s=%q\n' DISCOVERY_SOURCES_CONFIG "$SOURCES_CONFIG"
    printf '%s=%q\n' DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS "$POLL_INTERVAL"
    printf '%s=%q\n' DISCOVERY_LISTEN "$LISTEN"
    printf '%s=%q\n' DISCOVERY_METRICS_ADDRESS "$METRICS"
    printf '%s=%q\n' DISCOVERY_ENABLED true
    printf '%s=%q\n' DISCOVERY_EMBED_ENABLED true
    printf '%s=%q\n' DISCOVERY_EMBED_ENDPOINT "$EMBED_ENDPOINT"
    printf '%s=%q\n' DISCOVERY_EMBED_SECRET "$EMBED_SECRET"
    printf '%s=%q\n' DISCOVERY_ADMIN_SECRET "$ADMIN_SECRET"
    printf '%s=%q\n' DISCOVERY_USE_MANUAL_CURATED "${DISCOVERY_USE_MANUAL_CURATED:-1}"
    printf '%s=%q\n' RUST_LOG "${RUST_LOG:-info}"
  } > "${DISCOVERY_SESSION_FILE}.tmp"
  mv "${DISCOVERY_SESSION_FILE}.tmp" "$DISCOVERY_SESSION_FILE"
  log "Wrote $DISCOVERY_SESSION_FILE"
}

main() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --help|-h) sed -n '2,20p' "$0" | sed 's/^# \?//'; exit 0 ;;
      --refresh-session) refresh_session; exit 0 ;;
      *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
  done

  if [[ -f "$DISCOVERY_SESSION_FILE" ]]; then
    # shellcheck disable=SC1090
    source "$DISCOVERY_SESSION_FILE"
    EMBED_SECRET="${DISCOVERY_EMBED_SECRET:-$EMBED_SECRET}"
    EMBED_ENDPOINT="${DISCOVERY_EMBED_ENDPOINT:-$EMBED_ENDPOINT}"
  fi

  trap cleanup EXIT
  require_poc
  start_postgres
  run_discovery_service
  wait_for_health
  assert_indexed
  simulate_lifecycle_callback
  assert_metrics
  log "PASS: discovery-poc E2E complete"
}

main "$@"
