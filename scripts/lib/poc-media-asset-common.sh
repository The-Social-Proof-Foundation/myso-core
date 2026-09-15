#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared MediaAsset-centric PoC helpers for myso-core runnable scripts.
# Source after social-runtime-common.sh (and preferably poc-oracle-common.sh).

: "${POC_POST_FIRST:=1}"
: "${POC_DEFAULT_MEDIA_URL:=https://pub-b2100065e2e5444db484a5fed5d87096.r2.dev/image/2026/05/23a44811-b991-418e-88ee-b99c3973fa91.png}"
: "${POC_COMPOSITION_TIMEOUT_SEC:=180}"

poc_media_run_id() {
    printf '%s' "${SOCIAL_RUN_ID:-${POC_RUN_ID:-$(date +%s)}}"
}

poc_media_repo() {
    if declare -F poc_oracle_resolve_repo >/dev/null 2>&1; then
        poc_oracle_resolve_repo
        return 0
    fi
    if [[ -n "${MYSO_POC_REPO:-}" ]]; then
        printf '%s' "$MYSO_POC_REPO"
        return 0
    fi
    printf '%s' "${REPO_ROOT}/../proof-of-creativity"
}

literal_move_hex_bytes() {
    local hex="$1"
    hex="${hex#0x}"
    printf 'x"%s"' "$hex"
}

poc_id_vector_literal() {
    local acc="" sep="" id
    if [[ $# -eq 0 || -z "${1:-}" ]]; then
        printf 'vector[]'
        return 0
    fi
    for id in "$@"; do
        id="$(normalize_hex_id "$id")" || continue
        acc="${acc}${sep}@${id}"
        sep=","
    done
    if [[ -z "$acc" ]]; then
        printf 'vector[]'
        return 0
    fi
    printf 'vector[%s]' "$acc"
}

deterministic_commitments_for_run() {
    local run_id="${1:-$(poc_media_run_id)}"
    python3 - "$run_id" <<'PY'
import hashlib, sys
run_id = sys.argv[1].encode()
print(hashlib.sha256(b"content:" + run_id).hexdigest())
print(hashlib.sha256(b"fingerprint:" + run_id).hexdigest())
PY
}

poc_build_post_metadata_json() {
    local content_hex="$1" fingerprint_hex="$2"
    content_hex="${content_hex#0x}"
    fingerprint_hex="${fingerprint_hex#0x}"
    jq -nc --arg content "$content_hex" --arg fingerprint "$fingerprint_hex" \
        '{content_commitment: $content, observed_fingerprint_commitment: $fingerprint}'
}

poc_python() {
    local poc_repo
    poc_repo="$(poc_media_repo)"
    [[ -d "$poc_repo" ]] || {
        echo "proof-of-creativity repo not found at $poc_repo (set MYSO_POC_REPO)" >&2
        return 1
    }
    (
        cd "$poc_repo"
        export MYSO_POC_PACKAGE_ID="${PKG_SOCIAL:-${MYSO_POC_PACKAGE_ID:-}}"
        export MYSO_PLATFORM_PACKAGE_ADDRESS="${PKG_SOCIAL:-${MYSO_PLATFORM_PACKAGE_ADDRESS:-0x50c1}}"
        export MYSO_POC_CONFIG_ID="${POC_CONFIG_ID:-${MYSO_POC_CONFIG_ID:-}}"
        export MYSO_POC_REGISTRY_ID="${POC_REGISTRY_ID:-${MYSO_POC_REGISTRY_ID:-}}"
        export MYSO_POC_VAULT_DIRECTORY_ID="${POC_VAULT_DIRECTORY_ID:-${MYSO_POC_VAULT_DIRECTORY_ID:-}}"
        export MYSO_TOKEN_REGISTRY_ID="${TOKEN_REGISTRY_ID:-${MYSO_TOKEN_REGISTRY_ID:-}}"
        export POC_GOVERNANCE_REGISTRY_ID="${POC_GOVERNANCE_REGISTRY_ID:-}"
        export MYSO_ECOSYSTEM_TREASURY_ID="${ECOSYSTEM_TREASURY_ID:-${MYSO_ECOSYSTEM_TREASURY_ID:-}}"
        export MYSO_CLOCK_OBJECT_ID="${CLOCK_ID:-${MYSO_CLOCK_OBJECT_ID:-0x6}}"
        export GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
        export POC_ORACLE_NETWORK="${POC_ORACLE_NETWORK:-localnet}"
        export MYSO_INTEGRATION_ENABLED="${MYSO_INTEGRATION_ENABLED:-true}"
        PYTHONPATH="${poc_repo}${PYTHONPATH:+:$PYTHONPATH}" python3 "$@"
    )
}

extract_request_id_from_digest() {
    local digest="$1" req_id
    req_id="$(extract_created_object_by_type "$digest" "MediaResolutionRequest")"
    [[ -n "$req_id" ]] || req_id="$(extract_event_field "$digest" MediaResolutionRequestedEvent request_id 2>/dev/null || true)"
    [[ -n "$req_id" ]] || return 1
    normalize_hex_id "$req_id"
}

extract_media_asset_from_digest() {
    local digest="$1" asset_id
    asset_id="$(extract_created_object_by_type "$digest" "media_asset::MediaAsset")"
    [[ -n "$asset_id" ]] || asset_id="$(extract_created_object_by_type "$digest" "MediaAsset")"
    [[ -n "$asset_id" ]] || asset_id="$(extract_event_field "$digest" MediaAssetResolvedEvent media_asset_id 2>/dev/null || true)"
    [[ -n "$asset_id" ]] || return 1
    normalize_hex_id "$asset_id"
}

wait_for_gql_media_asset_by_submitter() {
    local owner="$1" attempt resp asset_id
    owner="$(normalize_hex_id "$owner")" || return 1
    for attempt in $(seq 1 45); do
        resp="$(graphql_post \
            'query MediaAssets {
                objects(filter: { type: "0x50c1::media_asset::MediaAsset", ownerKind: SHARED }, first: 50) {
                    nodes { address }
                }
            }' \
            '{}')" || resp='{}'
        while IFS= read -r asset_id; do
            [[ -n "$asset_id" ]] || continue
            if object_exists_on_fullnode "$asset_id"; then
                normalize_hex_id "$asset_id"
                return 0
            fi
        done < <(echo "$resp" | jq -r '.data.objects.nodes[]?.address // empty')
        sleep 1
    done
    return 1
}

gql_post_composition_snapshot() {
    local post_id="$1" vars
    post_id="$(normalize_hex_id "$post_id")" || return 1
    vars="$(jq -nc --arg id "$post_id" '{id: $id}')"
    graphql_post \
        'query PostComposition($id: ID!) {
            post(id: $id) {
                id
                mediaAssetIds
                compositionStatus
                monetizationStatus
                enableSpt
            }
        }' \
        "$vars"
}

