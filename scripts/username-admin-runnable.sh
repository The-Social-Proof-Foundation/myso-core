#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper for profile::UsernameAdminCap (admin_reassign_username only).
# Single-profile rename: assign an unclaimed new_username to one profile and free
# that profile's prior username. No other profile is touched.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed (UsernameAdminCap claimed by admin wallet)
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Framework/localnet includes single-profile admin_reassign_username
#   - `myso`, `curl`, `jq` on PATH
#
# Session: network.config/username-admin/username-admin-session.env
#
# Usage:
#   ./scripts/username-admin-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/username-admin-runnable.sh --run-all
#   ./scripts/username-admin-runnable.sh   # interactive menu

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/username-admin/username-admin-session.env"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

readonly REASSIGN_REASON_CODE='2'

SOCIAL_RUN_ID="$(date +%s)"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"

PRIMARY_ADDRESS=''
PRIMARY_PROFILE_ID=''
PRIMARY_USERNAME=''
PRIMARY_PRIOR_USERNAME=''
NEW_USERNAME=''
USERNAME_ADMIN_CAP_ID=''
ADMIN_SENDER=''
ADMIN_REASSIGN_TX=''

USERNAME_ADMIN_SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID
    USERNAME_ADMIN_CAP_ID ADMIN_SENDER
    PRIMARY_ADDRESS PRIMARY_PROFILE_ID PRIMARY_USERNAME PRIMARY_PRIOR_USERNAME
    NEW_USERNAME ADMIN_REASSIGN_TX
)

readonly USERNAME_ADMIN_GQL_EXTRAS='query UsernameAdminCapExtras {
  usernameAdminCap: objects(filter: { type: "0x50c1::profile::UsernameAdminCap" }, last: 1) { nodes { address } }
}'

usage() {
    sed -n '2,20p' "$0" | sed 's/^# \?//'
}

save_username_admin_session() {
    social_save_session "${USERNAME_ADMIN_SESSION_KEYS[@]}"
}

load_username_admin_session() {
    social_load_session
}

resolve_username_admin_sender() {
    local admin_addr
    if [[ -z "${USERNAME_ADMIN_CAP_ID:-}" ]] || ! object_exists_on_fullnode "$USERNAME_ADMIN_CAP_ID"; then
        social_validate_session_id_on_fullnode USERNAME_ADMIN_CAP_ID \
            '0x50c1::profile::UsernameAdminCap' ANY 1 || return 1
    fi
    [[ -n "${USERNAME_ADMIN_CAP_ID:-}" ]] || return 1
    admin_addr="$(object_address_owner "$USERNAME_ADMIN_CAP_ID" 2>/dev/null)" || return 1
    ADMIN_SENDER="$(normalize_hex_id "$admin_addr")"
    log_session_use "ADMIN_SENDER" "$ADMIN_SENDER"
    log_session_use "USERNAME_ADMIN_CAP_ID" "$USERNAME_ADMIN_CAP_ID"
}

refresh_username_admin_session_from_graphql() {
    social_refresh_session_from_graphql || return 1
    load_username_admin_session

    local json
    log_step "Refreshing UsernameAdminCap from GraphQL ($GRAPHQL_URL)"
    json="$(graphql_post "$USERNAME_ADMIN_GQL_EXTRAS")" || return 1
    USERNAME_ADMIN_CAP_ID="$(gql_object_address "$json" usernameAdminCap)"
    if [[ -n "${USERNAME_ADMIN_CAP_ID:-}" ]] && ! object_exists_on_fullnode "$USERNAME_ADMIN_CAP_ID"; then
        echo "USERNAME_ADMIN_CAP_ID=${USERNAME_ADMIN_CAP_ID} not on fullnode; run bootstrap then --refresh-session" >&2
        USERNAME_ADMIN_CAP_ID=''
    fi
    require_hex_ids USERNAME_ADMIN_CAP_ID USERNAME_REGISTRY_ID || return 1
    resolve_username_admin_sender || return 1
    save_username_admin_session
}

ensure_username_admin_session() {
    load_username_admin_session
    if [[ -z "${USERNAME_REGISTRY_ID:-}" || -z "${USERNAME_ADMIN_CAP_ID:-}" || -z "${ADMIN_SENDER:-}" ]]; then
        log_step "Session incomplete — refreshing from GraphQL"
        refresh_username_admin_session_from_graphql || return 1
        load_username_admin_session
    fi
    require_session_fields USERNAME_REGISTRY_ID USERNAME_ADMIN_CAP_ID PROFILE_CONFIG_ID \
        AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID || return 1
    resolve_username_admin_sender || return 1
}

