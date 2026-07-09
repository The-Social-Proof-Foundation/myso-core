#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared helpers for SPoT oracle runnable scripts.
# Source after setting REPO_ROOT and SOCIAL_SESSION_SAVE_PATH.

: "${REPO_ROOT:?REPO_ROOT must be set before sourcing spot-oracle-common.sh}"
: "${SOCIAL_SESSION_SAVE_PATH:=$REPO_ROOT/network.config/spot-oracle/spot-oracle-session.env}"

# shellcheck source=lib/social-runtime-common.sh
source "${REPO_ROOT}/scripts/lib/social-runtime-common.sh"

readonly SPOT_DEFAULT_ORACLE_ADDRESS='0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8'
readonly SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX='736c869f584b6fdf1d961541e515304cdbeaf8e3d7789ae79fd05e2d9da34578'

readonly SPOT_ORACLE_GQL_EXTRAS='query SpotOracleSessionExtras {
  spotConfig: objects(filter: { type: "0x50c1::social_proof_of_truth::SpotConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  spotClaimRegistry: objects(filter: { type: "0x50c1::social_proof_of_truth::SpotClaimRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  spotOracleAdminCap: objects(filter: { type: "0x50c1::social_proof_of_truth::SpotOracleAdminCap" }, last: 1) { nodes { address } }
}'

SPOT_CONFIG_ID=''
SPOT_REGISTRY_ID=''
SPOT_ORACLE_ADMIN_CAP_ID=''
ORACLE_ADDRESS=''

SPOT_ORACLE_SESSION_KEYS=(
    GRAPHQL_URL SOCIAL_SERVER_URL PKG_SOCIAL CLOCK_ID
    ECOSYSTEM_TREASURY_ID PLATFORM_OBJECT_ID PLATFORM_CONFIG_ID POST_CONFIG_ID
    SPOT_CONFIG_ID SPOT_REGISTRY_ID SPOT_ORACLE_ADMIN_CAP_ID ORACLE_ADDRESS
    SPOT_ORACLE_DATABASE_URL SPOT_ORACLE_SOURCES_CONFIG SPOT_ORACLE_LISTEN
    SPOT_ORACLE_METRICS_ADDRESS SPOT_ORACLE_SOCIAL_SERVER_URL SPOT_ORACLE_MYSO_RPC
    SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID SPOT_ORACLE_REGISTRY_OBJECT_ID SPOT_ORACLE_ADMIN_CAP_OBJECT_ID SPOT_ORACLE_PRIVATE_KEY_HEX
    SPOT_ORACLE_PLATFORM_OBJECT_ID SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID
    SPOT_ORACLE_SOCIAL_SYNC_SECRET SPOT_ORACLE_SYNC_SECRET
    SPOT_ORACLE_ENABLED SPOT_ORACLE_LIVE_SOURCES RUST_LOG
    SPOT_ORACLE_STREAMING_URL SPOT_ORACLE_INGEST_MODE
    SPOT_ORACLE_DISCOVERY_CLIENT_URL SPOT_ORACLE_DISCOVERY_CLIENT_SECRET
    DISCOVERY_CLIENT_SECRET DISCOVERY_LISTEN
)

spot_oracle_normalize_bool_env() {
    local name="$1" val
    val="${!name:-}"
    case "$val" in
        1|yes|y|on|YES|Y|ON|Yes) printf -v "$name" '%s' 'true' ;;
        0|no|n|off|NO|N|OFF|No) printf -v "$name" '%s' 'false' ;;
        true|false) ;;
        '') printf -v "$name" '%s' 'false' ;;
        *) printf -v "$name" '%s' 'false' ;;
    esac
}

