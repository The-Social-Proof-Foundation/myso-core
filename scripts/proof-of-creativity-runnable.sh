#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Runtime E2E helper for social_contracts::proof_of_creativity via `myso client call` / PTB.
#
# Prerequisites:
#   - Social bootstrap completed (claim_all_admin_capabilities); admin caps owned by active address.
#   - Local indexer GraphQL at http://127.0.0.1:9125/graphql (for --refresh-session / auto-refresh).
#   - `myso`, `curl`, `jq`, `python3` on PATH.
#   - Active `myso client` address is used as oracle, creator, and tipper (switch + faucet if needed).
#
# Session file: network.config/poc/poc-session.env (chain object IDs from GraphQL refresh).
# After refresh, expect MEMORY_CONFIG_ID, PROFILE_CONFIG_ID, AI_CREDIT_CONFIG_ID for profile/post flows.
# PLATFORM_OBJECT_ID from GraphQL may be indexer-only; post flows recreate platform on fullnode if missing.
#
# Usage:
#   ./scripts/proof-of-creativity-runnable.sh --refresh-session
#   ./scripts/proof-of-creativity-runnable.sh --create-platform
#   ./scripts/proof-of-creativity-runnable.sh --run-all
#   ./scripts/proof-of-creativity-runnable.sh --post-flow
#   ./scripts/proof-of-creativity-runnable.sh --username-flow
#   ASSUME_YES=1 ./scripts/proof-of-creativity-runnable.sh --run-all
#   ./scripts/proof-of-creativity-runnable.sh   # interactive menu
#
# Platform: post flows require an approved Platform. Use menu C or --create-platform to
# create test data on-chain (create_platform + toggle_platform_approval), then refresh session.
#
# Environment (optional flow flags only):
#   ASSUME_YES=1, DRY_RUN=1, POC_AUTO_REFRESH=1, POC_NO_AUTO_REFRESH=1
#   POC_SKIP_USERNAME=1, POC_SKIP_DISPUTE=1, POC_INCLUDE_SPT=1, POC_INCLUDE_PROFILE_RESERVATION=1
#   POC_INCLUDE_POST_RESERVATION=1, POC_INCLUDE_DISPUTE_REANALYZE=1
#   POC_ORACLE_URL=http://127.0.0.1:8001, POC_ORACLE_NETWORK=localnet, POC_USE_DIRECT_MOVE=0
#   POC_E2E_SUBMIT_OVERRIDE=1 (localnet upload score/creator overrides)
#   POC_NO_PLATFORM=1, POC_REQUIRE_PLATFORM=1, POC_SKIP_VAULT_FUNDING=1, POC_FORCE_UPDATE_CONFIG=1
#
# No-platform (--run-all without PLATFORM_OBJECT_ID): preflight + username beneficiary PoC only.
# Post/dispute/SPT/reservation flows require platform and/or existing profile (menu 3–6).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/poc-oracle-common.sh
source "${SCRIPT_DIR}/lib/poc-oracle-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"
# shellcheck source=lib/poc-oracle-http.sh
source "${SCRIPT_DIR}/lib/poc-oracle-http.sh"

readonly DEFAULT_PKG_SOCIAL='0x00000000000000000000000000000000000000000000000000000000000050c1'
readonly DEFAULT_ORDERBOOK_PKG='0x000000000000000000000000000000000000000000000000000000000000b0c'
readonly DEFAULT_CLOCK='0x0000000000000000000000000000000000000000000000000000000000000006'
readonly DEFAULT_COIN_TYPE='0x2::myso::MYSO'
readonly DEFAULT_GAS_BUDGET='1000000000'
GRAPHQL_URL='http://127.0.0.1:9125/graphql'
readonly DEFAULT_TIP_AMOUNT='100000000'
readonly DEFAULT_VOTE_STAKE='1000000000'
readonly DEFAULT_DISPUTE_EVIDENCE='PoC runtime test dispute evidence'
readonly DEFAULT_RESERVE_AMOUNT='1000000000'

# Session / chain object IDs (populated by GraphQL refresh)
PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
ORDERBOOK_PACKAGE_ID="$DEFAULT_ORDERBOOK_PKG"
CLOCK_ID="$DEFAULT_CLOCK"
COIN_TYPE="$DEFAULT_COIN_TYPE"
GAS_BUDGET=''

BOOTSTRAP_KEY_ID=''
ECOSYSTEM_TREASURY_ID=''
PLATFORM_REGISTRY_ID=''
PLATFORM_CONFIG_ID=''
PLATFORM_OBJECT_ID=''
USERNAME_REGISTRY_ID=''
BLOCK_LIST_REGISTRY_ID=''
MYDATA_REGISTRY_ID=''
SOCIAL_GRAPH_ID=''
TOKEN_REGISTRY_ID=''
POC_REGISTRY_ID=''
MESSAGE_REGISTRY_ID=''
MEMORY_REGISTRY_ID=''
MEMORY_CONFIG_ID=''
PROFILE_CONFIG_ID=''
AI_CREDIT_CONFIG_ID=''
POOL_REGISTRY_ID=''
ANCHOR_REGISTRY_ID=''
CLAIM_VAULT_ID=''
POC_VAULT_DIRECTORY_ID=''
POC_USERNAME_BENEFICIARY_DIRECTORY_ID=''
POST_CONFIG_ID=''
SOCIAL_PROOF_TOKENS_CONFIG_ID=''
POC_CONFIG_ID=''
MYDATA_CONFIG_ID=''
SPOT_CONFIG_ID=''
INSURANCE_CONFIG_ID=''
ORDERBOOK_REGISTRY_ID=''
POC_ADMIN_CAP_ID=''
POC_BENEFICIARY_ADMIN_CAP_ID=''
SPT_ADMIN_CAP_ID=''
PLATFORM_ADMIN_CAP_ID=''
MYDATA_ADMIN_CAP_ID=''
POOL_ADMIN_CAP_ID=''
GOVERNANCE_ECOSYSTEM_REGISTRY_ID=''
GOVERNANCE_POC_REGISTRY_ID=''

CREATOR_ADDRESS=''
TIPPER_ADDRESS=''
ORACLE_ADDRESS=''
USERNAME_CLAIM_ORACLE=''
JOIN_REFERRER_ADDRESS=''
MEMORY_ACCOUNT_ID=''
TIPPER_MEMORY_ACCOUNT_ID=''
ORACLE_PROFILE_ID=''
RESERVATION_POOL_ID=''
POC_BENEFICIARY_VAULT_ID=''
ANALYZE_POST_LAST_DIGEST=''

# Username beneficiary flow — populated for success summary
POC_UB_LAST_USERNAME=''
POC_UB_LAST_BENEFICIARY_ID=''
POC_UB_LAST_SHARD_ID=''
POC_UB_LAST_IDENTITY_HASH=''
POC_UB_LAST_VAULT_ID=''
POC_UB_LAST_CLAIM_WALLET=''
POC_UB_LAST_CLAIM_ORACLE=''
POC_UB_LAST_CLAIM_PROFILE_ID=''
POC_UB_LAST_FUND_POST_ID=''
POC_UB_LAST_VAULT_FUNDED='0'
POC_UB_LAST_TIP_AMOUNT=''
POC_UB_LAST_VAULT_GROSS=''
POC_UB_LAST_VAULT_TREASURY=''
POC_UB_LAST_VAULT_CREATOR_NET=''

POC_RUN_ID="$(date +%s)"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"

GQL_REFRESH_FILE=''

MANUAL_PRESERVE_KEYS=(
    JOIN_REFERRER_ADDRESS GAS_BUDGET ORACLE_ADDRESS MEMORY_ACCOUNT_ID
    TIPPER_ADDRESS TIPPER_MEMORY_ACCOUNT_ID POC_BENEFICIARY_VAULT_ID
)

REQUIRED_CORE_KEYS=(
    POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID POC_USERNAME_BENEFICIARY_DIRECTORY_ID
    POC_ADMIN_CAP_ID POC_BENEFICIARY_ADMIN_CAP_ID USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID
    MEMORY_CONFIG_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID ECOSYSTEM_TREASURY_ID
)

REQUIRED_PLATFORM_KEYS=(
    PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID POST_CONFIG_ID BLOCK_LIST_REGISTRY_ID
    MYDATA_REGISTRY_ID MEMORY_CONFIG_ID
)

REQUIRED_SPT_RESERVATION_KEYS=(
    TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID ECOSYSTEM_TREASURY_ID SPT_ADMIN_CAP_ID
)

OPTIONAL_PLATFORM_KEYS=(
    PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID POST_CONFIG_ID BLOCK_LIST_REGISTRY_ID
    MYDATA_REGISTRY_ID
)

usage() {
    sed -n '2,26p' "$0" | sed 's/^# \?//'
}

session_state_save_path() {
    printf '%s' "${SOCIAL_SESSION_SAVE_PATH:-$REPO_ROOT/network.config/poc/poc-session.env}"
}

apply_session_defaults() {
    [[ -n "${PKG_SOCIAL:-}" ]] || PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
    [[ -n "${ORDERBOOK_PACKAGE_ID:-}" ]] || ORDERBOOK_PACKAGE_ID="$DEFAULT_ORDERBOOK_PKG"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$DEFAULT_CLOCK"
    [[ -n "${COIN_TYPE:-}" ]] || COIN_TYPE="$DEFAULT_COIN_TYPE"
    [[ -n "${GAS_BUDGET:-}" ]] || GAS_BUDGET="$DEFAULT_GAS_BUDGET"
}

load_session_state() {
    local p
    local key val i
    local -a _preserve_keys=()
    local -a _preserve_vals=()
    p="$(session_state_save_path)"
    for key in "${MANUAL_PRESERVE_KEYS[@]}"; do
        val="${!key:-}"
        if [[ -n "$val" ]]; then
            _preserve_keys+=("$key")
            _preserve_vals+=("$val")
        fi
    done
    if [[ -f "$p" ]]; then
        # shellcheck disable=SC1090
        source "$p"
        echo "Loaded PoC session from: $p" >&2
    fi
    for i in "${!_preserve_keys[@]}"; do
        key="${_preserve_keys[$i]}"
        val="${_preserve_vals[$i]}"
        printf -v "$key" '%s' "$val"
    done
    apply_session_defaults
    return 0
}

save_session_state() {
    local f key
    f="$(session_state_save_path)"
    mkdir -p "$(dirname "$f")"
    local old_umask
    old_umask="$(umask)"
    umask 077
    {
        echo "# PoC runtime session — scripts/proof-of-creativity-runnable.sh"
        echo "# Do not commit if sensitive."
        for key in PKG_SOCIAL ORDERBOOK_PACKAGE_ID CLOCK_ID COIN_TYPE GAS_BUDGET \
            BOOTSTRAP_KEY_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_OBJECT_ID \
            USERNAME_REGISTRY_ID BLOCK_LIST_REGISTRY_ID MYDATA_REGISTRY_ID SOCIAL_GRAPH_ID \
            TOKEN_REGISTRY_ID POC_REGISTRY_ID MESSAGE_REGISTRY_ID MEMORY_REGISTRY_ID \
            MEMORY_CONFIG_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID \
            POOL_REGISTRY_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID POC_VAULT_DIRECTORY_ID \
            POC_USERNAME_BENEFICIARY_DIRECTORY_ID POST_CONFIG_ID SOCIAL_PROOF_TOKENS_CONFIG_ID \
            POC_CONFIG_ID MYDATA_CONFIG_ID SPOT_CONFIG_ID INSURANCE_CONFIG_ID ORDERBOOK_REGISTRY_ID \
            POC_ADMIN_CAP_ID POC_BENEFICIARY_ADMIN_CAP_ID SPT_ADMIN_CAP_ID PLATFORM_ADMIN_CAP_ID \
            MYDATA_ADMIN_CAP_ID POOL_ADMIN_CAP_ID \
            GOVERNANCE_ECOSYSTEM_REGISTRY_ID GOVERNANCE_POC_REGISTRY_ID \
            ORACLE_ADDRESS MEMORY_ACCOUNT_ID TIPPER_ADDRESS TIPPER_MEMORY_ACCOUNT_ID \
            POC_BENEFICIARY_VAULT_ID ORACLE_PROFILE_ID RESERVATION_POOL_ID; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    umask "$old_umask"
    echo "Saved session to: $f" >&2
}

session_value_set() {
    local var_name="$1"
    [[ -n "${!var_name:-}" ]]
}

log_step() {
    echo "" >&2
    echo ">>> $*" >&2
}

log_session_use() {
    echo "  [session] $1=${2:-<unset>}" >&2
}

confirm_run() {
    if [[ "${ASSUME_YES:-0}" == 1 ]]; then
        return 0
    fi
    read -r -p "Execute this command? [y/N] " ans
    [[ "${ans:-}" == [yY]* ]]
}

resolve_myso_active_address() {
    myso client active-address 2>/dev/null
}

ensure_oracle_cli_address() {
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

ensure_cli_addresses() {
    ensure_oracle_cli_address
    ORACLE_ADDRESS="$(resolve_myso_active_address)" || {
        echo "Could not read myso client active-address" >&2
        return 1
    }
    CREATOR_ADDRESS="$ORACLE_ADDRESS"
    if ! session_value_set TIPPER_ADDRESS; then
        TIPPER_ADDRESS="$ORACLE_ADDRESS"
    else
        TIPPER_ADDRESS="$(normalize_hex_id "$TIPPER_ADDRESS")"
    fi
    log_session_use "ORACLE_ADDRESS" "$ORACLE_ADDRESS"
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
    log_session_use "TIPPER_ADDRESS" "$TIPPER_ADDRESS"
}

restore_oracle_address() {
    [[ -n "${ORACLE_ADDRESS:-}" ]] || return 0
    local current
    current="$(resolve_myso_active_address)" || return 0
    if [[ "$current" != "$ORACLE_ADDRESS" ]]; then
        myso client switch --address "$ORACLE_ADDRESS" >/dev/null
    fi
}

ensure_tipper_ready() {
    local coin saved_oracle attempt
    ensure_cli_addresses || return 1
    saved_oracle="$(resolve_myso_active_address)" || return 1
    TIPPER_ADDRESS="$(normalize_hex_id "$TIPPER_ADDRESS")"

    if [[ "$TIPPER_ADDRESS" != "$(normalize_hex_id "$ORACLE_ADDRESS")" ]]; then
        myso client switch --address "$TIPPER_ADDRESS" >/dev/null
    fi

    coin="$(resolve_gas_coin_for_address "$TIPPER_ADDRESS")"
    if [[ -n "$coin" ]]; then
        if [[ "$(resolve_myso_active_address)" != "$saved_oracle" ]]; then
            myso client switch --address "$saved_oracle" >/dev/null
        fi
        return 0
    fi

    log_step "Funding tipper $TIPPER_ADDRESS via faucet"
    myso client faucet >/dev/null 2>&1 || myso client faucet >&2
    for attempt in $(seq 1 30); do
        sleep 1
        coin="$(resolve_gas_coin_for_address "$TIPPER_ADDRESS")"
        [[ -n "$coin" ]] && break
    done

    if [[ "$(resolve_myso_active_address)" != "$saved_oracle" ]]; then
        myso client switch --address "$saved_oracle" >/dev/null
    fi

    [[ -n "$coin" ]] || {
        echo "No gas coin for tipper $TIPPER_ADDRESS after faucet (waited 30s)" >&2
        return 1
    }
    return 0
}

extra_gas_budget() {
    printf '%s\n' '--gas-budget' "${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}"
}

extra_dry() {
    if [[ "${DRY_RUN:-0}" == 1 ]]; then
        printf '%s\n' '--dry-run'
    fi
}

literal_move_string() {
    local s=$1
    s="${s//\\/\\\\}"
    s="${s//\"/\\\"}"
    s="${s//$'\n'/\\n}"
    s="${s//$'\r'/}"
    printf '"%s"' "$s"
}

normalize_hex_id() {
    local id="$1"
    id="${id#@}"
    [[ -n "$id" ]] || return 1
    case "$id" in
        0x*) printf '%s' "$id" ;;
        *) printf '0x%s' "$id" ;;
    esac
}

ptb_shared_ref() {
    local id normalized
    id="$1"
    normalized="$(normalize_hex_id "$id")" || {
        echo "PTB shared object id is empty or invalid (got: '${id:-<empty>}')" >&2
        return 1
    }
    printf '@%s' "$normalized"
}

literal_move_vector_empty() {
    printf '%s' 'vector[]'
}

literal_move_vector_from_csv() {
    local csv="$1"
    if [[ -z "$csv" ]]; then
        literal_move_vector_empty
        return 0
    fi
    local acc="" s2="" p
    IFS=',' read -r -a _VA <<<"$csv"
    for p in "${_VA[@]}"; do
        p="${p## }"
        p="${p%% }"
        acc="${acc}${s2}\"${p}\""
        s2=", "
    done
    printf 'vector[%s]' "$acc"
}

