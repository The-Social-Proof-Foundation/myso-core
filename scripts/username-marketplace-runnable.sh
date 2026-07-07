#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper for profile::UsernameMarketplace (list → offer → accept).
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126
#   - After changing indexer handlers, rebuild and restart social indexer + social-server before --run-all
#   - `myso`, `curl`, `jq` on PATH
#
# Session: network.config/username-marketplace/marketplace-session.env
#
# Usage:
#   ./scripts/username-marketplace-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/username-marketplace-runnable.sh --run-all
#   ./scripts/username-marketplace-runnable.sh --reject-flow
#   ./scripts/username-marketplace-runnable.sh   # interactive menu
#
# Each accept/reject run uses fresh ephemeral seller + buyer wallets and new
# premium{runId}/seller{runId} usernames so the flow can be repeated on localnet.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/username-marketplace/marketplace-session.env"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

readonly MIN_LISTING_PRICE='5000000000'
readonly OFFER_AMOUNT='5000000000'
readonly EXPECTED_FEE='250000000'
readonly SELLER_NET='4750000000'

SOCIAL_RUN_ID="$(date +%s)"
RUN_MODE=''
DO_REFRESH=0
ASSUME_YES="${ASSUME_YES:-0}"

SELLER_ADDRESS=''
BUYER_ADDRESS=''
SELLER_PROFILE_ID=''
BUYER_PROFILE_ID=''
LISTING_USERNAME=''
REPLACEMENT_USERNAME=''
PAY_COIN_ID=''
SELLER_MEMORY_ACCOUNT_ID=''

MARKETPLACE_SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    USERNAME_REGISTRY_ID USERNAME_MARKETPLACE_ID PROFILE_CONFIG_ID
    AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID ECOSYSTEM_TREASURY_ID
    SELLER_ADDRESS BUYER_ADDRESS SELLER_PROFILE_ID BUYER_PROFILE_ID
    LISTING_USERNAME REPLACEMENT_USERNAME SELLER_MEMORY_ACCOUNT_ID
)

usage() {
    sed -n '2,20p' "$0" | sed 's/^# \?//'
}

save_marketplace_session() {
    social_save_session "${MARKETPLACE_SESSION_KEYS[@]}"
}

load_marketplace_session() {
    social_load_session
}

setup_usernames_for_run() {
    LISTING_USERNAME="premium${SOCIAL_RUN_ID}"
    REPLACEMENT_USERNAME="seller${SOCIAL_RUN_ID}"
}

ensure_seller_wallet() {
    if [[ -z "${SELLER_ADDRESS:-}" ]]; then
        SELLER_ADDRESS="$(create_ephemeral_wallet "um_seller_${SOCIAL_RUN_ID}")" || return 1
    fi
    SELLER_ADDRESS="$(normalize_hex_id "$SELLER_ADDRESS")"
    # Seller only pays gas (listing/accept); one faucet (~5 MYSO) is enough.
    ensure_wallet_funded "$SELLER_ADDRESS" "$((SOCIAL_DEFAULT_GAS_BUDGET * 3))" || return 1
    log_session_use "SELLER_ADDRESS" "$SELLER_ADDRESS"
}

ensure_buyer_wallet() {
    if [[ -z "${BUYER_ADDRESS:-}" ]]; then
        BUYER_ADDRESS="$(create_ephemeral_wallet "um_buyer_${SOCIAL_RUN_ID}")" || return 1
    fi
    BUYER_ADDRESS="$(normalize_hex_id "$BUYER_ADDRESS")"
    ensure_wallet_funded "$BUYER_ADDRESS" "$((OFFER_AMOUNT + SOCIAL_DEFAULT_GAS_BUDGET))" || return 1
    log_session_use "BUYER_ADDRESS" "$BUYER_ADDRESS"
}

