#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# SPoT oracle E2E helper: refresh chain object IDs from GraphQL, submit on-chain PTBs,
# and poll the shared oracle postgres / GraphQL for review → resolve progress.
#
# Prerequisites:
#   - ./scripts/run-spot-oracle.sh running in another terminal (postgres + oracle workers)
#   - ./scripts/bootstrap.sh completed; social-proof2 owns SpotOracleAdminCap
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126 (required for on-chain walkthrough)
#   - docker, curl, jq, cargo on PATH
#   - Live HTTP sources (CoinGecko etc.) OR SPOT_ORACLE_LIVE_SOURCES=false with stubs
#   - Discovery is NOT required (SPoT fetches trusted sources directly)
#
# Session: network.config/spot-oracle/spot-oracle-session.env
#
# SPoT is always-on: every post is analyzed — there is no enable_spot opt-in.
#
# Usage:
#   ./scripts/run-spot-oracle.sh                              # terminal 1
#   ./scripts/spot-oracle-runnable.sh                         # terminal 2 — interactive menu
#   ./scripts/spot-oracle-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --create-post
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --create-post-and-verify
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all-onchain
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-election-onchain
#     (election CADR: review + on-chain market only; skips maturity evidence/resolve wait)
#   ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-walkthrough
#   ./scripts/spot-oracle-runnable.sh --reset-checkpoint              # after chain reset
#   ./scripts/spot-oracle-runnable.sh --run-lazy-payout               # Move unit test

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/spot-oracle/spot-oracle-session.env"
# shellcheck source=lib/spot-oracle-common.sh
source "${SCRIPT_DIR}/lib/spot-oracle-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"
SOCIAL_RUN_ID="$(date +%s)"
CREATOR_ADDRESS=''
CREATOR_PROFILE_ID=''
MEMORY_ACCOUNT_ID=''
POST_ID=''
LAST_TX_DIGEST=''
CHECKPOINT_RESET_TO="${CHECKPOINT_RESET_TO:-0}"
BETTOR_ADDRESS=''
BET_OPTION_ID=''
BET_AMOUNT_MIST=''
BET_TX_DIGEST=''
PAYOUT_TX_DIGEST=''
SPOT_MARKET_ID=''
SPOT_CLAIM_TEXT="${SPOT_CLAIM_TEXT:-Will BTC trade above \$1 by end of day?}"
SPOT_ELECTION_CLAIM_TEXT="${SPOT_ELECTION_CLAIM_TEXT:-JD Vance will win the 2028 presidential election.}"

SPOT_SESSION_KEYS=(
    "${SPOT_ORACLE_SESSION_KEYS[@]}"
    CREATOR_ADDRESS CREATOR_PROFILE_ID MEMORY_ACCOUNT_ID POST_ID LAST_TX_DIGEST
    SPOT_CLAIM_TEXT SPOT_CLAIM_ID SPOT_MARKET_ID
    BETTOR_ADDRESS BET_OPTION_ID BET_AMOUNT_MIST BET_TX_DIGEST PAYOUT_TX_DIGEST
    INSURANCE_CONFIG_ID INSURANCE_ROUTER_CONFIG_ID INSURANCE_BACKSTOP_ID INSURANCE_ADMIN_CAP_ID
    INSURANCE_VAULT_ID INSURANCE_POLICY_ID INSURANCE_BUY_TX INSURANCE_CLAIM_TX
)

save_spot_session() {
    spot_oracle_map_session_to_oracle_env
    social_save_session "${SPOT_SESSION_KEYS[@]}"
}

usage() {
    sed -n '2,28p' "$0" | sed 's/^# \?//'
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
    save_spot_session
}

# Fixed-claim pipeline modes must not append [SOCIAL_RUN_ID] — it breaks semantic dedup.
spot_should_append_run_id() {
    if [[ -n "${SPOT_APPEND_RUN_ID:-}" ]]; then
        [[ "${SPOT_APPEND_RUN_ID}" == "1" ]]
        return
    fi
    case "${RUN_MODE:-}" in
        run_all_onchain|run_election_onchain|run_walkthrough) return 1 ;;
        *) return 0 ;;
    esac
}

spot_post_body_text() {
    local claim="$1"
    if spot_should_append_run_id; then
        printf '%s [%s]' "$claim" "$SOCIAL_RUN_ID"
    else
        printf '%s' "$claim"
    fi
}

create_spot_enabled_post() {
    local pinned_claim="${SPOT_CLAIM_TEXT:-}"
    load_spot_oracle_session_preserving_claim "$pinned_claim"
    if [[ -z "${PLATFORM_OBJECT_ID:-}" || -z "${USERNAME_REGISTRY_ID:-}" ]]; then
        log_step "Refreshing session for post creation"
        refresh_spot_oracle_session_from_graphql || return 1
        load_spot_oracle_session_preserving_claim "$pinned_claim"
    fi
    if [[ -t 0 && "${ASSUME_YES:-0}" != "1" && -z "${SPOT_CLAIM_TEXT:-}" ]]; then
        spot_prompt_claim_text
    fi
    step_creator_profile_and_join || return 1

    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1

    local body_lit media_opt out digest post_body
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    post_body="$(spot_post_body_text "$SPOT_CLAIM_TEXT")"
    body_lit="$(literal_move_string "$post_body")"
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
    log_step "create_public_post (always-on SPoT) content=${SPOT_CLAIM_TEXT}"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::create_public_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" \
        "$media_opt" \
        none none none none none none none \
        none some\(true\) \
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
        echo "create_public_post did not produce a Post object" >&2
        return 1
    }
    POST_ID="$(normalize_hex_id "$POST_ID")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "LAST_TX_DIGEST" "$LAST_TX_DIGEST"
    save_spot_session

    print_run_summary_header "SPoT Post Created"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "Creator" "$CREATOR_ADDRESS"
    print_run_summary_line "Claim" "$SPOT_CLAIM_TEXT"
    print_run_summary_line "Digest" "$LAST_TX_DIGEST"
    print_run_summary_footer
}

