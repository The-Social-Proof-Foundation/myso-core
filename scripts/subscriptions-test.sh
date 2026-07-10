#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Canonical E2E integration test for profile subscriptions (subscription.move)
# and subscriber-only post access (post.move gating).
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - Local stack: myso start --with-faucet --with-indexer=postgres://... \
#       --with-social-indexer --with-graphql --with-mydata --with-messaging
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126
#   - Key-server secrets: network.config/mydata/local-mydata-secrets.env
#   - `myso`, `mydata`, `curl`, `jq` on PATH
#
# Session: network.config/subscription/subscription-session.env
#
# Usage:
#   ./scripts/subscriptions-test.sh --refresh-session
#   ASSUME_YES=1 ./scripts/subscriptions-test.sh --run-all
#   ./scripts/subscriptions-test.sh --lenient-offchain   # debug: skip REST/GQL hard fails
#   ./scripts/subscriptions-test.sh --with-encrypted-post  # optional key-server subflow
#   SUBSCRIPTION_USE_MARKETPLACE_SELLER=1 ./scripts/subscriptions-test.sh  # reuse marketplace seller profile
#   ./scripts/subscriptions-test.sh --run-menu-all          # menus 1-7,10-13 (one subscribe per phase)
#   ./scripts/subscriptions-test.sh --run-all               # menu 8 integration runner
#   ./scripts/subscriptions-test.sh --run-core              # menu 9 core runner
#   GQL_WAIT_MAX=30 ./scripts/subscriptions-test.sh --run-menu-all  # tune indexer poll budget

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/subscription/subscription-session.env"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/subscription-test-common.sh
source "${SCRIPT_DIR}/lib/subscription-test-common.sh"
# shellcheck source=lib/mydata-test-common.sh
source "${SCRIPT_DIR}/lib/mydata-test-common.sh"
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
SUBSCRIPTION_ACTIVE='0'
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
    MYDATA_REGISTRY_ID MYDATA_CONFIG_ID MYDATA_ADMIN_CAP_ID
    SUBSCRIPTION_CONFIG_ID SUBSCRIPTION_ADMIN_CAP_ID
    KEY_SERVER_URL PUBLIC_KEY KEY_SERVER_OBJECT_ID MYDATA_SECRETS_FILE
    CREATOR_ADDRESS SUBSCRIBER_ADDRESS NONSUB_ADDRESS CREATOR_PROFILE_ID
    MEMORY_ACCOUNT_ID SERVICE_ID SUBSCRIPTION_ID SUBSCRIPTION_ACTIVE POST_ID PAY_COIN_ID
    TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID
    MYDATA_ID ENCRYPTION_ID_HEX ENCRYPT_CIPHERTEXT_HEX ENCRYPTED_PLAINTEXT_EXPECTED
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
    if [[ -z "${CREATOR_ADDRESS:-}" ]]; then
        CREATOR_ADDRESS="$(resolve_myso_active_address)" || return 1
        CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")"
        log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
    else
        CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")" || return 1
    fi
    ensure_wallet_funded "$CREATOR_ADDRESS" "$((MONTHLY_FEE * 4 + SOCIAL_DEFAULT_GAS_BUDGET * 10))" || return 1
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
    local lines profile_id_existing snap username mem active_addr
    if [[ "${SUBSCRIPTION_USE_MARKETPLACE_SELLER:-0}" == 1 ]]; then
        subscription_import_creator_from_marketplace || return 1
    fi
    if [[ -n "${CREATOR_PROFILE_ID:-}" ]] && object_exists_on_fullnode "$CREATOR_PROFILE_ID"; then
        CREATOR_PROFILE_ID="$(normalize_hex_id "$CREATOR_PROFILE_ID")"
        subscription_sync_creator_address_from_profile "$CREATOR_PROFILE_ID" || return 1
        ensure_wallet_funded "$CREATOR_ADDRESS" "$((MONTHLY_FEE * 4 + SOCIAL_DEFAULT_GAS_BUDGET * 10))" || return 1
        snap="$(gql_profile_snapshot "$CREATOR_ADDRESS" 2>/dev/null)" || snap='{}'
        mem="$(echo "$snap" | jq -r '.data.profile.memoryAccountId // empty')"
        [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
        log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
        return 0
    fi
    profile_id_existing="$(resolve_owned_profile_for_address "$CREATOR_ADDRESS")" || profile_id_existing=''
    if [[ -n "$profile_id_existing" ]]; then
        CREATOR_PROFILE_ID="$(normalize_hex_id "$profile_id_existing")"
        snap="$(gql_profile_snapshot "$CREATOR_ADDRESS" 2>/dev/null)" || snap='{}'
        mem="$(echo "$snap" | jq -r '.data.profile.memoryAccountId // empty')"
        [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
        log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
        active_addr="$(resolve_myso_active_address 2>/dev/null)" || active_addr=''
        if [[ -n "$active_addr" && "$(normalize_hex_id "$active_addr")" != "$(normalize_hex_id "$CREATOR_ADDRESS")" ]]; then
            log_step "Subscription creator profile is on $CREATOR_ADDRESS (myso active wallet is $(normalize_hex_id "$active_addr")). For marketplace seller profile: SUBSCRIPTION_USE_MARKETPLACE_SELLER=1"
        fi
        return 0
    fi
    active_addr="$(resolve_myso_active_address 2>/dev/null)" || active_addr=''
    if [[ -f "$(subscription_marketplace_session_path)" && "${SUBSCRIPTION_USE_MARKETPLACE_SELLER:-0}" != 1 ]]; then
        log_step "No profile for $CREATOR_ADDRESS. Username marketplace profiles live on other wallets — set SUBSCRIPTION_USE_MARKETPLACE_SELLER=1 to reuse seller profile."
    fi
    log_step "No profile for wallet $CREATOR_ADDRESS — creating subscription test profile"
    username="subcreator${SOCIAL_RUN_ID}"
    lines="$(create_profile_for_address "$CREATOR_ADDRESS" "Subscription Creator ${SOCIAL_RUN_ID}" "$username")" || return 1
    CREATOR_PROFILE_ID="$(normalize_hex_id "$(echo "$lines" | sed -n '1p')")"
    mem="$(echo "$lines" | sed -n '2p')"
    [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
    [[ -n "${MEMORY_ACCOUNT_ID:-}" ]] && log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
    save_subscription_session
}

ensure_memory_account() {
    local mem snap
    if [[ -n "${MEMORY_ACCOUNT_ID:-}" ]] && object_exists_on_fullnode "$MEMORY_ACCOUNT_ID"; then
        MEMORY_ACCOUNT_ID="$(normalize_hex_id "$MEMORY_ACCOUNT_ID")"
        return 0
    fi
    snap="$(gql_profile_snapshot "$CREATOR_ADDRESS" 2>/dev/null)" || snap='{}'
    mem="$(echo "$snap" | jq -r '.data.profile.memoryAccountId // empty')"
    if [[ -n "$mem" ]]; then
        mem="$(normalize_hex_id "$mem")" || mem=''
        if [[ -n "$mem" ]] && object_exists_on_fullnode "$mem"; then
            MEMORY_ACCOUNT_ID="$mem"
            log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
            return 0
        fi
    fi
    echo "MemoryAccount required for create_post (creator=$CREATOR_ADDRESS)" >&2
    return 1
}

ensure_subscription_platform() {
    ensure_creator_profile || return 1
    if [[ -n "${PLATFORM_OBJECT_ID:-}" ]] && object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
        ensure_joined_platform "$CREATOR_ADDRESS" || return 1
        return 0
    fi
    log_step "No live PLATFORM_OBJECT_ID on fullnode — creating test platform"
    create_test_platform || return 1
    ensure_joined_platform "$CREATOR_ADDRESS" || return 1
    save_subscription_session
}

flow_refresh_session() {
    social_refresh_session_from_graphql || return 1
    subscription_load_mydata_secrets || true
    save_subscription_session
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
    local out digest existing_svc svc_owner
    subscription_require_session_objects || return 1
    ensure_creator_profile || return 1
    existing_svc="$(subscription_resolve_existing_service_for_profile "$CREATOR_PROFILE_ID" 2>/dev/null)" || existing_svc=''
    if [[ -n "$existing_svc" ]]; then
        SERVICE_ID="$(normalize_hex_id "$existing_svc")"
        log_step "Found on-chain subscription service for profile $CREATOR_PROFILE_ID: $SERVICE_ID"
        log_session_use "SERVICE_ID" "$SERVICE_ID"
        save_subscription_session
        record_result "create_service" "pass"
        return 0
    fi
    if [[ -n "${SERVICE_ID:-}" ]] && object_exists_on_fullnode "$SERVICE_ID"; then
        if gql_subscription_service_matches_profile "$SERVICE_ID" "$CREATOR_PROFILE_ID"; then
            log_step "Reusing existing subscription service $SERVICE_ID"
            log_session_use "SERVICE_ID" "$(normalize_hex_id "$SERVICE_ID")"
            record_result "create_service" "pass"
            return 0
        fi
        log_step "Ignoring stale SERVICE_ID $SERVICE_ID (not indexed for profile $CREATOR_PROFILE_ID)"
        SERVICE_ID=''
    fi
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
    ensure_default_billing_period || return 1
    ensure_subscriber_wallet || return 1
    if subscription_reuse_active_if_present "$SUBSCRIBER_ADDRESS"; then
        save_subscription_session
        record_result "subscribe" "reused"
        return 0
    fi
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
    SUBSCRIPTION_ACTIVE=1
    log_session_use "SUBSCRIPTION_ID" "$SUBSCRIPTION_ID"
    log_session_use "SUBSCRIPTION_ACTIVE" "$SUBSCRIPTION_ACTIVE"
    save_subscription_session
    record_result "subscribe" "pass"
}

flow_create_gated_post() {
    local mode="${1:-plaintext}"
    if [[ "$mode" == "encrypted" ]]; then
        flow_create_encrypted_gated_post
        return $?
    fi
    local out digest body_lit ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    subscription_require_session_objects || return 1
    ensure_subscription_platform || return 1
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
        "$body_lit" none \
        none none none none none none none \
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

flow_create_encrypted_gated_post() {
    local out digest body_lit ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    local mydata_arg plaintext teaser
    subscription_require_mydata_stack || return 1
    subscription_require_session_objects SERVICE_ID || return 1
    ensure_subscription_platform || return 1
    ensure_memory_account || return 1
    plaintext="[PRIVATE] Full subscriber-only body for run ${SOCIAL_RUN_ID}. Secret payload onlyy Brandonnn can see and is not stored in posts.content."
    teaser="[PUBLIC TEASER] Subscribe to unlock the encrypted post (run ${SOCIAL_RUN_ID})."
    log_step "Encrypted body (MyData): $plaintext"
    log_step "Public teaser (Post.content): $teaser"
    mydata_create_and_share_encrypted "$CREATOR_ADDRESS" "$plaintext" || return 1
    log_session_use "ENCRYPT_CIPHERTEXT_HEX" "$ENCRYPT_CIPHERTEXT_HEX"
    log_session_use "ENCRYPTED_PLAINTEXT_EXPECTED" "$ENCRYPTED_PLAINTEXT_EXPECTED"
    body_lit="$(literal_move_string "$teaser")"
    mydata_arg="$(ptb_option_address_from_arg "some($MYDATA_ID)")"
    ref_ur="$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" || return 1
    ref_pr="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || return 1
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || return 1
    ref_cfg="$(ptb_shared_ref "$POST_CONFIG_ID")" || return 1
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || return 1
    ref_mr="$(ptb_shared_ref "$MYDATA_REGISTRY_ID")" || return 1
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    log_step "Creating encrypted post (mydata_id=$MYDATA_ID)"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::create_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" none \
        none none none none none none none \
        none none "$mydata_arg" \
        "$ref_mr" "$ref_mem" "$ref_clk")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    POST_ID="$(extract_created_object_by_type "$digest" "post::Post")" || return 1
    log_session_use "POST_ID" "$POST_ID"
    log_step "Enabling post subscription gate (encrypted)"
    out="$(run_myso_call_as_capture "$CREATOR_ADDRESS" post enable_post_subscription_gate \
        "@$(normalize_hex_id "$POST_ID")" "@$(normalize_hex_id "$SERVICE_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    tx_has_event_named "$digest" "PostSubscriptionGateEnabledEvent" || return 1
    save_subscription_session
    record_result "encrypted_post_gate" "pass"
    log_step "Subscriber decrypt preview"
    subscription_print_decrypted_post_body "$SUBSCRIBER_ADDRESS" || return 1
    record_result "encrypted_post_decrypt_preview" "pass"
}

flow_mydata_approve_profile_subscription_checks() {
    subscription_require_session_objects MYDATA_ID SERVICE_ID MEMORY_ACCOUNT_ID || return 1
    subscription_ensure_active_subscription flow_subscribe || return 1
    [[ -n "${ENCRYPTION_ID_HEX:-}" ]] || {
        echo "Run encrypted post flow first (ENCRYPTION_ID_HEX missing)" >&2
        return 1
    }
    log_step "Policy dry-run: subscriber should succeed"
    subscription_dry_run_mydata_policy "$SUBSCRIBER_ADDRESS" || return 1
    ensure_nonsub_wallet || return 1
    log_step "Policy dry-run: non-subscriber should abort"
    local out
    out="$(DRY_RUN=1 SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$NONSUB_ADDRESS" \
        --move-call "${PKG_SOCIAL}::mydata::mydata_approve_profile_subscription" \
        "$(literal_move_vector_u8_from_hex "$ENCRYPTION_ID_HEX")" \
        "@$(normalize_hex_id "$MEMORY_CONFIG_ID")" \
        "@$(normalize_hex_id "$MYDATA_ID")" \
        "@$(normalize_hex_id "$MEMORY_ACCOUNT_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" \
        "@$(normalize_hex_id "$CLOCK_ID")" 2>&1)" || true
    assert_tx_aborts "$out" || return 1
    log_step "Policy checks passed — decrypting subscriber body"
    subscription_print_decrypted_post_body "$SUBSCRIBER_ADDRESS" || return 1
    record_result "mydata_policy_checks" "pass"
}

flow_subscriber_decrypt_encrypted_post() {
    subscription_require_session_objects POST_ID SERVICE_ID MYDATA_ID || return 1
    subscription_print_decrypted_post_body "$SUBSCRIBER_ADDRESS" || return 1
    ensure_nonsub_wallet || return 1
    log_step "Non-subscriber decrypt should fail"
    if subscription_subscriber_decrypt_encrypted_post "$NONSUB_ADDRESS" >/dev/null 2>&1; then
        echo "Expected non-subscriber decrypt to fail" >&2
        return 1
    fi
    record_result "subscriber_decrypt" "pass"
}

flow_post_cancel_decrypt_denied() {
    [[ "${WITH_ENCRYPTED_POST:-0}" == 1 && -n "${ENCRYPT_CIPHERTEXT_HEX:-}" ]] || return 0
    log_step "Post-cancel decrypt should fail"
    if subscription_subscriber_decrypt_encrypted_post "$SUBSCRIBER_ADDRESS" >/dev/null 2>&1; then
        echo "Expected post-cancel decrypt to fail" >&2
        return 1
    fi
    record_result "post_cancel_decrypt_denied" "pass"
}

flow_subscriber_view_post() {
    local out ref_post ref_svc ref_sub ref_clk
    subscription_require_session_objects POST_ID SERVICE_ID SUBSCRIPTION_ID || return 1
    [[ "${SUBSCRIPTION_ACTIVE:-0}" == 1 ]] || {
        echo "Expected active subscription from menu 3 (SUBSCRIPTION_ACTIVE=1)" >&2
        return 1
    }
    subscription_on_chain_unexpired "$SUBSCRIPTION_ID" || {
        echo "Subscription $SUBSCRIPTION_ID expired on-chain (check billing_period_ms; prior short-billing test may have left 5s period)" >&2
        return 1
    }
    ref_post="$(ptb_shared_ref "$POST_ID")" || return 1
    ref_svc="$(ptb_shared_ref "$SERVICE_ID")" || return 1
    ref_sub="@$(normalize_hex_id "$SUBSCRIPTION_ID")"
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    log_step "Subscriber views gated post (assert + record in one PTB)"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$SUBSCRIBER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::assert_can_view_post" \
        "$ref_post" "$ref_svc" "$ref_sub" "$ref_clk" \
        --move-call "${PKG_SOCIAL}::post::record_post_subscription_view" \
        "$ref_post" "$ref_svc" "$ref_sub" "$ref_clk")" || return 1
    assert_tx_success "$out" || return 1
    record_result "subscriber_view" "pass"
}

flow_nonsub_denied() {
    local out
    subscription_require_session_objects POST_ID SERVICE_ID SUBSCRIPTION_ID || return 1
    [[ "${SUBSCRIPTION_ACTIVE:-0}" == 1 ]] || {
        echo "Expected active subscription from menu 3 (SUBSCRIPTION_ACTIVE=1)" >&2
        return 1
    }
    subscription_on_chain_unexpired "$SUBSCRIPTION_ID" || {
        echo "Subscription $SUBSCRIPTION_ID expired on-chain (check billing_period_ms; prior short-billing test may have left 5s period)" >&2
        return 1
    }
    ensure_nonsub_wallet || return 1
    log_step "Non-subscriber denied post view (dry-run)"
    out="$(DRY_RUN=1 run_myso_call_as_capture "$NONSUB_ADDRESS" post assert_can_view_post \
        "@$(normalize_hex_id "$POST_ID")" \
        "@$(normalize_hex_id "$SERVICE_ID")" \
        "@$(normalize_hex_id "$SUBSCRIPTION_ID")" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || true
    assert_tx_aborts "$out" || { echo "Expected non-subscriber assert_can_view_post to abort" >&2; return 1; }
    if ! wait_for_gql_subscription_access "$NONSUB_ADDRESS" "$SERVICE_ID" false; then
        echo "GQL subscriptionAccess expected=false for non-subscriber" >&2
        return 1
    fi
    if [[ "${VERIFY_REST_LAYERS:-0}" == 1 ]]; then
        wait_for_rest_subscription_access "$NONSUB_ADDRESS" "$SERVICE_ID" false || \
            echo "  (REST subscription-access lagged; GQL verified non-subscriber)" >&2
    fi
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
    SUBSCRIPTION_ACTIVE=0
    log_session_use "SUBSCRIPTION_ACTIVE" "$SUBSCRIPTION_ACTIVE"
    save_subscription_session
    record_result "cancel" "pass"
}

admin_set_billing_period() {
    local period_ms="$1" admin_addr
    subscription_require_session_objects SUBSCRIPTION_ADMIN_CAP_ID || return 1
    admin_addr="$(object_address_owner "$SUBSCRIPTION_ADMIN_CAP_ID" 2>/dev/null)" || admin_addr=''
    admin_addr="$(normalize_hex_id "${admin_addr:-$CREATOR_ADDRESS}")" || return 1
    ensure_wallet_funded "$admin_addr" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    local out
    log_step "Admin set billing_period_ms=$period_ms (admin=$admin_addr)"
    out="$(run_myso_call_as_capture "$admin_addr" subscription update_subscription_config \
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

ensure_default_billing_period() {
    admin_set_billing_period "$DEFAULT_BILLING_PERIOD_MS" || return 1
    ORIGINAL_BILLING_PERIOD_MS="$DEFAULT_BILLING_PERIOD_MS"
    log_session_use "ORIGINAL_BILLING_PERIOD_MS" "$ORIGINAL_BILLING_PERIOD_MS"
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
    subscription_require_session_objects SERVICE_ID POST_ID || return 1
    subscription_ensure_active_subscription flow_subscribe || return 1
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

flow_run_menu_all() {
    local step failed=0 started_ts step_ts elapsed
    local -a steps=(
        '1:flow_refresh_session'
        '2:flow_create_service'
        '3:flow_subscribe'
        '4:flow_create_gated_post'
        '5:flow_subscriber_view_post'
        '6:flow_nonsub_denied'
        '7:flow_cancel_subscription'
        '10:flow_negative_smoke'
        '11:flow_create_encrypted_gated_post'
        '12:flow_subscriber_decrypt_encrypted_post'
        '13:flow_mydata_approve_profile_subscription_checks'
    )
    export SKIP_CONFIRM_RUN=1
    export ASSUME_YES=1
    subscription_load_mydata_secrets || true
    ensure_default_billing_period || return 1
    started_ts="$SECONDS"
    log_step "Starting menu-all (${#steps[@]} steps; menus 8–9 are separate: --run-all / --run-core)"
    for step in "${steps[@]}"; do
        local num="${step%%:*}" fn="${step#*:}"
        step_ts="$SECONDS"
        log_step "════ Menu $num: $fn ════"
        if ! "$fn"; then
            elapsed=$((SECONDS - step_ts))
            echo "FAILED menu $num ($fn) after ${elapsed}s" >&2
            failed=1
            break
        fi
        elapsed=$((SECONDS - step_ts))
        log_step "Menu $num done (${elapsed}s)"
        save_subscription_session
    done
    elapsed=$((SECONDS - started_ts))
    if [[ "$failed" == 0 ]]; then
        print_run_summary_header "Profile Subscription E2E — menu 1-13"
        print_run_summary_line "Creator" "$CREATOR_ADDRESS"
        print_run_summary_line "Subscriber" "${SUBSCRIBER_ADDRESS:-n/a}"
        print_run_summary_line "Service" "${SERVICE_ID:-n/a}"
        print_run_summary_line "Subscription" "${SUBSCRIPTION_ID:-n/a}"
        print_run_summary_line "Post" "${POST_ID:-n/a}"
        print_run_summary_line "Elapsed" "${elapsed}s"
        print_run_summary_footer
    else
        echo "Menu-all aborted after ${elapsed}s" >&2
    fi
    return "$failed"
}

flow_run_all() {
    trap restore_billing_period EXIT
    ensure_default_billing_period || return 1
    flow_create_service || return 1
    flow_subscribe || return 1
    flow_renew_subscription || return 1
    flow_fund_renewal_balance || return 1
    if [[ "${WITH_ENCRYPTED_POST:-0}" == 1 ]]; then
        subscription_load_mydata_secrets || true
        flow_create_gated_post encrypted || return 1
    else
        flow_create_gated_post plaintext || return 1
    fi
    flow_nonsub_denied || return 1
    flow_subscriber_view_post || return 1
    if [[ "${WITH_ENCRYPTED_POST:-0}" == 1 ]]; then
        flow_mydata_approve_profile_subscription_checks || return 1
        flow_subscriber_decrypt_encrypted_post || return 1
    fi
    flow_platform_subscribe || true
    flow_short_billing_expiry || return 1
    flow_update_service_fee || return 1
    flow_negative_smoke || return 1
    flow_cancel_subscription || return 1
    if [[ "${WITH_ENCRYPTED_POST:-0}" == 1 ]]; then
        flow_post_cancel_decrypt_denied || return 1
    fi
    print_summary
}

flow_run_core() {
    trap restore_billing_period EXIT
    ensure_default_billing_period || return 1
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
    if [[ -n "${MYDATA_ID:-}" ]]; then
        print_run_summary_line "MyData" "$MYDATA_ID"
        print_run_summary_line "Key server" "${KEY_SERVER_URL:-n/a}"
    fi
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
    echo "  11) Create encrypted gated post + decrypt preview"
    echo "  12) Subscriber decrypt via key server (+ non-subscriber negative)"
    echo "  13) MyData policy dry-runs + subscriber decrypt"
    echo "  q) Quit"
    read -r -p 'Choice: ' choice
    case "$choice" in
        1) flow_refresh_session ;;
        2) flow_create_service ;;
        3) flow_subscribe ;;
        4) flow_create_gated_post ;;
        5) flow_subscriber_view_post ;;
        6) flow_nonsub_denied ;;
        7) flow_cancel_subscription ;;
        8) flow_run_all ;;
        9) flow_run_core ;;
        10) flow_negative_smoke ;;
        11) flow_create_encrypted_gated_post ;;
        12) flow_subscriber_decrypt_encrypted_post ;;
        13) flow_mydata_approve_profile_subscription_checks ;;
        q|Q) exit 0 ;;
        *) echo "Unknown choice" >&2 ;;
    esac
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --refresh-session) RUN_MODE=refresh ;;
            --run-menu-all)
                RUN_MODE=run_menu_all
                export SKIP_CONFIRM_RUN=1
                export ASSUME_YES=1
                ;;
            --run-all) RUN_MODE=run_all; export SKIP_CONFIRM_RUN=1; export ASSUME_YES=1 ;;
            --run-core) RUN_MODE=run_core; export SKIP_CONFIRM_RUN=1; export ASSUME_YES=1 ;;
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
    if [[ "${WITH_ENCRYPTED_POST:-0}" == 1 ]]; then
        subscription_refresh_session || exit 1
        subscription_load_mydata_secrets || true
    fi

    if [[ "${SUBSCRIPTION_USE_MARKETPLACE_SELLER:-0}" == 1 ]]; then
        subscription_import_creator_from_marketplace || true
    fi

    case "${RUN_MODE:-}" in
        refresh) flow_refresh_session ;;
        run_menu_all) flow_run_menu_all ;;
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
