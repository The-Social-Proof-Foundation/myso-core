#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared HTTP helpers for proof-of-creativity oracle E2E scripts (myso-core).

: "${POC_ORACLE_URL:=http://127.0.0.1:8000}"
: "${POC_ORACLE_NETWORK:=localnet}"
: "${POC_USE_DIRECT_MOVE:=0}"
: "${POC_ORACLE_ATTESTATION_TIMEOUT_SEC:=180}"

poc_oracle_base_url() {
    printf '%s' "${POC_ORACLE_URL%/}"
}

poc_oracle_health_ok() {
    local base health mysocial ready integration
    base="$(poc_oracle_base_url)"
    health="$(curl -sf "${base}/oracle/health?network=${POC_ORACLE_NETWORK}" 2>/dev/null)" || health=''
    if [[ -n "$health" ]]; then
        mysocial="$(echo "$health" | jq -c '.mysocial // empty' 2>/dev/null)"
    fi
    if [[ -z "${mysocial:-}" || "$mysocial" == "null" ]]; then
        health="$(curl -sf "${base}/health" 2>/dev/null)" || {
            echo "PoC API health check failed at ${base}/health" >&2
            echo "Start PoC stack: myso start --with-poc (or docker compose --profile app in proof-of-creativity)" >&2
            return 1
        }
        mysocial="$(echo "$health" | jq -c '.components.mysocial // .mysocial // empty' 2>/dev/null)"
    fi
    ready="$(echo "$mysocial" | jq -r '.ready_for_submission // false')"
    integration="$(echo "$mysocial" | jq -r '.integration_requested // false')"
    if [[ "$ready" == "true" ]]; then
        return 0
    fi
    if [[ "$integration" != "true" ]]; then
        return 0
    fi
    echo "PoC oracle integration is enabled but not ready for submission." >&2
    echo "  mysocial: $mysocial" >&2
    echo "Fix: set MYSO_INTEGRATION_ENABLED=true and ORACLE_PRIVATE_KEY_LOCALNET in proof-of-creativity/.env" >&2
    echo "  (myso start --with-poc writes these; or run ensure_poc_oracle_key_in_env from poc-oracle-common.sh)" >&2
    return 1
}

poc_oracle_wait_for_attestation() {
    local post_id="$1"
    local timeout_sec="${2:-$POC_ORACLE_ATTESTATION_TIMEOUT_SEC}"
    wait_for_poc_post_attested "$post_id" "$timeout_sec"
}

poc_oracle_wait_for_composition() {
    local post_id="$1"
    local expected="${2:-VERIFIED}"
    local timeout_sec="${3:-$POC_ORACLE_ATTESTATION_TIMEOUT_SEC}"
    wait_for_poc_post_composed "$post_id" "$expected" "$timeout_sec"
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
