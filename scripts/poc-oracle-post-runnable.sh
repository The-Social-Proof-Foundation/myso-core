#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper: create a PoC+SPT-enabled post with media on an approved platform, then
# optionally tip and reserve SPT into that post for the proof-of-creativity oracle
# (localnet.yaml at ../proof-of-creativity/config/networks/).
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - `myso`, `curl`, `jq` on PATH
#
# Session: network.config/poc-oracle/poc-oracle-session.env
#
# Usage:
#   ./scripts/poc-oracle-post-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/poc-oracle-post-runnable.sh --run-all
#   ASSUME_YES=1 ./scripts/poc-oracle-post-runnable.sh --tip-post
#   ASSUME_YES=1 ./scripts/poc-oracle-post-runnable.sh --reserve-post
#   ASSUME_YES=1 ./scripts/poc-oracle-post-runnable.sh --self-match-analyze
#   ./scripts/poc-oracle-post-runnable.sh   # interactive menu

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
: "${SOCIAL_SESSION_SAVE_PATH:=$REPO_ROOT/network.config/poc-oracle/poc-oracle-session.env}"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"
# shellcheck source=lib/poc-oracle-common.sh
source "${SCRIPT_DIR}/lib/poc-oracle-common.sh"
# shellcheck source=lib/poc-oracle-http.sh
source "${SCRIPT_DIR}/lib/poc-oracle-http.sh"

readonly POST_MEDIA_URL='https://pub-b2100065e2e5444db484a5fed5d87096.r2.dev/image/2026/05/23a44811-b991-418e-88ee-b99c3973fa91.png'
readonly PLATFORM_WEBSITE_URL='https://pub-1f3749a8084a44c3abbd97a4875268a1.r2.dev'

