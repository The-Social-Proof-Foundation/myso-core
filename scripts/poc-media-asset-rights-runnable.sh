#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper: resolve a MediaAsset, submit a PoC governance rights dispute,
# fast-forward governance, and verify oracle implement + indexer GraphQL state.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql
#   - proof-of-creativity stack (api + oracle-worker + postgres) OR POC_USE_DIRECT_MOVE=1
#   - `myso`, `curl`, `jq`, `python3` on PATH
#
# Session: network.config/poc/poc-media-asset-rights-session.env
#
# Usage:
#   ./scripts/poc-media-asset-rights-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/poc-media-asset-rights-runnable.sh --run-all
#   ASSUME_YES=1 ./scripts/poc-media-asset-rights-runnable.sh --resolve-asset
#   ASSUME_YES=1 ./scripts/poc-media-asset-rights-runnable.sh --submit-dispute
#   ASSUME_YES=1 ./scripts/poc-media-asset-rights-runnable.sh --fast-forward-governance
#   ASSUME_YES=1 ./scripts/poc-media-asset-rights-runnable.sh --wait-oracle-implement
#   ASSUME_YES=1 ./scripts/poc-media-asset-rights-runnable.sh --verify-gql
#   ./scripts/poc-media-asset-rights-runnable.sh   # interactive menu
#
# Environment:
#   POC_ORACLE_URL          PoC API base (default http://127.0.0.1:8000)
#   POC_ORACLE_NETWORK      network profile (default localnet)
#   MYSO_POC_REPO           path to proof-of-creativity (default ../proof-of-creativity)
#   POC_USE_DIRECT_MOVE=1   skip PoC API; oracle steps use on-chain PTBs only
#   POC_WAIT_ORACLE=1       in --run-all, poll worker instead of direct implement (default 1)
#   GOVERNANCE_VOTE_WALLET  wallet for delegate vote (default active myso address)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
: "${SOCIAL_SESSION_SAVE_PATH:=$REPO_ROOT/network.config/poc/poc-media-asset-rights-session.env}"
: "${MYSO_POC_REPO:=$(cd "${REPO_ROOT}/../proof-of-creativity" 2>/dev/null && pwd || echo "${REPO_ROOT}/../proof-of-creativity")}"
: "${POC_WAIT_ORACLE:=1}"
: "${POC_USE_DIRECT_MOVE:=0}"

# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"
# shellcheck source=lib/poc-oracle-common.sh
source "${SCRIPT_DIR}/lib/poc-oracle-common.sh"
# shellcheck source=lib/poc-oracle-http.sh
source "${SCRIPT_DIR}/lib/poc-oracle-http.sh"

readonly DEFAULT_DISPUTE_PAYMENT_MIST='110000000000'
readonly DEFAULT_COMMUNITY_VOTE_PAYMENT_MIST='10000000000'
readonly MEDIA_TYPE_IMAGE='1'
readonly ORIGINALITY_ORIGINAL='1'
readonly ASSET_KIND_UNSPECIFIED='0'

RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"
SOCIAL_RUN_ID="$(date +%s)"

POC_GOVERNANCE_REGISTRY_ID=''
GOVERNANCE_ADMIN_CAP_ID=''
CREATOR_ADDRESS=''
CHALLENGER_ADDRESS=''
NEW_RIGHTS_HOLDER_ADDRESS=''
MEDIA_ASSET_ID=''
RESOLUTION_REQUEST_ID=''
PROPOSAL_ID=''
CLAIMS_COMMITMENT_HEX=''
LAST_TX_DIGEST=''
SUBMIT_DISPUTE_TX_DIGEST=''
FINALIZE_GOV_TX_DIGEST=''
IMPLEMENT_TX_DIGEST=''
RIGHTS_VERSION_BEFORE=''
RIGHTS_VERSION_AFTER=''

POC_RIGHTS_SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET GRAPHQL_URL
    USERNAME_REGISTRY_ID PROFILE_CONFIG_ID ECOSYSTEM_TREASURY_ID
    POC_CONFIG_ID POC_REGISTRY_ID POC_ADMIN_CAP_ID POC_VAULT_DIRECTORY_ID
    POC_GOVERNANCE_REGISTRY_ID GOVERNANCE_ADMIN_CAP_ID
    CREATOR_ADDRESS CHALLENGER_ADDRESS NEW_RIGHTS_HOLDER_ADDRESS
    MEDIA_ASSET_ID RESOLUTION_REQUEST_ID PROPOSAL_ID CLAIMS_COMMITMENT_HEX
    LAST_TX_DIGEST SUBMIT_DISPUTE_TX_DIGEST FINALIZE_GOV_TX_DIGEST IMPLEMENT_TX_DIGEST
    RIGHTS_VERSION_BEFORE RIGHTS_VERSION_AFTER
    POC_ORACLE_URL POC_ORACLE_NETWORK MYSO_POC_REPO
)

usage() {
    sed -n '2,32p' "$0" | sed 's/^# \?//'
}

save_poc_rights_session() {
    social_save_session "${POC_RIGHTS_SESSION_KEYS[@]}"
}

load_poc_rights_session() {
    social_load_session
}

poc_rights_repo() {
    poc_oracle_resolve_repo
}

literal_move_hex_bytes() {
    local hex="$1"
    hex="${hex#0x}"
    printf 'x"%s"' "$hex"
}

run_myso_call_as_capture() {
    local sender="$1" module="$2" func="$3"
    shift 3
    local -a cmd call_args=() out rc=0
    local arg g
    sender="$(normalize_hex_id "$sender")" || return 1
    while IFS= read -r -d '' arg; do call_args+=("$arg"); done < <(normalize_client_call_args "$@")
    cmd=(myso client call --package "$PKG_SOCIAL" --sender "$sender" \
        --module "$module" --function "$func")
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
    out="$("${cmd[@]}" 2>&1)" || rc=$?
    echo "$out" >&2
    printf '%s' "$out"
    return "$rc"
}

