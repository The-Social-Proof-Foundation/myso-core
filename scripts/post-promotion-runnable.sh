#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper for post promoted-post flows (create → activate → confirm view).
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - `myso`, `curl`, `jq` on PATH
#
# Session: network.config/post-promotion/promotion-session.env
#
# Usage:
#   ./scripts/post-promotion-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/post-promotion-runnable.sh --run-all
#   ./scripts/post-promotion-runnable.sh   # interactive menu

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/post-promotion/promotion-session.env"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"

readonly PAYMENT_PER_VIEW='1000000'
readonly PROMOTION_BUDGET='3000000'
readonly VIEW_DURATION_MS='3000'

SOCIAL_RUN_ID="$(date +%s)"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"

CREATOR_ADDRESS=''
VIEWER_ADDRESS=''
CREATOR_PROFILE_ID=''
MEMORY_ACCOUNT_ID=''
POST_ID=''
PROMOTION_ID=''
MODERATORS_GROUP_ID=''
VIEWER_BALANCE_BEFORE=''

PROMOTION_SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID
    MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID
    PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MYDATA_REGISTRY_ID
    MODERATORS_GROUP_ID CREATOR_ADDRESS VIEWER_ADDRESS CREATOR_PROFILE_ID
    MEMORY_ACCOUNT_ID POST_ID PROMOTION_ID
)

usage() {
    sed -n '2,17p' "$0" | sed 's/^# \?//'
}

save_promotion_session() {
    social_save_session "${PROMOTION_SESSION_KEYS[@]}"
}

load_promotion_session() {
    social_load_session
}

ensure_creator_wallet() {
    CREATOR_ADDRESS="$(resolve_myso_active_address)" || {
        echo "Could not read myso client active-address" >&2
        return 1
    }
    CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")"
    ensure_wallet_funded "$CREATOR_ADDRESS" "$((PROMOTION_BUDGET + SOCIAL_DEFAULT_GAS_BUDGET * 2))" || return 1
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
}

ensure_viewer_wallet() {
    if [[ -z "${VIEWER_ADDRESS:-}" ]]; then
        VIEWER_ADDRESS="$(create_ephemeral_wallet "promo_viewer_${SOCIAL_RUN_ID}")" || return 1
    fi
    VIEWER_ADDRESS="$(normalize_hex_id "$VIEWER_ADDRESS")"
    ensure_wallet_funded "$VIEWER_ADDRESS" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    log_session_use "VIEWER_ADDRESS" "$VIEWER_ADDRESS"
}

ensure_platform_ready() {
    if [[ -z "${PLATFORM_OBJECT_ID:-}" ]]; then
        log_step "No PLATFORM_OBJECT_ID — creating test platform"
        create_test_platform || return 1
        save_promotion_session
    fi
    require_session_fields PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID BLOCK_LIST_REGISTRY_ID \
        POST_CONFIG_ID MYDATA_REGISTRY_ID MEMORY_CONFIG_ID || return 1
}

