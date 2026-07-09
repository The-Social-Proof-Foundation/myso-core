#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared HTTP helpers for proof-of-creativity oracle E2E scripts (myso-core).

: "${POC_ORACLE_URL:=http://127.0.0.1:8000}"
: "${POC_ORACLE_NETWORK:=localnet}"
: "${POC_USE_DIRECT_MOVE:=0}"
: "${POC_E2E_SUBMIT_OVERRIDE:=1}"
: "${POC_ORACLE_FIXTURE_URL:=https://pub-b2100065e2e5444db484a5fed5d87096.r2.dev/image/2026/05/23a44811-b991-418e-88ee-b99c3973fa91.png}"
: "${POC_ORACLE_ATTESTATION_TIMEOUT_SEC:=180}"

poc_oracle_base_url() {
    printf '%s' "${POC_ORACLE_URL%/}"
}

poc_oracle_health_ok() {
    local base health ready integration
    base="$(poc_oracle_base_url)"
    health="$(curl -sf "${base}/oracle/health?network=${POC_ORACLE_NETWORK}" 2>/dev/null)" || {
        echo "PoC oracle health check failed at ${base}/oracle/health?network=${POC_ORACLE_NETWORK}" >&2
        echo "Start PoC stack: myso start --with-poc (or docker compose --profile app in proof-of-creativity)" >&2
        return 1
    }
    ready="$(echo "$health" | jq -r '.mysocial.ready_for_submission // false')"
    integration="$(echo "$health" | jq -r '.mysocial.integration_requested // false')"
    if [[ "$ready" == "true" ]]; then
        return 0
    fi
    if [[ "$integration" != "true" ]]; then
        return 0
    fi
    echo "PoC oracle integration is enabled but not ready for submission." >&2
    echo "  health: $health" >&2
    echo "Fix: set MYSO_INTEGRATION_ENABLED=true and ORACLE_PRIVATE_KEY_LOCALNET in proof-of-creativity/.env" >&2
    echo "  (myso start --with-poc writes these; or run ensure_poc_oracle_key_in_env from poc-oracle-common.sh)" >&2
    echo "For E2E score/creator overrides also set POC_E2E_SUBMIT_OVERRIDE=1 in proof-of-creativity/.env" >&2
    return 1
}

poc_oracle_wait_for_attestation() {
    local post_id="$1"
    local timeout_sec="${2:-$POC_ORACLE_ATTESTATION_TIMEOUT_SEC}"
    local base deadline resp status digest
    base="$(poc_oracle_base_url)"
    deadline=$(( $(date +%s) + timeout_sec ))
    while [[ $(date +%s) -lt "$deadline" ]]; do
        resp="$(curl -sf "${base}/oracle/posts/${post_id}?network=${POC_ORACLE_NETWORK}" 2>/dev/null)" || resp=''
        status="$(echo "$resp" | jq -r '.analysis_status // empty')"
        digest="$(echo "$resp" | jq -r '.tx_digest // empty')"
        if [[ "$status" == "attested" && -n "$digest" ]]; then
            ANALYZE_POST_LAST_DIGEST="$digest"
            printf '%s' "$digest"
            return 0
        fi
        sleep 2
    done
    echo "Timed out waiting for oracle attestation on post $post_id" >&2
    return 1
}

_poc_oracle_fixture_path() {
    local tmp
    tmp="$(mktemp "${TMPDIR:-/tmp}/poc-oracle-fixture.XXXXXX.png")"
    if ! curl -sfL "$POC_ORACLE_FIXTURE_URL" -o "$tmp"; then
        rm -f "$tmp"
        echo "Failed to download PoC oracle fixture from $POC_ORACLE_FIXTURE_URL" >&2
        return 1
    fi
    printf '%s' "$tmp"
}

