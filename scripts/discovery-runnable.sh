#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Discovery service live-fetch E2E. Proves the scheduler polls REAL RSS / GitHub /
# HTTP sources and persists `discovery_assets` rows with verifiable `content_hash`.
#
# Usage:
#   ./scripts/discovery-runnable.sh                 # run E2E + tear down
#   ./scripts/discovery-runnable.sh --refresh-session
#   KEEP_STACK=1 ./scripts/discovery-runnable.sh    # leave postgres up after E2E
#   REUSE_DB=1 KEEP_STACK=1 ./scripts/discovery-runnable.sh  # skip volume wipe
#   DISCOVERY_EMBED_ENABLED=true ./scripts/discovery-runnable.sh
#     # optional embed mode: requires PoC at DISCOVERY_EMBED_ENDPOINT or fails clearly
#
# Session env: network.config/discovery/discovery-session.env
# E2E always recreates the discovery volume unless REUSE_DB=1 (avoids stale
# sqlx checksums after greenfield migration edits).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

COMPOSE_FILE="$REPO_ROOT/crates/myso-discovery-service/docker-compose.yml"

PG_PORT="${DISCOVERY_PG_PORT:-5434}"
PG_USER="${DISCOVERY_PG_USER:-poc}"
PG_PASSWORD="${DISCOVERY_PG_PASSWORD:-poc}"
PG_DB="${DISCOVERY_PG_DB:-discovery}"
DB_URL="postgresql://${PG_USER}:${PG_PASSWORD}@127.0.0.1:${PG_PORT}/${PG_DB}"
SOURCES_CONFIG="${DISCOVERY_SOURCES_CONFIG:-crates/myso-discovery-service/config/discovery/sources.factual.localnet.yaml}"
POLL_INTERVAL="${DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS:-30}"
LISTEN="${DISCOVERY_LISTEN:-127.0.0.1:8096}"
# Avoid clash with `myso start` metrics on :9186.
METRICS="${DISCOVERY_METRICS_ADDRESS:-127.0.0.1:9286}"
KEEP_STACK="${KEEP_STACK:-0}"
WE_STARTED_PG=0
SVC_PID=""

log() { echo ">>> $*" >&2; }

psql_exec() {
  # $1 = SQL. Runs inside the compose postgres service (no TTY).
  docker compose -f "$COMPOSE_FILE" exec -T discovery-postgres \
    psql -U "$PG_USER" -d "$PG_DB" -tAc "$1"
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

EMBED_ENABLED="${DISCOVERY_EMBED_ENABLED:-false}"

require_poc_if_embed_enabled() {
  case "${EMBED_ENABLED}" in
    1|true|TRUE|yes|YES) ;;
    *) return 0 ;;
  esac
  local endpoint="${DISCOVERY_EMBED_ENDPOINT:-http://127.0.0.1:8000/internal/discovery/embed}"
  local base="${endpoint%/internal/discovery/embed}"
  base="${base:-http://127.0.0.1:8000}"
  if ! curl -sf --max-time 3 "${base}/health" >/dev/null 2>&1 \
    && ! curl -sf --max-time 3 "${base}/" >/dev/null 2>&1; then
    echo "FAIL: DISCOVERY_EMBED_ENABLED=${EMBED_ENABLED} but PoC is unreachable at ${base}" >&2
    echo "Start proof-of-creativity API or set DISCOVERY_EMBED_ENABLED=false" >&2
    return 1
  fi
  log "PoC reachable for embed mode (${base})"
}

run_discovery_service() {
  log "Building + running myso-discovery-service (poll interval ${POLL_INTERVAL}s, sources: $SOURCES_CONFIG, embed=${EMBED_ENABLED})"
  DISCOVERY_DATABASE_URL="$DB_URL" \
  DISCOVERY_SOURCES_CONFIG="$SOURCES_CONFIG" \
  DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS="$POLL_INTERVAL" \
  DISCOVERY_WORKER_CONCURRENCY="${DISCOVERY_WORKER_CONCURRENCY:-2}" \
  DISCOVERY_LISTEN="$LISTEN" \
  DISCOVERY_METRICS_ADDRESS="$METRICS" \
  DISCOVERY_ENABLED=true \
  DISCOVERY_EMBED_ENABLED="$EMBED_ENABLED" \
  DISCOVERY_EMBED_ENDPOINT="${DISCOVERY_EMBED_ENDPOINT:-http://127.0.0.1:8000/internal/discovery/embed}" \
  DISCOVERY_EMBED_SECRET="${DISCOVERY_EMBED_SECRET:-}" \
  DISCOVERY_ADMIN_SECRET="${DISCOVERY_ADMIN_SECRET:-local-discovery-admin}" \
  DISCOVERY_CLIENT_SECRET="${DISCOVERY_CLIENT_SECRET:-local-discovery-client}" \
  RUST_LOG="${RUST_LOG:-info}" \
  cargo run -p myso-discovery-service &
  SVC_PID=$!
}

