#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Runtime E2E helper for contra::contra confidential transfers via `myso client ptb`.
#
# Prerequisites:
#   - Local myso node + indexer GraphQL at http://127.0.0.1:9125/graphql
#   - Contra genesis: shared TokenRegistry + AccountRegistry (0xc1fe::contra)
#   - CoinCreationAdminCap on the active address (e.g. social bootstrap claim)
#   - `myso`, `curl`, `jq`, `python3`, `cargo` on PATH
#   - Built `build_transfer_bundle` (auto via `cargo run -p contra-crypto-fixtures`)
#
# Session file: network.config/contra/contra-session.env
#
# Usage:
#   ./scripts/contra-runnable.sh --refresh-session
#   ./scripts/contra-runnable.sh --publish-test-coin
#   ./scripts/contra-runnable.sh --setup-token
#   ./scripts/contra-runnable.sh --setup-accounts
#   ./scripts/contra-runnable.sh --wrap-flow
#   ./scripts/contra-runnable.sh --transfer-flow
#   ./scripts/contra-runnable.sh --unwrap-flow
#   ASSUME_YES=1 ./scripts/contra-runnable.sh --run-all
#   ./scripts/contra-runnable.sh   # interactive menu
#
# Environment:
#   ASSUME_YES=1, DRY_RUN=1, GAS_BUDGET, MYSO, CONTRA_CRYPTO, GRAPHQL_URL

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CONTRA_E2E_PKG="${REPO_ROOT}/crates/myso-framework/packages/contra-e2e"

readonly DEFAULT_PKG_CONTRA='0x000000000000000000000000000000000000000000000000000000000000c1fe'
readonly DEFAULT_DENY_LIST='0x0000000000000000000000000000000000000000000000000000000000000403'
readonly DEFAULT_CLOCK='0x0000000000000000000000000000000000000000000000000000000000000006'
readonly DEFAULT_COIN_REGISTRY='0x000000000000000000000000000000000000000000000000000000000000000c'
readonly DEFAULT_GAS_BUDGET='1000000000'
readonly DEFAULT_MINT_AMOUNT='100'
readonly DEFAULT_TRANSFER_AMOUNT='50'
readonly DEFAULT_UNWRAP_AMOUNT='30'

GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
GAS_BUDGET="${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}"
ASSUME_YES="${ASSUME_YES:-0}"

PKG_CONTRA="$DEFAULT_PKG_CONTRA"
DENY_LIST_ID="$DEFAULT_DENY_LIST"
CLOCK_ID="$DEFAULT_CLOCK"
COIN_REGISTRY_ID="$DEFAULT_COIN_REGISTRY"
TOKEN_REGISTRY_ID=''
ACCOUNT_REGISTRY_ID=''
TEST_COIN_PKG=''
COIN_TYPE=''
TREASURY_CAP_ID=''
MANAGEMENT_CAP_ID=''
CONFIDENTIAL_TOKEN_ID=''
POOL_ID=''
SENDER_ADDRESS=''
RECEIVER_ADDRESS=''
SENDER_ACCOUNT_ID=''
RECEIVER_ACCOUNT_ID=''
SENDER_SK='12345'
RECEIVER_SK='67890'
SENDER_PK=''
RECEIVER_PK=''
TRANSFER_BLINDING='32533'
NEW_BALANCE_BLINDING='10097'
SENDER_BALANCE='100'
MINT_AMOUNT="$DEFAULT_MINT_AMOUNT"

SESSION_KEYS=(
    PKG_CONTRA DENY_LIST_ID CLOCK_ID COIN_REGISTRY_ID GAS_BUDGET TOKEN_REGISTRY_ID ACCOUNT_REGISTRY_ID
    TEST_COIN_PKG COIN_TYPE TREASURY_CAP_ID MANAGEMENT_CAP_ID CONFIDENTIAL_TOKEN_ID POOL_ID
    SENDER_ADDRESS RECEIVER_ADDRESS SENDER_ACCOUNT_ID RECEIVER_ACCOUNT_ID
    SENDER_SK RECEIVER_SK SENDER_PK RECEIVER_PK
    TRANSFER_BLINDING NEW_BALANCE_BLINDING SENDER_BALANCE MINT_AMOUNT
)