ensure_primary_wallet() {
    PRIMARY_ADDRESS="$(resolve_myso_active_address)" || {
        echo "Could not read myso client active-address" >&2
        return 1
    }
    PRIMARY_ADDRESS="$(normalize_hex_id "$PRIMARY_ADDRESS")"
    ensure_wallet_funded "$PRIMARY_ADDRESS" "$((SOCIAL_DEFAULT_GAS_BUDGET * 2))" || return 1
    log_session_use "PRIMARY_ADDRESS" "$PRIMARY_ADDRESS"
}

step_ensure_primary_profile() {
    local lines profile_id username snap existing_user profile_id_existing
    ensure_primary_wallet || return 1
    username="ua${SOCIAL_RUN_ID}"
    switch_wallet "$PRIMARY_ADDRESS" || return 1
    profile_id_existing="$(resolve_owned_profile_for_address "$PRIMARY_ADDRESS")" || profile_id_existing=''
    if [[ -n "$profile_id_existing" ]]; then
        PRIMARY_PROFILE_ID="$(normalize_hex_id "$profile_id_existing")"
        snap="$(gql_profile_snapshot "$PRIMARY_ADDRESS" 2>/dev/null)" || snap='{}'
        existing_user="$(echo "$snap" | jq -r '.data.profile.username // empty')"
        if [[ -n "$existing_user" ]]; then
            PRIMARY_USERNAME="$existing_user"
            log_step "Reusing primary profile $PRIMARY_PROFILE_ID username=$PRIMARY_USERNAME"
        else
            echo "Primary profile $PRIMARY_PROFILE_ID has no username; create a username first or use a fresh wallet." >&2
            restore_wallet
            return 1
        fi
    else
        lines="$(create_profile_for_address "$PRIMARY_ADDRESS" "Username Admin Primary ${SOCIAL_RUN_ID}" "$username")" || {
            restore_wallet
            return 1
        }
        profile_id="$(echo "$lines" | sed -n '1p')"
        PRIMARY_PROFILE_ID="$(normalize_hex_id "$profile_id")"
        PRIMARY_USERNAME="$username"
        log_step "Created primary profile $PRIMARY_PROFILE_ID username=$PRIMARY_USERNAME"
    fi
    restore_wallet
    PRIMARY_PRIOR_USERNAME="$PRIMARY_USERNAME"
    log_session_use "PRIMARY_PROFILE_ID" "$PRIMARY_PROFILE_ID"
    log_session_use "PRIMARY_USERNAME" "$PRIMARY_USERNAME"
    log_session_use "PRIMARY_PRIOR_USERNAME" "$PRIMARY_PRIOR_USERNAME"
    save_username_admin_session
}

prompt_with_default() {
    local label="$1"
    local default="$2"
    local _read
    if [[ "${ASSUME_YES:-0}" == 1 ]] || [[ ! -t 0 ]]; then
        printf '%s' "$default"
        return 0
    fi
    if [[ -n "$default" ]]; then
        read -r -p "${label} [${default}]: " _read || true
        printf '%s' "${_read:-$default}"
    else
        read -r -p "${label}: " _read
        printf '%s' "$_read"
    fi
}

resolve_new_username() {
    local default="uarenew${SOCIAL_RUN_ID}" chosen
    chosen="$(prompt_with_default "New unclaimed username for primary profile" "$default")"
    chosen="$(echo "$chosen" | tr '[:upper:]' '[:lower:]' | tr -d '[:space:]')"
    if [[ -z "$chosen" ]]; then
        echo "New username cannot be empty" >&2
        return 1
    fi
    if [[ "$chosen" == "${PRIMARY_PRIOR_USERNAME:-}" ]]; then
        echo "New username must differ from the profile's current username ($PRIMARY_PRIOR_USERNAME)" >&2
        return 1
    fi
    if ! assert_on_chain_registry_username_absent "$chosen"; then
        echo "Choose an unregistered username (Move EUsernameNotAvailable otherwise)." >&2
        return 1
    fi
    NEW_USERNAME="$chosen"
}

