#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared helpers for social E2E runnable scripts (username marketplace, post promotion).
# Source from runnable scripts; REPO_ROOT defaults from this file's location (scripts/lib → repo root).
# Runnable scripts may override REPO_ROOT and SOCIAL_SESSION_SAVE_PATH before sourcing.

if [[ -n "${_SOCIAL_RUNTIME_COMMON_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_SOCIAL_RUNTIME_COMMON_SOURCED=1

_social_runtime_lib_dir() {
    cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd
}

_social_runtime_repo_root() {
    local lib_dir
    lib_dir="$(_social_runtime_lib_dir)"
    cd "${lib_dir}/../.." && pwd
}

: "${REPO_ROOT:=$(_social_runtime_repo_root)}"
: "${SOCIAL_SESSION_SAVE_PATH:=$REPO_ROOT/network.config/social/social-session.env}"

readonly SOCIAL_DEFAULT_PKG='0x00000000000000000000000000000000000000000000000000000000000050c1'
readonly SOCIAL_DEFAULT_CLOCK='0x0000000000000000000000000000000000000000000000000000000000000006'
readonly SOCIAL_DEFAULT_COIN_TYPE='0x2::myso::MYSO'
readonly SOCIAL_DEFAULT_GAS_BUDGET='1000000000'
readonly SOCIAL_DEFAULT_PLATFORM_LOGO_URL='https://pub-1f3749a8084a44c3abbd97a4875268a1.r2.dev/Logo%20Regular%20-%20Accent-1.png'
readonly SOCIAL_DEFAULT_PLATFORM_COVER_PHOTO_URL='https://pub-1f3749a8084a44c3abbd97a4875268a1.r2.dev/mysocial-banner.png'

GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
SOCIAL_SERVER_URL="${SOCIAL_SERVER_URL:-http://127.0.0.1:9126}"

PKG_SOCIAL="${PKG_SOCIAL:-$SOCIAL_DEFAULT_PKG}"
CLOCK_ID="${CLOCK_ID:-$SOCIAL_DEFAULT_CLOCK}"
COIN_TYPE="${COIN_TYPE:-$SOCIAL_DEFAULT_COIN_TYPE}"
GAS_BUDGET="${GAS_BUDGET:-$SOCIAL_DEFAULT_GAS_BUDGET}"

USERNAME_REGISTRY_ID=''
USERNAME_MARKETPLACE_ID=''
PROFILE_CONFIG_ID=''
AI_CREDIT_CONFIG_ID=''
MEMORY_REGISTRY_ID=''
MEMORY_CONFIG_ID=''
ECOSYSTEM_TREASURY_ID=''
PLATFORM_REGISTRY_ID=''
PLATFORM_OBJECT_ID=''
PLATFORM_ADMIN_CAP_ID=''
BLOCK_LIST_REGISTRY_ID=''
POST_CONFIG_ID=''
MYDATA_REGISTRY_ID=''
MYDATA_CONFIG_ID=''
MYDATA_ADMIN_CAP_ID=''

SOCIAL_RUN_ID="${SOCIAL_RUN_ID:-$(date +%s)}"
ASSUME_YES="${ASSUME_YES:-0}"
GQL_REFRESH_FILE=''
WALLET_STACK=()

log_step() {
    echo "" >&2
    echo ">>> $*" >&2
}

log_session_use() {
    echo "  [session] $1=${2:-<unset>}" >&2
}

confirm_run() {
    if [[ "${ASSUME_YES:-0}" == 1 || "${SKIP_CONFIRM_RUN:-0}" == 1 ]]; then
        return 0
    fi
    if ! [[ -t 0 ]]; then
        log_step "Auto-confirming on-chain step (non-interactive stdin)"
        return 0
    fi
    read -r -p "Execute this command? [y/N] " ans
    [[ "${ans:-}" == [yY]* ]]
}

log_wait_progress() {
    local label="$1" attempt="$2" max="$3" detail="${4:-}"
    if (( attempt == 1 || attempt % 5 == 0 || attempt == max )); then
        if [[ -n "$detail" ]]; then
            log_step "Waiting: $label ($attempt/$max) $detail"
        else
            log_step "Waiting: $label ($attempt/$max)"
        fi
    fi
}

run_with_timeout() {
    local sec="$1"
    shift
    if command -v timeout >/dev/null 2>&1; then
        timeout "$sec" "$@"
    else
        "$@"
    fi
}

session_value_set() {
    local var_name="$1"
    [[ -n "${!var_name:-}" ]]
}

social_apply_defaults() {
    [[ -n "${PKG_SOCIAL:-}" ]] || PKG_SOCIAL="$SOCIAL_DEFAULT_PKG"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$SOCIAL_DEFAULT_CLOCK"
    [[ -n "${COIN_TYPE:-}" ]] || COIN_TYPE="$SOCIAL_DEFAULT_COIN_TYPE"
    [[ -n "${GAS_BUDGET:-}" ]] || GAS_BUDGET="$SOCIAL_DEFAULT_GAS_BUDGET"
}

social_load_session() {
    if [[ -f "$SOCIAL_SESSION_SAVE_PATH" ]]; then
        # shellcheck disable=SC1090
        source "$SOCIAL_SESSION_SAVE_PATH"
        echo "Loaded session from: $SOCIAL_SESSION_SAVE_PATH" >&2
    fi
    social_apply_defaults
}

social_save_session() {
    local f key
    f="$SOCIAL_SESSION_SAVE_PATH"
    mkdir -p "$(dirname "$f")"
    local old_umask
    old_umask="$(umask)"
    umask 077
    {
        echo "# Social E2E session — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
        for key in "$@"; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    umask "$old_umask"
    echo "Saved session to: $f" >&2
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

normalize_hex_id() {
    local id="$1"
    id="${id#@}"
    [[ -n "$id" ]] || return 1
    case "$id" in
        0x*) printf '%s' "$id" ;;
        *) printf '0x%s' "$id" ;;
    esac
}

object_exists_on_fullnode() {
    local id="$1"
    id="$(normalize_hex_id "$id")" || return 1
    myso client object "$id" >/dev/null 2>&1
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

# PTB move-call expects vector<u8> as vector[Nu8,...], not a bare 0x hex literal.
literal_move_vector_u8_from_hex() {
    local hex="${1#0x}"
    python3 - "$hex" <<'PY'
import sys
data = bytes.fromhex(sys.argv[1])
print("vector[" + ",".join(f"{b}u8" for b in data) + "]")
PY
}

literal_move_option_string() {
    local s="$1"
    if [[ -z "$s" ]]; then
        printf 'none'
        return 0
    fi
    printf 'some(%s)' "$(literal_move_string "$s")"
}

extra_gas_budget() {
    printf '%s\n' '--gas-budget' "${GAS_BUDGET:-$SOCIAL_DEFAULT_GAS_BUDGET}"
}

extra_dry() {
    if [[ "${DRY_RUN:-0}" == 1 ]]; then
        printf '%s\n' '--dry-run'
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

resolve_myso_active_address() {
    myso client active-address 2>/dev/null
}

switch_wallet() {
    local addr="$1" current
    addr="$(normalize_hex_id "$addr")" || return 1
    current="$(resolve_myso_active_address)" || current=''
    WALLET_STACK+=("$current")
    if [[ -n "$current" && "$(normalize_hex_id "$current" 2>/dev/null || true)" == "$addr" ]]; then
        return 0
    fi
    myso client switch --address "$addr" >/dev/null
}

restore_wallet() {
    local saved depth
    depth="${#WALLET_STACK[@]}"
    [[ "$depth" -gt 0 ]] || return 0
    saved="${WALLET_STACK[$((depth - 1))]}"
    unset 'WALLET_STACK[$((depth - 1))]'
    [[ -n "$saved" ]] || return 0
    local current
    current="$(resolve_myso_active_address)" || return 0
    if [[ "$(normalize_hex_id "$current")" != "$(normalize_hex_id "$saved")" ]]; then
        myso client switch --address "$saved" >/dev/null
    fi
}

create_ephemeral_wallet() {
    local label="$1" addr
    label="${label:-social_${SOCIAL_RUN_ID}}"
    addr="$(myso client new-address ed25519 "$label" --json 2>/dev/null | jq -r '.address // empty')"
    [[ -n "$addr" ]] || { echo "Could not create wallet $label" >&2; return 1; }
    normalize_hex_id "$addr"
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

resolve_max_coin_balance() {
    local addr="$1"
    resolve_gas_coins_json_for_address "$addr" \
        | jq '[.[].mistBalance | tonumber] | max // 0' 2>/dev/null
}

resolve_total_coin_balance() {
    local addr="$1"
    resolve_gas_coins_json_for_address "$addr" \
        | jq '[.[].mistBalance | tonumber] | add // 0' 2>/dev/null
}

ensure_wallet_funded() {
    local addr="$1" min_mist="${2:-$SOCIAL_DEFAULT_GAS_BUDGET}" attempt total_bal tap
    addr="$(normalize_hex_id "$addr")" || return 1
    switch_wallet "$addr" || return 1
    total_bal="$(resolve_total_coin_balance "$addr")"
    if [[ -n "$total_bal" && "$total_bal" -ge "$min_mist" ]]; then
        restore_wallet
        return 0
    fi
    log_step "Funding $addr via faucet (need >= $min_mist MIST total; ~5 MYSO per request)"
    for tap in $(seq 1 5); do
        total_bal="$(resolve_total_coin_balance "$addr")"
        if [[ -n "$total_bal" && "$total_bal" -ge "$min_mist" ]]; then
            break
        fi
        myso client faucet --address "$addr" >/dev/null 2>&1 \
            || myso client faucet --address "$addr" >&2 \
            || true
        [[ "$tap" -lt 5 ]] && sleep 2
    done
    local max_wait="${FAUCET_WAIT_MAX:-20}"
    for attempt in $(seq 1 "$max_wait"); do
        log_wait_progress "faucet balance" "$attempt" "$max_wait" "need >= $min_mist MIST"
        sleep 1
        total_bal="$(resolve_total_coin_balance "$addr")"
        [[ -n "$total_bal" && "$total_bal" -ge "$min_mist" ]] && break
    done
    restore_wallet
    [[ -n "$total_bal" && "$total_bal" -ge "$min_mist" ]] || {
        echo "Wallet $addr not funded after faucet requests (total=${total_bal:-0}, need=$min_mist)" >&2
        return 1
    }
}

ensure_two_gas_coins_for_address() {
    local addr="$1" attempt json count
    addr="$(normalize_hex_id "$addr")" || return 1
    switch_wallet "$addr" || return 1
    for attempt in $(seq 1 5); do
        json="$(resolve_gas_coins_json_for_address "$addr")" || json='[]'
        count="$(echo "$json" | jq 'length')"
        [[ "$count" -ge 2 ]] && break
        log_step "Requesting faucet for additional gas coin ($addr has ${count:-0})"
        myso client faucet --address "$addr" >/dev/null 2>&1 \
            || myso client faucet --address "$addr" >&2 \
            || true
        sleep 2
    done
    restore_wallet
    json="$(resolve_gas_coins_json_for_address "$addr")" || json='[]'
    count="$(echo "$json" | jq 'length')"
    [[ "$count" -ge 2 ]] || {
        echo "Address $addr needs two gas coins (payment + gas); has ${count:-0}" >&2
        return 1
    }
}

pick_payment_and_gas_coins_for_address() {
    local addr="$1" amount="$2"
    local min_gas="${3:-${GAS_BUDGET:-$SOCIAL_DEFAULT_GAS_BUDGET}}"
    local json pay_coin gas_coin pair attempt
    addr="$(normalize_hex_id "$addr")" || return 1
    ensure_wallet_funded "$addr" "$((amount + min_gas))" || return 1
    ensure_two_gas_coins_for_address "$addr" || return 1

    for attempt in $(seq 1 6); do
        json="$(resolve_gas_coins_json_for_address "$addr")" || return 1
        # Require distinct coins: payment >= amount and gas >= gas budget (not just any leftover).
        pair="$(echo "$json" | jq -r --argjson amt "$amount" --argjson gasmin "$min_gas" '
            def oid: (.gasCoinId // .coinObjectId);
            . as $all
            | first(
                $all[] as $p
                | select((($p.mistBalance | tonumber) >= $amt))
                | $all[] as $g
                | select((($g | oid) != ($p | oid)) and (($g.mistBalance | tonumber) >= $gasmin))
                | "\($p | oid) \($g | oid)"
              ) // empty
        ')"
        if [[ -n "$pair" ]]; then
            read -r pay_coin gas_coin <<<"$pair"
            printf '%s %s' "$pay_coin" "$gas_coin"
            return 0
        fi
        log_step "Requesting faucet for distinct payment (>=${amount}) + gas (>=${min_gas}) coins ($addr)"
        myso client faucet --address "$addr" >/dev/null 2>&1 \
            || myso client faucet --address "$addr" >&2 \
            || true
        sleep 2
    done
    echo "Could not pick payment (>=${amount}) and gas (>=${min_gas}) coins for $addr" >&2
    return 1
}

graphql_is_reachable() {
    local url="${1:-$GRAPHQL_URL}"
    [[ -n "$url" ]] || return 1
    curl -sf --connect-timeout 3 --max-time 5 "$url" \
        -H 'Content-Type: application/json' \
        -d '{"query":"{ __typename }"}' >/dev/null 2>&1
}

graphql_refresh_hint() {
    cat >&2 <<EOF
GraphQL is unreachable at ${GRAPHQL_URL:-<unset>}.
Start the localnet social indexer (GraphQL + social-server), then retry:
  myso start --with-social-indexer
Or set GRAPHQL_URL to your indexer endpoint and run:
  ./scripts/spot-oracle-runnable.sh --refresh-session
EOF
}

graphql_post() {
    local query="$1"
    local vars="${2-}"
    local body http_code resp
    if [[ -z "$vars" ]]; then
        vars='{}'
    fi
    body="$(jq -nc --arg q "$query" --argjson v "$vars" '{query: $q, variables: $v}')" || return 1
    resp="$(curl -sf --connect-timeout 5 --max-time 30 -w '\n%{http_code}' -X POST "$GRAPHQL_URL" \
        -H 'Content-Type: application/json' \
        -d "$body" 2>/dev/null)" || {
        echo "GraphQL request failed: $GRAPHQL_URL" >&2
        graphql_refresh_hint
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

readonly SOCIAL_GQL_BATCH='query SocialE2ESessionObjects {
  ecosystemTreasury: objects(filter: { type: "0x50c1::profile::EcosystemTreasury", ownerKind: SHARED }, last: 1) { nodes { address } }
  usernameRegistry: objects(filter: { type: "0x50c1::profile::UsernameRegistry", ownerKind: SHARED }, last: 1) { nodes { address } }
  usernameMarketplace: objects(filter: { type: "0x50c1::profile::UsernameMarketplace", ownerKind: SHARED }, last: 1) { nodes { address } }
  profileConfig: objects(filter: { type: "0x50c1::profile::ProfileConfig", ownerKind: SHARED }, last: 1) { nodes { address } }
  memoryRegistry: objects(filter: { type: "0x50c1::memory::MemoryRegistry", ownerKind: SHARED }, last: 1) { nodes { address } }
  memoryConfig: objects(filter: { type: "0x50c1::memory::MemoryConfig", ownerKind: SHARED }, last: 1) { nodes { address } }
  aiCreditConfig: objects(filter: { type: "0x50c1::ai_credit::AiCreditConfig", ownerKind: SHARED }, last: 1) { nodes { address } }
  platformRegistry: objects(filter: { type: "0x50c1::platform::PlatformRegistry", ownerKind: SHARED }, last: 1) { nodes { address } }
  platformConfig: objects(filter: { type: "0x50c1::platform::PlatformConfig", ownerKind: SHARED }, last: 1) { nodes { address } }
  blocklistRegistry: objects(filter: { type: "0x50c1::block_list::BlockListRegistry", ownerKind: SHARED }, last: 1) { nodes { address } }
  mydataRegistry: objects(filter: { type: "0x50c1::mydata::MyDataRegistry", ownerKind: SHARED }, last: 1) { nodes { address } }
  mydataConfig: objects(filter: { type: "0x50c1::mydata::MyDataConfig", ownerKind: SHARED }, last: 1) { nodes { address } }
  mydataAdminCap: objects(filter: { type: "0x50c1::mydata::MyDataAdminCap" }, last: 1) { nodes { address } }
  postConfig: objects(filter: { type: "0x50c1::post::PostConfig", ownerKind: SHARED }, last: 1) { nodes { address } }
  subscriptionConfig: objects(filter: { type: "0x50c1::subscription::SubscriptionConfig", ownerKind: SHARED }, last: 1) { nodes { address } }
  subscriptionAdminCap: objects(filter: { type: "0x50c1::subscription::SubscriptionAdminCap" }, last: 1) { nodes { address } }
  platformAdminCap: objects(filter: { type: "0x50c1::platform::PlatformAdminCap" }, last: 1) { nodes { address } }
  platform: objects(filter: { type: "0x50c1::platform::Platform" }, last: 1) { nodes { address } }
}'

gql_set_refresh() {
    local key="$1" val="$2"
    [[ -n "$val" ]] || return 0
    printf '%s=%q\n' "$key" "$val" >> "$GQL_REFRESH_FILE"
}

collect_social_gql_mappings() {
    local json="$1" alias val env_key
    for alias in ecosystemTreasury usernameRegistry usernameMarketplace profileConfig \
        memoryRegistry memoryConfig aiCreditConfig platformRegistry platformConfig blocklistRegistry \
        mydataRegistry mydataConfig mydataAdminCap postConfig subscriptionConfig subscriptionAdminCap platformAdminCap platform; do
        case "$alias" in
            ecosystemTreasury) env_key=ECOSYSTEM_TREASURY_ID ;;
            usernameRegistry) env_key=USERNAME_REGISTRY_ID ;;
            usernameMarketplace) env_key=USERNAME_MARKETPLACE_ID ;;
            profileConfig) env_key=PROFILE_CONFIG_ID ;;
            memoryRegistry) env_key=MEMORY_REGISTRY_ID ;;
            memoryConfig) env_key=MEMORY_CONFIG_ID ;;
            aiCreditConfig) env_key=AI_CREDIT_CONFIG_ID ;;
            platformRegistry) env_key=PLATFORM_REGISTRY_ID ;;
            platformConfig) env_key=PLATFORM_CONFIG_ID ;;
            blocklistRegistry) env_key=BLOCK_LIST_REGISTRY_ID ;;
            mydataRegistry) env_key=MYDATA_REGISTRY_ID ;;
            mydataConfig) env_key=MYDATA_CONFIG_ID ;;
            mydataAdminCap) env_key=MYDATA_ADMIN_CAP_ID ;;
            postConfig) env_key=POST_CONFIG_ID ;;
            subscriptionConfig) env_key=SUBSCRIPTION_CONFIG_ID ;;
            subscriptionAdminCap) env_key=SUBSCRIPTION_ADMIN_CAP_ID ;;
            platformAdminCap) env_key=PLATFORM_ADMIN_CAP_ID ;;
            platform) env_key=PLATFORM_OBJECT_ID ;;
            *) continue ;;
        esac
        val="$(gql_object_address "$json" "$alias")"
        gql_set_refresh "$env_key" "$val"
    done
}