readonly POC_ORACLE_GQL_EXTRAS='query PocOracleSessionExtras {
  pocConfig: objects(filter: { type: "0x50c1::proof_of_creativity::PoCConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocRegistry: objects(filter: { type: "0x50c1::proof_of_creativity::PoCRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocVaultDirectory: objects(filter: { type: "0x50c1::poc_vault::PoCVaultDirectory", ownerKind: SHARED }, first: 1) { nodes { address } }
  proofOfCreativityAdminCap: objects(filter: { type: "0x50c1::proof_of_creativity::PoCAdminCap" }, last: 1) { nodes { address } }
  socialProofTokenRegistry: objects(filter: { type: "0x50c1::social_proof_tokens::TokenRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  sptConfig: objects(filter: { type: "0x50c1::social_proof_tokens::SocialProofTokensConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  socialProofTokensAdminCap: objects(filter: { type: "0x50c1::social_proof_tokens::SocialProofTokensAdminCap" }, last: 1) { nodes { address } }
  platform: objects(filter: { type: "0x50c1::platform::Platform" }, last: 1) { nodes { address } }
}'

readonly DEFAULT_TIP_AMOUNT='100000000'
readonly DEFAULT_RESERVE_AMOUNT='1000000000'

SOCIAL_RUN_ID="$(date +%s)"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"

POC_CONFIG_ID=''
POC_REGISTRY_ID=''
POC_ADMIN_CAP_ID=''
POC_VAULT_DIRECTORY_ID=''
TOKEN_REGISTRY_ID=''
SOCIAL_PROOF_TOKENS_CONFIG_ID=''
SPT_ADMIN_CAP_ID=''
CREATOR_ADDRESS=''
CREATOR_PROFILE_ID=''
MEMORY_ACCOUNT_ID=''
TIPPER_ADDRESS=''
TIPPER_MEMORY_ACCOUNT_ID=''
POST_ID=''
RESERVATION_POOL_ID=''
POC_BENEFICIARY_VAULT_ID=''
LAST_TX_DIGEST=''
LAST_TIP_TX_DIGEST=''
LAST_RESERVE_TX_DIGEST=''
SELF_MATCH_ANALYZE_DIGEST=''
POST_GQL_SNAPSHOT=''
GQL_INDEXED='false'

POC_ORACLE_SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID
    MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_OBJECT_ID
    PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MYDATA_REGISTRY_ID
    POC_CONFIG_ID POC_REGISTRY_ID POC_ADMIN_CAP_ID POC_VAULT_DIRECTORY_ID
    TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID SPT_ADMIN_CAP_ID
    CREATOR_ADDRESS CREATOR_PROFILE_ID MEMORY_ACCOUNT_ID
    TIPPER_ADDRESS TIPPER_MEMORY_ACCOUNT_ID
    POST_ID RESERVATION_POOL_ID POC_BENEFICIARY_VAULT_ID
    LAST_TX_DIGEST LAST_TIP_TX_DIGEST LAST_RESERVE_TX_DIGEST SELF_MATCH_ANALYZE_DIGEST
)

usage() {
    sed -n '2,18p' "$0" | sed 's/^# \?//'
}

save_poc_oracle_session() {
    social_save_session "${POC_ORACLE_SESSION_KEYS[@]}"
}

load_poc_oracle_session() {
    social_load_session
}

bytes_to_hex_arg() {
    python3 - "$1" <<'PY'
import sys
print("0x" + sys.argv[1].encode("utf-8").hex())
PY
}

run_myso_call() {
    local module="$1" func="$2"
    shift 2
    local -a cmd call_args=()
    local arg
    while IFS= read -r -d '' arg; do call_args+=("$arg"); done < <(normalize_client_call_args "$@")
    cmd=(myso client call --package "$PKG_SOCIAL" --module "$module" --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    if ((${#call_args[@]} > 0)) && [[ "${call_args[0]}" == --args ]]; then
        call_args=("${call_args[@]:1}")
    fi
    cmd+=(--args)
    cmd+=("${call_args[@]}")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        "${cmd[@]}"
    else
        return 0
    fi
}

run_myso_call_as() {
    local sender="$1" module="$2" func="$3"
    shift 3
    local -a cmd call_args=()
    local arg
    while IFS= read -r -d '' arg; do call_args+=("$arg"); done < <(normalize_client_call_args "$@")
    sender="$(normalize_hex_id "$sender")" || return 1
    cmd=(myso client call --package "$PKG_SOCIAL" --sender "$sender" \
        --module "$module" --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    if ((${#call_args[@]} > 0)) && [[ "${call_args[0]}" == --args ]]; then
        call_args=("${call_args[@]:1}")
    fi
    cmd+=(--args)
    cmd+=("${call_args[@]}")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        "${cmd[@]}"
    else
        return 0
    fi
}

read_spt_trading_enabled() {
    local json
    [[ -n "${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}" ]] || return 1
    json="$(myso client object "$SOCIAL_PROOF_TOKENS_CONFIG_ID" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r '
        .. | objects | select(has("trading_enabled")) | .trading_enabled // empty
    ' | head -n1
}

ensure_spt_trading_enabled() {
    local enabled
    enabled="$(read_spt_trading_enabled)" || true
    if [[ "$enabled" == "true" ]]; then
        log_session_use "SPT trading_enabled" "true"
        return 0
    fi
    require_session_fields SPT_ADMIN_CAP_ID SOCIAL_PROOF_TOKENS_CONFIG_ID CLOCK_ID || return 1
    require_hex_ids SPT_ADMIN_CAP_ID SOCIAL_PROOF_TOKENS_CONFIG_ID CLOCK_ID || return 1
    log_step "Enabling SPT trading (toggle_emergency_kill_switch)"
    SKIP_CONFIRM_RUN=1 run_myso_call social_proof_tokens toggle_emergency_kill_switch \
        "$SPT_ADMIN_CAP_ID" "$SOCIAL_PROOF_TOKENS_CONFIG_ID" true \
        "$(bytes_to_hex_arg "PoC oracle post enable trading")" "$CLOCK_ID"
}

resolve_latest_post_id() {
    local resp vars owner post_id
    if [[ -n "${POST_ID:-}" ]] && object_exists_on_fullnode "$POST_ID"; then
        normalize_hex_id "$POST_ID"
        return 0
    fi
    [[ -n "${CREATOR_ADDRESS:-}" ]] || CREATOR_ADDRESS="$(resolve_myso_active_address)" || return 1
    owner="$(normalize_hex_id "$CREATOR_ADDRESS")" || return 1
    vars="$(jq -nc --arg owner "$owner" '{owner: $owner}')"
    resp="$(graphql_post \
        'query LatestCreatorPost($owner: String!) {
            posts(owner: $owner, limit: 1) { id createdAt enableSpt }
        }' \
        "$vars")" || return 1
    post_id="$(echo "$resp" | jq -r '.data.posts[0].id // empty')"
    [[ -n "$post_id" ]] || {
        echo "No indexed post found for creator $owner; run menu 1 first." >&2
        return 1
    }
    POST_ID="$(normalize_hex_id "$post_id")"
    log_session_use "POST_ID" "$POST_ID"
    save_poc_oracle_session
    normalize_hex_id "$POST_ID"
}

gql_beneficiary_vault_id_for_address() {
    local addr="$1" resp vars vault_id
    [[ -n "$addr" ]] || return 1
    vars="$(jq -nc --arg addr "$addr" '{addr: $addr}')"
    resp="$(graphql_post \
        'query VaultForBeneficiary($addr: MySoAddress!) { pocBeneficiaryVaultByBeneficiary(beneficiary: $addr) { vaultId } }' \
        "$vars")" || return 1
    vault_id="$(echo "$resp" | jq -r '.data.pocBeneficiaryVaultByBeneficiary.vaultId // empty' | head -n1)"
    [[ -n "$vault_id" ]] || return 1
    normalize_hex_id "$vault_id"
}

parse_beneficiary_address_from_vault_object() {
    local vault_id="$1" json
    vault_id="$(normalize_hex_id "$vault_id")" || return 1
    json="$(myso client object "$vault_id" --json 2>/dev/null)" || return 1
    python3 - "$json" <<'PY'
import json, sys
j = json.loads(sys.argv[1])
contents = j.get("data", {}).get("Move", {}).get("contents")
if not contents or len(contents) < 64:
    raise SystemExit(1)
print("0x" + bytes(contents[32:64]).hex())
PY
}

lookup_beneficiary_vault_id_on_fullnode() {
    local beneficiary="$1" resp cursor="" vault_id ben normalized_ben
    beneficiary="$(normalize_hex_id "$beneficiary")" || return 1
    while true; do
        if [[ -z "$cursor" ]]; then
            resp="$(graphql_post \
                'query { objects(filter: { type: "0x50c1::poc_vault::PoCBeneficiaryVault", ownerKind: SHARED }, first: 50) { nodes { address } pageInfo { hasNextPage endCursor } } }' \
                '{}')" \
                || return 1
        else
            resp="$(graphql_post \
                'query VaultScan($cursor: String!) { objects(filter: { type: "0x50c1::poc_vault::PoCBeneficiaryVault", ownerKind: SHARED }, first: 50, after: $cursor) { nodes { address } pageInfo { hasNextPage endCursor } } }' \
                "$(jq -nc --arg cursor "$cursor" '{cursor: $cursor}')")" || return 1
        fi
        while IFS= read -r vault_id; do
            [[ -n "$vault_id" ]] || continue
            ben="$(parse_beneficiary_address_from_vault_object "$vault_id")" || continue
            normalized_ben="$(normalize_hex_id "$ben")" || continue
            if [[ "$normalized_ben" == "$beneficiary" ]]; then
                normalize_hex_id "$vault_id"
                return 0
            fi
        done < <(echo "$resp" | jq -r '.data.objects.nodes[]?.address // empty')
        if [[ "$(echo "$resp" | jq -r '.data.objects.pageInfo.hasNextPage // false')" != "true" ]]; then
            break
        fi
        cursor="$(echo "$resp" | jq -r '.data.objects.pageInfo.endCursor // empty')"
        [[ -n "$cursor" ]] || break
    done
    return 1
}

read_poc_oracle_address_from_graphql() {
    local resp oracle
    resp="$(graphql_post '{ pocConfiguration { oracleAddress } }' '{}')" || return 1
    oracle="$(echo "$resp" | jq -r '.data.pocConfiguration.oracleAddress // empty' | head -n1)"
    [[ -n "$oracle" ]] || return 1
    normalize_hex_id "$oracle"
}

ptb_option_address_from_arg() {
    local arg="$1"
    if [[ "$arg" == none ]]; then
        printf 'none'
        return 0
    fi
    if [[ "$arg" =~ ^some\((0x[0-9a-fA-F]+)\)$ ]]; then
        printf 'some(@%s)' "${BASH_REMATCH[1]}"
        return 0
    fi
    printf '%s' "$arg"
}

# Back-to-back PTBs mutate the same gas coin; resolve a fresh coin id before each submit.
LAST_PTB_GAS_COIN_USED=''

resolve_fresh_gas_coin_for_address() {
    local addr="$1" exclude="${2:-}" json coin
    addr="$(normalize_hex_id "$addr")" || return 1
    json="$(resolve_gas_coins_json_for_address "$addr")" || return 1
    if [[ -n "$exclude" ]]; then
        exclude="$(normalize_hex_id "$exclude")"
        coin="$(echo "$json" | jq -r --arg ex "$exclude" '
            [.[] | (.gasCoinId // .coinObjectId) as $id | select($id != null and $id != "" and $id != $ex) | $id][0] // empty
        ')"
        if [[ -n "$coin" ]]; then
            normalize_hex_id "$coin"
            return 0
        fi
    fi
    resolve_gas_coin_for_address "$addr"
}

begin_fresh_ptb_gas_coin() {
    local sender="$1"
    BEGIN_FRESH_PTB_GAS_SAVED="${PTB_GAS_COIN_ID:-}"
    PTB_GAS_COIN_ID="$(resolve_fresh_gas_coin_for_address "$sender" "$LAST_PTB_GAS_COIN_USED")" || return 1
    PTB_GAS_COIN_ID="$(normalize_hex_id "$PTB_GAS_COIN_ID")"
    LAST_PTB_GAS_COIN_USED="$PTB_GAS_COIN_ID"
}

end_fresh_ptb_gas_coin() {
    if [[ -n "${BEGIN_FRESH_PTB_GAS_SAVED:-}" ]]; then
        PTB_GAS_COIN_ID="$BEGIN_FRESH_PTB_GAS_SAVED"
    else
        unset PTB_GAS_COIN_ID
    fi
    unset BEGIN_FRESH_PTB_GAS_SAVED
}

poc_oracle_load_localnet_env

analyze_post_as_oracle() {
    local post_id="$1" media_type="$2" score="$3" original_creator_arg="$4" \
        deriv_target="$5" embed_audio="$6" apply_explicit="$7" explicit_outcome="$8"
    if [[ "${POC_USE_DIRECT_MOVE:-0}" == "1" ]]; then
        analyze_post_as_oracle_direct "$post_id" "$media_type" "$score" "$original_creator_arg" \
            "$deriv_target" "$embed_audio" "$apply_explicit" "$explicit_outcome"
        return $?
    fi
    sync_poc_config_oracle_on_chain "$POC_DEFAULT_ORACLE_ADDRESS" || return 1
    ensure_poc_oracle_key_in_env || return 1
    poc_oracle_analyze_post "$post_id" "$media_type" "$score" "$original_creator_arg" \
        "$deriv_target" "$embed_audio" "$apply_explicit" "$explicit_outcome"
}

analyze_post_as_oracle_direct() {
    local post_id="$1" media_type="$2" score="$3" original_creator_arg="$4" \
        deriv_target="$5" embed_audio="$6" apply_explicit="$7" explicit_outcome="$8"
    local oracle ref_cfg ref_reg ref_vault ref_post ref_clk creator_arg out digest
    require_hex_ids POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID CLOCK_ID || return 1
    creator_arg="$(ptb_option_address_from_arg "$original_creator_arg")"
    oracle="$(read_poc_oracle_address_from_graphql)" || {
        echo "Could not resolve PoC oracle address from GraphQL" >&2
        return 1
    }
    switch_wallet "$oracle" || return 1
    log_step "analyze_and_update_post post=$post_id score=$score creator=$original_creator_arg"
    ref_cfg="$(ptb_shared_ref "$POC_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_reg="$(ptb_shared_ref "$POC_REGISTRY_ID")" || { restore_wallet; return 1; }
    ref_vault="$(ptb_shared_ref "$POC_VAULT_DIRECTORY_ID")" || { restore_wallet; return 1; }
    ref_post="$(ptb_shared_ref "$post_id")" || { restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_wallet; return 1; }
    begin_fresh_ptb_gas_coin "$oracle" || { restore_wallet; return 1; }
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$oracle" \
        --move-call "${PKG_SOCIAL}::proof_of_creativity::analyze_and_update_post" \
        "$ref_cfg" "$ref_reg" "$ref_vault" "$ref_post" \
        "$media_type" "$score" "$creator_arg" "$deriv_target" \
        "$embed_audio" "$apply_explicit" "$explicit_outcome" \
        none none "$ref_clk")" || {
        end_fresh_ptb_gas_coin
        restore_wallet
        return 1
    }
    end_fresh_ptb_gas_coin
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    [[ -n "$digest" ]]
    printf '%s' "$digest"
}

assert_self_match_analyze_tx() {
    local digest="$1"
    [[ -n "$digest" ]] || return 1
    assert_poc_scenario_events self_match "$digest" || return 1
    log_step "self-match analyze OK digest=$digest (badge issued, no redirect)"
}

create_poc_post_for_analyze() {
    local label="$1"
    local out digest post_id body_lit media_opt ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1
    body_lit="$(literal_move_string "$label")"
    media_opt="some(vector[$(literal_move_string "$POST_MEDIA_URL")])"
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
    begin_fresh_ptb_gas_coin "$CREATOR_ADDRESS" || { restore_wallet; return 1; }
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::create_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" \
        "$media_opt" \
        none none none none none none none \
        some\(true\) none none \
        "$ref_mr" "$ref_mem" "$ref_clk")" || {
        end_fresh_ptb_gas_coin
        restore_wallet
        return 1
    }
    end_fresh_ptb_gas_coin
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    post_id="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post_id" ]] || post_id="$(extract_created_object_by_type "$digest" "Post")"
    [[ -n "$post_id" ]] || {
        echo "create_post did not produce a Post object" >&2
        return 1
    }
    normalize_hex_id "$post_id"
}

run_self_match_analyze_flow() {
    load_poc_oracle_session
    SOCIAL_RUN_ID="$(date +%s)"
    require_session_fields USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID \
        MEMORY_REGISTRY_ID MEMORY_CONFIG_ID POST_CONFIG_ID MYDATA_REGISTRY_ID \
        TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID \
        PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID \
        POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID CLOCK_ID || {
        echo "Run --refresh-session first" >&2
        return 1
    }
    local post1 post2 digest
    LAST_PTB_GAS_COIN_USED=''
    ensure_platform_ready || return 1
    step_creator_profile_and_join || return 1
    ensure_creator_wallet || return 1

    log_step "self-match 1/3 baseline original post + analyze"
    post1="$(create_poc_post_for_analyze "PoC self-match baseline ${SOCIAL_RUN_ID}")" || return 1
    digest="$(analyze_post_as_oracle "$post1" 1 50 none 0 false false 0)" || return 1
    assert_self_match_analyze_tx "$digest" || return 1

    log_step "self-match 2/3 second post analyzed with original_creator=post owner (score 100)"
    post2="$(create_poc_post_for_analyze "PoC self-match repost ${SOCIAL_RUN_ID}")" || return 1
    digest="$(analyze_post_as_oracle "$post2" 1 100 "some($CREATOR_ADDRESS)" 1 false false 0)" || return 1
    assert_self_match_analyze_tx "$digest" || return 1
    SELF_MATCH_ANALYZE_DIGEST="$digest"
    POST_ID="$(normalize_hex_id "$post2")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "SELF_MATCH_ANALYZE_DIGEST" "$SELF_MATCH_ANALYZE_DIGEST"
    save_poc_oracle_session

    log_step "self-match 3/3 verify GraphQL has no revenue redirect on repost"
    local gql_resp redirect
    gql_resp="$(gql_post_snapshot "$post2" 2>/dev/null)" || gql_resp='{}'
    redirect="$(echo "$gql_resp" | jq -r '.data.post.revenueRedirectTo // empty')"
    if [[ -n "$redirect" && "$redirect" != "null" ]]; then
        echo "self-match post $post2 unexpectedly has revenueRedirectTo=$redirect" >&2
        return 1
    fi

    print_run_summary_header "PoC Oracle Post — self-match analyze completed"
    print_run_summary_line "Baseline post" "$(normalize_hex_id "$post1")"
    print_run_summary_line "Self-match post" "$(normalize_hex_id "$post2")"
    print_run_summary_line "Creator" "$(normalize_hex_id "$CREATOR_ADDRESS")"
    print_run_summary_line "Analyze digest" "$SELF_MATCH_ANALYZE_DIGEST"
    print_run_summary_footer
}

ensure_beneficiary_vault_via_royalty_free_analyze() {
    local post_id="$1" owner="$2"
    local oracle ref_cfg ref_reg ref_vault ref_post ref_clk out vault_id
    post_id="$(normalize_hex_id "$post_id")" || return 1
    owner="$(normalize_hex_id "$owner")" || return 1
    require_session_fields POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID CLOCK_ID || return 1
    require_hex_ids POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID CLOCK_ID || return 1

    oracle="$(read_poc_oracle_address_from_graphql)" || oracle="$owner"
    switch_wallet "$oracle" || return 1
    ref_cfg="$(ptb_shared_ref "$POC_CONFIG_ID")" || { restore_wallet; return 1; }
    ref_reg="$(ptb_shared_ref "$POC_REGISTRY_ID")" || { restore_wallet; return 1; }
    ref_vault="$(ptb_shared_ref "$POC_VAULT_DIRECTORY_ID")" || { restore_wallet; return 1; }
    ref_post="$(ptb_shared_ref "$post_id")" || { restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_wallet; return 1; }

    log_step "Creating PoCBeneficiaryVault for $owner (royalty-free analyze on post $post_id)"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$oracle" \
        --move-call "${PKG_SOCIAL}::proof_of_creativity::analyze_and_update_post" \
        "$ref_cfg" "$ref_reg" "$ref_vault" "$ref_post" \
        1 0 none 0 false true 4 \
        none none "$ref_clk")" || {
        restore_wallet
        return 1
    }
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    vault_id="$(extract_created_object_by_type "$digest" "PoCBeneficiaryVault")" || true
    if [[ -z "$vault_id" ]]; then
        vault_id="$(gql_beneficiary_vault_id_for_address "$owner")" || true
    fi
    if [[ -z "$vault_id" ]]; then
        vault_id="$(lookup_beneficiary_vault_id_on_fullnode "$owner")" || true
    fi
    [[ -n "$vault_id" ]] && object_exists_on_fullnode "$vault_id" || {
        echo "royalty-free analyze did not create PoCBeneficiaryVault for $owner" >&2
        return 1
    }
    POC_BENEFICIARY_VAULT_ID="$(normalize_hex_id "$vault_id")"
    log_session_use "POC_BENEFICIARY_VAULT_ID" "$POC_BENEFICIARY_VAULT_ID"
    save_poc_oracle_session
    printf '%s' "$POC_BENEFICIARY_VAULT_ID"
}

resolve_beneficiary_vault_for_post_owner() {
    local owner="$1" post_id="${2:-}" vault_id
    owner="$(normalize_hex_id "$owner")" || return 1
    if [[ -n "${POC_BENEFICIARY_VAULT_ID:-}" ]] && object_exists_on_fullnode "$POC_BENEFICIARY_VAULT_ID"; then
        local ben
        if ben="$(parse_beneficiary_address_from_vault_object "$POC_BENEFICIARY_VAULT_ID" 2>/dev/null)"; then
            if [[ "$(normalize_hex_id "$ben")" == "$owner" ]]; then
                normalize_hex_id "$POC_BENEFICIARY_VAULT_ID"
                return 0
            fi
        fi
    fi
    vault_id="$(gql_beneficiary_vault_id_for_address "$owner")" || true
    if [[ -z "$vault_id" ]]; then
        vault_id="$(lookup_beneficiary_vault_id_on_fullnode "$owner")" || true
    fi
    if [[ -n "$vault_id" ]] && object_exists_on_fullnode "$vault_id"; then
        POC_BENEFICIARY_VAULT_ID="$(normalize_hex_id "$vault_id")"
        log_session_use "POC_BENEFICIARY_VAULT_ID" "$POC_BENEFICIARY_VAULT_ID"
        save_poc_oracle_session
        printf '%s' "$POC_BENEFICIARY_VAULT_ID"
        return 0
    fi
    [[ -n "$post_id" ]] || post_id="$(resolve_latest_post_id)" || return 1
    ensure_beneficiary_vault_via_royalty_free_analyze "$post_id" "$owner"
}

ensure_tipper_wallet() {
    local addr owner
    owner="$(normalize_hex_id "${CREATOR_ADDRESS:-$(resolve_myso_active_address)}")" || return 1
    if [[ -n "${TIPPER_ADDRESS:-}" ]]; then
        TIPPER_ADDRESS="$(normalize_hex_id "$TIPPER_ADDRESS")"
        if [[ "$TIPPER_ADDRESS" != "$owner" ]]; then
            ensure_wallet_funded "$TIPPER_ADDRESS" "$((DEFAULT_TIP_AMOUNT + SOCIAL_DEFAULT_GAS_BUDGET * 2))" || return 1
            log_session_use "TIPPER_ADDRESS" "$TIPPER_ADDRESS"
            return 0
        fi
        TIPPER_ADDRESS=''
    fi
    TIPPER_ADDRESS="$(create_ephemeral_wallet "poc_oracle_tipper_${SOCIAL_RUN_ID}")" || return 1
    ensure_wallet_funded "$TIPPER_ADDRESS" "$((DEFAULT_TIP_AMOUNT + SOCIAL_DEFAULT_GAS_BUDGET * 2))" || return 1
    log_session_use "TIPPER_ADDRESS" "$TIPPER_ADDRESS"
    save_poc_oracle_session
}

ensure_tipper_profile_and_join() {
    local lines profile_id mem username
    ensure_tipper_wallet || return 1
    switch_wallet "$TIPPER_ADDRESS" || return 1
    profile_id="$(resolve_owned_profile_for_address "$TIPPER_ADDRESS")" || profile_id=''
    if [[ -n "$profile_id" ]]; then
        mem="$(gql_profile_snapshot "$TIPPER_ADDRESS" | jq -r '.data.profile.memoryAccountId // empty')"
        [[ -n "$mem" ]] && TIPPER_MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    else
        username="tipper${SOCIAL_RUN_ID}"
        lines="$(create_profile_for_address "$TIPPER_ADDRESS" "PoC Oracle Tipper ${SOCIAL_RUN_ID}" "$username")" || {
            restore_wallet
            return 1
        }
        profile_id="$(echo "$lines" | sed -n '1p')"
        mem="$(echo "$lines" | sed -n '2p')"
        [[ -n "$mem" ]] && TIPPER_MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    fi
    [[ -n "${TIPPER_MEMORY_ACCOUNT_ID:-}" ]] || TIPPER_MEMORY_ACCOUNT_ID="$(gql_profile_snapshot "$TIPPER_ADDRESS" | jq -r '.data.profile.memoryAccountId // empty')"
    [[ -n "${TIPPER_MEMORY_ACCOUNT_ID:-}" ]] || {
        echo "Tipper MemoryAccount required" >&2
        restore_wallet
        return 1
    }
    ensure_joined_platform || { restore_wallet; return 1; }
    restore_wallet
    log_session_use "TIPPER_MEMORY_ACCOUNT_ID" "$TIPPER_MEMORY_ACCOUNT_ID"
    save_poc_oracle_session
}

reservation_pool_associated_post_id() {
    local pool_id="$1" json
    pool_id="$(normalize_hex_id "$pool_id")" || return 1
    json="$(myso client object "$pool_id" --json 2>/dev/null)" || return 1
    python3 - "$json" <<'PY'
import json, sys
j = json.loads(sys.argv[1])
contents = j.get("data", {}).get("Move", {}).get("contents")
if not contents or len(contents) < 64:
    raise SystemExit(1)
print("0x" + bytes(contents[32:64]).hex())
PY
}

extract_reservation_pool_from_digest() {
    local digest="$1" pool_id
    [[ -n "$digest" ]] || return 1
    pool_id="$(extract_created_object_by_type "$digest" "ReservationPoolObject")"
    [[ -n "$pool_id" ]] || pool_id="$(extract_created_object_by_type "$digest" "ReservationPool")"
    [[ -n "$pool_id" ]] || return 1
    normalize_hex_id "$pool_id"
}

gql_post_reservation_pool_id() {
    local post_id="$1" resp vars pool_id
    post_id="$(normalize_hex_id "$post_id")" || return 1
    vars="$(jq -nc --arg id "$post_id" '{id: $id}')"
    resp="$(graphql_post \
        'query PostReservationPool($id: ID!) { post(id: $id) { sptId enableSpt } }' \
        "$vars")" || return 1
    pool_id="$(echo "$resp" | jq -r '.data.post.sptId // empty')"
    [[ -n "$pool_id" ]] || return 1
    normalize_hex_id "$pool_id"
}

ensure_reservation_pool_for_post() {
    local post_id="$1" pool_id associated_post out digest
    local ref_token ref_spt ref_post ref_clk
    post_id="$(normalize_hex_id "$post_id")" || return 1
    if [[ -n "${RESERVATION_POOL_ID:-}" ]] && object_exists_on_fullnode "$RESERVATION_POOL_ID"; then
        associated_post="$(reservation_pool_associated_post_id "$RESERVATION_POOL_ID" 2>/dev/null)" || associated_post=''
        if [[ -n "$associated_post" ]] && [[ "$(normalize_hex_id "$associated_post")" == "$post_id" ]]; then
            log_step "Reusing reservation pool $RESERVATION_POOL_ID for post $post_id"
            return 0
        fi
        if [[ -n "$associated_post" ]]; then
            log_step "Session pool $RESERVATION_POOL_ID is for post $associated_post, not $post_id — resolving pool for $post_id"
        else
            log_step "Could not verify session pool $RESERVATION_POOL_ID for post $post_id — resolving pool for $post_id"
        fi
        RESERVATION_POOL_ID=''
    fi
    pool_id="$(gql_post_reservation_pool_id "$post_id" 2>/dev/null)" || pool_id=''
    if [[ -z "$pool_id" ]]; then
        require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID CLOCK_ID || return 1
        ref_token="$(ptb_shared_ref "$TOKEN_REGISTRY_ID")" || return 1
        ref_spt="$(ptb_shared_ref "$SOCIAL_PROOF_TOKENS_CONFIG_ID")" || return 1
        ref_post="$(ptb_shared_ref "$post_id")" || return 1
        ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
        switch_wallet "$CREATOR_ADDRESS" || return 1
        log_step "create_reservation_pool_for_post post=$post_id"
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
            --move-call "${PKG_SOCIAL}::social_proof_tokens::create_reservation_pool_for_post" \
            "$ref_token" "$ref_spt" "$ref_post" "$ref_clk")" || {
            restore_wallet
            return 1
        }
        restore_wallet
        digest="$(extract_tx_digest "$out")"
        pool_id="$(extract_reservation_pool_from_digest "$digest" 2>/dev/null)" || pool_id=''
    fi
    [[ -n "$pool_id" ]] || {
        echo "create_reservation_pool_for_post did not produce a reservation pool for post $post_id" >&2
        return 1
    }
    RESERVATION_POOL_ID="$(normalize_hex_id "$pool_id")"
    log_session_use "RESERVATION_POOL_ID" "$RESERVATION_POOL_ID"
    save_poc_oracle_session
}

refresh_poc_oracle_session_from_graphql() {
    social_refresh_session_from_graphql || return 1
    load_poc_oracle_session

    local json
    log_step "Refreshing PoC oracle extras from GraphQL ($GRAPHQL_URL)"
    json="$(graphql_post "$POC_ORACLE_GQL_EXTRAS")" || return 1

    POC_CONFIG_ID="$(gql_object_address "$json" pocConfig)"
    POC_REGISTRY_ID="$(gql_object_address "$json" pocRegistry)"
    POC_ADMIN_CAP_ID="$(gql_object_address "$json" proofOfCreativityAdminCap)"
    POC_VAULT_DIRECTORY_ID="$(gql_object_address "$json" pocVaultDirectory)"
    TOKEN_REGISTRY_ID="$(gql_object_address "$json" socialProofTokenRegistry)"
    SOCIAL_PROOF_TOKENS_CONFIG_ID="$(gql_object_address "$json" sptConfig)"
    SPT_ADMIN_CAP_ID="$(gql_object_address "$json" socialProofTokensAdminCap)"

    if [[ -z "${PLATFORM_OBJECT_ID:-}" ]]; then
        PLATFORM_OBJECT_ID="$(gql_object_address "$json" platform)"
    fi
    if [[ -n "${PLATFORM_OBJECT_ID:-}" ]] && ! object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
        echo "GraphQL PLATFORM_OBJECT_ID not on localnet fullnode; omitting from session." >&2
        PLATFORM_OBJECT_ID=''
    fi

    log_session_use "POC_CONFIG_ID" "$POC_CONFIG_ID"
    log_session_use "POC_REGISTRY_ID" "$POC_REGISTRY_ID"
    log_session_use "POC_ADMIN_CAP_ID" "$POC_ADMIN_CAP_ID"
    log_session_use "POC_VAULT_DIRECTORY_ID" "$POC_VAULT_DIRECTORY_ID"
    log_session_use "TOKEN_REGISTRY_ID" "$TOKEN_REGISTRY_ID"
    log_session_use "SOCIAL_PROOF_TOKENS_CONFIG_ID" "$SOCIAL_PROOF_TOKENS_CONFIG_ID"
    log_session_use "SPT_ADMIN_CAP_ID" "$SPT_ADMIN_CAP_ID"
    log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
    save_poc_oracle_session
}

gql_platform_approved() {
    local platform_id="$1" resp vars approved
    platform_id="$(normalize_hex_id "$platform_id")" || return 1
    vars="$(jq -nc --arg id "$platform_id" '{id: $id}')"
    resp="$(graphql_post \
        'query PlatformApproved($id: ID!) { platform(id: $id) { isApproved name links logo } }' \
        "$vars")" || return 1
    approved="$(echo "$resp" | jq -r '.data.platform.isApproved // false')"
    [[ "$approved" == "true" ]]
}

find_reusable_approved_platform() {
    local resp id
    resp="$(graphql_post \
        'query ApprovedPlatforms {
            platforms(approvedOnly: true, limit: 25) { id name isApproved links logo }
        }')" || return 1
    while IFS= read -r id; do
        [[ -n "$id" ]] || continue
        if object_exists_on_fullnode "$id" && gql_platform_approved "$id"; then
            normalize_hex_id "$id"
            return 0
        fi
    done < <(echo "$resp" | jq -r '.data.platforms[]?.id // empty')
    return 1
}

create_poc_oracle_platform() {
    local out digest platform_id ref_preg ref_pcfg ref_clk ref_cap
    require_session_fields PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID CLOCK_ID || return 1
    require_hex_ids PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID CLOCK_ID || return 1

    ref_preg="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_pcfg="$(ptb_shared_ref "$PLATFORM_CONFIG_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    ref_cap="$(ptb_shared_ref "$PLATFORM_ADMIN_CAP_ID")" || return 1

    log_step "Creating PoC oracle test platform (poc${SOCIAL_RUN_ID})"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --move-call "${PKG_SOCIAL}::platform::create_platform" \
        "$ref_preg" \
        "$ref_pcfg" \
        "$(literal_move_string "PoC Oracle Platform ${SOCIAL_RUN_ID}")" \
        "$(literal_move_string 'PoC oracle E2E test platform')" \
        "$(literal_move_string 'Platform for proof-of-creativity oracle post testing')" \
        "$(literal_move_string "$SOCIAL_DEFAULT_PLATFORM_LOGO_URL")" \
        "$(literal_move_string "${PLATFORM_WEBSITE_URL}/terms")" \
        "$(literal_move_string "${PLATFORM_WEBSITE_URL}/privacy")" \
        "$(literal_move_vector_empty)" \
        "$(literal_move_vector_from_csv "$PLATFORM_WEBSITE_URL")" \
        "$(literal_move_string 'Social Network')" \
        none 2 \
        "$(literal_move_string '2023-01-01')" \
        false \
        none none none none none none none \
        "$(literal_move_option_string "$SOCIAL_DEFAULT_PLATFORM_COVER_PHOTO_URL")" none \
        "$ref_clk")" || return 1

    digest="$(extract_tx_digest "$out")"
    platform_id="$(extract_created_object_by_type "$digest" "platform::Platform")"
    [[ -n "$platform_id" ]] || platform_id="$(extract_created_object_by_type "$digest" "Platform")"
    [[ -n "$platform_id" ]] || { echo "Could not find Platform from create tx" >&2; return 1; }

    log_step "Approving platform $platform_id"
    platform_id="$(normalize_hex_id "$platform_id")"
    SKIP_CONFIRM_RUN=1 invoke_ptb \
        --move-call "${PKG_SOCIAL}::platform::toggle_platform_approval" \
        "$ref_preg" \
        "$ref_pcfg" \
        "$(ptb_shared_ref "$platform_id")" \
        "$ref_cap" \
        none || return 1

    PLATFORM_OBJECT_ID="$(normalize_hex_id "$platform_id")"
    log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
}

ensure_platform_ready() {
    local reusable_id
    if [[ -n "${PLATFORM_OBJECT_ID:-}" ]] && object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
        if gql_platform_approved "$PLATFORM_OBJECT_ID"; then
            log_step "Reusing session platform $(normalize_hex_id "$PLATFORM_OBJECT_ID")"
            return 0
        fi
        echo "Session PLATFORM_OBJECT_ID exists but is not approved; searching for another." >&2
        PLATFORM_OBJECT_ID=''
    fi

    if reusable_id="$(find_reusable_approved_platform)"; then
        PLATFORM_OBJECT_ID="$(normalize_hex_id "$reusable_id")"
        log_step "Reusing approved platform from GraphQL: $PLATFORM_OBJECT_ID"
        log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
        save_poc_oracle_session
        return 0
    fi

    log_step "No approved platform on fullnode — creating and approving a new one"
    create_poc_oracle_platform || return 1
    save_poc_oracle_session
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
    username="pocoracle${SOCIAL_RUN_ID}"
    switch_wallet "$CREATOR_ADDRESS" || return 1
    profile_id_existing="$(resolve_owned_profile_for_address "$CREATOR_ADDRESS")" || profile_id_existing=''
    if [[ -n "$profile_id_existing" ]]; then
        CREATOR_PROFILE_ID="$(normalize_hex_id "$profile_id_existing")"
        snap="$(gql_profile_snapshot "$CREATOR_ADDRESS" 2>/dev/null)" || snap='{}'
        mem="$(echo "$snap" | jq -r '.data.profile.memoryAccountId // empty')"
        [[ -n "$mem" ]] && MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
        log_step "Reusing creator profile $CREATOR_PROFILE_ID"
    else
        lines="$(create_profile_for_address "$CREATOR_ADDRESS" "PoC Oracle Creator ${SOCIAL_RUN_ID}" "$username")" || {
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
    save_poc_oracle_session
}

step_create_poc_post() {
    local out digest body_lit media_opt pool_id
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    local ref_token ref_spt ref_post ref_clk_pool

    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1

    body_lit="$(literal_move_string "PoC oracle test post ${SOCIAL_RUN_ID}")"
    media_opt="some(vector[$(literal_move_string "$POST_MEDIA_URL")])"

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
    log_step "create_post enable_spt=true media=$POST_MEDIA_URL"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::create_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" \
        "$media_opt" \
        none none none none none none none \
        some\(true\) none none \
        "$ref_mr" "$ref_mem" "$ref_clk")" || {
        restore_wallet
        return 1
    }
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    LAST_TX_DIGEST="$digest"
    wait_for_tx_finalized "$digest" || return 1
    assert_poc_scenario_events post_created "$digest" || return 1
    POST_ID="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$POST_ID" ]] || POST_ID="$(extract_created_object_by_type "$digest" "Post")"
    [[ -n "$POST_ID" ]] || {
        echo "create_post did not produce a Post object" >&2
        return 1
    }
    POST_ID="$(normalize_hex_id "$POST_ID")"

    ref_token="$(ptb_shared_ref "$TOKEN_REGISTRY_ID")" || return 1
    ref_spt="$(ptb_shared_ref "$SOCIAL_PROOF_TOKENS_CONFIG_ID")" || return 1
    ref_post="$(ptb_shared_ref "$POST_ID")" || return 1
    ref_clk_pool="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    switch_wallet "$CREATOR_ADDRESS" || return 1
    log_step "create_reservation_pool_for_post post=$POST_ID"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$CREATOR_ADDRESS" \
        --move-call "${PKG_SOCIAL}::social_proof_tokens::create_reservation_pool_for_post" \
        "$ref_token" "$ref_spt" "$ref_post" "$ref_clk_pool")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    assert_poc_scenario_events reservation_pool_created "$digest" || return 1
    pool_id="$(extract_reservation_pool_from_digest "$digest" 2>/dev/null)" || pool_id=''
    [[ -n "$pool_id" ]] || {
        echo "create_reservation_pool_for_post did not produce ReservationPoolObject" >&2
        return 1
    }
    RESERVATION_POOL_ID="$(normalize_hex_id "$pool_id")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "RESERVATION_POOL_ID" "$RESERVATION_POOL_ID"
    log_session_use "LAST_TX_DIGEST" "$LAST_TX_DIGEST"
    save_poc_oracle_session
}

gql_post_snapshot() {
    local post_id="$1" resp vars
    post_id="$(normalize_hex_id "$post_id")" || return 1
    vars="$(jq -nc --arg id "$post_id" '{id: $id}')"
    resp="$(graphql_post \
        'query PostSnapshot($id: ID!) {
            post(id: $id) {
                id
                content
                enableSpt
                mediaUrls
                platformId
                owner
                createdAt
                tipsReceived
                totalTipVolume
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

wait_for_gql_post() {
    local post_id="$1" attempt resp post_key
    post_id="$(normalize_hex_id "$post_id")" || return 1
    post_key="$(normalize_hex_id "$post_id" | tr '[:upper:]' '[:lower:]' | sed 's/^0x//')"
    for attempt in $(seq 1 30); do
        resp="$(gql_post_snapshot "$post_id" 2>/dev/null)" || resp='{}'
        if [[ -n "$(echo "$resp" | jq -r '.data.post.id // empty')" ]]; then
            printf '%s' "$resp"
            return 0
        fi
        if [[ $((attempt % 5)) -eq 0 ]]; then
            echo "  waiting for GraphQL indexer (attempt ${attempt}/30, post=${post_key:0:12}...)..." >&2
        fi
        sleep 1
    done
    echo "GraphQL indexer has not caught up yet for post $post_id (on-chain tx succeeded)." >&2
    return 1
}

assert_poc_post_graphql() {
    local resp media_urls platform_id media_match
    log_step "GraphQL assert PoC post indexed with media"
    if ! resp="$(wait_for_gql_post "$POST_ID")"; then
        GQL_INDEXED='false'
        return 0
    fi

    media_urls="$(echo "$resp" | jq -c '.data.post.mediaUrls // null')"
    platform_id="$(echo "$resp" | jq -r '.data.post.platformId // empty')"

    if [[ "$(echo "$resp" | jq -r '.data.post.enableSpt // false')" != "true" ]]; then
        echo "Warning: post.enableSpt expected true got $(echo "$resp" | jq -r '.data.post.enableSpt // false')" >&2
        GQL_INDEXED='false'
        POST_GQL_SNAPSHOT="$resp"
        return 0
    fi
    media_match="$(echo "$media_urls" | jq -r --arg url "$POST_MEDIA_URL" '
        if type == "array" then any(. == $url)
        elif type == "string" then . == $url
        else false end
    ')"
    if [[ "$media_match" != "true" ]]; then
        echo "Warning: post.mediaUrls expected to include media URL; got: $media_urls" >&2
        GQL_INDEXED='false'
        POST_GQL_SNAPSHOT="$resp"
        return 0
    fi
    if [[ "$(normalize_hex_id "$platform_id")" != "$(normalize_hex_id "$PLATFORM_OBJECT_ID")" ]]; then
        echo "Warning: post.platformId expected $PLATFORM_OBJECT_ID got $platform_id" >&2
        GQL_INDEXED='false'
        POST_GQL_SNAPSHOT="$resp"
        return 0
    fi
    log_step "GraphQL PoC post assertions passed"
    POST_GQL_SNAPSHOT="$resp"
    GQL_INDEXED='true'
}

print_poc_oracle_post_summary() {
    local gql_resp="${POST_GQL_SNAPSHOT:-}"
    local content media_urls owner created_at gql_status outcome
    [[ -n "$gql_resp" ]] || gql_resp="$(gql_post_snapshot "$POST_ID" 2>/dev/null)" || gql_resp='{}'

    content="$(echo "$gql_resp" | jq -r '.data.post.content // empty')"
    media_urls="$(echo "$gql_resp" | jq -c '.data.post.mediaUrls // null')"
    owner="$(echo "$gql_resp" | jq -r '.data.post.owner // empty')"
    created_at="$(echo "$gql_resp" | jq -r '.data.post.createdAt // empty')"
    gql_status="${GQL_INDEXED:-false}"

    print_run_summary_header "PoC Oracle Post E2E — completed"
    print_run_summary_line "Run ID" "$SOCIAL_RUN_ID"
    print_run_summary_line "On-chain status" "SUCCESS (post created)"
    print_run_summary_line "GraphQL indexed" "$gql_status"
    print_run_summary_line "Transaction digest" "${LAST_TX_DIGEST:-<unknown>}"
    print_run_summary_line "Post object ID" "$(normalize_hex_id "${POST_ID:-}")"
    print_run_summary_line "Platform object ID" "$(normalize_hex_id "${PLATFORM_OBJECT_ID:-}")"
    print_run_summary_line "Creator wallet" "$(normalize_hex_id "${CREATOR_ADDRESS:-}")"
    print_run_summary_line "Creator profile" "$(normalize_hex_id "${CREATOR_PROFILE_ID:-}")"
    print_run_summary_line "Memory account" "$(normalize_hex_id "${MEMORY_ACCOUNT_ID:-}")"
    print_run_summary_line "PoC config" "$(normalize_hex_id "${POC_CONFIG_ID:-}")"
    print_run_summary_line "PoC registry" "$(normalize_hex_id "${POC_REGISTRY_ID:-}")"
    print_run_summary_line "Post content" "${content:-PoC oracle test post ${SOCIAL_RUN_ID}}"
    print_run_summary_line "enableSpt (on-chain)" "true"
    print_run_summary_line "enableSpt (GraphQL)" "$(echo "$gql_resp" | jq -r '.data.post.enableSpt // pending')"
    print_run_summary_line "Reservation pool" "$(normalize_hex_id "${RESERVATION_POOL_ID:-}")"
    print_run_summary_line "Media URL" "$POST_MEDIA_URL"
    print_run_summary_line "mediaUrls (GraphQL)" "${media_urls:-pending}"
    print_run_summary_line "Post owner (GraphQL)" "${owner:-pending}"
    print_run_summary_line "createdAt (GraphQL)" "${created_at:-pending}"
    print_run_summary_line "GraphQL endpoint" "$GRAPHQL_URL"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_line "Oracle config" "../proof-of-creativity/config/networks/localnet.yaml"
    print_run_summary_line "localnet.yaml hints" "objects.poc_config / poc_registry / username_registry / clock"
    outcome="Post $(normalize_hex_id "${POST_ID:-}") was created on-chain with SPT enabled and media attached. "
    if [[ "$gql_status" == "true" ]]; then
        outcome+="GraphQL indexer confirmed enableSpt and mediaUrls. "
    else
        outcome+="GraphQL indexer has not confirmed the post yet (oracle can still read chain events). "
    fi
    outcome+="Use POST_ID in your PoC oracle run; update localnet.yaml object IDs from the session file above."
    print_run_summary_line "Outcome" "$outcome"
    print_run_summary_footer

    echo "" >&2
    echo "── PoC oracle quick reference (copy into localnet.yaml objects) ──" >&2
    printf '  poc_config: "%s"\n' "$(normalize_hex_id "${POC_CONFIG_ID:-}")" >&2
    printf '  poc_registry: "%s"\n' "$(normalize_hex_id "${POC_REGISTRY_ID:-}")" >&2
    printf '  username_registry: "%s"\n' "$(normalize_hex_id "${USERNAME_REGISTRY_ID:-}")" >&2
    printf '  clock: "%s"\n' "$(normalize_hex_id "${CLOCK_ID:-}")" >&2
    echo "" >&2
    echo "── Target post for oracle analysis ──" >&2
    printf '  post_id: %s\n' "$(normalize_hex_id "${POST_ID:-}")" >&2
    printf '  tx_digest: %s\n' "${LAST_TX_DIGEST:-}" >&2
    printf '  media_url: %s\n' "$POST_MEDIA_URL" >&2
    echo "" >&2
}

step_tip_latest_post() {
    local post_id tip_coin gas_coin saved_gas owner vault_id ref_post ref_vault ref_tip ref_mem ref_clk out digest
    post_id="$(resolve_latest_post_id)" || return 1
    owner="$(object_address_owner "$post_id" 2>/dev/null)" || owner="${CREATOR_ADDRESS:-}"
    [[ -n "$owner" ]] || { echo "Could not resolve post owner for $post_id" >&2; return 1; }
    CREATOR_ADDRESS="$(normalize_hex_id "$owner")"
    ensure_platform_ready || return 1
    ensure_tipper_profile_and_join || return 1
    if [[ "$(normalize_hex_id "$TIPPER_ADDRESS")" == "$(normalize_hex_id "$CREATOR_ADDRESS")" ]]; then
        echo "Self-tipping is forbidden; tipper must differ from post owner $CREATOR_ADDRESS" >&2
        return 1
    fi
    vault_id="$(resolve_beneficiary_vault_for_post_owner "$CREATOR_ADDRESS" "$post_id")" || return 1
    POC_BENEFICIARY_VAULT_ID="$(normalize_hex_id "$vault_id")"

    switch_wallet "$TIPPER_ADDRESS" || return 1
    read -r tip_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$TIPPER_ADDRESS" "$DEFAULT_TIP_AMOUNT")" || {
        restore_wallet
        return 1
    }
    ref_post="$(ptb_shared_ref "$post_id")" || { restore_wallet; return 1; }
    ref_vault="$(ptb_shared_ref "$vault_id")" || { restore_wallet; return 1; }
    ref_tip="$(ptb_shared_ref "$tip_coin")" || { restore_wallet; return 1; }
    ref_mem="$(ptb_shared_ref "$TIPPER_MEMORY_ACCOUNT_ID")" || { restore_wallet; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_wallet; return 1; }
    saved_gas="${PTB_GAS_COIN_ID:-}"
    PTB_GAS_COIN_ID="$gas_coin"

    log_step "tip_post amount=$DEFAULT_TIP_AMOUNT post=$post_id tipper=$TIPPER_ADDRESS vault=$vault_id"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$TIPPER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::tip_post<${COIN_TYPE}>" \
        "$ref_post" "$ref_vault" "$ref_tip" "$DEFAULT_TIP_AMOUNT" "$POC_MIN_VAULT_DEPOSIT" \
        "$ref_mem" "$ref_clk")" || {
        PTB_GAS_COIN_ID="$saved_gas"
        restore_wallet
        return 1
    }
    PTB_GAS_COIN_ID="$saved_gas"
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    assert_poc_scenario_events tip_post "$digest" || return 1
    LAST_TIP_TX_DIGEST="$digest"
    POST_ID="$(normalize_hex_id "$post_id")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "LAST_TIP_TX_DIGEST" "$LAST_TIP_TX_DIGEST"
    save_poc_oracle_session
}

step_reserve_into_latest_post() {
    local post_id owner vault_id reserve_amount pay_amount
    local ref_token ref_spt ref_pool ref_treasury ref_preg ref_plat ref_blr ref_post ref_vault ref_clk out digest
    post_id="$(resolve_latest_post_id)" || return 1
    owner="$(object_address_owner "$post_id" 2>/dev/null)" || owner="${CREATOR_ADDRESS:-}"
    [[ -n "$owner" ]] || { echo "Could not resolve post owner for $post_id" >&2; return 1; }
    CREATOR_ADDRESS="$(normalize_hex_id "$owner")"

    require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID ECOSYSTEM_TREASURY_ID \
        PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID BLOCK_LIST_REGISTRY_ID CLOCK_ID \
        POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID || {
        echo "Run --refresh-session first (SPT + platform + PoC ids required)" >&2
        return 1
    }

    ensure_platform_ready || return 1
    ensure_spt_trading_enabled || return 1
    ensure_reservation_pool_for_post "$post_id" || return 1
    vault_id="$(resolve_beneficiary_vault_for_post_owner "$CREATOR_ADDRESS" "$post_id")" || return 1
    POC_BENEFICIARY_VAULT_ID="$(normalize_hex_id "$vault_id")"
    log_session_use "POC_BENEFICIARY_VAULT_ID" "$POC_BENEFICIARY_VAULT_ID"
    ensure_tipper_profile_and_join || return 1

    reserve_amount="${RESERVE_AMOUNT:-$DEFAULT_RESERVE_AMOUNT}"
    pay_amount="${RESERVE_PAY_AMOUNT:-$(( reserve_amount + 200000000 ))}"

    ref_token="$(ptb_shared_ref "$TOKEN_REGISTRY_ID")" || return 1
    ref_spt="$(ptb_shared_ref "$SOCIAL_PROOF_TOKENS_CONFIG_ID")" || return 1
    ref_pool="$(ptb_shared_ref "$RESERVATION_POOL_ID")" || return 1
    ref_treasury="$(ptb_shared_ref "$ECOSYSTEM_TREASURY_ID")" || return 1
    ref_preg="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || return 1
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || return 1
    ref_post="$(ptb_shared_ref "$post_id")" || return 1
    ref_vault="$(ptb_shared_ref "$vault_id")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1

    switch_wallet "$TIPPER_ADDRESS" || return 1
    log_step "reserve_towards_post_with_platform amount=$reserve_amount post=$post_id reserver=$TIPPER_ADDRESS pool=$RESERVATION_POOL_ID"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$TIPPER_ADDRESS" \
        --split-coins gas "[${pay_amount}]" \
        --assign pay_coin \
        --move-call "${PKG_SOCIAL}::social_proof_tokens::reserve_towards_post_with_platform" \
        "$ref_token" "$ref_spt" "$POC_MIN_VAULT_DEPOSIT" \
        "$ref_pool" "$ref_treasury" "$ref_preg" "$ref_plat" "$ref_blr" \
        "$ref_post" "$ref_vault" pay_coin.0 "$reserve_amount" "$ref_clk")" || {
        restore_wallet
        return 1
    }
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    assert_poc_scenario_events spt_reserve "$digest" || return 1
    LAST_RESERVE_TX_DIGEST="$digest"
    POST_ID="$(normalize_hex_id "$post_id")"
    log_session_use "POST_ID" "$POST_ID"
    log_session_use "RESERVATION_POOL_ID" "$RESERVATION_POOL_ID"
    log_session_use "LAST_RESERVE_TX_DIGEST" "$LAST_RESERVE_TX_DIGEST"
    save_poc_oracle_session
}

print_tip_post_summary() {
    local gql_resp tips total
    gql_resp="$(gql_post_snapshot "$POST_ID" 2>/dev/null)" || gql_resp='{}'
    tips="$(echo "$gql_resp" | jq -r '.data.post.tipsReceived // empty')"
    total="$(echo "$gql_resp" | jq -r '.data.post.totalTipVolume // empty')"
    print_run_summary_header "PoC Oracle Post — tip completed"
    print_run_summary_line "Post" "$(normalize_hex_id "$POST_ID")"
    print_run_summary_line "Tipper" "$(normalize_hex_id "$TIPPER_ADDRESS")"
    print_run_summary_line "Tip amount" "$(format_mist_with_units "$DEFAULT_TIP_AMOUNT")"
    print_run_summary_line "Transaction digest" "${LAST_TIP_TX_DIGEST:-<unknown>}"
    print_run_summary_line "tipsReceived (GraphQL)" "${tips:-pending}"
    print_run_summary_line "totalTipVolume (GraphQL)" "${total:-pending}"
    print_run_summary_footer
}

print_reserve_post_summary() {
    print_run_summary_header "PoC Oracle Post — SPT reservation completed"
    print_run_summary_line "Post" "$(normalize_hex_id "$POST_ID")"
    print_run_summary_line "Reservation pool" "$(normalize_hex_id "$RESERVATION_POOL_ID")"
    print_run_summary_line "Reserver" "$(normalize_hex_id "$TIPPER_ADDRESS")"
    print_run_summary_line "Reserve amount" "$(format_mist_with_units "${RESERVE_AMOUNT:-$DEFAULT_RESERVE_AMOUNT}")"
    print_run_summary_line "Transaction digest" "${LAST_RESERVE_TX_DIGEST:-<unknown>}"
    print_run_summary_line "Beneficiary vault" "$(normalize_hex_id "${POC_BENEFICIARY_VAULT_ID:-}")"
    print_run_summary_footer
}

run_tip_latest_post_flow() {
    load_poc_oracle_session
    SOCIAL_RUN_ID="$(date +%s)"
    [[ -n "${CREATOR_ADDRESS:-}" ]] || ensure_creator_wallet || return 1
    step_tip_latest_post || return 1
    print_tip_post_summary
}

run_reserve_latest_post_flow() {
    load_poc_oracle_session
    SOCIAL_RUN_ID="$(date +%s)"
    [[ -n "${CREATOR_ADDRESS:-}" ]] || ensure_creator_wallet || return 1
    step_reserve_into_latest_post || return 1
    print_reserve_post_summary
}

run_poc_oracle_post_flow() {
    load_poc_oracle_session
    SOCIAL_RUN_ID="$(date +%s)"
    require_session_fields USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID \
        MEMORY_REGISTRY_ID MEMORY_CONFIG_ID POST_CONFIG_ID MYDATA_REGISTRY_ID \
        TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID \
        PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID || {
        echo "Run --refresh-session first" >&2
        return 1
    }

    ensure_platform_ready || return 1
    step_creator_profile_and_join || return 1
    step_create_poc_post || return 1
    assert_poc_post_graphql
    save_poc_oracle_session
    print_poc_oracle_post_summary
}

show_menu() {
    echo ""
    echo "=== PoC Oracle Post E2E Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Create PoC+SPT post (--run-all)"
    echo " 2) Tip latest post"
    echo " 3) Reserve SPT into latest post"
    echo " 4) Self-match analyze (same creator, no redirect)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) refresh_poc_oracle_session_from_graphql; load_poc_oracle_session ;;
        1) run_poc_oracle_post_flow ;;
        2) run_tip_latest_post_flow ;;
        3) run_reserve_latest_post_flow ;;
        4) run_self_match_analyze_flow ;;
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
            --tip-post) RUN_MODE=tip_post; shift ;;
            --reserve-post) RUN_MODE=reserve_post; shift ;;
            --self-match-analyze) RUN_MODE=self_match_analyze; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    load_poc_oracle_session

    case "${RUN_MODE:-}" in
        refresh) refresh_poc_oracle_session_from_graphql; load_poc_oracle_session; exit 0 ;;
        run_all) run_poc_oracle_post_flow; exit 0 ;;
        tip_post) run_tip_latest_post_flow; exit 0 ;;
        reserve_post) run_reserve_latest_post_flow; exit 0 ;;
        self_match_analyze) run_self_match_analyze_flow; exit 0 ;;
        '') show_menu ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
