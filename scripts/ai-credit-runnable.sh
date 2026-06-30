#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper for social_contracts::ai_credit + myso-ai-credit-oracle.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed; social-proof2 owns admin caps.
#   - Local chain RPC at http://127.0.0.1:9001
#   - Social indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Social-server at http://127.0.0.1:9126
#   - `myso`, `curl`, `jq`, `cargo` on PATH
#
# Session: network.config/ai-credit/ai-credit-session.env
#
# Usage:
#   ./scripts/ai-credit-runnable.sh --refresh-session
#   ./scripts/ai-credit-runnable.sh --start-oracle
#   ./scripts/ai-credit-runnable.sh --stop-oracle
#   ASSUME_YES=1 ./scripts/ai-credit-runnable.sh --run-all
#   ./scripts/ai-credit-runnable.sh --clean-receipts
#   ./scripts/ai-credit-runnable.sh   # interactive menu

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

readonly DEFAULT_PKG_SOCIAL='0x00000000000000000000000000000000000000000000000000000000000050c1'
readonly DEFAULT_CLOCK='0x0000000000000000000000000000000000000000000000000000000000000006'
readonly DEFAULT_GAS_BUDGET='1000000000'
readonly DEFAULT_ORACLE_ADDRESS='0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8'
readonly DEFAULT_ORACLE_PRIVATE_KEY_HEX='736c869f584b6fdf1d961541e515304cdbeaf8e3d7789ae79fd05e2d9da34578'
readonly DEFAULT_ORACLE_PUBKEY_HEX='cbe364936f8520e38004f3f970074fdcf1b87f0f25795fc3c74f2bae55737448'
readonly DEFAULT_AGENT_DERIVED_ADDRESS='0x00000000000000000000000000000000000000000000000000000000000a11ce'
readonly DEFAULT_AGENT_PUBKEY_HEX='0101010101010101010101010101010101010101010101010101010101010101'
readonly CAP_AI_SPEND='16384'
readonly DEPOSIT_MIST='5000000000'
readonly DEPOSIT_GAS_HEADROOM='100000000'
readonly CLASS_DELEGATED_AI='1'
readonly REGISTER_SCOPE='3'

GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
SOCIAL_SERVER_URL="${SOCIAL_SERVER_URL:-http://127.0.0.1:9126}"
ORACLE_URL="${ORACLE_URL:-http://127.0.0.1:8095}"
MYSO_RPC_URL="${MYSO_RPC_URL:-http://127.0.0.1:9001}"

PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
CLOCK_ID="$DEFAULT_CLOCK"
GAS_BUDGET=''

USERNAME_REGISTRY_ID=''
MEMORY_REGISTRY_ID=''
AI_CREDIT_CONFIG_ID=''
AI_CREDIT_ORACLE_ADMIN_CAP_ID=''

OWNER_ADDRESS="$DEFAULT_ORACLE_ADDRESS"
AI_CREDIT_BALANCE_ID=''
MEMORY_ACCOUNT_ID=''
ORG_ID=''
AGENT_OBJECT_ID=''
AGENT_DERIVED_ADDRESS="$DEFAULT_AGENT_DERIVED_ADDRESS"

AI_CREDIT_RUN_ID="$(date +%s)"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"
GQL_REFRESH_FILE=''

ORACLE_PID_FILE="$REPO_ROOT/network.config/ai-credit/oracle.pid"
ORACLE_ENV_FILE="$REPO_ROOT/network.config/ai-credit/oracle.env"
ORACLE_LOG_FILE="$REPO_ROOT/network.config/ai-credit/oracle.log"
RECEIPT_STORE="$REPO_ROOT/network.config/ai-credit/receipts.json"

AI_CREDIT_USAGE_SYNC_SECRET="${AI_CREDIT_USAGE_SYNC_SECRET:-local-dev-sync-secret}"
AI_CREDIT_SETTLEMENT_SECRET="${AI_CREDIT_SETTLEMENT_SECRET:-local-dev-settle-secret}"

usage() {
    sed -n '2,22p' "$0" | sed 's/^# \?//'
}

session_state_save_path() {
    printf '%s' "$REPO_ROOT/network.config/ai-credit/ai-credit-session.env"
}

apply_session_defaults() {
    [[ -n "${PKG_SOCIAL:-}" ]] || PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$DEFAULT_CLOCK"
    [[ -n "${GAS_BUDGET:-}" ]] || GAS_BUDGET="$DEFAULT_GAS_BUDGET"
    [[ -n "${OWNER_ADDRESS:-}" ]] || OWNER_ADDRESS="$DEFAULT_ORACLE_ADDRESS"
}

load_session_state() {
    local p
    p="$(session_state_save_path)"
    if [[ -f "$p" ]]; then
        # shellcheck disable=SC1090
        source "$p"
        echo "Loaded AI credit session from: $p" >&2
    fi
    apply_session_defaults
}