verify_spot_analysis() {
    require_spot_oracle_service || return 1
    require_session_fields POST_ID || return 1
    local attempts="${SPOT_VERIFY_ATTEMPTS:-20}" delay="${SPOT_VERIFY_DELAY:-3}" i json status
    log_step "Polling GraphQL spotAnalysis for POST_ID=$POST_ID (awaiting oracle finalize)"
    for ((i = 1; i <= attempts; i++)); do
        json="$(graphql_post 'query PostSpotAnalysis($id: ID!) {
  post(id: $id) {
    postId
    spotAnalysis { status detectedClaimCount rejectedClaimCount truncatedClaimCount futureAcceptedCount pastVerifiedCount }
    spotVerdicts { claimIndex verdict summary relatedMarketId }
    spotRecord { recordObjectId status }
  }
}' "$(jq -nc --arg id "$POST_ID" '{id: $id}')")" || {
            sleep "$delay"
            continue
        }
        status="$(echo "$json" | jq -r '.data.post.spotAnalysis.status // "UNKNOWN"')"
        echo "  attempt ${i}/${attempts}: spotAnalysis.status=${status}"
        if [[ "$status" == "COMPLETED" || "$status" == "COMPLETED_NO_ACTIONABLE" ]]; then
            echo "$json" | jq '.data.post'
            print_run_summary_header "SPoT Analysis Finalized"
            print_run_summary_line "POST_ID" "$POST_ID"
            print_run_summary_line "Status" "$status"
            print_run_summary_line "DetectedClaims" "$(echo "$json" | jq -r '.data.post.spotAnalysis.detectedClaimCount')"
            print_run_summary_line "FutureAccepted" "$(echo "$json" | jq -r '.data.post.spotAnalysis.futureAcceptedCount')"
            print_run_summary_line "PastVerified" "$(echo "$json" | jq -r '.data.post.spotAnalysis.pastVerifiedCount')"
            print_run_summary_footer
            return 0
        fi
        sleep "$delay"
    done
    echo "spotAnalysis did not reach a terminal status after ${attempts} attempts — is the oracle running?" >&2
    return 1
}

reset_oracle_checkpoint_watermark() {
    local target="${1:-${CHECKPOINT_RESET_TO:-0}}"
    local db="${SPOT_ORACLE_DATABASE_URL:-postgresql://spot:spot@127.0.0.1:5435/spot_oracle}"
    if [[ ! "$target" =~ ^[0-9]+$ ]]; then
        echo "reset target must be a non-negative integer (got '$target')" >&2
        return 1
    fi
    if ! command -v psql >/dev/null 2>&1; then
        echo "psql not found on PATH — install the postgres client to use this option." >&2
        return 1
    fi
    log_step "Resetting spot-oracle checkpoint watermark to $target"
    psql "$db" -v ON_ERROR_STOP=1 -c \
        "UPDATE checkpoint_ingest_state SET last_checkpoint_seq = ${target}, updated_at = now() WHERE id = 1;" || {
        echo "Failed to update watermark. Is spot-oracle postgres reachable at: $db ?" >&2
        return 1
    }
    local now
    now="$(psql "$db" -tA -c "SELECT last_checkpoint_seq FROM checkpoint_ingest_state WHERE id = 1;" 2>/dev/null)"
    echo "  checkpoint watermark is now: ${now:-?}"
    echo "  NOTE: restart the oracle for this to take effect: ./scripts/run-spot-oracle.sh"
}

run_create_post_and_verify() {
    create_spot_enabled_post || return 1
    if [[ "${SPOT_SKIP_VERIFY:-0}" != "1" ]]; then
        verify_spot_analysis || {
            echo "Note: analysis not yet finalized; re-run menu option 3 once the oracle catches up." >&2
            return 1
        }
    fi
}

psql_exec() {
    spot_oracle_psql_exec "$1"
}