poc_oracle_upload_fixture() {
    local post_id="$1"
    local media_type="${2:-1}"
    local score="${3:-50}"
    local original_creator_arg="${4:-none}"
    local deriv_target="${5:-0}"
    local embed_audio="${6:-false}"
    local apply_explicit="${7:-false}"
    local explicit_outcome="${8:-0}"
    local royalty_free="${9:-false}"
    local force_reanalyze="${10:-false}"

    local base fixture creator_param creator_q tx_hash
    base="$(poc_oracle_base_url)"
    fixture="$(_poc_oracle_fixture_path)" || return 1

    creator_param=""
    if [[ "$original_creator_arg" != "none" && -n "$original_creator_arg" ]]; then
        creator_param="${original_creator_arg#some(}"
        creator_param="${creator_param%)}"
    fi

    local url="${base}/upload?post_id=${post_id}&force_reanalyze=${force_reanalyze}&royalty_free=${royalty_free}"
    if [[ "${POC_E2E_SUBMIT_OVERRIDE}" == "1" ]]; then
        url="${url}&e2e_score=${score}&e2e_derivative_target=${deriv_target}"
        if [[ -n "$creator_param" ]]; then
            url="${url}&e2e_original_creator=${creator_param}"
        fi
    fi

    local resp
    resp="$(curl -sf -F "file=@${fixture};type=image/png" "$url")" || {
        rm -f "$fixture"
        echo "PoC oracle upload failed for post $post_id" >&2
        return 1
    }
    rm -f "$fixture"
    tx_hash="$(echo "$resp" | jq -r '.tx_hash // empty')"
    if [[ -n "$tx_hash" && "$tx_hash" != "null" ]]; then
        ANALYZE_POST_LAST_DIGEST="$tx_hash"
        printf '%s' "$tx_hash"
        return 0
    fi
    poc_oracle_wait_for_attestation "$post_id"
}

poc_oracle_analyze_post() {
    local post_id="$1" media_type="$2" score="$3" original_creator_arg="$4" \
        deriv_target="$5" embed_audio="$6" apply_explicit="$7" explicit_outcome="$8"
    local sender="${9:-}"
    local force_reanalyze="${10:-false}"
    _="$sender embed_audio apply_explicit explicit_outcome media_type"
    log_step "oracle analyze post=$post_id score=$score creator=$original_creator_arg"
    poc_oracle_health_ok || return 1
    poc_oracle_upload_fixture "$post_id" "$media_type" "$score" "$original_creator_arg" \
        "$deriv_target" "$embed_audio" "$apply_explicit" "$explicit_outcome" "false" "$force_reanalyze"
}

poc_oracle_claim_beneficiary() {
    local identity_hash="$1"
    local wallet="$2"
    local beneficiary_id="${3:-}"
    local mock_handle="${4:-}"
    local display_name="${5:-Creator}"
    local bio="${6:-bio}"

    local base body resp tx
    base="$(poc_oracle_base_url)"
    body="$(jq -nc \
        --arg wallet "$wallet" \
        --arg beneficiary_id "$beneficiary_id" \
        --arg attested_x_handle "$mock_handle" \
        --arg display_name "$display_name" \
        --arg bio "$bio" \
        '{
            claimant_address: $wallet,
            beneficiary_id: (if $beneficiary_id == "" then null else $beneficiary_id end),
            attested_x_handle: (if $attested_x_handle == "" then null else $attested_x_handle end),
            display_name: $display_name,
            bio: $bio,
            profile_picture_url: "",
            cover_photo_url: ""
        }')"

    local -a hdr=(-H "Content-Type: application/json")
    if [[ -n "$mock_handle" ]]; then
        hdr+=(-H "X-PoC-Mock-Handle: ${mock_handle}")
    fi
    hdr+=(-H "X-PoC-Mock-Identity-Hash: ${identity_hash}")

    resp="$(curl -sf "${hdr[@]}" \
        -X POST \
        -d "$body" \
        "${base}/oracle/beneficiaries/${identity_hash}/claim?sync=true&network=${POC_ORACLE_NETWORK}")" || {
        echo "PoC oracle beneficiary claim failed" >&2
        return 1
    }
    tx="$(echo "$resp" | jq -r '.claim_tx_digest // .beneficiary.claim_tx_digest // empty')"
    POC_ORACLE_LAST_CLAIM_TX="$tx"
    [[ -n "$tx" ]] || tx="$(echo "$resp" | jq -r '.job_id // empty')"
    [[ -n "$tx" ]]
    printf '%s' "$tx"
}