prepare_new_run_wallets() {
    SELLER_ADDRESS=''
    SELLER_PROFILE_ID=''
    SELLER_MEMORY_ACCOUNT_ID=''
    BUYER_ADDRESS=''
    BUYER_PROFILE_ID=''
    BUYER_USERNAME=''
}

step_create_seller_profile() {
    local lines profile_id mem existing_user snap candidates=() resolved
    setup_usernames_for_run
    ensure_seller_wallet || return 1
    switch_wallet "$SELLER_ADDRESS" || return 1
    profile_id="$(resolve_owned_profile_for_address "$SELLER_ADDRESS")" || profile_id=''
    if [[ -n "$profile_id" ]]; then
        SELLER_PROFILE_ID="$(normalize_hex_id "$profile_id")"
        snap="$(gql_profile_snapshot "$SELLER_ADDRESS" 2>/dev/null)" || snap='{}'
        existing_user="$(echo "$snap" | jq -r '.data.profile.username // empty')"
        mem="$(echo "$snap" | jq -r '.data.profile.memoryAccountId // empty')"
        while IFS= read -r -d '' cand; do
            [[ -n "$cand" ]] && candidates+=("$cand")
        done < <(collect_marketplace_username_candidates "$existing_user")
        resolved="$(resolve_registry_username_for_profile "$SELLER_PROFILE_ID" "${candidates[@]}")" || {
            echo "Could not resolve on-chain username owned by seller profile $SELLER_PROFILE_ID (candidates: ${candidates[*]:-none})" >&2
            restore_wallet
            return 1
        }
        LISTING_USERNAME="$resolved"
        [[ -n "$mem" ]] && SELLER_MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
        log_step "Reusing seller profile $SELLER_PROFILE_ID on-chain listing username=$LISTING_USERNAME"
    else
        lines="$(create_profile_for_address "$SELLER_ADDRESS" "Premium Seller ${SOCIAL_RUN_ID}" "$LISTING_USERNAME")" || {
            restore_wallet
            return 1
        }
        profile_id="$(echo "$lines" | sed -n '1p')"
        mem="$(echo "$lines" | sed -n '2p')"
        SELLER_PROFILE_ID="$(normalize_hex_id "$profile_id")"
        [[ -n "$mem" ]] && SELLER_MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    fi
    restore_wallet
    log_session_use "SELLER_PROFILE_ID" "$SELLER_PROFILE_ID"
    log_session_use "LISTING_USERNAME" "$LISTING_USERNAME"
    save_marketplace_session
}

step_create_buyer_profile() {
    local lines profile_id buyer_username profile_id_existing
    ensure_buyer_wallet || return 1
    buyer_username="buyer${SOCIAL_RUN_ID}"
    BUYER_USERNAME="$buyer_username"
    switch_wallet "$BUYER_ADDRESS" || return 1
    profile_id_existing="$(resolve_owned_profile_for_address "$BUYER_ADDRESS")" || profile_id_existing=''
    if [[ -n "$profile_id_existing" ]]; then
        BUYER_PROFILE_ID="$(normalize_hex_id "$profile_id_existing")"
        log_step "Reusing buyer profile $BUYER_PROFILE_ID"
    else
        lines="$(create_profile_for_address "$BUYER_ADDRESS" "Buyer ${SOCIAL_RUN_ID}" "$buyer_username")" || {
            restore_wallet
            return 1
        }
        profile_id="$(echo "$lines" | sed -n '1p')"
        BUYER_PROFILE_ID="$(normalize_hex_id "$profile_id")"
    fi
    restore_wallet
    log_session_use "BUYER_PROFILE_ID" "$BUYER_PROFILE_ID"
    log_session_use "BUYER_USERNAME" "$BUYER_USERNAME"
    save_marketplace_session
}

username_marketplace_listing_active() {
    local username="$1" resp listed
    [[ -n "$username" ]] || return 1
    resp="$(gql_username_availability_snapshot "$username" 2>/dev/null)" || resp='{}'
    listed="$(echo "$resp" | jq -r '.data.usernameAvailability.marketplaceListed // false')"
    [[ "$listed" == "true" ]]
}

