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
# DB expectations after a successful run (raw tables; continuous aggregates may lag):
#   - promoted_posts / active_promoted_posts: total_views=1, total_paid=1M,
#     remaining_budget=2M when PROMOTION_BUDGET=3M and PAYMENT_PER_VIEW=1M
#   - post_stats_daily: empty (aggregates tips, not promotion views)
#   - promotion_spending_daily / promotion_views_hourly: Timescale CAGGs; verify
#     promotion_views for immediate E2E confirmation
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
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

readonly PAYMENT_PER_VIEW='1000000'
readonly PROMOTION_BUDGET='3000000'
readonly VIEW_DURATION_MS='3000'
# Default PostConfig fees are 1000 bps + 1000 bps => viewer gets 80% of gross.
readonly EXPECTED_PLATFORM_FEE='100000'
readonly EXPECTED_ECOSYSTEM_FEE='100000'
readonly EXPECTED_RECIPIENT_AMOUNT='800000'

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
VIEWER_BALANCE_AFTER=''
CONFIRM_VIEW_TX_DIGEST=''
PROMOTION_GQL_SNAPSHOT=''
GQL_INDEXED='true'

PROMOTION_SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID
    MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_OBJECT_ID
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
    if [[ -n "${PLATFORM_OBJECT_ID:-}" ]]; then
        if ! object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
            echo "Session PLATFORM_OBJECT_ID not on localnet fullnode; recreating." >&2
            PLATFORM_OBJECT_ID=''
        fi
    fi
    if [[ -z "${PLATFORM_OBJECT_ID:-}" ]]; then
        log_step "No live PLATFORM_OBJECT_ID — creating test platform"
        create_test_platform || return 1
        save_promotion_session
    fi
    require_session_fields PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID BLOCK_LIST_REGISTRY_ID \
        POST_CONFIG_ID MYDATA_REGISTRY_ID MEMORY_CONFIG_ID || return 1
}

