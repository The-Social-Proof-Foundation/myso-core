#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# PoC off-network discovery E2E. Proves creative media crawl → internal embed → indexed
# lifecycle inside proof-of-creativity (no discovery-service).
#
# Prerequisites:
#   - PoC docker stack with discovery-worker (default :8000)
#   - docker, curl, python3 on PATH
#
# Usage:
#   ./scripts/discovery-poc-runnable.sh
#   POC_ORACLE_URL=http://127.0.0.1:8001 ./scripts/discovery-poc-runnable.sh
#   REUSE_POC=1 ./scripts/discovery-poc-runnable.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

POC_URL="${POC_ORACLE_URL:-http://127.0.0.1:8000}"
POC_REPO="${MYSO_POC_REPO:-$(cd "${REPO_ROOT}/../proof-of-creativity" 2>/dev/null && pwd || true)}"
PG_PORT="${POSTGRES_PORT:-5432}"
PG_USER="${POSTGRES_USER:-poc}"
PG_PASSWORD="${POSTGRES_PASSWORD:-poc}"
PG_DB="${POSTGRES_DB:-proof_of_creativity}"
DB_URL="postgresql://${PG_USER}:${PG_PASSWORD}@127.0.0.1:${PG_PORT}/${PG_DB}"
EXPECTED="${DISCOVERY_EXPECTED_ASSETS:-3}"
POLL_INTERVAL="${DISCOVERY_POLL_INTERVAL_SECONDS:-30}"

log() { echo ">>> $*" >&2; }

psql_exec() {
  PGPASSWORD="$PG_PASSWORD" psql -h 127.0.0.1 -p "$PG_PORT" -U "$PG_USER" -d "$PG_DB" -tAc "$1"
}

require_poc() {
  if ! curl -sf --max-time 5 "${POC_URL}/health" >/dev/null 2>&1; then
    echo "FAIL: PoC unreachable at ${POC_URL}" >&2
    echo "Start: docker compose --profile app up postgres redis api grpc-sync oracle-worker discovery-worker" >&2
    return 1
  fi
  log "PoC reachable at ${POC_URL}"
  if ! psql_exec "SELECT 1 FROM information_schema.tables WHERE table_name = 'discovery_assets' LIMIT 1" | grep -q 1; then
    echo "FAIL: discovery_assets table missing — run alembic upgrade head in proof-of-creativity" >&2
    return 1
  fi
  log "discovery_assets table present"
}

trigger_poll() {
  if [[ -z "$POC_REPO" || ! -d "$POC_REPO" ]]; then
    log "WARN: MYSO_POC_REPO not set — relying on discovery-worker poll only"
    return 0
  fi
  log "Triggering one-shot discovery scheduler poll"
  (
    cd "$POC_REPO"
    DATABASE_URL="$DB_URL" \
    DISCOVERY_SOURCES_CONFIG="${DISCOVERY_SOURCES_CONFIG:-config/discovery/sources.localnet.yaml}" \
    POC_USE_MANUAL_CURATED="${POC_USE_MANUAL_CURATED:-1}" \
    python3 scripts/run_discovery_poll_once.py
  ) || true
}

assert_indexed() {
  log "Waiting for indexed discovery assets (poll ${POLL_INTERVAL}s + embed)..."
  sleep "$POLL_INTERVAL"
  local i count indexed kind
  for i in $(seq 1 60); do
    count="$(psql_exec "SELECT COUNT(*) FROM discovery_assets" || echo 0)"
    count="${count// /}"
    indexed="$(psql_exec "SELECT COUNT(*) FROM discovery_assets WHERE lifecycle_state = 'indexed'" || echo 0)"
    indexed="${indexed// /}"
    if [[ -n "$count" && "$count" -ge "$EXPECTED" && -n "$indexed" && "$indexed" -ge "$EXPECTED" ]]; then
      kind="$(psql_exec "SELECT content_kind FROM discovery_assets WHERE lifecycle_state = 'indexed' ORDER BY updated_at DESC LIMIT 1" || true)"
      kind="${kind// /}"
      kind="${kind//$'\n'/}"
      if [[ "$kind" != "media" ]]; then
        echo "FAIL: expected content_kind=media, got '${kind}'" >&2
        return 1
      fi
      log "discovery_assets=$count indexed=$indexed content_kind=$kind"
      log "PASS: all ${EXPECTED} media assets indexed"
      return 0
    fi
    sleep 5
  done
  echo "FAIL: expected ${EXPECTED}/${EXPECTED} indexed assets" >&2
  psql_exec "SELECT lifecycle_state, COUNT(*) FROM discovery_assets GROUP BY lifecycle_state ORDER BY lifecycle_state" >&2 || true
  psql_exec "SELECT status, COUNT(*) FROM discovery_jobs GROUP BY status ORDER BY status" >&2 || true
  return 1
}

simulate_lifecycle_transition() {
  local asset_id
  asset_id="$(psql_exec "SELECT id::text FROM discovery_assets WHERE lifecycle_state = 'indexed' ORDER BY updated_at DESC LIMIT 1" || true)"
  asset_id="${asset_id// /}"
  if [[ -z "$asset_id" ]]; then
    echo "FAIL: no indexed asset for lifecycle transition" >&2
    return 1
  fi
  if [[ -n "$POC_REPO" && -d "$POC_REPO" ]]; then
    log "Transitioning asset $asset_id to matched via DiscoveryStore"
    (
      cd "$POC_REPO"
      DATABASE_URL="$DB_URL" python3 -c "
from app.discovery.store import DiscoveryStore
DiscoveryStore().transition_asset('${asset_id}', 'match_detected')
"
    )
  fi
  local state
  state="$(psql_exec "SELECT lifecycle_state FROM discovery_assets WHERE id = '${asset_id}'" || true)"
  state="${state// /}"
  if [[ "$state" != "matched" ]]; then
    echo "FAIL: expected lifecycle_state=matched got '${state}'" >&2
    return 1
  fi
  log "PASS: lifecycle advanced asset to matched"
}

main() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --help|-h) sed -n '2,18p' "$0" | sed 's/^# \?//'; exit 0 ;;
      *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
  done
  require_poc
  trigger_poll
  assert_indexed
  simulate_lifecycle_transition
  log "PASS: discovery-poc E2E complete"
}

main "$@"
