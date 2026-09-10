#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Grant DripDrop PlatformBadgeAdmin to the badge-distributor wallet, then
# optionally smoke-assign the Premium+ platform badge.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed (DripDrop platform exists)
#   - myso start --with-faucet --with-indexer --with-social-indexer --with-graphql
#   - DRIPDROP_BADGE_MOD_ADDRESS set (backend PREMIUM_BADGE_SIGNER wallet)
#
# Session: network.config/dripdrop/dripdrop-premium-badges-session.env
# Optional secrets: network.config/dripdrop/local-premium-badge-secrets.env
#
# Usage:
#   DRIPDROP_BADGE_MOD_ADDRESS=0x... ASSUME_YES=1 ./scripts/dripdrop-premium-plus-badges.sh
#   ASSIGN_RECIPIENT_ADDRESS=0x... ASSUME_YES=1 ./scripts/dripdrop-premium-plus-badges.sh --assign
#   ./scripts/dripdrop-premium-plus-badges.sh --refresh-session
#   DRY_RUN=1 DRIPDROP_BADGE_MOD_ADDRESS=0x... ./scripts/dripdrop-premium-plus-badges.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/dripdrop/dripdrop-premium-badges-session.env"
PLANS_SESSION="$REPO_ROOT/network.config/dripdrop/dripdrop-subscription-plans-session.env"
SOCIAL_SESSION="$REPO_ROOT/network.config/social/social-session.env"

# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/dripdrop-badge-common.sh
source "${SCRIPT_DIR}/lib/dripdrop-badge-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

ASSUME_YES="${ASSUME_YES:-0}"
DO_REFRESH=0
DO_ASSIGN=0
DRIPDROP_BADGE_ADMIN_ADDRESS="${DRIPDROP_BADGE_ADMIN_ADDRESS:-}"
DRIPDROP_BADGE_MOD_ADDRESS="${DRIPDROP_BADGE_MOD_ADDRESS:-}"
ASSIGN_RECIPIENT_ADDRESS="${ASSIGN_RECIPIENT_ADDRESS:-}"
RECIPIENT_PROFILE_ID="${RECIPIENT_PROFILE_ID:-}"
PERMISSIONED_GROUP_ID="${PERMISSIONED_GROUP_ID:-}"
PREMIUM_BADGE_ID="${PREMIUM_BADGE_ID:-}"

SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID GAS_BUDGET
    PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_OBJECT_ID DRIPDROP_PLATFORM_ID
    PERMISSIONED_GROUP_ID
    DRIPDROP_BADGE_ADMIN_ADDRESS DRIPDROP_BADGE_MOD_ADDRESS
    PREMIUM_BADGE_NAME PREMIUM_BADGE_DESCRIPTION
    PREMIUM_BADGE_ICON_URL PREMIUM_BADGE_MEDIA_URL PREMIUM_BADGE_TYPE
    PREMIUM_BADGE_ID
)

usage() {
    sed -n '2,21p' "$0" | sed 's/^# \?//'
}

save_badge_session() {
    social_save_session "${SESSION_KEYS[@]}"
}

import_social_and_plans() {
    if [[ -f "$SOCIAL_SESSION" ]]; then
        # shellcheck disable=SC1090
        source "$SOCIAL_SESSION"
        log_step "Imported live IDs from social-session.env"
    fi
    if [[ -f "$PLANS_SESSION" ]]; then
        # shellcheck disable=SC1090
        source "$PLANS_SESSION"
        log_step "Imported IDs from dripdrop-subscription-plans-session.env"
    fi
}

load_badge_session() {
    social_load_session
    import_social_and_plans
    dripdrop_load_badge_secrets
    dripdrop_apply_badge_defaults
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
    DRIPDROP_PLATFORM_ID="$(normalize_hex_id "$DRIPDROP_PLATFORM_ID")"
    log_session_use "DRIPDROP_PLATFORM_ID" "$DRIPDROP_PLATFORM_ID"
}

ensure_badge_wallets() {
    if [[ -z "${DRIPDROP_BADGE_ADMIN_ADDRESS:-}" ]]; then
        DRIPDROP_BADGE_ADMIN_ADDRESS="$(resolve_myso_active_address)" || return 1
    fi
    DRIPDROP_BADGE_ADMIN_ADDRESS="$(normalize_hex_id "$DRIPDROP_BADGE_ADMIN_ADDRESS")" || return 1
    if [[ -z "${DRIPDROP_BADGE_MOD_ADDRESS:-}" ]]; then
        echo "Set DRIPDROP_BADGE_MOD_ADDRESS (DripDrop badge distributor / PREMIUM_BADGE_SIGNER wallet)" >&2
        return 1
    fi
    DRIPDROP_BADGE_MOD_ADDRESS="$(normalize_hex_id "$DRIPDROP_BADGE_MOD_ADDRESS")" || return 1
    log_session_use "DRIPDROP_BADGE_ADMIN_ADDRESS" "$DRIPDROP_BADGE_ADMIN_ADDRESS"
    log_session_use "DRIPDROP_BADGE_MOD_ADDRESS" "$DRIPDROP_BADGE_MOD_ADDRESS"
}

ensure_moderators_group() {
    PERMISSIONED_GROUP_ID="$(dripdrop_resolve_moderators_group_id "$DRIPDROP_PLATFORM_ID")" || return 1
    log_session_use "PERMISSIONED_GROUP_ID" "$PERMISSIONED_GROUP_ID"
}