# Return object addresses for a Move type from GraphQL (newest-first when using last).
gql_object_addresses_for_type() {
    local move_type="$1" owner_kind="${2:-SHARED}" limit="${3:-8}"
    local json query
    if [[ "$owner_kind" == "SHARED" ]]; then
        query="query { objects(filter: { type: \"${move_type}\", ownerKind: SHARED }, last: ${limit}) { nodes { address } } }"
    else
        query="query { objects(filter: { type: \"${move_type}\" }, last: ${limit}) { nodes { address } } }"
    fi
    json="$(graphql_post "$query")" || return 1
    echo "$json" | jq -r '.data.objects.nodes[]?.address // empty'
}

gql_live_object_address_for_type() {
    local move_type="$1" owner_kind="${2:-SHARED}"
    local candidate
    while IFS= read -r candidate; do
        [[ -n "$candidate" ]] || continue
        if object_exists_on_fullnode "$candidate"; then
            normalize_hex_id "$candidate"
            return 0
        fi
    done < <(gql_object_addresses_for_type "$move_type" "$owner_kind")
    return 1
}

# Back-compat helper used by subscription SPT fetch.
gql_object_address_for_type() {
    gql_live_object_address_for_type "$@"
}

# After a chain reset, GraphQL may still index old object ids. Keep only ids that exist on fullnode.
social_validate_session_id_on_fullnode() {
    local env_key="$1" move_type="$2" owner_kind="${3:-SHARED}" required="${4:-1}"
    local id resolved
    id="${!env_key:-}"
    if [[ -n "$id" ]] && object_exists_on_fullnode "$id"; then
        printf -v "$env_key" '%s' "$(normalize_hex_id "$id")"
        return 0
    fi
    if [[ -n "$id" ]]; then
        echo "GraphQL ${env_key}=${id} not on fullnode; re-resolving ${move_type}" >&2
    fi
    resolved="$(gql_live_object_address_for_type "$move_type" "$owner_kind")" || resolved=''
    if [[ -n "$resolved" ]]; then
        printf -v "$env_key" '%s' "$resolved"
        log_session_use "$env_key" "${!env_key}"
        return 0
    fi
    printf -v "$env_key" '%s' ''
    if [[ "$required" == 1 ]]; then
        echo "Could not resolve live ${env_key} (${move_type}) on fullnode — run bootstrap and wait for indexer sync" >&2
        return 1
    fi
    echo "Optional ${env_key} (${move_type}) not on fullnode; continuing without it" >&2
    return 0
}