wait_for_accepted_review() {
    local post_filter="${1:-}"
    local reviews=0 accepted=0 reason='' i review_sql accepted_sql
    if [[ -n "$post_filter" ]]; then
        review_sql="SELECT COUNT(*) FROM oracle_reviews WHERE post_id = '${post_filter}'"
        accepted_sql="SELECT COUNT(*) FROM oracle_reviews WHERE post_id = '${post_filter}' AND decision = 'accepted'"
    else
        review_sql="SELECT COUNT(*) FROM oracle_reviews"
        accepted_sql="SELECT COUNT(*) FROM oracle_reviews WHERE decision = 'accepted'"
    fi
    log_step "Waiting for accepted oracle_reviews${post_filter:+ for $post_filter} (up to 3 min)..."
    for i in $(seq 1 90); do
        reviews="$(psql_exec "$review_sql" || echo 0)"
        reviews="${reviews// /}"
        accepted="$(psql_exec "$accepted_sql" || echo 0)"
        accepted="${accepted// /}"
        if [[ -n "$accepted" && "$accepted" -gt 0 ]]; then
            REVIEWS_COUNT="$reviews"
            ACCEPTED_COUNT="$accepted"
            return 0
        fi
        if [[ -n "$post_filter" ]]; then
            reason="$(psql_exec "SELECT reject_reason FROM oracle_reviews WHERE post_id = '${post_filter}' AND decision = 'rejected' ORDER BY created_at DESC LIMIT 1" || true)"
            reason="${reason// /}"
            if [[ "$reason" == "missing_deadline" ]]; then
                echo "FAIL: claim rejected — add when the claim should be evaluated (e.g. 'by the end of tomorrow', 'before July 31, 2027', or 'JD Vance will win the 2028 presidential election')." >&2
                return 1
            fi
            if [[ "$reason" == "missing_threshold" ]]; then
                echo "FAIL: claim rejected — price claims need a measurable threshold (e.g. 'Will BTC trade above \$1 by end of tomorrow?')." >&2
                return 1
            fi
            if [[ "$reason" == "deadline_too_far" ]]; then
                echo "FAIL: claim rejected (deadline_too_far) — deadline exceeds max horizon (elections: ~4 years; default: ~2 years)." >&2
                return 1
            fi
            if [[ "$reason" == "deadline_in_past" ]]; then
                echo "FAIL: claim rejected (deadline_in_past) — use a deadline ahead of SPOT_ORACLE_MIN_DEADLINE_LEAD_SECS (local E2E: 'in 40 seconds')." >&2
                return 1
            fi
        fi
        sleep 2
    done
    echo "FAIL: no accepted oracle_reviews after pipeline (total=${reviews:-0})" >&2
    return 1
}

wait_for_linked_or_accepted_review() {
    local post_filter="${1:-}"
    local reviews=0 terminal=0 reason='' decision='' i review_sql terminal_sql
    if [[ -n "$post_filter" ]]; then
        review_sql="SELECT COUNT(*) FROM oracle_reviews WHERE post_id = '${post_filter}'"
        terminal_sql="SELECT COUNT(*) FROM oracle_reviews WHERE post_id = '${post_filter}' AND decision IN ('accepted', 'linked')"
    else
        review_sql="SELECT COUNT(*) FROM oracle_reviews"
        terminal_sql="SELECT COUNT(*) FROM oracle_reviews WHERE decision IN ('accepted', 'linked')"
    fi
    log_step "Waiting for accepted/linked oracle_reviews${post_filter:+ for $post_filter} (up to 3 min)..."
    for i in $(seq 1 90); do
        reviews="$(psql_exec "$review_sql" || echo 0)"
        reviews="${reviews// /}"
        terminal="$(psql_exec "$terminal_sql" || echo 0)"
        terminal="${terminal// /}"
        if [[ -n "$terminal" && "$terminal" -gt 0 ]]; then
            REVIEWS_COUNT="$reviews"
            ACCEPTED_COUNT="$terminal"
            decision="$(psql_exec "SELECT decision FROM oracle_reviews WHERE post_id = '${post_filter}' ORDER BY created_at DESC LIMIT 1" || true)"
            decision="${decision// /}"
            REVIEW_DECISION="${decision:-accepted}"
            return 0
        fi
        if [[ -n "$post_filter" ]]; then
            reason="$(psql_exec "SELECT reject_reason FROM oracle_reviews WHERE post_id = '${post_filter}' AND decision = 'rejected' ORDER BY created_at DESC LIMIT 1" || true)"
            reason="${reason// /}"
            if [[ -n "$reason" ]]; then
                echo "FAIL: claim rejected (${reason})" >&2
                return 1
            fi
        fi
        sleep 2
    done
    echo "FAIL: no accepted/linked oracle_reviews after pipeline (total=${reviews:-0})" >&2
    return 1
}

wait_for_market_active() {
    local post_id="$1"
    local require_spot_id="${2:-0}"
    local status='' market_obj='' i
    log_step "Waiting for market active${require_spot_id:+ + spot_market_object_id} (POST_ID=$post_id)..."
    for i in $(seq 1 90); do
        status="$(psql_exec "SELECT status FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
        status="${status// /}"
        market_obj="$(psql_exec "SELECT COALESCE(spot_market_object_id, '') FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
        market_obj="${market_obj// /}"
        if [[ "$status" == "waiting" || "$status" == "active" || "$status" == "resolving" || "$status" == "resolved" ]]; then
            if [[ "$require_spot_id" != "1" || -n "$market_obj" ]]; then
                MARKET_STATUS="$status"
                SPOT_MARKET_ID="${market_obj:-$SPOT_MARKET_ID}"
                return 0
            fi
        fi
        sleep 2
    done
    echo "FAIL: market not active (status='${status:-}' spot_market_object_id='${market_obj:-}')" >&2
    return 1
}

wait_for_evidence() {
    local post_filter="${1:-}"
    local evidence=0 hash='' i evidence_sql hash_sql
    if [[ -n "$post_filter" ]]; then
        evidence_sql="SELECT COUNT(*) FROM evidence e JOIN markets m ON m.id = e.market_id WHERE m.post_id = '${post_filter}'"
        hash_sql="SELECT e.content_hash FROM evidence e JOIN markets m ON m.id = e.market_id WHERE m.post_id = '${post_filter}' ORDER BY e.fetched_at DESC LIMIT 1"
    else
        evidence_sql="SELECT COUNT(*) FROM evidence"
        hash_sql="SELECT content_hash FROM evidence ORDER BY fetched_at DESC LIMIT 1"
    fi
    log_step "Waiting for evidence (after claim deadline, up to 5 min)..."
    for i in $(seq 1 150); do
        evidence="$(psql_exec "$evidence_sql" || echo 0)"
        evidence="${evidence// /}"
        if [[ -n "$evidence" && "$evidence" -gt 0 ]]; then
            hash="$(psql_exec "$hash_sql" || true)"
            hash="${hash// /}"
            EVIDENCE_COUNT="$evidence"
            SAMPLE_HASH="$hash"
            return 0
        fi
        sleep 2
    done
    echo "FAIL: no evidence rows after review+resolve pipeline" >&2
    return 1
}