resolve_assign_recipient_profile() {
    local addr="$1"
    addr="$(normalize_hex_id "$addr")" || return 1
    if [[ -n "${RECIPIENT_PROFILE_ID:-}" ]] && object_exists_on_fullnode "$RECIPIENT_PROFILE_ID"; then
        RECIPIENT_PROFILE_ID="$(normalize_hex_id "$RECIPIENT_PROFILE_ID")"
    else
        RECIPIENT_PROFILE_ID="$(resolve_owned_profile_for_address "$addr")" || RECIPIENT_PROFILE_ID=''
        RECIPIENT_PROFILE_ID="$(normalize_hex_id "${RECIPIENT_PROFILE_ID:-}")" || {
            echo "Profile not found for assign recipient $addr" >&2
            return 1
        }
    fi
    ASSIGN_RECIPIENT_ADDRESS="$addr"
    log_session_use "ASSIGN_RECIPIENT_ADDRESS" "$ASSIGN_RECIPIENT_ADDRESS"
    log_session_use "RECIPIENT_PROFILE_ID" "$RECIPIENT_PROFILE_ID"
}

run_grant_phase() {
    local developer
    require_session_fields PKG_SOCIAL PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID CLOCK_ID || return 1
    ensure_dripdrop_platform || return 1
    ensure_badge_wallets || return 1
    ensure_moderators_group || return 1
    ensure_wallet_funded "$DRIPDROP_BADGE_ADMIN_ADDRESS" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    ensure_wallet_funded "$DRIPDROP_BADGE_MOD_ADDRESS" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1

    developer="$(dripdrop_platform_developer_address "$DRIPDROP_PLATFORM_ID" 2>/dev/null || true)"
    developer="$(normalize_hex_id "${developer:-}")" || developer=''
    if [[ -n "$developer" && "$developer" == "$DRIPDROP_BADGE_MOD_ADDRESS" ]]; then
        log_step "Mod wallet is the DripDrop platform developer; PlatformBadgeAdmin already implied"
    elif dripdrop_wallet_can_manage_badges "$DRIPDROP_PLATFORM_ID" "$DRIPDROP_BADGE_MOD_ADDRESS" 2>/dev/null; then
        log_step "Mod wallet already has canManageBadges; skipping grant"
    else
        dripdrop_grant_badge_admin \
            "$DRIPDROP_BADGE_ADMIN_ADDRESS" \
            "$DRIPDROP_PLATFORM_ID" \
            "$PERMISSIONED_GROUP_ID" \
            "$DRIPDROP_BADGE_MOD_ADDRESS" || return 1
    fi
    PREMIUM_BADGE_ID="$(dripdrop_premium_badge_id "$DRIPDROP_PLATFORM_ID" "$PREMIUM_BADGE_NAME")"
    log_session_use "PREMIUM_BADGE_ID" "$PREMIUM_BADGE_ID"
}

run_assign_phase() {
    [[ "$DO_ASSIGN" == 1 ]] || return 0
    if [[ -z "${ASSIGN_RECIPIENT_ADDRESS:-}" ]]; then
        echo "--assign requires an address argument or ASSIGN_RECIPIENT_ADDRESS" >&2
        return 1
    fi
    resolve_assign_recipient_profile "$ASSIGN_RECIPIENT_ADDRESS" || return 1
    dripdrop_assign_premium_badge "$DRIPDROP_BADGE_MOD_ADDRESS" "$RECIPIENT_PROFILE_ID" >/dev/null || return 1
    if [[ "${DRY_RUN:-0}" != 1 ]]; then
        dripdrop_wait_for_profile_badge "$ASSIGN_RECIPIENT_ADDRESS" "$PREMIUM_BADGE_ID" || return 1
    fi
}

run_flow() {
    load_badge_session
    if [[ "$DO_REFRESH" == 1 ]]; then
        social_refresh_session_from_graphql || exit 1
        load_badge_session
    fi
    dripdrop_apply_badge_defaults
    run_grant_phase || exit 1
    run_assign_phase || exit 1
    save_badge_session
    print_run_summary_header "DripDrop Premium+ badges"
    print_run_summary_line "Platform" "$DRIPDROP_PLATFORM_ID"
    print_run_summary_line "Moderators group" "$PERMISSIONED_GROUP_ID"
    print_run_summary_line "Admin" "$DRIPDROP_BADGE_ADMIN_ADDRESS"
    print_run_summary_line "Mod / signer" "$DRIPDROP_BADGE_MOD_ADDRESS"
    print_run_summary_line "Badge name" "$PREMIUM_BADGE_NAME"
    print_run_summary_line "Badge id" "$PREMIUM_BADGE_ID"
    print_run_summary_line "Icon URL" "$PREMIUM_BADGE_ICON_URL"
    if [[ "$DO_ASSIGN" == 1 ]]; then
        print_run_summary_line "Assigned to" "$ASSIGN_RECIPIENT_ADDRESS"
        print_run_summary_line "Profile" "$RECIPIENT_PROFILE_ID"
    fi
    print_run_summary_footer
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --refresh-session)
            DO_REFRESH=1
            shift
            ;;
        --assign)
            DO_ASSIGN=1
            if [[ $# -ge 2 && "$2" != -* ]]; then
                ASSIGN_RECIPIENT_ADDRESS="$2"
                shift 2
            else
                shift
            fi
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