social_validate_shared_session_ids_on_fullnode() {
    local failed=0
    social_validate_session_id_on_fullnode ECOSYSTEM_TREASURY_ID '0x50c1::profile::EcosystemTreasury' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode USERNAME_REGISTRY_ID '0x50c1::profile::UsernameRegistry' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode USERNAME_MARKETPLACE_ID '0x50c1::profile::UsernameMarketplace' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode PROFILE_CONFIG_ID '0x50c1::profile::ProfileConfig' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode MEMORY_REGISTRY_ID '0x50c1::memory::MemoryRegistry' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode MEMORY_CONFIG_ID '0x50c1::memory::MemoryConfig' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode AI_CREDIT_CONFIG_ID '0x50c1::ai_credit::AiCreditConfig' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode PLATFORM_REGISTRY_ID '0x50c1::platform::PlatformRegistry' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode PLATFORM_CONFIG_ID '0x50c1::platform::PlatformConfig' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode BLOCK_LIST_REGISTRY_ID '0x50c1::block_list::BlockListRegistry' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode MYDATA_REGISTRY_ID '0x50c1::mydata::MyDataRegistry' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode MYDATA_CONFIG_ID '0x50c1::mydata::MyDataConfig' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode POST_CONFIG_ID '0x50c1::post::PostConfig' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode SUBSCRIPTION_CONFIG_ID '0x50c1::subscription::SubscriptionConfig' SHARED 1 || failed=1
    social_validate_session_id_on_fullnode SUBSCRIPTION_ADMIN_CAP_ID '0x50c1::subscription::SubscriptionAdminCap' ANY 1 || failed=1
    social_validate_session_id_on_fullnode MYDATA_ADMIN_CAP_ID '0x50c1::mydata::MyDataAdminCap' ANY 0 || true
    social_validate_session_id_on_fullnode PLATFORM_ADMIN_CAP_ID '0x50c1::platform::PlatformAdminCap' ANY 0 || true
    social_validate_session_id_on_fullnode PLATFORM_OBJECT_ID '0x50c1::platform::Platform' ANY 0 || true
    return "$failed"
}