spot_oracle_apply_runtime_defaults() {
    GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
    SOCIAL_SERVER_URL="${SOCIAL_SERVER_URL:-http://127.0.0.1:9126}"
    SPOT_ORACLE_DATABASE_URL="${SPOT_ORACLE_DATABASE_URL:-postgresql://spot:spot@127.0.0.1:5435/spot_oracle}"
    SPOT_ORACLE_SOURCES_CONFIG="${SPOT_ORACLE_SOURCES_CONFIG:-crates/myso-spot-oracle/config/discovery/sources.localnet.yaml}"
    SPOT_ORACLE_LISTEN="${SPOT_ORACLE_LISTEN:-127.0.0.1:8097}"
    SPOT_ORACLE_METRICS_ADDRESS="${SPOT_ORACLE_METRICS_ADDRESS:-127.0.0.1:9187}"
    SPOT_ORACLE_SOCIAL_SERVER_URL="${SPOT_ORACLE_SOCIAL_SERVER_URL:-$SOCIAL_SERVER_URL}"
    SPOT_ORACLE_MYSO_RPC="${SPOT_ORACLE_MYSO_RPC:-http://127.0.0.1:9000}"
    SPOT_ORACLE_ENABLED="${SPOT_ORACLE_ENABLED:-true}"
    SPOT_ORACLE_LIVE_SOURCES="${SPOT_ORACLE_LIVE_SOURCES:-true}"
    SPOT_ORACLE_INGEST_MODE="${SPOT_ORACLE_INGEST_MODE:-checkpoint}"
    SPOT_ORACLE_STREAMING_URL="${SPOT_ORACLE_STREAMING_URL:-$SPOT_ORACLE_MYSO_RPC}"
    DISCOVERY_LISTEN="${DISCOVERY_LISTEN:-127.0.0.1:8096}"
    SPOT_ORACLE_DISCOVERY_CLIENT_URL="${SPOT_ORACLE_DISCOVERY_CLIENT_URL:-http://${DISCOVERY_LISTEN}}"
    DISCOVERY_CLIENT_SECRET="${DISCOVERY_CLIENT_SECRET:-local-discovery-client}"
    SPOT_ORACLE_DISCOVERY_CLIENT_SECRET="${SPOT_ORACLE_DISCOVERY_CLIENT_SECRET:-$DISCOVERY_CLIENT_SECRET}"
    RUST_LOG="${RUST_LOG:-info}"
    ORACLE_ADDRESS="${ORACLE_ADDRESS:-$SPOT_DEFAULT_ORACLE_ADDRESS}"
    SPOT_ORACLE_PRIVATE_KEY_HEX="${SPOT_ORACLE_PRIVATE_KEY_HEX:-$SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX}"
    # Align social-server gate with oracle client (same secret both sides).
    SPOT_ORACLE_SOCIAL_SYNC_SECRET="${SPOT_ORACLE_SOCIAL_SYNC_SECRET:-${SPOT_ORACLE_SYNC_SECRET:-local-spot-oracle-sync}}"
    SPOT_ORACLE_SYNC_SECRET="${SPOT_ORACLE_SYNC_SECRET:-$SPOT_ORACLE_SOCIAL_SYNC_SECRET}"
    spot_oracle_normalize_bool_env SPOT_ORACLE_ENABLED
    spot_oracle_normalize_bool_env SPOT_ORACLE_LIVE_SOURCES
}