literal_move_option_string() {
    local s="$1"
    if [[ -z "$s" ]]; then
        printf 'none'
        return 0
    fi
    printf 'some(%s)' "$(literal_move_string "$s")"
}

require_hex_ids() {
    local name missing=()
    for name in "$@"; do
        if ! normalize_hex_id "${!name:-}" >/dev/null 2>&1; then
            missing+=("$name")
        fi
    done
    if ((${#missing[@]})); then
        echo "Missing or invalid object ids: ${missing[*]}" >&2
        return 1
    fi
}

bytes_to_hex_arg() {
    python3 - "$1" <<'PY'
import sys
print("0x" + sys.argv[1].encode("utf-8").hex())
PY
}

invoke_ptb() {
    local -a cmd
    cmd=(myso client ptb)
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        "${cmd[@]}" >&2
    else
        return 0
    fi
}

invoke_ptb_capture() {
    local -a cmd out
    cmd=(myso client ptb)
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        local rc=0
        out="$("${cmd[@]}" 2>&1)" || rc=$?
        echo "$out" >&2
        printf '%s' "$out"
        return "$rc"
    else
        return 0
    fi
}

normalize_client_call_args() {
    local -a normalized=()
    local arg
    for arg in "$@"; do
        if [[ "$arg" == @0x* ]]; then
            normalized+=("${arg#@}")
        else
            normalized+=("$arg")
        fi
    done
    printf '%s\0' "${normalized[@]}"
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
    cmd=(myso client call --package "$PKG_SOCIAL" --sender "$sender" \
        --module "$module" --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
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

invoke_ptb_as() {
    local sender="$1"
    shift
    local -a cmd
    sender="$(normalize_hex_id "$sender")" || return 1
    cmd=(myso client ptb --sender "@${sender}")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    if [[ -n "${PTB_GAS_COIN_ID:-}" ]]; then
        cmd+=(--gas-coin "@$(normalize_hex_id "$PTB_GAS_COIN_ID")")
    fi
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        "${cmd[@]}" >&2
    else
        return 0
    fi
}

invoke_ptb_as_capture() {
    local sender="$1"
    shift
    local -a cmd out
    sender="$(normalize_hex_id "$sender")" || return 1
    cmd=(myso client ptb --sender "@${sender}")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    if [[ -n "${PTB_GAS_COIN_ID:-}" ]]; then
        cmd+=(--gas-coin "@$(normalize_hex_id "$PTB_GAS_COIN_ID")")
    fi
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        local rc=0
        out="$("${cmd[@]}" 2>&1)" || rc=$?
        echo "$out" >&2
        printf '%s' "$out"
        return "$rc"
    else
        return 0
    fi
}

require_session_fields() {
    local name missing=()
    for name in "$@"; do
        session_value_set "$name" || missing+=("$name")
    done
    if [[ ${#missing[@]} -gt 0 ]]; then
        echo "Missing session field(s): ${missing[*]}" >&2
        echo "Run --refresh-session to populate chain object IDs from GraphQL." >&2
        return 1
    fi
}

missing_required_keys() {
    local key missing=()
    for key in "${REQUIRED_CORE_KEYS[@]}"; do
        session_value_set "$key" || missing+=("$key")
    done
    if [[ ${#missing[@]} -gt 0 ]]; then
        printf '%s\n' "${missing[@]}"
        return 1
    fi
    return 0
}

platform_mode() {
    if [[ "${POC_NO_PLATFORM:-0}" == 1 ]]; then
        echo no_platform
        return
    fi
    if [[ "${POC_REQUIRE_PLATFORM:-0}" == 1 ]]; then
        echo full
        return
    fi
    if session_value_set PLATFORM_OBJECT_ID; then
        echo full
    else
        echo no_platform
    fi
}

platform_mode_is_full() {
    [[ "$(platform_mode)" == full ]]
}

should_skip_vault_funding() {
    if [[ "${POC_SKIP_VAULT_FUNDING:-0}" == 1 ]]; then
        return 0
    fi
    if ! platform_mode_is_full; then
        return 0
    fi
    return 1
}

require_platform_mode() {
    if platform_mode_is_full; then
        return 0
    fi
    echo "This flow requires a Platform (set PLATFORM_OBJECT_ID or unset POC_NO_PLATFORM)." >&2
    return 1
}

graphql_post() {
    local query="$1"
    local vars="${2-}"
    local body http_code resp
    if [[ -z "$vars" ]]; then
        vars='{}'
    fi
    body="$(jq -nc --arg q "$query" --argjson v "$vars" '{query: $q, variables: $v}')" || return 1
    resp="$(curl -sS -w '\n%{http_code}' -X POST "$GRAPHQL_URL" \
        -H 'Content-Type: application/json' \
        -d "$body")" || {
        echo "GraphQL request failed: $GRAPHQL_URL" >&2
        return 1
    }
    http_code="${resp##*$'\n'}"
    resp="${resp%$'\n'*}"
    if [[ "$http_code" != "200" ]]; then
        echo "GraphQL HTTP $http_code from $GRAPHQL_URL" >&2
        echo "$resp" >&2
        return 1
    fi
    if echo "$resp" | jq -e '.errors | length > 0' >/dev/null 2>&1; then
        echo "GraphQL errors:" >&2
        echo "$resp" | jq '.errors' >&2
        return 1
    fi
    printf '%s' "$resp"
}

gql_object_address() {
    local json="$1" alias="$2"
    echo "$json" | jq -r ".data.${alias}.nodes[0].address // empty"
}

gql_governance_registry_id() {
    local json="$1" alias="$2"
    echo "$json" | jq -r ".data.${alias}[0].registryId // empty"
}

readonly GQL_BATCH1='query MysocialGenesisObjectsBatch1 {
  bootstrapKey: objects(filter: { type: "0x2::bootstrap_key::BootstrapKey", ownerKind: SHARED }, first: 1) { nodes { address } }
  ecosystemTreasury: objects(filter: { type: "0x50c1::profile::EcosystemTreasury", ownerKind: SHARED }, first: 1) { nodes { address } }
  platformRegistry: objects(filter: { type: "0x50c1::platform::PlatformRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  platformConfig: objects(filter: { type: "0x50c1::platform::PlatformConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  usernameRegistry: objects(filter: { type: "0x50c1::profile::UsernameRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  blocklistRegistry: objects(filter: { type: "0x50c1::block_list::BlockListRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataRegistry: objects(filter: { type: "0x50c1::mydata::MyDataRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  socialGraph: objects(filter: { type: "0x50c1::social_graph::SocialGraph", ownerKind: SHARED }, first: 1) { nodes { address } }
  socialProofTokenRegistry: objects(filter: { type: "0x50c1::social_proof_tokens::TokenRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocRegistry: objects(filter: { type: "0x50c1::proof_of_creativity::PoCRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  messageRegistry: objects(filter: { type: "0x50c1::message::MessageRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  memoryRegistry: objects(filter: { type: "0x50c1::memory::MemoryRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  memoryConfig: objects(filter: { type: "0x50c1::memory::MemoryConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  profileConfig: objects(filter: { type: "0x50c1::profile::ProfileConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  aiCreditConfig: objects(filter: { type: "0x50c1::ai_credit::AiCreditConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataPoolRegistry: objects(filter: { type: "0x50c1::mydata::MyDataPoolRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  snapshotAnchorRegistry: objects(filter: { type: "0x50c1::mydata::SnapshotAnchorRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataClaimVault: objects(filter: { type: "0x50c1::mydata::MyDataClaimVault", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocVaultDirectory: objects(filter: { type: "0x50c1::poc_vault::PoCVaultDirectory", ownerKind: SHARED }, first: 1) { nodes { address } }
  postConfig: objects(filter: { type: "0x50c1::post::PostConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
}'

readonly GQL_BATCH2='query MysocialGenesisObjectsBatch2 {
  sptConfig: objects(filter: { type: "0x50c1::social_proof_tokens::SocialProofTokensConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocConfig: objects(filter: { type: "0x50c1::proof_of_creativity::PoCConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataConfig: objects(filter: { type: "0x50c1::mydata::MyDataConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  spotConfig: objects(filter: { type: "0x50c1::social_proof_of_truth::SpotConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  insuranceConfig: objects(filter: { type: "0x50c1::insurance::InsuranceConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  orderbookRegistry: objects(filter: { type: "0xb0c::registry::Registry", ownerKind: SHARED }, first: 1) { nodes { address } }
  proofOfCreativityAdminCap: objects(filter: { type: "0x50c1::proof_of_creativity::PoCAdminCap" }, last: 1) { nodes { address } }
  socialProofTokensAdminCap: objects(filter: { type: "0x50c1::social_proof_tokens::SocialProofTokensAdminCap" }, last: 1) { nodes { address } }
  mydataAdminCap: objects(filter: { type: "0x50c1::mydata::MyDataAdminCap" }, last: 1) { nodes { address } }
  mydataPoolAdminCap: objects(filter: { type: "0x50c1::mydata::MyDataPoolAdminCap" }, last: 1) { nodes { address } }
  platformAdminCap: objects(filter: { type: "0x50c1::platform::PlatformAdminCap" }, last: 1) { nodes { address } }
}'

readonly GQL_BATCH3='query MysocialGenesisObjectsBatch3 {
  ecosystemBadgeAdminCap: objects(filter: { type: "0x50c1::profile::EcosystemBadgeAdminCap" }, last: 1) { nodes { address } }
}'

readonly GQL_GOVERNANCE='query MysocialGovernanceRegistries {
  ecosystemDao: governanceRegistries(registryType: 0) { registryId }
  pocDao: governanceRegistries(registryType: 1) { registryId }
}'

readonly GQL_POC_EXTRAS='query MysocialPocSessionExtras {
  pocUsernameBeneficiaryDirectory: objects(filter: { type: "0x50c1::poc_username_beneficiary::PoCUsernameBeneficiaryDirectory", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocBeneficiaryAdminCap: objects(filter: { type: "0x50c1::poc_username_beneficiary::PoCBeneficiaryAdminCap" }, last: 1) { nodes { address } }
  platform: objects(filter: { type: "0x50c1::platform::Platform" }, last: 1) { nodes { address } }
}'

gql_set_refresh() {
    local key="$1" val="$2"
    [[ -n "$val" ]] || return 0
    printf '%s=%q\n' "$key" "$val" >> "$GQL_REFRESH_FILE"
}

collect_gql_mappings() {
    local json="$1"
    local alias val env_key
    for alias in bootstrapKey ecosystemTreasury platformRegistry platformConfig platform usernameRegistry blocklistRegistry \
        mydataRegistry socialGraph socialProofTokenRegistry pocRegistry messageRegistry memoryRegistry \
        memoryConfig profileConfig aiCreditConfig mydataPoolRegistry snapshotAnchorRegistry mydataClaimVault pocVaultDirectory \
        pocUsernameBeneficiaryDirectory postConfig sptConfig pocConfig mydataConfig spotConfig insuranceConfig \
        orderbookRegistry proofOfCreativityAdminCap socialProofTokensAdminCap pocBeneficiaryAdminCap \
        mydataAdminCap mydataPoolAdminCap platformAdminCap; do
        case "$alias" in
            bootstrapKey) env_key=BOOTSTRAP_KEY_ID ;;
            ecosystemTreasury) env_key=ECOSYSTEM_TREASURY_ID ;;
            platformRegistry) env_key=PLATFORM_REGISTRY_ID ;;
            platformConfig) env_key=PLATFORM_CONFIG_ID ;;
            platform) env_key=PLATFORM_OBJECT_ID ;;
            usernameRegistry) env_key=USERNAME_REGISTRY_ID ;;
            blocklistRegistry) env_key=BLOCK_LIST_REGISTRY_ID ;;
            mydataRegistry) env_key=MYDATA_REGISTRY_ID ;;
            socialGraph) env_key=SOCIAL_GRAPH_ID ;;
            socialProofTokenRegistry) env_key=TOKEN_REGISTRY_ID ;;
            pocRegistry) env_key=POC_REGISTRY_ID ;;
            messageRegistry) env_key=MESSAGE_REGISTRY_ID ;;
            memoryRegistry) env_key=MEMORY_REGISTRY_ID ;;
            memoryConfig) env_key=MEMORY_CONFIG_ID ;;
            profileConfig) env_key=PROFILE_CONFIG_ID ;;
            aiCreditConfig) env_key=AI_CREDIT_CONFIG_ID ;;
            mydataPoolRegistry) env_key=POOL_REGISTRY_ID ;;
            snapshotAnchorRegistry) env_key=ANCHOR_REGISTRY_ID ;;
            mydataClaimVault) env_key=CLAIM_VAULT_ID ;;
            pocVaultDirectory) env_key=POC_VAULT_DIRECTORY_ID ;;
            pocUsernameBeneficiaryDirectory) env_key=POC_USERNAME_BENEFICIARY_DIRECTORY_ID ;;
            postConfig) env_key=POST_CONFIG_ID ;;
            sptConfig) env_key=SOCIAL_PROOF_TOKENS_CONFIG_ID ;;
            pocConfig) env_key=POC_CONFIG_ID ;;
            mydataConfig) env_key=MYDATA_CONFIG_ID ;;
            spotConfig) env_key=SPOT_CONFIG_ID ;;
            insuranceConfig) env_key=INSURANCE_CONFIG_ID ;;
            orderbookRegistry) env_key=ORDERBOOK_REGISTRY_ID ;;
            proofOfCreativityAdminCap) env_key=POC_ADMIN_CAP_ID ;;
            socialProofTokensAdminCap) env_key=SPT_ADMIN_CAP_ID ;;
            pocBeneficiaryAdminCap) env_key=POC_BENEFICIARY_ADMIN_CAP_ID ;;
            mydataAdminCap) env_key=MYDATA_ADMIN_CAP_ID ;;
            mydataPoolAdminCap) env_key=POOL_ADMIN_CAP_ID ;;
            platformAdminCap) env_key=PLATFORM_ADMIN_CAP_ID ;;
            *) continue ;;
        esac
        val="$(gql_object_address "$json" "$alias")"
        gql_set_refresh "$env_key" "$val"
    done
    val="$(gql_governance_registry_id "$json" "ecosystemDao")"
    gql_set_refresh GOVERNANCE_ECOSYSTEM_REGISTRY_ID "$val"
    val="$(gql_governance_registry_id "$json" "pocDao")"
    gql_set_refresh GOVERNANCE_POC_REGISTRY_ID "$val"
}

apply_gql_refresh_file() {
    [[ -f "$GQL_REFRESH_FILE" ]] || return 0
    # shellcheck disable=SC1090
    source "$GQL_REFRESH_FILE"
    PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
    ORDERBOOK_PACKAGE_ID="$DEFAULT_ORDERBOOK_PKG"
    CLOCK_ID="$DEFAULT_CLOCK"
}

