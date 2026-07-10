#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared PoC oracle E2E helpers: localnet keys, on-chain event asserts, oracle config sync.

: "${REPO_ROOT:?REPO_ROOT must be set before sourcing poc-oracle-common.sh}"

# shellcheck source=lib/social-runtime-common.sh
source "${REPO_ROOT}/scripts/lib/social-runtime-common.sh"

readonly POC_DEFAULT_ORACLE_ADDRESS='0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8'
readonly POC_DEFAULT_ORACLE_PRIVATE_KEY_HEX='736c869f584b6fdf1d961541e515304cdbeaf8e3d7789ae79fd05e2d9da34578'
readonly POC_ORACLE_LOCALNET_ENV="${REPO_ROOT}/network.config/poc/oracle-localnet.env"
readonly POC_MAX_DISPUTES_PER_POST='2'
readonly POC_MIN_VAULT_DEPOSIT='1'
readonly POC_DEFAULT_GAS_BUDGET='1000000000'

POC_ORACLE_URL="${POC_ORACLE_URL:-http://127.0.0.1:8001}"
POC_ORACLE_NETWORK="${POC_ORACLE_NETWORK:-localnet}"

POC_ORACLE_SESSION_KEYS=(
    POC_DEFAULT_ORACLE_ADDRESS POC_DEFAULT_ORACLE_PRIVATE_KEY_HEX
    POC_ORACLE_URL POC_ORACLE_NETWORK
    POC_CONFIG_ID POC_REGISTRY_ID POC_ADMIN_CAP_ID POC_VAULT_DIRECTORY_ID
    POC_BENEFICIARY_ADMIN_CAP_ID CLOCK_ID PKG_SOCIAL
    GRAPHQL_URL
)