wait_for_health() {
  local i
  for i in $(seq 1 180); do
    if curl -sf "http://${LISTEN}/health" >/dev/null 2>&1; then
      log "discovery service healthy at http://${LISTEN}"
      return 0
    fi
    if ! kill -0 "$SVC_PID" 2>/dev/null; then
      echo "discovery service exited early before becoming healthy" >&2
      return 1
    fi
    sleep 1
  done
  echo "discovery service did not become healthy" >&2
  return 1
}

assert_assets() {
  log "Waiting for first scheduler poll (sleep ${POLL_INTERVAL}s)..."
  sleep "$POLL_INTERVAL"
  local i count url hash source_id kind
  for i in $(seq 1 40); do
    count="$(psql_exec "SELECT COUNT(*) FROM discovery_assets" || echo 0)"
    count="${count// /}"
    if [[ -n "$count" && "$count" -gt 0 ]]; then
      log "discovery_assets count = $count"
      # Assert sample has url + content_hash + source_id + content_kind=text (factual corpus).
      local row rest
      row="$(psql_exec "SELECT external_source_url || '|' || COALESCE(content_hash,'') || '|' || COALESCE(source_id::text,'') || '|' || COALESCE(content_kind,'') FROM discovery_assets WHERE content_hash IS NOT NULL ORDER BY discovered_at DESC LIMIT 1" || true)"
      url="${row%%|*}"
      rest="${row#*|}"
      hash="${rest%%|*}"
      rest="${rest#*|}"
      source_id="${rest%%|*}"
      kind="${rest#*|}"
      kind="${kind//$'\n'/}"
      if [[ -z "$hash" ]]; then
        echo "FAIL: sample discovery_asset has null content_hash" >&2
        return 1
      fi
      if [[ -z "$url" ]]; then
        echo "FAIL: sample discovery_asset has null external_source_url" >&2
        return 1
      fi
      if [[ -z "$source_id" ]]; then
        echo "FAIL: sample discovery_asset has null source_id" >&2
        return 1
      fi
      if [[ "$kind" != "text" ]]; then
        echo "FAIL: expected content_kind=text for factual corpus, got '${kind}'" >&2
        return 1
      fi
      log "sample asset: url=$url hash=$hash source_id=$source_id content_kind=$kind"
      log "PASS: real source fetch produced discovery_assets with verifiable content_hash + source_id"
      return 0
    fi
    sleep 5
  done
  echo "FAIL: no discovery_assets rows after polling (check source connectivity / RUST_LOG)" >&2
  return 1
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
    log "Tearing down discovery postgres"
    docker compose -f "$COMPOSE_FILE" down -v >/dev/null 2>&1 || true
  fi
  return "$rc"
}

DISCOVERY_SESSION_FILE="$REPO_ROOT/network.config/discovery/discovery-session.env"

refresh_discovery_session() {
  mkdir -p "$(dirname "$DISCOVERY_SESSION_FILE")"
  {
    echo "# Discovery service runtime session — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf '%s=%q\n' DISCOVERY_DATABASE_URL "$DB_URL"
    printf '%s=%q\n' DISCOVERY_SOURCES_CONFIG "$SOURCES_CONFIG"
    printf '%s=%q\n' DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS "$POLL_INTERVAL"
    printf '%s=%q\n' DISCOVERY_WORKER_CONCURRENCY "${DISCOVERY_WORKER_CONCURRENCY:-2}"
    printf '%s=%q\n' DISCOVERY_LISTEN "$LISTEN"
    printf '%s=%q\n' DISCOVERY_METRICS_ADDRESS "$METRICS"
    printf '%s=%q\n' DISCOVERY_ENABLED true
    printf '%s=%q\n' DISCOVERY_EMBED_ENABLED "${DISCOVERY_EMBED_ENABLED:-false}"
    printf '%s=%q\n' DISCOVERY_EMBED_ENDPOINT "${DISCOVERY_EMBED_ENDPOINT:-http://127.0.0.1:8000/internal/discovery/embed}"
    printf '%s=%q\n' DISCOVERY_ADMIN_SECRET "${DISCOVERY_ADMIN_SECRET:-local-discovery-admin}"
    printf '%s=%q\n' DISCOVERY_MAX_RETRIES "${DISCOVERY_MAX_RETRIES:-5}"
    printf '%s=%q\n' RUST_LOG "${RUST_LOG:-info}"
  } > "${DISCOVERY_SESSION_FILE}.tmp"
  mv "${DISCOVERY_SESSION_FILE}.tmp" "$DISCOVERY_SESSION_FILE"
  log "Wrote session to $DISCOVERY_SESSION_FILE"
}

load_discovery_session() {
  if [[ -f "$DISCOVERY_SESSION_FILE" ]]; then
    # shellcheck disable=SC1090
    source "$DISCOVERY_SESSION_FILE"
    log "Loaded session from $DISCOVERY_SESSION_FILE"
  fi
}

main() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --help|-h)
        sed -n '2,16p' "$0" | sed 's/^# \?//'
        exit 0
        ;;
      --refresh-session)
        refresh_discovery_session
        exit 0
        ;;
      *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
  done

  load_discovery_session
  EMBED_ENABLED="${DISCOVERY_EMBED_ENABLED:-false}"
  trap cleanup EXIT
  start_postgres
  require_poc_if_embed_enabled
  run_discovery_service
  wait_for_health
  assert_assets
}

main "$@"
