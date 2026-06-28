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
#   - CREATOR_ADDRESS and TIPPER_ADDRESS in client keystore (manual in menu [0]).
#   - Active CLI address acts as PoC oracle (script can update PoCConfig via PoCAdminCap).
#
# Session file: network.config/poc/poc-session.env (override POC_SESSION).
#
# Usage:
#   ./scripts/proof-of-creativity-runnable.sh --refresh-session
#   ./scripts/proof-of-creativity-runnable.sh --run-all
#   ASSUME_YES=1 ./scripts/proof-of-creativity-runnable.sh --run-all
#   ./scripts/proof-of-creativity-runnable.sh   # interactive menu
#
# Environment:
#   MYSO, CLIENT_CONFIG, GRAPHQL_URL, POC_SESSION, ASSUME_YES=1, DRY_RUN=1
#   POC_AUTO_REFRESH=1 (default), POC_NO_AUTO_REFRESH=1, POC_SKIP_USERNAME=1, POC_SKIP_DISPUTE=1
#   POC_INCLUDE_SPT=1, POC_FORCE_UPDATE_CONFIG=1, POC_RUN_ID

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

readonly DEFAULT_PKG_SOCIAL='0x00000000000000000000000000000000000000000000000000000000000050c1'
readonly DEFAULT_ORDERBOOK_PKG='0x000000000000000000000000000000000000000000000000000000000000b0c'
readonly DEFAULT_CLOCK='0x0000000000000000000000000000000000000000000000000000000000000006'
readonly DEFAULT_COIN_TYPE='0x2::myso::MYSO'
readonly DEFAULT_GAS_BUDGET='1000000000'
readonly DEFAULT_GRAPHQL_URL='http://127.0.0.1:9125/graphql'
readonly DEFAULT_TIP_AMOUNT='100000000'
readonly DEFAULT_VOTE_STAKE='1000000000'
readonly DEFAULT_DISPUTE_EVIDENCE='PoC runtime test dispute evidence'

# Session / chain object IDs (populated by GraphQL refresh or menu 0)
CLIENT_CONFIG=''
PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
ORDERBOOK_PACKAGE_ID="$DEFAULT_ORDERBOOK_PKG"
CLOCK_ID="$DEFAULT_CLOCK"
COIN_TYPE="$DEFAULT_COIN_TYPE"
GAS_BUDGET=''
GRAPHQL_URL="${GRAPHQL_URL:-$DEFAULT_GRAPHQL_URL}"

BOOTSTRAP_KEY_ID=''
ECOSYSTEM_TREASURY_ID=''
PLATFORM_REGISTRY_ID=''
PLATFORM_OBJECT_ID=''
USERNAME_REGISTRY_ID=''
BLOCK_LIST_REGISTRY_ID=''
MYDATA_REGISTRY_ID=''
SOCIAL_GRAPH_ID=''
TOKEN_REGISTRY_ID=''
POC_REGISTRY_ID=''
MESSAGE_REGISTRY_ID=''
MEMORY_REGISTRY_ID=''
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
MYDATA_ADMIN_CAP_ID=''
POOL_ADMIN_CAP_ID=''
GOVERNANCE_ECOSYSTEM_REGISTRY_ID=''
GOVERNANCE_POC_REGISTRY_ID=''

CREATOR_ADDRESS=''
TIPPER_ADDRESS=''
JOIN_REFERRER_ADDRESS=''
MEMORY_ACCOUNT_ID=''
TIPPER_MEMORY_ACCOUNT_ID=''

POC_RUN_ID="${POC_RUN_ID:-$(date +%s)}"
RUN_MODE=''
ASSUME_YES="${ASSUME_YES:-0}"

GQL_REFRESH_FILE=''

MANUAL_PRESERVE_KEYS=(
    CLIENT_CONFIG CREATOR_ADDRESS TIPPER_ADDRESS JOIN_REFERRER_ADDRESS GAS_BUDGET
    MEMORY_ACCOUNT_ID TIPPER_MEMORY_ACCOUNT_ID POC_RUN_ID
)

REQUIRED_RUN_ALL_KEYS=(
    POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID POC_USERNAME_BENEFICIARY_DIRECTORY_ID
    POC_ADMIN_CAP_ID POC_BENEFICIARY_ADMIN_CAP_ID USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID
    ECOSYSTEM_TREASURY_ID POST_CONFIG_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID
    BLOCK_LIST_REGISTRY_ID MYDATA_REGISTRY_ID CREATOR_ADDRESS TIPPER_ADDRESS
)

usage() {
    sed -n '2,26p' "$0" | sed 's/^# \?//'
}

session_state_save_path() {
    if [[ -n "${POC_SESSION:-}" ]]; then
        printf '%s' "$POC_SESSION"
    else
        printf '%s' "$REPO_ROOT/network.config/poc/poc-session.env"
    fi
}