wait_for_gql_post_composition() {
    local post_id="$1" expected="${2:-}"
    local timeout_sec="${3:-$POC_COMPOSITION_TIMEOUT_SEC}"
    local deadline resp status ids
    post_id="$(normalize_hex_id "$post_id")" || return 1
    deadline=$(( $(date +%s) + timeout_sec ))
    while [[ $(date +%s) -lt "$deadline" ]]; do
        resp="$(gql_post_composition_snapshot "$post_id" 2>/dev/null)" || resp='{}'
        status="$(echo "$resp" | jq -r '.data.post.compositionStatus // empty')"
        ids="$(echo "$resp" | jq -c '.data.post.mediaAssetIds // empty')"
        if [[ -n "$expected" ]]; then
            if [[ "$status" == "$expected" ]]; then
                printf '%s' "$resp"
                return 0
            fi
        elif [[ -n "$status" && "$status" != "NONE" && "$status" != "null" ]]; then
            printf '%s' "$resp"
            return 0
        fi
        if [[ -n "$ids" && "$ids" != "null" && "$ids" != "[]" && "$ids" != '""' ]]; then
            if [[ -z "$expected" ]]; then
                printf '%s' "$resp"
                return 0
            fi
        fi
        sleep 2
    done
    echo "Timed out waiting for GraphQL composition on post $post_id (last status=${status:-<none>})" >&2
    return 1
}