username_marketplace_listing_seller_profile_id() {
    local username="$1" resp
    resp="$(gql_username_availability_snapshot "$username" 2>/dev/null)" || resp='{}'
    echo "$resp" | jq -r '.data.usernameAvailability.listingSellerProfileId // empty'
}

step_cancel_username_listing() {
    require_session_fields USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID SELLER_PROFILE_ID CLOCK_ID || return 1
    switch_wallet "$SELLER_ADDRESS" || return 1
    log_step "cancel_username_listing username=$LISTING_USERNAME (stale listing cleanup)"
    SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$SELLER_ADDRESS" profile cancel_username_listing \
        "@${USERNAME_MARKETPLACE_ID}" "@${USERNAME_REGISTRY_ID}" "$SELLER_PROFILE_ID" \
        "$(literal_move_string "$LISTING_USERNAME")" "@${CLOCK_ID}" || {
        restore_wallet
        return 1
    }
    restore_wallet
}

step_create_username_listing() {
    local out rc listing_seller seller_norm
    require_session_fields USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID SELLER_PROFILE_ID CLOCK_ID || return 1
    [[ -n "${SELLER_PROFILE_ID:-}" ]] || {
        SELLER_PROFILE_ID="$(resolve_owned_profile_for_address "$SELLER_ADDRESS")" || true
    }
    [[ -n "${SELLER_PROFILE_ID:-}" ]] || { echo "SELLER_PROFILE_ID required" >&2; return 1; }

    seller_norm="$(normalize_hex_id "$SELLER_PROFILE_ID")"
    if username_marketplace_listing_active "$LISTING_USERNAME"; then
        listing_seller="$(username_marketplace_listing_seller_profile_id "$LISTING_USERNAME")"
        if [[ "$(normalize_hex_id "$listing_seller")" == "$seller_norm" ]]; then
            log_step "Reusing existing marketplace listing for $LISTING_USERNAME"
            return 0
        fi
        echo "Username $LISTING_USERNAME is already listed by profile $listing_seller (expected $seller_norm)" >&2
        return 1
    fi

    switch_wallet "$SELLER_ADDRESS" || return 1
    log_step "create_username_listing username=$LISTING_USERNAME min=$MIN_LISTING_PRICE"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$SELLER_ADDRESS" profile create_username_listing \
        "@${USERNAME_MARKETPLACE_ID}" "@${USERNAME_REGISTRY_ID}" "$SELLER_PROFILE_ID" \
        "$(literal_move_string "$LISTING_USERNAME")" "$MIN_LISTING_PRICE" "@${CLOCK_ID}" 2>&1)" || rc=$?
    if [[ "${rc:-0}" -ne 0 ]]; then
        if echo "$out" | grep -qE 'Abort Code: 37|EListingAlreadyExists'; then
            log_step "Listing already exists on-chain for $LISTING_USERNAME; attempting cancel + retry"
            restore_wallet
            step_cancel_username_listing || return 1
            switch_wallet "$SELLER_ADDRESS" || return 1
            SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$SELLER_ADDRESS" profile create_username_listing \
                "@${USERNAME_MARKETPLACE_ID}" "@${USERNAME_REGISTRY_ID}" "$SELLER_PROFILE_ID" \
                "$(literal_move_string "$LISTING_USERNAME")" "$MIN_LISTING_PRICE" "@${CLOCK_ID}" || {
                restore_wallet
                return 1
            }
            restore_wallet
            return 0
        fi
        echo "$out" >&2
        restore_wallet
        return 1
    fi
    restore_wallet
}

assert_listing_locked_indexer() {
    assert_username_marketplace_locked "$LISTING_USERNAME" "$SELLER_PROFILE_ID" || return 1
    assert_username_availability_case_insensitive "$LISTING_USERNAME" || return 1
}