step_admin_reassign_username() {
    local out digest
    require_hex_ids USERNAME_ADMIN_CAP_ID USERNAME_REGISTRY_ID PRIMARY_PROFILE_ID || return 1
    resolve_new_username || return 1
    [[ -n "${NEW_USERNAME:-}" ]] || {
        echo "NEW_USERNAME required" >&2
        return 1
    }
    ensure_wallet_funded "$ADMIN_SENDER" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    switch_wallet "$ADMIN_SENDER" || return 1
    log_step "admin_reassign_username profile=$PRIMARY_PROFILE_ID new_username=$NEW_USERNAME reason=$REASSIGN_REASON_CODE"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$ADMIN_SENDER" profile admin_reassign_username \
        "@${USERNAME_ADMIN_CAP_ID}" "@${USERNAME_REGISTRY_ID}" \
        "$PRIMARY_PROFILE_ID" \
        "$(literal_move_string "$NEW_USERNAME")" \
        "$REASSIGN_REASON_CODE")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    ADMIN_REASSIGN_TX="$digest"
    PRIMARY_USERNAME="$NEW_USERNAME"
    log_session_use "NEW_USERNAME" "$NEW_USERNAME"
    log_session_use "ADMIN_REASSIGN_TX" "$ADMIN_REASSIGN_TX"
    log_session_use "PRIMARY_USERNAME" "$PRIMARY_USERNAME"
    save_username_admin_session
}

assert_reassign_indexed() {
    log_step "Assert rename indexed: primary owns $NEW_USERNAME; prior $PRIMARY_PRIOR_USERNAME freed"
    assert_on_chain_registry_username "$NEW_USERNAME" "$PRIMARY_PROFILE_ID" || return 1
    assert_on_chain_registry_username_absent "$PRIMARY_PRIOR_USERNAME" || return 1
    wait_for_gql_profile_field "$PRIMARY_ADDRESS" '.data.profile.username' "$NEW_USERNAME" || return 1
    wait_for_gql_username_availability_field "$NEW_USERNAME" \
        '.data.usernameAvailability.registryClaimed' 'true' || return 1
    wait_for_gql_username_availability_field "$NEW_USERNAME" \
        '.data.usernameAvailability.available' 'false' || return 1
    wait_for_gql_username_availability_field "$PRIMARY_PRIOR_USERNAME" \
        '.data.usernameAvailability.available' 'true' || return 1
    wait_for_gql_username_availability_field "$PRIMARY_PRIOR_USERNAME" \
        '.data.usernameAvailability.registryClaimed' 'false' || return 1
    log_step "Rename assertions passed (profile named; prior username available)"
}

print_username_admin_summary() {
    print_run_summary_header "Username Admin E2E — rename completed"
    print_run_summary_line "Run ID" "$SOCIAL_RUN_ID"
    print_run_summary_line "Admin sender" "$(normalize_hex_id "$ADMIN_SENDER")"
    print_run_summary_line "UsernameAdminCap" "$(normalize_hex_id "$USERNAME_ADMIN_CAP_ID")"
    print_run_summary_line "Primary wallet" "$(normalize_hex_id "$PRIMARY_ADDRESS") (profile $(normalize_hex_id "$PRIMARY_PROFILE_ID"))"
    print_run_summary_line "Primary prior username (freed)" "$PRIMARY_PRIOR_USERNAME"
    print_run_summary_line "New username" "$NEW_USERNAME"
    print_run_summary_line "Reassign tx" "${ADMIN_REASSIGN_TX:-<none>}"
    print_run_summary_line "Primary username (after)" "$PRIMARY_USERNAME"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Outcome" "Admin renamed primary profile to an unclaimed username; prior username freed for reclaim"
    print_run_summary_footer
}

run_username_admin_flow() {
    SOCIAL_RUN_ID="$(date +%s)"
    ensure_username_admin_session || return 1

    NEW_USERNAME=''
    PRIMARY_PRIOR_USERNAME=''
    ADMIN_REASSIGN_TX=''

    step_ensure_primary_profile || return 1
    step_admin_reassign_username || return 1
    assert_reassign_indexed || return 1
    save_username_admin_session
    print_username_admin_summary
}

show_menu() {
    echo ""
    echo "=== Username Admin Cap E2E Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Run rename flow (--run-all)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) refresh_username_admin_session_from_graphql; load_username_admin_session ;;
        1) run_username_admin_flow ;;
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
            --refresh-session) RUN_MODE=refresh; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    load_username_admin_session

    case "${RUN_MODE:-}" in
        refresh)
            refresh_username_admin_session_from_graphql || exit 1
            load_username_admin_session
            exit 0
            ;;
        run_all) run_username_admin_flow; exit 0 ;;
        '')
            if [[ ! -t 0 ]]; then
                echo "No TTY — use: ASSUME_YES=1 ./scripts/username-admin-runnable.sh --run-all" >&2
                exit 1
            fi
            show_menu
            ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
