#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Create DripDrop MYUSD profile subscription plans after a greenfield reset.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - Multi-coin subscription Move changes deployed (generic create_subscription_plan<T>)
#   - DripDrop platform + profile exist on chain (bootstrap or prior session)
#   - MYUSD coin type published (bridge token package)
#   - Set MYUSD_COIN_TYPE (e.g. 0xPACKAGE::myusd::MYUSD) or PKG_MYUSD
#
# Session: network.config/dripdrop/dripdrop-subscription-plans-session.env
# Reuses network.config/subscription/subscription-session.env SERVICE_ID when present.
#
# Usage:
#   ./scripts/dripdrop-myusd-subscription-plans.sh --refresh-session
#   ASSUME_YES=1 ./scripts/dripdrop-myusd-subscription-plans.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/dripdrop/dripdrop-subscription-plans-session.env"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/subscription-test-common.sh
source "${SCRIPT_DIR}/lib/subscription-test-common.sh"

readonly MONTHLY_TITLE='DripDrop Badge Monthly'
readonly ANNUAL_TITLE='DripDrop Badge Annual'
readonly MONTHLY_PRICE='3990000'
readonly ANNUAL_PRICE='39990000'
readonly MONTHLY_DURATION_MS='2592000000'
readonly ANNUAL_DURATION_MS='31536000000'

ASSUME_YES="${ASSUME_YES:-0}"
DO_REFRESH=0
CREATOR_ADDRESS=''
CREATOR_PROFILE_ID=''
SERVICE_ID=''
MONTHLY_PLAN_ID=''
ANNUAL_PLAN_ID=''
MYUSD_COIN_TYPE="${MYUSD_COIN_TYPE:-}"

SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    SUBSCRIPTION_CONFIG_ID PLATFORM_OBJECT_ID DRIPDROP_PLATFORM_ID
    CREATOR_ADDRESS CREATOR_PROFILE_ID SERVICE_ID
    MYUSD_COIN_TYPE MONTHLY_PLAN_ID ANNUAL_PLAN_ID
)

usage() {
    sed -n '2,22p' "$0" | sed 's/^# \?//'
}

save_plans_session() {
    social_save_session "${SESSION_KEYS[@]}"
}

load_plans_session() {
    social_load_session
    if [[ -z "${SERVICE_ID:-}" && -f "$REPO_ROOT/network.config/subscription/subscription-session.env" ]]; then
        # shellcheck disable=SC1091
        source "$REPO_ROOT/network.config/subscription/subscription-session.env"
        log_step "Imported SERVICE_ID from subscription-session.env if set"
    fi
}

resolve_myusd_coin_type() {
    if [[ -n "${MYUSD_COIN_TYPE:-}" ]]; then
        printf '%s' "$MYUSD_COIN_TYPE"
        return 0
    fi
    if [[ -n "${PKG_MYUSD:-}" ]]; then
        printf '%s::myusd::MYUSD' "$(normalize_hex_id "$PKG_MYUSD")"
        return 0
    fi
    echo "Set MYUSD_COIN_TYPE (published MYUSD type, e.g. 0xPACKAGE::myusd::MYUSD) or PKG_MYUSD" >&2
    return 1
}

ensure_dripdrop_platform() {
    if [[ -n "${DRIPDROP_PLATFORM_ID:-}" ]]; then
        PLATFORM_OBJECT_ID="$(normalize_hex_id "$DRIPDROP_PLATFORM_ID")"
    elif [[ -n "${PLATFORM_OBJECT_ID:-}" ]]; then
        PLATFORM_OBJECT_ID="$(normalize_hex_id "$PLATFORM_OBJECT_ID")"
        DRIPDROP_PLATFORM_ID="$PLATFORM_OBJECT_ID"
    else
        echo "PLATFORM_OBJECT_ID / DRIPDROP_PLATFORM_ID is required (run bootstrap or --refresh-session)" >&2
        return 1
    fi
    log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
}

