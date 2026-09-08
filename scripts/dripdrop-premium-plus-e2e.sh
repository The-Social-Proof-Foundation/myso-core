#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# DripDrop Premium+ chain E2E:
#   load Premium+ MYUSD plans → fund subscriber → subscribe_to_profile<MYUSD>
#   (non-platform) → wait for GraphQL subscriptionAccess → optional backend
#   GET /premium/status → cancel → verify access false.
#
# Prerequisites:
#   ./scripts/bootstrap.sh
#   ./scripts/dripdrop-myusd-subscription-plans.sh
#   myso start --with-faucet --with-indexer --with-social-indexer --with-graphql
#   MYUSD published (orderbook/bridge session: MYUSD_COIN_TYPE, MYUSD_TREASURY_CAP_ID)
#
# Optional backend (dripdrop-backend):
#   DRIPDROP_BACKEND_URL=http://127.0.0.1:3001
#   DRIPDROP_SUBSCRIBER_TOKEN  (MySocial JWT for GET /premium/status)
#
# Usage:
#   ASSUME_YES=1 ./scripts/dripdrop-premium-plus-e2e.sh
#   ./scripts/dripdrop-premium-plus-e2e.sh --refresh-session

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/dripdrop/dripdrop-premium-plus-e2e-session.env"
PLANS_SESSION="$REPO_ROOT/network.config/dripdrop/dripdrop-subscription-plans-session.env"
ORDERBOOK_SESSION="$REPO_ROOT/network.config/orderbook/orderbook-session.env"
BACKEND_URL="${DRIPDROP_BACKEND_URL:-}"

# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/subscription-test-common.sh
source "${SCRIPT_DIR}/lib/subscription-test-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

readonly MONTHLY_PRICE='3990000'

ASSUME_YES="${ASSUME_YES:-0}"
DO_REFRESH=0
SUBSCRIBER_ADDRESS=''
SUBSCRIPTION_ID=''
PLAN_ID=''

SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID GAS_BUDGET
    BLOCK_LIST_REGISTRY_ID SUBSCRIPTION_CONFIG_ID ECOSYSTEM_TREASURY_ID
    SERVICE_ID DRIPDROP_PREMIUM_SERVICE_ID MONTHLY_PLAN_ID ANNUAL_PLAN_ID
    MYUSD_COIN_TYPE MYUSD_TREASURY_CAP_ID
    SUBSCRIBER_ADDRESS SUBSCRIPTION_ID PLAN_ID
)

usage() {
    sed -n '2,23p' "$0" | sed 's/^# \?//'
}

import_plans_session() {
    [[ -f "$PLANS_SESSION" ]] || {
        echo "Plans session missing: $PLANS_SESSION (run dripdrop-myusd-subscription-plans.sh)" >&2
        return 1
    }
    # shellcheck disable=SC1090
    source "$PLANS_SESSION"
    if [[ -z "${DRIPDROP_PREMIUM_SERVICE_ID:-}" && -n "${SERVICE_ID:-}" ]]; then
        DRIPDROP_PREMIUM_SERVICE_ID="$SERVICE_ID"
    fi
    SERVICE_ID="${DRIPDROP_PREMIUM_SERVICE_ID:-${SERVICE_ID:-}}"
}

import_myusd_from_orderbook() {
    if [[ -n "${MYUSD_COIN_TYPE:-}" && -n "${MYUSD_TREASURY_CAP_ID:-}" ]]; then
        return 0
    fi
    [[ -f "$ORDERBOOK_SESSION" ]] || return 0
    # shellcheck disable=SC1090
    source "$ORDERBOOK_SESSION"
}