poc_oracle_load_localnet_env() {
    if [[ -f "$POC_ORACLE_LOCALNET_ENV" ]]; then
        local line key val
        while IFS= read -r line || [[ -n "$line" ]]; do
            [[ "$line" =~ ^[[:space:]]*# ]] && continue
            [[ "$line" != *"="* ]] && continue
            key="${line%%=*}"
            val="${line#*=}"
            val="${val#\'}"; val="${val%\'}"
            case "$key" in
                POC_DEFAULT_ORACLE_ADDRESS|POC_DEFAULT_ORACLE_PRIVATE_KEY_HEX) continue ;;
            esac
            printf -v "$key" '%s' "$val"
        done < "$POC_ORACLE_LOCALNET_ENV"
    fi
    POC_ORACLE_URL="${POC_ORACLE_URL:-http://127.0.0.1:8001}"
    POC_ORACLE_NETWORK="${POC_ORACLE_NETWORK:-localnet}"
}

wait_for_tx_finalized() {
    local digest="$1" attempt json status
    [[ -n "$digest" ]] || return 1
    for attempt in $(seq 1 30); do
        json="$(myso client tx-block "$digest" --json 2>/dev/null)" || {
            sleep 0.5
            continue
        }
        status="$(echo "$json" | jq -r '.effects.V2.status // .effects.status // empty | tostring')"
        if [[ "$status" == "Success" ]]; then
            return 0
        fi
        if [[ -n "$status" && "$status" != "null" && "$status" != "Unknown" ]]; then
            echo "tx $digest finished with status=$status" >&2
            return 1
        fi
        sleep 0.5
    done
    echo "Timed out waiting for tx $digest to finalize" >&2
    return 1
}

poc_oracle_resolve_repo() {
    if [[ -n "${MYSO_POC_REPO:-}" ]]; then
        printf '%s' "$MYSO_POC_REPO"
        return 0
    fi
    if [[ -d "${REPO_ROOT}/../proof-of-creativity" ]]; then
        printf '%s' "${REPO_ROOT}/../proof-of-creativity"
        return 0
    fi
    printf '%s' "${REPO_ROOT}/../proof-of-creativity"
}

ensure_poc_oracle_key_in_env() {
    local poc_repo env_path
    poc_oracle_load_localnet_env
    poc_repo="$(poc_oracle_resolve_repo)"
    env_path="${poc_repo}/.env"
    [[ -d "$poc_repo" ]] || {
        echo "proof-of-creativity repo not found at $poc_repo" >&2
        return 1
    }
    python3 - "$env_path" <<'PY'
import sys
from pathlib import Path

env_path = Path(sys.argv[1])
lines = env_path.read_text(encoding="utf-8").splitlines() if env_path.is_file() else []
upserts = {
    "MYSO_INTEGRATION_ENABLED": "true",
    "MYSO_REFRESH_SESSION_OBJECTS": "true",
    "DISCOVERY_EMBED_SECRET": "discovery-secret",
    "DISCOVERY_ENABLED": "true",
    "POC_E2E_SUBMIT_OVERRIDE": "1",
    "POC_IDENTITY_VERIFIER": "mock",
    "ORACLE_PRIVATE_KEY_LOCALNET": "736c869f584b6fdf1d961541e515304cdbeaf8e3d7789ae79fd05e2d9da34578",
    "POC_ADMIN_PRIVATE_KEY_LOCALNET": "736c869f584b6fdf1d961541e515304cdbeaf8e3d7789ae79fd05e2d9da34578",
}
out = []
seen = set()
for line in lines:
    key = line.split("=", 1)[0] if "=" in line else ""
    if key in upserts:
        out.append(f"{key}={upserts[key]}")
        seen.add(key)
    else:
        out.append(line)
for key, val in upserts.items():
    if key not in seen:
        out.append(f"{key}={val}")
env_path.write_text("\n".join(out) + "\n", encoding="utf-8")
print(f"Updated {env_path}")
PY
}

list_tx_event_names() {
    local digest="$1" json
    [[ -n "$digest" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r '
        [.. | objects | select(has("type_") or has("type"))]
        | map(.type_ // .type)
        | map(.name? // empty)
        | .[]
    ' | sort -u
}

assert_tx_events() {
    local digest="$1" event
    shift
    [[ -n "$digest" ]] || {
        echo "assert_tx_events: missing digest" >&2
        return 1
    }
    for event in "$@"; do
        tx_has_event_named "$digest" "$event" || {
            echo "FAIL: tx $digest missing required event $event" >&2
            echo "  events present: $(list_tx_event_names "$digest" | tr '\n' ' ')" >&2
            return 1
        }
    done
    return 0
}

assert_tx_event_absent() {
    local digest="$1" event_name="$2"
    [[ -n "$digest" && -n "$event_name" ]] || return 1
    if tx_has_event_named "$digest" "$event_name"; then
        echo "FAIL: tx $digest unexpectedly emitted $event_name" >&2
        return 1
    fi
    return 0
}

extract_event_field() {
    tx_event_field "$@"
}

read_poc_config_oracle_address() {
    local json
    [[ -n "${POC_CONFIG_ID:-}" ]] || return 1
    json="$(myso client object "$POC_CONFIG_ID" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r '
        .. | objects | select(has("oracle_address")) | .oracle_address // empty
    ' | head -n1
}

ensure_poc_admin_signer() {
    local cap_owner active
    [[ -n "${POC_ADMIN_CAP_ID:-}" ]] || return 0
    cap_owner="$(object_address_owner "$POC_ADMIN_CAP_ID")" || return 0
    cap_owner="$(normalize_hex_id "$cap_owner")" || return 0
    active="$(resolve_myso_active_address)" || return 0
    if [[ "$active" != "$cap_owner" ]]; then
        log_step "Switching active address to PoCAdminCap owner $cap_owner"
        myso client switch --address "$cap_owner" >/dev/null
    fi
}

# Mirrors proof-of-creativity-runnable.sh run_myso_call (strips @ from shared object args).
poc_oracle_myso_call_capture() {
    local module="$1" func="$2" rc=0 out
    shift 2
    local -a cmd call_args=()
    local arg g
    while IFS= read -r -d '' arg; do call_args+=("$arg"); done < <(normalize_client_call_args "$@")
    cmd=(myso client call --package "${PKG_SOCIAL:-0x00000000000000000000000000000000000000000000000000000000000050c1}" \
        --module "$module" --function "$func")
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("${call_args[@]}")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    out="$("${cmd[@]}" 2>&1)" || rc=$?
    echo "$out" >&2
    printf '%s' "$out"
    return "$rc"
}

# Shared with proof-of-creativity-runnable.sh update_poc_config_oracle().
poc_oracle_update_config() {
    local oracle="$1" out digest
    local admin_cap config_id clock_id
    oracle="$(normalize_hex_id "$oracle")" || return 1
    require_session_fields POC_CONFIG_ID POC_ADMIN_CAP_ID CLOCK_ID || return 1
    require_hex_ids POC_CONFIG_ID POC_ADMIN_CAP_ID CLOCK_ID || return 1
    admin_cap="$(normalize_hex_id "$POC_ADMIN_CAP_ID")"
    config_id="$(normalize_hex_id "$POC_CONFIG_ID")"
    clock_id="$(normalize_hex_id "$CLOCK_ID")"
    ensure_poc_admin_signer || return 1
    log_step "Updating PoCConfig oracle_address=$oracle"
    out="$(SKIP_CONFIRM_RUN=1 poc_oracle_myso_call_capture proof_of_creativity update_poc_config \
        --args "$admin_cap" "@${config_id}" "$oracle" \
        95 95 95 100 \
        5000000000 1000000000 100000000000 \
        3000 5000 10 10000 \
        100 500 3000 \
        0 10000 10000 500 \
        "$POC_MAX_DISPUTES_PER_POST" "$POC_MIN_VAULT_DEPOSIT" \
        "@${clock_id}")" || {
        return 1
    }
    digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    if [[ -n "$digest" ]]; then
        wait_for_tx_finalized "$digest" || return 1
        assert_tx_events "$digest" PoCConfigUpdatedEvent || return 1
        POC_CONFIG_SYNC_DIGEST="$digest"
    fi
    return 0
}

poc_oracle_run_update_config() {
    poc_oracle_update_config "$1"
}

poc_oracle_sync_worker_stack() {
    poc_oracle_load_localnet_env
    sync_poc_config_oracle_on_chain "$POC_DEFAULT_ORACLE_ADDRESS" || return 1
    ensure_poc_oracle_key_in_env || return 1
    poc_oracle_health_ok || return 1
}

sync_poc_config_oracle_on_chain() {
    local oracle="${1:-$POC_DEFAULT_ORACLE_ADDRESS}" current
    poc_oracle_load_localnet_env
    oracle="$(normalize_hex_id "$oracle")" || return 1
    ensure_wallet_funded "$oracle" "$((POC_DEFAULT_GAS_BUDGET * 2))" || true
    current="$(read_poc_config_oracle_address)" || true
    if [[ -n "$current" ]] && [[ "$(normalize_hex_id "$current")" == "$oracle" ]] \
        && [[ "${POC_FORCE_UPDATE_CONFIG:-0}" != 1 ]]; then
        log_step "PoCConfig oracle already $oracle"
        return 0
    fi
    poc_oracle_run_update_config "$oracle"
}

assert_poc_scenario_events() {
    local scenario="$1" digest="$2"
    [[ -n "$digest" ]] || return 1
    case "$scenario" in
        oracle_config_sync)
            assert_tx_events "$digest" PoCConfigUpdatedEvent
            ;;
        post_created)
            assert_tx_events "$digest" PostCreatedEvent
            ;;
        reservation_pool_created)
            assert_tx_events "$digest" ReservationPoolCreatedEvent
            ;;
        analyze_original)
            assert_tx_events "$digest" AnalysisSubmittedEvent
            assert_tx_event_absent "$digest" RevenueRedirectionActivatedEvent
            ;;
        analyze_badge)
            assert_tx_events "$digest" AnalysisSubmittedEvent PoCBadgeIssuedEvent
            ;;
        analyze_derivative)
            assert_tx_events "$digest" AnalysisSubmittedEvent RevenueRedirectionActivatedEvent PoCResultAppliedEvent
            ;;
        self_match)
            assert_tx_events "$digest" AnalysisSubmittedEvent PoCBadgeIssuedEvent
            assert_tx_event_absent "$digest" RevenueRedirectionActivatedEvent
            ;;
        royalty_free)
            assert_tx_events "$digest" AnalysisSubmittedEvent PoCResultAppliedEvent
            assert_tx_event_absent "$digest" RevenueRedirectionActivatedEvent
            ;;
        tip_post)
            assert_tx_events "$digest" TipEvent PoCBeneficiaryVaultDepositEvent
            ;;
        spt_reserve)
            assert_tx_events "$digest" ReservationCreatedEvent
            ;;
        username_provision)
            assert_tx_events "$digest" UsernameBeneficiaryProvisionedEvent
            ;;
        username_claim)
            assert_tx_events "$digest" UsernameBeneficiaryClaimedEvent CreatorIdentityWalletLinkedEvent
            ;;
        dispute_submit)
            assert_tx_events "$digest" PoCDisputeSubmittedEvent
            ;;
        dispute_vote)
            assert_tx_events "$digest" DisputeVoteCastEvent
            ;;
        dispute_resolve)
            assert_tx_events "$digest" PoCDisputeResolvedEvent
            ;;
        *)
            echo "Unknown PoC scenario: $scenario" >&2
            return 1
            ;;
    esac
}