wait_for_market_resolved() {
    local post_id="$1"
    local status='' i
    log_step "Waiting for market resolved (POST_ID=$post_id, up to 5 min)..."
    for i in $(seq 1 150); do
        status="$(psql_exec "SELECT status FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
        status="${status// /}"
        if [[ "$status" == "resolved" ]]; then
            MARKET_STATUS="$status"
            return 0
        fi
        sleep 2
    done
    echo "FAIL: market not resolved (status='${status:-}')" >&2
    return 1
}

collect_onchain_pipeline_state() {
    local post_id="$1"
    MARKET_STATUS="$(psql_exec "SELECT status FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
    MARKET_STATUS="${MARKET_STATUS// /}"
    SPOT_MARKET_ID="$(psql_exec "SELECT COALESCE(spot_market_object_id, '') FROM markets WHERE post_id = '${post_id}' LIMIT 1" || true)"
    SPOT_MARKET_ID="${SPOT_MARKET_ID// /}"
    SPOT_CLAIM_ID="$(psql_exec "SELECT COALESCE(sc.spot_claim_object_id, '') FROM post_claim_links pcl JOIN spot_claims sc ON sc.id = pcl.claim_id WHERE pcl.post_id = '${post_id}' LIMIT 1" || true)"
    SPOT_CLAIM_ID="${SPOT_CLAIM_ID// /}"
    MARKET_DEADLINE="$(psql_exec "SELECT COALESCE(sm.deadline::text, '') FROM post_claim_links pcl JOIN spot_markets sm ON sm.id = pcl.market_id WHERE pcl.post_id = '${post_id}' LIMIT 1" || true)"
    MARKET_DEADLINE="${MARKET_DEADLINE// /}"
    SEMANTIC_CLAIM_HASH="$(psql_exec "SELECT COALESCE(sc.semantic_claim_hash, '') FROM post_claim_links pcl JOIN spot_claims sc ON sc.id = pcl.claim_id WHERE pcl.post_id = '${post_id}' LIMIT 1" || true)"
    SEMANTIC_CLAIM_HASH="${SEMANTIC_CLAIM_HASH// /}"
}

poll_spot_analysis_status_optional() {
    local post_id="$1"
    local attempts="${2:-15}"
    local delay="${3:-2}"
    local i json
    SPOT_ANALYSIS_STATUS="PENDING"
    log_step "Polling spotAnalysis finalization (optional, up to $((attempts * delay))s)..."
    for ((i = 1; i <= attempts; i++)); do
        json="$(graphql_post 'query PostSpotAnalysis($id: ID!) {
  post(id: $id) { spotAnalysis { status futureAcceptedCount } }
}' "$(jq -nc --arg id "$post_id" '{id: $id}')")" || {
            sleep "$delay"
            continue
        }
        SPOT_ANALYSIS_STATUS="$(echo "$json" | jq -r '.data.post.spotAnalysis.status // "UNKNOWN"')"
        if [[ "$SPOT_ANALYSIS_STATUS" == "COMPLETED" || "$SPOT_ANALYSIS_STATUS" == "COMPLETED_NO_ACTIONABLE" ]]; then
            SPOT_FUTURE_ACCEPTED="$(echo "$json" | jq -r '.data.post.spotAnalysis.futureAcceptedCount // 0')"
            return 0
        fi
        sleep "$delay"
    done
    return 0
}

print_onchain_pipeline_summary() {
    local title="$1"
    local note="${2:-}"
    log_session_use "SPOT_CLAIM_ID" "${SPOT_CLAIM_ID:-}"
    log_session_use "SPOT_MARKET_ID" "${SPOT_MARKET_ID:-}"
    save_spot_session
    print_run_summary_header "$title"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Claim" "$SPOT_CLAIM_TEXT"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "market status" "${MARKET_STATUS:-<unknown>}"
    print_run_summary_line "spot_claim_object_id" "${SPOT_CLAIM_ID:-<none>}"
    print_run_summary_line "spot_market_object_id" "${SPOT_MARKET_ID:-<none>}"
    print_run_summary_line "market deadline (UTC)" "${MARKET_DEADLINE:-<none>}"
    print_run_summary_line "semantic_claim_hash" "${SEMANTIC_CLAIM_HASH:-<none>}"
    print_run_summary_line "spotAnalysis status" "${SPOT_ANALYSIS_STATUS:-<not polled>}"
    print_run_summary_line "oracle_reviews (accepted)" "${ACCEPTED_COUNT:-$REVIEWS_COUNT}"
    print_run_summary_line "SpotConfig" "${SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID:-<unset>}"
    print_run_summary_line "SpotClaimRegistry" "${SPOT_ORACLE_REGISTRY_OBJECT_ID:-<unset>}"
    if [[ -n "$note" ]]; then
        print_run_summary_line "Note" "$note"
    fi
    print_run_summary_footer
}

wait_for_reviews_and_evidence() {
    wait_for_accepted_review || return 1
    wait_for_evidence || return 1
}

