#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Canonical E2E integration test for profile subscriptions (subscription.move)
# and subscriber-only post access (post.move gating).
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126
#   - `myso`, `curl`, `jq` on PATH
#
# Session: network.config/subscription/subscription-session.env
#
# Usage:
#   ./scripts/subscriptions-test.sh --refresh-session
#   ASSUME_YES=1 ./scripts/subscriptions-test.sh --run-all
#   ./scripts/subscriptions-test.sh --lenient-offchain   # debug: skip REST/GQL hard fails
#   ./scripts/subscriptions-test.sh --with-encrypted-post  # optional key-server subflow

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/subscription/subscription-session.env"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/subscription-test-common.sh
source "${SCRIPT_DIR}/lib/subscription-test-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

readonly MONTHLY_FEE='1000000000'
readonly SHORT_BILLING_PERIOD_MS='5000'
readonly DEFAULT_BILLING_PERIOD_MS='2592000000'
readonly RENEWAL_MONTHS='0'

SOCIAL_RUN_ID="$(date +%s)"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"
LENIENT_OFFCHAIN="${LENIENT_OFFCHAIN:-0}"
WITH_ENCRYPTED_POST="${WITH_ENCRYPTED_POST:-0}"

CREATOR_ADDRESS=''
SUBSCRIBER_ADDRESS=''
NONSUB_ADDRESS=''
CREATOR_PROFILE_ID=''
MEMORY_ACCOUNT_ID=''
SERVICE_ID=''
SUBSCRIPTION_ID=''
POST_ID=''
ORIGINAL_BILLING_PERIOD_MS=''
PAY_COIN_ID=''
TOKEN_REGISTRY_ID=''
SOCIAL_PROOF_TOKENS_CONFIG_ID=''

declare -a RUN_RESULTS=()

SUBSCRIPTION_SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID
    MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID
    PLATFORM_OBJECT_ID PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID
    MYDATA_REGISTRY_ID SUBSCRIPTION_CONFIG_ID SUBSCRIPTION_ADMIN_CAP_ID
    CREATOR_ADDRESS SUBSCRIBER_ADDRESS NONSUB_ADDRESS CREATOR_PROFILE_ID
    MEMORY_ACCOUNT_ID SERVICE_ID SUBSCRIPTION_ID POST_ID PAY_COIN_ID
    TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID
    ORIGINAL_BILLING_PERIOD_MS
)

usage() {
    sed -n '2,22p' "$0" | sed 's/^# \?//'
}

save_subscription_session() {
    social_save_session "${SUBSCRIPTION_SESSION_KEYS[@]}"
}

load_subscription_session() {
    social_load_session
}

record_result() {
    local name="$1" status="$2"
    RUN_RESULTS+=("${name}:${status}")
}

verify_offchain_or_lenient() {
    if [[ "$LENIENT_OFFCHAIN" == 1 ]]; then
        echo "  (lenient-offchain: skipping hard off-chain check)" >&2
        return 0
    fi
    "$@"
}

ensure_creator_wallet() {
    CREATOR_ADDRESS="$(resolve_myso_active_address)" || return 1
    CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")"
    ensure_wallet_funded "$CREATOR_ADDRESS" "$((MONTHLY_FEE * 4 + SOCIAL_DEFAULT_GAS_BUDGET * 10))" || return 1
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
}

ensure_subscriber_wallet() {
    if [[ -z "${SUBSCRIBER_ADDRESS:-}" ]]; then
        SUBSCRIBER_ADDRESS="$(create_ephemeral_wallet "sub_subscriber_${SOCIAL_RUN_ID}")" || return 1
    fi
    SUBSCRIBER_ADDRESS="$(normalize_hex_id "$SUBSCRIBER_ADDRESS")"
    ensure_wallet_funded "$SUBSCRIBER_ADDRESS" "$((MONTHLY_FEE * 3 + SOCIAL_DEFAULT_GAS_BUDGET * 10))" || return 1
    log_session_use "SUBSCRIBER_ADDRESS" "$SUBSCRIBER_ADDRESS"
}

ensure_nonsub_wallet() {
    if [[ -z "${NONSUB_ADDRESS:-}" ]]; then
        NONSUB_ADDRESS="$(create_ephemeral_wallet "sub_nonsub_${SOCIAL_RUN_ID}")" || return 1
    fi
    NONSUB_ADDRESS="$(normalize_hex_id "$NONSUB_ADDRESS")"
    ensure_wallet_funded "$NONSUB_ADDRESS" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    log_session_use "NONSUB_ADDRESS" "$NONSUB_ADDRESS"
}

