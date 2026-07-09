#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Create a SPoT-enabled post (enable_spot=true) for oracle ingestion E2E.
# Pattern mirrors poc-oracle-post-runnable.sh but opts into SPoT only.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126 (for later pending-posts poll)
#   - `myso`, `curl`, `jq` on PATH
#
# Session: network.config/spot-oracle/spot-oracle-session.env
#
# Usage:
#   ./scripts/spot-oracle-post-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/spot-oracle-post-runnable.sh --run-all
#   ./scripts/spot-oracle-post-runnable.sh   # interactive

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/spot-oracle/spot-oracle-session.env"
# shellcheck source=lib/spot-oracle-common.sh
source "${SCRIPT_DIR}/lib/spot-oracle-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

SOCIAL_RUN_ID="$(date +%s)"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"
CREATOR_ADDRESS=''
CREATOR_PROFILE_ID=''
MEMORY_ACCOUNT_ID=''
POST_ID=''
LAST_TX_DIGEST=''
SPOT_CLAIM_TEXT="${SPOT_CLAIM_TEXT:-Will BTC trade above \$1 by end of day?}"

SPOT_POST_SESSION_KEYS=(
    "${SPOT_ORACLE_SESSION_KEYS[@]}"
    CREATOR_ADDRESS CREATOR_PROFILE_ID MEMORY_ACCOUNT_ID POST_ID LAST_TX_DIGEST
)

usage() {
    sed -n '2,22p' "$0" | sed 's/^# \?//'
}

save_spot_post_session() {
    spot_oracle_map_session_to_oracle_env
    social_save_session "${SPOT_POST_SESSION_KEYS[@]}"
}

ensure_creator_wallet() {
    CREATOR_ADDRESS="$(resolve_myso_active_address)" || {
        echo "Could not read myso client active-address" >&2
        return 1
    }
    CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")"
    ensure_wallet_funded "$CREATOR_ADDRESS" "$((SOCIAL_DEFAULT_GAS_BUDGET * 2))" || return 1
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
}

step_creator_profile_and_join() {
    local lines profile_id mem username snap profile_id_existing
    ensure_creator_wallet || return 1
    username="spotoracle${SOCIAL_RUN_ID}"
    switch_wallet "$CREATOR_ADDRESS" || return 1
    profile_id_existing="$(resolve_owned_profile_for_address "$CREATOR_ADDRESS")" || profile_id_existing=''
    if [[ -n "$profile_id_existing" ]]; then
        CREATOR_PROFILE_ID="$(normalize_hex_id "$profile_id_existing")"
        snap="$(gql_profile_snapshot "$CREATOR_ADDRESS" 2>/dev/null)" || snap='{}'
        mem="$(echo "$snap" | jq -r '.data.profile.memoryAccountId // empty')"
        [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
        log_step "Reusing creator profile $CREATOR_PROFILE_ID"
    else
        lines="$(create_profile_for_address "$CREATOR_ADDRESS" "SPoT Oracle Creator ${SOCIAL_RUN_ID}" "$username")" || {
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
        echo "MemoryAccount required for create_post" >&2
        restore_wallet
        return 1
    }
    ensure_joined_platform || { restore_wallet; return 1; }
    restore_wallet
    log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
    log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
    save_spot_post_session
}

ensure_spt_objects_for_post() {
    if [[ -n "${TOKEN_REGISTRY_ID:-}" && -n "${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}" ]]; then
        require_hex_ids TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
        return 0
    fi
    local json
    log_step "Fetching TokenRegistry + SocialProofTokensConfig from GraphQL"
    json="$(graphql_post 'query SpotPostSptObjects {
  socialProofTokenRegistry: objects(filter: { type: "0x50c1::social_proof_tokens::TokenRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  sptConfig: objects(filter: { type: "0x50c1::social_proof_tokens::SocialProofTokensConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
}')" || return 1
    TOKEN_REGISTRY_ID="$(gql_object_address "$json" socialProofTokenRegistry)"
    SOCIAL_PROOF_TOKENS_CONFIG_ID="$(gql_object_address "$json" sptConfig)"
    require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
    require_hex_ids TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
}

create_spot_enabled_post() {
    load_spot_oracle_session
    if [[ -z "${PLATFORM_OBJECT_ID:-}" || -z "${USERNAME_REGISTRY_ID:-}" ]]; then
        log_step "Refreshing session for post creation"
        refresh_spot_oracle_session_from_graphql || return 1
        load_spot_oracle_session
    fi
    step_creator_profile_and_join || return 1

    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1

    local body_lit media_opt out digest
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    body_lit="$(literal_move_string "${SPOT_CLAIM_TEXT} [${SOCIAL_RUN_ID}]")"
    media_opt='none'

    ref_ur="$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" || return 1
    ref_pr="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || return 1
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || return 1
    ref_cfg="$(ptb_shared_ref "$POST_CONFIG_ID")" || return 1
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || return 1
    ref_mr="$(ptb_shared_ref "$MYDATA_REGISTRY_ID")" || return 1
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1

    switch_wallet "$CREATOR_ADDRESS" || return 1
    log_step "create_post enable_spot=true content=${SPOT_CLAIM_TEXT}"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::create_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" \
        "$media_opt" \
        none none none none none none none \
        none some\(true\) none \
        "$ref_mr" "$ref_mem" "$ref_clk")" || {
        restore_wallet
        return 1
    }
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    LAST_TX_DIGEST="$digest"
    POST_ID="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$POST_ID" ]] || POST_ID="$(extract_created_object_by_type "$digest" "Post")"
    [[ -n "$POST_ID" ]] || {
        echo "create_post did not produce a Post object" >&2
        return 1
    }
    POST_ID="$(normalize_hex_id "$POST_ID")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "LAST_TX_DIGEST" "$LAST_TX_DIGEST"
    save_spot_post_session

    print_run_summary_header "SPoT Post Created"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "Creator" "$CREATOR_ADDRESS"
    print_run_summary_line "Claim" "$SPOT_CLAIM_TEXT"
    print_run_summary_line "Digest" "$LAST_TX_DIGEST"
    print_run_summary_footer
}

show_menu() {
    echo ""
    echo "=== SPoT Post Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Create enable_spot=true post (--run-all)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) refresh_spot_oracle_session_from_graphql; load_spot_oracle_session ;;
        1) create_spot_enabled_post ;;
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

    load_spot_oracle_session

    case "${RUN_MODE:-}" in
        refresh)
            refresh_spot_oracle_session_from_graphql || exit 1
            load_spot_oracle_session
            exit 0
            ;;
        run_all) create_spot_enabled_post ;;
        '')
            if [[ ! -t 0 ]]; then
                echo "No TTY — use: ASSUME_YES=1 ./scripts/spot-oracle-post-runnable.sh --run-all" >&2
                exit 1
            fi
            show_menu
            ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