usage() {
    sed -n '2,26p' "$0" | sed 's/^# \?//'
}

session_state_save_path() {
    printf '%s' "$REPO_ROOT/network.config/contra/contra-session.env"
}

load_session_state() {
    local p
    p="$(session_state_save_path)"
    if [[ -f "$p" ]]; then
        # shellcheck disable=SC1090
        source "$p"
        echo "Loaded contra session from: $p" >&2
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
        echo "# Contra runtime session — scripts/contra-runnable.sh"
        for key in "${SESSION_KEYS[@]}"; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    umask "$old_umask"
    echo "Saved session to: $f" >&2
}

apply_session_defaults() {
    [[ -n "${PKG_CONTRA:-}" ]] || PKG_CONTRA="$DEFAULT_PKG_CONTRA"
    [[ -n "${DENY_LIST_ID:-}" ]] || DENY_LIST_ID="$DEFAULT_DENY_LIST"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$DEFAULT_CLOCK"
    [[ -n "${COIN_REGISTRY_ID:-}" ]] || COIN_REGISTRY_ID="$DEFAULT_COIN_REGISTRY"
    [[ -n "${GAS_BUDGET:-}" ]] || GAS_BUDGET="$DEFAULT_GAS_BUDGET"
    [[ -n "${SENDER_SK:-}" ]] || SENDER_SK='12345'
    [[ -n "${RECEIVER_SK:-}" ]] || RECEIVER_SK='67890'
    [[ -n "${TRANSFER_BLINDING:-}" ]] || TRANSFER_BLINDING='32533'
    [[ -n "${NEW_BALANCE_BLINDING:-}" ]] || NEW_BALANCE_BLINDING='10097'
    [[ -n "${SENDER_BALANCE:-}" ]] || SENDER_BALANCE='100'
    [[ -n "${MINT_AMOUNT:-}" ]] || MINT_AMOUNT="$DEFAULT_MINT_AMOUNT"
}

confirm_run() {
    if [[ "${ASSUME_YES:-0}" == 1 ]]; then
        return 0
    fi
    read -r -p "Execute this command? [y/N] " ans
    [[ "${ans:-}" == [yY]* ]]
}

extra_gas_budget() {
    printf '%s\n' '--gas-budget' "$GAS_BUDGET"
}

extra_dry() {
    if [[ "${DRY_RUN:-0}" == 1 ]]; then
        printf '%s\n' '--dry-run'
    fi
}