poc_oracle_stack_ready() {
    local base="${POC_ORACLE_URL%/}" gql="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
    poc_oracle_load_localnet_env
    curl -sf "${base}/health" >/dev/null 2>&1 || {
        echo "PoC API not reachable at ${base}/health" >&2
        return 1
    }
    if declare -F poc_oracle_health_ok >/dev/null 2>&1; then
        poc_oracle_health_ok || return 1
    fi
    curl -sf -X POST "$gql" -H 'content-type: application/json' \
        -d '{"query":"{ __typename }"}' >/dev/null 2>&1 || {
        echo "GraphQL not reachable at $gql" >&2
        return 1
    }
    log_step "PoC stack ready (api=$base graphql=$gql)"
    return 0
}

wait_for_poc_post_attested() {
    local post_id="$1" timeout_sec="${2:-180}" base deadline resp status digest
    base="${POC_ORACLE_URL%/}"
    deadline=$(( $(date +%s) + timeout_sec ))
    while [[ $(date +%s) -lt "$deadline" ]]; do
        resp="$(curl -sf "${base}/oracle/posts/${post_id}?network=${POC_ORACLE_NETWORK}" 2>/dev/null)" || resp=''
        status="$(echo "$resp" | jq -r '.analysis_status // empty')"
        digest="$(echo "$resp" | jq -r '.tx_digest // empty')"
        if [[ "$status" == "attested" && -n "$digest" ]]; then
            ANALYZE_TX_DIGEST="$digest"
            printf '%s' "$digest"
            return 0
        fi
        sleep 2
    done
    echo "Timed out waiting for PoC attestation on post $post_id" >&2
    return 1
}

gql_cross_check_post_poc() {
    local post_id="$1" attempt resp
    post_id="$(normalize_hex_id "$post_id")" || return 1
    for attempt in $(seq 1 30); do
        resp="$(graphql_post \
            'query($id: ID!) { post(id: $id) { id enableSpt mediaUrls pocOutcome revenueRedirectTo } }' \
            "$(jq -nc --arg id "$post_id" '{id: $id}')" 2>/dev/null)" || resp='{}'
        if [[ -n "$(echo "$resp" | jq -r '.data.post.id // empty')" ]]; then
            printf '%s' "$resp"
            return 0
        fi
        sleep 1
    done
    echo "WARN: GraphQL has not indexed post $post_id yet (chain events passed)" >&2
    return 0
}