wait_for_post_ingest() {
    local post_id="$1"
    local i found=0
    log_step "Waiting for checkpoint ingest to ingest ${post_id}..."
    for i in $(seq 1 90); do
        found="$(psql_exec "SELECT COUNT(*) FROM markets WHERE post_id = '${post_id}'" || echo 0)"
        found="${found// /}"
        if [[ -n "$found" && "$found" -gt 0 ]]; then
            return 0
        fi
        sleep 2
    done
    echo "FAIL: market for POST_ID=${post_id} not ingested (is SubscribeCheckpoints streaming?)" >&2
    return 1
}

walkthrough_ensure_platform() {
    if [[ -z "${PLATFORM_OBJECT_ID:-}" ]] || ! object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
        SOCIAL_RUN_ID="${SOCIAL_RUN_ID:-$(date +%s)}"
        log_step "No platform on localnet — creating test platform"
        create_test_platform || return 1
        save_spot_session
    fi
}

walkthrough_preflight() {
    require_social_stack_for_onchain || return 1
    validate_onchain_oracle_key || return 1
    log_step "Preflight OK (GraphQL + social-server + oracle key; no Discovery)"
}

walkthrough_refresh_session_if_needed() {
    if [[ -z "${SPOT_CONFIG_ID:-}" || -z "${SPOT_REGISTRY_ID:-}" || -z "${SPOT_ORACLE_ADMIN_CAP_ID:-}" ]]; then
        refresh_spot_oracle_session_from_graphql || return 1
        load_spot_oracle_session
    fi
}

run_market_walkthrough() {
    spot_prompt_walkthrough_claim || return 1
    local walkthrough_claim="$SPOT_CLAIM_TEXT"
    load_spot_oracle_session
    SPOT_CLAIM_TEXT="$walkthrough_claim"
    export SPOT_CLAIM_TEXT
    save_spot_session

    walkthrough_preflight || return 1
    walkthrough_refresh_session_if_needed || return 1

    require_external_oracle_stack || return 1
    walkthrough_ensure_platform || return 1

    log_step "Creating always-on SPoT post"
    SOCIAL_RUN_ID="${SOCIAL_RUN_ID:-$(date +%s)}"
    create_spot_enabled_post || return 1
    load_spot_oracle_session
    [[ -n "${POST_ID:-}" ]] || {
        echo "FAIL: POST_ID missing after create_spot_enabled_post" >&2
        return 1
    }
    log_session_use "POST_ID" "$POST_ID"
    save_spot_session

    wait_for_post_ingest "$POST_ID" || return 1
    wait_for_accepted_review "$POST_ID" || return 1
    wait_for_market_active "$POST_ID" 1 || return 1

    SPOT_MARKET_ID="$(spot_resolve_market_id "$POST_ID" "${SPOT_MARKET_ID:-}")" || return 1
    log_session_use "SPOT_MARKET_ID" "$SPOT_MARKET_ID"

    local options_json
    options_json="$(psql_exec "SELECT betting_options::text FROM markets WHERE post_id = '${POST_ID}' LIMIT 1" || true)"
    options_json="${options_json// /}"
    [[ -n "$options_json" && "$options_json" != "[]" ]] || options_json='["Yes","No"]'
    log_step "Market active — POST_ID=$POST_ID market=$SPOT_MARKET_ID options=${options_json}"

    spot_prompt_bet_side "$options_json" || return 1
    spot_prompt_bet_amount_mist

    BETTOR_ADDRESS="${BETTOR_ADDRESS:-${CREATOR_ADDRESS:-}}"
    [[ -n "$BETTOR_ADDRESS" ]] || BETTOR_ADDRESS="$(resolve_myso_active_address)" || return 1
    BETTOR_ADDRESS="$(normalize_hex_id "$BETTOR_ADDRESS")"
    log_session_use "BETTOR_ADDRESS" "$BETTOR_ADDRESS"

    spot_place_bet_for_post "$BETTOR_ADDRESS" "$POST_ID" "$SPOT_MARKET_ID" "$BET_OPTION_ID" "$BET_AMOUNT_MIST" || return 1
    save_spot_session

    if [[ "${ENABLE_INSURANCE_E2E:-0}" == "1" ]]; then
        log_step "ENABLE_INSURANCE_E2E=1 — preparing vault + buying coverage"
        spot_insurance_e2e_prepare || return 1
        spot_insurance_buy_coverage "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" "$BET_OPTION_ID" || return 1
        save_spot_session
    fi

    log_step "Waiting for oracle resolve (~1–3 min for price-threshold claims)..."
    wait_for_market_resolved "$POST_ID" || return 1

    if [[ "${ENABLE_INSURANCE_E2E:-0}" == "1" ]]; then
        spot_insurance_claim "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" || return 1
        spot_insurance_assert_duplicate_claim_fails "$BETTOR_ADDRESS" "$SPOT_MARKET_ID" || return 1
        save_spot_session
    fi

    if [[ "${ASSUME_YES:-0}" != "1" ]]; then
        read -r -p "Market resolved. Press Enter to claim payout (or Ctrl+C to skip)... " _
    fi
    spot_claim_payout "$BETTOR_ADDRESS" "$POST_ID" "$SPOT_MARKET_ID" || return 1
    save_spot_session

    print_run_summary_header "SPoT Market Walkthrough — PASS"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Claim" "$SPOT_CLAIM_TEXT"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "SpotMarket" "$SPOT_MARKET_ID"
    print_run_summary_line "Bettor" "$BETTOR_ADDRESS"
    print_run_summary_line "Bet side" "option_id=${BET_OPTION_ID}"
    print_run_summary_line "Bet amount (MIST)" "$BET_AMOUNT_MIST"
    print_run_summary_line "Bet tx" "${BET_TX_DIGEST:-<none>}"
    print_run_summary_line "Payout tx" "${PAYOUT_TX_DIGEST:-<none>}"
    if [[ "${ENABLE_INSURANCE_E2E:-0}" == "1" ]]; then
        print_run_summary_line "Insurance policy" "${INSURANCE_POLICY_ID:-<none>}"
        print_run_summary_line "Insurance claim tx" "${INSURANCE_CLAIM_TX:-<none>}"
    fi
    print_run_summary_line "Market status" "${MARKET_STATUS:-resolved}"
    print_run_summary_footer
}