save_session_state() {
    local f key
    f="$(session_state_save_path)"
    mkdir -p "$(dirname "$f")"
    local old_umask
    old_umask="$(umask)"
    umask 077
    {
        echo "# AI credit runtime session — scripts/ai-credit-runnable.sh"
        for key in PKG_SOCIAL CLOCK_ID GAS_BUDGET USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID \
            AI_CREDIT_CONFIG_ID AI_CREDIT_ORACLE_ADMIN_CAP_ID OWNER_ADDRESS \
            AI_CREDIT_BALANCE_ID MEMORY_ACCOUNT_ID ORG_ID AGENT_OBJECT_ID AGENT_DERIVED_ADDRESS; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    umask "$old_umask"
    echo "Saved session to: $f" >&2
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

session_value_set() {
    local var_name="$1"
    [[ -n "${!var_name:-}" ]]
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

literal_move_string() {
    local s=$1
    s="${s//\\/\\\\}"
    s="${s//\"/\\\"}"
    s="${s//$'\n'/\\n}"
    s="${s//$'\r'/}"
    printf '"%s"' "$s"
}

move_vector_u8_from_hex() {
    python3 - "$1" <<'PY'
import sys
hex_str = sys.argv[1].removeprefix("0x")
data = bytes.fromhex(hex_str)
print("vector[" + ",".join(f"{b}u8" for b in data) + "]")
PY
}

# myso client call expects vector<u8> as a 0x-prefixed byte string, not vector[Nu8,...].
move_pure_hex_from_hex() {
    local hex="$1"
    hex="${hex#0x}"
    printf '0x%s' "$hex"
}

# Pure address literals in PTB must use @ prefix; bare 0x... is parsed as InferredNum.
move_address_from_hex() {
    local addr
    addr="$(normalize_hex_id "$1")" || return 1
    printf '@%s' "$addr"
}

extra_gas_budget() {
    printf '%s\n' '--gas-budget' "${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}"
}

extra_dry() {
    if [[ "${DRY_RUN:-0}" == 1 ]]; then
        printf '%s\n' '--dry-run'
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
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    if [[ -n "${PTB_GAS_COIN_ID:-}" ]]; then
        cmd+=(--gas-coin "@${PTB_GAS_COIN_ID#@}")
    fi
    cmd+=("$@")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        out="$("${cmd[@]}" 2>&1)" || {
            echo "$out" >&2
            if is_insufficient_coin_balance_error "$out"; then
                local min_mist
                min_mist="$(ptb_required_mist_from_args "$@")"
                request_faucet_for_address "$sender" "$min_mist" || return 1
                out="$("${cmd[@]}" 2>&1)" || { echo "$out" >&2; return 1; }
            else
                return 1
            fi
        }
        echo "$out" >&2
        printf '%s' "$out"
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
    local arg out
    while IFS= read -r -d '' arg; do call_args+=("$arg"); done < <(normalize_client_call_args "$@")
    cmd=(myso client call --package "$PKG_SOCIAL" --module "$module" --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=(--args)
    cmd+=("${call_args[@]}")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        out="$("${cmd[@]}" 2>&1)" || {
            echo "$out" >&2
            if is_insufficient_coin_balance_error "$out"; then
                ensure_oracle_owner_address || return 1
                request_faucet_for_address "$OWNER_ADDRESS" "$(( DEFAULT_GAS_BUDGET * 2 ))" || return 1
                out="$("${cmd[@]}" 2>&1)" || { echo "$out" >&2; return 1; }
            else
                return 1
            fi
        }
        echo "$out" >&2
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

readonly GQL_AI_CREDIT_BATCH='query AiCreditSessionObjects {
  usernameRegistry: objects(filter: { type: "0x50c1::profile::UsernameRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  memoryRegistry: objects(filter: { type: "0x50c1::memory::MemoryRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  aiCreditConfig: objects(filter: { type: "0x50c1::ai_credit::AiCreditConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  aiCreditOracleAdminCap: objects(filter: { type: "0x50c1::ai_credit::AiCreditOracleAdminCap" }, last: 1) { nodes { address } }
}'

gql_set_refresh() {
    local key="$1" val="$2"
    [[ -n "$val" ]] || return 0
    printf '%s=%q\n' "$key" "$val" >> "$GQL_REFRESH_FILE"
}

collect_gql_mappings() {
    local json="$1" alias val env_key
    for alias in usernameRegistry memoryRegistry aiCreditConfig aiCreditOracleAdminCap; do
        case "$alias" in
            usernameRegistry) env_key=USERNAME_REGISTRY_ID ;;
            memoryRegistry) env_key=MEMORY_REGISTRY_ID ;;
            aiCreditConfig) env_key=AI_CREDIT_CONFIG_ID ;;
            aiCreditOracleAdminCap) env_key=AI_CREDIT_ORACLE_ADMIN_CAP_ID ;;
            *) continue ;;
        esac
        val="$(gql_object_address "$json" "$alias")"
        gql_set_refresh "$env_key" "$val"
    done
}

apply_gql_refresh_file() {
    [[ -f "$GQL_REFRESH_FILE" ]] || return 0
    # shellcheck disable=SC1090
    source "$GQL_REFRESH_FILE"
    PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
    CLOCK_ID="$DEFAULT_CLOCK"
}

refresh_ai_credit_session_from_graphql() {
    command -v curl >/dev/null 2>&1 || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }

    log_step "Refreshing ai-credit session from GraphQL ($GRAPHQL_URL)"
    GQL_REFRESH_FILE="$(mktemp)"
    local json
    json="$(graphql_post "$GQL_AI_CREDIT_BATCH")"
    collect_gql_mappings "$json"

    local f
    f="$(session_state_save_path)"
    mkdir -p "$(dirname "$f")"
    {
        echo "# AI credit runtime session — scripts/ai-credit-runnable.sh"
        echo "# Refreshed from GraphQL $(date -u +%Y-%m-%dT%H:%M:%SZ)"
        cat "$GQL_REFRESH_FILE"
        printf '%s=%q\n' PKG_SOCIAL "$DEFAULT_PKG_SOCIAL"
        printf '%s=%q\n' CLOCK_ID "$DEFAULT_CLOCK"
        printf '%s=%q\n' GAS_BUDGET "${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}"
        printf '%s=%q\n' OWNER_ADDRESS "$DEFAULT_ORACLE_ADDRESS"
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    rm -f "$GQL_REFRESH_FILE"
    load_session_state
    log_session_use "USERNAME_REGISTRY_ID" "$USERNAME_REGISTRY_ID"
    log_session_use "MEMORY_REGISTRY_ID" "$MEMORY_REGISTRY_ID"
    log_session_use "AI_CREDIT_CONFIG_ID" "$AI_CREDIT_CONFIG_ID"
    log_session_use "AI_CREDIT_ORACLE_ADMIN_CAP_ID" "$AI_CREDIT_ORACLE_ADMIN_CAP_ID"
}

resolve_myso_active_address() {
    myso client active-address 2>/dev/null
}

ensure_oracle_owner_address() {
    local active owner
    active="$(resolve_myso_active_address)" || {
        echo "Could not read myso client active-address" >&2
        return 1
    }
    owner="$(normalize_hex_id "$DEFAULT_ORACLE_ADDRESS")"
    if [[ "$(normalize_hex_id "$active")" != "$owner" ]]; then
        log_step "Switching active address to social-proof2 ($owner)"
        myso client switch --address "$owner" >/dev/null
    fi
    OWNER_ADDRESS="$owner"
    log_session_use "OWNER_ADDRESS" "$OWNER_ADDRESS"
}

resolve_max_gas_coin_balance() {
    local addr="$1"
    resolve_gas_coins_json_for_address "$addr" \
        | jq '[.[].mistBalance | tonumber] | max // 0' 2>/dev/null
}

wait_for_gas_coin_balance_at_least() {
    local addr="$1" min_mist="$2" attempt max_bal
    addr="$(normalize_hex_id "$addr")" || return 1
    for attempt in $(seq 1 60); do
        max_bal="$(resolve_max_gas_coin_balance "$addr")"
        [[ -n "$max_bal" && "$max_bal" -ge "$min_mist" ]] && return 0
        sleep 1
    done
    echo "No gas coin with balance >= $min_mist for $addr (max=${max_bal:-0}) after 60s" >&2
    return 1
}

ptb_required_mist_from_args() {
    local arg
    for arg in "$@"; do
        if [[ "$arg" =~ ^\[([0-9]+)\]$ ]]; then
            echo $(( ${BASH_REMATCH[1]} + DEPOSIT_GAS_HEADROOM ))
            return 0
        fi
    done
    echo $(( DEFAULT_GAS_BUDGET * 2 ))
}

is_insufficient_coin_balance_error() {
    echo "$1" | grep -qiE 'Insufficient coin balance|insufficient.*coin'
}

request_faucet_for_address() {
    local addr="$1" min_balance="${2:-0}" attempt before after n
    addr="$(normalize_hex_id "$addr")" || return 1
    before="$(resolve_max_gas_coin_balance "$addr")"
    [[ -n "$before" ]] || before=0
    log_step "Requesting faucet tokens for $addr (2x ~5 MYSO each)"
    for n in 1 2; do
        myso client faucet --address "$addr" >/dev/null 2>&1 \
            || myso client faucet --address "$addr" >&2 \
            || return 1
        [[ "$n" -lt 2 ]] && sleep 2
    done
    if [[ "$min_balance" -gt 0 ]]; then
        wait_for_gas_coin_balance_at_least "$addr" "$min_balance" || return 1
        return 0
    fi
    for attempt in $(seq 1 60); do
        after="$(resolve_max_gas_coin_balance "$addr")"
        [[ -n "$after" && "$after" -gt "$before" ]] && return 0
        sleep 1
    done
    echo "Gas balance for $addr did not increase after faucet (before=$before after=${after:-0})" >&2
    return 1
}

ensure_owner_funded() {
    local coin attempt
    ensure_oracle_owner_address || return 1
    coin="$(resolve_gas_coin_for_address "$OWNER_ADDRESS")"
    if [[ -n "$coin" ]]; then
        return 0
    fi
    request_faucet_for_address "$OWNER_ADDRESS" || return 1
}

resolve_gas_coins_json_for_address() {
    local addr="$1"
    addr="$(normalize_hex_id "$addr")" || return 1
    myso client gas "$addr" --json 2>/dev/null
}

resolve_gas_coin_for_address() {
    local addr="$1" json
    json="$(resolve_gas_coins_json_for_address "$addr")" || return 1
    echo "$json" | jq -r '.[0].gasCoinId // .[0].coinObjectId // empty' | head -n1
}

pick_deposit_and_gas_coins_for_address() {
    local addr="$1" split_amount="$2"
    local json deposit_coin gas_coin min_gas=10000000
    addr="$(normalize_hex_id "$addr")" || return 1
    json="$(resolve_gas_coins_json_for_address "$addr")" || return 1
    deposit_coin="$(echo "$json" | jq -r --argjson amt "$split_amount" '
        [.[] | select((.mistBalance | tonumber) >= $amt)]
        | sort_by(-(.mistBalance | tonumber))
        | (.[0].gasCoinId // .[0].coinObjectId // empty)
    ')"
    [[ -n "$deposit_coin" ]] || {
        echo "No coin with balance >= $split_amount for $addr" >&2
        return 1
    }
    gas_coin="$(echo "$json" | jq -r --arg dep "$deposit_coin" --argjson mingas "$min_gas" '
        [.[] | select(
            ((.gasCoinId // .coinObjectId) != $dep)
            and ((.mistBalance | tonumber) >= $mingas)
        )]
        | (.[0].gasCoinId // .[0].coinObjectId // empty)
    ')"
    [[ -n "$gas_coin" ]] || {
        echo "Need a second gas coin (distinct from deposit coin) for $addr" >&2
        return 1
    }
    printf '%s %s' "$deposit_coin" "$gas_coin"
}

ensure_deposit_funding() {
    local addr="$1" split_amount="$2" attempt
    addr="$(normalize_hex_id "$addr")" || return 1
    if pick_deposit_and_gas_coins_for_address "$addr" "$split_amount" >/dev/null 2>&1; then
        return 0
    fi
    request_faucet_for_address "$addr" "$split_amount" || return 1
    for attempt in $(seq 1 30); do
        if pick_deposit_and_gas_coins_for_address "$addr" "$split_amount" >/dev/null 2>&1; then
            return 0
        fi
        sleep 1
    done
    echo "Could not obtain distinct deposit and gas coins for $addr" >&2
    return 1
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
    [[ -n "$digest" ]] && { printf '%s' "$digest"; return 0; }
    digest="$(echo "$out" | grep -Eo '"transaction_digest"[[:space:]]*:[[:space:]]*"[^"]+"' \
        | tail -n1 | sed -E 's/.*"([^"]+)"$/\1/')"
    [[ -n "$digest" ]] && printf '%s' "$digest"
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
        | select(. != null and . != "")
    ' | head -n1)"
    [[ -n "$result" ]] || return 1
    printf '%s' "$result"
}

gql_ai_credit_snapshot() {
    local addr="$1" resp vars
    addr="$(normalize_hex_id "$addr")" || return 1
    vars="$(jq -nc --arg addr "$addr" '{addr: $addr}')"
    resp="$(graphql_post \
        'query AiCreditProfile($addr: MySoAddress!) {
            profile(address: $addr) {
                profileId
                memoryAccountId
                aiCreditBalance {
                    balanceId
                    balanceMist
                    credits
                    spentTotalMist
                    settlementNonce
                    active
                }
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

wait_for_gql_ai_credit() {
    local addr="$1" field="$2" min_val="$3" attempt resp val
    addr="$(normalize_hex_id "$addr")" || return 1
    for attempt in $(seq 1 60); do
        resp="$(gql_ai_credit_snapshot "$addr")" || true
        val="$(echo "$resp" | jq -r ".data.profile.aiCreditBalance.${field} // empty" 2>/dev/null || true)"
        if [[ -n "$val" && "$val" -ge "$min_val" ]]; then
            echo "$resp" >&2
            return 0
        fi
        [[ "$attempt" -lt 60 ]] || break
        sleep 1
    done
    echo "Timed out waiting for profile.aiCreditBalance.${field} >= ${min_val} (last: ${val:-<none>})" >&2
    echo "$resp" | jq '.' >&2 || true
    return 1
}

wait_for_gql_settlement_nonce_at_least() {
    local addr="$1" min_nonce="$2" attempt resp val
    addr="$(normalize_hex_id "$addr")" || return 1
    for attempt in $(seq 1 90); do
        resp="$(gql_ai_credit_snapshot "$addr")" || true
        val="$(echo "$resp" | jq -r '.data.profile.aiCreditBalance.settlementNonce // empty' 2>/dev/null || true)"
        if [[ -n "$val" && "$val" -ge "$min_nonce" ]]; then
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "Timed out waiting for GraphQL settlementNonce >= $min_nonce (last: ${val:-<none>})" >&2
    return 1
}

# On-chain AiCreditBalance BCS layout (fixed fields before agent_budgets table).
rpc_ai_credit_balance_fields() {
    local balance_id="$1"
    balance_id="$(normalize_hex_id "$balance_id")" || return 1
    python3 - "$balance_id" <<'PY'
import json, struct, subprocess, sys

balance_id = sys.argv[1]
raw = subprocess.check_output(
    ["myso", "client", "object", balance_id, "--json"],
    stderr=subprocess.DEVNULL,
    text=True,
)
obj = json.loads(raw)
contents = obj.get("data", {}).get("Move", {}).get("contents")
if not contents:
    sys.exit(1)
data = bytes(contents)

def read_u64(data, off):
    return struct.unpack_from("<Q", data, off)[0]

def read_option_u64(data, off):
    tag = data[off]
    if tag == 0:
        return None, off + 1
    return struct.unpack_from("<Q", data, off + 1)[0], off + 9

off = 32 + 32 + 32 + 32 + 8  # uid, ids, Balance<MYSO>.value
spent_total_mist = read_u64(data, off); off += 8
off += 8 + 8 + 8 + 8  # spent_day, spent_month, day_anchor, month_anchor
_, off = read_option_u64(data, off)  # daily_cap_mist
_, off = read_option_u64(data, off)  # monthly_cap_mist
settlement_nonce = read_u64(data, off)
print(f"{settlement_nonce} {spent_total_mist}")
PY
}

rpc_ai_credit_settlement_nonce() {
    local balance_id="$1" fields
    fields="$(rpc_ai_credit_balance_fields "$balance_id")" || return 1
    echo "$fields" | awk '{print $1}'
}

rpc_ai_credit_spent_total_mist() {
    local balance_id="$1" fields
    fields="$(rpc_ai_credit_balance_fields "$balance_id")" || return 1
    echo "$fields" | awk '{print $2}'
}

wait_for_on_chain_settlement() {
    local balance_id="$1" min_nonce="$2" min_spent="$3" attempt nonce spent
    balance_id="$(normalize_hex_id "$balance_id")" || return 1
    for attempt in $(seq 1 90); do
        nonce="$(rpc_ai_credit_settlement_nonce "$balance_id" 2>/dev/null || true)"
        spent="$(rpc_ai_credit_spent_total_mist "$balance_id" 2>/dev/null || true)"
        if [[ -n "$nonce" && -n "$spent" && "$nonce" -ge "$min_nonce" && "$spent" -ge "$min_spent" ]]; then
            echo "On-chain settlement confirmed (settlement_nonce=$nonce spent_total_mist=$spent)" >&2
            return 0
        fi
        sleep 1
    done
    echo "Timed out waiting for on-chain settlement (need nonce>=$min_nonce spent>=$min_spent; last nonce=${nonce:-<none>} spent=${spent:-<none>})" >&2
    return 1
}

oracle_receipts_settled_for_balance() {
    local balance_id="$1" count
    balance_id="$(normalize_hex_id "$balance_id")" || return 1
    [[ -f "$RECEIPT_STORE" ]] || return 1
    count="$(jq -r --arg bal "$balance_id" '
        [.lines[] | select(.balance_id == $bal and .settled == true)] | length
    ' "$RECEIPT_STORE" 2>/dev/null || echo 0)"
    [[ "$count" -gt 0 ]]
}

rest_get_profile_ai_credit() {
    local addr="$1"
    addr="$(normalize_hex_id "$addr")" || return 1
    curl -sS "${SOCIAL_SERVER_URL}/profiles/${addr}/ai-credit"
}

rest_get_usage_history() {
    local balance_id="$1"
    balance_id="$(normalize_hex_id "$balance_id")" || return 1
    curl -sS "${SOCIAL_SERVER_URL}/ai-credit/${balance_id}/usage-history"
}

oracle_get_usage_history() {
    local balance_id="$1"
    balance_id="$(normalize_hex_id "$balance_id")" || return 1
    curl -sS "${ORACLE_URL}/v1/ai-credit/usage-history?balance_id=${balance_id}"
}

usage_history_count() {
    local resp="$1"
    echo "$resp" | jq 'if type == "array" then length elif .items? then (.items | length) else 0 end' 2>/dev/null || echo 0
}

rest_get_ai_credit_config() {
    curl -sS "${SOCIAL_SERVER_URL}/ai-credit/config"
}

assert_rest_balance_field() {
    local addr="$1" field="$2" min_val="$3" attempt resp val
    for attempt in $(seq 1 60); do
        resp="$(rest_get_profile_ai_credit "$addr")" || true
        val="$(echo "$resp" | jq -r ".balance.${field} // empty" 2>/dev/null || true)"
        if [[ -n "$val" && "$val" -ge "$min_val" ]]; then
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "REST balance.${field} expected >= ${min_val}, last ${val:-<none>}" >&2
    echo "$resp" | jq '.' >&2 || true
    return 1
}

assert_usage_history_nonempty() {
    local balance_id="$1" attempt resp count
    balance_id="$(normalize_hex_id "$balance_id")" || return 1
    for attempt in $(seq 1 60); do
        resp="$(rest_get_usage_history "$balance_id" 2>/dev/null)" || resp='[]'
        count="$(usage_history_count "$resp")"
        if [[ "$count" -gt 0 ]]; then
            echo "$resp" >&2
            return 0
        fi
        resp="$(oracle_get_usage_history "$balance_id" 2>/dev/null)" || resp='[]'
        count="$(usage_history_count "$resp")"
        if [[ "$count" -gt 0 ]]; then
            log_step "Usage history found via oracle (social-server ingest may be read-only)"
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "No usage-history rows for balance $balance_id (checked social-server and oracle)" >&2
    return 1
}

usage_history_response_settled() {
    local resp="$1" receipt_id="${2:-}"
    if [[ -n "$receipt_id" ]]; then
        echo "$resp" | jq -e --arg rid "$receipt_id" '
            (. | if type == "array" then . elif .items? then .items else [] end)
            | [.[] | select(.receipt_id == $rid)]
            | length == 1
            and all(.[]; .settled == true and ((.settlement_tx // "") | length) > 0)
        ' >/dev/null 2>&1
    else
        echo "$resp" | jq -e '
            (. | if type == "array" then . elif .items? then .items else [] end)
            | length > 0
            and all(.[]; .settled == true and ((.settlement_tx // "") | length) > 0)
        ' >/dev/null 2>&1
    fi
}

assert_usage_history_settled() {
    local balance_id="$1" receipt_id="${2:-}" attempt resp
    balance_id="$(normalize_hex_id "$balance_id")" || return 1
    for attempt in $(seq 1 90); do
        resp="$(rest_get_usage_history "$balance_id" 2>/dev/null)" || resp='[]'
        if usage_history_response_settled "$resp" "$receipt_id"; then
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "Timed out waiting for REST usage-history settled=true (balance=$balance_id receipt_id=${receipt_id:-all})" >&2
    echo "$resp" | jq '.' >&2 || echo "$resp" >&2
    return 1
}

gql_ai_credit_usage_history() {
    local addr="$1" resp vars
    addr="$(normalize_hex_id "$addr")" || return 1
    vars="$(jq -nc --arg addr "$addr" '{addr: $addr}')"
    resp="$(graphql_post \
        'query AiCreditUsageHistory($addr: MySoAddress!) {
            profile(address: $addr) {
                aiCreditBalance {
                    balanceId
                    usageHistory(first: 20) {
                        receiptId
                        settled
                        settlementTx
                        amountMist
                    }
                }
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

gql_usage_history_response_settled() {
    local resp="$1" receipt_id="${2:-}"
    if [[ -n "$receipt_id" ]]; then
        echo "$resp" | jq -e --arg rid "$receipt_id" '
            (.data.profile.aiCreditBalance.usageHistory // [])
            | [.[] | select(.receiptId == $rid)]
            | length == 1
            and all(.[]; .settled == true and ((.settlementTx // "") | length) > 0)
        ' >/dev/null 2>&1
    else
        echo "$resp" | jq -e '
            (.data.profile.aiCreditBalance.usageHistory // [])
            | length > 0
            and all(.[]; .settled == true and ((.settlementTx // "") | length) > 0)
        ' >/dev/null 2>&1
    fi
}

assert_gql_usage_history_settled() {
    local addr="$1" receipt_id="${2:-}" attempt resp
    addr="$(normalize_hex_id "$addr")" || return 1
    for attempt in $(seq 1 90); do
        resp="$(gql_ai_credit_usage_history "$addr" 2>/dev/null)" || resp='{}'
        if gql_usage_history_response_settled "$resp" "$receipt_id"; then
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "Timed out waiting for GraphQL usageHistory settled=true (addr=$addr receipt_id=${receipt_id:-all})" >&2
    echo "$resp" | jq '.' >&2 || echo "$resp" >&2
    return 1
}

verify_settlement_complete() {
    local balance_id="$1" owner="$2" nonce_before="$3" spent_before="$4" receipt_id="$5"
    local spent_after nonce_after

    [[ -n "$receipt_id" && "$receipt_id" != "null" ]] || {
        echo "Missing receipt_id from oracle record_usage response" >&2
        return 1
    }

    log_step "Waiting for on-chain settlement (RPC settlement_nonce > $nonce_before)"
    wait_for_on_chain_settlement "$balance_id" $((nonce_before + 1)) $((spent_before + 1))

    log_step "Waiting for indexer/GraphQL settlement (settlementNonce > $nonce_before)"
    wait_for_gql_settlement_nonce_at_least "$owner" $((nonce_before + 1))

    log_step "Waiting for usage-history settled=true in REST"
    assert_usage_history_settled "$balance_id" "$receipt_id"

    log_step "Waiting for usage-history settled=true in GraphQL"
    assert_gql_usage_history_settled "$owner" "$receipt_id"

    spent_after="$(gql_ai_credit_snapshot "$owner" | jq -r '.data.profile.aiCreditBalance.spentTotalMist // 0')"
    nonce_after="$(gql_ai_credit_snapshot "$owner" | jq -r '.data.profile.aiCreditBalance.settlementNonce // 0')"
    if [[ "$spent_after" -le "$spent_before" ]]; then
        echo "Expected GraphQL spentTotalMist to increase (before=$spent_before after=$spent_after nonce=$nonce_after)" >&2
        return 1
    fi
    log_step "Settlement confirmed on-chain and indexed (nonce $nonce_before -> $nonce_after, spent $spent_before -> $spent_after MIST)"
}

oracle_health_ok() {
    curl -sf "${ORACLE_URL}/health" >/dev/null 2>&1
}

write_oracle_env_stamp() {
    mkdir -p "$(dirname "$ORACLE_ENV_FILE")"
    {
        printf 'SOCIAL_SERVER_URL=%q\n' "$SOCIAL_SERVER_URL"
        printf 'ORACLE_URL=%q\n' "$ORACLE_URL"
        printf 'MYSO_RPC_URL=%q\n' "$MYSO_RPC_URL"
        printf 'AI_CREDIT_CONFIG_ID=%q\n' "$(normalize_hex_id "$AI_CREDIT_CONFIG_ID")"
        printf 'AI_CREDIT_SETTLEMENT_KEY_HEX=%q\n' "00${DEFAULT_ORACLE_PRIVATE_KEY_HEX}"
        printf 'AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS=%q\n' "${AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS:-86400}"
    } >"$ORACLE_ENV_FILE"
}

oracle_env_matches_current() {
    [[ -f "$ORACLE_ENV_FILE" ]] || return 1
    # shellcheck disable=SC1090
    source "$ORACLE_ENV_FILE"
    [[ "${SOCIAL_SERVER_URL:-}" == "$SOCIAL_SERVER_URL" ]] \
        && [[ "${ORACLE_URL:-}" == "$ORACLE_URL" ]] \
        && [[ "${MYSO_RPC_URL:-}" == "$MYSO_RPC_URL" ]] \
        && [[ "${AI_CREDIT_CONFIG_ID:-}" == "$(normalize_hex_id "$AI_CREDIT_CONFIG_ID")" ]] \
        && [[ "${AI_CREDIT_SETTLEMENT_KEY_HEX:-}" == "00${DEFAULT_ORACLE_PRIVATE_KEY_HEX}" ]] \
        && [[ "${AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS:-86400}" == "${AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS:-86400}" ]]
}

start_oracle_background() {
    require_session_fields AI_CREDIT_CONFIG_ID || return 1
    mkdir -p "$(dirname "$ORACLE_PID_FILE")"
    if [[ -f "$ORACLE_PID_FILE" ]]; then
        local old_pid
        old_pid="$(cat "$ORACLE_PID_FILE")"
        if kill -0 "$old_pid" 2>/dev/null; then
            if oracle_health_ok && oracle_env_matches_current; then
                log_step "Oracle already running (pid $old_pid)"
                return 0
            fi
            if oracle_health_ok && ! oracle_env_matches_current; then
                log_step "Restarting oracle (runtime config changed, e.g. SOCIAL_SERVER_URL=$SOCIAL_SERVER_URL)"
                stop_oracle_background
            fi
        else
            rm -f "$ORACLE_PID_FILE"
        fi
    fi
    if [[ "${CLEAN_RECEIPTS:-0}" == 1 || "${RUN_MODE:-}" == clean_receipts ]]; then
        rm -f "$RECEIPT_STORE"
    fi
    log_step "Starting myso-ai-credit-oracle on ${ORACLE_URL}"
    local oracle_bin="$REPO_ROOT/target/debug/myso-ai-credit-oracle"
    (
        cd "$REPO_ROOT"
        export AI_CREDIT_ORACLE_PRIVATE_KEY_HEX="$DEFAULT_ORACLE_PRIVATE_KEY_HEX"
        export AI_CREDIT_SETTLEMENT_KEY_HEX="00${DEFAULT_ORACLE_PRIVATE_KEY_HEX}"
        export AI_CREDIT_CONFIG_OBJECT_ID="$(normalize_hex_id "$AI_CREDIT_CONFIG_ID")"
        export AI_CREDIT_MYSO_RPC="$MYSO_RPC_URL"
        export AI_CREDIT_SOCIAL_SERVER_URL="$SOCIAL_SERVER_URL"
        export AI_CREDIT_USAGE_SYNC_SECRET="$AI_CREDIT_USAGE_SYNC_SECRET"
        export AI_CREDIT_SETTLEMENT_SECRET="$AI_CREDIT_SETTLEMENT_SECRET"
        export AI_CREDIT_SETTLE_THRESHOLD_MIST=1
        export AI_CREDIT_SETTLE_MIN_COUNT=1
        export AI_CREDIT_SETTLE_MAX_AGE_SECS=2
        export AI_CREDIT_SETTLEMENT_INTERVAL_SECS=5
        export AI_CREDIT_RECEIPT_STORE="$RECEIPT_STORE"
        export AI_CREDIT_STRICT_CATALOG=false
        export AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS="${AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS:-86400}"
        export RUST_LOG="${RUST_LOG:-info,myso_ai_credit_oracle=debug}"
        if [[ -x "$oracle_bin" ]]; then
            exec "$oracle_bin"
        fi
        exec cargo run -q -p myso-ai-credit-oracle --
    ) >>"$ORACLE_LOG_FILE" 2>&1 &
    echo $! >"$ORACLE_PID_FILE"
    local attempt
    for attempt in $(seq 1 120); do
        if oracle_health_ok; then
            write_oracle_env_stamp
            log_step "Oracle healthy (pid $(cat "$ORACLE_PID_FILE"), log: $ORACLE_LOG_FILE)"
            return 0
        fi
        sleep 1
    done
    echo "Oracle failed to become healthy; see $ORACLE_LOG_FILE" >&2
    tail -n 40 "$ORACLE_LOG_FILE" >&2 || true
    return 1
}

stop_oracle_background() {
    if [[ ! -f "$ORACLE_PID_FILE" ]]; then
        log_step "No oracle pid file"
        return 0
    fi
    local pid
    pid="$(cat "$ORACLE_PID_FILE")"
    if kill -0 "$pid" 2>/dev/null; then
        log_step "Stopping oracle pid $pid"
        kill "$pid" 2>/dev/null || true
        wait "$pid" 2>/dev/null || true
    fi
    rm -f "$ORACLE_PID_FILE" "$ORACLE_ENV_FILE"
}

oracle_trigger_settle() {
    curl -sS -X POST "${ORACLE_URL}/internal/ai-credit/settle" \
        -H "x-ai-credit-settlement-secret: ${AI_CREDIT_SETTLEMENT_SECRET}"
}

oracle_preflight() {
    local owner="$1" agent_id="$2"
    curl -sS -X POST "${ORACLE_URL}/v1/ai-credit/preflight" \
        -H 'Content-Type: application/json' \
        -d "$(jq -nc \
            --arg owner "$(normalize_hex_id "$owner")" \
            --arg agent "$agent_id" \
            '{
                owner: $owner,
                agent_object_id: $agent,
                operation: "inference",
                model_id: "openai/gpt-4o-mini",
                estimated_tokens_in: 2000,
                estimated_tokens_out: 500
            }')"
}

oracle_record_usage() {
    local owner="$1" balance_id="$2" memory_id="$3" agent_id="$4"
    curl -sS -X POST "${ORACLE_URL}/v1/ai-credit/usage" \
        -H 'Content-Type: application/json' \
        -d "$(jq -nc \
            --arg owner "$(normalize_hex_id "$owner")" \
            --arg balance "$(normalize_hex_id "$balance_id")" \
            --arg memory "$(normalize_hex_id "$memory_id")" \
            --arg agent "$agent_id" \
            '{
                owner: $owner,
                balance_id: $balance,
                memory_account_id: $memory,
                agent_object_id: $agent,
                usage_kind: 1,
                model_id: "openai/gpt-4o-mini",
                tokens_in: 2000,
                tokens_out: 500
            }')"
}

update_oracle_pubkey_on_chain() {
    local pk_vec cap_id cfg_ref
    require_session_fields AI_CREDIT_ORACLE_ADMIN_CAP_ID AI_CREDIT_CONFIG_ID || return 1
    require_hex_ids AI_CREDIT_ORACLE_ADMIN_CAP_ID AI_CREDIT_CONFIG_ID || return 1
    ensure_oracle_owner_address || return 1
    pk_vec="$(move_pure_hex_from_hex "$DEFAULT_ORACLE_PUBKEY_HEX")"
    cap_id="$(normalize_hex_id "$AI_CREDIT_ORACLE_ADMIN_CAP_ID")"
    cfg_ref="$(ptb_shared_ref "$AI_CREDIT_CONFIG_ID")"
    log_step "Updating on-chain oracle pubkey (update_oracle_pubkey)"
    SKIP_CONFIRM_RUN=1 run_myso_call ai_credit update_oracle_pubkey \
        "$cap_id" "$cfg_ref" "$pk_vec"
}

address_has_profile() {
    local addr="$1" resp
    resp="$(gql_ai_credit_snapshot "$addr")" || return 1
    echo "$resp" | jq -e '.data.profile.profileId != null' >/dev/null 2>&1
}

create_profile_with_ai_credit() {
    local sender="$1" out digest mem balance profile username
    sender="$(normalize_hex_id "$sender")" || return 1
    require_session_fields USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID AI_CREDIT_CONFIG_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID AI_CREDIT_CONFIG_ID CLOCK_ID || return 1
    username="aic${AI_CREDIT_RUN_ID}${RANDOM}"
    log_step "Creating profile + AiCreditBalance for $sender (username=$username)"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" \
        --move-call "${PKG_SOCIAL}::profile::create_profile" \
        "$(ptb_shared_ref "$USERNAME_REGISTRY_ID")" \
        "$(ptb_shared_ref "$MEMORY_REGISTRY_ID")" \
        "$(ptb_shared_ref "$AI_CREDIT_CONFIG_ID")" \
        "$(literal_move_string "AI Credit E2E")" \
        "$(literal_move_string "$username")" \
        "$(literal_move_string "ai credit runnable")" \
        'vector[]' 'vector[]' \
        "$(ptb_shared_ref "$CLOCK_ID")")" || return 1
    digest="$(extract_tx_digest "$out")"
    mem="$(extract_created_object_by_type "$digest" "memory::MemoryAccount")"
    [[ -n "$mem" ]] || mem="$(extract_created_object_by_type "$digest" "MemoryAccount")"
    balance="$(extract_created_object_by_type "$digest" "ai_credit::AiCreditBalance")"
    [[ -n "$balance" ]] || balance="$(extract_created_object_by_type "$digest" "AiCreditBalance")"
    profile="$(extract_created_object_by_type "$digest" "profile::Profile")"
    [[ -n "$mem" && -n "$balance" ]] || {
        echo "create_profile missing MemoryAccount or AiCreditBalance in tx $digest" >&2
        return 1
    }
    MEMORY_ACCOUNT_ID="$(normalize_hex_id "$mem")"
    AI_CREDIT_BALANCE_ID="$(normalize_hex_id "$balance")"
    log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
    log_session_use "AI_CREDIT_BALANCE_ID" "$AI_CREDIT_BALANCE_ID"
    [[ -n "$profile" ]] && log_session_use "profile_id" "$(normalize_hex_id "$profile")"
    save_session_state
    printf '%s' "$digest"
}

ensure_profile_and_balance() {
    local resp
    ensure_oracle_owner_address || return 1
    if address_has_profile "$OWNER_ADDRESS"; then
        log_step "Profile already exists for $OWNER_ADDRESS — loading ids from GraphQL"
        resp="$(gql_ai_credit_snapshot "$OWNER_ADDRESS")"
        MEMORY_ACCOUNT_ID="$(echo "$resp" | jq -r '.data.profile.memoryAccountId // empty')"
        AI_CREDIT_BALANCE_ID="$(echo "$resp" | jq -r '.data.profile.aiCreditBalance.balanceId // empty')"
        [[ -n "$MEMORY_ACCOUNT_ID" && -n "$AI_CREDIT_BALANCE_ID" ]] || {
            echo "Profile exists but missing memoryAccountId or aiCreditBalance in GraphQL" >&2
            return 1
        }
        log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
        log_session_use "AI_CREDIT_BALANCE_ID" "$AI_CREDIT_BALANCE_ID"
        save_session_state
        return 0
    fi
    create_profile_with_ai_credit "$OWNER_ADDRESS" >/dev/null
    wait_for_gql_ai_credit "$OWNER_ADDRESS" balanceMist 0
}

create_agentic_organization_if_needed() {
    local out digest org_ref mem_ref clk_ref
    if session_value_set ORG_ID; then
        log_session_use "ORG_ID (existing)" "$ORG_ID"
        return 0
    fi
    require_session_fields MEMORY_ACCOUNT_ID CLOCK_ID || return 1
    mem_ref="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")"
    clk_ref="$(ptb_shared_ref "$CLOCK_ID")"
    log_step "Creating agentic organization"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$OWNER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::memory::create_agentic_organization" \
        "$mem_ref" 0 \
        "some($(literal_move_string "AI Credit Org"))" \
        "some($(literal_move_string "E2E test org"))" \
        "$clk_ref")" || return 1
    digest="$(extract_tx_digest "$out")"
    ORG_ID="$(extract_created_object_by_type "$digest" "memory::AgenticOrganization")"
    [[ -n "$ORG_ID" ]] || ORG_ID="$(extract_created_object_by_type "$digest" "AgenticOrganization")"
    [[ -n "$ORG_ID" ]] || { echo "Could not find AgenticOrganization in tx $digest" >&2; return 1; }
    ORG_ID="$(normalize_hex_id "$ORG_ID")"
    log_session_use "ORG_ID" "$ORG_ID"
    save_session_state
}

register_sub_agent_if_needed() {
    local out digest agent_pk_vec derived_addr mem_ref org_ref clk_ref
    if session_value_set AGENT_OBJECT_ID; then
        log_session_use "AGENT_OBJECT_ID (existing)" "$AGENT_OBJECT_ID"
        return 0
    fi
    require_session_fields MEMORY_ACCOUNT_ID ORG_ID CLOCK_ID || return 1
    agent_pk_vec="$(move_vector_u8_from_hex "$DEFAULT_AGENT_PUBKEY_HEX")"
    derived_addr="$(move_address_from_hex "$AGENT_DERIVED_ADDRESS")"
    mem_ref="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")"
    org_ref="$(ptb_shared_ref "$ORG_ID")"
    clk_ref="$(ptb_shared_ref "$CLOCK_ID")"
    log_step "Registering sub-agent with CAP_AI_SPEND"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$OWNER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::memory::register_sub_agent" \
        "$mem_ref" "$org_ref" \
        "$agent_pk_vec" "$derived_addr" \
        "$(literal_move_string "ai-credit-e2e-agent")" \
        "$CLASS_DELEGATED_AI" 0 \
        "$CAP_AI_SPEND" "$CAP_AI_SPEND" \
        "$REGISTER_SCOPE" 0 \
        none none none \
        "$clk_ref")" || return 1
    digest="$(extract_tx_digest "$out")"
    AGENT_OBJECT_ID="$(extract_created_object_by_type "$digest" "memory::SubAgent")"
    [[ -n "$AGENT_OBJECT_ID" ]] || AGENT_OBJECT_ID="$(extract_created_object_by_type "$digest" "SubAgent")"
    [[ -n "$AGENT_OBJECT_ID" ]] || { echo "Could not find SubAgent in tx $digest" >&2; return 1; }
    AGENT_OBJECT_ID="$(normalize_hex_id "$AGENT_OBJECT_ID")"
    log_session_use "AGENT_OBJECT_ID" "$AGENT_OBJECT_ID"
    save_session_state
    sleep 2
}

set_agent_budget_and_deposit() {
    local out digest cfg_ref bal_ref agent_ref clk_ref split_amount deposit_coin gas_coin current_balance
    require_session_fields AI_CREDIT_CONFIG_ID AI_CREDIT_BALANCE_ID AGENT_OBJECT_ID CLOCK_ID || return 1
    split_amount=$(( DEPOSIT_MIST - DEPOSIT_GAS_HEADROOM ))
    cfg_ref="$(ptb_shared_ref "$AI_CREDIT_CONFIG_ID")"
    bal_ref="$(ptb_shared_ref "$AI_CREDIT_BALANCE_ID")"
    agent_ref="$(ptb_shared_ref "$AGENT_OBJECT_ID")"
    clk_ref="$(ptb_shared_ref "$CLOCK_ID")"
    log_step "Setting agent budget ($DEPOSIT_MIST MIST)"
    SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$OWNER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::ai_credit::set_agent_budget" \
        "$cfg_ref" "$bal_ref" "$agent_ref" \
        "some($DEPOSIT_MIST)" none none none \
        "$clk_ref" >/dev/null

    current_balance="$(gql_ai_credit_snapshot "$OWNER_ADDRESS" \
        | jq -r '.data.profile.aiCreditBalance.balanceMist // 0' 2>/dev/null || echo 0)"
    if [[ "$current_balance" -ge "$split_amount" ]]; then
        log_step "AiCreditBalance already has ${current_balance} MIST (>= ${split_amount}); skipping deposit"
    else
        ensure_deposit_funding "$OWNER_ADDRESS" "$split_amount" || return 1
        read -r deposit_coin gas_coin <<< "$(pick_deposit_and_gas_coins_for_address "$OWNER_ADDRESS" "$split_amount")"
        log_step "Depositing $split_amount MIST into AiCreditBalance (split @${deposit_coin#0x}, gas @${gas_coin#0x})"
        PTB_GAS_COIN_ID="$gas_coin"
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$OWNER_ADDRESS" \
            --split-coins "@${deposit_coin}" "[$split_amount]" \
            --assign pay_coin \
            --move-call "${PKG_SOCIAL}::ai_credit::deposit" \
            "$cfg_ref" "$bal_ref" pay_coin.0)" || { unset PTB_GAS_COIN_ID; return 1; }
        unset PTB_GAS_COIN_ID
        digest="$(extract_tx_digest "$out")"
        log_step "Deposit tx digest: $digest"
    fi
    wait_for_gql_ai_credit "$OWNER_ADDRESS" balanceMist "$split_amount"
    assert_rest_balance_field "$OWNER_ADDRESS" balance_mist "$split_amount"
}

run_oracle_usage_flow() {
    local pre usage_resp allowed amount receipt_id spent_before nonce_before settle_resp
    require_session_fields OWNER_ADDRESS AI_CREDIT_BALANCE_ID MEMORY_ACCOUNT_ID AGENT_OBJECT_ID || return 1

    log_step "Oracle preflight"
    pre="$(oracle_preflight "$OWNER_ADDRESS" "$AGENT_OBJECT_ID")"
    echo "$pre" | jq '.' >&2
    allowed="$(echo "$pre" | jq -r '.allowed // false')"
    [[ "$allowed" == "true" ]] || {
        echo "Preflight not allowed: $(echo "$pre" | jq -r '.reason // "unknown"')" >&2
        return 1
    }

    spent_before="$(rpc_ai_credit_spent_total_mist "$AI_CREDIT_BALANCE_ID" 2>/dev/null || echo 0)"
    nonce_before="$(rpc_ai_credit_settlement_nonce "$AI_CREDIT_BALANCE_ID" 2>/dev/null || echo 0)"

    log_step "Oracle record usage"
    usage_resp="$(oracle_record_usage "$OWNER_ADDRESS" "$AI_CREDIT_BALANCE_ID" "$MEMORY_ACCOUNT_ID" "$AGENT_OBJECT_ID")"
    echo "$usage_resp" | jq '.' >&2
    amount="$(echo "$usage_resp" | jq -r '.amount_mist // empty')"
    [[ -n "$amount" && "$amount" != "null" ]] || {
        echo "Usage recording failed: $(echo "$usage_resp" | jq -r '. // empty' 2>/dev/null || echo "$usage_resp")" >&2
        return 1
    }
    receipt_id="$(echo "$usage_resp" | jq -r '.receipt_id // empty')"

    log_step "Waiting for usage-history ingest"
    assert_usage_history_nonempty "$AI_CREDIT_BALANCE_ID"

    log_step "Triggering settlement flush"
    settle_resp="$(oracle_trigger_settle)" || true
    echo "$settle_resp" | jq '.' >&2 || echo "$settle_resp" >&2
    sleep 2
    settle_resp="$(oracle_trigger_settle)" || true
    echo "$settle_resp" | jq '.' >&2 || echo "$settle_resp" >&2

    verify_settlement_complete "$AI_CREDIT_BALANCE_ID" "$OWNER_ADDRESS" \
        "$nonce_before" "$spent_before" "$receipt_id"

    log_step "REST config snapshot"
    rest_get_ai_credit_config | jq '.' >&2 || true
}

run_oracle_usage_second() {
    local usage_resp receipt_id nonce_before spent_before settle_resp
    nonce_before="$(rpc_ai_credit_settlement_nonce "$AI_CREDIT_BALANCE_ID" 2>/dev/null || echo 0)"
    spent_before="$(rpc_ai_credit_spent_total_mist "$AI_CREDIT_BALANCE_ID" 2>/dev/null || echo 0)"
    log_step "Second oracle usage (on-chain nonce before=$nonce_before spent=$spent_before)"
    usage_resp="$(oracle_record_usage "$OWNER_ADDRESS" "$AI_CREDIT_BALANCE_ID" "$MEMORY_ACCOUNT_ID" "$AGENT_OBJECT_ID")"
    echo "$usage_resp" | jq '.' >&2
    receipt_id="$(echo "$usage_resp" | jq -r '.receipt_id // empty')"

    log_step "Waiting for second usage-history ingest"
    assert_usage_history_nonempty "$AI_CREDIT_BALANCE_ID"

    log_step "Triggering settlement flush (second usage)"
    settle_resp="$(oracle_trigger_settle)" || true
    echo "$settle_resp" | jq '.' >&2 || echo "$settle_resp" >&2
    sleep 2
    settle_resp="$(oracle_trigger_settle)" || true
    echo "$settle_resp" | jq '.' >&2 || echo "$settle_resp" >&2

    verify_settlement_complete "$AI_CREDIT_BALANCE_ID" "$OWNER_ADDRESS" \
        "$nonce_before" "$spent_before" "$receipt_id"

    log_step "Final GraphQL ai-credit snapshot"
    gql_ai_credit_snapshot "$OWNER_ADDRESS" | jq '.' >&2
}

run_all_flow() {
    ensure_owner_funded || return 1
    require_session_fields USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID AI_CREDIT_CONFIG_ID AI_CREDIT_ORACLE_ADMIN_CAP_ID || {
        echo "Run --refresh-session first" >&2
        return 1
    }
    start_oracle_background || return 1
    update_oracle_pubkey_on_chain || return 1
    ensure_profile_and_balance || return 1
    create_agentic_organization_if_needed || return 1
    register_sub_agent_if_needed || return 1
    set_agent_budget_and_deposit || return 1
    run_oracle_usage_flow || return 1
    run_oracle_usage_second || return 1
    log_step "AI credit E2E run-all completed successfully"
}

show_menu() {
    cat >&2 <<'MENU'

AI Credit E2E (scripts/ai-credit-runnable.sh)
  1) Refresh session from GraphQL
  2) Start oracle (background)
  3) Stop oracle
  4) Run full E2E (--run-all)
  5) On-chain setup only (profile → deposit)
  6) Oracle usage + settlement only (requires setup)
  q) Quit
MENU
}

main() {
    load_session_state
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -h|--help) usage; exit 0 ;;
            --refresh-session) RUN_MODE=refresh; shift ;;
            --start-oracle) RUN_MODE=start_oracle; shift ;;
            --stop-oracle) RUN_MODE=stop_oracle; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            --clean-receipts) RUN_MODE=clean_receipts; shift ;;
            -y) ASSUME_YES=1; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    case "${RUN_MODE:-menu}" in
        refresh) refresh_ai_credit_session_from_graphql ;;
        start_oracle)
            require_session_fields AI_CREDIT_CONFIG_ID || { refresh_ai_credit_session_from_graphql || exit 1; }
            start_oracle_background
            ;;
        stop_oracle) stop_oracle_background ;;
        clean_receipts)
            rm -f "$RECEIPT_STORE"
            log_step "Removed receipt store $RECEIPT_STORE"
            ;;
        run_all) run_all_flow ;;
        menu)
            show_menu
            read -r -p "Choice: " choice
            case "${choice:-q}" in
                1) refresh_ai_credit_session_from_graphql ;;
                2)
                    require_session_fields AI_CREDIT_CONFIG_ID 2>/dev/null || refresh_ai_credit_session_from_graphql
                    start_oracle_background
                    ;;
                3) stop_oracle_background ;;
                4) run_all_flow ;;
                5)
                    ensure_owner_funded
                    require_session_fields USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID AI_CREDIT_CONFIG_ID AI_CREDIT_ORACLE_ADMIN_CAP_ID
                    update_oracle_pubkey_on_chain
                    ensure_profile_and_balance
                    create_agentic_organization_if_needed
                    register_sub_agent_if_needed
                    set_agent_budget_and_deposit
                    ;;
                6)
                    start_oracle_background
                    run_oracle_usage_flow
                    ;;
                q|Q) exit 0 ;;
                *) echo "Invalid choice" >&2; exit 1 ;;
            esac
            ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