pick_myusd_coin() {
    local owner="$1" amount="$2" json coin
    owner="$(normalize_hex_id "$owner")" || return 1
    json="$(myso client objects "$owner" --json 2>/dev/null)" || return 1
    coin="$(echo "$json" | jq -r --arg t "$MYUSD_COIN_TYPE" --argjson amt "$amount" '
        .[]?
        | select(
            ((.data.Move.type_.Other? // empty | .address + "::" + .module + "::" + .name) == $t)
            or ((.type? | tostring) | contains($t))
            or ((.data.type? | tostring) | contains($t))
          )
        | select((.data.content.fields.balance // .balance // 0 | tonumber) >= $amt)
        | (.data.objectId // .objectId // .object_id // empty)
        | select(length > 0)
    ' | head -n 1)"
    [[ -n "$coin" ]] || {
        echo "No MYUSD coin >= $amount for $owner" >&2
        return 1
    }
    printf '@%s' "$(normalize_hex_id "$coin")"
}

fund_subscriber_myusd() {
    local amount="$1" out minter
    require_session_fields MYUSD_COIN_TYPE MYUSD_TREASURY_CAP_ID SUBSCRIBER_ADDRESS || return 1
    minter="$(resolve_myso_active_address)" || return 1
    log_step "Minting $amount MYUSD to $SUBSCRIBER_ADDRESS"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$minter" \
        --move-call "0x2::coin::mint_and_transfer<${MYUSD_COIN_TYPE}>" \
        "@$(normalize_hex_id "$MYUSD_TREASURY_CAP_ID")" \
        "$amount" \
        "$(normalize_hex_id "$SUBSCRIBER_ADDRESS")")" || return 1
    assert_tx_success "$out" || {
        echo "mint_and_transfer MYUSD failed" >&2
        return 1
    }
}

maybe_backend_status() {
    local token="${DRIPDROP_SUBSCRIBER_TOKEN:-}"
    if [[ -z "$BACKEND_URL" || -z "$token" ]]; then
        echo "Backend /premium/status skipped (set DRIPDROP_BACKEND_URL + DRIPDROP_SUBSCRIBER_TOKEN)."
        return 0
    fi
    log_step "GET ${BACKEND_URL}/premium/status"
    curl -sS "${BACKEND_URL}/premium/status" \
        -H "Authorization: Bearer ${token}" \
        | tee /tmp/dripdrop-premium-status.json
    echo
}

run_flow() {
    social_load_session || true
    import_plans_session || return 1
    import_myusd_from_orderbook
    if [[ "$DO_REFRESH" == 1 ]]; then
        social_refresh_session_from_graphql || exit 1
        import_plans_session || return 1
    fi
    require_session_fields PKG_SOCIAL CLOCK_ID SUBSCRIPTION_CONFIG_ID SERVICE_ID \
        MONTHLY_PLAN_ID MYUSD_COIN_TYPE BLOCK_LIST_REGISTRY_ID ECOSYSTEM_TREASURY_ID || return 1

    PLAN_ID="$(normalize_hex_id "$MONTHLY_PLAN_ID")"
    SERVICE_ID="$(normalize_hex_id "$SERVICE_ID")"

    if [[ -z "${SUBSCRIBER_ADDRESS:-}" ]]; then
        SUBSCRIBER_ADDRESS="$(create_ephemeral_wallet "dripdrop_premium_sub_$(date +%s)")" || return 1
    fi
    SUBSCRIBER_ADDRESS="$(normalize_hex_id "$SUBSCRIBER_ADDRESS")"
    ensure_wallet_funded "$SUBSCRIBER_ADDRESS" "$((SOCIAL_DEFAULT_GAS_BUDGET * 8))" || return 1

    if [[ -z "${MYUSD_TREASURY_CAP_ID:-}" ]]; then
        echo "MYUSD_TREASURY_CAP_ID required to fund subscriber (orderbook-session.env)" >&2
        return 1
    fi
    fund_subscriber_myusd "$MONTHLY_PRICE" || return 1
    sleep 2

    local coin out digest
    coin="$(pick_myusd_coin "$SUBSCRIBER_ADDRESS" "$MONTHLY_PRICE")" || return 1
    log_step "subscribe_to_profile<MYUSD> (non-platform) plan=$PLAN_ID"
    out="$(subscription_call_subscribe_to_profile_typed \
        "$SUBSCRIBER_ADDRESS" "$coin" "$MYUSD_COIN_TYPE" false 0 "$PLAN_ID")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "ProfileSubscriptionCreatedEvent" || return 1
    SUBSCRIPTION_ID="$(extract_created_object_by_type "$digest" "subscription::ProfileSubscription")" || return 1
    log_session_use "SUBSCRIPTION_ID" "$SUBSCRIPTION_ID"
    log_session_use "SUBSCRIBER_ADDRESS" "$SUBSCRIBER_ADDRESS"

    verify_subscription_layers "$SUBSCRIBER_ADDRESS" "$SERVICE_ID" true "$SUBSCRIPTION_ID" || return 1
    maybe_backend_status

    log_step "cancel_subscription<MYUSD>"
    out="$(subscription_call_cancel_subscription_typed \
        "$SUBSCRIBER_ADDRESS" "$SUBSCRIPTION_ID" "$MYUSD_COIN_TYPE")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "ProfileSubscriptionCancelledEvent" || return 1
    verify_subscription_layers "$SUBSCRIBER_ADDRESS" "$SERVICE_ID" false || return 1

    social_save_session "${SESSION_KEYS[@]}"
    print_run_summary_header "DripDrop Premium+ E2E"
    print_run_summary_line "Service" "$SERVICE_ID"
    print_run_summary_line "Monthly plan" "$PLAN_ID"
    print_run_summary_line "Subscriber" "$SUBSCRIBER_ADDRESS"
    print_run_summary_line "Subscription" "$SUBSCRIPTION_ID"
    echo "OK: subscribe (access=true) then cancel (access=false)."
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --refresh-session)
            DO_REFRESH=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage
            exit 1
            ;;
    esac
done

run_flow