poc_first_media_asset_id_from_gql() {
    local post_id="$1" resp asset_id
    resp="$(gql_post_composition_snapshot "$post_id" 2>/dev/null)" || resp='{}'
    asset_id="$(echo "$resp" | jq -r '
        .data.post.mediaAssetIds
        | if type == "array" then .[0]
          elif type == "string" then .
          else empty end
    ')"
    [[ -n "$asset_id" && "$asset_id" != "null" ]] || return 1
    normalize_hex_id "$asset_id"
}

wait_for_post_media_asset_id() {
    local post_id="$1" timeout_sec="${2:-$POC_COMPOSITION_TIMEOUT_SEC}"
    local deadline asset_id
    post_id="$(normalize_hex_id "$post_id")" || return 1
    deadline=$(( $(date +%s) + timeout_sec ))
    while [[ $(date +%s) -lt "$deadline" ]]; do
        asset_id="$(poc_first_media_asset_id_from_gql "$post_id" 2>/dev/null)" || asset_id=''
        if [[ -n "$asset_id" ]]; then
            MEDIA_ASSET_ID="$asset_id"
            printf '%s' "$asset_id"
            return 0
        fi
        sleep 2
    done
    echo "Timed out waiting for MediaAsset id on post $post_id" >&2
    return 1
}

create_post_post_first() {
    local sender="$1" body_lit="$2" media_url="$3" content_hex="$4" fingerprint_hex="$5"
    local enable_spt_arg="${6:-none}"
    local metadata_json metadata_opt media_urls_opt
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    local out

    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1

    [[ -n "$media_url" ]] || media_url="$POC_DEFAULT_MEDIA_URL"
    if [[ -z "$content_hex" || -z "$fingerprint_hex" ]]; then
        mapfile -t _commits < <(deterministic_commitments_for_run)
        content_hex="${content_hex:-${_commits[0]}}"
        fingerprint_hex="${fingerprint_hex:-${_commits[1]}}"
    fi
    export POC_CONTENT_HEX="${content_hex#0x}"
    export POC_FINGERPRINT_HEX="${fingerprint_hex#0x}"

    metadata_json="$(poc_build_post_metadata_json "$content_hex" "$fingerprint_hex")" || return 1
    metadata_opt="some($(literal_move_string "$metadata_json"))"
    media_urls_opt="some(vector[$(literal_move_string "$media_url")])"

    ref_ur="$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" || return 1
    ref_pr="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || return 1
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || return 1
    ref_cfg="$(ptb_shared_ref "$POST_CONFIG_ID")" || return 1
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || return 1
    ref_mr="$(ptb_shared_ref "$MYDATA_REGISTRY_ID")" || return 1
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1

    log_step "create_post post-first media=$media_url enable_spt=$enable_spt_arg"
    if [[ -n "$sender" ]]; then
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" \
            --move-call "${PKG_SOCIAL}::post::create_post" \
            "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
            "$body_lit" \
            "$(poc_id_vector_literal)" \
            "$media_urls_opt" \
            none \
            "$metadata_opt" \
            none none none none none \
            "$enable_spt_arg" none 1 none none none \
            "$ref_mr" "$ref_mem" "$ref_clk")" || return 1
    else
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
            --move-call "${PKG_SOCIAL}::post::create_post" \
            "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
            "$body_lit" \
            "$(poc_id_vector_literal)" \
            "$media_urls_opt" \
            none \
            "$metadata_opt" \
            none none none none none \
            "$enable_spt_arg" none 1 none none none \
            "$ref_mr" "$ref_mem" "$ref_clk")" || return 1
    fi
    printf '%s' "$out"
}