spot_oracle_map_session_to_oracle_env() {
    if [[ -n "${SPOT_CONFIG_ID:-}" ]]; then
        SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID="$SPOT_CONFIG_ID"
    fi
    if [[ -n "${SPOT_REGISTRY_ID:-}" ]]; then
        SPOT_ORACLE_REGISTRY_OBJECT_ID="$SPOT_REGISTRY_ID"
    fi
    if [[ -n "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        SPOT_ORACLE_ADMIN_CAP_OBJECT_ID="$SPOT_ORACLE_ADMIN_CAP_ID"
    fi
    if [[ -n "${PLATFORM_OBJECT_ID:-}" ]]; then
        SPOT_ORACLE_PLATFORM_OBJECT_ID="$PLATFORM_OBJECT_ID"
    fi
    if [[ -n "${ECOSYSTEM_TREASURY_ID:-}" ]]; then
        SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID="$ECOSYSTEM_TREASURY_ID"
    fi
}

load_spot_oracle_session() {
    if [[ -f "$SOCIAL_SESSION_SAVE_PATH" ]]; then
        # shellcheck disable=SC1090
        source "$SOCIAL_SESSION_SAVE_PATH"
        echo "Loaded session from: $SOCIAL_SESSION_SAVE_PATH" >&2
    fi
    social_apply_defaults
    spot_oracle_apply_runtime_defaults
    spot_oracle_map_session_to_oracle_env
}

save_spot_oracle_session() {
    spot_oracle_map_session_to_oracle_env
    social_save_session "${SPOT_ORACLE_SESSION_KEYS[@]}"
}

export_spot_oracle_env() {
    load_spot_oracle_session
    export GRAPHQL_URL SOCIAL_SERVER_URL PKG_SOCIAL CLOCK_ID
    export ECOSYSTEM_TREASURY_ID PLATFORM_OBJECT_ID PLATFORM_CONFIG_ID POST_CONFIG_ID
    export SPOT_CONFIG_ID SPOT_REGISTRY_ID SPOT_ORACLE_ADMIN_CAP_ID ORACLE_ADDRESS
    export SPOT_ORACLE_DATABASE_URL SPOT_ORACLE_SOURCES_CONFIG SPOT_ORACLE_LISTEN
    export SPOT_ORACLE_METRICS_ADDRESS SPOT_ORACLE_SOCIAL_SERVER_URL SPOT_ORACLE_MYSO_RPC
    export SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID SPOT_ORACLE_REGISTRY_OBJECT_ID SPOT_ORACLE_ADMIN_CAP_OBJECT_ID SPOT_ORACLE_PRIVATE_KEY_HEX
    export SPOT_ORACLE_PLATFORM_OBJECT_ID SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID
    export SPOT_ORACLE_SOCIAL_SYNC_SECRET SPOT_ORACLE_SYNC_SECRET
    export SPOT_ORACLE_ENABLED SPOT_ORACLE_LIVE_SOURCES RUST_LOG
    export SPOT_ORACLE_STREAMING_URL SPOT_ORACLE_INGEST_MODE
    export SPOT_ORACLE_DISCOVERY_CLIENT_URL SPOT_ORACLE_DISCOVERY_CLIENT_SECRET
    export DISCOVERY_CLIENT_SECRET DISCOVERY_LISTEN
}

require_social_stack_for_onchain() {
    local gql="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
    local social="${SOCIAL_SERVER_URL:-http://127.0.0.1:9126}"
    if ! curl -sf --max-time 3 "$gql" -H 'content-type: application/json' \
        -d '{"query":"{ __typename }"}' >/dev/null 2>&1; then
        echo "FAIL: GraphQL unreachable at $gql — start localnet indexer" >&2
        return 1
    fi
    if ! curl -sf --max-time 3 "${social}/health" >/dev/null 2>&1 \
        && ! curl -sf --max-time 3 "${social}/ready" >/dev/null 2>&1; then
        echo "FAIL: social-server unreachable at $social" >&2
        return 1
    fi
    local ingest_mode="${SPOT_ORACLE_INGEST_MODE:-checkpoint}"
    if [[ "$ingest_mode" == "http" || "$ingest_mode" == "both" ]]; then
        local code
        code="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 5 \
            -H "x-spot-oracle-sync-secret: ${SPOT_ORACLE_SOCIAL_SYNC_SECRET:-local-spot-oracle-sync}" \
            "${social}/spot/pending-posts?limit=1" || echo 000)"
        if [[ "$code" == "404" ]]; then
            echo "FAIL: GET /spot/pending-posts returned 404 — rebuild/restart social-server with SPoT routes" >&2
            return 1
        fi
        if [[ "$code" != "200" ]]; then
            echo "FAIL: GET /spot/pending-posts returned HTTP $code (expected 200)" >&2
            return 1
        fi
        log_step "social-server pending-posts OK (HTTP $code)"
    else
        log_step "checkpoint ingest mode — skipping pending-posts probe"
    fi
}

validate_onchain_oracle_key() {
    require_spot_oracle_session_fields || return 1
    if [[ -z "${SPOT_ORACLE_PRIVATE_KEY_HEX:-}" ]]; then
        echo "FAIL: SPOT_ORACLE_PRIVATE_KEY_HEX required for on-chain mode" >&2
        return 1
    fi
    if [[ "${SPOT_ORACLE_PRIVATE_KEY_HEX}" == "$SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX" ]]; then
        echo "FAIL: refusing default SPOT_ORACLE_PRIVATE_KEY_HEX for on-chain mode" >&2
        echo "Export a funded oracle key that owns SpotOracleAdminCap, then re-run --run-all-onchain" >&2
        return 1
    fi
    if [[ ${#SPOT_ORACLE_PRIVATE_KEY_HEX} -lt 64 ]]; then
        echo "FAIL: SPOT_ORACLE_PRIVATE_KEY_HEX looks too short" >&2
        return 1
    fi
    if [[ -z "${SPOT_ORACLE_PLATFORM_OBJECT_ID:-}" || -z "${SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID:-}" ]]; then
        echo "FAIL: PLATFORM_OBJECT_ID / ECOSYSTEM_TREASURY_ID required for on-chain oracle_resolve" >&2
        echo "Run --refresh-session after bootstrap so session maps platform + treasury IDs" >&2
        return 1
    fi
    if [[ -z "${SPOT_ORACLE_REGISTRY_OBJECT_ID:-}" ]]; then
        echo "FAIL: SPOT_REGISTRY_ID / SPOT_ORACLE_REGISTRY_OBJECT_ID required for claim/market PTBs" >&2
        echo "Run --refresh-session after bootstrap so session maps SpotClaimRegistry" >&2
        return 1
    fi
}

refresh_spot_oracle_session_from_graphql() {
    command -v curl >/dev/null 2>&1 || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }

    social_refresh_session_from_graphql || return 1
    load_spot_oracle_session

    local json
    log_step "Refreshing SPoT oracle extras from GraphQL ($GRAPHQL_URL)"
    json="$(graphql_post "$SPOT_ORACLE_GQL_EXTRAS")" || return 1

    SPOT_CONFIG_ID="$(gql_object_address "$json" spotConfig)"
    SPOT_REGISTRY_ID="$(gql_object_address "$json" spotClaimRegistry)"
    SPOT_ORACLE_ADMIN_CAP_ID="$(gql_object_address "$json" spotOracleAdminCap)"

    if [[ -n "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]] && ! object_exists_on_fullnode "$SPOT_ORACLE_ADMIN_CAP_ID"; then
        echo "SPOT_ORACLE_ADMIN_CAP_ID=${SPOT_ORACLE_ADMIN_CAP_ID} not on fullnode; run bootstrap then --refresh-session" >&2
        SPOT_ORACLE_ADMIN_CAP_ID=''
    fi
    if [[ -n "${SPOT_CONFIG_ID:-}" ]] && ! object_exists_on_fullnode "$SPOT_CONFIG_ID"; then
        echo "SPOT_CONFIG_ID=${SPOT_CONFIG_ID} not on fullnode; run bootstrap then --refresh-session" >&2
        SPOT_CONFIG_ID=''
    fi
    if [[ -n "${SPOT_REGISTRY_ID:-}" ]] && ! object_exists_on_fullnode "$SPOT_REGISTRY_ID"; then
        echo "SPOT_REGISTRY_ID=${SPOT_REGISTRY_ID} not on fullnode; run bootstrap then --refresh-session" >&2
        SPOT_REGISTRY_ID=''
    fi

    ORACLE_ADDRESS="${ORACLE_ADDRESS:-$SPOT_DEFAULT_ORACLE_ADDRESS}"
    SPOT_ORACLE_PRIVATE_KEY_HEX="${SPOT_ORACLE_PRIVATE_KEY_HEX:-$SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX}"
    spot_oracle_apply_runtime_defaults
    spot_oracle_map_session_to_oracle_env

    log_session_use "SPOT_CONFIG_ID" "$SPOT_CONFIG_ID"
    log_session_use "SPOT_REGISTRY_ID" "$SPOT_REGISTRY_ID"
    log_session_use "SPOT_ORACLE_ADMIN_CAP_ID" "$SPOT_ORACLE_ADMIN_CAP_ID"
    log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
    log_session_use "ECOSYSTEM_TREASURY_ID" "$ECOSYSTEM_TREASURY_ID"
    log_session_use "SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID" "$SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID"
    log_session_use "SPOT_ORACLE_REGISTRY_OBJECT_ID" "$SPOT_ORACLE_REGISTRY_OBJECT_ID"
    log_session_use "SPOT_ORACLE_ADMIN_CAP_OBJECT_ID" "$SPOT_ORACLE_ADMIN_CAP_OBJECT_ID"

    save_spot_oracle_session
}

spot_oracle_force_offchain_mode() {
    log_step "Off-chain E2E: skip on-chain PTBs (markets activate locally)"
    unset SPOT_ORACLE_PRIVATE_KEY_HEX
    unset SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID
    unset SPOT_ORACLE_ADMIN_CAP_OBJECT_ID
}

require_spot_oracle_session_fields() {
    require_session_fields SPOT_CONFIG_ID SPOT_REGISTRY_ID SPOT_ORACLE_ADMIN_CAP_ID || return 1
    require_hex_ids SPOT_CONFIG_ID SPOT_REGISTRY_ID SPOT_ORACLE_ADMIN_CAP_ID || return 1
}

maybe_auto_refresh_spot_session() {
    load_spot_oracle_session
    if [[ -z "${SPOT_CONFIG_ID:-}" || -z "${SPOT_REGISTRY_ID:-}" || -z "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        log_step "SPoT session incomplete — auto-refreshing from GraphQL"
        refresh_spot_oracle_session_from_graphql || return 1
    fi
}

DISCOVERY_COMPOSE_FILE="${REPO_ROOT}/crates/myso-discovery-service/docker-compose.yml"
DISCOVERY_SVC_PID=''

start_discovery_for_spot_e2e() {
    local listen="${DISCOVERY_LISTEN:-127.0.0.1:8096}"
    local db_url="${DISCOVERY_DATABASE_URL:-postgresql://poc:poc@127.0.0.1:5434/discovery}"
    local sources="${DISCOVERY_SOURCES_CONFIG:-crates/myso-discovery-service/config/discovery/sources.factual.localnet.yaml}"
    log_step "Starting discovery postgres + service for SPoT factual settlement"
    docker compose -f "$DISCOVERY_COMPOSE_FILE" up -d discovery-postgres
    local i
    for i in $(seq 1 60); do
        if docker compose -f "$DISCOVERY_COMPOSE_FILE" exec -T discovery-postgres \
            pg_isready -U poc -d discovery >/dev/null 2>&1; then
            break
        fi
        sleep 1
    done
    DISCOVERY_DATABASE_URL="$db_url" \
    DISCOVERY_SOURCES_CONFIG="$sources" \
    DISCOVERY_LISTEN="$listen" \
    DISCOVERY_METRICS_ADDRESS="${DISCOVERY_METRICS_ADDRESS:-127.0.0.1:9286}" \
    DISCOVERY_ENABLED=true \
    DISCOVERY_EMBED_ENABLED=false \
    DISCOVERY_CLIENT_SECRET="${DISCOVERY_CLIENT_SECRET:-local-discovery-client}" \
    DISCOVERY_ADMIN_SECRET="${DISCOVERY_ADMIN_SECRET:-local-discovery-admin}" \
    RUST_LOG="${RUST_LOG:-info}" \
    cargo run -p myso-discovery-service &
    DISCOVERY_SVC_PID=$!
    for i in $(seq 1 120); do
        if curl -sf "http://${listen}/health" >/dev/null 2>&1; then
            log_step "discovery service healthy at http://${listen}"
            return 0
        fi
        if ! kill -0 "$DISCOVERY_SVC_PID" 2>/dev/null; then
            echo "FAIL: discovery service exited before becoming healthy" >&2
            return 1
        fi
        sleep 1
    done
    echo "FAIL: discovery service did not become healthy" >&2
    return 1
}

stop_discovery_for_spot_e2e() {
    if [[ -n "$DISCOVERY_SVC_PID" ]]; then
        kill "$DISCOVERY_SVC_PID" 2>/dev/null || true
        wait "$DISCOVERY_SVC_PID" 2>/dev/null || true
        DISCOVERY_SVC_PID=''
    fi
}