apply_session_defaults() {
    [[ -n "${PKG_SOCIAL:-}" ]] || PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
    [[ -n "${ORDERBOOK_PACKAGE_ID:-}" ]] || ORDERBOOK_PACKAGE_ID="$DEFAULT_ORDERBOOK_PKG"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$DEFAULT_CLOCK"
    [[ -n "${COIN_TYPE:-}" ]] || COIN_TYPE="$DEFAULT_COIN_TYPE"
    [[ -n "${GAS_BUDGET:-}" ]] || GAS_BUDGET="$DEFAULT_GAS_BUDGET"
    if [[ -z "${CLIENT_CONFIG:-}" ]]; then
        if [[ -f "$PWD/network.config/client.yaml" ]]; then
            CLIENT_CONFIG="$PWD/network.config/client.yaml"
        elif [[ -f "$REPO_ROOT/network.config/client.yaml" ]]; then
            CLIENT_CONFIG="$REPO_ROOT/network.config/client.yaml"
        fi
    fi
}

load_session_state() {
    local paths p loaded=0
    paths=()
    [[ -n "${POC_SESSION:-}" ]] && paths+=("$POC_SESSION")
    paths+=("$PWD/network.config/poc/poc-session.env")
    paths+=("$REPO_ROOT/network.config/poc/poc-session.env")
    for p in "${paths[@]}"; do
        [[ -n "$p" && -f "$p" ]] || continue
        # shellcheck disable=SC1090
        source "$p"
        loaded=1
        echo "Loaded PoC session from: $p" >&2
        break
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
        for key in CLIENT_CONFIG PKG_SOCIAL ORDERBOOK_PACKAGE_ID CLOCK_ID COIN_TYPE GAS_BUDGET GRAPHQL_URL \
            BOOTSTRAP_KEY_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
            USERNAME_REGISTRY_ID BLOCK_LIST_REGISTRY_ID MYDATA_REGISTRY_ID SOCIAL_GRAPH_ID \
            TOKEN_REGISTRY_ID POC_REGISTRY_ID MESSAGE_REGISTRY_ID MEMORY_REGISTRY_ID \
            POOL_REGISTRY_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID POC_VAULT_DIRECTORY_ID \
            POC_USERNAME_BENEFICIARY_DIRECTORY_ID POST_CONFIG_ID SOCIAL_PROOF_TOKENS_CONFIG_ID \
            POC_CONFIG_ID MYDATA_CONFIG_ID SPOT_CONFIG_ID INSURANCE_CONFIG_ID ORDERBOOK_REGISTRY_ID \
            POC_ADMIN_CAP_ID POC_BENEFICIARY_ADMIN_CAP_ID MYDATA_ADMIN_CAP_ID POOL_ADMIN_CAP_ID \
            GOVERNANCE_ECOSYSTEM_REGISTRY_ID GOVERNANCE_POC_REGISTRY_ID \
            CREATOR_ADDRESS TIPPER_ADDRESS JOIN_REFERRER_ADDRESS MEMORY_ACCOUNT_ID TIPPER_MEMORY_ACCOUNT_ID \
            POC_RUN_ID; do
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

resolve_myso() {
    if [[ -n "${MYSO:-}" && -x "${MYSO}" ]]; then
        echo "$MYSO"
        return
    fi
    if command -v myso &>/dev/null; then
        command -v myso
        return
    fi
    for cand in "$REPO_ROOT/target/debug/myso" "$REPO_ROOT/target/release/myso"; do
        if [[ -x "$cand" ]]; then
            echo "$cand"
            return
        fi
    done
    echo ""
}

extra_gas_budget() {
    printf '%s\n' '--gas-budget' "${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}"
}

extra_dry() {
    if [[ "${DRY_RUN:-0}" == 1 ]]; then
        printf '%s\n' '--dry-run'
    fi
}

myso_client_base() {
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" ]] || { echo "myso binary not found" >&2; return 1; }
    [[ -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || { echo "Set CLIENT_CONFIG (menu 0)" >&2; return 1; }
    printf '%s' "$myso client --client.config $CLIENT_CONFIG"
}

resolve_myso_active_address() {
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" && -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || return 1
    "$myso" client --client.config "$CLIENT_CONFIG" active-address 2>/dev/null
}

literal_move_string() {
    local s=$1
    s="${s//\\/\\\\}"
    s="${s//\"/\\\"}"
    s="${s//$'\n'/\\n}"
    s="${s//$'\r'/}"
    printf "'\"%s\"'" "$s"
}

bytes_to_hex_arg() {
    python3 - "$1" <<'PY'
import sys
print("0x" + sys.argv[1].encode("utf-8").hex())
PY
}

invoke_ptb() {
    local myso cmd
    myso="$(resolve_myso)"
    [[ -n "$myso" ]] || return 1
    cmd=("$myso" client --client.config "$CLIENT_CONFIG" ptb)
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        "${cmd[@]}"
    else
        return 0
    fi
}

run_myso_call() {
    local module="$1" func="$2"
    shift 2
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" ]] || return 1
    local -a cmd
    cmd=("$myso" client --client.config "$CLIENT_CONFIG" call --package "$PKG_SOCIAL" --module "$module" --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
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
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" ]] || return 1
    local -a cmd
    cmd=("$myso" client --client.config "$CLIENT_CONFIG" call --package "$PKG_SOCIAL" --sender "$sender" \
        --module "$module" --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
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
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" ]] || return 1
    local -a cmd
    cmd=("$myso" client --client.config "$CLIENT_CONFIG" ptb --sender "$sender")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        "${cmd[@]}"
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
        echo "Run --refresh-session and menu [0] for CREATOR_ADDRESS / TIPPER_ADDRESS." >&2
        return 1
    fi
}

missing_required_keys() {
    local key missing=()
    for key in "${REQUIRED_RUN_ALL_KEYS[@]}"; do
        session_value_set "$key" || missing+=("$key")
    done
    if [[ ${#missing[@]} -gt 0 ]]; then
        printf '%s\n' "${missing[@]}"
        return 1
    fi
    return 0
}

graphql_post() {
    local query="$1"
    local vars="${2:-{}}"
    local body http_code resp
    body="$(jq -nc --arg q "$query" --argjson v "$vars" '{query: $q, variables: $v}')"
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
  usernameRegistry: objects(filter: { type: "0x50c1::profile::UsernameRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  blocklistRegistry: objects(filter: { type: "0x50c1::block_list::BlockListRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataRegistry: objects(filter: { type: "0x50c1::mydata::MyDataRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  socialGraph: objects(filter: { type: "0x50c1::social_graph::SocialGraph", ownerKind: SHARED }, first: 1) { nodes { address } }
  socialProofTokenRegistry: objects(filter: { type: "0x50c1::social_proof_tokens::TokenRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocRegistry: objects(filter: { type: "0x50c1::proof_of_creativity::PoCRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  messageRegistry: objects(filter: { type: "0x50c1::message::MessageRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  memoryRegistry: objects(filter: { type: "0x50c1::memory::MemoryRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataPoolRegistry: objects(filter: { type: "0x50c1::mydata::MyDataPoolRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  snapshotAnchorRegistry: objects(filter: { type: "0x50c1::mydata::SnapshotAnchorRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataClaimVault: objects(filter: { type: "0x50c1::mydata::MyDataClaimVault", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocVaultDirectory: objects(filter: { type: "0x50c1::poc_vault::PoCVaultDirectory", ownerKind: SHARED }, first: 1) { nodes { address } }
  postConfig: objects(filter: { type: "0x50c1::post::PostConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  sptConfig: objects(filter: { type: "0x50c1::social_proof_tokens::SocialProofTokensConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  pocConfig: objects(filter: { type: "0x50c1::proof_of_creativity::PoCConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
}'

readonly GQL_BATCH2='query MysocialGenesisObjectsBatch2 {
  mydataConfig: objects(filter: { type: "0x50c1::mydata::MyDataConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  spotConfig: objects(filter: { type: "0x50c1::social_proof_of_truth::SpotConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  insuranceConfig: objects(filter: { type: "0x50c1::insurance::InsuranceConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  orderbookRegistry: objects(filter: { type: "0xb0c::registry::Registry", ownerKind: SHARED }, first: 1) { nodes { address } }
  proofOfCreativityAdminCap: objects(filter: { type: "0x50c1::proof_of_creativity::PoCAdminCap" }, last: 1) { nodes { address } }
  mydataAdminCap: objects(filter: { type: "0x50c1::mydata::MyDataAdminCap" }, last: 1) { nodes { address } }
  mydataPoolAdminCap: objects(filter: { type: "0x50c1::mydata::MyDataPoolAdminCap" }, last: 1) { nodes { address } }
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
    for alias in bootstrapKey ecosystemTreasury platformRegistry platform usernameRegistry blocklistRegistry \
        mydataRegistry socialGraph socialProofTokenRegistry pocRegistry messageRegistry memoryRegistry \
        mydataPoolRegistry snapshotAnchorRegistry mydataClaimVault pocVaultDirectory \
        pocUsernameBeneficiaryDirectory postConfig sptConfig pocConfig mydataConfig spotConfig insuranceConfig \
        orderbookRegistry proofOfCreativityAdminCap pocBeneficiaryAdminCap mydataAdminCap mydataPoolAdminCap; do
        case "$alias" in
            bootstrapKey) env_key=BOOTSTRAP_KEY_ID ;;
            ecosystemTreasury) env_key=ECOSYSTEM_TREASURY_ID ;;
            platformRegistry) env_key=PLATFORM_REGISTRY_ID ;;
            platform) env_key=PLATFORM_OBJECT_ID ;;
            usernameRegistry) env_key=USERNAME_REGISTRY_ID ;;
            blocklistRegistry) env_key=BLOCK_LIST_REGISTRY_ID ;;
            mydataRegistry) env_key=MYDATA_REGISTRY_ID ;;
            socialGraph) env_key=SOCIAL_GRAPH_ID ;;
            socialProofTokenRegistry) env_key=TOKEN_REGISTRY_ID ;;
            pocRegistry) env_key=POC_REGISTRY_ID ;;
            messageRegistry) env_key=MESSAGE_REGISTRY_ID ;;
            memoryRegistry) env_key=MEMORY_REGISTRY_ID ;;
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
            pocBeneficiaryAdminCap) env_key=POC_BENEFICIARY_ADMIN_CAP_ID ;;
            mydataAdminCap) env_key=MYDATA_ADMIN_CAP_ID ;;
            mydataPoolAdminCap) env_key=POOL_ADMIN_CAP_ID ;;
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
        echo "GRAPHQL_URL=$GRAPHQL_URL"
        echo "PKG_SOCIAL=$DEFAULT_PKG_SOCIAL"
        echo "ORDERBOOK_PACKAGE_ID=$DEFAULT_ORDERBOOK_PKG"
        echo "CLOCK_ID=$DEFAULT_CLOCK"
        echo "COIN_TYPE=$DEFAULT_COIN_TYPE"
        for key in BOOTSTRAP_KEY_ID ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_OBJECT_ID \
            USERNAME_REGISTRY_ID BLOCK_LIST_REGISTRY_ID MYDATA_REGISTRY_ID SOCIAL_GRAPH_ID \
            TOKEN_REGISTRY_ID POC_REGISTRY_ID MESSAGE_REGISTRY_ID MEMORY_REGISTRY_ID \
            POOL_REGISTRY_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID POC_VAULT_DIRECTORY_ID \
            POC_USERNAME_BENEFICIARY_DIRECTORY_ID POST_CONFIG_ID SOCIAL_PROOF_TOKENS_CONFIG_ID \
            POC_CONFIG_ID MYDATA_CONFIG_ID SPOT_CONFIG_ID INSURANCE_CONFIG_ID ORDERBOOK_REGISTRY_ID \
            POC_ADMIN_CAP_ID POC_BENEFICIARY_ADMIN_CAP_ID MYDATA_ADMIN_CAP_ID POOL_ADMIN_CAP_ID \
            GOVERNANCE_ECOSYSTEM_REGISTRY_ID GOVERNANCE_POC_REGISTRY_ID \
            CLIENT_CONFIG CREATOR_ADDRESS TIPPER_ADDRESS JOIN_REFERRER_ADDRESS GAS_BUDGET \
            MEMORY_ACCOUNT_ID TIPPER_MEMORY_ACCOUNT_ID POC_RUN_ID; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    chmod 600 "$f" 2>/dev/null || true

    echo "GraphQL refresh summary:" >&2
    local req
    for req in "${REQUIRED_RUN_ALL_KEYS[@]}"; do
        if session_value_set "$req"; then
            echo "  OK  $req" >&2
        else
            echo "  MISS $req" >&2
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
        echo "Warning: PLATFORM_OBJECT_ID not found — create a platform via PTB or set manually in menu [0]." >&2
    fi
}

object_address_owner() {
    local myso object_id json
    myso="$(resolve_myso)"
    object_id="$1"
    [[ -n "$myso" && -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || return 1
    json="$("$myso" client --client.config "$CLIENT_CONFIG" object "$object_id" --json 2>/dev/null)" || return 1
    printf '%s' "$json" | jq -r '
        .. | objects | select(has("AddressOwner")) | .AddressOwner | .owner // .Owner // empty
        | if type == "string" then . elif type == "object" then .address // empty else empty end
    ' | head -n1
}

extract_tx_digest() {
    local out="$1"
    echo "$out" | grep -Eo 'Transaction Digest: [0-9a-zA-Z+/=_-]+' | head -n1 | awk '{print $3}' \
        || echo "$out" | grep -Eo '[A-Za-z0-9+/]{43,44}=' | head -n1
}

extract_created_object_by_type() {
    local digest="$1" type_substring="$2"
    local myso json
    myso="$(resolve_myso)"
    [[ -n "$digest" ]] || return 1
    json="$("$myso" client --client.config "$CLIENT_CONFIG" tx-block "$digest" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r --arg t "$type_substring" '
        .. | objects
        | select(.objectType? // .type? | tostring | contains($t))
        | .objectId // .reference?.objectId // empty
    ' | head -n1
}

resolve_shard_id_for_username() {
    local username="$1"
    local myso json idx shard_id
    myso="$(resolve_myso)"
    [[ -n "${POC_USERNAME_BENEFICIARY_DIRECTORY_ID:-}" ]] || return 1
    json="$("$myso" client --client.config "$CLIENT_CONFIG" object "$POC_USERNAME_BENEFICIARY_DIRECTORY_ID" --json 2>/dev/null)" || return 1
    idx="$(python3 - "$username" <<'PY'
import hashlib, sys
u = sys.argv[1].encode("utf-8")
h = hashlib.blake2b(u, digest_size=32).digest()
print(h[0] % 256)
PY
)"
    shard_id="$(echo "$json" | jq -r --argjson i "$idx" '
        .content.fields.shard_ids // .data.content.fields.shard_ids // empty
        | if type == "array" then .[$i] // .[$i|tostring] else empty end
        | if type == "string" then . else .id // .objectId // empty end
    ')"
    [[ -n "$shard_id" && "$shard_id" != "null" ]] || return 1
    printf '%s' "$shard_id"
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

resolve_gas_coin_for_address() {
    local addr="$1"
    local myso json
    myso="$(resolve_myso)"
    json="$("$myso" client --client.config "$CLIENT_CONFIG" gas --json --address "$addr" 2>/dev/null)" || return 1
    echo "$json" | jq -r '.[0].gasCoinId // .[0].coinObjectId // empty' | head -n1
}

resolve_memory_account_for_address() {
    local addr="$1"
    local myso json
    myso="$(resolve_myso)"
    json="$("$myso" client --client.config "$CLIENT_CONFIG" objects "$addr" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r '
        .[]? | select(.type? | tostring | contains("MemoryAccount")) | .data.objectId // .objectId // empty
    ' | head -n1
}

read_poc_config_oracle() {
    local myso json
    myso="$(resolve_myso)"
    [[ -n "${POC_CONFIG_ID:-}" ]] || return 1
    json="$("$myso" client --client.config "$CLIENT_CONFIG" object "$POC_CONFIG_ID" --json 2>/dev/null)" || return 1
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

preflight_oracle_and_config() {
    require_session_fields POC_CONFIG_ID POC_ADMIN_CAP_ID CLOCK_ID || return 1
    local oracle active current_oracle
    oracle="$(resolve_myso_active_address)" || { echo "Could not read active-address" >&2; return 1; }
    log_step "Preflight: oracle active-address = $oracle"
    current_oracle="$(read_poc_config_oracle)" || true

    if [[ "$current_oracle" != "$oracle" ]] || [[ "${POC_FORCE_UPDATE_CONFIG:-0}" == 1 ]]; then
        log_step "Updating PoCConfig (oracle=$oracle, short voting for runtime test)"
        SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity update_poc_config \
            --args "$POC_ADMIN_CAP_ID" "@${POC_CONFIG_ID}" "$oracle" \
            95 95 95 100 \
            5000000000 1000000000 100000000000 \
            3000 5000 10 10000 \
            100 500 3000 \
            0 10000 10000 500 \
            "@${CLOCK_ID}"
    fi
}

ensure_oracle_profile_and_memory() {
    local oracle mem
    oracle="$(resolve_myso_active_address)" || return 1
    mem="$(resolve_memory_account_for_address "$oracle")"
    if [[ -n "$mem" ]]; then
        MEMORY_ACCOUNT_ID="$mem"
        log_session_use "Oracle MemoryAccount" "$mem"
        return 0
    fi
    log_step "Creating oracle profile (no MemoryAccount found)"
    local uname body
    uname="pocoracle${POC_RUN_ID}"
    body="$(literal_move_string "PoC Oracle ${POC_RUN_ID}")"
    local uname_lit bio_lit
    uname_lit="$(literal_move_string "$uname")"
    bio_lit="$(literal_move_string "PoC runtime test oracle")"
    SKIP_CONFIRM_RUN=1 run_myso_call profile create_profile \
        --args "@${USERNAME_REGISTRY_ID}" "@${MEMORY_REGISTRY_ID}" \
        "$body" "$uname_lit" "$bio_lit" 0x 0x "@${CLOCK_ID}"
    mem="$(resolve_memory_account_for_address "$oracle")"
    [[ -n "$mem" ]] || { echo "Failed to resolve MemoryAccount after create_profile" >&2; return 1; }
    MEMORY_ACCOUNT_ID="$mem"
    save_session_state
}

ensure_tipper_memory_account() {
    [[ -n "${TIPPER_MEMORY_ACCOUNT_ID:-}" ]] && return 0
    local mem
    mem="$(resolve_memory_account_for_address "$TIPPER_ADDRESS")"
    if [[ -n "$mem" ]]; then
        TIPPER_MEMORY_ACCOUNT_ID="$mem"
        return 0
    fi
    echo "Warning: no MemoryAccount for TIPPER_ADDRESS; tip steps may fail." >&2
    return 0
}

create_post_poc_enabled() {
    local body_lit="$1"
    log_step "Creating post: $body_lit"
    SKIP_CONFIRM_RUN=1 invoke_ptb \
        --move-call "${PKG_SOCIAL}::post::create_post" \
        "@${USERNAME_REGISTRY_ID}" "@${PLATFORM_REGISTRY_ID}" "@${PLATFORM_OBJECT_ID}" \
        "@${BLOCK_LIST_REGISTRY_ID}" "@${POST_CONFIG_ID}" \
        "$body_lit" \
        none none none none none none none none \
        none some\(true\) none none \
        "@${MYDATA_REGISTRY_ID}" "@${MEMORY_ACCOUNT_ID}" "@${CLOCK_ID}"
}

analyze_post() {
    local post_id="$1" media_type="$2" score="$3" original_creator_arg="$4" \
        deriv_target="$5" embed_audio="$6" apply_explicit="$7" explicit_outcome="$8"
    log_step "analyze_and_update_post post=$post_id score=$score"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity analyze_and_update_post \
        --args "@${POC_CONFIG_ID}" "@${POC_REGISTRY_ID}" "@${POC_VAULT_DIRECTORY_ID}" "@${post_id}" \
        "$media_type" "$score" "$original_creator_arg" "$deriv_target" "$embed_audio" \
        "$apply_explicit" "$explicit_outcome" none none "@${CLOCK_ID}"
}

tip_post_as_tipper() {
    local post_id="$1" vault_id="$2" amount="$3"
    local coin mem
    coin="$(resolve_gas_coin_for_address "$TIPPER_ADDRESS")"
    [[ -n "$coin" ]] || { echo "No coin for tipper $TIPPER_ADDRESS" >&2; return 1; }
    mem="${TIPPER_MEMORY_ACCOUNT_ID:-}"
    [[ -n "$mem" ]] || mem="$(resolve_memory_account_for_address "$TIPPER_ADDRESS")"
    [[ -n "$mem" ]] || { echo "Tipper MemoryAccount required" >&2; return 1; }
    log_step "tip_post amount=$amount post=$post_id"
    SKIP_CONFIRM_RUN=1 invoke_ptb_as "$TIPPER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::post::tip_post<${COIN_TYPE}>" \
        "@${post_id}" "@${vault_id}" "@${coin}" "$amount" "@${mem}" "@${CLOCK_ID}"
}

run_username_beneficiary_flow() {
    require_session_fields POC_BENEFICIARY_ADMIN_CAP_ID POC_USERNAME_BENEFICIARY_DIRECTORY_ID \
        POC_VAULT_DIRECTORY_ID USERNAME_REGISTRY_ID MEMORY_REGISTRY_ID POC_CONFIG_ID \
        ECOSYSTEM_TREASURY_ID CREATOR_ADDRESS TIPPER_ADDRESS || return 1

    local ub_username identity_hash x_handle shard_id digest beneficiary_id vault_id beneficiary_addr
    ub_username="pocub${POC_RUN_ID}"
    identity_hash="0x$(printf 'id-%s' "$POC_RUN_ID" | xxd -p -c 256 | tr -d '\n')"
    x_handle="$(bytes_to_hex_arg "$ub_username")"
    local username_bytes
    username_bytes="$(bytes_to_hex_arg "$ub_username")"

    shard_id="$(resolve_shard_id_for_username "$ub_username")" || {
        echo "Could not resolve beneficiary shard for username $ub_username" >&2
        return 1
    }

    log_step "1a create_username_beneficiary username=$ub_username"
    local out
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity create_username_beneficiary \
        --args "$POC_BENEFICIARY_ADMIN_CAP_ID" \
        "@${POC_USERNAME_BENEFICIARY_DIRECTORY_ID}" "@${shard_id}" \
        "@${POC_VAULT_DIRECTORY_ID}" "@${USERNAME_REGISTRY_ID}" \
        "$username_bytes" 1 "$identity_hash" "$x_handle" "@${CLOCK_ID}")"
    digest="$(extract_tx_digest "$out")"
    beneficiary_id="$(extract_created_object_by_type "$digest" "PoCUsernameBeneficiary")"
    [[ -n "$beneficiary_id" ]] || { echo "Could not find PoCUsernameBeneficiary from tx" >&2; return 1; }

    beneficiary_addr="$(identity_beneficiary_address 1 "${identity_hash#0x}")"
    vault_id="$(extract_created_object_by_type "$digest" "PoCBeneficiaryVault")"
    if [[ -z "$vault_id" ]]; then
        local bjson myso_bin
        myso_bin="$(resolve_myso)"
        bjson="$("$myso_bin" client --client.config "$CLIENT_CONFIG" object "$beneficiary_id" --json 2>/dev/null)" || true
        vault_id="$(echo "$bjson" | jq -r '.. | objects | select(has("vault_id")) | .vault_id // empty' | head -n1)"
    fi

    log_step "1b claim_username_beneficiary wallet=$CREATOR_ADDRESS"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity claim_username_beneficiary \
        --args "@${POC_CONFIG_ID}" "@${POC_USERNAME_BENEFICIARY_DIRECTORY_ID}" "@${shard_id}" \
        "@${USERNAME_REGISTRY_ID}" "@${MEMORY_REGISTRY_ID}" "@${beneficiary_id}" \
        0x "$x_handle" \
        "$(bytes_to_hex_arg "Creator")" "$(bytes_to_hex_arg "bio")" 0x 0x \
        "$CREATOR_ADDRESS" "@${CLOCK_ID}"

    log_step "1c Fund username beneficiary vault via derivative post + tip"
    local fund_body fund_post digest2
    fund_body="$(literal_move_string "PoC fund vault ${POC_RUN_ID}")"
    out="$(create_post_poc_enabled "$fund_body")"
    digest2="$(extract_tx_digest "$out")"
    fund_post="$(extract_created_object_by_type "$digest2" "post::Post")"
    [[ -n "$fund_post" ]] || fund_post="$(extract_created_object_by_type "$digest2" "Post")"
    [[ -n "$fund_post" ]] || { echo "Could not find Post for vault funding" >&2; return 1; }

    analyze_post "$fund_post" 1 100 "some($beneficiary_addr)" 1 false false 0
    [[ -n "$vault_id" ]] || vault_id="$(extract_created_object_by_type "$digest2" "PoCBeneficiaryVault")"
    [[ -n "$vault_id" ]] || { echo "Could not resolve beneficiary vault id" >&2; return 1; }
    ensure_tipper_memory_account
    tip_post_as_tipper "$fund_post" "$vault_id" "$DEFAULT_TIP_AMOUNT"

    local referrer="${JOIN_REFERRER_ADDRESS:-$(resolve_myso_active_address)}"
    log_step "1d claim_username_beneficiary_vault_balance (creator)"
    SKIP_CONFIRM_RUN=1 run_myso_call_as "$CREATOR_ADDRESS" proof_of_creativity claim_username_beneficiary_vault_balance \
        --type-args "$COIN_TYPE" \
        --args "@${POC_CONFIG_ID}" "@${POC_USERNAME_BENEFICIARY_DIRECTORY_ID}" "@${beneficiary_id}" \
        "@${ECOSYSTEM_TREASURY_ID}" "@${vault_id}" "some($referrer)" "@${CLOCK_ID}"
}

run_post_poc_flow() {
    require_session_fields POC_CONFIG_ID POC_REGISTRY_ID POC_VAULT_DIRECTORY_ID \
        ECOSYSTEM_TREASURY_ID CREATOR_ADDRESS TIPPER_ADDRESS || return 1

    local out digest post1 post2 post3 vault_id beneficiary_addr
    beneficiary_addr="$CREATOR_ADDRESS"

    log_step "2a-2b Original post + analyze"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC original ${POC_RUN_ID}")")"
    digest="$(extract_tx_digest "$out")"
    post1="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post1" ]] || post1="$(extract_created_object_by_type "$digest" "Post")"
    analyze_post "$post1" 1 50 none 0 false false 0

    log_step "2c Derivative escrow post + analyze"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC derivative ${POC_RUN_ID}")")"
    digest="$(extract_tx_digest "$out")"
    post2="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post2" ]] || post2="$(extract_created_object_by_type "$digest" "Post")"
    analyze_post "$post2" 1 100 "some($beneficiary_addr)" 1 false false 0

    vault_id="$(extract_created_object_by_type "$digest" "PoCBeneficiaryVault")"
    [[ -n "$vault_id" ]] || {
        echo "Warning: vault not in tx effects; re-query after analyze may be needed" >&2
    }
    if [[ -n "$vault_id" ]]; then
        ensure_tipper_memory_account
        tip_post_as_tipper "$post2" "$vault_id" "$DEFAULT_TIP_AMOUNT"
        log_step "2e claim_beneficiary_vault_balance"
        SKIP_CONFIRM_RUN=1 run_myso_call_as "$beneficiary_addr" proof_of_creativity claim_beneficiary_vault_balance \
            --type-args "$COIN_TYPE" \
            --args "@${POC_CONFIG_ID}" "@${ECOSYSTEM_TREASURY_ID}" "@${vault_id}" \
            "some($(resolve_myso_active_address))" "@${CLOCK_ID}"
    fi

    log_step "2f Explicit royalty-free outcome post"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC royalty-free ${POC_RUN_ID}")")"
    digest="$(extract_tx_digest "$out")"
    post3="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post3" ]] || post3="$(extract_created_object_by_type "$digest" "Post")"
    analyze_post "$post3" 1 0 none 0 false true 4
}

run_spt_sync_flow() {
    require_session_fields TOKEN_REGISTRY_ID SOCIAL_PROOF_TOKENS_CONFIG_ID || return 1
    local out digest post_id pool_id
    log_step "3 SPT: create post + reservation pool + token"
    out="$(create_post_poc_enabled "$(literal_move_string "PoC SPT ${POC_RUN_ID}")")"
    digest="$(extract_tx_digest "$out")"
    post_id="$(extract_created_object_by_type "$digest" "post::Post")"
    [[ -n "$post_id" ]] || post_id="$(extract_created_object_by_type "$digest" "Post")"

    out="$(SKIP_CONFIRM_RUN=1 run_myso_call social_proof_tokens create_reservation_pool_for_post \
        --args "@${TOKEN_REGISTRY_ID}" "@${SOCIAL_PROOF_TOKENS_CONFIG_ID}" "@${post_id}")"
    digest="$(extract_tx_digest "$out")"
    pool_id="$(extract_created_object_by_type "$digest" "ReservationPool")"
    [[ -n "$pool_id" ]] || pool_id="$(extract_created_object_by_type "$digest" "TokenPool")"

    SKIP_CONFIRM_RUN=1 run_myso_call social_proof_tokens create_social_proof_token \
        --args "@${TOKEN_REGISTRY_ID}" "@${SOCIAL_PROOF_TOKENS_CONFIG_ID}" "@${pool_id}"

    log_step "3 analyze_and_update_post_sync_token_pool"
    SKIP_CONFIRM_RUN=1 run_myso_call proof_of_creativity analyze_and_update_post_sync_token_pool \
        --args "@${POC_CONFIG_ID}" "@${POC_REGISTRY_ID}" "@${TOKEN_REGISTRY_ID}" \
        "@${POC_VAULT_DIRECTORY_ID}" "@${post_id}" "@${pool_id}" \
        1 100 "some($CREATOR_ADDRESS)" 1 false false 0 none none "@${CLOCK_ID}"
}

run_dispute_flow() {
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
}

run_all_e2e() {
    maybe_auto_refresh_session
    require_session_fields "${REQUIRED_RUN_ALL_KEYS[@]}" || return 1
    preflight_oracle_and_config
    ensure_oracle_profile_and_memory

    if [[ "${POC_SKIP_USERNAME:-0}" != 1 ]]; then
        run_username_beneficiary_flow
    fi
    run_post_poc_flow
    if [[ "${POC_INCLUDE_SPT:-0}" == 1 ]]; then
        run_spt_sync_flow
    fi
    if [[ "${POC_SKIP_DISPUTE:-0}" != 1 ]]; then
        run_dispute_flow
    fi
    save_session_state
    log_step "PoC runtime E2E complete."
}

prompt_with_default() {
    local label="$1" default="$2" _read
    if [[ -n "$default" ]]; then
        read -r -p "${label} [${default}]: " _read || true
        printf '%s' "${_read:-$default}"
    else
        read -r -p "${label}: " _read
        printf '%s' "$_read"
    fi
}

menu_session_setup() {
    echo ""
    echo "=== Menu [0] Session setup (manual overrides) ==="
    CLIENT_CONFIG="$(prompt_with_default "CLIENT_CONFIG" "${CLIENT_CONFIG:-}")"
    GRAPHQL_URL="$(prompt_with_default "GRAPHQL_URL" "${GRAPHQL_URL:-$DEFAULT_GRAPHQL_URL}")"
    CREATOR_ADDRESS="$(prompt_with_default "CREATOR_ADDRESS (keystore)" "${CREATOR_ADDRESS:-}")"
    TIPPER_ADDRESS="$(prompt_with_default "TIPPER_ADDRESS (keystore)" "${TIPPER_ADDRESS:-}")"
    JOIN_REFERRER_ADDRESS="$(prompt_with_default "JOIN_REFERRER_ADDRESS (optional)" "${JOIN_REFERRER_ADDRESS:-}")"
    GAS_BUDGET="$(prompt_with_default "GAS_BUDGET" "${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}")"
    read -r -p "Run GraphQL refresh now? [Y/n] " do_refresh
    if [[ -z "${do_refresh:-}" || "${do_refresh}" == [yY]* ]]; then
        refresh_poc_session_from_graphql
        load_session_state
        CLIENT_CONFIG="$(prompt_with_default "CLIENT_CONFIG (after refresh)" "${CLIENT_CONFIG:-}")"
        CREATOR_ADDRESS="$(prompt_with_default "CREATOR_ADDRESS" "${CREATOR_ADDRESS:-}")"
        TIPPER_ADDRESS="$(prompt_with_default "TIPPER_ADDRESS" "${TIPPER_ADDRESS:-}")"
    fi
    save_session_state
    echo "Session saved."
}

show_menu() {
    echo ""
    echo "=== PoC Runtime Test Menu ==="
    echo " 0) Session setup (manual + optional GraphQL refresh)"
    echo " R) Refresh poc-session.env from GraphQL"
    echo " 1) Run full E2E (--run-all)"
    echo " 2) Username beneficiary flow only"
    echo " 3) Post PoC flow only"
    echo " 4) Dispute flow only"
    echo " 5) SPT sync flow only"
    echo " h) Help"
    echo " q) Quit"
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) menu_session_setup ;;
        [Rr]) refresh_poc_session_from_graphql; load_session_state ;;
        1) run_all_e2e ;;
        2) maybe_auto_refresh_session; run_username_beneficiary_flow ;;
        3) maybe_auto_refresh_session; preflight_oracle_and_config; ensure_oracle_profile_and_memory; run_post_poc_flow ;;
        4) maybe_auto_refresh_session; preflight_oracle_and_config; ensure_oracle_profile_and_memory; run_dispute_flow ;;
        5) maybe_auto_refresh_session; preflight_oracle_and_config; ensure_oracle_profile_and_memory; run_spt_sync_flow ;;
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
            --refresh-session) RUN_MODE=refresh; shift ;;
            --no-auto-refresh) POC_NO_AUTO_REFRESH=1; shift ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    load_session_state

    case "${RUN_MODE:-}" in
        refresh) refresh_poc_session_from_graphql; exit 0 ;;
        run_all) run_all_e2e; exit 0 ;;
        "") show_menu ;;
        *) echo "Unknown run mode: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