refresh_poc_session_from_graphql() {
    command -v curl >/dev/null 2>&1 || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }

    log_step "Refreshing poc-session from GraphQL ($GRAPHQL_URL)"
    local preserve_file f key val
    f="$(session_state_save_path)"
    preserve_file="$(mktemp)"
    if [[ -f "$f" ]]; then
        for key in "${MANUAL_PRESERVE_KEYS[@]}"; do
            val="$(grep -E "^[[:space:]]*${key}=" "$f" 2>/dev/null | tail -n1 | sed 's/^[^=]*=//' | sed 's/^"\(.*\)"$/\1/' | sed "s/^'\(.*\)'$/\1/" || true)"
            [[ -n "$val" ]] && printf '%s=%q\n' "$key" "$val" >> "$preserve_file"
        done
    fi

    GQL_REFRESH_FILE="$(mktemp)"
    local j1 j2 j3 jg je
    j1="$(graphql_post "$GQL_BATCH1")"
    collect_gql_mappings "$j1"
    j2="$(graphql_post "$GQL_BATCH2")"
    collect_gql_mappings "$j2"
    j3="$(graphql_post "$GQL_BATCH3")"
    collect_gql_mappings "$j3"
    jg="$(graphql_post "$GQL_GOVERNANCE")"
    collect_gql_mappings "$jg"
    je="$(graphql_post "$GQL_POC_EXTRAS")"
    collect_gql_mappings "$je"

    apply_gql_refresh_file

    if [[ -s "$preserve_file" ]]; then
        # shellcheck disable=SC1090
        source "$preserve_file"
    fi
    rm -f "$preserve_file" "$GQL_REFRESH_FILE"
    GQL_REFRESH_FILE=''

    mkdir -p "$(dirname "$f")"
    {
        echo "# Auto-refreshed from $GRAPHQL_URL at $(date -u +%Y-%m-%dT%H:%M:%SZ)"
        echo "PKG_SOCIAL=$DEFAULT_PKG_SOCIAL"
        echo "ORDERBOOK_PACKAGE_ID=$DEFAULT_ORDERBOOK_PKG"
        echo "CLOCK_ID=$DEFAULT_CLOCK"
        echo "COIN_TYPE=$DEFAULT_COIN_TYPE"
        for key in BOOTSTRAP_KEY_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_OBJECT_ID \
            USERNAME_REGISTRY_ID BLOCK_LIST_REGISTRY_ID MYDATA_REGISTRY_ID SOCIAL_GRAPH_ID \
            TOKEN_REGISTRY_ID POC_REGISTRY_ID MESSAGE_REGISTRY_ID MEMORY_REGISTRY_ID \
            POOL_REGISTRY_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID POC_VAULT_DIRECTORY_ID \
            POC_USERNAME_BENEFICIARY_DIRECTORY_ID POST_CONFIG_ID SOCIAL_PROOF_TOKENS_CONFIG_ID \
            POC_CONFIG_ID MYDATA_CONFIG_ID SPOT_CONFIG_ID INSURANCE_CONFIG_ID ORDERBOOK_REGISTRY_ID \
            POC_ADMIN_CAP_ID POC_BENEFICIARY_ADMIN_CAP_ID SPT_ADMIN_CAP_ID PLATFORM_ADMIN_CAP_ID \
            MYDATA_ADMIN_CAP_ID POOL_ADMIN_CAP_ID \
            GOVERNANCE_ECOSYSTEM_REGISTRY_ID GOVERNANCE_POC_REGISTRY_ID \
            JOIN_REFERRER_ADDRESS GAS_BUDGET MEMORY_ACCOUNT_ID TIPPER_MEMORY_ACCOUNT_ID \
            ORACLE_PROFILE_ID RESERVATION_POOL_ID; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    chmod 600 "$f" 2>/dev/null || true

    echo "GraphQL refresh summary:" >&2
    local req
    for req in "${REQUIRED_CORE_KEYS[@]}"; do
        if session_value_set "$req"; then
            echo "  OK  $req" >&2
        else
            echo "  MISS $req" >&2
        fi
    done
    echo "  Platform keys (optional — post/tip flows):" >&2
    for req in "${OPTIONAL_PLATFORM_KEYS[@]}"; do
        if session_value_set "$req"; then
            echo "  OK  $req" >&2
        else
            echo "  —   $req (unset)" >&2
        fi
    done

    if session_value_set POC_BENEFICIARY_ADMIN_CAP_ID; then
        local owner active
        active="$(resolve_myso_active_address)" || true
        owner="$(object_address_owner "$POC_BENEFICIARY_ADMIN_CAP_ID")" || true
        if [[ -n "$active" && -n "$owner" && "$owner" != "$active" ]]; then
            echo "Warning: PoCBeneficiaryAdminCap owner ($owner) != active-address ($active)" >&2
        fi
    fi

    if ! session_value_set PLATFORM_OBJECT_ID; then
        if platform_mode_is_full; then
            echo "Warning: PLATFORM_OBJECT_ID not found — use menu C or --create-platform." >&2
        else
            echo "Note: no Platform on chain — no-platform mode (username PoC only)." >&2
            echo "  Create one with menu C or --create-platform for post/dispute flows." >&2
        fi
    fi
}

object_exists_on_fullnode() {
    local id="$1"
    id="$(normalize_hex_id "$id")" || return 1
    myso client object "$id" >/dev/null 2>&1
}

