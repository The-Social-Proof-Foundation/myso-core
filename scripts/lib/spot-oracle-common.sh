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
    SPOT_ORACLE_SOURCES_CONFIG="${SPOT_ORACLE_SOURCES_CONFIG:-crates/myso-spot-oracle/config/sources.localnet.yaml}"
    SPOT_ORACLE_LISTEN="${SPOT_ORACLE_LISTEN:-127.0.0.1:8097}"
    SPOT_ORACLE_METRICS_ADDRESS="${SPOT_ORACLE_METRICS_ADDRESS:-127.0.0.1:9187}"
    SPOT_ORACLE_SOCIAL_SERVER_URL="${SPOT_ORACLE_SOCIAL_SERVER_URL:-$SOCIAL_SERVER_URL}"
    SPOT_ORACLE_MYSO_RPC="${SPOT_ORACLE_MYSO_RPC:-http://127.0.0.1:9000}"
    SPOT_ORACLE_ENABLED="${SPOT_ORACLE_ENABLED:-true}"
    SPOT_ORACLE_LIVE_SOURCES="${SPOT_ORACLE_LIVE_SOURCES:-true}"
    SPOT_ORACLE_INGEST_MODE="${SPOT_ORACLE_INGEST_MODE:-checkpoint}"
    SPOT_ORACLE_STREAMING_URL="${SPOT_ORACLE_STREAMING_URL:-$SPOT_ORACLE_MYSO_RPC}"
    RUST_LOG="${RUST_LOG:-info}"
    ORACLE_ADDRESS="${ORACLE_ADDRESS:-$SPOT_DEFAULT_ORACLE_ADDRESS}"
    if [[ -n "${SPOT_ORACLE_PRIVATE_KEY_HEX:-}" ]]; then
        SPOT_ORACLE_PRIVATE_KEY_EXPLICIT=1
    else
        SPOT_ORACLE_PRIVATE_KEY_HEX="$SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX"
        SPOT_ORACLE_PRIVATE_KEY_EXPLICIT=0
    fi
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
    if [[ "${SPOT_ORACLE_PRIVATE_KEY_EXPLICIT:-0}" != "1" \
        && "${SPOT_ORACLE_PRIVATE_KEY_HEX}" == "$SPOT_DEFAULT_ORACLE_PRIVATE_KEY_HEX" ]]; then
        echo "FAIL: SPOT_ORACLE_PRIVATE_KEY_HEX not set for on-chain mode" >&2
        echo "Set it in ${SOCIAL_SESSION_SAVE_PATH}, then re-run" >&2
        return 1
    fi
    if [[ ${#SPOT_ORACLE_PRIVATE_KEY_HEX} -lt 64 ]]; then
        echo "FAIL: SPOT_ORACLE_PRIVATE_KEY_HEX looks too short" >&2
        return 1
    fi
    if [[ -z "${SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID:-}" ]]; then
        echo "FAIL: ECOSYSTEM_TREASURY_ID required for on-chain oracle_resolve" >&2
        echo "Run --refresh-session after bootstrap so session maps treasury ID" >&2
        return 1
    fi
    if [[ -z "${SPOT_ORACLE_PLATFORM_OBJECT_ID:-}" ]]; then
        log_step "PLATFORM_OBJECT_ID unset — post/walkthrough will create or join a platform before resolve"
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

    if ! graphql_is_reachable; then
        echo "GraphQL unreachable at ${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}" >&2
        graphql_refresh_hint
        return 1
    fi

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

SPOT_DEFAULT_CLAIM_TEXT='Will BTC trade above $1 in 3 minutes?'
SPOT_DEFAULT_BET_AMOUNT_MIST='100000000'

spot_prompt_with_default() {
    local label="$1" default="$2" entered=''
    if [[ "${ASSUME_YES:-0}" == "1" ]]; then
        printf '%s' "$default"
        return 0
    fi
    read -r -p "${label} [${default}]: " entered
    printf '%s' "${entered:-$default}"
}

spot_prompt_claim_text() {
    local default="${SPOT_CLAIM_TEXT:-$SPOT_DEFAULT_CLAIM_TEXT}"
    if [[ "${ASSUME_YES:-0}" == "1" && -n "${SPOT_CLAIM_TEXT:-}" ]]; then
        log_step "Using SPOT_CLAIM_TEXT=${SPOT_CLAIM_TEXT}"
        return 0
    fi
    echo "Tip: include when the claim should be evaluated (e.g. 'by the end of tomorrow' or 'in 3 minutes')." >&2
    SPOT_CLAIM_TEXT="$(spot_prompt_with_default "Enter your prediction claim" "$default")"
    export SPOT_CLAIM_TEXT
    log_step "Claim: ${SPOT_CLAIM_TEXT}"
}

# Full walkthrough (#1): ask for a fresh prediction before any infra/session work.
# Always prompts on a TTY — ASSUME_YES does not skip this step.
spot_prompt_walkthrough_claim() {
    SOCIAL_RUN_ID="$(date +%s)"
    export SOCIAL_RUN_ID

    echo ""
    echo "=== Your future prediction ==="
    echo "This claim drives the full walkthrough: post → oracle review → on-chain market → bet → resolve."
    echo "Tip: include a deadline (e.g. 'in 3 minutes' for a quick run, or 'by the end of tomorrow')." >&2

    if [[ -t 0 ]]; then
        local entered=''
        read -r -p "Enter your future prediction: " entered
        entered="${entered#"${entered%%[![:space:]]*}"}"
        entered="${entered%"${entered##*[![:space:]]}"}"
        [[ -n "$entered" ]] || {
            echo "FAIL: prediction claim cannot be empty" >&2
            return 1
        }
        SPOT_CLAIM_TEXT="$entered"
    elif [[ "${ASSUME_YES:-0}" == "1" ]]; then
        # Non-interactive: unique threshold per run so on-chain semantic hash does not collide.
        SPOT_CLAIM_TEXT="Will BTC trade above \$${SOCIAL_RUN_ID} in 3 minutes?"
        log_step "Auto-generated prediction (non-interactive): ${SPOT_CLAIM_TEXT}"
    else
        echo "FAIL: walkthrough requires an interactive TTY, or ASSUME_YES=1 for auto-generated claim" >&2
        return 1
    fi

    POST_ID=''
    SPOT_CLAIM_ID=''
    SPOT_MARKET_ID=''
    BET_TX_DIGEST=''
    PAYOUT_TX_DIGEST=''
    export SPOT_CLAIM_TEXT POST_ID SPOT_CLAIM_ID SPOT_MARKET_ID BET_TX_DIGEST PAYOUT_TX_DIGEST
    log_step "Prediction claim: ${SPOT_CLAIM_TEXT}"
}

spot_prompt_bet_amount_mist() {
    local default="${BET_AMOUNT_MIST:-$SPOT_DEFAULT_BET_AMOUNT_MIST}"
    BET_AMOUNT_MIST="$(spot_prompt_with_default "Bet amount in MIST" "$default")"
    export BET_AMOUNT_MIST
    log_step "Bet amount: ${BET_AMOUNT_MIST} MIST"
}

spot_prompt_bet_side() {
    local options_json="$1"
    local count i label choice
    count="$(echo "$options_json" | jq 'length' 2>/dev/null || echo 0)"
    [[ "$count" -gt 0 ]] || {
        echo "No betting options available" >&2
        return 1
    }
    if [[ "${ASSUME_YES:-0}" == "1" ]]; then
        BET_OPTION_ID="${BET_OPTION_ID:-0}"
        export BET_OPTION_ID
        log_step "Bet side (ASSUME_YES): option_id=${BET_OPTION_ID}"
        return 0
    fi
    echo "Betting options:" >&2
    for ((i = 0; i < count; i++)); do
        label="$(echo "$options_json" | jq -r ".[$i]")"
        echo "  ${i}) ${label}" >&2
    done
    read -r -p "Choose side (0-$((count - 1))): " choice
    choice="${choice:-0}"
    if [[ "$choice" -lt 0 || "$choice" -ge "$count" ]]; then
        echo "Invalid option_id: ${choice}" >&2
        return 1
    fi
    BET_OPTION_ID="$choice"
    export BET_OPTION_ID
    log_step "Bet side: option_id=${BET_OPTION_ID} ($(echo "$options_json" | jq -r ".[${BET_OPTION_ID}]"))"
}

spot_http_route() {
    local post_id="$1"
    post_id="$(normalize_hex_id "$post_id")" || return 1
    local social="${SOCIAL_SERVER_URL:-http://127.0.0.1:9126}"
    curl -sf --max-time 15 "${social}/spot/route/${post_id}" 2>/dev/null
}

spot_resolve_market_id() {
    local post_id="$1"
    local market_id="${2:-}"
    if [[ -n "$market_id" ]]; then
        normalize_hex_id "$market_id"
        return 0
    fi
    local route_json
    route_json="$(spot_http_route "$post_id")" || true
    if [[ -n "$route_json" ]]; then
        market_id="$(echo "$route_json" | jq -r '.target_market_id // empty')"
        if [[ -n "$market_id" && "$market_id" != "null" ]]; then
            normalize_hex_id "$market_id"
            return 0
        fi
    fi
    echo "Could not resolve SpotMarket id for post ${post_id}" >&2
    return 1
}

spot_place_bet_for_post() {
    local bettor="$1" post_id="$2" market_id="$3" option_id="$4" amount_mist="$5"
    local pay_coin gas_coin out digest
    local ref_cfg ref_reg ref_mkt ref_post ref_clk

    bettor="$(normalize_hex_id "$bettor")" || return 1
    post_id="$(normalize_hex_id "$post_id")" || return 1
    market_id="$(normalize_hex_id "$market_id")" || return 1
    require_hex_ids SPOT_CONFIG_ID SPOT_REGISTRY_ID CLOCK_ID || return 1

    ensure_wallet_funded "$bettor" "$((amount_mist + SOCIAL_DEFAULT_GAS_BUDGET))" || return 1
    switch_wallet "$bettor" || return 1
    read -r pay_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$bettor" "$amount_mist")" || {
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID="$gas_coin"

    ref_cfg="$(ptb_shared_ref "$SPOT_CONFIG_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_reg="$(ptb_shared_ref "$SPOT_REGISTRY_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_mkt="$(ptb_shared_ref "$market_id")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_post="$(ptb_shared_ref "$post_id")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }

    log_step "place_spot_bet_for_post bettor=${bettor} option=${option_id} amount=${amount_mist}"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$bettor" \
        --split-coins "@${pay_coin}" "[${amount_mist}]" \
        --assign bet_coin \
        --move-call "${PKG_SOCIAL}::social_proof_of_truth::place_spot_bet_for_post" \
        "$ref_cfg" "$ref_reg" "$ref_mkt" "$ref_post" \
        bet_coin.0 "$option_id" "$amount_mist" none \
        "$ref_clk")" || {
        PTB_GAS_COIN_ID=''
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID=''
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    BET_TX_DIGEST="$digest"
    log_session_use "BET_TX_DIGEST" "$BET_TX_DIGEST"
    printf '%s' "$digest"
}

spot_claim_payout() {
    local bettor="$1" post_id="$2" market_id="$3"
    local out digest
    local ref_cfg ref_mkt ref_post ref_clk

    bettor="$(normalize_hex_id "$bettor")" || return 1
    post_id="$(normalize_hex_id "$post_id")" || return 1
    market_id="$(normalize_hex_id "$market_id")" || return 1
    require_hex_ids SPOT_CONFIG_ID CLOCK_ID || return 1

    ensure_wallet_funded "$bettor" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    switch_wallet "$bettor" || return 1

    ref_cfg="$(ptb_shared_ref "$SPOT_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_mkt="$(ptb_shared_ref "$market_id")" || { restore_wallet; return 1; }
    ref_post="$(ptb_shared_ref "$post_id")" || { restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_wallet; return 1; }

    log_step "claim_payout bettor=${bettor} post=${post_id}"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$bettor" \
        --move-call "${PKG_SOCIAL}::social_proof_of_truth::claim_payout" \
        "$ref_cfg" "$ref_mkt" "$ref_post" "$ref_clk")" || {
        restore_wallet
        return 1
    }
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    PAYOUT_TX_DIGEST="$digest"
    log_session_use "PAYOUT_TX_DIGEST" "$PAYOUT_TX_DIGEST"
    printf '%s' "$digest"
}

# --- Optional insurance E2E helpers (ENABLE_INSURANCE_E2E=1) ---

readonly SPOT_INSURANCE_GQL_EXTRAS='query SpotInsuranceExtras {
  insuranceConfig: objects(filter: { type: "0x50c1::insurance::InsuranceConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  insuranceRouterConfig: objects(filter: { type: "0x50c1::insurance::InsuranceRouterConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  insuranceBackstop: objects(filter: { type: "0x50c1::insurance::InsuranceBackstopPool", ownerKind: SHARED }, first: 1) { nodes { address } }
  insuranceAdminCap: objects(filter: { type: "0x50c1::insurance::InsuranceAdminCap" }, last: 1) { nodes { address } }
}'

spot_insurance_refresh_ids() {
    local json
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }
    log_step "Refreshing insurance object IDs from GraphQL"
    json="$(graphql_post "$SPOT_INSURANCE_GQL_EXTRAS")" || return 1
    INSURANCE_CONFIG_ID="$(gql_object_address "$json" insuranceConfig)"
    INSURANCE_ROUTER_CONFIG_ID="$(gql_object_address "$json" insuranceRouterConfig)"
    INSURANCE_BACKSTOP_ID="$(gql_object_address "$json" insuranceBackstop)"
    INSURANCE_ADMIN_CAP_ID="$(gql_object_address "$json" insuranceAdminCap)"
    log_session_use "INSURANCE_CONFIG_ID" "$INSURANCE_CONFIG_ID"
    log_session_use "INSURANCE_ROUTER_CONFIG_ID" "$INSURANCE_ROUTER_CONFIG_ID"
    log_session_use "INSURANCE_BACKSTOP_ID" "$INSURANCE_BACKSTOP_ID"
    log_session_use "INSURANCE_ADMIN_CAP_ID" "$INSURANCE_ADMIN_CAP_ID"
    require_hex_ids INSURANCE_CONFIG_ID INSURANCE_ROUTER_CONFIG_ID INSURANCE_BACKSTOP_ID INSURANCE_ADMIN_CAP_ID || return 1
}

spot_insurance_enable() {
    local admin="${1:-${ORACLE_ADDRESS:-}}"
    local out digest
    local ref_cfg ref_clk
    admin="$(normalize_hex_id "$admin")" || return 1
    require_hex_ids INSURANCE_CONFIG_ID INSURANCE_ADMIN_CAP_ID CLOCK_ID || return 1
    ensure_wallet_funded "$admin" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    switch_wallet "$admin" || return 1
    ref_cfg="$(ptb_shared_ref "$INSURANCE_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_wallet; return 1; }
    log_step "set_insurance_enabled=true"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$admin" \
        --move-call "${PKG_SOCIAL}::insurance::set_insurance_enabled" \
        "@${INSURANCE_ADMIN_CAP_ID}" "$ref_cfg" true "$ref_clk")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    log_session_use "INSURANCE_ENABLE_TX" "$digest"
}

spot_insurance_create_vault_and_fund() {
    local underwriter="${1:-${ORACLE_ADDRESS:-}}"
    local deposit_mist="${2:-${INSURANCE_VAULT_DEPOSIT_MIST:-5000000000}}"
    local out digest pay_coin gas_coin
    local ref_cfg ref_vault

    underwriter="$(normalize_hex_id "$underwriter")" || return 1
    require_hex_ids INSURANCE_CONFIG_ID || return 1
    ensure_wallet_funded "$underwriter" "$((deposit_mist + SOCIAL_DEFAULT_GAS_BUDGET))" || return 1
    switch_wallet "$underwriter" || return 1

    if [[ -z "${INSURANCE_VAULT_ID:-}" ]]; then
        log_step "create_vault underwriter=${underwriter}"
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$underwriter" \
            --move-call "${PKG_SOCIAL}::insurance::create_vault" \
            25 5000 0 0)" || {
            restore_wallet
            return 1
        }
        digest="$(extract_tx_digest "$out")"
        INSURANCE_VAULT_ID="$(extract_created_object_by_type "$digest" "insurance::UnderwriterVault")"
        [[ -n "$INSURANCE_VAULT_ID" ]] || INSURANCE_VAULT_ID="$(extract_created_object_by_type "$digest" "UnderwriterVault")"
        [[ -n "$INSURANCE_VAULT_ID" ]] || {
            echo "FAIL: could not extract UnderwriterVault id from $digest" >&2
            restore_wallet
            return 1
        }
        INSURANCE_VAULT_ID="$(normalize_hex_id "$INSURANCE_VAULT_ID")"
        log_session_use "INSURANCE_VAULT_ID" "$INSURANCE_VAULT_ID"
    fi

    read -r pay_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$underwriter" "$deposit_mist")" || {
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID="$gas_coin"
    ref_cfg="$(ptb_shared_ref "$INSURANCE_CONFIG_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_vault="$(ptb_shared_ref "$INSURANCE_VAULT_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    log_step "deposit_capital vault=${INSURANCE_VAULT_ID} amount=${deposit_mist}"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$underwriter" \
        --split-coins "@${pay_coin}" "[${deposit_mist}]" \
        --assign deposit_coin \
        --move-call "${PKG_SOCIAL}::insurance::deposit_capital" \
        "$ref_cfg" "$ref_vault" deposit_coin.0)" || {
        PTB_GAS_COIN_ID=''
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID=''
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    log_session_use "INSURANCE_DEPOSIT_TX" "$digest"
}

spot_insurance_buy_coverage() {
    local insured="$1" market_id="$2" option_id="$3"
    local coverage_amount="${4:-${INSURANCE_COVERAGE_AMOUNT_MIST:-${BET_AMOUNT_MIST:-100000000}}}"
    local coverage_bps="${5:-${INSURANCE_COVERAGE_BPS:-8000}}"
    local duration_ms="${6:-${INSURANCE_DURATION_MS:-259200000}}"
    local premium_mist="${7:-${INSURANCE_PREMIUM_MIST:-500000000}}"
    local out digest pay_coin gas_coin
    local ref_cfg ref_router ref_backstop ref_spot ref_vault ref_mkt ref_clk

    insured="$(normalize_hex_id "$insured")" || return 1
    market_id="$(normalize_hex_id "$market_id")" || return 1
    require_hex_ids INSURANCE_CONFIG_ID INSURANCE_ROUTER_CONFIG_ID INSURANCE_BACKSTOP_ID \
        SPOT_CONFIG_ID INSURANCE_VAULT_ID CLOCK_ID || return 1

    ensure_wallet_funded "$insured" "$((premium_mist + SOCIAL_DEFAULT_GAS_BUDGET))" || return 1
    switch_wallet "$insured" || return 1
    read -r pay_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$insured" "$premium_mist")" || {
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID="$gas_coin"
    ref_cfg="$(ptb_shared_ref "$INSURANCE_CONFIG_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_router="$(ptb_shared_ref "$INSURANCE_ROUTER_CONFIG_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_backstop="$(ptb_shared_ref "$INSURANCE_BACKSTOP_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_spot="$(ptb_shared_ref "$SPOT_CONFIG_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_vault="$(ptb_shared_ref "$INSURANCE_VAULT_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_mkt="$(ptb_shared_ref "$market_id")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }

    log_step "buy_coverage insured=${insured} option=${option_id} coverage=${coverage_amount}"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$insured" \
        --split-coins "@${pay_coin}" "[${premium_mist}]" \
        --assign prem_coin \
        --move-call "${PKG_SOCIAL}::insurance::buy_coverage" \
        "$ref_cfg" "$ref_router" "$ref_backstop" "$ref_spot" "$ref_vault" "$ref_mkt" \
        "$option_id" "$coverage_amount" "$coverage_bps" "$duration_ms" \
        prem_coin.0 "$ref_clk")" || {
        PTB_GAS_COIN_ID=''
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID=''
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    INSURANCE_BUY_TX="$digest"
    INSURANCE_POLICY_ID="$(extract_created_object_by_type "$digest" "insurance::CoveragePolicy")"
    [[ -n "$INSURANCE_POLICY_ID" ]] || INSURANCE_POLICY_ID="$(extract_created_object_by_type "$digest" "CoveragePolicy")"
    [[ -n "$INSURANCE_POLICY_ID" ]] || {
        echo "FAIL: could not extract CoveragePolicy from $digest" >&2
        return 1
    }
    INSURANCE_POLICY_ID="$(normalize_hex_id "$INSURANCE_POLICY_ID")"
    log_session_use "INSURANCE_POLICY_ID" "$INSURANCE_POLICY_ID"
    log_session_use "INSURANCE_BUY_TX" "$INSURANCE_BUY_TX"
}

spot_insurance_claim() {
    local insured="$1" market_id="$2" policy_id="${3:-${INSURANCE_POLICY_ID:-}}"
    local out digest
    local ref_cfg ref_spot ref_vault ref_mkt ref_pol ref_clk

    insured="$(normalize_hex_id "$insured")" || return 1
    market_id="$(normalize_hex_id "$market_id")" || return 1
    policy_id="$(normalize_hex_id "$policy_id")" || return 1
    require_hex_ids INSURANCE_CONFIG_ID SPOT_CONFIG_ID INSURANCE_VAULT_ID CLOCK_ID || return 1

    ensure_wallet_funded "$insured" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    switch_wallet "$insured" || return 1
    ref_cfg="$(ptb_shared_ref "$INSURANCE_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_spot="$(ptb_shared_ref "$SPOT_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_vault="$(ptb_shared_ref "$INSURANCE_VAULT_ID")" || { restore_wallet; return 1; }
    ref_mkt="$(ptb_shared_ref "$market_id")" || { restore_wallet; return 1; }
    ref_pol="$(ptb_shared_ref "$policy_id")" || { restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_wallet; return 1; }

    log_step "insurance::claim policy=${policy_id}"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$insured" \
        --move-call "${PKG_SOCIAL}::insurance::claim" \
        "$ref_cfg" "$ref_spot" "$ref_vault" "$ref_mkt" "$ref_pol" "$ref_clk")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    INSURANCE_CLAIM_TX="$digest"
    log_session_use "INSURANCE_CLAIM_TX" "$INSURANCE_CLAIM_TX"
}

spot_insurance_assert_duplicate_claim_fails() {
    local insured="$1" market_id="$2" policy_id="${3:-${INSURANCE_POLICY_ID:-}}"
    local out rc=0
    local ref_cfg ref_spot ref_vault ref_mkt ref_pol ref_clk

    insured="$(normalize_hex_id "$insured")" || return 1
    market_id="$(normalize_hex_id "$market_id")" || return 1
    policy_id="$(normalize_hex_id "$policy_id")" || return 1
    ensure_wallet_funded "$insured" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    switch_wallet "$insured" || return 1
    ref_cfg="$(ptb_shared_ref "$INSURANCE_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_spot="$(ptb_shared_ref "$SPOT_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_vault="$(ptb_shared_ref "$INSURANCE_VAULT_ID")" || { restore_wallet; return 1; }
    ref_mkt="$(ptb_shared_ref "$market_id")" || { restore_wallet; return 1; }
    ref_pol="$(ptb_shared_ref "$policy_id")" || { restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_wallet; return 1; }

    log_step "Asserting duplicate insurance::claim fails"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$insured" \
        --move-call "${PKG_SOCIAL}::insurance::claim" \
        "$ref_cfg" "$ref_spot" "$ref_vault" "$ref_mkt" "$ref_pol" "$ref_clk")" || rc=$?
    restore_wallet
    if [[ "$rc" -eq 0 ]]; then
        echo "FAIL: duplicate insurance::claim succeeded (expected abort)" >&2
        return 1
    fi
    log_step "Duplicate claim aborted as expected (rc=$rc)"
}

spot_insurance_e2e_prepare() {
    spot_insurance_refresh_ids || return 1
    spot_insurance_enable "${ORACLE_ADDRESS}" || return 1
    spot_insurance_create_vault_and_fund "${ORACLE_ADDRESS}" || return 1
}