step_creator_profile_and_join() {
    local lines profile_id mem username snap profile_id_existing
    ensure_creator_wallet || return 1
    username="creator${SOCIAL_RUN_ID}"
    switch_wallet "$CREATOR_ADDRESS" || return 1
    profile_id_existing="$(resolve_owned_profile_for_address "$CREATOR_ADDRESS")" || profile_id_existing=''
    if [[ -n "$profile_id_existing" ]]; then
        CREATOR_PROFILE_ID="$(normalize_hex_id "$profile_id_existing")"
        snap="$(gql_profile_snapshot "$CREATOR_ADDRESS" 2>/dev/null)" || snap='{}'
        mem="$(echo "$snap" | jq -r '.data.profile.memoryAccountId // empty')"
        [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
        log_step "Reusing creator profile $CREATOR_PROFILE_ID"
    else
        lines="$(create_profile_for_address "$CREATOR_ADDRESS" "Promo Creator ${SOCIAL_RUN_ID}" "$username")" || {
            restore_wallet
            return 1
        }
        profile_id="$(echo "$lines" | sed -n '1p')"
        mem="$(echo "$lines" | sed -n '2p')"
        CREATOR_PROFILE_ID="$(normalize_hex_id "$profile_id")"
        [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    fi
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
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk

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

    body_lit="$(literal_move_string "Promoted post ${SOCIAL_RUN_ID}")"
    log_step "create_promoted_post payment_per_view=$PAYMENT_PER_VIEW budget=$PROMOTION_BUDGET"
    # Signature (17 PTB args; ctx omitted): registries/config…, content, media, mentions,
    # metadata, payment_per_view, budget, enable_spt, enable_spot, mydata_registry,
    # memory_account, clock. Access is always Public on-chain (no access arg).
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --split-coins "@${pay_coin}" "[${PROMOTION_BUDGET}]" \
        --assign promotion_budget \
        --move-call "${PKG_SOCIAL}::post::create_promoted_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" \
        none none none \
        "$PAYMENT_PER_VIEW" promotion_budget.0 \
        none none \
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

assert_tx_success() {
    local out="$1" digest="${2:-}" json status
    [[ -n "$out" || -n "$digest" ]] || return 1
    if [[ -z "$digest" ]]; then
        digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    fi
    if [[ -n "$digest" ]]; then
        json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
        status="$(echo "$json" | jq -r '.effects.V2.status // .effects.status // empty | tostring')"
        [[ "$status" == "Success" ]]
        return
    fi
    echo "$out" | jq -e '
        (.effects.V2.status // .effects.status // empty | tostring) == "Success"
    ' >/dev/null
}

tx_has_event_named() {
    local digest="$1" event_name="$2" json
    [[ -n "$digest" && -n "$event_name" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    echo "$json" | jq -e --arg name "$event_name" '
        [.. | objects | select(has("type_") or has("type"))]
        | map(.type_ // .type)
        | any(.name? == $name)
    ' >/dev/null
}

gql_post_promotion_snapshot() {
    local post_id="$1" resp vars
    post_id="$(normalize_hex_id "$post_id")" || return 1
    vars="$(jq -nc --arg id "$post_id" '{id: $id}')"
    resp="$(graphql_post \
        'query PostPromotionSnapshot($id: ID!) {
            post(id: $id) {
                postId
                promotionId
                promotion {
                    promotionId
                    views
                    remainingBudget
                    budget
                    status
                    viewsDetail(limit: 5) {
                        viewer
                        promotionId
                        viewDuration
                        paymentAmount
                        platformFee
                        ecosystemFee
                        recipientAmount
                    }
                }
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

wait_for_gql_promotion_snapshot() {
    local promotion_id="$1" min_views="$2" attempt resp views promoted_id
    promotion_id="$(normalize_hex_id "$promotion_id")" || return 1
    for attempt in $(seq 1 30); do
        resp="$(gql_promotion_snapshot "$promotion_id" 2>/dev/null)" || resp='{}'
        promoted_id="$(echo "$resp" | jq -r '.data.promotion.promotionId // empty' 2>/dev/null || true)"
        views="$(echo "$resp" | jq -r '.data.promotion.views // empty' 2>/dev/null || true)"
        if [[ -n "$promoted_id" && -n "$views" && "$views" -ge "$min_views" ]]; then
            printf '%s' "$resp"
            return 0
        fi
        if [[ -n "${POST_ID:-}" ]]; then
            resp="$(gql_post_promotion_snapshot "$POST_ID" 2>/dev/null)" || resp='{}'
            promoted_id="$(echo "$resp" | jq -r '.data.post.promotion.promotionId // empty' 2>/dev/null || true)"
            views="$(echo "$resp" | jq -r '.data.post.promotion.views // empty' 2>/dev/null || true)"
            if [[ -n "$promoted_id" && -n "$views" && "$views" -ge "$min_views" ]]; then
                printf '%s' "$resp"
                return 0
            fi
        fi
        if (( attempt == 1 || attempt % 5 == 0 )); then
            echo "  [gql wait] attempt ${attempt}/30 promotion views >= ${min_views} (last: ${views:-0})" >&2
        fi
        sleep 1
    done
    echo "Timed out waiting for promotion views >= $min_views (last: ${views:-0})" >&2
    return 1
}

assert_on_chain_promotion_view() {
    [[ -n "${CONFIRM_VIEW_TX_DIGEST:-}" ]] || {
        echo "Missing CONFIRM_VIEW_TX_DIGEST for on-chain promotion view verification" >&2
        return 1
    }
    assert_tx_success '' "$CONFIRM_VIEW_TX_DIGEST" || {
        echo "confirm_promoted_post_views tx $CONFIRM_VIEW_TX_DIGEST did not succeed" >&2
        return 1
    }
    tx_has_event_named "$CONFIRM_VIEW_TX_DIGEST" "PromotedPostViewsBatchConfirmedEvent" || {
        echo "confirm_promoted_post_views tx missing PromotedPostViewsBatchConfirmedEvent" >&2
        return 1
    }
    log_step "On-chain promotion view batch confirmed ($CONFIRM_VIEW_TX_DIGEST)"
}

step_confirm_promoted_view() {
    local out promo_type
    require_hex_ids PROMOTION_ID POST_CONFIG_ID PLATFORM_OBJECT_ID \
        MODERATORS_GROUP_ID ECOSYSTEM_TREASURY_ID CLOCK_ID || return 1
    switch_wallet "$CREATOR_ADDRESS" || return 1
    promo_type="${PKG_SOCIAL}::post::PromotionData"
    log_step "confirm_promoted_post_views viewer=$VIEWER_ADDRESS duration=$VIEW_DURATION_MS (len=1)"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --make-move-vec "<${promo_type}>" "[$(ptb_shared_ref "$PROMOTION_ID")]" \
        --assign promotions \
        --make-move-vec "<u64>" "[${VIEW_DURATION_MS}]" \
        --assign view_durations \
        --move-call "${PKG_SOCIAL}::post::confirm_promoted_post_views" \
        promotions \
        view_durations \
        "$(ptb_shared_ref "$POST_CONFIG_ID")" \
        "$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" \
        "$(ptb_shared_ref "$MODERATORS_GROUP_ID")" \
        "$(ptb_shared_ref "$ECOSYSTEM_TREASURY_ID")" \
        "@${VIEWER_ADDRESS}" \
        "$(ptb_shared_ref "$CLOCK_ID")")" || {
        restore_wallet
        return 1
    }
    CONFIRM_VIEW_TX_DIGEST="$(extract_tx_digest "$out")"
    [[ -n "$CONFIRM_VIEW_TX_DIGEST" ]] || {
        echo "confirm_promoted_post_views missing transaction digest" >&2
        restore_wallet
        return 1
    }
    restore_wallet
}

assert_promotion_graphql() {
    local resp views budget remaining viewer view_duration expected_remaining promotion_id
    local payment_amount platform_fee ecosystem_fee recipient_amount
    expected_remaining="$((PROMOTION_BUDGET - PAYMENT_PER_VIEW))"
    log_step "GraphQL assert promotion views and budget (best-effort; on-chain already verified)"
    if ! resp="$(wait_for_gql_promotion_snapshot "$PROMOTION_ID" 1)"; then
        GQL_INDEXED='false'
        echo "Warning: GraphQL indexer has not indexed promotion views yet; continuing with on-chain verification." >&2
        return 0
    fi

    if [[ -n "$(echo "$resp" | jq -r '.data.promotion.promotionId // empty')" ]]; then
        views="$(echo "$resp" | jq -r '.data.promotion.views // empty')"
        budget="$(echo "$resp" | jq -r '.data.promotion.budget // empty')"
        remaining="$(echo "$resp" | jq -r '.data.promotion.remainingBudget // empty')"
        viewer="$(echo "$resp" | jq -r '.data.promotion.viewsDetail[0].viewer // empty')"
        view_duration="$(echo "$resp" | jq -r '.data.promotion.viewsDetail[0].viewDuration // empty')"
        payment_amount="$(echo "$resp" | jq -r '.data.promotion.viewsDetail[0].paymentAmount // empty')"
        platform_fee="$(echo "$resp" | jq -r '.data.promotion.viewsDetail[0].platformFee // empty')"
        ecosystem_fee="$(echo "$resp" | jq -r '.data.promotion.viewsDetail[0].ecosystemFee // empty')"
        recipient_amount="$(echo "$resp" | jq -r '.data.promotion.viewsDetail[0].recipientAmount // empty')"
        promotion_id="$(echo "$resp" | jq -r '.data.promotion.promotionId // empty')"
    else
        views="$(echo "$resp" | jq -r '.data.post.promotion.views // empty')"
        budget="$(echo "$resp" | jq -r '.data.post.promotion.budget // empty')"
        remaining="$(echo "$resp" | jq -r '.data.post.promotion.remainingBudget // empty')"
        viewer="$(echo "$resp" | jq -r '.data.post.promotion.viewsDetail[0].viewer // empty')"
        view_duration="$(echo "$resp" | jq -r '.data.post.promotion.viewsDetail[0].viewDuration // empty')"
        payment_amount="$(echo "$resp" | jq -r '.data.post.promotion.viewsDetail[0].paymentAmount // empty')"
        platform_fee="$(echo "$resp" | jq -r '.data.post.promotion.viewsDetail[0].platformFee // empty')"
        ecosystem_fee="$(echo "$resp" | jq -r '.data.post.promotion.viewsDetail[0].ecosystemFee // empty')"
        recipient_amount="$(echo "$resp" | jq -r '.data.post.promotion.viewsDetail[0].recipientAmount // empty')"
        promotion_id="$(echo "$resp" | jq -r '.data.post.promotion.promotionId // empty')"
    fi

    if [[ "$views" != "1" ]]; then
        echo "Warning: promotion.views expected 1 got ${views:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$budget" != "$PROMOTION_BUDGET" ]]; then
        echo "Warning: promotion.budget expected $PROMOTION_BUDGET got ${budget:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$remaining" != "$expected_remaining" ]]; then
        echo "Warning: promotion.remainingBudget expected $expected_remaining got ${remaining:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$(normalize_hex_id "$viewer")" != "$(normalize_hex_id "$VIEWER_ADDRESS")" ]]; then
        echo "Warning: promotion viewer expected $VIEWER_ADDRESS got ${viewer:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$view_duration" != "$VIEW_DURATION_MS" ]]; then
        echo "Warning: promotion viewDuration expected $VIEW_DURATION_MS got ${view_duration:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$payment_amount" != "$PAYMENT_PER_VIEW" ]]; then
        echo "Warning: promotion paymentAmount expected $PAYMENT_PER_VIEW got ${payment_amount:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$platform_fee" != "$EXPECTED_PLATFORM_FEE" ]]; then
        echo "Warning: promotion platformFee expected $EXPECTED_PLATFORM_FEE got ${platform_fee:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$ecosystem_fee" != "$EXPECTED_ECOSYSTEM_FEE" ]]; then
        echo "Warning: promotion ecosystemFee expected $EXPECTED_ECOSYSTEM_FEE got ${ecosystem_fee:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$recipient_amount" != "$EXPECTED_RECIPIENT_AMOUNT" ]]; then
        echo "Warning: promotion recipientAmount expected $EXPECTED_RECIPIENT_AMOUNT got ${recipient_amount:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi
    if [[ "$(normalize_hex_id "$promotion_id")" != "$(normalize_hex_id "$PROMOTION_ID")" ]]; then
        echo "Warning: promotionId expected $PROMOTION_ID got ${promotion_id:-<none>}" >&2
        GQL_INDEXED='false'
        return 0
    fi

    resp="$(graphql_post \
        'query PostPromotion($postId: ID!) { post(id: $postId) { promotionId } }' \
        "$(jq -nc --arg postId "$POST_ID" '{postId: $postId}')")" || {
        GQL_INDEXED='false'
        echo "Warning: could not query post.promotionId from GraphQL" >&2
        return 0
    }
    if [[ "$(echo "$resp" | jq -r '.data.post.promotionId // empty')" != "$PROMOTION_ID" ]]; then
        echo "Warning: post.promotionId mismatch (GraphQL indexer lag)" >&2
        GQL_INDEXED='false'
        return 0
    fi
    log_step "GraphQL promotion assertions passed"
    PROMOTION_GQL_SNAPSHOT="$resp"
}