step_creator_profile_and_join() {
    local lines profile_id mem username
    ensure_creator_wallet || return 1
    username="creator${SOCIAL_RUN_ID}"
    switch_wallet "$CREATOR_ADDRESS" || return 1
    lines="$(create_profile_for_address "$CREATOR_ADDRESS" "Promo Creator ${SOCIAL_RUN_ID}" "$username")" || {
        restore_wallet
        return 1
    }
    profile_id="$(echo "$lines" | sed -n '1p')"
    mem="$(echo "$lines" | sed -n '2p')"
    CREATOR_PROFILE_ID="$(normalize_hex_id "$profile_id")"
    [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    [[ -n "${MEMORY_ACCOUNT_ID:-}" ]] || MEMORY_ACCOUNT_ID="$(gql_profile_snapshot "$CREATOR_ADDRESS" | jq -r '.data.profile.memoryAccountId // empty')"
    [[ -n "${MEMORY_ACCOUNT_ID:-}" ]] || {
        echo "MemoryAccount required for create_promoted_post" >&2
        restore_wallet
        return 1
    }
    ensure_joined_platform || { restore_wallet; return 1; }
    restore_wallet
    log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
    log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
    save_promotion_session
}

step_create_promoted_post() {
    local out digest pay_coin gas_coin saved_gas body_lit
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk ref_budget

    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1

    switch_wallet "$CREATOR_ADDRESS" || return 1
    read -r pay_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$CREATOR_ADDRESS" "$PROMOTION_BUDGET")" || {
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID="$gas_coin"
    saved_gas="$gas_coin"

    ref_ur="$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_pr="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_cfg="$(ptb_shared_ref "$POST_CONFIG_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_mr="$(ptb_shared_ref "$MYDATA_REGISTRY_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }
    ref_budget="$(ptb_shared_ref "$pay_coin")" || { PTB_GAS_COIN_ID=''; restore_wallet; return 1; }

    body_lit="$(literal_move_string "Promoted post ${SOCIAL_RUN_ID}")"
    log_step "create_promoted_post payment_per_view=$PAYMENT_PER_VIEW budget=$PROMOTION_BUDGET"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::create_promoted_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" \
        none none none none \
        "$PAYMENT_PER_VIEW" "$ref_budget" \
        none none none \
        "$ref_mr" "$ref_mem" "$ref_clk")" || {
        PTB_GAS_COIN_ID=''
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID=''
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    POST_ID="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$POST_ID" ]] || POST_ID="$(extract_created_object_by_type "$digest" "Post")"
    PROMOTION_ID="$(extract_created_object_by_type "$digest" "post::PromotionData")"
    [[ -n "$PROMOTION_ID" ]] || PROMOTION_ID="$(extract_created_object_by_type "$digest" "PromotionData")"
    [[ -n "$POST_ID" && -n "$PROMOTION_ID" ]] || {
        echo "create_promoted_post missing Post or PromotionData in tx effects" >&2
        return 1
    }
    POST_ID="$(normalize_hex_id "$POST_ID")"
    PROMOTION_ID="$(normalize_hex_id "$PROMOTION_ID")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "PROMOTION_ID" "$PROMOTION_ID"
    save_promotion_session
}

step_resolve_moderators_group() {
    MODERATORS_GROUP_ID="$(gql_platform_moderators_group_id "$PLATFORM_OBJECT_ID")" || true
    [[ -n "${MODERATORS_GROUP_ID:-}" ]] || {
        echo "Could not resolve platform.moderatorsGroupId from GraphQL" >&2
        return 1
    }
    log_session_use "MODERATORS_GROUP_ID" "$MODERATORS_GROUP_ID"
    save_promotion_session
}

step_activate_promotion() {
    require_hex_ids POST_ID PROMOTION_ID PLATFORM_OBJECT_ID MODERATORS_GROUP_ID CLOCK_ID || return 1
    switch_wallet "$CREATOR_ADDRESS" || return 1
    log_step "toggle_promotion_status activate=true"
    SKIP_CONFIRM_RUN=1 invoke_ptb_as "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::toggle_promotion_status" \
        "$(ptb_shared_ref "$POST_ID")" \
        "$(ptb_shared_ref "$PROMOTION_ID")" \
        "$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" \
        "$(ptb_shared_ref "$MODERATORS_GROUP_ID")" \
        true \
        "$(ptb_shared_ref "$CLOCK_ID")" || {
        restore_wallet
        return 1
    }
    restore_wallet
}

step_viewer_join_platform() {
    ensure_viewer_wallet || return 1
    switch_wallet "$VIEWER_ADDRESS" || return 1
    ensure_joined_platform || { restore_wallet; return 1; }
    restore_wallet
}

step_confirm_promoted_view() {
    require_hex_ids POST_ID PROMOTION_ID POST_CONFIG_ID PLATFORM_OBJECT_ID \
        MODERATORS_GROUP_ID CLOCK_ID || return 1
    switch_wallet "$CREATOR_ADDRESS" || return 1
    log_step "confirm_promoted_post_view viewer=$VIEWER_ADDRESS duration=$VIEW_DURATION_MS"
    SKIP_CONFIRM_RUN=1 invoke_ptb_as "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::confirm_promoted_post_view" \
        "$(ptb_shared_ref "$POST_ID")" \
        "$(ptb_shared_ref "$PROMOTION_ID")" \
        "$(ptb_shared_ref "$POST_CONFIG_ID")" \
        "$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" \
        "$(ptb_shared_ref "$MODERATORS_GROUP_ID")" \
        "@${VIEWER_ADDRESS}" \
        "$VIEW_DURATION_MS" \
        "$(ptb_shared_ref "$CLOCK_ID")" || {
        restore_wallet
        return 1
    }
    restore_wallet
}