assert_rights_tx_events() {
    local scenario="$1" digest="$2"
    [[ -n "$digest" ]] || return 1
    case "$scenario" in
        dispute_submit)
            assert_tx_events "$digest" \
                MediaAssetRightsDisputeProposedEvent \
                MediaAssetGovernanceProposalLinkedEvent
            ;;
        rights_finalize_gov)
            if tx_has_event_named "$digest" ProposalApprovedEvent \
                || tx_has_event_named "$digest" ProposalRejectedEvent \
                || tx_has_event_named "$digest" MediaAssetGovernanceProposalClearedEvent; then
                return 0
            fi
            echo "WARN: finalize tx $digest had no expected governance outcome event (continuing)" >&2
            return 0
            ;;
        rights_implement)
            assert_tx_events "$digest" \
                MediaAssetRightsUpdatedEvent \
                MediaAssetGovernanceProposalClearedEvent
            ;;
        asset_resolved)
            assert_tx_events "$digest" MediaAssetResolvedEvent
            ;;
        resolution_requested)
            assert_tx_events "$digest" MediaResolutionRequestedEvent
            ;;
        *)
            echo "Unknown rights scenario: $scenario" >&2
            return 1
            ;;
    esac
}

read_poc_governance_registry_from_gql() {
    local resp reg
    resp="$(graphql_post \
        '{ pocConfiguration { disputeGovernanceRegistryId } governanceRegistries(registryType: 1) { registryId } }' \
        '{}')" || return 1
    reg="$(echo "$resp" | jq -r '.data.pocConfiguration.disputeGovernanceRegistryId // empty')"
    if [[ -z "$reg" ]]; then
        reg="$(echo "$resp" | jq -r '.data.governanceRegistries[0].registryId // empty')"
    fi
    [[ -n "$reg" ]] || return 1
    normalize_hex_id "$reg"
}