social_refresh_session_from_graphql() {
    command -v curl >/dev/null 2>&1 || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }

    local old_session preserved_keys=(
        SELLER_ADDRESS BUYER_ADDRESS SELLER_PROFILE_ID BUYER_PROFILE_ID
        LISTING_USERNAME REPLACEMENT_USERNAME SELLER_MEMORY_ACCOUNT_ID PAY_COIN_ID
    )
    local key old_session_file=""
    if [[ -f "$SOCIAL_SESSION_SAVE_PATH" ]]; then
        old_session_file="$(mktemp)"
        cp "$SOCIAL_SESSION_SAVE_PATH" "$old_session_file"
    fi

    log_step "Refreshing session from GraphQL ($GRAPHQL_URL)"
    if ! graphql_is_reachable; then
        echo "GraphQL unreachable at $GRAPHQL_URL" >&2
        graphql_refresh_hint
        rm -f "$old_session_file"
        return 1
    fi
    GQL_REFRESH_FILE="$(mktemp)"
    local json
    json="$(graphql_post "$SOCIAL_GQL_BATCH")" || {
        rm -f "$GQL_REFRESH_FILE" "$old_session_file"
        GQL_REFRESH_FILE=''
        return 1
    }
    collect_social_gql_mappings "$json"

    if [[ -f "$GQL_REFRESH_FILE" ]]; then
        # shellcheck disable=SC1090
        source "$GQL_REFRESH_FILE"
    fi
    social_validate_shared_session_ids_on_fullnode || {
        rm -f "$old_session_file"
        return 1
    }
    rm -f "$GQL_REFRESH_FILE"
    GQL_REFRESH_FILE=''

    PKG_SOCIAL="$SOCIAL_DEFAULT_PKG"
    CLOCK_ID="$SOCIAL_DEFAULT_CLOCK"
    social_apply_defaults

    mkdir -p "$(dirname "$SOCIAL_SESSION_SAVE_PATH")"
    {
        echo "# Refreshed from GraphQL $(date -u +%Y-%m-%dT%H:%M:%SZ)"
        printf '%s=%q\n' PKG_SOCIAL "$PKG_SOCIAL"
        printf '%s=%q\n' CLOCK_ID "$CLOCK_ID"
        printf '%s=%q\n' COIN_TYPE "$COIN_TYPE"
        printf '%s=%q\n' GAS_BUDGET "$GAS_BUDGET"
        for key in USERNAME_REGISTRY_ID USERNAME_MARKETPLACE_ID PROFILE_CONFIG_ID \
            AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID \
            PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_OBJECT_ID PLATFORM_ADMIN_CAP_ID \
            BLOCK_LIST_REGISTRY_ID POST_CONFIG_ID MYDATA_REGISTRY_ID MYDATA_CONFIG_ID MYDATA_ADMIN_CAP_ID \
            SUBSCRIPTION_CONFIG_ID SUBSCRIPTION_ADMIN_CAP_ID; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${SOCIAL_SESSION_SAVE_PATH}.tmp"
    if [[ -n "$old_session_file" && -f "$old_session_file" ]]; then
        {
            # shellcheck disable=SC1090
            source "$old_session_file"
            for key in "${preserved_keys[@]}"; do
                session_value_set "$key" && printf '%s=%q\n' "$key" "${!key}"
            done
        } >> "${SOCIAL_SESSION_SAVE_PATH}.tmp"
        rm -f "$old_session_file"
    fi
    # shellcheck disable=SC1090
    source "${SOCIAL_SESSION_SAVE_PATH}.tmp"
    mv "${SOCIAL_SESSION_SAVE_PATH}.tmp" "$SOCIAL_SESSION_SAVE_PATH"
    chmod 600 "$SOCIAL_SESSION_SAVE_PATH" 2>/dev/null || true

    log_session_use "USERNAME_MARKETPLACE_ID" "$USERNAME_MARKETPLACE_ID"
    log_session_use "USERNAME_REGISTRY_ID" "$USERNAME_REGISTRY_ID"
    log_session_use "PROFILE_CONFIG_ID" "$PROFILE_CONFIG_ID"
    log_session_use "PLATFORM_CONFIG_ID" "$PLATFORM_CONFIG_ID"
    log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
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
        out="$(run_with_timeout "${MYSO_CMD_TIMEOUT_SEC:-180}" "${cmd[@]}" 2>&1)" || rc=$?
        if [[ "$rc" == 124 ]]; then
            echo "Timed out after ${MYSO_CMD_TIMEOUT_SEC:-180}s: ${cmd[*]}" >&2
        fi
        echo "$out" >&2
        printf '%s' "$out"
        return "$rc"
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
        out="$(run_with_timeout "${MYSO_CMD_TIMEOUT_SEC:-180}" "${cmd[@]}" 2>&1)" || rc=$?
        if [[ "$rc" == 124 ]]; then
            echo "Timed out after ${MYSO_CMD_TIMEOUT_SEC:-180}s: ${cmd[*]}" >&2
        fi
        echo "$out" >&2
        printf '%s' "$out"
        return "$rc"
    else
        return 0
    fi
}