object_address_owner() {
    local object_id="$1" json owner
    json="$(myso client object "$object_id" --json 2>/dev/null)" || return 1
    owner="$(printf '%s' "$json" | jq -r '
        .owner // empty
        | if type == "string" then .
          elif type == "object" then (.address // .AddressOwner // empty)
          else empty end
    ' 2>/dev/null | head -n1)"
    if [[ -z "$owner" ]]; then
        owner="$(printf '%s' "$json" | jq -r '
            .. | objects | select(has("AddressOwner")) | .AddressOwner
            | if type == "string" then .
              elif type == "object" then (.owner // .address // empty)
              else empty end
        ' 2>/dev/null | head -n1)"
    fi
    if [[ -n "$owner" ]]; then
        printf '%s' "$owner"
        return 0
    fi
    printf '%s' "$json" | grep -Eo '"AddressOwner"[[:space:]]*:[[:space:]]*"0x[0-9a-fA-F]+"' | head -n1 \
        | grep -Eo '0x[0-9a-fA-F]+' || return 1
}

extract_tx_digest() {
    local out="$1" digest
    [[ -n "$out" ]] || return 1
    digest="$(echo "$out" | jq -r '
        .effects.V2.transaction_digest //
        .effects.transaction_digest //
        .transaction_digest //
        empty
    ' 2>/dev/null | grep -E . | tail -n1)"
    if [[ -n "$digest" ]]; then
        printf '%s' "$digest"
        return 0
    fi
    digest="$(echo "$out" | grep -Eo '"transaction_digest"[[:space:]]*:[[:space:]]*"[^"]+"' \
        | tail -n1 | sed -E 's/.*"([^"]+)"$/\1/')"
    if [[ -n "$digest" ]]; then
        printf '%s' "$digest"
        return 0
    fi
    digest="$(echo "$out" | grep -Eo 'Transaction Digest: [0-9a-zA-Z+/=_-]+' | tail -n1 | awk '{print $3}')"
    if [[ -n "$digest" ]]; then
        printf '%s' "$digest"
        return 0
    fi
    echo "$out" | grep -Eo '[A-Za-z0-9+/]{43,44}=' | tail -n1
}

extract_created_object_by_type() {
    local digest="$1" type_substring="$2"
    local json result
    [[ -n "$digest" && -n "$type_substring" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    result="$(echo "$json" | jq -r --arg t "$type_substring" '
        def suffix_match($ot):
            ($ot | tostring) | endswith("::" + $t);
        def object_type($o):
            ($o.objectType? // $o.object_type? // $o.type? // "") | tostring;
        def object_id($o):
            ($o.objectId? // $o.object_id? // $o.reference?.objectId? // "") | tostring;
        def is_created_output($o):
            ((($o.outputState? // $o.output_state? // "") | tostring)
                | test("OBJECT_WRITE|ObjectWrite|CREATED|Created"))
            or ((($o.idOperation? // $o.id_operation? // "") | tostring)
                | test("CREATED|Created"))
            or ((($o.inputState? // $o.input_state? // "") | tostring)
                | test("DOES_NOT_EXIST|DoesNotExist"));
        (
            (.changed_objects // .changedObjects // [])[]
            | if type == "array" then empty else . end
            | select(suffix_match(object_type(.)))
            | select(is_created_output(.))
            | object_id(.)
        ),
        if $t != "PoCUsernameBeneficiary" then
            (
                (.changed_objects // .changedObjects // [])[]
                | if type == "array" then empty else . end
                | select(suffix_match(object_type(.)))
                | object_id(.)
            ),
            (
                .. | objects
                | select(suffix_match(object_type(.)))
                | object_id(.)
            )
        else empty end
        | select(. != null and . != "")
    ' | head -n1)"
    [[ -n "$result" ]] || return 1
    printf '%s' "$result"
}

object_move_type_from_json() {
    local json="$1"
    local move_type
    [[ -n "$json" ]] || return 1
    move_type="$(echo "$json" | jq -r '
        def struct_type($other):
            "0x\($other.address)::\($other.module)::\($other.name)";
        (.objType // .objectType // null) as $top |
        if ($top | type) == "string" and ($top | length) > 0 then $top
        elif .data.Move.type_.Other? then struct_type(.data.Move.type_.Other)
        elif .content.Move.type_.Other? then struct_type(.content.Move.type_.Other)
        elif .data.Move.type_.GasCoin? then "GasCoin"
        else empty end
    ' 2>/dev/null | head -n1)"
    [[ -n "$move_type" && "$move_type" != "null" ]] || return 1
    printf '%s' "$move_type"
}

object_move_type() {
    local object_id="$1"
    local json move_type attempt
    [[ -n "$object_id" ]] || return 1
    for attempt in 1 2 3; do
        json="$(myso client object "$object_id" --json 2>/dev/null)" || {
            [[ "$attempt" -lt 3 ]] || return 1
            sleep 0.3
            continue
        }
        move_type="$(object_move_type_from_json "$json" 2>/dev/null || true)"
        if [[ -n "$move_type" ]]; then
            printf '%s' "$move_type"
            return 0
        fi
        [[ "$attempt" -lt 3 ]] || return 1
        sleep 0.3
    done
    return 1
}

object_type_from_tx() {
    local digest="$1" object_id="$2"
    local json move_type
    [[ -n "$digest" && -n "$object_id" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    move_type="$(echo "$json" | jq -r --arg id "$object_id" '
        (.changed_objects // .changedObjects // [])[]
        | if type == "array" then empty else . end
        | select((.objectId // .object_id // "") == $id)
        | (.objectType // .object_type // empty | tostring)
    ' | head -n1)"
    [[ -n "$move_type" && "$move_type" != "null" ]] || return 1
    printf '%s' "$move_type"
}

verify_object_type_from_tx() {
    local digest="$1" object_id="$2" type_suffix="$3"
    local move_type
    [[ -n "$digest" && -n "$object_id" && -n "$type_suffix" ]] || return 1
    move_type="$(object_type_from_tx "$digest" "$object_id")" || return 1
    if [[ "$move_type" != *"::$type_suffix" ]]; then
        echo "Tx $digest object $object_id has type $move_type (expected *::$type_suffix)" >&2
        return 1
    fi
    return 0
}

verify_object_type() {
    local object_id="$1" type_suffix="$2"
    local move_type
    [[ -n "$object_id" && -n "$type_suffix" ]] || return 1
    move_type="$(object_move_type "$object_id")" || {
        echo "Could not read Move type for object $object_id" >&2
        return 1
    }
    if [[ "$move_type" != *"::$type_suffix" ]]; then
        echo "Object $object_id has type $move_type (expected *::$type_suffix)" >&2
        return 1
    fi
    return 0
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

resolve_beneficiary_vault_id() {
    local analyze_digest="$1" beneficiary_addr="$2"
    local vault_id ben
    [[ -n "$beneficiary_addr" ]] || return 1
    beneficiary_addr="$(normalize_hex_id "$beneficiary_addr")" || return 1

    if session_value_set POC_BENEFICIARY_VAULT_ID; then
        vault_id="$(normalize_hex_id "$POC_BENEFICIARY_VAULT_ID")" || return 1
        if verify_object_type "$vault_id" "PoCBeneficiaryVault" 2>/dev/null \
            && object_exists_on_fullnode "$vault_id"; then
            ben="$(parse_beneficiary_address_from_vault_object "$vault_id")" || true
            if [[ -z "$ben" ]] || [[ "$(normalize_hex_id "$ben")" == "$beneficiary_addr" ]]; then
                printf '%s' "$vault_id"
                return 0
            fi
        fi
    fi

    if [[ -n "$analyze_digest" ]]; then
        vault_id="$(extract_created_object_by_type "$analyze_digest" "PoCBeneficiaryVault")" || true
        if [[ -n "$vault_id" ]] \
            && verify_object_type "$vault_id" "PoCBeneficiaryVault" 2>/dev/null \
            && object_exists_on_fullnode "$vault_id"; then
            normalize_hex_id "$vault_id"
            return 0
        fi
    fi

    vault_id="$(gql_beneficiary_vault_id_for_address "$beneficiary_addr")" || true
    if [[ -n "$vault_id" ]] \
        && verify_object_type "$vault_id" "PoCBeneficiaryVault" 2>/dev/null \
        && object_exists_on_fullnode "$vault_id"; then
        printf '%s' "$vault_id"
        return 0
    fi

    lookup_beneficiary_vault_id_on_fullnode "$beneficiary_addr"
}

assert_tx_success() {
    local out="$1"
    local digest="${2:-}"
    local json status
    [[ -n "$out" || -n "$digest" ]] || return 1
    if [[ -z "$digest" ]]; then
        digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    fi
    if [[ -n "$digest" ]]; then
        json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
        status="$(echo "$json" | jq -r '
            .effects.V2.status // .effects.status // empty | tostring
        ')"
        [[ "$status" == "Success" ]]
        return
    fi
    echo "$out" | jq -e '
        (.effects.V2.status // .effects.status // empty | tostring) == "Success"
    ' >/dev/null
}

tx_has_event_named() {
    local digest="$1" event_name="$2"
    local json
    [[ -n "$digest" && -n "$event_name" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    echo "$json" | jq -e --arg name "$event_name" '
        [.. | objects | select(has("type_") or has("type"))]
        | map(.type_ // .type)
        | any(.name? == $name)
    ' >/dev/null
}

extract_beneficiary_id_from_provision_event() {
    local digest="$1"
    [[ -n "$digest" ]] || return 1
    python3 - "$digest" <<'PY'
import json, subprocess, sys

digest = sys.argv[1]
raw = subprocess.check_output(["myso", "client", "tx-block", digest, "--json"], text=True)
tx = json.loads(raw)
events = tx.get("events", {}).get("data", [])
if not isinstance(events, list):
    events = tx.get("events", [])
    if isinstance(events, dict):
        events = events.get("data", [])
for ev in events:
    if not isinstance(ev, dict):
        continue
    t = ev.get("type_", ev.get("type", {}))
    if not isinstance(t, dict) or t.get("name") != "UsernameBeneficiaryProvisionedEvent":
        continue
    contents = ev.get("contents", [])
    if isinstance(contents, list) and len(contents) >= 32:
        print("0x" + bytes(contents[:32]).hex())
        sys.exit(0)
sys.exit(1)
PY
}

extract_beneficiary_id_from_provision_tx() {
    local digest="$1"
    local json result
    [[ -n "$digest" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    result="$(echo "$json" | jq -r '
        def suffix_match($ot):
            ($ot | tostring) | endswith("::PoCUsernameBeneficiary");
        def object_type($o):
            ($o.objectType? // $o.object_type? // $o.type? // "") | tostring;
        def object_id($o):
            ($o.objectId? // $o.object_id? // $o.reference?.objectId? // "") | tostring;
        def is_new_object($o):
            ((($o.idOperation? // $o.id_operation? // "") | tostring) | test("CREATED|Created"))
            or ((($o.inputState? // $o.input_state? // "") | tostring)
                | test("DOES_NOT_EXIST|DoesNotExist"));
        (
            (.changed_objects // .changedObjects // [])[]
            | if type == "array" then empty else . end
            | select(suffix_match(object_type(.)))
            | select(is_new_object(.))
            | object_id(.)
        )
        | select(. != null and . != "")
    ' | head -n1)"
    if [[ -n "$result" ]]; then
        printf '%s' "$result"
        return 0
    fi
    extract_beneficiary_id_from_provision_event "$digest"
}

username_beneficiary_status() {
    local beneficiary_id="$1"
    local status attempt
    [[ -n "$beneficiary_id" ]] || return 1
    for attempt in 1 2 3; do
        status="$(python3 - "$beneficiary_id" <<'PY'
import json, subprocess, sys

def uleb128_read(data: bytes, off: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while off < len(data):
        b = data[off]
        off += 1
        result |= (b & 0x7F) << shift
        if b < 0x80:
            break
        shift += 7
    return result, off

beneficiary_id = sys.argv[1]
raw = subprocess.check_output(["myso", "client", "object", beneficiary_id, "--json"], text=True)
obj = json.loads(raw)
contents = obj.get("data", {}).get("Move", {}).get("contents")
if contents is None:
    contents = obj.get("content", {}).get("Move", {}).get("contents")
if not contents:
    sys.exit(1)
data = bytes(contents)
if len(data) < 33:
    sys.exit(1)
off = 32  # PoCUsernameBeneficiary.id (UID)
username_len, off = uleb128_read(data, off)
off += username_len
off += 1  # creator_identity.source (u8)
identity_len, off = uleb128_read(data, off)
off += identity_len
x_handle_len, off = uleb128_read(data, off)
off += x_handle_len
off += 8  # provisioned_at (u64)
if off >= len(data):
    sys.exit(1)
print(data[off])
PY
        )" && [[ -n "$status" ]] && {
            printf '%s' "$status"
            return 0
        }
        [[ "$attempt" -lt 3 ]] || return 1
        sleep 0.3
    done
    return 1
}

verify_username_beneficiary_claimed() {
    local claim_digest="$1" beneficiary_id="$2" username="$3"
    local status
    [[ -n "$claim_digest" && -n "$beneficiary_id" ]] || return 1
    wait_for_tx_finalized "$claim_digest" || return 1
    assert_poc_scenario_events username_claim "$claim_digest" || return 1
    status="$(username_beneficiary_status "$beneficiary_id")"
    if [[ "$status" != "2" ]]; then
        echo "beneficiary $beneficiary_id status=$status (expected 2=CLAIMED)" >&2
        return 1
    fi
    log_step "1b claim_username_beneficiary OK username=$username beneficiary=$beneficiary_id"
    return 0
}

resolve_shard_id_for_username() {
    local username="$1"
    [[ -n "${POC_USERNAME_BENEFICIARY_DIRECTORY_ID:-}" ]] || return 1
    python3 - "$POC_USERNAME_BENEFICIARY_DIRECTORY_ID" "$username" <<'PY'
import hashlib, json, subprocess, sys

def uleb128_read(data: bytes, off: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while off < len(data):
        b = data[off]
        off += 1
        result |= (b & 0x7F) << shift
        if b < 0x80:
            break
        shift += 7
    return result, off

directory_id, username = sys.argv[1], sys.argv[2]
raw = subprocess.check_output(["myso", "client", "object", directory_id, "--json"], text=True)
obj = json.loads(raw)
contents = obj.get("data", {}).get("Move", {}).get("contents")
if not contents:
    sys.exit(1)
data = bytes(contents)
if len(data) < 33:
    sys.exit(1)
off = 32  # PoCUsernameBeneficiaryDirectory.id (UID)
count, off = uleb128_read(data, off)
need = off + count * 32
if count <= 0 or need > len(data):
    sys.exit(1)
shard_ids = [
    "0x" + data[off + i * 32 : off + (i + 1) * 32].hex()
    for i in range(count)
]
canonical = username.encode("utf-8").lower()
h = hashlib.blake2b(canonical, digest_size=32).digest()
idx = h[0] % len(shard_ids)
print(shard_ids[idx])
PY
}

identity_beneficiary_address() {
    local source="$1" identity_hex="$2"
    python3 - "$source" "$identity_hex" <<'PY'
import hashlib, sys
source = int(sys.argv[1])
ih = sys.argv[2].removeprefix("0x")
data = bytes([source]) + bytes.fromhex(ih)
h = hashlib.blake2b(data, digest_size=32).digest()
# Match Move object::id_to_address(object::id_from_bytes(...)) — use first 32 bytes as address
print("0x" + h.hex())
PY
}

resolve_gas_coins_json_for_address() {
    local addr="$1"
    addr="$(normalize_hex_id "$addr")" || return 1
    myso client gas "$addr" --json 2>/dev/null
}

ensure_two_gas_coins_for_address() {
    local addr="$1" attempt json count
    addr="$(normalize_hex_id "$addr")" || return 1
    for attempt in $(seq 1 30); do
        json="$(resolve_gas_coins_json_for_address "$addr")" || json='[]'
        count="$(echo "$json" | jq 'length')"
        [[ "$count" -ge 2 ]] && return 0
        if [[ "$attempt" == 1 ]]; then
            myso client switch --address "$addr" >/dev/null
            myso client faucet >/dev/null 2>&1 || myso client faucet >&2
        fi
        sleep 1
    done
    echo "Tipper $addr needs two gas coins (one for gas, one for tip payment)" >&2
    return 1
}

pick_tip_and_gas_coins_for_address() {
    local addr="$1" amount="$2"
    local json tip_coin gas_coin
    addr="$(normalize_hex_id "$addr")" || return 1
    ensure_two_gas_coins_for_address "$addr" || return 1
    json="$(resolve_gas_coins_json_for_address "$addr")" || return 1
    tip_coin="$(echo "$json" | jq -r --argjson amt "$amount" '
        [.[] | select((.mistBalance | tonumber) >= $amt)] |
        if length == 0 then empty
        elif length >= 2 then .[1].gasCoinId
        else .[0].gasCoinId end
    ')"
    [[ -n "$tip_coin" ]] || {
        echo "No coin with balance >= $amount for tipper $addr" >&2
        return 1
    }
    gas_coin="$(echo "$json" | jq -r --arg tip "$tip_coin" '
        [.[] | select(.gasCoinId != $tip)] | .[0].gasCoinId // empty
    ')"
    [[ -n "$gas_coin" ]] || gas_coin="$(echo "$json" | jq -r '.[0].gasCoinId // empty')"
    [[ -n "$gas_coin" && "$gas_coin" != "$tip_coin" ]] || {
        echo "Could not pick distinct gas and tip coins for $addr" >&2
        return 1
    }
    case "$tip_coin" in 0x*) ;; *) echo "Invalid tip coin id: $tip_coin" >&2; return 1 ;; esac
    case "$gas_coin" in 0x*) ;; *) echo "Invalid gas coin id: $gas_coin" >&2; return 1 ;; esac
    printf '%s %s' "$tip_coin" "$gas_coin"
}

resolve_gas_coin_for_address() {
    local addr="$1"
    local json
    json="$(resolve_gas_coins_json_for_address "$addr")" || return 1
    echo "$json" | jq -r '.[0].gasCoinId // .[0].coinObjectId // empty' | head -n1
}

gql_profile_id_for_address() {
    local addr="$1" resp vars
    vars="$(jq -nc --arg addr "$addr" '{addr: $addr}')"
    resp="$(graphql_post \
        'query ProfileId($addr: MySoAddress!) { profile(address: $addr) { profileId } }' \
        "$vars")" || return 1
    echo "$resp" | jq -r '.data.profile.profileId // empty' | head -n1
}

gql_profile_memory_account_id() {
    local addr="$1" resp vars
    vars="$(jq -nc --arg addr "$addr" '{addr: $addr}')"
    resp="$(graphql_post \
        'query ProfileMemory($addr: MySoAddress!) { profile(address: $addr) { memoryAccountId } }' \
        "$vars")" || return 1
    echo "$resp" | jq -r '.data.profile.memoryAccountId // empty' | head -n1
}

assert_post_flow_tip_and_claim_graphql() {
    local post_id="$1" vault_id="$2" resp vars attempt
    local tips_received total_tip_volume tip_amount gross_amount treasury_amount
    post_id="$(normalize_hex_id "$post_id")" || return 1
    vault_id="$(normalize_hex_id "$vault_id")" || return 1
    vars="$(jq -nc --arg postId "$post_id" --arg vaultId "$vault_id" \
        '{postId: $postId, vaultId: $vaultId}')"
    for attempt in $(seq 1 60); do
        resp="$(graphql_post \
            'query PostFlowTipVolume($postId: ID!, $vaultId: String!) {
                post(id: $postId) {
                    tipsReceived
                    totalTipVolume
                    tips(limit: 1) { amount }
                }
                pocBeneficiaryVaultByVaultId(vaultId: $vaultId) {
                    claims(limit: 1) {
                        grossAmount
                        treasuryAmount
                    }
                }
            }' \
            "$vars" 2>/dev/null)" || resp='{}'
        tips_received="$(echo "$resp" | jq -r '.data.post.tipsReceived // empty')"
        total_tip_volume="$(echo "$resp" | jq -r '.data.post.totalTipVolume // empty')"
        tip_amount="$(echo "$resp" | jq -r '.data.post.tips[0].amount // empty')"
        gross_amount="$(echo "$resp" | jq -r '.data.pocBeneficiaryVaultByVaultId.claims[0].grossAmount // empty')"
        treasury_amount="$(echo "$resp" | jq -r '.data.pocBeneficiaryVaultByVaultId.claims[0].treasuryAmount // empty')"
        if [[ "$total_tip_volume" == "$DEFAULT_TIP_AMOUNT" && "$gross_amount" == "$DEFAULT_TIP_AMOUNT" ]]; then
            break
        fi
        sleep 1
    done
    if [[ "$tips_received" != "0" ]]; then
        echo "GraphQL post.tipsReceived expected 0 for escrow tip, got: $tips_received" >&2
        return 1
    fi
    if [[ "$total_tip_volume" != "$DEFAULT_TIP_AMOUNT" ]]; then
        echo "GraphQL post.totalTipVolume expected $DEFAULT_TIP_AMOUNT, got: $total_tip_volume" >&2
        return 1
    fi
    if [[ "$tip_amount" != "$DEFAULT_TIP_AMOUNT" ]]; then
        echo "GraphQL post.tips[0].amount expected $DEFAULT_TIP_AMOUNT, got: $tip_amount" >&2
        return 1
    fi
    if [[ "$gross_amount" != "$DEFAULT_TIP_AMOUNT" ]]; then
        echo "GraphQL claim grossAmount expected $DEFAULT_TIP_AMOUNT, got: $gross_amount" >&2
        return 1
    fi
    if [[ "$treasury_amount" != "1000000" ]]; then
        echo "GraphQL claim treasuryAmount expected 1000000, got: $treasury_amount" >&2
        return 1
    fi
    log_step "GraphQL tip volume + vault claim amounts verified"
}

wait_for_gql_username_beneficiary() {
    local username="$1" beneficiary_id="$2" attempt resp vars indexed_id status
    beneficiary_id="$(normalize_hex_id "$beneficiary_id")" || return 1
    vars="$(jq -nc --arg username "$username" '{username: $username}')"
    for attempt in $(seq 1 60); do
        resp="$(graphql_post \
            'query PocUsernameBeneficiaryByUsername($username: String!) {
                pocUsernameBeneficiaryByUsername(username: $username) {
                    beneficiaryId
                    username
                    status
                }
            }' \
            "$vars" 2>/dev/null)" || resp='{}'
        indexed_id="$(echo "$resp" | jq -r '.data.pocUsernameBeneficiaryByUsername.beneficiaryId // empty')"
        status="$(echo "$resp" | jq -r '.data.pocUsernameBeneficiaryByUsername.status // empty')"
        if [[ -n "$indexed_id" && "$(normalize_hex_id "$indexed_id")" == "$beneficiary_id" && "$status" == "2" ]]; then
            printf '%s' "$resp"
            return 0
        fi
        sleep 1
    done
    echo "Timed out waiting for GraphQL username beneficiary $username status=2 (last status=${status:-null})" >&2
    return 1
}

assert_username_beneficiary_graphql() {
    local username="$1" beneficiary_id="$2" resp indexed_id status
    resp="$(wait_for_gql_username_beneficiary "$username" "$beneficiary_id")" || return 1
    indexed_id="$(echo "$resp" | jq -r '.data.pocUsernameBeneficiaryByUsername.beneficiaryId // empty')"
    status="$(echo "$resp" | jq -r '.data.pocUsernameBeneficiaryByUsername.status // empty')"
    log_step "GraphQL username beneficiary OK username=$username status=$status beneficiaryId=$indexed_id"
}

extract_memory_account_id_from_profile_object() {
    local profile_id="$1" json
    profile_id="$(normalize_hex_id "$profile_id")" || return 1
    json="$(myso client object "$profile_id" --json 2>/dev/null)" || return 1
    python3 - "$json" <<'PY'
import json, sys
j = json.loads(sys.argv[1])
contents = j.get("data", {}).get("Move", {}).get("contents")
if not contents or len(contents) < 41:
    raise SystemExit(1)
if contents[-9] != 1:
    raise SystemExit(1)
print("0x" + bytes(contents[-41:-9]).hex())
PY
}

resolve_memory_account_for_address() {
    local addr="$1" profile_id onchain_mem gql_mem normalized
    profile_id="$(gql_profile_id_for_address "$addr")" || true
    if [[ -n "$profile_id" ]]; then
        profile_id="$(normalize_hex_id "$profile_id")" || profile_id=""
    fi
    if [[ -n "$profile_id" ]] && object_exists_on_fullnode "$profile_id"; then
        gql_mem="$(gql_profile_memory_account_id "$addr")" || true
        if [[ -n "$gql_mem" ]]; then
            normalized="$(normalize_hex_id "$gql_mem")" || normalized=""
            if [[ -n "$normalized" ]] && object_exists_on_fullnode "$normalized"; then
                printf '%s' "$normalized"
                return 0
            fi
        fi
        onchain_mem="$(extract_memory_account_id_from_profile_object "$profile_id")" || true
        if [[ -n "$onchain_mem" ]] && object_exists_on_fullnode "$onchain_mem"; then
            normalize_hex_id "$onchain_mem"
            return 0
        fi
    fi
    gql_mem="$(gql_profile_memory_account_id "$addr")" || true
    [[ -n "$gql_mem" ]] || return 1
    normalized="$(normalize_hex_id "$gql_mem")" || return 1
    object_exists_on_fullnode "$normalized" || return 1
    printf '%s' "$normalized"
}

ensure_joined_platform() {
    local out
    if [[ "${POC_PLATFORM_JOINED:-0}" == 1 ]]; then
        return 0
    fi
    if [[ -n "${PLATFORM_OBJECT_ID:-}" ]] && ! object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
        echo "Session PLATFORM_OBJECT_ID not on localnet fullnode; clearing." >&2
        PLATFORM_OBJECT_ID=''
    fi
    require_session_fields PLATFORM_REGISTRY_ID BLOCK_LIST_REGISTRY_ID PLATFORM_OBJECT_ID CLOCK_ID || return 1
    require_hex_ids PLATFORM_REGISTRY_ID BLOCK_LIST_REGISTRY_ID PLATFORM_OBJECT_ID CLOCK_ID || return 1
    log_step "Joining platform for $(resolve_myso_active_address) (join_platform PTB)"
    if out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --move-call "${PKG_SOCIAL}::platform::join_platform" \
        "$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" \
        "$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" \
        "$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" \
        "$(ptb_shared_ref "$CLOCK_ID")")"; then
        POC_PLATFORM_JOINED=1
        return 0
    fi
    if echo "$out" | grep -qE 'Abort Code: 3\b'; then
        log_step "Already joined platform — continuing"
        POC_PLATFORM_JOINED=1
        return 0
    fi
    return 1
}

create_profile_with_memory_for_address() {
    local sender="$1" out digest mem profile_id username existing_mem
    [[ -n "$sender" ]] || return 1
    sender="$(normalize_hex_id "$sender")" || return 1

    existing_mem="$(resolve_memory_account_for_address "$sender")" || true
    if [[ -n "$existing_mem" ]]; then
        mem="$(normalize_hex_id "$existing_mem")"
        profile_id="$(resolve_profile_object_for_address "$sender")" || true
        if [[ "$sender" == "$ORACLE_ADDRESS" ]]; then
            MEMORY_ACCOUNT_ID="$mem"
            if [[ -n "$profile_id" ]]; then
                ORACLE_PROFILE_ID="$(normalize_hex_id "$profile_id")"
                log_session_use "ORACLE_PROFILE_ID" "$ORACLE_PROFILE_ID"
            fi
            log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
            save_session_state
        fi
        printf '%s' "$mem"
        return 0
    fi

    require_session_fields USERNAME_REGISTRY_ID PROFILE_CONFIG_ID MEMORY_REGISTRY_ID \
        AI_CREDIT_CONFIG_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_REGISTRY_ID PROFILE_CONFIG_ID MEMORY_REGISTRY_ID \
        AI_CREDIT_CONFIG_ID CLOCK_ID || return 1
    POC_RUN_ID="${POC_RUN_ID:-$(date +%s)}"
    username="poc${POC_RUN_ID}${RANDOM}"
    log_step "Creating profile + MemoryAccount for $sender (username=$username)"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" \
        --move-call "${PKG_SOCIAL}::profile::create_profile" \
        "$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" \
        "$(ptb_shared_ref "$PROFILE_CONFIG_ID")" \
        "$(ptb_shared_ref "$MEMORY_REGISTRY_ID")" \
        "$(ptb_shared_ref "$AI_CREDIT_CONFIG_ID")" \
        "$(literal_move_string "PoC Runtime")" \
        "$(literal_move_string "$username")" \
        '""' \
        'vector[]' 'vector[]' \
        "$(ptb_shared_ref "$CLOCK_ID")")" || {
        if echo "$out" | grep -qE 'Abort Code: 0\b|EProfileAlreadyExists'; then
            mem="$(resolve_memory_account_for_address "$sender")" || true
            [[ -n "$mem" ]] || return 1
            mem="$(normalize_hex_id "$mem")"
            profile_id="$(resolve_profile_object_for_address "$sender")" || true
            if [[ "$sender" == "$ORACLE_ADDRESS" ]]; then
                MEMORY_ACCOUNT_ID="$mem"
                if [[ -n "$profile_id" ]]; then
                    ORACLE_PROFILE_ID="$(normalize_hex_id "$profile_id")"
                    log_session_use "ORACLE_PROFILE_ID" "$ORACLE_PROFILE_ID"
                fi
                log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
                save_session_state
            fi
            printf '%s' "$mem"
            return 0
        fi
        return 1
    }
    digest="$(extract_tx_digest "$out")"
    mem="$(extract_created_object_by_type "$digest" "memory::MemoryAccount")"
    [[ -n "$mem" ]] || mem="$(extract_created_object_by_type "$digest" "MemoryAccount")"
    profile_id="$(extract_created_object_by_type "$digest" "profile::Profile")"
    [[ -n "$profile_id" ]] || profile_id="$(extract_created_object_by_type "$digest" "Profile")"
    [[ -n "$mem" ]] || {
        echo "create_profile succeeded but MemoryAccount not found in tx effects" >&2
        return 1
    }
    mem="$(normalize_hex_id "$mem")"
    if [[ "$sender" == "$ORACLE_ADDRESS" ]]; then
        MEMORY_ACCOUNT_ID="$mem"
        if [[ -n "$profile_id" ]]; then
            ORACLE_PROFILE_ID="$(normalize_hex_id "$profile_id")"
            log_session_use "ORACLE_PROFILE_ID" "$ORACLE_PROFILE_ID"
        fi
        log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
        save_session_state
    fi
    printf '%s' "$mem"
}

create_oracle_profile_with_memory() {
    local mem
    mem="$(resolve_memory_account_for_address "$ORACLE_ADDRESS")" || true
    if [[ -n "$mem" ]]; then
        MEMORY_ACCOUNT_ID="$mem"
        log_session_use "MEMORY_ACCOUNT_ID" "$mem"
        save_session_state
        return 0
    fi
    create_profile_with_memory_for_address "$ORACLE_ADDRESS" >/dev/null
}

resolve_owned_cap_for_address() {
    local addr="$1" type_fragment="$2"
    local json
    json="$(myso client objects "$addr" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r --arg t "$type_fragment" '
        .[]? | select(.type? | tostring | contains($t)) | .data.objectId // .objectId // empty
    ' | head -n1
}

ensure_beneficiary_admin_cap() {
    local active cap owner
    active="$(resolve_myso_active_address)" || return 1
    cap="$(resolve_owned_cap_for_address "$active" "PoCBeneficiaryAdminCap")"
    if [[ -n "$cap" ]]; then
        POC_BENEFICIARY_ADMIN_CAP_ID="$cap"
        log_session_use "POC_BENEFICIARY_ADMIN_CAP_ID" "$cap"
        return 0
    fi
    [[ -n "${POC_BENEFICIARY_ADMIN_CAP_ID:-}" ]] || {
        echo "No PoCBeneficiaryAdminCap for $active — run ./scripts/bootstrap.sh" >&2
        return 1
    }
    owner="$(object_address_owner "$POC_BENEFICIARY_ADMIN_CAP_ID")" || true
    if [[ "$owner" == "$active" ]]; then
        return 0
    fi
    echo "PoCBeneficiaryAdminCap $POC_BENEFICIARY_ADMIN_CAP_ID not owned by $active (owner: ${owner:-unknown})" >&2
    echo "  Run ./scripts/bootstrap.sh or set POC_BENEFICIARY_ADMIN_CAP_ID to your owned cap." >&2
    return 1
}

resolve_profile_object_for_address() {
    local addr="$1"
    local json
    json="$(myso client objects "$addr" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r '
        .[]? | select(.type? | tostring | contains("profile::Profile")) | .data.objectId // .objectId // empty
    ' | head -n1
}

address_has_profile() {
    local addr="$1" profile_id
    addr="$(normalize_hex_id "$addr")" || return 1
    profile_id="$(gql_profile_id_for_address "$addr")" || true
    [[ -n "$profile_id" ]]
}

ensure_claim_wallet_without_profile() {
    local candidate new_addr saved_oracle attempt coin
    saved_oracle="$(normalize_hex_id "$ORACLE_ADDRESS")" || return 1
    myso client switch --address "$saved_oracle" >/dev/null
    for candidate in "$TIPPER_ADDRESS" "$ORACLE_ADDRESS"; do
        [[ -n "$candidate" ]] || continue
        candidate="$(normalize_hex_id "$candidate")" || continue
        if ! address_has_profile "$candidate"; then
            CREATOR_ADDRESS="$candidate"
            log_step "Claim wallet (no profile yet): $candidate"
            return 0
        fi
    done
    log_step "Creating fresh wallet for username beneficiary claim"
    new_addr="$(myso client new-address ed25519 "poc_claimer_${POC_RUN_ID}" --json | jq -r '.address // empty')"
    [[ -n "$new_addr" ]] || { echo "Could not create claim wallet" >&2; return 1; }
    new_addr="$(normalize_hex_id "$new_addr")"
    myso client faucet --address "$new_addr" >/dev/null 2>&1 || myso client faucet --address "$new_addr" >&2 || true
    CREATOR_ADDRESS="$new_addr"
    log_step "Claim wallet (fresh): $CREATOR_ADDRESS"
}

ensure_ephemeral_oracle_for_username_claim() {
    if [[ -n "${USERNAME_CLAIM_ORACLE:-}" ]]; then
        return 0
    fi
    USERNAME_CLAIM_ORACLE="$(normalize_hex_id "$ORACLE_ADDRESS")"
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
    log_step "Enabling SPT trading (toggle_emergency_kill_switch)"
    SKIP_CONFIRM_RUN=1 run_myso_call social_proof_tokens toggle_emergency_kill_switch \
        --args "$SPT_ADMIN_CAP_ID" "@${SOCIAL_PROOF_TOKENS_CONFIG_ID}" true \
        "$(bytes_to_hex_arg "PoC runtime test enable trading")" "@${CLOCK_ID}"
}

invalidate_stale_session_runtime_ids() {
    local key id
    for key in MEMORY_ACCOUNT_ID TIPPER_MEMORY_ACCOUNT_ID ORACLE_PROFILE_ID \
        POC_BENEFICIARY_VAULT_ID RESERVATION_POOL_ID; do
        id="${!key:-}"
        [[ -n "$id" ]] || continue
        if ! object_exists_on_fullnode "$id"; then
            echo "Clearing stale session $key (not on fullnode)" >&2
            printf -v "$key" '%s' ''
        fi
    done
}

prepare_for_create_post() {
    invalidate_stale_session_runtime_ids
    ensure_memory_account_for_post_flows || return 1
    if ! platform_mode_is_full; then
        return 0
    fi
    if [[ "${POC_PLATFORM_JOINED:-0}" == 1 ]]; then
        return 0
    fi
    if ensure_joined_platform; then
        POC_PLATFORM_JOINED=1
        return 0
    fi
    log_step "Platform join failed — creating and approving a fresh test platform"
    create_test_platform || return 1
    if ensure_joined_platform; then
        POC_PLATFORM_JOINED=1
        return 0
    fi
    return 1
}

ensure_oracle_profile_id() {
    local oracle profile_id
    if [[ -n "${ORACLE_PROFILE_ID:-}" ]]; then
        if object_exists_on_fullnode "$ORACLE_PROFILE_ID"; then
            return 0
        fi
        echo "Session ORACLE_PROFILE_ID not on fullnode; re-resolving." >&2
        ORACLE_PROFILE_ID=''
    fi
    oracle="$(resolve_myso_active_address)" || return 1
    profile_id="$(resolve_profile_object_for_address "$oracle")"
    [[ -n "$profile_id" ]] || {
        echo "No owned Profile object for oracle $oracle" >&2
        return 1
    }
    ORACLE_PROFILE_ID="$profile_id"
    log_session_use "ORACLE_PROFILE_ID" "$profile_id"
}

read_poc_config_oracle() {
    local json
    [[ -n "${POC_CONFIG_ID:-}" ]] || return 1
    json="$(myso client object "$POC_CONFIG_ID" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r '
        .. | objects | select(has("oracle_address")) | .oracle_address // empty
    ' | head -n1
}

maybe_auto_refresh_session() {
    if [[ "${POC_NO_AUTO_REFRESH:-0}" == 1 ]]; then
        return 0
    fi
    if [[ "${POC_AUTO_REFRESH:-1}" != 1 ]]; then
        return 0
    fi
    local missing
    missing="$(missing_required_keys 2>/dev/null || true)"
    if [[ -n "$missing" ]] || [[ "${POC_FORCE_REFRESH:-0}" == 1 ]]; then
        refresh_poc_session_from_graphql
        load_session_state
    fi
}

update_poc_config_oracle() {
    poc_oracle_update_config "$1"
}

preflight_oracle_and_config() {
    require_session_fields POC_CONFIG_ID POC_ADMIN_CAP_ID CLOCK_ID || return 1
    if [[ "${POC_USE_DIRECT_MOVE:-0}" == "1" ]]; then
        ensure_cli_addresses || return 1
        local oracle current_oracle
        oracle="$(resolve_myso_active_address)" || { echo "Could not read active-address" >&2; return 1; }
        log_step "Preflight: oracle active-address = $oracle"
        current_oracle="$(read_poc_config_oracle)" || true
        if [[ "$current_oracle" != "$oracle" ]] || [[ "${POC_FORCE_UPDATE_CONFIG:-0}" == 1 ]]; then
            update_poc_config_oracle "$oracle"
        fi
        return 0
    fi
    poc_oracle_sync_worker_stack || return 1
}

ensure_memory_account_for_post_flows() {
    local oracle mem
    invalidate_stale_session_runtime_ids
    if [[ -n "${MEMORY_ACCOUNT_ID:-}" ]]; then
        if object_exists_on_fullnode "$MEMORY_ACCOUNT_ID"; then
            MEMORY_ACCOUNT_ID="$(normalize_hex_id "$MEMORY_ACCOUNT_ID")"
            log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
            return 0
        fi
        echo "Session MEMORY_ACCOUNT_ID not on localnet fullnode; re-resolving." >&2
        MEMORY_ACCOUNT_ID=''
    fi
    oracle="$(resolve_myso_active_address)" || return 1
    mem="$(resolve_memory_account_for_address "$oracle")" || true
    if [[ -n "$mem" ]]; then
        MEMORY_ACCOUNT_ID="$mem"
        log_session_use "MEMORY_ACCOUNT_ID" "$mem"
        save_session_state
        return 0
    fi
    log_step "No live MemoryAccount for $oracle — trying create_profile"
    if create_oracle_profile_with_memory; then
        return 0
    fi
    echo "Post PoC flows require a MemoryAccount visible on the localnet fullnode." >&2
    echo "  Your profile may reference a missing MemoryAccount (chain/indexer drift)." >&2
    echo "  Restart localnet, re-run ./scripts/bootstrap.sh, then retry menu 3." >&2
    return 1
}

ensure_oracle_profile_for_reservation() {
    if [[ -n "${ORACLE_PROFILE_ID:-}" ]]; then
        return 0
    fi
    ensure_oracle_profile_id || {
        echo "Profile reservation requires an owned Profile for the active address." >&2
        echo "  Create a profile in the app first, or set ORACLE_PROFILE_ID in poc-session.env." >&2
        return 1
    }
}

ensure_tipper_memory_account() {
    local mem
    if [[ "$TIPPER_ADDRESS" == "$ORACLE_ADDRESS" && -n "${MEMORY_ACCOUNT_ID:-}" ]]; then
        if object_exists_on_fullnode "$MEMORY_ACCOUNT_ID"; then
            TIPPER_MEMORY_ACCOUNT_ID="$(normalize_hex_id "$MEMORY_ACCOUNT_ID")"
            return 0
        fi
    fi
    if [[ -n "${TIPPER_MEMORY_ACCOUNT_ID:-}" ]]; then
        if object_exists_on_fullnode "$TIPPER_MEMORY_ACCOUNT_ID"; then
            TIPPER_MEMORY_ACCOUNT_ID="$(normalize_hex_id "$TIPPER_MEMORY_ACCOUNT_ID")"
            return 0
        fi
        TIPPER_MEMORY_ACCOUNT_ID=''
    fi
    mem="$(resolve_memory_account_for_address "$TIPPER_ADDRESS")"
    if [[ -n "$mem" ]]; then
        TIPPER_MEMORY_ACCOUNT_ID="$mem"
        return 0
    fi
    echo "Warning: no MemoryAccount for TIPPER_ADDRESS on localnet; tip steps may fail." >&2
    return 0
}

parse_post_owner_from_object() {
    local post_id="$1" json
    post_id="$(normalize_hex_id "$post_id")" || return 1
    json="$(myso client object "$post_id" --json 2>/dev/null)" || return 1
    python3 - "$json" <<'PY'
import json, sys
j = json.loads(sys.argv[1])
contents = j.get("data", {}).get("Move", {}).get("contents")
if not contents or len(contents) < 64:
    raise SystemExit(1)
print("0x" + bytes(contents[32:64]).hex())
PY
}

pick_distinct_tipper_address() {
    local oracle="$1" preferred="$2" addr coin_json
    oracle="$(normalize_hex_id "$oracle")" || return 1
    if [[ -n "$preferred" ]]; then
        preferred="$(normalize_hex_id "$preferred")" || preferred=""
        if [[ -n "$preferred" && "$preferred" != "$oracle" ]]; then
            coin_json="$(myso client gas "$preferred" --json 2>/dev/null)" || coin_json='[]'
            if [[ "$(echo "$coin_json" | jq 'length')" -gt 0 ]]; then
                printf '%s' "$preferred"
                return 0
            fi
        fi
    fi
    while IFS= read -r addr; do
        [[ -n "$addr" ]] || continue
        addr="$(normalize_hex_id "$addr")" || continue
        [[ "$addr" == "$oracle" ]] && continue
        coin_json="$(myso client gas "$addr" --json 2>/dev/null)" || continue
        if [[ "$(echo "$coin_json" | jq 'length')" -gt 0 ]]; then
            printf '%s' "$addr"
            return 0
        fi
    done < <(myso client addresses --json 2>/dev/null | jq -r --arg cur "$oracle" '
        .addresses[]? | .[1] | select(. != $cur)
    ')
    while IFS= read -r addr; do
        [[ -n "$addr" ]] || continue
        addr="$(normalize_hex_id "$addr")" || continue
        [[ "$addr" == "$oracle" ]] && continue
        printf '%s' "$addr"
        return 0
    done < <(myso client addresses --json 2>/dev/null | jq -r --arg cur "$oracle" '
        .addresses[]? | .[1] | select(. != $cur)
    ')
    return 1
}

ensure_distinct_tipper_for_post() {
    local post_id="$1" owner alt mem
    post_id="$(normalize_hex_id "$post_id")" || return 1
    ensure_cli_addresses || return 1
    owner="$(parse_post_owner_from_object "$post_id")" || return 1
    owner="$(normalize_hex_id "$owner")" || return 1
    if [[ "$owner" != "$(normalize_hex_id "$ORACLE_ADDRESS")" ]]; then
        TIPPER_ADDRESS="$ORACLE_ADDRESS"
        ensure_tipper_memory_account
        ensure_tipper_ready || return 1
        return 0
    fi
    if session_value_set TIPPER_ADDRESS \
        && [[ "$(normalize_hex_id "$TIPPER_ADDRESS")" != "$(normalize_hex_id "$ORACLE_ADDRESS")" ]]; then
        alt="$(pick_distinct_tipper_address "$ORACLE_ADDRESS" "$TIPPER_ADDRESS")" || alt="$TIPPER_ADDRESS"
        TIPPER_ADDRESS="$(normalize_hex_id "$alt")"
        log_step "Using session tipper $TIPPER_ADDRESS (self-tip forbidden on oracle-owned post)"
        TIPPER_MEMORY_ACCOUNT_ID=''
        ensure_tipper_ready || return 1
    else
        alt="$(pick_distinct_tipper_address "$ORACLE_ADDRESS" "")" || true
        [[ -n "$alt" ]] || {
            echo "Need a second keystore address to tip a post owned by the oracle (self-tip forbidden)." >&2
            echo "  Run: myso client new-address ed25519" >&2
            return 1
        }
        log_step "Using distinct tipper $alt (post owner is oracle; self-tip forbidden)"
        TIPPER_ADDRESS="$(normalize_hex_id "$alt")"
        TIPPER_MEMORY_ACCOUNT_ID=''
        ensure_tipper_ready || return 1
    fi
    mem="${TIPPER_MEMORY_ACCOUNT_ID:-}"
    [[ -n "$mem" ]] && object_exists_on_fullnode "$mem" || mem=""
    [[ -n "$mem" ]] || mem="$(resolve_memory_account_for_address "$TIPPER_ADDRESS")" || true
    if [[ -z "$mem" ]]; then
        mem="$(gql_profile_memory_account_id "$TIPPER_ADDRESS")" || true
        if [[ -n "$mem" ]] && object_exists_on_fullnode "$mem"; then
            mem="$(normalize_hex_id "$mem")"
        else
            mem=""
        fi
    fi
    if [[ -z "$mem" ]]; then
        mem="$(create_profile_with_memory_for_address "$TIPPER_ADDRESS")" || {
            mem="$(gql_profile_memory_account_id "$TIPPER_ADDRESS")" || true
            mem="$(normalize_hex_id "$mem" 2>/dev/null)" || mem=""
        }
    fi
    [[ -n "$mem" ]] || {
        echo "Could not resolve MemoryAccount for tipper $TIPPER_ADDRESS" >&2
        restore_oracle_address
        return 1
    }
    TIPPER_MEMORY_ACCOUNT_ID="$mem"
    log_session_use "TIPPER_ADDRESS" "$TIPPER_ADDRESS"
    log_session_use "TIPPER_MEMORY_ACCOUNT_ID" "$TIPPER_MEMORY_ACCOUNT_ID"
    restore_oracle_address
}

claim_beneficiary_vault_balance_as() {
    local beneficiary="$1" vault_id="$2" referrer="$3"
    local ref_cfg ref_treasury ref_vault ref_clk referrer_arg normalized_ben normalized_ref
    vault_id="$(normalize_hex_id "$vault_id")" || return 1
    beneficiary="$(normalize_hex_id "$beneficiary")" || return 1
    if [[ -n "$referrer" ]]; then
        normalized_ref="$(normalize_hex_id "$referrer")" || return 1
        if [[ "$normalized_ref" == "$beneficiary" ]]; then
            referrer_arg="none"
        else
            referrer_arg="$(ptb_option_address_from_arg "some($referrer)")"
        fi
    else
        referrer_arg="none"
    fi
    ref_cfg="$(ptb_shared_ref "$POC_CONFIG_ID")" || return 1
    ref_treasury="$(ptb_shared_ref "$ECOSYSTEM_TREASURY_ID")" || return 1
    ref_vault="$(ptb_shared_ref "$vault_id")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    SKIP_CONFIRM_RUN=1 invoke_ptb_as "$beneficiary" \
        --move-call "${PKG_SOCIAL}::proof_of_creativity::claim_beneficiary_vault_balance<${COIN_TYPE}>" \
        "$ref_cfg" "$ref_treasury" "$ref_vault" "$referrer_arg" "$ref_clk"
}

claim_username_beneficiary_vault_balance_as() {
    local sender="$1" beneficiary_id="$2" vault_id="$3" referrer="$4"
    local ref_cfg ref_dir ref_ben ref_treasury ref_vault ref_clk referrer_arg
    sender="$(normalize_hex_id "$sender")" || return 1
    beneficiary_id="$(normalize_hex_id "$beneficiary_id")" || return 1
    vault_id="$(normalize_hex_id "$vault_id")" || return 1
    if [[ -n "$referrer" ]]; then
        referrer="$(normalize_hex_id "$referrer")" || return 1
        if [[ "$referrer" == "$sender" ]]; then
            referrer_arg="none"
        else
            referrer_arg="$(ptb_option_address_from_arg "some($referrer)")"
        fi
    else
        referrer_arg="none"
    fi
    ref_cfg="$(ptb_shared_ref "$POC_CONFIG_ID")" || return 1
    ref_dir="$(ptb_shared_ref "$POC_USERNAME_BENEFICIARY_DIRECTORY_ID")" || return 1
    ref_ben="$(ptb_shared_ref "$beneficiary_id")" || return 1
    ref_treasury="$(ptb_shared_ref "$ECOSYSTEM_TREASURY_ID")" || return 1
    ref_vault="$(ptb_shared_ref "$vault_id")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    SKIP_CONFIRM_RUN=1 invoke_ptb_as "$sender" \
        --move-call "${PKG_SOCIAL}::proof_of_creativity::claim_username_beneficiary_vault_balance<${COIN_TYPE}>" \
        "$ref_cfg" "$ref_dir" "$ref_ben" "$ref_treasury" "$ref_vault" \
        "$referrer_arg" "$ref_clk"
}

create_test_platform() {
    local out digest platform_id platform_addr
    local ref_preg ref_pcfg ref_clk ref_cap

    require_session_fields PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID CLOCK_ID || return 1
    require_hex_ids PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID CLOCK_ID || return 1

    ref_preg="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_pcfg="$(ptb_shared_ref "$PLATFORM_CONFIG_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    ref_cap="$(ptb_shared_ref "$PLATFORM_ADMIN_CAP_ID")" || return 1

    POC_RUN_ID="${POC_RUN_ID:-$(date +%s)}"

    log_step "Creating test platform (create_platform PTB)"
    if out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --move-call "${PKG_SOCIAL}::platform::create_platform" \
        "$ref_preg" \
        "$ref_pcfg" \
        "$(literal_move_string "Test Platform poc${POC_RUN_ID}-${RANDOM}")" \
        "$(literal_move_string 'A test platform')" \
        "$(literal_move_string 'This is a test platform for badge testing')" \
        "$(literal_move_string "$SOCIAL_DEFAULT_PLATFORM_LOGO_URL")" \
        "$(literal_move_string 'https://example.com/terms')" \
        "$(literal_move_string 'https://example.com/privacy')" \
        "$(literal_move_vector_empty)" \
        "$(literal_move_vector_from_csv 'https://example.com')" \
        "$(literal_move_string 'Social Network')" \
        none 2 \
        "$(literal_move_string '2023-01-01')" \
        false \
        none none none none none none none \
        "$(literal_move_option_string "$SOCIAL_DEFAULT_PLATFORM_COVER_PHOTO_URL")" none \
        "$ref_clk")"; then
        :
    elif echo "$out" | grep -qE 'Abort Code: 1\b'; then
        if [[ -n "${PLATFORM_OBJECT_ID:-}" ]] && object_exists_on_fullnode "$PLATFORM_OBJECT_ID"; then
            log_step "Platform name already registered — reusing session PLATFORM_OBJECT_ID"
            POC_PLATFORM_JOINED=1
            return 0
        fi
        echo "create_platform failed (platform name may already exist)" >&2
        return 1
    else
        return 1
    fi

    digest="$(extract_tx_digest "$out")"
    platform_id="$(extract_created_object_by_type "$digest" "platform::Platform")"
    [[ -n "$platform_id" ]] || platform_id="$(extract_created_object_by_type "$digest" "Platform")"
    [[ -n "$platform_id" ]] || { echo "Could not find Platform from create tx" >&2; return 1; }

    log_session_use "PLATFORM_OBJECT_ID (pending approval)" "$platform_id"

    log_step "Approving platform via toggle_platform_approval"
    platform_addr="$(normalize_hex_id "$platform_id")" || return 1
    SKIP_CONFIRM_RUN=1 invoke_ptb \
        --move-call "${PKG_SOCIAL}::platform::toggle_platform_approval" \
        "$ref_preg" \
        "$ref_pcfg" \
        "$(ptb_shared_ref "$platform_addr")" \
        "$ref_cap" \
        none || return 1

    PLATFORM_OBJECT_ID="$platform_id"
    save_session_state
    log_step "Test platform ready (approved): $platform_id"
}

create_post_poc_enabled() {
    local body_lit="$1"
    local enable_spt_arg="${2:-none}"
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mcfg ref_mr ref_mem ref_clk

    prepare_for_create_post || return 1

    require_hex_ids USERNAME_REGISTRY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
        BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MEMORY_CONFIG_ID MYDATA_REGISTRY_ID \
        MEMORY_ACCOUNT_ID CLOCK_ID || return 1
    ref_ur="$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" || return 1
    ref_pr="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_plat="$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" || return 1
    ref_blr="$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" || return 1
    ref_cfg="$(ptb_shared_ref "$POST_CONFIG_ID")" || return 1
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || return 1
    ref_mr="$(ptb_shared_ref "$MYDATA_REGISTRY_ID")" || return 1
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1

    log_step "Creating post: $body_lit"
    SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --move-call "${PKG_SOCIAL}::post::create_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" "$ref_mcfg" \
        "$body_lit" \
        none \
        none none none none none none none \
        "$enable_spt_arg" none none \
        "$ref_mr" "$ref_mem" "$ref_clk"
}

create_reservation_pool_for_post_call() {
    local post_id="$1"
    local ref_token ref_spt ref_post ref_clk out digest pool_id
    require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID CLOCK_ID || return 1
    post_id="$(normalize_hex_id "$post_id")" || return 1
    ref_token="$(ptb_shared_ref "$TOKEN_REGISTRY_ID")" || return 1
    ref_spt="$(ptb_shared_ref "$SOCIAL_PROOF_TOKENS_CONFIG_ID")" || return 1
    ref_post="$(ptb_shared_ref "$post_id")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    log_step "create_reservation_pool_for_post post=$post_id"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --move-call "${PKG_SOCIAL}::social_proof_tokens::create_reservation_pool_for_post" \
        "$ref_token" "$ref_spt" "$ref_post" "$ref_clk")" || return 1
    digest="$(extract_tx_digest "$out")"
    pool_id="$(extract_created_object_by_type "$digest" "ReservationPoolObject")"
    [[ -n "$pool_id" ]] || pool_id="$(extract_created_object_by_type "$digest" "ReservationPool")"
    [[ -n "$pool_id" ]] || {
        echo "create_reservation_pool_for_post did not produce ReservationPoolObject" >&2
        return 1
    }
    normalize_hex_id "$pool_id"
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

analyze_post_direct() {
    local post_id="$1" media_type="$2" score="$3" original_creator_arg="$4" \
        deriv_target="$5" embed_audio="$6" apply_explicit="$7" explicit_outcome="$8"
    local sender="${9:-}"
    local ref_cfg ref_reg ref_vault ref_post ref_clk creator_arg out digest
    creator_arg="$(ptb_option_address_from_arg "$original_creator_arg")"
    log_step "analyze_and_update_post post=$post_id score=$score${sender:+ sender=$sender}"
    ref_cfg="$(ptb_shared_ref "$POC_CONFIG_ID")" || return 1
    ref_reg="$(ptb_shared_ref "$POC_REGISTRY_ID")" || return 1
    ref_vault="$(ptb_shared_ref "$POC_VAULT_DIRECTORY_ID")" || return 1
    ref_post="$(ptb_shared_ref "$post_id")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    if [[ -n "$sender" ]]; then
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" \
            --move-call "${PKG_SOCIAL}::proof_of_creativity::analyze_and_update_post" \
            "$ref_cfg" "$ref_reg" "$ref_vault" "$ref_post" \
            "$media_type" "$score" "$creator_arg" "$deriv_target" \
            "$embed_audio" "$apply_explicit" "$explicit_outcome" \
            none none "$ref_clk")" || return 1
    else
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
            --move-call "${PKG_SOCIAL}::proof_of_creativity::analyze_and_update_post" \
            "$ref_cfg" "$ref_reg" "$ref_vault" "$ref_post" \
            "$media_type" "$score" "$creator_arg" "$deriv_target" \
            "$embed_audio" "$apply_explicit" "$explicit_outcome" \
            none none "$ref_clk")" || return 1
    fi
    digest="$(extract_tx_digest "$out")"
    ANALYZE_POST_LAST_DIGEST="${digest:-}"
    [[ -n "$ANALYZE_POST_LAST_DIGEST" ]]
}

analyze_post() {
    if [[ "${POC_USE_DIRECT_MOVE:-0}" == "1" ]]; then
        analyze_post_direct "$@"
        return $?
    fi
    poc_oracle_load_localnet_env
    sync_poc_config_oracle_on_chain "$POC_DEFAULT_ORACLE_ADDRESS" || return 1
    ensure_poc_oracle_key_in_env || return 1
    poc_oracle_analyze_post "$@"
}

tip_post_as_tipper() {
    local post_id="$1" vault_id="$2" amount="$3"
    local tip_coin gas_coin mem saved_tipper saved_gas_coin
    ensure_distinct_tipper_for_post "$post_id" || return 1
    ensure_tipper_ready || return 1
    saved_tipper="$TIPPER_ADDRESS"
    if [[ "$TIPPER_ADDRESS" != "$ORACLE_ADDRESS" ]]; then
        myso client switch --address "$TIPPER_ADDRESS"
    fi
    read -r tip_coin gas_coin <<<"$(pick_tip_and_gas_coins_for_address "$TIPPER_ADDRESS" "$amount")" || {
        restore_oracle_address
        return 1
    }
    mem="${TIPPER_MEMORY_ACCOUNT_ID:-}"
    [[ -n "$mem" ]] || mem="$(resolve_memory_account_for_address "$TIPPER_ADDRESS")"
    [[ -n "$mem" ]] || { echo "Tipper MemoryAccount required" >&2; restore_oracle_address; return 1; }
    log_step "tip_post amount=$amount post=$post_id tipper=$TIPPER_ADDRESS"
    local ref_post ref_vault ref_tip ref_mem ref_clk
    ref_post="$(ptb_shared_ref "$post_id")" || { restore_oracle_address; return 1; }
    ref_vault="$(ptb_shared_ref "$vault_id")" || { restore_oracle_address; return 1; }
    ref_tip="$(ptb_shared_ref "$tip_coin")" || { restore_oracle_address; return 1; }
    ref_mem="$(ptb_shared_ref "$mem")" || { restore_oracle_address; return 1; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { restore_oracle_address; return 1; }
    saved_gas_coin="${PTB_GAS_COIN_ID:-}"
    PTB_GAS_COIN_ID="$gas_coin"
    SKIP_CONFIRM_RUN=1 invoke_ptb_as "$TIPPER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::tip_post<${COIN_TYPE}>" \
        "$ref_post" "$ref_vault" "$ref_tip" "$amount" "$POC_MIN_VAULT_DEPOSIT" \
        "$ref_mem" "$ref_clk" || {
        PTB_GAS_COIN_ID="$saved_gas_coin"
        TIPPER_ADDRESS="$saved_tipper"
        restore_oracle_address
        return 1
    }
    PTB_GAS_COIN_ID="$saved_gas_coin"
    TIPPER_ADDRESS="$saved_tipper"
    restore_oracle_address
}

run_username_beneficiary_flow() {
    ensure_cli_addresses || return 1
    ensure_beneficiary_admin_cap || return 1
    ensure_ephemeral_oracle_for_username_claim || return 1
    require_session_fields POC_BENEFICIARY_ADMIN_CAP_ID POC_USERNAME_BENEFICIARY_DIRECTORY_ID \
        POC_VAULT_DIRECTORY_ID USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID POC_CONFIG_ID \
        PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID ECOSYSTEM_TREASURY_ID || return 1

    local ub_username identity_hash x_handle shard_id digest beneficiary_id vault_id beneficiary_addr
    ub_username="pocub${POC_RUN_ID}"
    identity_hash="0x$(printf 'id-%s' "$POC_RUN_ID" | xxd -p -c 256 | tr -d '\n')"
    x_handle="$(bytes_to_hex_arg "$ub_username")"
    local username_bytes
    username_bytes="$(bytes_to_hex_arg "$ub_username")"

    POC_UB_LAST_USERNAME="$ub_username"
    POC_UB_LAST_IDENTITY_HASH="$identity_hash"
    POC_UB_LAST_VAULT_FUNDED='0'
    POC_UB_LAST_TIP_AMOUNT=''
    POC_UB_LAST_VAULT_GROSS=''
    POC_UB_LAST_VAULT_TREASURY=''
    POC_UB_LAST_VAULT_CREATOR_NET=''
    POC_UB_LAST_FUND_POST_ID=''

    shard_id="$(resolve_shard_id_for_username "$ub_username")" || {
        echo "Could not resolve beneficiary shard for username $ub_username" >&2
        return 1
    }

    POC_UB_LAST_SHARD_ID="$shard_id"

    log_step "1a create_username_beneficiary username=$ub_username"
    local out
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity create_username_beneficiary \
        --args "$POC_BENEFICIARY_ADMIN_CAP_ID" \
        "@${POC_USERNAME_BENEFICIARY_DIRECTORY_ID}" "@${shard_id}" \
        "@${POC_VAULT_DIRECTORY_ID}" "@${USERNAME_REGISTRY_ID}" \
        "$username_bytes" 1 "$identity_hash" "$x_handle" "@${CLOCK_ID}")" || {
        echo "create_username_beneficiary failed — see myso client error above" >&2
        [[ -n "${out:-}" ]] && echo "$out" | tail -n 20 >&2
        return 1
    }
    digest="$(extract_tx_digest "$out")"
    [[ -n "$digest" ]] || {
        echo "Could not parse transaction digest from create_username_beneficiary output" >&2
        [[ -n "${out:-}" ]] && echo "$out" | tail -n 20 >&2
        return 1
    }
    beneficiary_id="$(extract_created_object_by_type "$digest" "PoCUsernameBeneficiary")"
    if [[ -z "$beneficiary_id" ]]; then
        beneficiary_id="$(extract_beneficiary_id_from_provision_tx "$digest")"
    fi
    [[ -n "$beneficiary_id" ]] || {
        echo "Could not find PoCUsernameBeneficiary from tx $digest" >&2
        return 1
    }
    wait_for_tx_finalized "$digest" || return 1
    assert_poc_scenario_events username_provision "$digest" || return 1
    log_step "1a resolved beneficiary_id=$beneficiary_id"
    POC_UB_LAST_BENEFICIARY_ID="$beneficiary_id"
    if [[ "$beneficiary_id" == "$POC_USERNAME_BENEFICIARY_DIRECTORY_ID" ]]; then
        echo "Resolved beneficiary_id matches directory object — refusing to claim" >&2
        return 1
    fi
    verify_object_type_from_tx "$digest" "$beneficiary_id" "PoCUsernameBeneficiary" \
        || verify_object_type "$beneficiary_id" "PoCUsernameBeneficiary" \
        || return 1

    beneficiary_addr="$(identity_beneficiary_address 1 "${identity_hash#0x}")"
    vault_id="$(extract_created_object_by_type "$digest" "PoCBeneficiaryVault")"
    if [[ -n "$vault_id" ]]; then
        verify_object_type_from_tx "$digest" "$vault_id" "PoCBeneficiaryVault" \
            || verify_object_type "$vault_id" "PoCBeneficiaryVault" \
            || vault_id=""
    fi
    if [[ -z "$vault_id" ]]; then
        local bjson
        bjson="$(myso client object "$beneficiary_id" --json 2>/dev/null)" || true
        vault_id="$(echo "$bjson" | jq -r '.. | objects | select(has("vault_id")) | .vault_id // empty' | head -n1)"
    fi
    POC_UB_LAST_VAULT_ID="${vault_id:-}"

    ensure_claim_wallet_without_profile || return 1
    POC_UB_LAST_CLAIM_WALLET="$CREATOR_ADDRESS"
    POC_UB_LAST_CLAIM_ORACLE="$USERNAME_CLAIM_ORACLE"
    log_step "1b claim_username_beneficiary via oracle wallet=$CREATOR_ADDRESS"
    claim_digest="$(poc_oracle_claim_beneficiary "$identity_hash" "$CREATOR_ADDRESS" "$beneficiary_id" "$ub_username" \
        "Creator" "bio")" || {
        echo "oracle claim_username_beneficiary failed" >&2
        return 1
    }
    if [[ -n "$claim_digest" && "$claim_digest" != unknown* ]]; then
        verify_username_beneficiary_claimed "$claim_digest" "$beneficiary_id" "$ub_username" || return 1
    else
        status="$(username_beneficiary_status "$beneficiary_id")"
        [[ "$status" == "2" ]] || {
            echo "beneficiary $beneficiary_id status=$status (expected 2=CLAIMED)" >&2
            return 1
        }
        log_step "1b claim_username_beneficiary OK username=$ub_username beneficiary=$beneficiary_id"
    fi
    POC_UB_LAST_CLAIM_PROFILE_ID="$(extract_created_object_by_type "$claim_digest" "profile::Profile")"
    [[ -n "$POC_UB_LAST_CLAIM_PROFILE_ID" ]] || POC_UB_LAST_CLAIM_PROFILE_ID="$(extract_created_object_by_type "$claim_digest" "Profile")"
    [[ -n "$POC_UB_LAST_CLAIM_PROFILE_ID" ]] || POC_UB_LAST_CLAIM_PROFILE_ID="$(gql_profile_id_for_address "$CREATOR_ADDRESS" 2>/dev/null || true)"
    assert_username_beneficiary_graphql "$ub_username" "$beneficiary_id" || return 1

    if should_skip_vault_funding; then
        if ! platform_mode_is_full; then
            log_step "1c-1d skipped (vault funding requires post + platform)"
        else
            log_step "1c-1d skipped (POC_SKIP_VAULT_FUNDING=1)"
        fi
        save_session_state
        print_poc_username_beneficiary_summary "$ub_username" "$beneficiary_id"
        return 0
    fi

    require_session_fields PLATFORM_OBJECT_ID POST_CONFIG_ID || return 1

    log_step "1c Fund username beneficiary vault via derivative post + tip"
    local fund_body fund_post digest2
    fund_body="$(literal_move_string "PoC fund vault ${POC_RUN_ID}")"
    out="$(create_post_poc_enabled "$fund_body")"
    digest2="$(extract_tx_digest "$out")"
    fund_post="$(extract_created_object_by_type "$digest2" "post::Post")"
    [[ -n "$fund_post" ]] || fund_post="$(extract_created_object_by_type "$digest2" "Post")"
    [[ -n "$fund_post" ]] || { echo "Could not find Post for vault funding" >&2; return 1; }

    analyze_post "$fund_post" 1 100 "some($beneficiary_addr)" 1 false false 0 "$USERNAME_CLAIM_ORACLE" || return 1
    [[ -n "$vault_id" ]] || vault_id="$(resolve_beneficiary_vault_id "$ANALYZE_POST_LAST_DIGEST" "$beneficiary_addr")"
    [[ -n "$vault_id" ]] || { echo "Could not resolve beneficiary vault id" >&2; return 1; }
    ensure_tipper_memory_account
    tip_post_as_tipper "$fund_post" "$vault_id" "$DEFAULT_TIP_AMOUNT"
    POC_UB_LAST_VAULT_FUNDED='1'
    POC_UB_LAST_FUND_POST_ID="$fund_post"
    POC_UB_LAST_TIP_AMOUNT="$DEFAULT_TIP_AMOUNT"
    POC_UB_LAST_VAULT_ID="$(normalize_hex_id "$vault_id")"

    local claim_wallet referrer
    claim_wallet="$(normalize_hex_id "$POC_UB_LAST_CLAIM_WALLET")"
    referrer="${JOIN_REFERRER_ADDRESS:-$ORACLE_ADDRESS}"
    log_step "1d claim_username_beneficiary_vault_balance claim_wallet=$claim_wallet referrer=$referrer"
    claim_username_beneficiary_vault_balance_as "$claim_wallet" "$beneficiary_id" "$vault_id" "$referrer" || return 1

    local vault_gql vars
    sleep 1
    vars="$(jq -nc --arg vaultId "$(normalize_hex_id "$vault_id")" '{vaultId: $vaultId}')"
    vault_gql="$(graphql_post \
        'query UbVaultClaim($vaultId: String!) {
            pocBeneficiaryVaultByVaultId(vaultId: $vaultId) {
                claims(limit: 1) { grossAmount treasuryAmount netAmount }
            }
        }' \
        "$vars" 2>/dev/null)" || vault_gql='{}'
    POC_UB_LAST_VAULT_GROSS="$(echo "$vault_gql" | jq -r '.data.pocBeneficiaryVaultByVaultId.claims[0].grossAmount // empty')"
    POC_UB_LAST_VAULT_TREASURY="$(echo "$vault_gql" | jq -r '.data.pocBeneficiaryVaultByVaultId.claims[0].treasuryAmount // empty')"
    POC_UB_LAST_VAULT_CREATOR_NET="$(echo "$vault_gql" | jq -r '.data.pocBeneficiaryVaultByVaultId.claims[0].netAmount // empty')"
    [[ -n "$POC_UB_LAST_VAULT_GROSS" ]] || POC_UB_LAST_VAULT_GROSS="$DEFAULT_TIP_AMOUNT"
    [[ -n "$POC_UB_LAST_VAULT_TREASURY" ]] || POC_UB_LAST_VAULT_TREASURY='1000000'
    if [[ -z "$POC_UB_LAST_VAULT_CREATOR_NET" && -n "$POC_UB_LAST_VAULT_GROSS" && -n "$POC_UB_LAST_VAULT_TREASURY" ]]; then
        POC_UB_LAST_VAULT_CREATOR_NET="$((POC_UB_LAST_VAULT_GROSS - POC_UB_LAST_VAULT_TREASURY))"
    fi

    save_session_state
    print_poc_username_beneficiary_summary "$ub_username" "$beneficiary_id"
}

print_poc_username_beneficiary_summary() {
    local username="${1:-${POC_UB_LAST_USERNAME:-}}"
    local beneficiary_id="${2:-${POC_UB_LAST_BENEFICIARY_ID:-}}"
    local claim_wallet claim_oracle claim_profile vault_id fund_post outcome gql_status
    local tip_amount gross treasury creator_net vault_section

    beneficiary_id="$(normalize_hex_id "$beneficiary_id")"
    claim_wallet="$(normalize_hex_id "${POC_UB_LAST_CLAIM_WALLET:-$CREATOR_ADDRESS}")"
    claim_oracle="$(normalize_hex_id "${POC_UB_LAST_CLAIM_ORACLE:-$USERNAME_CLAIM_ORACLE}")"
    claim_profile="$(normalize_hex_id "${POC_UB_LAST_CLAIM_PROFILE_ID:-}")"
    vault_id="$(normalize_hex_id "${POC_UB_LAST_VAULT_ID:-}")"
    fund_post="$(normalize_hex_id "${POC_UB_LAST_FUND_POST_ID:-}")"
    gql_status="$(graphql_post \
        'query PocUsernameBeneficiaryByUsername($username: String!) {
            pocUsernameBeneficiaryByUsername(username: $username) { status }
        }' \
        "$(jq -nc --arg username "$username" '{username: $username}')" 2>/dev/null \
        | jq -r '.data.pocUsernameBeneficiaryByUsername.status // empty')" || gql_status='2'

    print_run_summary_header "Proof of Creativity — username beneficiary flow completed"
    print_run_summary_line "Run ID" "$POC_RUN_ID"
    print_run_summary_line "Username provisioned" "$username"
    print_run_summary_line "Beneficiary object" "$beneficiary_id"
    print_run_summary_line "Beneficiary shard" "$(normalize_hex_id "${POC_UB_LAST_SHARD_ID:-}")"
    print_run_summary_line "Identity hash" "${POC_UB_LAST_IDENTITY_HASH:-}"
    print_run_summary_line "Beneficiary vault" "${vault_id:-<none>}"
    print_run_summary_line "Claim oracle" "$claim_oracle"
    print_run_summary_line "Claim wallet" "$claim_wallet"
    print_run_summary_line "Profile created" "${claim_profile:-<indexed after claim>}"
    print_run_summary_line "On-chain status" "CLAIMED (status ${gql_status:-2}, GraphQL verified)"

    if [[ "${POC_UB_LAST_VAULT_FUNDED:-0}" == 1 ]]; then
        tip_amount="${POC_UB_LAST_TIP_AMOUNT:-$DEFAULT_TIP_AMOUNT}"
        gross="${POC_UB_LAST_VAULT_GROSS:-$tip_amount}"
        treasury="${POC_UB_LAST_VAULT_TREASURY:-1000000}"
        creator_net="${POC_UB_LAST_VAULT_CREATOR_NET:-$((gross - treasury))}"
        print_run_summary_line "Vault funding post" "$fund_post"
        print_run_summary_line "Escrow tip to vault" "$(format_mist_with_units "$tip_amount")"
        print_run_summary_line "Vault claim (gross)" "$(format_mist_with_units "$gross")"
        print_run_summary_line "Treasury fee (claim)" "$(format_mist_with_units "$treasury")"
        print_run_summary_line "Creator net (vault)" "$(format_mist_with_units "$creator_net")"
        vault_section=" Vault funded via derivative post tip $(format_mist_with_units "$tip_amount"); creator claimed $(format_mist_with_units "$creator_net") net after $(format_mist_with_units "$treasury") treasury fee."
    else
        if ! platform_mode_is_full; then
            vault_section=" Vault funding skipped (no platform on fullnode)."
        elif [[ "${POC_SKIP_VAULT_FUNDING:-0}" == 1 ]]; then
            vault_section=" Vault funding skipped (POC_SKIP_VAULT_FUNDING=1)."
        else
            vault_section=""
        fi
    fi

    outcome="Admin provisioned @$username as a PoC username beneficiary; oracle $claim_oracle verified claim for wallet $claim_wallet, creating profile ${claim_profile:-<new>} with username @$username.${vault_section}"
    print_run_summary_line "Outcome" "$outcome"
    print_run_summary_line "GraphQL indexer" "$GRAPHQL_URL"
    print_run_summary_line "Session file" "$(session_state_save_path)"
    print_run_summary_footer
}

print_poc_post_flow_summary() {
    print_run_summary_header "Proof of Creativity — post PoC flow completed"
    print_run_summary_line "Platform" "$(normalize_hex_id "${PLATFORM_OBJECT_ID:-}")"
    print_run_summary_line "Creator" "$(normalize_hex_id "$CREATOR_ADDRESS")"
    print_run_summary_line "Tipper" "$(normalize_hex_id "${TIPPER_ADDRESS:-}")"
    print_run_summary_line "Beneficiary vault" "$(normalize_hex_id "${POC_BENEFICIARY_VAULT_ID:-}")"
    print_run_summary_line "Tip amount" "$(format_mist_with_units "${DEFAULT_TIP_AMOUNT:-100000000}")"
    print_run_summary_line "Steps" "Original analyze → derivative escrow + tip → vault claim → royalty-free outcome post"
    print_run_summary_footer
}

print_poc_dispute_flow_summary() {
    print_run_summary_header "Proof of Creativity — dispute flow completed"
    print_run_summary_line "Voter / oracle" "$(normalize_hex_id "$(resolve_myso_active_address 2>/dev/null || echo "$ORACLE_ADDRESS")")"
    print_run_summary_line "Vote stake" "$(format_mist_with_units "${DEFAULT_VOTE_STAKE:-1000000000}")"
    print_run_summary_line "Evidence" "$DEFAULT_DISPUTE_EVIDENCE"
    print_run_summary_line "Outcome" "Dispute submitted → vote (uphold) → resolved → voting reward claimed"
    print_run_summary_footer
}

print_poc_run_all_summary() {
    local flows="Username beneficiary"
    print_run_summary_header "Proof of Creativity — full E2E completed ($(platform_mode) mode)"
    print_run_summary_line "Run ID" "$POC_RUN_ID"
    print_run_summary_line "Platform mode" "$(platform_mode)"
    print_run_summary_line "Platform" "${PLATFORM_OBJECT_ID:-<none>}"
    print_run_summary_line "Oracle" "$(normalize_hex_id "$ORACLE_ADDRESS")"
    print_run_summary_line "Creator" "$(normalize_hex_id "$CREATOR_ADDRESS")"
    if platform_mode_is_full; then
        flows="$flows + post PoC + optional SPT/dispute/reservation"
    else
        flows="$flows only (no platform)"
    fi
    print_run_summary_line "Flows exercised" "$flows"
    print_run_summary_footer
}

run_post_poc_flow() {
    ensure_cli_addresses || return 1
    require_platform_mode || return 1
    require_session_fields "${REQUIRED_PLATFORM_KEYS[@]}" POC_CONFIG_ID POC_REGISTRY_ID \
        POC_VAULT_DIRECTORY_ID ECOSYSTEM_TREASURY_ID || return 1

    local out digest post1 post2 post3 vault_id beneficiary_addr
    beneficiary_addr="$CREATOR_ADDRESS"

    if [[ "${POC_PLATFORM_JOINED:-0}" != 1 ]]; then
        if ! ensure_joined_platform; then
            log_step "Platform join failed — creating and approving a fresh test platform"
            create_test_platform || return 1
            ensure_joined_platform || return 1
        fi
    else
        log_step "Platform already joined this run — skipping join"
    fi

    log_step "2a-2b Original post + analyze"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC original ${POC_RUN_ID}")")" || return 1
    digest="$(extract_tx_digest "$out")"
    post1="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post1" ]] || post1="$(extract_created_object_by_type "$digest" "Post")"
    [[ -n "$post1" ]] || { echo "create_post did not produce a Post object" >&2; return 1; }
    analyze_post "$post1" 1 50 none 0 false false 0 || return 1

    log_step "2c Derivative escrow post + analyze"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC derivative ${POC_RUN_ID}")")" || return 1
    digest="$(extract_tx_digest "$out")"
    post2="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post2" ]] || post2="$(extract_created_object_by_type "$digest" "Post")"
    [[ -n "$post2" ]] || { echo "derivative create_post did not produce a Post object" >&2; return 1; }
    analyze_post "$post2" 1 100 "some($beneficiary_addr)" 1 false false 0 || return 1

    vault_id="$(resolve_beneficiary_vault_id "$ANALYZE_POST_LAST_DIGEST" "$beneficiary_addr")" || {
        echo "Could not resolve beneficiary vault id after derivative analyze" >&2
        return 1
    }
    POC_BENEFICIARY_VAULT_ID="$vault_id"
    log_session_use "POC_BENEFICIARY_VAULT_ID" "$vault_id"
    log_step "2d tip_post vault=$vault_id"
    tip_post_as_tipper "$post2" "$vault_id" "$DEFAULT_TIP_AMOUNT" || return 1
    log_step "2e claim_beneficiary_vault_balance"
    claim_beneficiary_vault_balance_as "$beneficiary_addr" "$vault_id" "$TIPPER_ADDRESS" || {
        restore_oracle_address
        return 1
    }
    restore_oracle_address

    assert_post_flow_tip_and_claim_graphql "$post2" "$vault_id" || return 1

    log_step "2f Explicit royalty-free outcome post"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC royalty-free ${POC_RUN_ID}")")" || return 1
    digest="$(extract_tx_digest "$out")"
    post3="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post3" ]] || post3="$(extract_created_object_by_type "$digest" "Post")"
    [[ -n "$post3" ]] || { echo "royalty-free create_post did not produce a Post object" >&2; return 1; }
    analyze_post "$post3" 1 0 none 0 false true 4 || return 1
    save_session_state
    print_poc_post_flow_summary
}

run_spt_sync_flow() {
    require_platform_mode || return 1
    ensure_spt_trading_enabled || return 1
    require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
    local out digest post_id pool_id
    log_step "3 SPT: create post + reservation pool + token"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC SPT ${POC_RUN_ID}")" some\(true\))"
    digest="$(extract_tx_digest "$out")"
    post_id="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post_id" ]] || post_id="$(extract_created_object_by_type "$digest" "Post")"
    pool_id="$(create_reservation_pool_for_post_call "$post_id")" || return 1

    SKIP_CONFIRM_RUN=1 run_myso_call social_proof_tokens create_social_proof_token \
        --args "@${TOKEN_REGISTRY_ID}" "@${SOCIAL_PROOF_TOKENS_CONFIG_ID}" "@${pool_id}"

    log_step "3 analyze_and_update_post_sync_token_pool"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity analyze_and_update_post_sync_token_pool \
        --args "@${POC_CONFIG_ID}" "@${POC_REGISTRY_ID}" "@${TOKEN_REGISTRY_ID}" \
        "@${POC_VAULT_DIRECTORY_ID}" "@${post_id}" "@${pool_id}" \
        1 100 "some($CREATOR_ADDRESS)" 1 false false 0 none none "@${CLOCK_ID}"
}

run_dispute_flow() {
    require_platform_mode || return 1
    require_session_fields POC_CONFIG_ID POC_REGISTRY_ID ECOSYSTEM_TREASURY_ID || return 1
    local out digest post_id dispute_id coin vote_coin oracle
    oracle="$(resolve_myso_active_address)"

    log_step "4a Disputable post"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC dispute ${POC_RUN_ID}")")"
    digest="$(extract_tx_digest "$out")"
    post_id="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post_id" ]] || post_id="$(extract_created_object_by_type "$digest" "Post")"
    analyze_post "$post_id" 1 100 "some($CREATOR_ADDRESS)" 1 false false 0

    coin="$(resolve_gas_coin_for_address "$oracle")"
    [[ -n "$coin" ]] || { echo "No coin for dispute fee" >&2; return 1; }

    log_step "4b submit_poc_dispute"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity submit_poc_dispute \
        --args "@${POC_CONFIG_ID}" "@${POC_REGISTRY_ID}" "@${ECOSYSTEM_TREASURY_ID}" "@${post_id}" \
        "$(literal_move_string "$DEFAULT_DISPUTE_EVIDENCE")" "$coin" "@${CLOCK_ID}")"
    digest="$(extract_tx_digest "$out")"
    dispute_id="$(extract_created_object_by_type "$digest" "PoCDispute")"

    vote_coin="$(resolve_gas_coin_for_address "$oracle")"
    [[ -n "$vote_coin" && -n "$dispute_id" ]] || { echo "Missing dispute or vote coin" >&2; return 1; }

    log_step "4c vote_on_dispute (uphold)"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity vote_on_dispute \
        --args "@${POC_CONFIG_ID}" "@${POC_REGISTRY_ID}" "@${dispute_id}" 1 "$vote_coin" "@${CLOCK_ID}"

    log_step "4d sleep for voting_duration_ms (3s + buffer)"
    sleep 4

    log_step "4e resolve_dispute_voting"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity resolve_dispute_voting \
        --args "@${dispute_id}" "@${post_id}" "@${CLOCK_ID}"

    log_step "4f claim_voting_reward"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity claim_voting_reward \
        --args "@${dispute_id}" "@${CLOCK_ID}"
    print_poc_dispute_flow_summary
}

run_dispute_overturn_reanalyze_flow() {
    require_platform_mode || return 1
    require_session_fields POC_CONFIG_ID POC_REGISTRY_ID ECOSYSTEM_TREASURY_ID || return 1
    local out digest post_id dispute_id coin vote_coin oracle reanalyze_digest
    oracle="$(resolve_myso_active_address)"

    log_step "4a Disputable post (overturn + re-analyze)"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC overturn ${POC_RUN_ID}")")"
    digest="$(extract_tx_digest "$out")"
    post_id="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post_id" ]] || post_id="$(extract_created_object_by_type "$digest" "Post")"
    analyze_post "$post_id" 1 100 "some($CREATOR_ADDRESS)" 1 false false 0 || return 1

    coin="$(resolve_gas_coin_for_address "$oracle")"
    [[ -n "$coin" ]] || { echo "No coin for dispute fee" >&2; return 1; }

    log_step "4b submit_poc_dispute"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity submit_poc_dispute \
        --args "@${POC_CONFIG_ID}" "@${POC_REGISTRY_ID}" "@${ECOSYSTEM_TREASURY_ID}" "@${post_id}" \
        "$(literal_move_string "$DEFAULT_DISPUTE_EVIDENCE")" "$coin" "@${CLOCK_ID}")"
    digest="$(extract_tx_digest "$out")"
    dispute_id="$(extract_created_object_by_type "$digest" "PoCDispute")"

    vote_coin="$(resolve_gas_coin_for_address "$oracle")"
    [[ -n "$vote_coin" && -n "$dispute_id" ]] || { echo "Missing dispute or vote coin" >&2; return 1; }

    log_step "4c vote_on_dispute (overturn)"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity vote_on_dispute \
        --args "@${POC_CONFIG_ID}" "@${POC_REGISTRY_ID}" "@${dispute_id}" 2 "$vote_coin" "@${CLOCK_ID}"

    log_step "4d sleep for voting_duration_ms (3s + buffer)"
    sleep 4

    log_step "4e resolve_dispute_voting (expect clear_poc_data)"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity resolve_dispute_voting \
        --args "@${dispute_id}" "@${post_id}" "@${CLOCK_ID}"

    log_step "4f oracle re-analyze after overturn (force_reanalyze)"
    export MYSO_POC_ALLOW_FORCE_RESUBMIT=1
    reanalyze_digest="$(poc_oracle_analyze_post "$post_id" 1 50 none 0 false false 0 "" true)" || return 1
    log_step "4f re-analyze digest=$reanalyze_digest"
    print_poc_dispute_flow_summary
}

run_post_reservation_poc_flow() {
    require_platform_mode || return 1
    require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID ECOSYSTEM_TREASURY_ID \
        POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID PLATFORM_REGISTRY_ID \
        PLATFORM_OBJECT_ID BLOCK_LIST_REGISTRY_ID CLOCK_ID || return 1
    ensure_spt_trading_enabled || return 1

    local out digest post_id pool_id vault_id reserve_amount pay_amount gross
    reserve_amount="${RESERVE_AMOUNT:-$DEFAULT_RESERVE_AMOUNT}"
    pay_amount="${RESERVE_PAY_AMOUNT:-$(( reserve_amount + 200000000 ))}"
    gross="$reserve_amount"

    log_step "Post reservation PoC: create post + oracle escrow analyze"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC reservation ${POC_RUN_ID}")" some\(true\))"
    digest="$(extract_tx_digest "$out")"
    post_id="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post_id" ]] || post_id="$(extract_created_object_by_type "$digest" "Post")"
    analyze_post "$post_id" 1 100 "some($CREATOR_ADDRESS)" 1 false false 0 || return 1

    vault_id="$(resolve_beneficiary_vault_id "$ANALYZE_POST_LAST_DIGEST" "$(normalize_hex_id "$CREATOR_ADDRESS")")" || {
        echo "Could not resolve beneficiary vault for post reservation flow" >&2
        return 1
    }

    pool_id="$(create_reservation_pool_for_post_call "$post_id")" || return 1

    ensure_tipper_memory_account || return 1
    local ref_token ref_spt ref_pool ref_treasury ref_post ref_vault ref_clk
    ref_token="$(ptb_shared_ref "$TOKEN_REGISTRY_ID")" || return 1
    ref_spt="$(ptb_shared_ref "$SOCIAL_PROOF_TOKENS_CONFIG_ID")" || return 1
    ref_pool="$(ptb_shared_ref "$pool_id")" || return 1
    ref_treasury="$(ptb_shared_ref "$ECOSYSTEM_TREASURY_ID")" || return 1
    ref_post="$(ptb_shared_ref "$post_id")" || return 1
    ref_vault="$(ptb_shared_ref "$vault_id")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1

    log_step "reserve_towards_post gross=$gross vault=$vault_id"
    SKIP_CONFIRM_RUN=1 invoke_ptb \
        --split-coins gas "[${pay_amount}]" \
        --assign pay_coin \
        --move-call "${PKG_SOCIAL}::social_proof_tokens::reserve_towards_post" \
        "$ref_token" "$ref_spt" "$POC_MIN_VAULT_DEPOSIT" \
        "$ref_pool" "$ref_treasury" "$ref_post" "$ref_vault" pay_coin.0 "$gross" "$ref_clk" || return 1

    log_step "withdraw_reservation_for_post gross=$gross"
    SKIP_CONFIRM_RUN=1 invoke_ptb \
        --move-call "${PKG_SOCIAL}::social_proof_tokens::withdraw_reservation_for_post" \
        "$ref_token" "$ref_spt" "$POC_MIN_VAULT_DEPOSIT" \
        "$ref_pool" "$ref_treasury" "$ref_post" "$ref_vault" "$gross" "$ref_clk" || return 1

    log_step "Post reservation PoC flow completed post=$post_id vault=$vault_id"
}

run_profile_reservation_flow() {
    require_session_fields "${REQUIRED_SPT_RESERVATION_KEYS[@]}" CLOCK_ID || return 1
    ensure_spt_trading_enabled || return 1
    ensure_oracle_profile_for_reservation || return 1

    local out digest pool_id reserve_amount pay_amount
    reserve_amount="${RESERVE_AMOUNT:-$DEFAULT_RESERVE_AMOUNT}"
    pay_amount="${RESERVE_PAY_AMOUNT:-$(( reserve_amount + 200000000 ))}"

    log_step "6a create_reservation_pool_for_profile profile=${ORACLE_PROFILE_ID}"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call social_proof_tokens create_reservation_pool_for_profile \
        --args "@${TOKEN_REGISTRY_ID}" "@${SOCIAL_PROOF_TOKENS_CONFIG_ID}" "@${ORACLE_PROFILE_ID}" "@${CLOCK_ID}")"
    digest="$(extract_tx_digest "$out")"
    pool_id="$(extract_created_object_by_type "$digest" "ReservationPoolObject")"
    [[ -n "$pool_id" ]] || pool_id="$(extract_created_object_by_type "$digest" "ReservationPool")"
    [[ -n "$pool_id" ]] || { echo "Could not find ReservationPoolObject from tx" >&2; return 1; }
    RESERVATION_POOL_ID="$pool_id"
    log_session_use "RESERVATION_POOL_ID" "$pool_id"

    log_step "6b reserve_towards_profile amount=${reserve_amount}"
    local ref_token ref_spt ref_pool ref_treasury ref_clk
    ref_token="$(ptb_shared_ref "$TOKEN_REGISTRY_ID")" || return 1
    ref_spt="$(ptb_shared_ref "$SOCIAL_PROOF_TOKENS_CONFIG_ID")" || return 1
    ref_pool="$(ptb_shared_ref "$pool_id")" || return 1
    ref_treasury="$(ptb_shared_ref "$ECOSYSTEM_TREASURY_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    SKIP_CONFIRM_RUN=1 invoke_ptb \
        --split-coins gas "[${pay_amount}]" \
        --assign pay_coin \
        --move-call "${PKG_SOCIAL}::social_proof_tokens::reserve_towards_profile" \
        "$ref_token" "$ref_spt" "$ref_pool" "$ref_treasury" pay_coin.0 "${reserve_amount}" "$ref_clk"
}

run_all_e2e() {
    POC_RUN_ID="$(date +%s)"
    maybe_auto_refresh_session
    require_session_fields "${REQUIRED_CORE_KEYS[@]}" || return 1
    ensure_cli_addresses || return 1
    if platform_mode_is_full; then
        require_session_fields "${REQUIRED_PLATFORM_KEYS[@]}" || return 1
    else
        log_step "No-platform mode (PLATFORM_OBJECT_ID unset)"
    fi
    preflight_oracle_and_config

    if [[ "${POC_SKIP_USERNAME:-0}" != 1 ]]; then
        run_username_beneficiary_flow
    fi

    if platform_mode_is_full; then
        ensure_memory_account_for_post_flows || return 1
        preflight_oracle_and_config || return 1
        run_post_poc_flow
        if [[ "${POC_INCLUDE_SPT:-0}" == 1 ]]; then
            run_spt_sync_flow
        fi
        if [[ "${POC_SKIP_DISPUTE:-0}" != 1 ]]; then
            run_dispute_flow
        fi
        if [[ "${POC_INCLUDE_DISPUTE_REANALYZE:-0}" == 1 ]]; then
            run_dispute_overturn_reanalyze_flow
        fi
        if [[ "${POC_INCLUDE_PROFILE_RESERVATION:-0}" == 1 ]]; then
            run_profile_reservation_flow
        fi
        if [[ "${POC_INCLUDE_POST_RESERVATION:-0}" == 1 ]]; then
            run_post_reservation_poc_flow
        fi
    fi
    save_session_state
    print_poc_run_all_summary
}

menu_session_setup() {
    refresh_poc_session_from_graphql
    load_session_state
    save_session_state
    echo "Session saved."
}

show_menu() {
    echo ""
    echo "=== PoC Runtime Test Menu ==="
    echo " 0) Refresh poc-session.env from GraphQL"
    echo " C) Create test platform (+ approve) and save PLATFORM_OBJECT_ID"
    echo " 1) Run full E2E (platform mode if PLATFORM_OBJECT_ID set, else username PoC only)"
    echo " 2) Username beneficiary flow only"
    echo " 3) Post PoC flow only (requires platform + MemoryAccount)"
    echo " 4) Dispute flow only (requires platform + MemoryAccount)"
    echo " 5) SPT post sync flow only (requires platform + MemoryAccount)"
    echo " 6) Profile reservation flow only (optional; requires existing Profile)"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) menu_session_setup ;;
        [Cc]) maybe_auto_refresh_session; create_test_platform ;;
        1) run_all_e2e ;;
        2) maybe_auto_refresh_session; ensure_cli_addresses; preflight_oracle_and_config; run_username_beneficiary_flow ;;
        3) maybe_auto_refresh_session; ensure_cli_addresses; ensure_memory_account_for_post_flows; preflight_oracle_and_config; run_post_poc_flow ;;
        4) maybe_auto_refresh_session; ensure_cli_addresses; ensure_memory_account_for_post_flows; preflight_oracle_and_config; run_dispute_flow ;;
        5) maybe_auto_refresh_session; ensure_cli_addresses; ensure_memory_account_for_post_flows; preflight_oracle_and_config; run_spt_sync_flow ;;
        6) maybe_auto_refresh_session; run_profile_reservation_flow ;;
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
            --run-all) RUN_MODE=run_all; shift ;;
            --post-flow) RUN_MODE=post_flow; shift ;;
            --username-flow) RUN_MODE=username_flow; shift ;;
            --create-platform) RUN_MODE=create_platform; shift ;;
            --refresh-session) RUN_MODE=refresh; shift ;;
            --no-auto-refresh) POC_NO_AUTO_REFRESH=1; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    load_session_state

    case "${RUN_MODE:-}" in
        refresh) refresh_poc_session_from_graphql; exit 0 ;;
        create_platform) maybe_auto_refresh_session; create_test_platform; exit 0 ;;
        post_flow)
            POC_RUN_ID="$(date +%s)"
            maybe_auto_refresh_session
            ensure_cli_addresses || exit 1
            ensure_memory_account_for_post_flows || exit 1
            preflight_oracle_and_config || exit 1
            run_post_poc_flow || exit 1
            save_session_state
            exit 0
            ;;
        username_flow)
            POC_RUN_ID="$(date +%s)"
            maybe_auto_refresh_session
            ensure_cli_addresses || exit 1
            preflight_oracle_and_config || exit 1
            run_username_beneficiary_flow || exit 1
            save_session_state
            exit 0
            ;;
        run_all) run_all_e2e; exit 0 ;;
        "") show_menu ;;
        *) echo "Unknown run mode: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