create_post_with_assets() {
    local sender="$1" body_lit="$2" enable_spt_arg="$3"
    shift 3
    local -a asset_ids=("$@")
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk
    local asset_lit out

    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1

    asset_lit="$(poc_id_vector_literal "${asset_ids[@]}")"
    ref_ur="$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" || return 1
    ref_pr="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || return 1
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || return 1
    ref_cfg="$(ptb_shared_ref "$POST_CONFIG_ID")" || return 1
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || return 1
    ref_mr="$(ptb_shared_ref "$MYDATA_REGISTRY_ID")" || return 1
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1

    log_step "create_post with media_asset_ids=$asset_lit enable_spt=$enable_spt_arg"
    if [[ -n "$sender" ]]; then
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" \
            --move-call "${PKG_SOCIAL}::post::create_post" \
            "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
            "$body_lit" \
            "$asset_lit" \
            none none none \
            none none none none none \
            "$enable_spt_arg" none 1 none none none \
            "$ref_mr" "$ref_mem" "$ref_clk")" || return 1
    else
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
            --move-call "${PKG_SOCIAL}::post::create_post" \
            "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
            "$body_lit" \
            "$asset_lit" \
            none none none \
            none none none none none \
            "$enable_spt_arg" none 1 none none none \
            "$ref_mr" "$ref_mem" "$ref_clk")" || return 1
    fi
    printf '%s' "$out"
}

submit_analyze_post_composition_direct() {
    local post_id="$1" asset_id="$2" opts_json="${3:-{}}"
    local out digest
    post_id="$(normalize_hex_id "$post_id")" || return 1
    asset_id="$(normalize_hex_id "$asset_id")" || return 1
    require_session_fields POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID || return 1

    log_step "analyze_post_composition post=$post_id asset=$asset_id"
    out="$(poc_python - "$post_id" "$asset_id" "$opts_json" <<'PY'
import json, sys
from app.services.composition_submission import (
    AssetVersionInput,
    PAYOUT_ESCROW,
    PAYOUT_WALLET,
    USAGE_SOCIAL_POST,
    build_analyze_post_composition_move_call,
    build_composition_submission_from_assets,
)
from app.services.myso_client import init_myso_client

post_id, asset_id, opts_raw = sys.argv[1:4]
opts = json.loads(opts_raw or "{}")
share_bps = int(opts.get("share_bps") or 0)
beneficiary = opts.get("beneficiary") or None
payout_mode = int(opts.get("payout_mode") if opts.get("payout_mode") is not None else PAYOUT_WALLET)
if str(opts.get("payout_mode", "")).lower() == "escrow":
    payout_mode = PAYOUT_ESCROW
spt_pool_id = opts.get("spt_pool_id") or None
contains_derivatives = bool(opts.get("contains_derivatives") or False)
contains_unresolved = bool(opts.get("contains_unresolved_assets") or False)
reasoning = opts.get("reasoning") or "Direct-move composition (E2E)"
derivative_target = 1 if payout_mode == PAYOUT_ESCROW else 0

asset = AssetVersionInput(
    asset_id=asset_id,
    rights_version=int(opts.get("rights_version") or 1),
    economics_version=int(opts.get("economics_version") or 1),
    usage_class=int(opts.get("usage_class") or USAGE_SOCIAL_POST),
    share_bps=share_bps,
    beneficiary=beneficiary,
    payout_mode=payout_mode,
    source_asset_id=opts.get("source_asset_id") or asset_id,
)
manifest_entries = [asset] if share_bps > 0 and beneficiary else None
submission = build_composition_submission_from_assets(
    post_id=post_id,
    assets=[asset],
    manifest_entries=manifest_entries,
    derivative_redirection_target=derivative_target,
    contains_derivatives=contains_derivatives,
    contains_unresolved_assets=contains_unresolved,
    reasoning=reasoning,
    spt_pool_id=spt_pool_id,
)
move_call = build_analyze_post_composition_move_call(submission)
client = init_myso_client()
if client is None:
    raise SystemExit("MySocial client not configured in proof-of-creativity")
result = client.submit_analyze_post_composition(move_call)
print(json.dumps(result))
PY
)" || return 1

    digest="$(echo "$out" | jq -r '.tx_hash // .digest // empty')"
    [[ -n "$digest" ]] || digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    [[ -n "$digest" ]] || {
        echo "analyze_post_composition did not return a tx digest" >&2
        return 1
    }
    wait_for_tx_finalized "$digest" || return 1
    ANALYZE_POST_LAST_DIGEST="$digest"
    ANALYZE_TX_DIGEST="$digest"
    printf '%s' "$digest"
}