refresh_poc_rights_session_from_graphql() {
    social_refresh_session_from_graphql || return 1
    load_poc_rights_session

    local json reg
    log_step "Refreshing PoC media-asset rights session from GraphQL ($GRAPHQL_URL)"

    json="$(graphql_post \
        'query PocRightsSessionExtras {
          pocConfig: objects(filter: { type: "0x50c1::proof_of_creativity::PoCConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
          pocRegistry: objects(filter: { type: "0x50c1::proof_of_creativity::PoCRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
          pocVaultDirectory: objects(filter: { type: "0x50c1::poc_vault::PoCVaultDirectory", ownerKind: SHARED }, first: 1) { nodes { address } }
          proofOfCreativityAdminCap: objects(filter: { type: "0x50c1::proof_of_creativity::PoCAdminCap" }, last: 1) { nodes { address } }
          governanceAdminCap: objects(filter: { type: "0x50c1::governance::GovernanceAdminCap" }, last: 1) { nodes { address } }
          ecosystemTreasury: objects(filter: { type: "0x50c1::profile::EcosystemTreasury", ownerKind: SHARED }, first: 1) { nodes { address } }
        }')" || return 1

    POC_CONFIG_ID="$(gql_object_address "$json" pocConfig)"
    POC_REGISTRY_ID="$(gql_object_address "$json" pocRegistry)"
    POC_ADMIN_CAP_ID="$(gql_object_address "$json" proofOfCreativityAdminCap)"
    GOVERNANCE_ADMIN_CAP_ID="$(gql_object_address "$json" governanceAdminCap)"
    POC_VAULT_DIRECTORY_ID="$(gql_object_address "$json" pocVaultDirectory)"
    ECOSYSTEM_TREASURY_ID="$(gql_object_address "$json" ecosystemTreasury)"
    POC_GOVERNANCE_REGISTRY_ID="$(read_poc_governance_registry_from_gql)" || true

    log_session_use "POC_CONFIG_ID" "$POC_CONFIG_ID"
    log_session_use "POC_REGISTRY_ID" "$POC_REGISTRY_ID"
    log_session_use "POC_ADMIN_CAP_ID" "$POC_ADMIN_CAP_ID"
    log_session_use "GOVERNANCE_ADMIN_CAP_ID" "$GOVERNANCE_ADMIN_CAP_ID"
    log_session_use "POC_VAULT_DIRECTORY_ID" "$POC_VAULT_DIRECTORY_ID"
    log_session_use "ECOSYSTEM_TREASURY_ID" "$ECOSYSTEM_TREASURY_ID"
    log_session_use "POC_GOVERNANCE_REGISTRY_ID" "$POC_GOVERNANCE_REGISTRY_ID"
    save_poc_rights_session
}

ensure_creator_wallet() {
    CREATOR_ADDRESS="$(resolve_myso_active_address)" || {
        echo "Could not read myso client active-address" >&2
        return 1
    }
    CREATOR_ADDRESS="$(normalize_hex_id "$CREATOR_ADDRESS")"
    ensure_wallet_funded "$CREATOR_ADDRESS" "$((SOCIAL_DEFAULT_GAS_BUDGET * 4))" || return 1
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
}

ensure_challenger_wallet() {
    if [[ -n "${CHALLENGER_ADDRESS:-}" ]] && object_exists_on_fullnode "$CHALLENGER_ADDRESS"; then
        CHALLENGER_ADDRESS="$(normalize_hex_id "$CHALLENGER_ADDRESS")"
        ensure_wallet_funded "$CHALLENGER_ADDRESS" "$((DEFAULT_DISPUTE_PAYMENT_MIST + SOCIAL_DEFAULT_GAS_BUDGET * 2))" || return 1
        log_session_use "CHALLENGER_ADDRESS" "$CHALLENGER_ADDRESS"
        return 0
    fi
    CHALLENGER_ADDRESS="$(create_ephemeral_wallet "poc_rights_challenger_${SOCIAL_RUN_ID}")" || return 1
    ensure_wallet_funded "$CHALLENGER_ADDRESS" "$((DEFAULT_DISPUTE_PAYMENT_MIST + SOCIAL_DEFAULT_GAS_BUDGET * 2))" || return 1
    log_session_use "CHALLENGER_ADDRESS" "$CHALLENGER_ADDRESS"
    save_poc_rights_session
}

ensure_new_rights_holder() {
    if [[ -n "${NEW_RIGHTS_HOLDER_ADDRESS:-}" ]]; then
        NEW_RIGHTS_HOLDER_ADDRESS="$(normalize_hex_id "$NEW_RIGHTS_HOLDER_ADDRESS")"
        log_session_use "NEW_RIGHTS_HOLDER_ADDRESS" "$NEW_RIGHTS_HOLDER_ADDRESS"
        return 0
    fi
    NEW_RIGHTS_HOLDER_ADDRESS="$(create_ephemeral_wallet "poc_rights_holder_${SOCIAL_RUN_ID}")" || return 1
    ensure_wallet_funded "$NEW_RIGHTS_HOLDER_ADDRESS" "$SOCIAL_DEFAULT_GAS_BUDGET" || true
    log_session_use "NEW_RIGHTS_HOLDER_ADDRESS" "$NEW_RIGHTS_HOLDER_ADDRESS"
    save_poc_rights_session
}

deterministic_commitments_for_run() {
    python3 - "$SOCIAL_RUN_ID" <<'PY'
import hashlib, sys
run_id = sys.argv[1].encode()
content = hashlib.sha256(b"content:" + run_id).digest()
fingerprint = hashlib.sha256(b"fingerprint:" + run_id).digest()
print(content.hex())
print(fingerprint.hex())
PY
}

extract_request_id_from_digest() {
    local digest="$1" req_id
    req_id="$(extract_created_object_by_type "$digest" "MediaResolutionRequest")"
    [[ -n "$req_id" ]] || req_id="$(extract_event_field "$digest" MediaResolutionRequestedEvent request_id 2>/dev/null || true)"
    [[ -n "$req_id" ]] || return 1
    normalize_hex_id "$req_id"
}

extract_proposal_id_from_digest() {
    local digest="$1" pid
    pid="$(extract_created_object_by_type "$digest" "governance::Proposal")"
    [[ -n "$pid" ]] || pid="$(extract_created_object_by_type "$digest" "Proposal")"
    [[ -n "$pid" ]] || pid="$(extract_event_field "$digest" MediaAssetRightsDisputeProposedEvent proposal_id 2>/dev/null || true)"
    [[ -n "$pid" ]] || return 1
    normalize_hex_id "$pid"
}

extract_media_asset_from_digest() {
    local digest="$1" asset_id
    asset_id="$(extract_created_object_by_type "$digest" "media_asset::MediaAsset")"
    [[ -n "$asset_id" ]] || asset_id="$(extract_created_object_by_type "$digest" "MediaAsset")"
    [[ -n "$asset_id" ]] || asset_id="$(extract_event_field "$digest" MediaAssetResolvedEvent media_asset_id 2>/dev/null || true)"
    [[ -n "$asset_id" ]] || return 1
    normalize_hex_id "$asset_id"
}

poc_rights_python() {
    local poc_repo
    poc_repo="$(poc_rights_repo)"
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
        export POC_GOVERNANCE_REGISTRY_ID="${POC_GOVERNANCE_REGISTRY_ID:-}"
        export MYSO_ECOSYSTEM_TREASURY_ID="${ECOSYSTEM_TREASURY_ID:-${MYSO_ECOSYSTEM_TREASURY_ID:-}}"
        export MYSO_CLOCK_OBJECT_ID="${CLOCK_ID:-${MYSO_CLOCK_OBJECT_ID:-0x6}}"
        export GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
        export POC_ORACLE_NETWORK="${POC_ORACLE_NETWORK:-localnet}"
        PYTHONPATH="${poc_repo}${PYTHONPATH:+:$PYTHONPATH}" python3 "$@"
    )
}

build_default_claims_json() {
    local holder="$1"
    poc_rights_python - "$holder" <<'PY'
import json, sys
holder = sys.argv[1]
claims = [
    {"claim_type": 1, "claimant": holder, "asset_id": None, "rights_mask": 0, "scope": 0, "verification_status": 2, "evidence_commitment": None},
    {"claim_type": 3, "claimant": holder, "asset_id": None, "rights_mask": 63, "scope": 0, "verification_status": 2, "evidence_commitment": None},
    {"claim_type": 4, "claimant": holder, "asset_id": None, "rights_mask": 0, "scope": 0, "verification_status": 2, "evidence_commitment": None},
]
grants = [{
    "usage_class": 1,
    "granted_rights": 17,
    "license_type": 1,
    "compensation_type": 2,
    "compensation_bps": 10000,
    "attribution_required": False,
    "derivatives_permitted": True,
    "commercial_use_permitted": False,
    "effective_from": 0,
    "expires_at": None,
    "revocable": True,
}]
print(json.dumps({"claims": claims, "usage_grants": grants}))
PY
}

prepare_claims_bundle_via_api() {
    local asset_id="$1" holder="$2" body resp commitment
    asset_id="$(normalize_hex_id "$asset_id")" || return 1
    holder="$(normalize_hex_id "$holder")" || return 1
    body="$(build_default_claims_json "$holder")" || return 1
    resp="$(curl -sf -X POST "$(poc_oracle_base_url)/oracle/disputes/media-asset-rights/prepare" \
        -H 'Content-Type: application/json' \
        -d "$(jq -nc --argjson bundle "$body" --arg asset "$asset_id" \
            '{media_asset_id: $asset, claims: $bundle.claims, usage_grants: $bundle.usage_grants}')")" || {
        echo "PoC API prepare failed (is proof-of-creativity api running?)" >&2
        return 1
    }
    commitment="$(echo "$resp" | jq -r '.claims_commitment // empty')"
    [[ -n "$commitment" ]] || return 1
    CLAIMS_COMMITMENT_HEX="${commitment#0x}"
    log_session_use "CLAIMS_COMMITMENT_HEX" "$CLAIMS_COMMITMENT_HEX"
    printf '%s' "$body"
}

prepare_claims_bundle_local() {
    local holder="$1" body commitment
    holder="$(normalize_hex_id "$holder")" || return 1
    body="$(build_default_claims_json "$holder")" || return 1
    commitment="$(poc_rights_python - "$body" <<'PY'
import json, sys
from app.chain.bcs_media_asset_claims import compute_claims_bundle_commitment
bundle = json.loads(sys.argv[1])
c = compute_claims_bundle_commitment(bundle["claims"], bundle["usage_grants"])
print(c.hex())
PY
)" || return 1
    CLAIMS_COMMITMENT_HEX="$commitment"
    log_session_use "CLAIMS_COMMITMENT_HEX" "$CLAIMS_COMMITMENT_HEX"
    printf '%s' "$body"
}

store_claims_bundle_via_api() {
    local proposal_id="$1" asset_id="$2" submitter="$3" bundle_json="$4"
    curl -sf -X POST "$(poc_oracle_base_url)/oracle/disputes/media-asset-rights/submit" \
        -H 'Content-Type: application/json' \
        -d "$(jq -nc \
            --arg proposal_id "$(normalize_hex_id "$proposal_id")" \
            --arg media_asset_id "$(normalize_hex_id "$asset_id")" \
            --arg submitter "$(normalize_hex_id "$submitter")" \
            --arg network "${POC_ORACLE_NETWORK}" \
            --argjson bundle "$bundle_json" \
            '{
                proposal_id: $proposal_id,
                media_asset_id: $media_asset_id,
                submitter: $submitter,
                network: $network,
                claims: $bundle.claims,
                usage_grants: $bundle.usage_grants
            }')" >/dev/null
}