step_create_username_offer() {
    local pay_coin gas_coin saved_gas
    require_session_fields USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID CLOCK_ID || return 1
    ensure_buyer_wallet || return 1

    switch_wallet "$BUYER_ADDRESS" || return 1
    read -r pay_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$BUYER_ADDRESS" "$OFFER_AMOUNT")" || {
        restore_wallet
        return 1
    }
    PAY_COIN_ID="$pay_coin"
    PTB_GAS_COIN_ID="$gas_coin"
    log_step "create_username_offer buyer=$BUYER_ADDRESS amount=$OFFER_AMOUNT"
    SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$BUYER_ADDRESS" profile create_username_offer \
        "@${USERNAME_MARKETPLACE_ID}" "@${USERNAME_REGISTRY_ID}" \
        "$(literal_move_string "$LISTING_USERNAME")" "$pay_coin" "$OFFER_AMOUNT" "@${CLOCK_ID}" || {
        PTB_GAS_COIN_ID=''
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID=''
    restore_wallet
}

assert_offer_pending_rest() {
    local url encoded buyer_norm
    encoded="$(python3 -c 'import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=""))' "$LISTING_USERNAME")"
    url="${SOCIAL_SERVER_URL}/usernames/${encoded}/offers"
    buyer_norm="$(normalize_hex_id "$BUYER_ADDRESS")"
    log_step "REST assert pending offer at $url"
    wait_for_rest_offer_status "$url" 'pending' "$buyer_norm" || return 1
    wait_for_rest_offer_field "$url" 'pending' "$buyer_norm" '.amount | tostring' "$OFFER_AMOUNT" || return 1
}

step_accept_username_offer() {
    require_session_fields USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID PROFILE_CONFIG_ID \
        ECOSYSTEM_TREASURY_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID SELLER_PROFILE_ID \
        PROFILE_CONFIG_ID ECOSYSTEM_TREASURY_ID CLOCK_ID || return 1
    [[ -n "${BUYER_ADDRESS:-}" ]] || { echo "BUYER_ADDRESS required" >&2; return 1; }

    ensure_seller_wallet || return 1
    switch_wallet "$SELLER_ADDRESS" || return 1
    log_step "active wallet=$(resolve_myso_active_address) seller=$SELLER_ADDRESS"
    log_step "accept_username_offer buyer=$BUYER_ADDRESS replacement=$REPLACEMENT_USERNAME"
    SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$SELLER_ADDRESS" profile accept_username_offer \
        "@${USERNAME_MARKETPLACE_ID}" "@${USERNAME_REGISTRY_ID}" "$SELLER_PROFILE_ID" \
        "$(literal_move_string "$LISTING_USERNAME")" "$BUYER_ADDRESS" \
        "$(literal_move_string "$REPLACEMENT_USERNAME")" \
        "@${PROFILE_CONFIG_ID}" "@${ECOSYSTEM_TREASURY_ID}" "@${CLOCK_ID}" || {
        restore_wallet
        return 1
    }
    restore_wallet
}

assert_offer_accepted_rest() {
    local url encoded fees_url buyer_norm
    encoded="$(python3 -c 'import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=""))' "$LISTING_USERNAME")"
    url="${SOCIAL_SERVER_URL}/usernames/${encoded}/offers"
    fees_url="${SOCIAL_SERVER_URL}/profiles/$(normalize_hex_id "$SELLER_ADDRESS")/username-sale-fees"
    buyer_norm="$(normalize_hex_id "$BUYER_ADDRESS")"
    log_step "REST assert accepted offer"
    wait_for_rest_offer_status "$url" 'accepted' "$buyer_norm" || return 1
    log_step "REST assert username sale fee"
    wait_for_rest_json "$fees_url" '.[0].fee_amount | tostring' "$EXPECTED_FEE" || return 1
    wait_for_rest_json "$fees_url" '.[0].sale_amount | tostring' "$OFFER_AMOUNT" || return 1
}