run_spot_move_tests() {
    local filter="$1"
    local pkg_dir="$REPO_ROOT/crates/myso-framework/packages/myso-social"
    log_step "Running Move tests (filter=$filter)"
    (cd "$pkg_dir" && myso move test --filter "$filter") || return 1
}

run_lazy_payout_e2e() {
    log_step "SPoT lazy payout: resolve retains escrow; claim_payout + claim_creator_payout are O(1)"
    run_spot_move_tests "test_lazy_claim_creator_payout" || return 1
    print_run_summary_header "SPoT Lazy Payout — PASS (Move)"
    print_run_summary_line "Scenario" "resolve → pending tables → lazy winner/creator claims"
    print_run_summary_footer
}

run_creator_fee_e2e() {
    log_step "SPoT creator fee: distinct pending entries per referring post"
    run_spot_move_tests "test_per_referrer_creator_fees" || return 1
    print_run_summary_header "SPoT Creator Fee — PASS (Move)"
    print_run_summary_line "Scenario" "two referrers → independent claim_creator_payout"
    print_run_summary_footer
}

run_expired_reclaim_e2e() {
    log_step "SPoT expired reclaim: past creator_claim_window_ms → reclaim_expired_creator_rewards"
    run_spot_move_tests "test_reclaim_expired_creator_rewards" || return 1
    print_run_summary_header "SPoT Expired Reclaim — PASS (Move)"
    print_run_summary_line "Scenario" "unclaimed creator rewards → ecosystem (+ platform remainder)"
    print_run_summary_footer
}

run_shared_claim_e2e() {
    log_step "SPoT shared claim: multiple posts link to one claim/market"
    run_spot_move_tests "test_shared_claim_multiple_posts" || return 1
    print_run_summary_header "SPoT Shared Claim — PASS (Move)"
    print_run_summary_line "Scenario" "link_post_to_spot_claim shares open market"
    print_run_summary_footer
}

run_router_only_e2e() {
    log_step "SPoT router-only: betting via unlinked post is rejected"
    run_spot_move_tests "test_router_rejects_unlinked_post" || return 1
    print_run_summary_header "SPoT Router-Only — PASS (Move)"
    print_run_summary_line "Scenario" "place_spot_bet_for_post rejects EPostNotLinked"
    print_run_summary_footer
}

run_ownership_transfer_e2e() {
    log_step "SPoT ownership: only settlement-recorded creator can claim_creator_payout"
    run_spot_move_tests "test_settlement_creator_only_can_claim" || return 1
    print_run_summary_header "SPoT Settlement Creator — PASS (Move)"
    print_run_summary_line "Scenario" "non-creator rejected at claim_creator_payout (ENotCreator)"
    print_run_summary_footer
}

run_onchain_pipeline_e2e() {
    local summary_title="${1:-SPoT Oracle E2E — PASS (pipeline)}"
    local pinned_claim="${SPOT_CLAIM_TEXT:-}"
    load_spot_oracle_session_preserving_claim "$pinned_claim"
    require_social_stack_for_onchain || return 1
    validate_onchain_oracle_key || return 1
    require_external_oracle_stack || return 1

    log_step "Creating always-on SPoT post"
    SOCIAL_RUN_ID="${SOCIAL_RUN_ID:-$(date +%s)}"
    ASSUME_YES=1 create_spot_enabled_post || return 1
    load_spot_oracle_session
    if [[ -z "${POST_ID:-}" ]]; then
        # shellcheck disable=SC1090
        source "$SOCIAL_SESSION_SAVE_PATH"
    fi
    [[ -n "${POST_ID:-}" ]] || {
        echo "FAIL: POST_ID missing after create_spot_enabled_post" >&2
        return 1
    }
    log_session_use "POST_ID" "$POST_ID"

    wait_for_post_ingest "$POST_ID" || return 1
    wait_for_linked_or_accepted_review "$POST_ID" || return 1
    wait_for_market_active "$POST_ID" 1 || return 1

    collect_onchain_pipeline_state "$POST_ID"
    poll_spot_analysis_status_optional "$POST_ID" || true

    print_onchain_pipeline_summary "$summary_title" \
        "Evidence collection and market resolve run at maturity (ResolveMarket job); not waited here."
}