step_tune_poc_governance_for_e2e() {
    local out digest voter
    require_session_fields POC_GOVERNANCE_REGISTRY_ID GOVERNANCE_ADMIN_CAP_ID CLOCK_ID || return 0
    require_hex_ids POC_GOVERNANCE_REGISTRY_ID GOVERNANCE_ADMIN_CAP_ID CLOCK_ID || return 0
    voter="$(resolve_myso_active_address)" || return 0
    log_step "Tuning PoC governance registry for fast E2E (delegate_count=2, short voting)"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$voter" governance \
        update_governance_parameters \
        "@${POC_GOVERNANCE_REGISTRY_ID}" \
        "@${GOVERNANCE_ADMIN_CAP_ID}" \
        2 90 1000 5 0 1 1 \
        "@${CLOCK_ID}")" || {
        echo "WARN: update_governance_parameters failed; voting may be slow" >&2
        return 0
    }
    digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    [[ -n "$digest" ]] && wait_for_tx_finalized "$digest" || true
}

step_submit_media_resolution() {
    local content_hex fingerprint_hex out digest
    require_hex_ids CLOCK_ID || return 1
    mapfile -t _commits < <(deterministic_commitments_for_run)
    content_hex="${_commits[0]}"
    fingerprint_hex="${_commits[1]}"
    export POC_CONTENT_HEX="$content_hex"
    export POC_FINGERPRINT_HEX="$fingerprint_hex"

    switch_wallet "$CREATOR_ADDRESS" || return 1
    log_step "submit_media_resolution content=${content_hex:0:16}… fingerprint=${fingerprint_hex:0:16}…"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$CREATOR_ADDRESS" media_asset submit_media_resolution \
        "$(literal_move_hex_bytes "$content_hex")" \
        "$(literal_move_hex_bytes "$fingerprint_hex")" \
        "$MEDIA_TYPE_IMAGE" \
        "@${CLOCK_ID}")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    assert_rights_tx_events resolution_requested "$digest" || return 1
    RESOLUTION_REQUEST_ID="$(extract_request_id_from_digest "$digest")" || {
        echo "Could not extract MediaResolutionRequest id from tx $digest" >&2
        return 1
    }
    LAST_TX_DIGEST="$digest"
    log_session_use "RESOLUTION_REQUEST_ID" "$RESOLUTION_REQUEST_ID"
    log_session_use "LAST_TX_DIGEST" "$LAST_TX_DIGEST"
    save_poc_rights_session
}

step_oracle_finalize_media_asset_direct() {
    local oracle out digest asset_id holder
    require_session_fields POC_CONFIG_ID RESOLUTION_REQUEST_ID CREATOR_ADDRESS CLOCK_ID || return 1
    require_hex_ids POC_CONFIG_ID RESOLUTION_REQUEST_ID CREATOR_ADDRESS CLOCK_ID || return 1
    if [[ -z "${POC_CONTENT_HEX:-}" || -z "${POC_FINGERPRINT_HEX:-}" ]]; then
        mapfile -t _commits < <(deterministic_commitments_for_run)
        export POC_CONTENT_HEX="${_commits[0]}"
        export POC_FINGERPRINT_HEX="${_commits[1]}"
    fi
    sync_poc_config_oracle_on_chain "$POC_DEFAULT_ORACLE_ADDRESS" || return 1
    ensure_poc_oracle_key_in_env || return 1
    holder="$(normalize_hex_id "${NEW_RIGHTS_HOLDER_ADDRESS:-$CREATOR_ADDRESS}")"

    oracle="$(read_poc_config_oracle_address)" || oracle="$POC_DEFAULT_ORACLE_ADDRESS"
    oracle="$(normalize_hex_id "$oracle")"

    log_step "oracle finalize_media_asset request=$RESOLUTION_REQUEST_ID holder=$holder"
    out="$(poc_rights_python - \
        "$POC_CONFIG_ID" "$RESOLUTION_REQUEST_ID" "$CREATOR_ADDRESS" "$holder" <<'PY'
import json, os, sys
from app.services.media_asset_submission import MediaResolutionResult, build_finalize_media_asset_move_call
from app.services.myso_client import init_myso_client

config_id, request_id, submitter, holder = sys.argv[1:5]
resolution = MediaResolutionResult(
    request_id=request_id,
    content_commitment=bytes.fromhex(os.environ["POC_CONTENT_HEX"]),
    observed_fingerprint_commitment=bytes.fromhex(os.environ["POC_FINGERPRINT_HEX"]),
    media_type=1,
    submitter=submitter,
)
mc = build_finalize_media_asset_move_call(resolution)
client = init_myso_client()
if client is None:
    raise SystemExit("MySocial client not configured in proof-of-creativity")
result = client.submit_move_call(mc)
print(json.dumps(result))
PY
)" || return 1

    digest="$(echo "$out" | jq -r '.tx_hash // .digest // empty')"
    [[ -n "$digest" ]] || digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    [[ -n "$digest" ]] || {
        echo "finalize_media_asset did not return tx digest" >&2
        return 1
    }
    wait_for_tx_finalized "$digest" || return 1
    assert_rights_tx_events asset_resolved "$digest" || return 1
    asset_id="$(extract_media_asset_from_digest "$digest")" || true
    if [[ -z "$asset_id" ]]; then
        asset_id="$(wait_for_gql_media_asset_by_submitter "$CREATOR_ADDRESS")" || return 1
    fi
    MEDIA_ASSET_ID="$(normalize_hex_id "$asset_id")"
    LAST_TX_DIGEST="$digest"
    log_session_use "MEDIA_ASSET_ID" "$MEDIA_ASSET_ID"
    log_session_use "LAST_TX_DIGEST" "$LAST_TX_DIGEST"
    save_poc_rights_session
}