resolve_myso() {
    if [[ -n "${MYSO:-}" ]]; then
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

resolve_contra_crypto() {
    if [[ -n "${CONTRA_CRYPTO:-}" && -x "${CONTRA_CRYPTO}" ]]; then
        echo "$CONTRA_CRYPTO"
        return
    fi
    for cand in \
        "$REPO_ROOT/target/debug/build_transfer_bundle" \
        "$REPO_ROOT/target/release/build_transfer_bundle"; do
        if [[ -x "$cand" ]]; then
            echo "$cand"
            return
        fi
    done
    echo "cargo run -p contra-crypto-fixtures --bin build_transfer_bundle --quiet --"
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
    if confirm_run; then
        "${cmd[@]}" >&2
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
        out="$("${cmd[@]}" 2>&1)" || { echo "$out" >&2; return 1; }
        echo "$out" >&2
        printf '%s' "$out"
    fi
}

graphql_post() {
    local query="$1"
    curl -sS -X POST "$GRAPHQL_URL" \
        -H 'Content-Type: application/json' \
        -d "$(jq -nc --arg q "$query" '{query: $q}')"
}

gql_shared_object() {
    local json="$1" alias="$2"
    echo "$json" | jq -r ".data.${alias}.nodes[0].address // empty"
}

refresh_contra_session_from_graphql() {
    command -v curl >/dev/null || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null || { echo "jq required" >&2; return 1; }
    log_step "Refreshing contra session from GraphQL ($GRAPHQL_URL)"
    local q j
    q='query ContraSessionObjects {
  tokenRegistry: objects(filter: { type: "0xc1fe::contra::TokenRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  accountRegistry: objects(filter: { type: "0xc1fe::contra::AccountRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
}'
    j="$(graphql_post "$q")"
    TOKEN_REGISTRY_ID="$(gql_shared_object "$j" tokenRegistry)"
    ACCOUNT_REGISTRY_ID="$(gql_shared_object "$j" accountRegistry)"
    apply_session_defaults
    save_session_state
    echo "  TOKEN_REGISTRY_ID=${TOKEN_REGISTRY_ID:-<missing>}" >&2
    echo "  ACCOUNT_REGISTRY_ID=${ACCOUNT_REGISTRY_ID:-<missing>}" >&2
}

log_step() {
    echo "" >&2
    echo ">>> $*" >&2
}

normalize_hex_id() {
    local id="$1"
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

resolve_account_id() {
    local owner="$1" crypto out
    [[ -n "${ACCOUNT_REGISTRY_ID:-}" ]] || { echo "Need ACCOUNT_REGISTRY_ID" >&2; return 1; }
    crypto="$(resolve_contra_crypto)"
    if [[ "$crypto" == cargo* ]]; then
        out="$($crypto account-id --registry-id "$ACCOUNT_REGISTRY_ID" --owner "$owner")"
    else
        out="$("$crypto" account-id --registry-id "$ACCOUNT_REGISTRY_ID" --owner "$owner")"
    fi
    echo "$out" | jq -r '.account_id // empty'
}

require_cmd() {
    local c
    for c in "$@"; do
        command -v "$c" >/dev/null || { echo "Missing required command: $c" >&2; return 1; }
    done
}

preflight() {
    require_cmd myso curl jq python3 cargo
    local myso_bin
    myso_bin="$(resolve_myso)"
    [[ -n "$myso_bin" ]] || { echo "myso not found" >&2; return 1; }
    export PATH="$(dirname "$myso_bin"):$PATH"
}

ensure_faucet() {
    log_step "Faucet for active address"
    myso client faucet >/dev/null 2>&1 || myso client faucet
}

ensure_receiver_address() {
    if [[ -n "${RECEIVER_ADDRESS:-}" ]]; then
        return 0
    fi
    local addrs active
    active="$(myso client active-address)"
    addrs="$(myso keytool list | jq -r '.[].mysoAddress' 2>/dev/null || true)"
    RECEIVER_ADDRESS="$(echo "$addrs" | grep -v "^${active}$" | head -n1 || true)"
    if [[ -z "$RECEIVER_ADDRESS" ]]; then
        log_step "Creating receiver key in keystore"
        RECEIVER_ADDRESS="$(myso client new-address ed25519 | jq -r '.address // .mysoAddress // empty')"
    fi
    [[ -n "$RECEIVER_ADDRESS" ]] || { echo "Could not resolve receiver address" >&2; return 1; }
    SENDER_ADDRESS="${SENDER_ADDRESS:-$active}"
    save_session_state
}

ensure_key_material() {
    local crypto out dir
    crypto="$(resolve_contra_crypto)"
    dir="$(mktemp -d)"
    if [[ "$crypto" == cargo* ]]; then
        $crypto keygen --secret "$SENDER_SK" --output "$dir/sender.json"
        $crypto keygen --secret "$RECEIVER_SK" --output "$dir/receiver.json"
    else
        "$crypto" keygen --secret "$SENDER_SK" --output "$dir/sender.json"
        "$crypto" keygen --secret "$RECEIVER_SK" --output "$dir/receiver.json"
    fi
    SENDER_PK="$(jq -r '.public_key' "$dir/sender.json")"
    RECEIVER_PK="$(jq -r '.public_key' "$dir/receiver.json")"
    rm -rf "$dir"
    save_session_state
}

extract_created_id() {
    local json="$1" type_substr="$2"
    echo "$json" | jq -r --arg t "$type_substr" '
        .objectChanges[]? | select(.type == "created" and (.objectType | contains($t))) | .objectId
    ' | head -n1
}

extract_published_package() {
    local json="$1"
    echo "$json" | jq -r '.objectChanges[]? | select(.type == "published") | .packageId' | head -n1
}

resolve_coin_creation_admin_cap() {
    local owner="$1" json cap
    json="$(myso client objects --json --address "$owner" 2>/dev/null)" || return 1
    cap="$(echo "$json" | jq -r '.[] | select(.data.type | contains("CoinCreationAdminCap")) | .data.objectId' | head -n1)"
    [[ -n "$cap" ]] || {
        echo "No CoinCreationAdminCap for $owner (claim via social bootstrap first)" >&2
        return 1
    }
    printf '%s' "$cap"
}

publish_test_coin() {
    preflight
    ensure_faucet
    local admin_cap ref_reg
    admin_cap="$(resolve_coin_creation_admin_cap "$(myso client active-address)")" || return 1
    ref_reg="$(ptb_shared_ref "$COIN_REGISTRY_ID")" || return 1
    log_step "Publishing contra-e2e test coin package"
    local pub_out pkg
    pub_out="$(myso client publish "$CONTRA_E2E_PKG" --json 2>&1)" || { echo "$pub_out" >&2; return 1; }
    pkg="$(extract_published_package "$pub_out")"
    [[ -n "$pkg" ]] || { echo "$pub_out" >&2; echo "Failed to parse publish output" >&2; return 1; }
    TEST_COIN_PKG="$pkg"
    COIN_TYPE="${TEST_COIN_PKG}::test_coin::TEST_COIN"
    log_step "Registering $COIN_TYPE on CoinRegistry"
    local out treasury
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --assign treasury \
        --move-call "${TEST_COIN_PKG}::test_coin::create" \
        --args "$ref_reg" "$admin_cap" \
        --transfer-object treasury @"$(myso client active-address)")"
    [[ -n "$out" ]] || return 1
    treasury="$(echo "$out" | jq -r '.objectChanges[]? | select(.type == "created" and (.objectType | contains("TreasuryCap"))) | .objectId' | head -n1)"
    [[ -n "$treasury" ]] || treasury="$(myso client objects --json | jq -r --arg t "$COIN_TYPE" '.[] | select(.data.type | contains("TreasuryCap") and contains($t)) | .data.objectId' | head -n1)"
    TREASURY_CAP_ID="$treasury"
    [[ -n "$TREASURY_CAP_ID" ]] || { echo "$out" >&2; echo "Failed to resolve TreasuryCap" >&2; return 1; }
    save_session_state
    echo "  TEST_COIN_PKG=$TEST_COIN_PKG" >&2
    echo "  TREASURY_CAP_ID=$TREASURY_CAP_ID" >&2
    echo "  COIN_TYPE=$COIN_TYPE" >&2
}

setup_confidential_token() {
    [[ -n "${TOKEN_REGISTRY_ID:-}" && -n "${TREASURY_CAP_ID:-}" && -n "${COIN_TYPE:-}" ]] || {
        echo "Need TOKEN_REGISTRY_ID, TREASURY_CAP_ID, COIN_TYPE (run --refresh-session and --publish-test-coin)" >&2
        return 1
    }
    preflight
    ensure_faucet
    log_step "Creating confidential token for $COIN_TYPE"
    local ref_reg
    ref_reg="$(ptb_shared_ref "$TOKEN_REGISTRY_ID")" || return 1
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_capture \
        --assign ct_mgmt \
        --move-call "${PKG_CONTRA}::contra::new_confidential_token" --type-args "$COIN_TYPE" \
        --args "$ref_reg" "$TREASURY_CAP_ID" 'vector[]' \
        --move-call "${PKG_CONTRA}::contra::share_confidential_token" --type-args "$COIN_TYPE" \
        --args ct_mgmt.0 \
        --transfer-object ct_mgmt.1 @"$(myso client active-address)")"
    [[ -n "$out" ]] || return 0
    ct="$(echo "$out" | jq -r '.objectChanges[]? | select(.type == "created" and (.objectType | contains("ConfidentialToken"))) | .objectId' | head -n1)"
    mgmt="$(echo "$out" | jq -r '.objectChanges[]? | select(.type == "created" and (.objectType | contains("ManagementCap"))) | .objectId' | head -n1)"
    pool="$(echo "$out" | jq -r '.objectChanges[]? | select(.type == "created" and (.objectType | contains("Pool"))) | .objectId' | head -n1)"
    CONFIDENTIAL_TOKEN_ID="${ct:-$CONFIDENTIAL_TOKEN_ID}"
    MANAGEMENT_CAP_ID="${mgmt:-$MANAGEMENT_CAP_ID}"
    POOL_ID="${pool:-$POOL_ID}"
    save_session_state
}

setup_accounts() {
    [[ -n "${ACCOUNT_REGISTRY_ID:-}" && -n "${COIN_TYPE:-}" && -n "${CONFIDENTIAL_TOKEN_ID:-}" ]] || {
        echo "Need ACCOUNT_REGISTRY_ID, COIN_TYPE, CONFIDENTIAL_TOKEN_ID" >&2
        return 1
    }
    preflight
    ensure_faucet
    ensure_receiver_address
    ensure_key_material

    local ref_reg ref_ct
    ref_reg="$(ptb_shared_ref "$ACCOUNT_REGISTRY_ID")" || return 1
    ref_ct="$(ptb_shared_ref "$CONFIDENTIAL_TOKEN_ID")" || return 1

    log_step "Creating + registering sender account ($SENDER_ADDRESS)"
    invoke_ptb \
        --assign sender_acct \
        --move-call "${PKG_CONTRA}::contra::new_account" \
        --args "$ref_reg" "$SENDER_ADDRESS" \
        --move-call "${PKG_CONTRA}::contra::share_account" --args sender_acct \
        --assign sender_auth \
        --move-call "${PKG_CONTRA}::contra::authorize_as_sender" --type-args "$COIN_TYPE" \
        --args "$ref_ct" \
        --move-call "${PKG_CONTRA}::contra::register" --type-args "$COIN_TYPE" \
        --args sender_acct sender_auth "$ref_ct" "$SENDER_PK" 'option::none()'

    log_step "Creating + registering receiver account ($RECEIVER_ADDRESS)"
    myso client switch --address "$RECEIVER_ADDRESS" >/dev/null
    ensure_faucet
    invoke_ptb \
        --assign receiver_acct \
        --move-call "${PKG_CONTRA}::contra::new_account" \
        --args "$ref_reg" "$RECEIVER_ADDRESS" \
        --move-call "${PKG_CONTRA}::contra::share_account" --args receiver_acct \
        --assign receiver_auth \
        --move-call "${PKG_CONTRA}::contra::authorize_as_sender" --type-args "$COIN_TYPE" \
        --args "$ref_ct" \
        --move-call "${PKG_CONTRA}::contra::register" --type-args "$COIN_TYPE" \
        --args receiver_acct receiver_auth "$ref_ct" "$RECEIVER_PK" 'option::none()'

    myso client switch --address "$SENDER_ADDRESS" >/dev/null

    log_step "Deriving shared Account object IDs"
    SENDER_ACCOUNT_ID="$(resolve_account_id "$SENDER_ADDRESS")"
    RECEIVER_ACCOUNT_ID="$(resolve_account_id "$RECEIVER_ADDRESS")"
    [[ -n "$SENDER_ACCOUNT_ID" && -n "$RECEIVER_ACCOUNT_ID" ]] || {
        echo "Failed to derive account object IDs" >&2
        return 1
    }
    save_session_state
}

wrap_flow() {
    [[ -n "${SENDER_ACCOUNT_ID:-}" && -n "${POOL_ID:-}" && -n "${TREASURY_CAP_ID:-}" ]] || {
        echo "Need SENDER_ACCOUNT_ID, POOL_ID, TREASURY_CAP_ID" >&2
        return 1
    }
    preflight
    myso client switch --address "$SENDER_ADDRESS" >/dev/null
    ensure_faucet
    local ref_acct ref_ct ref_deny ref_pool
    ref_acct="$(ptb_shared_ref "$SENDER_ACCOUNT_ID")" || return 1
    ref_ct="$(ptb_shared_ref "$CONFIDENTIAL_TOKEN_ID")" || return 1
    ref_deny="$(ptb_shared_ref "$DENY_LIST_ID")" || return 1
    ref_pool="$(ptb_shared_ref "$POOL_ID")" || return 1
    log_step "Minting $MINT_AMOUNT and wrapping into confidential balance"
    invoke_ptb \
        --assign coins \
        --move-call "0x2::coin::mint" --type-args "$COIN_TYPE" \
        --args "$TREASURY_CAP_ID" "$MINT_AMOUNT" \
        --assign auth \
        --move-call "${PKG_CONTRA}::contra::authorize_as_sender" --type-args "$COIN_TYPE" \
        --args "$ref_ct" \
        --move-call "${PKG_CONTRA}::contra::wrap" --type-args "$COIN_TYPE" \
        --args "$ref_acct" auth "$ref_ct" "$ref_deny" "$ref_pool" coins 'vector[]' \
        --move-call "${PKG_CONTRA}::contra::merge" --type-args "$COIN_TYPE" \
        --args "$ref_acct" auth
}

json_parts_to_move_vector() {
    local json_file="$1" field="$2"
    python3 - "$json_file" "$field" <<'PY'
import json, sys
data = json.load(open(sys.argv[1]))
parts = data[sys.argv[2]]
print("vector[" + ", ".join(p for p in parts) + "]")
PY
}

json_range_proofs_to_move_vector() {
    local json_file="$1"
    python3 - "$json_file" <<'PY'
import json, sys
data = json.load(open(sys.argv[1]))
print("vector[" + ", ".join(data["range_proofs"]) + "]")
PY
}

json_consistency_proofs_to_move_vector() {
    local json_file="$1"
    python3 - "$json_file" <<'PY'
import json, sys
data = json.load(open(sys.argv[1]))
items = []
for proof in data["well_formed_consistency_proofs"]:
    inner = "vector[" + ", ".join(proof) + "]"
    items.append(inner)
print("vector[" + ", ".join(items) + "]")
PY
}

json_singleton_consistency_to_move_vector() {
    local json_file="$1"
    python3 - "$json_file" <<'PY'
import json, sys
data = json.load(open(sys.argv[1]))
proof = data["well_formed_consistency_proof_parts"]
inner = "vector[" + ", ".join(proof) + "]"
print("vector[" + inner + "]")
PY
}

build_transfer_bundle_file() {
    local out="$1"
    local crypto
    crypto="$(resolve_contra_crypto)"
    if [[ "$crypto" == cargo* ]]; then
        $crypto transfer \
            --sender-account-id "$SENDER_ACCOUNT_ID" \
            --coin-type "$COIN_TYPE" \
            --sender-sk "$SENDER_SK" \
            --receiver-pk "$RECEIVER_PK" \
            --transfer-amount "$DEFAULT_TRANSFER_AMOUNT" \
            --sender-balance "$SENDER_BALANCE" \
            --transfer-blinding "$TRANSFER_BLINDING" \
            --new-balance-blinding "$NEW_BALANCE_BLINDING" \
            --output "$out"
    else
        "$crypto" transfer \
            --sender-account-id "$SENDER_ACCOUNT_ID" \
            --coin-type "$COIN_TYPE" \
            --sender-sk "$SENDER_SK" \
            --receiver-pk "$RECEIVER_PK" \
            --transfer-amount "$DEFAULT_TRANSFER_AMOUNT" \
            --sender-balance "$SENDER_BALANCE" \
            --transfer-blinding "$TRANSFER_BLINDING" \
            --new-balance-blinding "$NEW_BALANCE_BLINDING" \
            --output "$out"
    fi
}

transfer_flow() {
    [[ -n "${SENDER_ACCOUNT_ID:-}" && -n "${RECEIVER_ACCOUNT_ID:-}" && -n "${COIN_TYPE:-}" ]] || {
        echo "Need SENDER_ACCOUNT_ID, RECEIVER_ACCOUNT_ID, COIN_TYPE" >&2
        return 1
    }
    preflight
    ensure_key_material
    myso client switch --address "$SENDER_ADDRESS" >/dev/null
    local bundle recv_parts sender_parts new_parts consistency range_proofs total_parts balance_parts
    bundle="$(mktemp)"
    build_transfer_bundle_file "$bundle"
    recv_parts="$(json_parts_to_move_vector "$bundle" receiver_amount_parts)"
    sender_parts="$(json_parts_to_move_vector "$bundle" sender_amount_parts)"
    new_parts="$(json_parts_to_move_vector "$bundle" new_balance_parts)"
    consistency="$(json_consistency_proofs_to_move_vector "$bundle")"
    range_proofs="$(json_range_proofs_to_move_vector "$bundle")"
    total_parts="$(json_parts_to_move_vector "$bundle" total_consistency_proof_parts)"
    balance_parts="$(json_parts_to_move_vector "$bundle" balance_proof_parts)"
    local ref_sender ref_ct ref_deny ref_receiver
    ref_sender="$(ptb_shared_ref "$SENDER_ACCOUNT_ID")" || return 1
    ref_ct="$(ptb_shared_ref "$CONFIDENTIAL_TOKEN_ID")" || return 1
    ref_deny="$(ptb_shared_ref "$DENY_LIST_ID")" || return 1
    ref_receiver="$(ptb_shared_ref "$RECEIVER_ACCOUNT_ID")" || return 1
    log_step "Confidential transfer ($DEFAULT_TRANSFER_AMOUNT) sender -> receiver"
    invoke_ptb \
        --assign recv_amt \
        --move-call "${PKG_CONTRA}::decode::encrypted_amount" --args "$recv_parts" \
        --assign sender_amt \
        --move-call "${PKG_CONTRA}::decode::encrypted_amount" --args "$sender_parts" \
        --assign new_bal \
        --move-call "${PKG_CONTRA}::decode::encrypted_amount" --args "$new_parts" \
        --assign wf_proof \
        --move-call "${PKG_CONTRA}::encrypted_amount::new_well_formed_proof" \
        --args "$range_proofs" "$consistency" \
        --assign total_proof \
        --move-call "${PKG_CONTRA}::decode::elgamal_proof" --args "$total_parts" \
        --assign balance_proof \
        --move-call "${PKG_CONTRA}::decode::ddh_proof" --args "$balance_parts" \
        --assign auth \
        --move-call "${PKG_CONTRA}::contra::authorize_as_sender" --type-args "$COIN_TYPE" \
        --args "$ref_ct" \
        --assign batch \
        --move-call "${PKG_CONTRA}::contra::batched_transfer" --type-args "$COIN_TYPE" \
        --args "$ref_sender" auth "$ref_ct" "$ref_deny" \
        "vector[$RECEIVER_PK]" "vector[recv_amt]" wf_proof "vector[sender_amt]" total_proof new_bal balance_proof \
        --assign batch2 \
        --move-call "${PKG_CONTRA}::contra::add_to_batch" --type-args "$COIN_TYPE" \
        --args batch "$ref_receiver" 'vector[]' "$ref_deny" \
        --move-call "${PKG_CONTRA}::contra::finalize" --type-args "$COIN_TYPE" --args batch2
    rm -f "$bundle"

    log_step "Receiver merge"
    myso client switch --address "$RECEIVER_ADDRESS" >/dev/null
    invoke_ptb \
        --assign auth \
        --move-call "${PKG_CONTRA}::contra::authorize_as_sender" --type-args "$COIN_TYPE" \
        --args "$ref_ct" \
        --move-call "${PKG_CONTRA}::contra::merge" --type-args "$COIN_TYPE" \
        --args "$ref_receiver" auth
    myso client switch --address "$SENDER_ADDRESS" >/dev/null
}

build_unwrap_bundle_file() {
    local out="$1"
    local crypto
    crypto="$(resolve_contra_crypto)"
    if [[ "$crypto" == cargo* ]]; then
        $crypto unwrap \
            --account-id "$RECEIVER_ACCOUNT_ID" \
            --coin-type "$COIN_TYPE" \
            --owner-sk "$RECEIVER_SK" \
            --balance "$DEFAULT_TRANSFER_AMOUNT" \
            --unwrap-amount "$DEFAULT_UNWRAP_AMOUNT" \
            --output "$out"
    else
        "$crypto" unwrap \
            --account-id "$RECEIVER_ACCOUNT_ID" \
            --coin-type "$COIN_TYPE" \
            --owner-sk "$RECEIVER_SK" \
            --balance "$DEFAULT_TRANSFER_AMOUNT" \
            --unwrap-amount "$DEFAULT_UNWRAP_AMOUNT" \
            --output "$out"
    fi
}

unwrap_flow() {
    [[ -n "${RECEIVER_ACCOUNT_ID:-}" && -n "${COIN_TYPE:-}" ]] || {
        echo "Need RECEIVER_ACCOUNT_ID, COIN_TYPE" >&2
        return 1
    }
    preflight
    myso client switch --address "$RECEIVER_ADDRESS" >/dev/null
    local bundle new_parts consistency range_proofs balance_parts
    bundle="$(mktemp)"
    build_unwrap_bundle_file "$bundle"
    new_parts="$(json_parts_to_move_vector "$bundle" new_balance_parts)"
    consistency="$(json_singleton_consistency_to_move_vector "$bundle")"
    range_proofs="$(json_range_proofs_to_move_vector "$bundle")"
    balance_parts="$(json_parts_to_move_vector "$bundle" balance_proof_parts)"
    local ref_acct ref_ct ref_deny ref_pool
    ref_acct="$(ptb_shared_ref "$RECEIVER_ACCOUNT_ID")" || return 1
    ref_ct="$(ptb_shared_ref "$CONFIDENTIAL_TOKEN_ID")" || return 1
    ref_deny="$(ptb_shared_ref "$DENY_LIST_ID")" || return 1
    ref_pool="$(ptb_shared_ref "$POOL_ID")" || return 1
    log_step "Unwrap $DEFAULT_UNWRAP_AMOUNT public coins from receiver account"
    invoke_ptb \
        --assign new_bal \
        --move-call "${PKG_CONTRA}::decode::encrypted_amount" --args "$new_parts" \
        --assign wf_proof \
        --move-call "${PKG_CONTRA}::encrypted_amount::new_well_formed_proof" \
        --args "$range_proofs" "$consistency" \
        --assign balance_proof \
        --move-call "${PKG_CONTRA}::decode::ddh_proof" --args "$balance_parts" \
        --assign auth \
        --move-call "${PKG_CONTRA}::contra::authorize_as_sender" --type-args "$COIN_TYPE" \
        --args "$ref_ct" \
        --move-call "${PKG_CONTRA}::contra::unwrap" --type-args "$COIN_TYPE" \
        --args "$ref_acct" auth "$ref_ct" "$ref_deny" "$ref_pool" \
        new_bal wf_proof "$DEFAULT_UNWRAP_AMOUNT" balance_proof
    rm -f "$bundle"
}

run_all_e2e() {
    preflight
    load_session_state
    SENDER_ADDRESS="${SENDER_ADDRESS:-$(myso client active-address)}"
    refresh_contra_session_from_graphql || true
    [[ -n "${TEST_COIN_PKG:-}" ]] || publish_test_coin
    [[ -n "${CONFIDENTIAL_TOKEN_ID:-}" ]] || setup_confidential_token
    setup_accounts
    wrap_flow
    transfer_flow
    unwrap_flow
    log_step "Contra E2E run-all complete"
}

interactive_menu() {
    load_session_state
    while true; do
        echo "" >&2
        echo "Contra confidential transfer helper" >&2
        echo "  1) Refresh session (GraphQL)" >&2
        echo "  2) Publish test coin" >&2
        echo "  3) Setup confidential token" >&2
        echo "  4) Setup accounts (register sender + receiver)" >&2
        echo "  5) Wrap + merge (mint public -> private)" >&2
        echo "  6) Transfer flow" >&2
        echo "  7) Unwrap flow" >&2
        echo "  8) Run all" >&2
        echo "  q) Quit" >&2
        read -r -p "Choice: " choice
        case "$choice" in
            1) refresh_contra_session_from_graphql ;;
            2) publish_test_coin ;;
            3) setup_confidential_token ;;
            4) setup_accounts ;;
            5) wrap_flow ;;
            6) transfer_flow ;;
            7) unwrap_flow ;;
            8) run_all_e2e ;;
            q|Q) break ;;
            *) echo "Unknown choice" >&2 ;;
        esac
    done
}

main() {
    load_session_state
    if [[ $# -eq 0 ]]; then
        interactive_menu
        return 0
    fi
    case "$1" in
        --help|-h) usage ;;
        --refresh-session) refresh_contra_session_from_graphql ;;
        --publish-test-coin) publish_test_coin ;;
        --setup-token) setup_confidential_token ;;
        --setup-accounts) setup_accounts ;;
        --wrap-flow) wrap_flow ;;
        --transfer-flow) transfer_flow ;;
        --unwrap-flow) unwrap_flow ;;
        --run-all) run_all_e2e ;;
        *) echo "Unknown flag: $1" >&2; usage; return 1 ;;
    esac
}

main "$@"