ensure_creator() {
    if [[ -z "${CREATOR_ADDRESS:-}" ]]; then
        CREATOR_ADDRESS="$(resolve_myso_active_address)" || return 1
    fi
    CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")" || return 1
    if [[ -n "${ADMIN_PROFILE_ID:-}" ]] && object_exists_on_fullnode "$ADMIN_PROFILE_ID"; then
        CREATOR_PROFILE_ID="$(normalize_hex_id "$ADMIN_PROFILE_ID")"
    elif [[ -n "${CREATOR_PROFILE_ID:-}" ]] && object_exists_on_fullnode "$CREATOR_PROFILE_ID"; then
        CREATOR_PROFILE_ID="$(normalize_hex_id "$CREATOR_PROFILE_ID")"
    else
        CREATOR_PROFILE_ID="$(resolve_owned_profile_for_address "$CREATOR_ADDRESS")" || CREATOR_PROFILE_ID=''
        CREATOR_PROFILE_ID="$(normalize_hex_id "${CREATOR_PROFILE_ID:-}")" || {
            echo "DripDrop creator profile not found for $CREATOR_ADDRESS" >&2
            return 1
        }
    fi
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
    log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
}

ensure_service() {
    local out digest existing
    existing="$(subscription_resolve_existing_service_for_profile "$CREATOR_PROFILE_ID" 2>/dev/null)" || existing=''
    if [[ -n "$existing" ]]; then
        SERVICE_ID="$(normalize_hex_id "$existing")"
        log_step "Reusing subscription service $SERVICE_ID"
        log_session_use "SERVICE_ID" "$SERVICE_ID"
        return 0
    fi
    log_step "Creating profile subscription service on DripDrop profile"
    out="$(run_myso_call_as_capture "$CREATOR_ADDRESS" subscription create_profile_service_entry \
        "@$(normalize_hex_id "$CREATOR_PROFILE_ID")" "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || { echo "create_profile_service_entry failed" >&2; return 1; }
    digest="$(extract_tx_digest "$out")"
    SERVICE_ID="$(extract_created_object_by_type "$digest" "subscription::ProfileSubscriptionService")" || return 1
    log_session_use "SERVICE_ID" "$SERVICE_ID"
}

create_myusd_plan() {
    local title="$1" price="$2" duration_ms="$3" out digest plan_id
    out="$(subscription_call_create_subscription_plan_myusd \
        "$CREATOR_ADDRESS" "$title" "$price" "$duration_ms" "$MYUSD_COIN_TYPE")" || return 1
    assert_tx_success "$out" || { echo "create_subscription_plan<MYUSD> failed for $title" >&2; return 1; }
    digest="$(extract_tx_digest "$out")"
    plan_id="$(extract_created_object_by_type "$digest" "subscription::SubscriptionPlan")" || return 1
    printf '%s' "$plan_id"
}

run_create_plans() {
    require_session_fields PKG_SOCIAL SUBSCRIPTION_CONFIG_ID CLOCK_ID || return 1
    MYUSD_COIN_TYPE="$(resolve_myusd_coin_type)" || return 1
    ensure_dripdrop_platform || return 1
    ensure_creator || return 1
    ensure_service || return 1
    log_step "Creating MYUSD monthly plan ($MONTHLY_PRICE / $MONTHLY_DURATION_MS ms)"
    MONTHLY_PLAN_ID="$(create_myusd_plan "$MONTHLY_TITLE" "$MONTHLY_PRICE" "$MONTHLY_DURATION_MS")" || return 1
    log_session_use "MONTHLY_PLAN_ID" "$MONTHLY_PLAN_ID"
    log_step "Creating MYUSD annual plan ($ANNUAL_PRICE / $ANNUAL_DURATION_MS ms)"
    ANNUAL_PLAN_ID="$(create_myusd_plan "$ANNUAL_TITLE" "$ANNUAL_PRICE" "$ANNUAL_DURATION_MS")" || return 1
    log_session_use "ANNUAL_PLAN_ID" "$ANNUAL_PLAN_ID"
    save_plans_session
    echo ""
    echo "DripDrop MYUSD subscription plans"
    echo "  service:        $SERVICE_ID"
    echo "  coin_type:      $MYUSD_COIN_TYPE"
    echo "  monthly_plan:   $MONTHLY_PLAN_ID  ($MONTHLY_TITLE $MONTHLY_PRICE)"
    echo "  annual_plan:    $ANNUAL_PLAN_ID  ($ANNUAL_TITLE $ANNUAL_PRICE)"
    echo "  platform:       $PLATFORM_OBJECT_ID"
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

load_plans_session
if [[ "$DO_REFRESH" == 1 ]]; then
    social_refresh_session_from_graphql || exit 1
    load_plans_session
fi
run_create_plans