run_onchain_e2e() {
    load_spot_oracle_session
    require_social_stack_for_onchain || return 1
    validate_onchain_oracle_key || return 1
    require_external_oracle_stack || return 1

    log_step "Creating always-on SPoT post"
    SOCIAL_RUN_ID="${SOCIAL_RUN_ID:-$(date +%s)}"
    ASSUME_YES=1 create_spot_enabled_post || return 1
    load_spot_oracle_session
    if [[ -z "${POST_ID:-}" ]]; then
        # shellcheck disable=SC1090
        source "$SOCIAL_SESSION_SAVE_PATH"
    fi
    [[ -n "${POST_ID:-}" ]] || {
        echo "FAIL: POST_ID missing after create_spot_enabled_post" >&2
        return 1
    }
    log_session_use "POST_ID" "$POST_ID"

    wait_for_post_ingest "$POST_ID" || return 1

    wait_for_accepted_review "$POST_ID" || return 1
    wait_for_market_active "$POST_ID" 1 || return 1
    wait_for_evidence "$POST_ID" || return 1
    wait_for_market_resolved "$POST_ID" || return 1

    print_run_summary_header "SPoT Oracle E2E — PASS (on-chain path)"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "market status" "${MARKET_STATUS:-<unknown>}"
    print_run_summary_line "spot_claim_id" "${SPOT_CLAIM_ID:-<none>}"
    print_run_summary_line "spot_market_id" "${SPOT_MARKET_ID:-<none>}"
    print_run_summary_line "SpotConfig" "${SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID:-<unset>}"
    print_run_summary_line "SpotClaimRegistry" "${SPOT_ORACLE_REGISTRY_OBJECT_ID:-<unset>}"
    print_run_summary_line "oracle_reviews (accepted)" "${ACCEPTED_COUNT:-$REVIEWS_COUNT}"
    print_run_summary_line "evidence rows" "$EVIDENCE_COUNT"
    print_run_summary_line "sample content_hash" "${SAMPLE_HASH:-<none>}"
    print_run_summary_footer
}

run_onchain_election_e2e() {
    local saved_claim="$SPOT_CLAIM_TEXT"
    SPOT_CLAIM_TEXT="$SPOT_ELECTION_CLAIM_TEXT"
    export SPOT_CLAIM_TEXT
    log_step "Election CADR E2E claim: ${SPOT_CLAIM_TEXT}"
    run_onchain_pipeline_e2e "Election CADR E2E — PASS (first post)"
    local rc=$?
    if [[ $rc -ne 0 ]]; then
        SPOT_CLAIM_TEXT="$saved_claim"
        export SPOT_CLAIM_TEXT
        return $rc
    fi

    local first_market_id="${SPOT_MARKET_ID:-}"
    local first_post_id="${POST_ID:-}"
    log_step "Posting duplicate election claim to verify dedup/link"
    ASSUME_YES=1 create_spot_enabled_post || {
        SPOT_CLAIM_TEXT="$saved_claim"
        export SPOT_CLAIM_TEXT
        return 1
    }
    load_spot_oracle_session
    wait_for_post_ingest "$POST_ID" || {
        SPOT_CLAIM_TEXT="$saved_claim"
        export SPOT_CLAIM_TEXT
        return 1
    }
    wait_for_linked_or_accepted_review "$POST_ID" || {
        SPOT_CLAIM_TEXT="$saved_claim"
        export SPOT_CLAIM_TEXT
        return 1
    }
    wait_for_market_active "$POST_ID" 1 || {
        SPOT_CLAIM_TEXT="$saved_claim"
        export SPOT_CLAIM_TEXT
        return 1
    }
    collect_onchain_pipeline_state "$POST_ID"

    local link_kind
    link_kind="$(psql_exec "SELECT link_kind FROM post_claim_links WHERE post_id = '${POST_ID}' LIMIT 1" || true)"
    link_kind="${link_kind// /}"
    if [[ -n "$first_market_id" && -n "${SPOT_MARKET_ID:-}" && "$first_market_id" != "${SPOT_MARKET_ID}" ]]; then
        echo "FAIL: duplicate election claim created a new spot market (${first_market_id} vs ${SPOT_MARKET_ID})" >&2
        rc=1
    elif [[ "$link_kind" != "linked" ]]; then
        echo "FAIL: duplicate election post expected link_kind=linked, got '${link_kind:-<empty>}'" >&2
        rc=1
    else
        print_run_summary_header "Election Dedup — PASS"
        print_run_summary_line "first_post_id" "$first_post_id"
        print_run_summary_line "second_post_id" "$POST_ID"
        print_run_summary_line "shared_spot_market_id" "${SPOT_MARKET_ID:-<none>}"
        print_run_summary_line "link_kind" "$link_kind"
        print_run_summary_footer
    fi

    SPOT_CLAIM_TEXT="$saved_claim"
    export SPOT_CLAIM_TEXT
    return $rc
}

run_sports_smoke_e2e() {
    local saved_claim="$SPOT_CLAIM_TEXT"
    SPOT_CLAIM_TEXT="${SPOT_SPORTS_CLAIM_TEXT:-Spain will win the FIFA World Cup}"
    export SPOT_CLAIM_TEXT
    log_step "Sports classification smoke: ${SPOT_CLAIM_TEXT}"
    ASSUME_YES=1 create_spot_enabled_post || {
        SPOT_CLAIM_TEXT="$saved_claim"
        export SPOT_CLAIM_TEXT
        return 1
    }
    load_spot_oracle_session
    wait_for_post_ingest "$POST_ID" || return 1
    wait_for_linked_or_accepted_review "$POST_ID" || return 1
    local event_id
    event_id="$(psql_exec "SELECT normalized_fields->'resolver_hints'->>'matched_event_id' FROM canonical_claims cc JOIN oracle_reviews r ON r.canonical_claim_id = cc.id WHERE r.post_id = '${POST_ID}' ORDER BY r.created_at DESC LIMIT 1" || true)"
    event_id="${event_id// /}"
    if [[ "$event_id" != *"fifa"* && "$event_id" != *"world_cup"* ]]; then
        echo "FAIL: sports claim matched event_id='${event_id:-<empty>}' (expected FIFA/World Cup)" >&2
        SPOT_CLAIM_TEXT="$saved_claim"
        export SPOT_CLAIM_TEXT
        return 1
    fi
    print_run_summary_header "Sports Smoke — PASS"
    print_run_summary_line "POST_ID" "$POST_ID"
    print_run_summary_line "matched_event_id" "$event_id"
    print_run_summary_footer
    SPOT_CLAIM_TEXT="$saved_claim"
    export SPOT_CLAIM_TEXT
    return 0
}