wait_for_oracle_resolve_media_asset() {
    local attempt asset_id resp vars
    [[ -n "${RESOLUTION_REQUEST_ID:-}" ]] || {
        echo "RESOLUTION_REQUEST_ID missing; run --resolve-asset first" >&2
        return 1
    }
    poc_oracle_stack_ready || return 1
    log_step "Waiting for oracle-worker resolve_media_asset (request=$RESOLUTION_REQUEST_ID)"
    for attempt in $(seq 1 90); do
        if [[ -n "${MEDIA_ASSET_ID:-}" ]] && object_exists_on_fullnode "$MEDIA_ASSET_ID"; then
            return 0
        fi
        asset_id="$(wait_for_gql_media_asset_by_submitter "$CREATOR_ADDRESS" 2>/dev/null)" || asset_id=''
        if [[ -n "$asset_id" ]]; then
            MEDIA_ASSET_ID="$(normalize_hex_id "$asset_id")"
            log_session_use "MEDIA_ASSET_ID" "$MEDIA_ASSET_ID"
            save_poc_rights_session
            return 0
        fi
        if [[ $((attempt % 10)) -eq 0 ]]; then
            echo "  still waiting for MediaAsset resolution (attempt ${attempt}/90)…" >&2
        fi
        sleep 2
    done
    echo "Timed out waiting for oracle to finalize MediaAsset" >&2
    return 1
}

wait_for_gql_media_asset_by_submitter() {
    local owner="$1" attempt resp asset_id
    owner="$(normalize_hex_id "$owner")" || return 1
    for attempt in $(seq 1 45); do
        resp="$(graphql_post \
            'query MediaAssets($owner: String!) {
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

gql_media_asset_snapshot() {
    local asset_id="$1" vars
    asset_id="$(normalize_hex_id "$asset_id")" || return 1
    vars="$(jq -nc --arg id "$asset_id" '{id: $id}')"
    graphql_post \
        'query MediaAssetRights($id: ID!) {
            mediaAsset(id: $id) {
                mediaAssetId
                rightsVersion
                rightsDisputesSubmitted
                activeRightsProposal { proposalId status }
            }
            pocConfiguration { mediaAssetDisputeCost maxDisputesPerMediaAsset }
        }' \
        "$vars"
}

step_submit_rights_dispute() {
    local bundle_json metadata_lit payment_coin gas_coin out digest proposal_id
    require_session_fields \
        POC_CONFIG_ID POC_GOVERNANCE_REGISTRY_ID ECOSYSTEM_TREASURY_ID \
        MEDIA_ASSET_ID CLOCK_ID || return 1
    require_hex_ids \
        POC_CONFIG_ID POC_GOVERNANCE_REGISTRY_ID ECOSYSTEM_TREASURY_ID \
        MEDIA_ASSET_ID CLOCK_ID || return 1

    ensure_challenger_wallet || return 1
    ensure_new_rights_holder || return 1

    if [[ "${POC_USE_DIRECT_MOVE}" == "1" ]]; then
        bundle_json="$(prepare_claims_bundle_local "$NEW_RIGHTS_HOLDER_ADDRESS")" || return 1
    else
        poc_oracle_health_ok || return 1
        bundle_json="$(prepare_claims_bundle_via_api "$MEDIA_ASSET_ID" "$NEW_RIGHTS_HOLDER_ADDRESS")" || return 1
    fi
    [[ -n "${CLAIMS_COMMITMENT_HEX:-}" ]] || {
        echo "claims commitment missing after prepare" >&2
        return 1
    }

    metadata_lit="$(literal_move_string "{\"poc_proposal_kind\":\"media_asset_rights\",\"claims_commitment\":\"0x${CLAIMS_COMMITMENT_HEX}\"}")"

    switch_wallet "$CHALLENGER_ADDRESS" || return 1
    read -r payment_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$CHALLENGER_ADDRESS" "$DEFAULT_DISPUTE_PAYMENT_MIST")" || {
        restore_wallet
        return 1
    }

    log_step "submit_media_asset_rights_dispute_proposal asset=$MEDIA_ASSET_ID challenger=$CHALLENGER_ADDRESS"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$CHALLENGER_ADDRESS" proof_of_creativity \
        submit_media_asset_rights_dispute_proposal \
        "@${POC_CONFIG_ID}" \
        "@${POC_GOVERNANCE_REGISTRY_ID}" \
        "@${ECOSYSTEM_TREASURY_ID}" \
        "@${MEDIA_ASSET_ID}" \
        "$(literal_move_string "Media asset rights dispute ${SOCIAL_RUN_ID}")" \
        "$(literal_move_string "Challenge rights holder via PoC governance")" \
        "$(literal_move_hex_bytes "$CLAIMS_COMMITMENT_HEX")" \
        none none \
        "some(${metadata_lit})" \
        "@${payment_coin}" \
        "@${CLOCK_ID}")" || {
        restore_wallet
        return 1
    }
    restore_wallet

    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    assert_rights_tx_events dispute_submit "$digest" || return 1
    proposal_id="$(extract_proposal_id_from_digest "$digest")" || {
        echo "Could not extract Proposal id from dispute submit tx" >&2
        return 1
    }
    PROPOSAL_ID="$(normalize_hex_id "$proposal_id")"
    SUBMIT_DISPUTE_TX_DIGEST="$digest"

    if [[ "${POC_USE_DIRECT_MOVE}" != "1" ]]; then
        log_step "Persisting claims bundle to PoC API for proposal $PROPOSAL_ID"
        store_claims_bundle_via_api "$PROPOSAL_ID" "$MEDIA_ASSET_ID" "$CHALLENGER_ADDRESS" "$bundle_json" || {
            echo "WARN: bundle store via API failed; oracle implement will fail without bundle" >&2
        }
    fi

    log_session_use "PROPOSAL_ID" "$PROPOSAL_ID"
    log_session_use "SUBMIT_DISPUTE_TX_DIGEST" "$SUBMIT_DISPUTE_TX_DIGEST"
    save_poc_rights_session
}