assert_username_swap_graphql() {
    local buyer_prior_username="${BUYER_USERNAME:-buyer${SOCIAL_RUN_ID}}"
    log_step "Assert username swap (on-chain registry is authoritative)"
    assert_on_chain_registry_username "$LISTING_USERNAME" "$BUYER_PROFILE_ID" || return 1
    assert_on_chain_registry_username "$REPLACEMENT_USERNAME" "$SELLER_PROFILE_ID" || return 1
    assert_on_chain_registry_username_absent "$buyer_prior_username" || return 1

    wait_for_gql_profile_field "$SELLER_ADDRESS" '.data.profile.profileId' "$(normalize_hex_id "$SELLER_PROFILE_ID")" || return 1
    wait_for_gql_profile_field "$BUYER_ADDRESS" '.data.profile.profileId' "$(normalize_hex_id "$BUYER_PROFILE_ID")" || return 1
    wait_for_gql_profile_field "$SELLER_ADDRESS" '.data.profile.username' "$REPLACEMENT_USERNAME" || return 1
    wait_for_gql_profile_field "$BUYER_ADDRESS" '.data.profile.username' "$LISTING_USERNAME" || return 1
    wait_for_gql_username_availability_field "$LISTING_USERNAME" '.data.usernameAvailability.marketplaceListed' 'false' || return 1
    wait_for_gql_username_availability_field "$LISTING_USERNAME" '.data.usernameAvailability.registryClaimed' 'true' || return 1
    log_step "GraphQL username swap + 1:1 registry verified"
}

assert_on_chain_profile_owner_unchanged() {
    local owner
    [[ -n "${SELLER_PROFILE_ID:-}" ]] || return 0
    owner="$(object_address_owner "$SELLER_PROFILE_ID")" || return 1
    if [[ "$(normalize_hex_id "$owner")" != "$(normalize_hex_id "$SELLER_ADDRESS")" ]]; then
        echo "On-chain profile owner expected $SELLER_ADDRESS got $owner" >&2
        return 1
    fi
    log_step "On-chain profile owner unchanged: $owner"
}

step_reject_username_offer() {
    require_session_fields USERNAME_MARKETPLACE_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_MARKETPLACE_ID SELLER_PROFILE_ID CLOCK_ID || return 1
    switch_wallet "$SELLER_ADDRESS" || return 1
    log_step "reject_or_revoke_username_offer buyer=$BUYER_ADDRESS"
    SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$SELLER_ADDRESS" profile reject_or_revoke_username_offer \
        "@${USERNAME_MARKETPLACE_ID}" "$SELLER_PROFILE_ID" \
        "$(literal_move_string "$LISTING_USERNAME")" "$BUYER_ADDRESS" "@${CLOCK_ID}" || {
        restore_wallet
        return 1
    }
    restore_wallet
}

assert_offer_rejected_rest() {
    local url encoded buyer_norm
    encoded="$(python3 -c 'import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=""))' "$LISTING_USERNAME")"
    url="${SOCIAL_SERVER_URL}/usernames/${encoded}/offers"
    buyer_norm="$(normalize_hex_id "$BUYER_ADDRESS")"
    log_step "REST assert rejected offer"
    wait_for_rest_offer_status "$url" 'rejected' "$buyer_norm" || return 1
}

run_marketplace_flow() {
    load_marketplace_session
    SOCIAL_RUN_ID="$(date +%s)"
    setup_usernames_for_run
    prepare_new_run_wallets
    require_session_fields USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID PROFILE_CONFIG_ID \
        AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID ECOSYSTEM_TREASURY_ID || {
        echo "Run --refresh-session first" >&2
        return 1
    }

    step_create_seller_profile || return 1
    step_create_buyer_profile || return 1
    step_create_username_listing || return 1
    assert_listing_locked_indexer || return 1
    step_create_username_offer || return 1
    assert_offer_pending_rest || return 1
    step_accept_username_offer || return 1
    assert_username_swap_graphql || return 1
    assert_offer_accepted_rest || return 1
    assert_on_chain_profile_owner_unchanged || return 1
    save_marketplace_session
    print_username_marketplace_accept_summary
}