assert_promotion_graphql() {
    local resp views remaining viewer expected_remaining
    expected_remaining="$((PROMOTION_BUDGET - PAYMENT_PER_VIEW))"
    log_step "GraphQL assert promotion views and budget"
    resp="$(wait_for_gql_promotion_views "$PROMOTION_ID" 1)" || return 1
    remaining="$(echo "$resp" | jq -r '.data.promotion.remainingBudget // empty')"
    viewer="$(echo "$resp" | jq -r '.data.promotion.viewsDetail[0].viewer // empty')"
    if [[ "$remaining" != "$expected_remaining" ]]; then
        echo "promotion.remainingBudget expected $expected_remaining got $remaining" >&2
        return 1
    fi
    if [[ "$(normalize_hex_id "$viewer")" != "$(normalize_hex_id "$VIEWER_ADDRESS")" ]]; then
        echo "promotion.viewsDetail[0].viewer expected $VIEWER_ADDRESS got $viewer" >&2
        return 1
    fi

    resp="$(graphql_post \
        'query PostPromotion($postId: ID!) { post(id: $postId) { promotionId } }' \
        "$(jq -nc --arg postId "$POST_ID" '{postId: $postId}')")" || return 1
    if [[ "$(echo "$resp" | jq -r '.data.post.promotionId // empty')" != "$PROMOTION_ID" ]]; then
        echo "post.promotionId mismatch" >&2
        return 1
    fi
    log_step "GraphQL promotion assertions passed"
}

assert_viewer_balance_increased() {
    local after before delta
    switch_wallet "$VIEWER_ADDRESS" || return 1
    after="$(resolve_max_coin_balance "$VIEWER_ADDRESS")"
    restore_wallet
    before="${VIEWER_BALANCE_BEFORE:-0}"
    delta=$((after - before))
    if [[ "$delta" -lt "$PAYMENT_PER_VIEW" ]]; then
        echo "Viewer balance increase expected >= $PAYMENT_PER_VIEW (before=$before after=$after delta=$delta)" >&2
        return 1
    fi
    log_step "Viewer balance increased by $delta MIST (>= $PAYMENT_PER_VIEW)"
}

run_promotion_flow() {
    load_promotion_session
    SOCIAL_RUN_ID="$(date +%s)"
    require_session_fields USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID \
        MEMORY_REGISTRY_ID MEMORY_CONFIG_ID POST_CONFIG_ID MYDATA_REGISTRY_ID \
        PLATFORM_REGISTRY_ID PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID || {
        echo "Run --refresh-session first" >&2
        return 1
    }

    ensure_platform_ready || return 1
    step_creator_profile_and_join || return 1
    step_create_promoted_post || return 1
    step_resolve_moderators_group || return 1
    step_activate_promotion || return 1
    ensure_viewer_wallet || return 1
    switch_wallet "$VIEWER_ADDRESS" || return 1
    VIEWER_BALANCE_BEFORE="$(resolve_max_coin_balance "$VIEWER_ADDRESS")"
    restore_wallet
    step_viewer_join_platform || return 1
    step_confirm_promoted_view || return 1
    assert_promotion_graphql || return 1
    assert_viewer_balance_increased || return 1
    save_promotion_session
    log_step "Post promotion E2E complete"
}

show_menu() {
    echo ""
    echo "=== Post Promotion E2E Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Run full flow (--run-all)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) social_refresh_session_from_graphql; load_promotion_session ;;
        1) run_promotion_flow ;;
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

    load_promotion_session

    case "${RUN_MODE:-}" in
        refresh) social_refresh_session_from_graphql; load_promotion_session; exit 0 ;;
        run_all) run_promotion_flow; exit 0 ;;
        '') show_menu ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