governance_vote_wallet() {
    local w="${GOVERNANCE_VOTE_WALLET:-}"
    if [[ -n "$w" ]]; then
        normalize_hex_id "$w"
        return 0
    fi
    resolve_myso_active_address
}

step_delegate_approve_proposal() {
    local voter out digest reason_lit
    require_session_fields POC_GOVERNANCE_REGISTRY_ID ECOSYSTEM_TREASURY_ID PROPOSAL_ID CLOCK_ID || return 1
    voter="$(governance_vote_wallet)" || return 1
    voter="$(normalize_hex_id "$voter")"
    reason_lit="$(literal_move_string "Delegate approve rights dispute ${SOCIAL_RUN_ID}")"

    switch_wallet "$voter" || return 1
    log_step "delegate_vote_on_proposal approve=true proposal=$PROPOSAL_ID voter=$voter"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$voter" governance delegate_vote_on_proposal \
        "@${POC_GOVERNANCE_REGISTRY_ID}" \
        "@${PROPOSAL_ID}" \
        "@${ECOSYSTEM_TREASURY_ID}" \
        true \
        "some(${reason_lit})" \
        "@${CLOCK_ID}")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    LAST_TX_DIGEST="$digest"
    log_session_use "LAST_TX_DIGEST" "$LAST_TX_DIGEST"
    save_poc_rights_session
}

step_community_approve_proposal() {
    local out digest payment_coin gas_coin saved_gas
    require_session_fields POC_GOVERNANCE_REGISTRY_ID PROPOSAL_ID CLOCK_ID CHALLENGER_ADDRESS || return 1

    switch_wallet "$CHALLENGER_ADDRESS" || return 1
    read -r payment_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$CHALLENGER_ADDRESS" "$DEFAULT_COMMUNITY_VOTE_PAYMENT_MIST")" || {
        restore_wallet
        return 1
    }
    saved_gas="${PTB_GAS_COIN_ID:-}"

    log_step "community_vote_on_proposal approve=true proposal=$PROPOSAL_ID voter=$CHALLENGER_ADDRESS"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$CHALLENGER_ADDRESS" governance community_vote_on_proposal \
        "@${POC_GOVERNANCE_REGISTRY_ID}" \
        "@${PROPOSAL_ID}" \
        1 true \
        "@${payment_coin}" \
        "@${CLOCK_ID}")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    LAST_TX_DIGEST="$digest"
    log_session_use "LAST_TX_DIGEST" "$LAST_TX_DIGEST"
    save_poc_rights_session
}

step_finalize_rights_governance() {
    local out digest voter
    require_session_fields \
        POC_CONFIG_ID POC_GOVERNANCE_REGISTRY_ID PROPOSAL_ID MEDIA_ASSET_ID \
        ECOSYSTEM_TREASURY_ID CLOCK_ID || return 1

    voter="$(governance_vote_wallet)" || return 1
    switch_wallet "$voter" || return 1
    log_step "finalize_media_asset_rights_governance_proposal proposal=$PROPOSAL_ID"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$voter" proof_of_creativity \
        finalize_media_asset_rights_governance_proposal \
        "@${POC_CONFIG_ID}" \
        "@${POC_GOVERNANCE_REGISTRY_ID}" \
        "@${PROPOSAL_ID}" \
        "@${MEDIA_ASSET_ID}" \
        "@${ECOSYSTEM_TREASURY_ID}" \
        "@${CLOCK_ID}")" || {
        restore_wallet
        return 1
    }
    restore_wallet
    digest="$(extract_tx_digest "$out")"
    wait_for_tx_finalized "$digest" || return 1
    FINALIZE_GOV_TX_DIGEST="$digest"
    log_session_use "FINALIZE_GOV_TX_DIGEST" "$FINALIZE_GOV_TX_DIGEST"
    save_poc_rights_session
}

