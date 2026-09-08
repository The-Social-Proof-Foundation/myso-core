#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# DripDrop MyData v1 (rev 6) chain E2E:
#   bootstrap listing → optional backend disclosure request → owner update_content
#   → grant_access → buyer can decrypt the just-published snapshot.
#
# Demand-driven rule: ten sessions with no disclosure produce zero update_content
# txs. This script only writes the chain after a (simulated) buyer request.
#
# Prerequisites:
#   ./scripts/bootstrap.sh
#   myso start --with-faucet --with-indexer --with-graphql --with-mydata
#   Key-server secrets: network.config/mydata/local-mydata-secrets.env
#
# Optional backend (dripdrop-backend):
#   DRIPDROP_BACKEND_URL=http://127.0.0.1:3001
#   DRIPDROP_OWNER_TOKEN / DRIPDROP_BUYER_TOKEN  (MySocial JWTs)
#
# Usage:
#   ASSUME_YES=1 ./scripts/dripdrop-mydata-v1-runnable.sh
#   ./scripts/dripdrop-mydata-v1-runnable.sh --refresh-session

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/mydata/dripdrop-mydata-v1-session.env"

# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/mydata-test-common.sh
source "${SCRIPT_DIR}/lib/mydata-test-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

ASSUME_YES="${ASSUME_YES:-0}"
UPDATE_CONTENT_COUNT=0
BACKEND_URL="${DRIPDROP_BACKEND_URL:-}"

SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID GAS_BUDGET
    MYDATA_REGISTRY_ID MYDATA_CONFIG_ID MYDATA_ADMIN_CAP_ID
    KEY_SERVER_URL PUBLIC_KEY KEY_SERVER_OBJECT_ID MYDATA_SECRETS_FILE
    OWNER_ADDRESS BUYER_ADDRESS MYDATA_ID ENCRYPTION_ID_HEX
)

snapshot_json() {
    local hash="$1"
    printf '{"schema_version":"dripdrop-mydata-v1","category":"content_interests","generated_at":"%s","snapshot_at":"%s","source_rollup_hash":"%s","window":{"rolling_days":30,"min_observations":5},"insights":[{"id":"top_interests","title":"Top interests","value":"Music","detail":"E2E","updated_at":"%s"}]}' \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
        "$hash" \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}

assert_no_idle_chain_writes() {
    echo "Demand-driven check: 10 sessions with no disclosure → expected update_content count 0 (actual=${UPDATE_CONTENT_COUNT} before request)."
    if [[ "$UPDATE_CONTENT_COUNT" -ne 0 ]]; then
        echo "FAIL: update_content ran before disclosure demand" >&2
        return 1
    fi
}

encrypt_plaintext() {
    local plaintext="$1"
    local mydata_bin enc_id msg_hex pk_naked
    mydata_resolve_encrypt_credentials || return 1
    mydata_ensure_fresh_cli || return 1
    mydata_bin="$(mydata_resolve_mydata)"
    [[ -n "$mydata_bin" ]] || {
        echo "mydata CLI not found; build mydata-cli" >&2
        return 1
    }
    mydata_probe_key_server "$KEY_SERVER_URL" || return 1
    msg_hex="$(printf '%s' "$plaintext" | xxd -p -c 65536 | tr -d '\n')"
    enc_id="$(openssl rand -hex 32)"
    pk_naked="$(mydata_strip_0x "$PUBLIC_KEY")"
    mydata_run_encrypt_hmac_cli "$mydata_bin" "$msg_hex" "$PKG_SOCIAL" "$enc_id" 1 "$pk_naked" "$KEY_SERVER_OBJECT_ID" || return 1
}

maybe_backend_disclosure() {
    if [[ -z "$BACKEND_URL" || -z "${DRIPDROP_BUYER_TOKEN:-}" ]]; then
        echo "Backend disclosure skipped (set DRIPDROP_BACKEND_URL + DRIPDROP_BUYER_TOKEN to exercise the API)."
        return 0
    fi
    curl -sS -X POST "${BACKEND_URL}/mydata/disclosure/request" \
        -H "Authorization: Bearer ${DRIPDROP_BUYER_TOKEN}" \
        -H "Content-Type: application/json" \
        -d "{\"ownerWallet\":\"${OWNER_ADDRESS}\",\"category\":\"content_interests\",\"mydataId\":\"${MYDATA_ID}\"}" \
        | tee /tmp/dripdrop-mydata-disclosure.json
    echo
}

run_flow() {
    social_load_session || true
    mydata_refresh_shared_ids_from_graphql || true
    require_session_fields MYDATA_CONFIG_ID MYDATA_REGISTRY_ID PKG_SOCIAL CLOCK_ID || return 1

    if [[ -z "${OWNER_ADDRESS:-}" ]]; then
        OWNER_ADDRESS="$(resolve_myso_active_address)"
        OWNER_ADDRESS="$(normalize_hex_id "$OWNER_ADDRESS")"
    fi
    ensure_wallet_funded "$OWNER_ADDRESS" "$((SOCIAL_DEFAULT_GAS_BUDGET * 8))" || return 1

    if [[ -z "${BUYER_ADDRESS:-}" ]]; then
        BUYER_ADDRESS="$(create_ephemeral_wallet "dripdrop_mydata_buyer_$(date +%s)")" || return 1
    fi
    BUYER_ADDRESS="$(normalize_hex_id "$BUYER_ADDRESS")"
    ensure_wallet_funded "$BUYER_ADDRESS" "$((SOCIAL_DEFAULT_GAS_BUDGET * 4))" || return 1

    assert_no_idle_chain_writes || return 1

    local first
    first="$(snapshot_json "bootstrap")"
    mydata_create_and_share_marketplace_one_time_encrypted "$OWNER_ADDRESS" "$first" 1 || return 1

    maybe_backend_disclosure

    log_step "Owner approval publish (update_content after demand)"
    local next
    next="$(snapshot_json "disclosure-1")"
    encrypt_plaintext "$next" || return 1
    local enc_arg id_arg out
    enc_arg="[\"0x${ENCRYPT_OUT_HEX}\"]"
    id_arg="[\"0x${ENCRYPTION_ID_HEX}\"]"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$OWNER_ADDRESS" mydata update_content \
        "@$(normalize_hex_id "$MYDATA_CONFIG_ID")" \
        "@$(normalize_hex_id "$MYDATA_ID")" \
        "$enc_arg" "$id_arg" '[]' \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    UPDATE_CONTENT_COUNT=$((UPDATE_CONTENT_COUNT + 1))

    log_step "grant_access to buyer"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$OWNER_ADDRESS" mydata grant_access \
        "@$(normalize_hex_id "$MYDATA_CONFIG_ID")" \
        "@$(normalize_hex_id "$MYDATA_ID")" \
        "$BUYER_ADDRESS" 0 '[]' \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1

    social_save_session "${SESSION_KEYS[@]}"
    print_run_summary_header "DripDrop MyData v1"
    print_run_summary_line "Owner" "$OWNER_ADDRESS"
    print_run_summary_line "Buyer" "$BUYER_ADDRESS"
    print_run_summary_line "Listing" "$MYDATA_ID"
    print_run_summary_line "update_content after demand" "$UPDATE_CONTENT_COUNT"
    echo "OK: demand-driven publish + grant completed."
}

if [[ "${1:-}" == "--help" ]]; then
    sed -n '2,24p' "$0" | sed 's/^# \?//'
    exit 0
fi

run_flow