show_menu() {
    echo ""
    echo "=== SPoT Oracle E2E ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Full market walkthrough (requires ./scripts/run-spot-oracle.sh in another terminal)"
    echo " 2) Create post (always-on; oracle analyzes when ./scripts/run-spot-oracle.sh is running)"
    echo " 3) Verify spotAnalysis for session POST_ID"
    echo " 4) Create post + verify spotAnalysis"
    echo " 5) Reset checkpoint watermark (after chain reset; then restart run-spot-oracle.sh)"
    echo " ---"
    echo " Advanced / developer"
    echo " b) On-chain pipeline (post → review → resolve, no bet)"
    echo " i) On-chain pipeline with election claim (CADR; stops after market create)"
    echo " c) Move test: lazy payout"
    echo " d) Move test: creator fees"
    echo " e) Move test: expired reclaim"
    echo " f) Move test: shared claim"
    echo " g) Move test: router-only rejection"
    echo " h) Move test: settlement creator gate"
    echo " ?) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0)
            refresh_spot_oracle_session_from_graphql && load_spot_oracle_session || {
                echo "FAIL: session refresh aborted; existing session file unchanged." >&2
            }
            ;;
        1) run_market_walkthrough ;;
        2) create_spot_enabled_post ;;
        3) verify_spot_analysis ;;
        4) run_create_post_and_verify ;;
        5)
            local ck
            read -r -p "Reset checkpoint watermark to [0]: " ck
            reset_oracle_checkpoint_watermark "${ck:-0}"
            ;;
        [Bb]) run_onchain_e2e ;;
        [Ii]) run_onchain_election_e2e ;;
        [Cc]) run_lazy_payout_e2e ;;
        [Dd]) run_creator_fee_e2e ;;
        [Ee]) run_expired_reclaim_e2e ;;
        [Ff]) run_shared_claim_e2e ;;
        [Gg]) run_router_only_e2e ;;
        [Hh]) run_ownership_transfer_e2e ;;
        \?) usage ;;
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
            --create-post) RUN_MODE=create_post; shift ;;
            --verify-analysis) RUN_MODE=verify_analysis; shift ;;
            --create-post-and-verify) RUN_MODE=create_post_and_verify; shift ;;
            --reset-checkpoint) RUN_MODE=reset_checkpoint; CHECKPOINT_RESET_TO=0; shift ;;
            --reset-checkpoint=*) RUN_MODE=reset_checkpoint; CHECKPOINT_RESET_TO="${1#*=}"; shift ;;
            --run-all-onchain) RUN_MODE=run_all_onchain; shift ;;
            --run-election-onchain) RUN_MODE=run_election_onchain; shift ;;
            --run-sports-smoke) RUN_MODE=run_sports_smoke; shift ;;
            --run-walkthrough) RUN_MODE=run_walkthrough; shift ;;
            --run-lazy-payout) RUN_MODE=run_lazy_payout; shift ;;
            --run-creator-fee) RUN_MODE=run_creator_fee; shift ;;
            --run-expired-reclaim) RUN_MODE=run_expired_reclaim; shift ;;
            --run-shared-claim) RUN_MODE=run_shared_claim; shift ;;
            --run-router-only) RUN_MODE=run_router_only; shift ;;
            --run-ownership-transfer) RUN_MODE=run_ownership_transfer; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    local claim_from_env=''
    # Walkthrough / mode-specific E2E claims are set by the runner, not session/env.
    case "${RUN_MODE:-}" in
        run_walkthrough|run_election_onchain|run_sports_smoke) ;;
        *)
            if [[ -n "${SPOT_CLAIM_TEXT:-}" ]]; then
                claim_from_env="$SPOT_CLAIM_TEXT"
            fi
            ;;
    esac

    load_spot_oracle_session
    if [[ -n "$claim_from_env" ]]; then
        SPOT_CLAIM_TEXT="$claim_from_env"
        export SPOT_CLAIM_TEXT
    fi

    case "${RUN_MODE:-}" in
        refresh)
            refresh_spot_oracle_session_from_graphql || {
                echo "FAIL: could not refresh SPoT session from GraphQL ($GRAPHQL_URL)" >&2
                exit 1
            }
            load_spot_oracle_session
            exit 0
            ;;
        reset_checkpoint)
            reset_oracle_checkpoint_watermark "${CHECKPOINT_RESET_TO:-0}" || exit 1
            exit 0
            ;;
        create_post)
            create_spot_enabled_post
            ;;
        verify_analysis)
            verify_spot_analysis
            ;;
        create_post_and_verify)
            run_create_post_and_verify
            ;;
        run_all_onchain) run_onchain_e2e ;;
        run_election_onchain) run_onchain_election_e2e ;;
        run_sports_smoke) run_sports_smoke_e2e ;;
        run_walkthrough) run_market_walkthrough ;;
        run_lazy_payout) run_lazy_payout_e2e ;;
        run_creator_fee) run_creator_fee_e2e ;;
        run_expired_reclaim) run_expired_reclaim_e2e ;;
        run_shared_claim) run_shared_claim_e2e ;;
        run_router_only) run_router_only_e2e ;;
        run_ownership_transfer) run_ownership_transfer_e2e ;;
        '')
            if [[ ! -t 0 ]]; then
                echo "No TTY — use: ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --create-post" >&2
                exit 1
            fi
            show_menu
            ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