step_implement_rights_direct() {
    local out digest oracle
    require_session_fields \
        POC_CONFIG_ID POC_GOVERNANCE_REGISTRY_ID PROPOSAL_ID MEDIA_ASSET_ID \
        ECOSYSTEM_TREASURY_ID NEW_RIGHTS_HOLDER_ADDRESS || return 1
    sync_poc_config_oracle_on_chain "$POC_DEFAULT_ORACLE_ADDRESS" || return 1
    ensure_poc_oracle_key_in_env || return 1

    oracle="$(read_poc_config_oracle_address)" || oracle="$POC_DEFAULT_ORACLE_ADDRESS"
    oracle="$(normalize_hex_id "$oracle")"

    log_step "oracle implement_media_asset_rights proposal=$PROPOSAL_ID"
    out="$(poc_rights_python - \
        "$POC_CONFIG_ID" "$POC_GOVERNANCE_REGISTRY_ID" "$PROPOSAL_ID" "$MEDIA_ASSET_ID" \
        "$ECOSYSTEM_TREASURY_ID" "$NEW_RIGHTS_HOLDER_ADDRESS" <<'PY'
import json, sys
from app.services.media_asset_rights_governance_submission import build_implement_media_asset_rights_move_call
from app.services.media_asset_submission import default_claims, default_usage_grants
from app.services.myso_client import init_myso_client

config_id, gov_id, proposal_id, asset_id, treasury_id, holder = sys.argv[1:7]
claims = default_claims(holder)
grants = default_usage_grants(0)
mc = build_implement_media_asset_rights_move_call(
    proposal_id=proposal_id,
    media_asset_id=asset_id,
    claims=claims,
    usage_grants=grants,
    reasoning="E2E DAO approved media asset rights update",
    evidence_urls=None,
    config_id=config_id,
    governance_registry_id=gov_id,
    treasury_id=treasury_id,
)
client = init_myso_client()
if client is None:
    raise SystemExit("MySocial client not configured")
result = client.submit_move_call(mc)
print(json.dumps(result))
PY
)" || return 1

    digest="$(echo "$out" | jq -r '.tx_hash // .digest // empty')"
    [[ -n "$digest" ]] || digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    [[ -n "$digest" ]] || {
        echo "implement tx did not return digest" >&2
        return 1
    }
    wait_for_tx_finalized "$digest" || return 1
    assert_rights_tx_events rights_implement "$digest" || return 1
    IMPLEMENT_TX_DIGEST="$digest"
    log_session_use "IMPLEMENT_TX_DIGEST" "$IMPLEMENT_TX_DIGEST"
    save_poc_rights_session
}

wait_for_oracle_implement_rights() {
    local attempt before after resp active
    require_session_fields MEDIA_ASSET_ID PROPOSAL_ID || return 1
    poc_oracle_stack_ready || return 1

    resp="$(gql_media_asset_snapshot "$MEDIA_ASSET_ID" 2>/dev/null)" || resp='{}'
    before="$(echo "$resp" | jq -r '.data.mediaAsset.rightsVersion // 0')"
    RIGHTS_VERSION_BEFORE="$before"
    log_session_use "RIGHTS_VERSION_BEFORE" "$RIGHTS_VERSION_BEFORE"

    log_step "Waiting for oracle-worker poc_gov_implement_rights (proposal=$PROPOSAL_ID)"
    for attempt in $(seq 1 90); do
        resp="$(gql_media_asset_snapshot "$MEDIA_ASSET_ID" 2>/dev/null)" || resp='{}'
        after="$(echo "$resp" | jq -r '.data.mediaAsset.rightsVersion // 0')"
        active="$(echo "$resp" | jq -r '.data.mediaAsset.activeRightsProposal.proposalId // empty')"
        if [[ -n "$after" && "$after" != "$before" && "$after" != "0" && "$after" != "null" ]]; then
            RIGHTS_VERSION_AFTER="$after"
            log_session_use "RIGHTS_VERSION_AFTER" "$RIGHTS_VERSION_AFTER"
            save_poc_rights_session
            return 0
        fi
        if [[ -z "$active" || "$active" == "null" ]] && [[ "$attempt" -gt 5 ]]; then
            RIGHTS_VERSION_AFTER="$after"
            log_session_use "RIGHTS_VERSION_AFTER" "$RIGHTS_VERSION_AFTER"
            save_poc_rights_session
            return 0
        fi
        if [[ $((attempt % 10)) -eq 0 ]]; then
            echo "  waiting for rights implement (attempt ${attempt}/90, rightsVersion=$after)…" >&2
        fi
        sleep 2
    done
    echo "Timed out waiting for oracle to implement approved rights proposal" >&2
    return 1
}

step_verify_graphql() {
    local resp asset_id version disputes active proposal_id proposals_count
    require_session_fields MEDIA_ASSET_ID || return 1
    asset_id="$(normalize_hex_id "$MEDIA_ASSET_ID")"
    resp="$(gql_media_asset_snapshot "$asset_id")" || {
        echo "GraphQL mediaAsset query failed" >&2
        return 1
    }
    version="$(echo "$resp" | jq -r '.data.mediaAsset.rightsVersion // empty')"
    disputes="$(echo "$resp" | jq -r '.data.mediaAsset.rightsDisputesSubmitted // empty')"
    active="$(echo "$resp" | jq -r '.data.mediaAsset.activeRightsProposal.proposalId // empty')"
    proposals_count="$(graphql_post \
        'query RightsProposals($id: ID!) {
            mediaAssetRightsProposals(mediaAssetId: $id, limit: 10) { proposalId status }
        }' \
        "$(jq -nc --arg id "$asset_id" '{id: $id}')" \
        | jq -r '.data.mediaAssetRightsProposals | length // 0' 2>/dev/null)" || proposals_count='0'
    [[ -n "$version" ]] || {
        echo "mediaAsset not indexed yet for $asset_id" >&2
        return 1
    }
    if [[ -n "${RIGHTS_VERSION_BEFORE:-}" && -n "${RIGHTS_VERSION_AFTER:-}" ]]; then
        if [[ "$RIGHTS_VERSION_AFTER" -le "$RIGHTS_VERSION_BEFORE" ]]; then
            echo "Expected rightsVersion to increase (before=$RIGHTS_VERSION_BEFORE after=$RIGHTS_VERSION_AFTER)" >&2
            return 1
        fi
    fi
    if [[ -n "$active" && "$active" != "null" ]]; then
        echo "WARN: activeRightsProposal still set ($active) after implement" >&2
    fi
    log_step "GraphQL OK mediaAsset=$asset_id rightsVersion=$version rightsDisputesSubmitted=$disputes proposalsIndexed=$proposals_count"
    print_run_summary_header "PoC Media Asset Rights E2E — verify OK"
    print_run_summary_line "Media asset" "$asset_id"
    print_run_summary_line "Proposal" "$(normalize_hex_id "${PROPOSAL_ID:-}")"
    print_run_summary_line "rightsVersion" "$version"
    print_run_summary_line "rightsDisputesSubmitted" "$disputes"
    print_run_summary_line "rightsProposals indexed" "$proposals_count"
    print_run_summary_line "Submit dispute tx" "${SUBMIT_DISPUTE_TX_DIGEST:-}"
    print_run_summary_line "Finalize gov tx" "${FINALIZE_GOV_TX_DIGEST:-}"
    print_run_summary_line "Implement tx" "${IMPLEMENT_TX_DIGEST:-}"
    print_run_summary_line "Session file" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_footer
}