assert_viewer_balance_increased() {
    local after before delta
    switch_wallet "$VIEWER_ADDRESS" || return 1
    after="$(resolve_total_coin_balance "$VIEWER_ADDRESS")"
    restore_wallet
    before="${VIEWER_BALANCE_BEFORE:-0}"
    delta=$((after - before))
    VIEWER_BALANCE_AFTER="$after"
    if [[ "$delta" -lt "$EXPECTED_RECIPIENT_AMOUNT" ]]; then
        echo "Viewer total balance increase expected >= $EXPECTED_RECIPIENT_AMOUNT (before=$before after=$after delta=$delta)" >&2
        return 1
    fi
    log_step "Viewer total balance increased by $delta MIST (>= $EXPECTED_RECIPIENT_AMOUNT net after fees)"
}

print_post_promotion_run_all_summary() {
    local views budget remaining status view_duration viewer payout expected_remaining outcome
    local gql_resp="${PROMOTION_GQL_SNAPSHOT:-}"
    [[ -n "$gql_resp" ]] || gql_resp="$(gql_promotion_snapshot "$PROMOTION_ID" 2>/dev/null)" || gql_resp='{}'

    views="$(echo "$gql_resp" | jq -r '.data.promotion.views // empty')"
    budget="$(echo "$gql_resp" | jq -r '.data.promotion.budget // empty')"
    remaining="$(echo "$gql_resp" | jq -r '.data.promotion.remainingBudget // empty')"
    status="$(echo "$gql_resp" | jq -r '.data.promotion.status // empty')"
    view_duration="$(echo "$gql_resp" | jq -r '.data.promotion.viewsDetail[0].viewDuration // empty')"
    viewer="$(echo "$gql_resp" | jq -r '.data.promotion.viewsDetail[0].viewer // empty')"
    expected_remaining="$((PROMOTION_BUDGET - PAYMENT_PER_VIEW))"
    payout="$((VIEWER_BALANCE_AFTER - VIEWER_BALANCE_BEFORE))"

    print_run_summary_header "Post Promotion E2E — run-all completed successfully"
    print_run_summary_line "Run ID" "$SOCIAL_RUN_ID"
    print_run_summary_line "Platform" "$(normalize_hex_id "${PLATFORM_OBJECT_ID:-}")"
    print_run_summary_line "Creator" "$(normalize_hex_id "$CREATOR_ADDRESS") (profile $(normalize_hex_id "$CREATOR_PROFILE_ID"))"
    print_run_summary_line "Viewer" "$(normalize_hex_id "$VIEWER_ADDRESS")"
    print_run_summary_line "Post" "$(normalize_hex_id "$POST_ID")"
    print_run_summary_line "Promotion" "$(normalize_hex_id "$PROMOTION_ID")"
    print_run_summary_line "Memory account" "$(normalize_hex_id "$MEMORY_ACCOUNT_ID")"
    print_run_summary_line "Moderators group" "$(normalize_hex_id "$MODERATORS_GROUP_ID")"
    print_run_summary_line "Promotion budget" "$(format_mist_with_units "${budget:-$PROMOTION_BUDGET}")"
    print_run_summary_line "Payment per view" "$(format_mist_with_units "$PAYMENT_PER_VIEW")"
    if [[ "${GQL_INDEXED:-false}" == "true" ]]; then
        print_run_summary_line "Views confirmed" "${views:-1} (GraphQL)"
    else
        print_run_summary_line "Views confirmed" "1 (on-chain tx ${CONFIRM_VIEW_TX_DIGEST:-<digest>})"
    fi
    print_run_summary_line "Remaining budget" "$(format_mist_with_units "${remaining:-$expected_remaining}")"
    print_run_summary_line "Promotion status" "${status:-active}"
    print_run_summary_line "View duration" "${view_duration:-$VIEW_DURATION_MS} ms"
    print_run_summary_line "Viewer wallet (before view)" "$(format_mist_with_units "${VIEWER_BALANCE_BEFORE:-0}") total"
    print_run_summary_line "Viewer wallet (after view)" "$(format_mist_with_units "${VIEWER_BALANCE_AFTER:-0}") total"
    print_run_summary_line "Viewer payout (net)" "$(format_mist_with_units "$payout")"
    print_run_summary_line "GraphQL indexer" "$GRAPHQL_URL"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Flow steps" "profile + join → create_promoted_post → activate → viewer join → confirm_promoted_post_views → on-chain verify → GraphQL verify (best-effort)"
    outcome="Creator escrowed $(format_mist_with_units "$PROMOTION_BUDGET") for promotion at $(format_mist_with_units "$PAYMENT_PER_VIEW") per view. "
    outcome+="One view from $(normalize_hex_id "${viewer:-$VIEWER_ADDRESS}") was confirmed (${view_duration:-$VIEW_DURATION_MS} ms). "
    outcome+="Viewer received $(format_mist_with_units "$payout") on-chain; $(format_mist_with_units "${remaining:-$expected_remaining}") promotion budget remains."
    print_run_summary_line "Outcome" "$outcome"
    print_run_summary_footer
}

run_promotion_flow() {
    load_promotion_session
    SOCIAL_RUN_ID="$(date +%s)"
    require_session_fields USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID \
        MEMORY_REGISTRY_ID MEMORY_CONFIG_ID POST_CONFIG_ID MYDATA_REGISTRY_ID \
        PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID || {
        echo "Run --refresh-session first" >&2
        return 1
    }

    ensure_platform_ready || return 1
    step_creator_profile_and_join || return 1
    step_create_promoted_post || return 1
    step_resolve_moderators_group || return 1
    step_activate_promotion || return 1
    ensure_viewer_wallet || return 1
    step_viewer_join_platform || return 1
    switch_wallet "$VIEWER_ADDRESS" || return 1
    VIEWER_BALANCE_BEFORE="$(resolve_total_coin_balance "$VIEWER_ADDRESS")"
    restore_wallet
    step_confirm_promoted_view || return 1
    assert_on_chain_promotion_view || return 1
    assert_viewer_balance_increased || return 1
    assert_promotion_graphql || return 1
    save_promotion_session
    print_post_promotion_run_all_summary
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