run_myso_call_as_capture() {
    local sender="$1" module="$2" func="$3"
    shift 3
    local -a cmd call_args=() out
    local arg
    while IFS= read -r -d '' arg; do call_args+=("$arg"); done < <(normalize_client_call_args "$@")
    sender="$(normalize_hex_id "$sender")" || return 1
    cmd=(myso client call --package "$PKG_SOCIAL" --sender "$sender" \
        --module "$module" --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=(--args)
    cmd+=("${call_args[@]}")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        local rc=0
        out="$(run_with_timeout "${MYSO_CMD_TIMEOUT_SEC:-180}" "${cmd[@]}" 2>&1)" || rc=$?
        if [[ "$rc" == 124 ]]; then
            echo "Timed out after ${MYSO_CMD_TIMEOUT_SEC:-180}s: ${cmd[*]}" >&2
        fi
        echo "$out" >&2
        printf '%s' "$out"
        return "$rc"
    else
        return 0
    fi
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

assert_tx_success() {
    local out="$1" digest="${2:-}" json tx_status
    [[ -n "$out" || -n "$digest" ]] || return 1
    if [[ -n "$out" ]]; then
        if echo "$out" | grep -qE 'Dry run completed, execution status: success'; then
            return 0
        fi
        if echo "$out" | grep -qE 'TransactionEffectsV2 \{ status: Success|effects\.V2\.status.*Success'; then
            return 0
        fi
    fi
    if [[ -z "$digest" ]]; then
        digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    fi
    if [[ -n "$digest" ]]; then
        json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
        tx_status="$(echo "$json" | jq -r '.effects.V2.status // .effects.status // empty | tostring')"
        [[ "$tx_status" == "Success" ]]
        return
    fi
    echo "$out" | jq -e '
        (.effects.V2.status // .effects.status // empty | tostring) == "Success"
    ' >/dev/null
}

tx_has_event_named() {
    local digest="$1" event_name="$2" json
    [[ -n "$digest" && -n "$event_name" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    echo "$json" | jq -e --arg name "$event_name" '
        [.. | objects | select(has("type_") or has("type"))]
        | map(.type_ // .type)
        | any(.name? == $name)
    ' >/dev/null
}

tx_event_field() {
    local digest="$1" event_name="$2" field="$3" json value
    [[ -n "$digest" && -n "$event_name" && -n "$field" ]] || return 1

    # Some CLIs surface parsed event fields directly; use them when present.
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || json=''
    if [[ -n "$json" ]]; then
        value="$(echo "$json" | jq -r --arg field "$field" '
            [.. | objects | select(has("parsedJson") or has("parsed_json"))]
            | map(.parsedJson // .parsed_json)
            | map(select(. != null))
            | .[]
            | select(.[$field] != null)
            | .[$field]
            | tostring
        ' 2>/dev/null | head -n1)"
        if [[ -n "$value" ]]; then
            printf '%s' "$value"
            return 0
        fi
    fi

    # The myso CLI tx-block only returns raw BCS event `contents`, so fall back to
    # GraphQL for parsed event JSON. Retry briefly to absorb indexer lag.
    local gql resp attempt
    gql='query Tx($digest: String!) {
        transaction(digest: $digest) {
            effects { events { nodes { contents { type { repr } json } } } }
        }
    }'
    for attempt in $(seq 1 "${TX_EVENT_FIELD_GQL_RETRIES:-15}"); do
        resp="$(graphql_post "$gql" "$(jq -nc --arg digest "$digest" '{digest: $digest}')" 2>/dev/null)" || resp=''
        if [[ -n "$resp" ]]; then
            value="$(echo "$resp" | jq -r --arg name "$event_name" --arg field "$field" '
                (.data.transaction.effects.events.nodes // [])
                | map(select((.contents.type.repr // "") | endswith("::" + $name)))
                | map(.contents.json)
                | map(select(. != null and (.[$field] != null)))
                | .[0][$field] // empty
                | tostring
            ' 2>/dev/null)"
            if [[ -n "$value" && "$value" != "null" ]]; then
                printf '%s' "$value"
                return 0
            fi
        fi
        sleep 1
    done
    return 1
}

assert_tx_aborts() {
    local out="$1" digest="${2:-}"
    [[ -n "$out" || -n "$digest" ]] || return 1
    if [[ -n "$out" ]]; then
        if echo "$out" | grep -qE 'Dry run completed, execution status: failure'; then
            return 0
        fi
        if echo "$out" | grep -qE 'TransactionEffectsV2 \{ status: Failure'; then
            return 0
        fi
    fi
    if [[ -z "$digest" ]]; then
        digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    fi
    if [[ -n "$digest" ]]; then
        local json tx_status
        json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
        tx_status="$(echo "$json" | jq -r '.effects.V2.status // .effects.status // empty | tostring')"
        [[ "$tx_status" != "Success" ]]
        return
    fi
    echo "$out" | jq -e '
        (.effects.V2.status // .effects.status // empty | tostring) != "Success"
    ' >/dev/null 2>&1 && return 0
    echo "$out" | grep -qiE 'MoveAbort|Move Runtime Abort|abort code|InsufficientGas|InsufficientBalance' && return 0
    echo "$out" | grep -qiE 'not signed by the correct sender|Error checking transaction input objects' && return 0
    return 1
}

assert_tx_aborts_with_code() {
    local out="$1" expected_code="$2" digest="${3:-}" json status code
    [[ -n "$out" || -n "$digest" ]] || return 1
    assert_tx_aborts "$out" "$digest" || return 1
    if [[ -z "$digest" ]]; then
        digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    fi
    [[ -n "$digest" ]] || return 0
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    code="$(echo "$json" | jq -r '
        .. | objects
        | select(has("abortCode") or has("abort_code"))
        | (.abortCode // .abort_code)
        | if type == "object" then (.error_code // .errorCode // .code // empty) else . end
        | tostring
    ' | head -n1)"
    [[ -z "$code" || "$code" == "$expected_code" ]]
}

subscription_refresh_session() {
    social_refresh_session_from_graphql
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
        )
        | select(length > 0)
    ' | head -n1)"
    [[ -n "$result" ]] || return 1
    printf '%s' "$result"
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
    [[ -n "$owner" ]] && printf '%s' "$owner"
}

resolve_owned_profile_for_address() {
    local addr="$1" json profile_id
    addr="$(normalize_hex_id "$addr")" || return 1
    profile_id="$(gql_profile_id_for_address "$addr" 2>/dev/null)" || profile_id=''
    if [[ -n "$profile_id" ]]; then
        normalize_hex_id "$profile_id"
        return 0
    fi
    json="$(myso client objects "$addr" --json 2>/dev/null)" || return 1
    profile_id="$(echo "$json" | jq -r '
        .[]?
        | select(
            ((.data.Move.type_.Other? // empty | .module == "profile" and .name == "Profile")
                or (.type? | tostring | contains("profile::Profile")))
          )
        | .data.objectId // .objectId // .object_id // empty
    ' | head -n1)"
    [[ -n "$profile_id" ]] || return 1
    normalize_hex_id "$profile_id"
}

gql_profile_id_for_address() {
    local addr="$1" resp vars
    addr="$(normalize_hex_id "$addr")" || return 1
    vars="$(jq -nc --arg addr "$addr" '{addr: $addr}')"
    resp="$(graphql_post \
        'query ProfileId($addr: MySoAddress!) { profile(address: $addr) { profileId } }' \
        "$vars")" || return 1
    echo "$resp" | jq -r '.data.profile.profileId // empty' | head -n1
}

lookup_on_chain_profile_for_username() {
    local username="$1" out addr registry_id
    [[ -n "$username" ]] || return 1
    require_session_fields USERNAME_REGISTRY_ID || return 1
    registry_id="$(normalize_hex_id "$USERNAME_REGISTRY_ID")" || return 1
    out="$(myso client call --package "$PKG_SOCIAL" --module profile \
        --function lookup_profile_by_username --dry-run --json \
        --args "$registry_id" "$(literal_move_string "$username")" 2>/dev/null)" || return 1
    addr="$(echo "$out" | jq -r '.command_outputs[0].returnValues[0].json // empty' | head -1)"
    if [[ -z "$addr" || "$addr" == "null" ]]; then
        local b64
        b64="$(echo "$out" | jq -r '.command_outputs[0].returnValues[0].value.value // empty' | head -1)"
        [[ -n "$b64" && "$b64" != "null" && "$b64" != "AA==" ]] || return 1
        addr="$(python3 -c 'import base64,sys
raw=base64.b64decode(sys.argv[1])
if len(raw) < 1 or raw[0] != 1:
    sys.exit(1)
print("0x" + raw[1:33].hex())' "$b64" 2>/dev/null)" || return 1
    fi
    normalize_hex_id "$addr"
}

resolve_registry_username_for_profile() {
    local profile_id="$1" candidate owner
    shift
    profile_id="$(normalize_hex_id "$profile_id")" || return 1
    for candidate in "$@"; do
        [[ -n "$candidate" ]] || continue
        owner="$(lookup_on_chain_profile_for_username "$candidate" 2>/dev/null)" || continue
        if [[ "$(normalize_hex_id "$owner")" == "$profile_id" ]]; then
            printf '%s' "$candidate"
            return 0
        fi
    done
    return 1
}

collect_marketplace_username_candidates() {
    local gql_user="${1:-}"
    # Authoritative on-chain username first, then this run's planned listing/replacement only.
    # Do not scan session history — stale LISTING_USERNAME values break repeat E2E runs.
    [[ -n "$gql_user" ]] && printf '%s\0' "$gql_user"
    [[ -n "${LISTING_USERNAME:-}" ]] && printf '%s\0' "$LISTING_USERNAME"
    [[ -n "${REPLACEMENT_USERNAME:-}" ]] && printf '%s\0' "$REPLACEMENT_USERNAME"
}

gql_profile_snapshot() {
    local addr="$1" resp vars
    addr="$(normalize_hex_id "$addr")" || return 1
    vars="$(jq -nc --arg addr "$addr" '{addr: $addr}')"
    resp="$(graphql_post \
        'query ProfileSnapshot($addr: MySoAddress!) {
            profile(address: $addr) {
                profileId
                username
                memoryAccountId
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

gql_username_availability_snapshot() {
    local username="$1" resp vars
    [[ -n "$username" ]] || return 1
    vars="$(jq -nc --arg username "$username" '{username: $username}')"
    resp="$(graphql_post \
        'query UsernameAvailabilitySnapshot($username: String!) {
            usernameAvailability(username: $username) {
                username
                available
                registryClaimed
                beneficiaryProvisioned
                marketplaceListed
                listingSellerProfileId
                lockReasons
                unavailableReasons
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

rest_username_availability_snapshot() {
    local username="$1" encoded url
    encoded="$(python3 -c 'import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=""))' "$username")"
    url="${SOCIAL_SERVER_URL}/profiles/username/${encoded}/availability"
    curl -sS "$url" 2>/dev/null
}

wait_for_gql_username_availability_field() {
    local username="$1" jq_filter="$2" expected="$3" attempt resp val
    for attempt in $(seq 1 60); do
        resp="$(gql_username_availability_snapshot "$username" 2>/dev/null)" || resp='{}'
        val="$(echo "$resp" | jq -r "$jq_filter" 2>/dev/null || true)"
        if [[ -n "$val" && "$val" == "$expected" ]]; then
            echo "$resp" >&2
            return 0
        fi
        if (( attempt == 1 || attempt % 5 == 0 )); then
            echo "  [gql wait] attempt $attempt/60 usernameAvailability $username expected=$expected last=${val:-<none>}" >&2
        fi
        sleep 1
    done
    echo "Timed out waiting for usernameAvailability $username ($jq_filter expected $expected, last: ${val:-<none>})" >&2
    return 1
}

assert_username_marketplace_locked() {
    local username="$1" seller_profile_id="$2" resp seller_norm
    seller_norm="$(normalize_hex_id "$seller_profile_id")"
    log_step "Assert marketplace lock for username=$username seller=$seller_norm"
    wait_for_gql_username_availability_field "$username" '.data.usernameAvailability.marketplaceListed' 'true' || return 1
    wait_for_gql_username_availability_field "$username" '.data.usernameAvailability.available' 'false' || return 1
    wait_for_gql_username_availability_field "$username" '.data.usernameAvailability.listingSellerProfileId' "$seller_norm" || return 1
    resp="$(gql_username_availability_snapshot "$username" 2>/dev/null)" || resp='{}'
    if ! echo "$resp" | jq -e '.data.usernameAvailability.lockReasons | index("MARKETPLACE")' >/dev/null 2>&1; then
        echo "Expected MARKETPLACE in lockReasons for $username: $resp" >&2
        return 1
    fi
    if ! echo "$resp" | jq -e '.data.usernameAvailability.unavailableReasons | index("MARKETPLACE_LISTED")' >/dev/null 2>&1; then
        echo "Expected MARKETPLACE_LISTED in unavailableReasons for $username: $resp" >&2
        return 1
    fi
    log_step "Marketplace lock verified for $username"
}

assert_username_availability_case_insensitive() {
    local canonical="$1" upper resp canonical_avail upper_avail
    [[ -n "$canonical" ]] || return 1
    upper="$(printf '%s' "$canonical" | tr '[:lower:]' '[:upper:]')"
    [[ "$upper" != "$canonical" ]] || {
        log_step "Skip case-insensitive availability check (username already lowercase): $canonical"
        return 0
    }
    log_step "Assert case-insensitive usernameAvailability for $canonical vs $upper"
    resp="$(gql_username_availability_snapshot "$canonical" 2>/dev/null)" || resp='{}'
    canonical_avail="$(echo "$resp" | jq -c '.data.usernameAvailability // {}')"
    resp="$(gql_username_availability_snapshot "$upper" 2>/dev/null)" || resp='{}'
    upper_avail="$(echo "$resp" | jq -c '.data.usernameAvailability // {}')"
    if [[ "$canonical_avail" != "$upper_avail" ]]; then
        echo "Case-insensitive usernameAvailability mismatch:" >&2
        echo "  canonical ($canonical): $canonical_avail" >&2
        echo "  upper ($upper): $upper_avail" >&2
        return 1
    fi
    log_step "Case-insensitive usernameAvailability verified"
}

assert_on_chain_registry_username_absent() {
    local username="$1" owner
    owner="$(lookup_on_chain_profile_for_username "$username" 2>/dev/null)" || owner=''
    if [[ -n "$owner" ]]; then
        echo "On-chain registry: expected $username to be absent/unclaimed, still owned by $owner" >&2
        return 1
    fi
    log_step "On-chain registry: $username is not registered"
}

wait_for_gql_profile_field() {
    local addr="$1" jq_filter="$2" expected="$3" attempt resp val
    for attempt in $(seq 1 60); do
        resp="$(gql_profile_snapshot "$addr" 2>/dev/null)" || resp='{}'
        val="$(echo "$resp" | jq -r "$jq_filter" 2>/dev/null || true)"
        if [[ -n "$val" && "$val" == "$expected" ]]; then
            echo "$resp" >&2
            return 0
        fi
        if (( attempt == 1 || attempt % 5 == 0 )); then
            echo "  [gql wait] attempt $attempt/60 profile $addr expected=$expected last=${val:-<none>}" >&2
        fi
        sleep 1
    done
    echo "Timed out waiting for profile $addr ($jq_filter expected $expected, last: ${val:-<none>})" >&2
    return 1
}

assert_on_chain_registry_username() {
    local username="$1" expected_profile_id="$2" owner
    [[ -n "$username" && -n "$expected_profile_id" ]] || {
        echo "assert_on_chain_registry_username: username and profile id required" >&2
        return 1
    }
    owner="$(lookup_on_chain_profile_for_username "$username")" || {
        echo "On-chain registry: username $username not registered" >&2
        return 1
    }
    if [[ "$(normalize_hex_id "$owner")" != "$(normalize_hex_id "$expected_profile_id")" ]]; then
        echo "On-chain registry: $username owned by $owner, expected $expected_profile_id" >&2
        return 1
    fi
    log_step "On-chain registry: $username -> $expected_profile_id"
}

wait_for_rest_json() {
    local url="$1" jq_filter="$2" expected="$3" attempt resp val max="${REST_WAIT_MAX:-20}"
    for attempt in $(seq 1 "$max"); do
        log_wait_progress "REST" "$attempt" "$max" "$(basename "$url") expected=$expected"
        resp="$(curl -sf --connect-timeout 3 --max-time 8 "$url" 2>/dev/null)" || resp=''
        val="$(echo "$resp" | jq -r "$jq_filter" 2>/dev/null || true)"
        if [[ -n "$val" && "$val" == "$expected" ]]; then
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "Timed out REST $url ($jq_filter expected $expected, last: ${val:-<none>})" >&2
    echo "$resp" | jq '.' >&2 || echo "$resp" >&2
    return 1
}

wait_for_rest_offer_status() {
    local url="$1" status="$2" buyer_address="${3:-}" attempt resp val buyer_norm
    buyer_norm="$(normalize_hex_id "$buyer_address" 2>/dev/null || true)"
    for attempt in $(seq 1 60); do
        resp="$(curl -sS "$url" 2>/dev/null)" || resp='[]'
        if [[ -n "$buyer_norm" ]]; then
            val="$(echo "$resp" | jq -r \
                --arg s "$status" --arg b "$buyer_norm" \
                '(map(select(.status==$s and ((.buyer_address // "") | ascii_downcase) == ($b | ascii_downcase))) | sort_by(.updated_at) | reverse | .[0].status) // empty' \
                2>/dev/null || true)"
        else
            val="$(echo "$resp" | jq -r \
                --arg s "$status" \
                '(map(select(.status==$s)) | sort_by(.updated_at) | reverse | .[0].status) // empty' \
                2>/dev/null || true)"
        fi
        if [[ -n "$val" && "$val" == "$status" ]]; then
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "Timed out REST $url (offer status expected $status for buyer=${buyer_norm:-any}, last: ${val:-<none>})" >&2
    echo "$resp" | jq '.' >&2 || echo "$resp" >&2
    return 1
}

wait_for_rest_offer_field() {
    local url="$1" status="$2" buyer_address="$3" jq_field="$4" expected="$5" attempt resp val buyer_norm
    buyer_norm="$(normalize_hex_id "$buyer_address" 2>/dev/null || true)"
    for attempt in $(seq 1 60); do
        resp="$(curl -sS "$url" 2>/dev/null)" || resp='[]'
        val="$(echo "$resp" | jq -r \
            --arg s "$status" --arg b "$buyer_norm" \
            "(map(select(.status==\$s and ((.buyer_address // \"\") | ascii_downcase) == (\$b | ascii_downcase))) | sort_by(.updated_at) | reverse | .[0]${jq_field}) // empty" \
            2>/dev/null || true)"
        if [[ -n "$val" && "$val" == "$expected" ]]; then
            echo "$resp" >&2
            return 0
        fi
        sleep 1
    done
    echo "Timed out REST $url (offer field ${jq_field} expected $expected, last: ${val:-<none>})" >&2
    echo "$resp" | jq '.' >&2 || echo "$resp" >&2
    return 1
}

create_profile_for_address() {
    local sender="$1" display_name="$2" username="$3"
    local out digest profile_id mem
    [[ -n "$sender" && -n "$username" ]] || return 1
    require_session_fields USERNAME_REGISTRY_ID PROFILE_CONFIG_ID MEMORY_REGISTRY_ID \
        AI_CREDIT_CONFIG_ID CLOCK_ID || return 1
    require_hex_ids USERNAME_REGISTRY_ID PROFILE_CONFIG_ID MEMORY_REGISTRY_ID \
        AI_CREDIT_CONFIG_ID CLOCK_ID || return 1

    log_step "Creating profile for $sender username=$username"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$sender" profile create_profile \
        "@${USERNAME_REGISTRY_ID}" "@${PROFILE_CONFIG_ID}" "@${MEMORY_REGISTRY_ID}" \
        "@${AI_CREDIT_CONFIG_ID}" \
        "$(literal_move_string "${display_name:-Social E2E}")" \
        "$(literal_move_string "$username")" \
        '""' 'vector[]' 'vector[]' \
        "@${CLOCK_ID}")" || return 1

    digest="$(extract_tx_digest "$out")"
    [[ -n "$digest" ]] || digest="$(echo "$out" | grep -Eo '[A-Za-z0-9+/]{43,44}=' | tail -n1)"
    profile_id="$(extract_created_object_by_type "$digest" "profile::Profile")"
    [[ -n "$profile_id" ]] || profile_id="$(extract_created_object_by_type "$digest" "Profile")"
    [[ -n "$profile_id" ]] || profile_id="$(resolve_owned_profile_for_address "$sender")"
    [[ -n "$profile_id" ]] || {
        echo "create_profile did not yield Profile object id" >&2
        return 1
    }
    profile_id="$(normalize_hex_id "$profile_id")"
    mem="$(extract_created_object_by_type "$digest" "memory::MemoryAccount")"
    [[ -n "$mem" ]] || mem="$(extract_created_object_by_type "$digest" "MemoryAccount")"
    if [[ -n "$mem" ]]; then
        mem="$(normalize_hex_id "$mem")"
    fi
    printf '%s\n%s\n' "$profile_id" "${mem:-}"
}

ensure_joined_platform() {
    local member="${1:-}" out
    if [[ -z "$member" ]]; then
        member="$(resolve_myso_active_address)" || return 1
    fi
    member="$(normalize_hex_id "$member")" || return 1
    require_session_fields PLATFORM_REGISTRY_ID BLOCK_LIST_REGISTRY_ID PLATFORM_OBJECT_ID CLOCK_ID || return 1
    require_hex_ids PLATFORM_REGISTRY_ID BLOCK_LIST_REGISTRY_ID PLATFORM_OBJECT_ID CLOCK_ID || return 1
    log_step "Joining platform for $member"
    if out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$member" \
        --move-call "${PKG_SOCIAL}::platform::join_platform" \
        "$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" \
        "$(ptb_shared_ref "$BLOCK_LIST_REGISTRY_ID")" \
        "$(ptb_shared_ref "$PLATFORM_OBJECT_ID")" \
        "$(ptb_shared_ref "$CLOCK_ID")")"; then
        return 0
    fi
    if echo "$out" | grep -qE 'Abort Code: 3\b'; then
        log_step "Already joined platform — continuing"
        return 0
    fi
    return 1
}

create_test_platform() {
    local out digest platform_id ref_preg ref_pcfg ref_clk ref_cap
    require_session_fields PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID CLOCK_ID || return 1
    require_hex_ids PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_ADMIN_CAP_ID CLOCK_ID || return 1

    ref_preg="$(ptb_shared_ref "$PLATFORM_REGISTRY_ID")" || return 1
    ref_pcfg="$(ptb_shared_ref "$PLATFORM_CONFIG_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    ref_cap="$(ptb_shared_ref "$PLATFORM_ADMIN_CAP_ID")" || return 1

    log_step "Creating test platform (promo${SOCIAL_RUN_ID})"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --move-call "${PKG_SOCIAL}::platform::create_platform" \
        "$ref_preg" \
        "$ref_pcfg" \
        "$(literal_move_string "Promo Platform ${SOCIAL_RUN_ID}")" \
        "$(literal_move_string 'Post promotion E2E')" \
        "$(literal_move_string 'Test platform for promoted post flows')" \
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
        "$(literal_move_option_string "$SOCIAL_DEFAULT_PLATFORM_COVER_PHOTO_URL")" none none \
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

gql_platform_moderators_group_id() {
    local platform_id="$1" resp vars
    platform_id="$(normalize_hex_id "$platform_id")" || return 1
    vars="$(jq -nc --arg id "$platform_id" '{id: $id}')"
    resp="$(graphql_post \
        'query PlatformModerators($id: ID!) { platform(id: $id) { moderatorsGroupId } }' \
        "$vars")" || return 1
    echo "$resp" | jq -r '.data.platform.moderatorsGroupId // empty'
}

gql_promotion_snapshot() {
    local promotion_id="$1" resp vars
    promotion_id="$(normalize_hex_id "$promotion_id")" || return 1
    vars="$(jq -nc --arg id "$promotion_id" '{id: $id}')"
    resp="$(graphql_post \
        'query PromotionSnapshot($id: ID!) {
            promotion(id: $id) {
                promotionId
                views
                remainingBudget
                budget
                status
                viewsDetail(limit: 5) {
                    viewer
                    promotionId
                    viewDuration
                    paymentAmount
                    platformFee
                    ecosystemFee
                    recipientAmount
                }
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

wait_for_gql_promotion_views() {
    local promotion_id="$1" min_views="$2" attempt resp views
    for attempt in $(seq 1 60); do
        resp="$(gql_promotion_snapshot "$promotion_id" 2>/dev/null)" || resp='{}'
        views="$(echo "$resp" | jq -r '.data.promotion.views // 0' 2>/dev/null || echo 0)"
        if [[ -n "$views" && "$views" -ge "$min_views" ]]; then
            printf '%s' "$resp"
            return 0
        fi
        sleep 1
    done
    echo "Timed out waiting for promotion views >= $min_views (last: ${views:-0})" >&2
    return 1
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "social-runtime-common.sh is a library; source it from a runnable script, e.g.:" >&2
    echo "  source \"\${REPO_ROOT}/scripts/lib/social-runtime-common.sh\"" >&2
    exit 0
fi
