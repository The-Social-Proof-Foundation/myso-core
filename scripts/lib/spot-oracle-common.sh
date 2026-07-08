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
  spotOracleAdminCap: objects(filter: { type: "0x50c1::social_proof_of_truth::SpotOracleAdminCap" }, last: 1) { nodes { address } }
}'

SPOT_CONFIG_ID=''
SPOT_ORACLE_ADMIN_CAP_ID=''
ORACLE_ADDRESS=''

SPOT_ORACLE_SESSION_KEYS=(
    GRAPHQL_URL SOCIAL_SERVER_URL PKG_SOCIAL CLOCK_ID
    ECOSYSTEM_TREASURY_ID PLATFORM_OBJECT_ID PLATFORM_CONFIG_ID POST_CONFIG_ID
    SPOT_CONFIG_ID SPOT_ORACLE_ADMIN_CAP_ID ORACLE_ADDRESS
    SPOT_ORACLE_DATABASE_URL SPOT_ORACLE_SOURCES_CONFIG SPOT_ORACLE_LISTEN
    SPOT_ORACLE_METRICS_ADDRESS SPOT_ORACLE_SOCIAL_SERVER_URL SPOT_ORACLE_MYSO_RPC
    SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID SPOT_ORACLE_ADMIN_CAP_OBJECT_ID SPOT_ORACLE_PRIVATE_KEY_HEX
    SPOT_ORACLE_ENABLED SPOT_ORACLE_LIVE_SOURCES RUST_LOG
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
    RUST_LOG="${RUST_LOG:-info}"
    ORACLE_ADDRESS="${ORACLE_ADDRESS:-$SPOT_DEFAULT_ORACLE_ADDRESS}"
    SPOT_ORACLE_PRIVATE_KEY_HEX="${SPOT_ORACLE_PRIVATE_KEY_HEX:-$SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX}"
    spot_oracle_normalize_bool_env SPOT_ORACLE_ENABLED
    spot_oracle_normalize_bool_env SPOT_ORACLE_LIVE_SOURCES
}

spot_oracle_map_session_to_oracle_env() {
    if [[ -n "${SPOT_CONFIG_ID:-}" ]]; then
        SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID="$SPOT_CONFIG_ID"
    fi
    if [[ -n "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        SPOT_ORACLE_ADMIN_CAP_OBJECT_ID="$SPOT_ORACLE_ADMIN_CAP_ID"
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
    export SPOT_CONFIG_ID SPOT_ORACLE_ADMIN_CAP_ID ORACLE_ADDRESS
    export SPOT_ORACLE_DATABASE_URL SPOT_ORACLE_SOURCES_CONFIG SPOT_ORACLE_LISTEN
    export SPOT_ORACLE_METRICS_ADDRESS SPOT_ORACLE_SOCIAL_SERVER_URL SPOT_ORACLE_MYSO_RPC
    export SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID SPOT_ORACLE_ADMIN_CAP_OBJECT_ID SPOT_ORACLE_PRIVATE_KEY_HEX
    export SPOT_ORACLE_ENABLED SPOT_ORACLE_LIVE_SOURCES RUST_LOG
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
    SPOT_ORACLE_ADMIN_CAP_ID="$(gql_object_address "$json" spotOracleAdminCap)"

    if [[ -n "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]] && ! object_exists_on_fullnode "$SPOT_ORACLE_ADMIN_CAP_ID"; then
        echo "SPOT_ORACLE_ADMIN_CAP_ID=${SPOT_ORACLE_ADMIN_CAP_ID} not on fullnode; run bootstrap then --refresh-session" >&2
        SPOT_ORACLE_ADMIN_CAP_ID=''
    fi
    if [[ -n "${SPOT_CONFIG_ID:-}" ]] && ! object_exists_on_fullnode "$SPOT_CONFIG_ID"; then
        echo "SPOT_CONFIG_ID=${SPOT_CONFIG_ID} not on fullnode; run bootstrap then --refresh-session" >&2
        SPOT_CONFIG_ID=''
    fi

    ORACLE_ADDRESS="${ORACLE_ADDRESS:-$SPOT_DEFAULT_ORACLE_ADDRESS}"
    SPOT_ORACLE_PRIVATE_KEY_HEX="${SPOT_ORACLE_PRIVATE_KEY_HEX:-$SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX}"
    spot_oracle_apply_runtime_defaults
    spot_oracle_map_session_to_oracle_env

    log_session_use "SPOT_CONFIG_ID" "$SPOT_CONFIG_ID"
    log_session_use "SPOT_ORACLE_ADMIN_CAP_ID" "$SPOT_ORACLE_ADMIN_CAP_ID"
    log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
    log_session_use "ECOSYSTEM_TREASURY_ID" "$ECOSYSTEM_TREASURY_ID"
    log_session_use "SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID" "$SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID"
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
    require_session_fields SPOT_CONFIG_ID SPOT_ORACLE_ADMIN_CAP_ID || return 1
    require_hex_ids SPOT_CONFIG_ID SPOT_ORACLE_ADMIN_CAP_ID || return 1
}

maybe_auto_refresh_spot_session() {
    load_spot_oracle_session
    if [[ -z "${SPOT_CONFIG_ID:-}" || -z "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        log_step "SPoT session incomplete — auto-refreshing from GraphQL"
        refresh_spot_oracle_session_from_graphql || return 1
    fi
}