ensure_creator_profile() {
    CREATOR_PROFILE_ID="$(resolve_owned_profile_for_address "$CREATOR_ADDRESS")" || {
        echo "Creator profile not found; run bootstrap / profile creation first" >&2
        return 1
    }
    log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
}

ensure_memory_account() {
    local mem
    if [[ -n "${MEMORY_ACCOUNT_ID:-}" ]] && object_exists_on_fullnode "$MEMORY_ACCOUNT_ID"; then
        MEMORY_ACCOUNT_ID="$(normalize_hex_id "$MEMORY_ACCOUNT_ID")"
        return 0
    fi
    mem="$(resolve_memory_account_for_address "$CREATOR_ADDRESS")" || true
    [[ -n "$mem" ]] || { echo "MemoryAccount required for create_post" >&2; return 1; }
    MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
}

ensure_spt_objects_for_post() {
    if [[ -n "${TOKEN_REGISTRY_ID:-}" && -n "${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}" ]]; then
        require_hex_ids TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
        return 0
    fi
    local json
    log_step "Fetching TokenRegistry + SocialProofTokensConfig from GraphQL"
    json="$(graphql_post 'query SubscriptionPostSptObjects {
  socialProofTokenRegistry: objects(filter: { type: "0x50c1::social_proof_tokens::TokenRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  sptConfig: objects(filter: { type: "0x50c1::social_proof_tokens::SocialProofTokensConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
}')" || return 1
    TOKEN_REGISTRY_ID="$(gql_object_address "$json" socialProofTokenRegistry)"
    SOCIAL_PROOF_TOKENS_CONFIG_ID="$(gql_object_address "$json" sptConfig)"
    require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
    require_hex_ids TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
    log_session_use "TOKEN_REGISTRY_ID" "$TOKEN_REGISTRY_ID"
    log_session_use "SOCIAL_PROOF_TOKENS_CONFIG_ID" "$SOCIAL_PROOF_TOKENS_CONFIG_ID"
}

flow_create_service() {
    local out digest
    subscription_require_session_objects || return 1
    ensure_creator_profile || return 1
    log_step "Creating profile subscription service (fee=$MONTHLY_FEE)"
    out="$(run_myso_call_as_capture "$CREATOR_ADDRESS" subscription create_profile_service_entry \
        "@$(normalize_hex_id "$CREATOR_PROFILE_ID")" "$MONTHLY_FEE" "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || { echo "create_profile_service_entry failed" >&2; return 1; }
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "ProfileSubscriptionServiceCreatedEvent" || return 1
    SERVICE_ID="$(extract_created_object_by_type "$digest" "subscription::ProfileSubscriptionService")" || return 1
    log_session_use "SERVICE_ID" "$SERVICE_ID"
    save_subscription_session
    record_result "create_service" "pass"
}

flow_subscribe() {
    local out digest coin amount="$((MONTHLY_FEE * 2))"
    subscription_require_session_objects SERVICE_ID || return 1
    ensure_subscriber_wallet || return 1
    coin="$(pick_split_coin_for_amount "$SUBSCRIBER_ADDRESS" "$amount")" || return 1
    log_step "Subscribing to profile service $SERVICE_ID"
    out="$(run_myso_call_as_capture "$SUBSCRIBER_ADDRESS" subscription subscribe_to_profile \
        "@$(normalize_hex_id "$SUBSCRIPTION_CONFIG_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$ECOSYSTEM_TREASURY_ID")" \
        "$coin" false "$RENEWAL_MONTHS" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "ProfileSubscriptionCreatedEvent" || return 1
    SUBSCRIPTION_ID="$(extract_created_object_by_type "$digest" "subscription::ProfileSubscription")" || return 1
    log_session_use "SUBSCRIPTION_ID" "$SUBSCRIPTION_ID"
    verify_offchain_or_lenient verify_subscription_layers \
        "$SUBSCRIBER_ADDRESS" "$SERVICE_ID" true "$SUBSCRIPTION_ID" || return 1
    save_subscription_session
    record_result "subscribe" "pass"
}

flow_create_gated_post() {
    local out digest body_lit ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    subscription_require_session_objects || return 1
    ensure_memory_account || return 1
    body_lit="$(literal_move_string "Subscription gated post ${SOCIAL_RUN_ID}")"
    ref_ur="$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" || return 1
    ref_pr="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || return 1
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || return 1
    ref_cfg="$(ptb_shared_ref "$POST_CONFIG_ID")" || return 1
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || return 1
    ref_mr="$(ptb_shared_ref "$MYDATA_REGISTRY_ID")" || return 1
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    log_step "Creating post for subscription gate"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::create_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" none none none none none none none \
        none none none \
        "$ref_mr" "$ref_mem" "$ref_clk")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    POST_ID="$(extract_created_object_by_type "$digest" "post::Post")" || return 1
    log_session_use "POST_ID" "$POST_ID"
    log_step "Enabling post subscription gate"
    out="$(run_myso_call_as_capture "$CREATOR_ADDRESS" post enable_post_subscription_gate \
        "@$(normalize_hex_id "$POST_ID")" "@$(normalize_hex_id "$SERVICE_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "PostSubscriptionGateEnabledEvent" || return 1
    save_subscription_session
    record_result "post_gate" "pass"
}

flow_subscriber_view_post() {
    local out
    subscription_require_session_objects POST_ID SERVICE_ID SUBSCRIPTION_ID || return 1
    log_step "Subscriber views gated post (assert_can_view_post)"
    out="$(run_myso_call_as_capture "$SUBSCRIBER_ADDRESS" post assert_can_view_post \
        "@$(normalize_hex_id "$POST_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    log_step "Recording post subscription view"
    out="$(run_myso_call_as_capture "$SUBSCRIBER_ADDRESS" post record_post_subscription_view \
        "@$(normalize_hex_id "$POST_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    record_result "subscriber_view" "pass"
}

flow_nonsub_denied() {
    local out
    subscription_require_session_objects POST_ID SERVICE_ID SUBSCRIPTION_ID || return 1
    ensure_nonsub_wallet || return 1
    log_step "Non-subscriber denied post view (dry-run)"
    out="$(DRY_RUN=1 run_myso_call_as_capture "$NONSUB_ADDRESS" post assert_can_view_post \
        "@$(normalize_hex_id "$POST_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || true
    assert_tx_aborts "$out" || { echo "Expected non-subscriber assert_can_view_post to abort" >&2; return 1; }
    verify_offchain_or_lenient wait_for_rest_subscription_access "$NONSUB_ADDRESS" "$SERVICE_ID" false || return 1
    record_result "nonsub_denied" "pass"
}

flow_cancel_subscription() {
    local out digest
    subscription_require_session_objects SUBSCRIPTION_ID SERVICE_ID || return 1
    log_step "Cancelling subscription $SUBSCRIPTION_ID"
    out="$(run_myso_call_as_capture "$SUBSCRIBER_ADDRESS" subscription cancel_subscription \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "ProfileSubscriptionCancelledEvent" || return 1
    verify_offchain_or_lenient verify_subscription_layers "$SUBSCRIBER_ADDRESS" "$SERVICE_ID" false || return 1
    record_result "cancel" "pass"
}

admin_set_billing_period() {
    local period_ms="$1"
    subscription_require_session_objects SUBSCRIPTION_ADMIN_CAP_ID || return 1
    local out
    log_step "Admin set billing_period_ms=$period_ms"
    out="$(run_myso_call_as_capture "$CREATOR_ADDRESS" subscription update_subscription_config \
        "@$(normalize_hex_id "$SUBSCRIPTION_ADMIN_CAP_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_CONFIG_ID")" \
        "$period_ms" 12 250 250 0 10000 \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
}

restore_billing_period() {
    local target="${ORIGINAL_BILLING_PERIOD_MS:-$DEFAULT_BILLING_PERIOD_MS}"
    admin_set_billing_period "$target" || echo "Warning: failed to restore billing period" >&2
}

flow_renew_subscription() {
    local out digest coin
    subscription_require_session_objects SUBSCRIPTION_ID SERVICE_ID || return 1
    coin="$(pick_split_coin_for_amount "$SUBSCRIBER_ADDRESS" "$MONTHLY_FEE")" || return 1
    log_step "Manual renew subscription $SUBSCRIPTION_ID"
    out="$(run_myso_call_as_capture "$SUBSCRIBER_ADDRESS" subscription renew_subscription \
        "@$(normalize_hex_id "$SUBSCRIPTION_CONFIG_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" \
        "@$(normalize_hex_id "$ECOSYSTEM_TREASURY_ID")" \
        "$coin" "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "ProfileSubscriptionRenewedEvent" || return 1
    record_result "renew" "pass"
}

flow_fund_renewal_balance() {
    local out digest coin amount="$MONTHLY_FEE"
    subscription_require_session_objects SUBSCRIPTION_ID || return 1
    coin="$(pick_split_coin_for_amount "$SUBSCRIBER_ADDRESS" "$amount")" || return 1
    log_step "Fund renewal balance on $SUBSCRIPTION_ID"
    out="$(run_myso_call_as_capture "$SUBSCRIBER_ADDRESS" subscription fund_renewal_balance \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" "$coin" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "RenewalBalanceFundedEvent" || return 1
    record_result "fund_balance" "pass"
}

flow_short_billing_expiry() {
    local out digest coin expiry_sub=''
    subscription_require_session_objects SERVICE_ID POST_ID || return 1
    ensure_nonsub_wallet || return 1
    ORIGINAL_BILLING_PERIOD_MS="${ORIGINAL_BILLING_PERIOD_MS:-$DEFAULT_BILLING_PERIOD_MS}"
    admin_set_billing_period "$SHORT_BILLING_PERIOD_MS" || return 1
    coin="$(pick_split_coin_for_amount "$NONSUB_ADDRESS" "$((MONTHLY_FEE * 2))")" || return 1
    log_step "Subscribe (short billing) for expiry test subscriber $NONSUB_ADDRESS"
    out="$(run_myso_call_as_capture "$NONSUB_ADDRESS" subscription subscribe_to_profile \
        "@$(normalize_hex_id "$SUBSCRIPTION_CONFIG_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$ECOSYSTEM_TREASURY_ID")" \
        "$coin" false "$RENEWAL_MONTHS" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    expiry_sub="$(extract_created_object_by_type "$digest" "subscription::ProfileSubscription")" || return 1
    log_step "Waiting for short billing expiry"
    sleep $((SHORT_BILLING_PERIOD_MS / 1000 + 2))
    verify_offchain_or_lenient verify_subscription_layers "$NONSUB_ADDRESS" "$SERVICE_ID" false || return 1
    out="$(DRY_RUN=1 run_myso_call_as_capture "$NONSUB_ADDRESS" post assert_can_view_post \
        "@$(normalize_hex_id "$POST_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$expiry_sub")" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || true
    assert_tx_aborts "$out" || return 1
    coin="$(pick_split_coin_for_amount "$NONSUB_ADDRESS" "$MONTHLY_FEE")" || return 1
    out="$(run_myso_call_as_capture "$NONSUB_ADDRESS" subscription renew_subscription \
        "@$(normalize_hex_id "$SUBSCRIPTION_CONFIG_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$expiry_sub")" \
        "@$(normalize_hex_id "$ECOSYSTEM_TREASURY_ID")" \
        "$coin" "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    verify_offchain_or_lenient verify_subscription_layers "$NONSUB_ADDRESS" "$SERVICE_ID" true "$expiry_sub" || return 1
    record_result "expiry_renew" "pass"
}

flow_platform_subscribe() {
    [[ -n "${PLATFORM_OBJECT_ID:-}" ]] || { echo "PLATFORM_OBJECT_ID not set; skipping platform path" >&2; return 0; }
    local out digest coin amount="$((MONTHLY_FEE * 2))" platform_sub
    subscription_require_session_objects SERVICE_ID || return 1
    platform_sub="$(create_ephemeral_wallet "sub_platform_${SOCIAL_RUN_ID}")" || return 1
    platform_sub="$(normalize_hex_id "$platform_sub")"
    ensure_wallet_funded "$platform_sub" "$((amount + SOCIAL_DEFAULT_GAS_BUDGET * 5))" || return 1
    coin="$(pick_split_coin_for_amount "$platform_sub" "$amount")" || return 1
    log_step "Platform-path subscribe for $platform_sub"
    out="$(run_myso_call_as_capture "$platform_sub" subscription subscribe_to_profile_with_platform \
        "@$(normalize_hex_id "$SUBSCRIPTION_CONFIG_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$ECOSYSTEM_TREASURY_ID")" \
        "@$(normalize_hex_id "$PLATFORM_OBJECT_ID")" \
        "$coin" false "$RENEWAL_MONTHS" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "ProfileSubscriptionCreatedEvent" || return 1
    record_result "platform_subscribe" "pass"
}

flow_update_service_fee() {
    local out new_fee="$((MONTHLY_FEE * 2))"
    subscription_require_session_objects SERVICE_ID || return 1
    log_step "Update service fee to $new_fee"
    out="$(run_myso_call_as_capture "$CREATOR_ADDRESS" subscription update_service_fee \
        "@$(normalize_hex_id "$SERVICE_ID")" "$new_fee")" || return 1
    assert_tx_success "$out" || return 1
    record_result "update_fee" "pass"
}

flow_negative_smoke() {
    local out
    subscription_require_session_objects SERVICE_ID SUBSCRIPTION_ID POST_ID || return 1
    ensure_nonsub_wallet || return 1
    log_step "N21-ish: non-owner cancel should abort"
    out="$(DRY_RUN=1 run_myso_call_as_capture "$NONSUB_ADDRESS" subscription cancel_subscription \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")")" || true
    assert_tx_aborts "$out" || return 1
    log_step "N24: view post with wrong subscription context (non-subscriber)"
    out="$(DRY_RUN=1 run_myso_call_as_capture "$NONSUB_ADDRESS" post assert_can_view_post \
        "@$(normalize_hex_id "$POST_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || true
    assert_tx_aborts "$out" || return 1
    log_step "N22: enable gate with wrong service owner should abort"
    if [[ -n "${NONSUB_ADDRESS:-}" ]]; then
        out="$(DRY_RUN=1 run_myso_call_as_capture "$NONSUB_ADDRESS" post enable_post_subscription_gate \
            "@$(normalize_hex_id "$POST_ID")" "@$(normalize_hex_id "$SERVICE_ID")")" || true
        assert_tx_aborts "$out" || return 1
    fi
    record_result "negative_smoke" "pass"
}

flow_run_all() {
    trap restore_billing_period EXIT
    flow_create_service || return 1
    flow_subscribe || return 1
    flow_renew_subscription || return 1
    flow_fund_renewal_balance || return 1
    flow_create_gated_post || return 1
    flow_nonsub_denied || return 1
    flow_subscriber_view_post || return 1
    flow_platform_subscribe || true
    flow_short_billing_expiry || return 1
    flow_update_service_fee || return 1
    flow_negative_smoke || return 1
    flow_cancel_subscription || return 1
    print_summary
}

flow_run_core() {
    trap restore_billing_period EXIT
    flow_create_service || return 1
    flow_subscribe || return 1
    flow_create_gated_post || return 1
    flow_nonsub_denied || return 1
    flow_subscriber_view_post || return 1
    flow_cancel_subscription || return 1
    print_summary
}

print_summary() {
    print_run_summary_header "Profile Subscription E2E"
    print_run_summary_line "Creator" "$CREATOR_ADDRESS"
    print_run_summary_line "Subscriber" "$SUBSCRIBER_ADDRESS"
    print_run_summary_line "Service" "${SERVICE_ID:-n/a}"
    print_run_summary_line "Subscription" "${SUBSCRIPTION_ID:-n/a}"
    print_run_summary_line "Post" "${POST_ID:-n/a}"
    echo "  Results:" >&2
    local row
    for row in "${RUN_RESULTS[@]}"; do
        printf '    %s\n' "$row" >&2
    done
    print_run_summary_footer
}

interactive_menu() {
    echo "Profile Subscription E2E — menu"
    echo "  1) Refresh session from GraphQL"
    echo "  2) Create subscription service"
    echo "  3) Subscribe"
    echo "  4) Create gated post + enable gate"
    echo "  5) Subscriber view post"
    echo "  6) Non-subscriber denied"
    echo "  7) Cancel subscription"
    echo "  8) Run all (--run-all)"
    echo "  9) Run core only (--run-core)"
    echo "  10) Negative smoke tests"
    echo "  q) Quit"
    read -r -p 'Choice: ' choice
    case "$choice" in
        1) subscription_refresh_session ;;
        2) flow_create_service ;;
        3) flow_subscribe ;;
        4) flow_create_gated_post ;;
        5) flow_subscriber_view_post ;;
        6) flow_nonsub_denied ;;
        7) flow_cancel_subscription ;;
        8) flow_run_all ;;
        9) flow_run_core ;;
        10) flow_negative_smoke ;;
        q|Q) exit 0 ;;
        *) echo "Unknown choice" >&2 ;;
    esac
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --refresh-session) RUN_MODE=refresh ;;
            --run-all) RUN_MODE=run_all ;;
            --run-core) RUN_MODE=run_core ;;
            --lenient-offchain) LENIENT_OFFCHAIN=1 ;;
            --with-encrypted-post) WITH_ENCRYPTED_POST=1 ;;
            -h|--help) usage; exit 0 ;;
            *) echo "Unknown arg: $1" >&2; usage; exit 1 ;;
        esac
        shift
    done
}

main() {
    parse_args "$@"
    mkdir -p "$(dirname "$SOCIAL_SESSION_SAVE_PATH")"
    load_subscription_session 2>/dev/null || true
    social_apply_defaults
    ensure_creator_wallet || exit 1

    case "${RUN_MODE:-}" in
        refresh) subscription_refresh_session ;;
        run_all) flow_run_all ;;
        run_core) flow_run_core ;;
        '')
            if [[ "${ASSUME_YES:-0}" == 1 ]]; then
                flow_run_all
            else
                while true; do interactive_menu; done
            fi
            ;;
    esac
}

main "$@"