run_resolve_asset_flow() {
    load_poc_rights_session
    SOCIAL_RUN_ID="$(date +%s)"
    require_session_fields POC_CONFIG_ID CLOCK_ID || {
        echo "Run --refresh-session first" >&2
        return 1
    }
    ensure_creator_wallet || return 1
    ensure_new_rights_holder || return 1

    mapfile -t _commits < <(deterministic_commitments_for_run)
    export POC_CONTENT_HEX="${_commits[0]}"
    export POC_FINGERPRINT_HEX="${_commits[1]}"

    step_submit_media_resolution || return 1
    if [[ "${POC_USE_DIRECT_MOVE}" == "1" ]]; then
        step_oracle_finalize_media_asset_direct || return 1
    else
        wait_for_oracle_resolve_media_asset || return 1
    fi
    log_step "MediaAsset ready: $(normalize_hex_id "$MEDIA_ASSET_ID")"
}

run_submit_dispute_flow() {
    load_poc_rights_session
    SOCIAL_RUN_ID="$(date +%s)"
    [[ -n "${MEDIA_ASSET_ID:-}" ]] || {
        echo "MEDIA_ASSET_ID missing; run --resolve-asset first" >&2
        return 1
    }
    step_submit_rights_dispute || return 1
    log_step "Rights dispute proposal: $(normalize_hex_id "$PROPOSAL_ID")"
}

run_fast_forward_governance_flow() {
    load_poc_rights_session
    [[ -n "${PROPOSAL_ID:-}" ]] || {
        echo "PROPOSAL_ID missing; run --submit-dispute first" >&2
        return 1
    }
    step_delegate_approve_proposal || return 1
    step_community_approve_proposal || return 1
    step_finalize_rights_governance || return 1
    log_step "Governance finalized for proposal $(normalize_hex_id "$PROPOSAL_ID")"
}

run_implement_flow() {
    load_poc_rights_session
    [[ -n "${PROPOSAL_ID:-}" && -n "${MEDIA_ASSET_ID:-}" ]] || {
        echo "PROPOSAL_ID and MEDIA_ASSET_ID required" >&2
        return 1
    }
    if [[ "${POC_WAIT_ORACLE}" == "1" && "${POC_USE_DIRECT_MOVE}" != "1" ]]; then
        wait_for_oracle_implement_rights || return 1
    else
        step_implement_rights_direct || return 1
    fi
}

run_all_flow() {
    load_poc_rights_session
    SOCIAL_RUN_ID="$(date +%s)"
    refresh_poc_rights_session_from_graphql || return 1
    step_tune_poc_governance_for_e2e || true
    poc_oracle_sync_worker_stack || true
    run_resolve_asset_flow || return 1
    run_submit_dispute_flow || return 1
    run_fast_forward_governance_flow || return 1
    run_implement_flow || return 1
    step_verify_graphql || return 1
}

show_menu() {
    echo ""
    echo "=== PoC Media Asset Rights E2E Menu ==="
    echo " 0) Refresh session from GraphQL"
    echo " 1) Resolve MediaAsset (submit resolution + oracle finalize)"
    echo " 2) Submit rights governance dispute"
    echo " 3) Fast-forward governance (delegate + community + finalize)"
    echo " 4) Wait for / run oracle implement"
    echo " 5) Verify GraphQL state"
    echo " 6) Run full flow (--run-all)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) refresh_poc_rights_session_from_graphql; load_poc_rights_session ;;
        1) run_resolve_asset_flow ;;
        2) run_submit_dispute_flow ;;
        3) run_fast_forward_governance_flow ;;
        4) run_implement_flow ;;
        5) step_verify_graphql ;;
        6) run_all_flow ;;
        [Hh]) usage ;;
        [Qq]) exit 0 ;;
        *) echo "Invalid choice" ;;
    esac
    show_menu
}

main() {
    poc_oracle_load_localnet_env
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h) usage; exit 0 ;;
            -y) ASSUME_YES=1; shift ;;
            --refresh-session) RUN_MODE=refresh; shift ;;
            --resolve-asset) RUN_MODE=resolve_asset; shift ;;
            --submit-dispute) RUN_MODE=submit_dispute; shift ;;
            --fast-forward-governance) RUN_MODE=fast_forward; shift ;;
            --wait-oracle-implement) RUN_MODE=implement; shift ;;
            --verify-gql) RUN_MODE=verify; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            --direct-move) POC_USE_DIRECT_MOVE=1; shift ;;
            --wait-oracle) POC_WAIT_ORACLE=1; shift ;;
            --no-wait-oracle) POC_WAIT_ORACLE=0; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    load_poc_rights_session

    case "${RUN_MODE:-}" in
        refresh) refresh_poc_rights_session_from_graphql; load_poc_rights_session; exit 0 ;;
        resolve_asset) run_resolve_asset_flow; exit 0 ;;
        submit_dispute) run_submit_dispute_flow; exit 0 ;;
        fast_forward) run_fast_forward_governance_flow; exit 0 ;;
        implement) run_implement_flow; exit 0 ;;
        verify) step_verify_graphql; exit 0 ;;
        run_all) run_all_flow; exit 0 ;;
        '') show_menu ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