print_username_marketplace_accept_summary() {
    print_run_summary_header "Username Marketplace — accept flow completed"
    print_run_summary_line "Listed username (sold)" "$LISTING_USERNAME"
    print_run_summary_line "Seller" "$(normalize_hex_id "$SELLER_ADDRESS") (profile $(normalize_hex_id "$SELLER_PROFILE_ID"))"
    print_run_summary_line "Buyer" "$(normalize_hex_id "$BUYER_ADDRESS") (profile $(normalize_hex_id "$BUYER_PROFILE_ID"))"
    print_run_summary_line "Buyer username (after sale)" "$LISTING_USERNAME"
    print_run_summary_line "Seller replacement username" "$REPLACEMENT_USERNAME"
    print_run_summary_line "Offer amount" "$(format_mist_with_units "$OFFER_AMOUNT")"
    print_run_summary_line "Marketplace fee (5%)" "$(format_mist_with_units "$EXPECTED_FEE")"
    print_run_summary_line "Seller net proceeds" "$(format_mist_with_units "$SELLER_NET")"
    print_run_summary_line "Outcome" "Offer accepted; username transferred to buyer; seller keeps profile with replacement username"
    print_run_summary_footer
}

print_username_marketplace_reject_summary() {
    print_run_summary_header "Username Marketplace — reject flow completed"
    print_run_summary_line "Listed username" "$LISTING_USERNAME"
    print_run_summary_line "Seller" "$(normalize_hex_id "$SELLER_ADDRESS")"
    print_run_summary_line "Buyer" "$(normalize_hex_id "$BUYER_ADDRESS")"
    print_run_summary_line "Offer amount" "$(format_mist_with_units "$OFFER_AMOUNT")"
    print_run_summary_line "Outcome" "Offer rejected; listing remains locked to seller; buyer coin returned"
    print_run_summary_footer
}

run_reject_flow() {
    load_marketplace_session
    SOCIAL_RUN_ID="$(date +%s)"
    setup_usernames_for_run
    prepare_new_run_wallets
    require_session_fields USERNAME_MARKETPLACE_ID USERNAME_REGISTRY_ID PROFILE_CONFIG_ID \
        AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID ECOSYSTEM_TREASURY_ID || return 1

    step_create_seller_profile || return 1
    step_create_buyer_profile || return 1
    step_create_username_listing || return 1
    assert_listing_locked_indexer || return 1
    step_create_username_offer || return 1
    assert_offer_pending_rest || return 1
    step_reject_username_offer || return 1
    assert_offer_rejected_rest || return 1
    save_marketplace_session
    print_username_marketplace_reject_summary
}

show_menu() {
    echo ""
    echo "=== Username Marketplace E2E Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Run full accept flow (--run-all)"
    echo " 2) Run reject flow"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) social_refresh_session_from_graphql; load_marketplace_session ;;
        1) run_marketplace_flow ;;
        2) run_reject_flow ;;
        [Hh]) usage ;;
        [Qq]) exit 0 ;;
        *) echo "Invalid choice" ;;
    esac
    show_menu
}

main() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h) usage; exit 0 ;;
            -y) ASSUME_YES=1; shift ;;
            --refresh-session) DO_REFRESH=1; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            --reject-flow) RUN_MODE=reject; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    load_marketplace_session

    if [[ "$DO_REFRESH" == 1 ]]; then
        social_refresh_session_from_graphql
        load_marketplace_session
    fi

    case "${RUN_MODE:-}" in
        run_all) run_marketplace_flow; exit 0 ;;
        reject) run_reject_flow; exit 0 ;;
        '') [[ "$DO_REFRESH" == 1 ]] && exit 0; show_menu ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
