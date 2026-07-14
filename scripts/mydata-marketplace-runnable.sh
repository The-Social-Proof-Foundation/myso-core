#!/usr/bin/env bash
# Copyright (c) Mysten Labs, Inc.
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Interactive helper for social_contracts::mydata Move calls via `myso client call`.
#
# Hybrid memory auth (purchase / approve):
#   - purchase_one_time, purchase_subscription, and mydata_approve take a MemoryAccount object id;
#     they do NOT take SubAgent object ids or ancestor chains in the PTB.
#   - Human buyer/principal: sign as your principal address; MemoryAccount is still a required Move arg.
#   - Agent buyer/approver: sign as the agent derived_address; on-chain registry on the passed
#     MemoryAccount resolves auth (spend limits for purchase, CAP_MYDATA_READ for approve).
#   - MEMORY_ACCOUNT_ID: menus 3/4/7 always prompt (Enter keeps session default). Pass the buyer's
#     MemoryAccount for purchase, or the listing owner's for approve / agent CAP_MYDATA_READ.
#
# Prerequisites:
#   - MySocial shared objects exist (bootstrap runs automatically at genesis).
#   - Shared MyData objects exist (mydata::bootstrap_init). Resolve IDs from GraphQL / explorer
#     (e.g. types ...::mydata::MyDataConfig, MyDataRegistry, MyDataPoolRegistry, ...).
#   - MyDataConfig.marketplace_enabled controls only new query/pool snapshot anchors (menu 14).
#     Profile-gated, one-time, and recurring MyData listings/purchases remain available when false.
#   - Hybrid MemoryAccount (VERSION 2) with on-chain agent index for agent purchase/approve flows.
#
# Production-like encryption (menu 2):
#   - Requires `mydata` from the myso-mydata repo (cargo build -p mydata-cli; same as myso start --with-mydata).
#   - Runs `mydata encrypt-hmac` per crates/myso-framework/.../bf_hmac_encryption.move comments.
#   - Default key server http://127.0.0.1:2024 (same URL `myso start --with-mydata` writes after mapping
#     bind 0.0.0.0:2024 to localhost): probes /service (HTTP 4xx still counts as reachable).
#   - Loads PUBLIC_KEY and KEY_SERVER_OBJECT_ID from network.config/mydata/local-mydata-secrets.env
#     (NOT key-server-config.yaml — that is key-server runtime config only).
#   - --package-id for encrypt is the MySoSocial package (0x50c1); ciphertext must match create_and_share
#     EncryptedObject.package_id for permissioned key-server flows.
#   - Permissioned key servers re-check mydata_approve on every fetch_key; menu 18 revoke_access blocks
#     revoked buyers from obtaining new derived keys (already-fetched keys may still decrypt offline).
#
# Session reuse (menus 1–17):
#   - Saved values from menu 0 / marketplace-session.env are used automatically with [session] log lines.
#   - Prompt only when a required field is missing (or MYDATA_FORCE_PROMPT=1).
#
# Query marketplace E2E (pool flow; separate from single-listing buy/decrypt menus 2–7):
#   1) Admin: 11 create_broad_pool → 12 create_sub_pool (copy sub-pool id into SUB_POOL_ID in menu 0)
#   2) Owner: 2 create_and_share → 13 assign_mydata_to_pools (sign as listing owner)
#   3) Buyer: 15 record_snapshot_anchor (pay MYSO; off-chain query server validates amount + runs query)
#   4) Admin: 16 fund additional escrow (optional) → 17 publish_distribution atomically
#   5) Contributors: 18/19 claim with a locally validated proof; buyer: 20 refund after expiry
#   Menus 13–14 require `myso client active-address` = listing owner. Snapshot recording verifies
#   broad/sub-pool membership and inherits the broad pool's optional platform binding.
#
# Environment:
#   MYSO              Path to myso binary (optional)
#   MYDATA            Path to mydata binary (optional; package mydata-cli, binary name mydata)
#   MYDATA_REPO / MYSO_MYDATA_REPO   Path to myso-mydata repo
#   DRY_RUN=1         Pass --dry-run to myso client
#   MYDATA_MARKETPLACE_SESSION   Path to saved session env file (optional).
#                                   Default save: <repo>/network.config/mydata/marketplace-session.env
#                                   Load tries: this var, then ./network.config/.../ then repo path.
#   MYDATA_FORCE_PROMPT=1        Re-prompt for session-backed fields even when already set.
#   MYDATA_NO_AUTO_REFRESH=1     Skip auto GraphQL refresh when config ids are missing on startup.
#   Menu [0]: refresh MyDataConfig/Registry/Pool/Anchor/Vault/AdminCap ids from GraphQL.
#   Menu [s]: manual session setup (secrets file, listing ids, client.yaml). Blank CLIENT_CONFIG
#     defaults to <cwd>/network.config/client.yaml
#   Menus 3/4 always prompt for LISTING_ID, PAY_COIN_ID, and MEMORY_ACCOUNT_ID (Enter keeps session).
#   Menu 7 always prompts for MEMORY_ACCOUNT_ID. Shared-object ids (BlockListRegistry, MemoryConfig,
#     etc.) are refreshed from GraphQL and not preserved across menu 0 (stale session ids discarded).
#   Menus 3/4/15 must NOT pass --type-args: Move entry functions use hardcoded Coin<MYSO> (not generic).
#     Passing COIN_TYPE as --type-args causes Move Bytecode Verification Error.
#   Gas payment is never passed as --gas; the CLI auto-selects a Coin<MYSO> (excluding tx inputs).
#   GAS_BUDGET (default 1000000000 MIST = 1 MYSO) is passed as --gas-budget on every call. Without it,
#     the CLI dry-runs with max_tx_gas and fails when the gas coin balance is lower.
#   PAY_COIN_ID must be owned by `myso client active-address` (checked before purchase).
#   Legacy marketplace-session.env files may contain GAS_COIN_ID; it is ignored on load.
#   Agent flows: no SubAgent args in the PTB; configure client.yaml active address as derived_address.
#   MYDATA_MARKETPLACE_NO_SAVE=1 Skip writing session file after menu 0 / encrypt flow.
#   ASSUME_YES=1 / -y          Non-interactive yes for confirm_run, marketplace enable, and operation defaults.
#
# Usage: ./scripts/mydata-marketplace-runnable.sh [--refresh-session] [-y] [--help] [--no-session]
# Rename/link as runnable.sh if you prefer.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

print_mydata_operation_summary() {
    local operation="$1"
    shift
    if [[ "${MYDATA_LAST_CALL_EXECUTED:-1}" != 1 ]]; then
        print_run_summary_header "MyData Marketplace — ${operation} NOT executed (confirmation declined)"
        print_run_summary_footer
        return 0
    fi
    print_run_summary_header "MyData Marketplace — ${operation} completed"
    while [[ $# -ge 2 ]]; do
        print_run_summary_line "$1" "$2"
        shift 2
    done
    print_run_summary_footer
}

readonly DEFAULT_PKG_SOCIAL='0x00000000000000000000000000000000000000000000000000000000000050c1'
readonly DEFAULT_CLOCK='0x0000000000000000000000000000000000000000000000000000000000000006'
readonly DEFAULT_COIN_TYPE='0x2::myso::MYSO'
readonly DEFAULT_KEY_SERVER_URL='http://127.0.0.1:2024'
readonly DEFAULT_GAS_BUDGET='1000000000'
readonly DEFAULT_SECRETS_REL='network.config/mydata/local-mydata-secrets.env'
readonly G2_PUBLIC_KEY_HEX_LEN=192

readonly DEFAULT_MAX_ENCRYPTION_ID_BYTES='1024'
readonly DEFAULT_MAX_ENCRYPTED_DATA_BYTES='262144'
readonly DEFAULT_MAX_TAG_BYTES='64'
readonly DEFAULT_MAX_METADATA_BYTES='1024'
readonly DEFAULT_MAX_PAYMENT_REFERENCE_BYTES='256'
readonly DEFAULT_MAX_POOL_ASSIGNMENTS='32'
readonly DEFAULT_MAX_MERKLE_PROOF_DEPTH='64'
readonly DEFAULT_MAX_PAID_ACCESS_ENTRIES='100000'
readonly DEFAULT_CLAIM_WINDOW_MS='2592000000'
readonly DEFAULT_P2P_PLATFORM_FEE_BPS='250'
readonly DEFAULT_P2P_ECOSYSTEM_FEE_BPS='250'
readonly DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS='250'
readonly DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS='250'
readonly DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS='0'
readonly DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS='10000'
readonly DEFAULT_MAX_TAGS='10'
readonly DEFAULT_MAX_SUBSCRIPTION_DAYS='365'
readonly DEFAULT_MAX_FREE_ACCESS_GRANTS='100000'

readonly MYDATA_CONFIG_GQL='query MyDataConfiguration {
  mydataConfiguration {
    marketplaceEnabled
    maxTags
    maxSubscriptionDays
    maxFreeAccessGrants
    maxEncryptionIdBytes
    maxEncryptedDataBytes
    maxTagBytes
    maxMetadataBytes
    maxPaymentReferenceBytes
    maxPoolAssignments
    maxMerkleProofDepth
    maxPaidAccessEntries
    defaultClaimWindowMs
    p2PPlatformFeeBps
    p2PEcosystemFeeBps
    mydataMarketplacePlatformFeeBps
    mydataMarketplaceEcosystemFeeBps
    nonPlatformPlatformToCreatorBps
    nonPlatformPlatformToTreasuryBps
  }
}'

readonly MYDATA_MARKETPLACE_GQL_EXTRAS='query MyDataMarketplaceSessionObjects {
  mydataConfig: objects(filter: { type: "0x50c1::mydata::MyDataConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataRegistry: objects(filter: { type: "0x50c1::mydata::MyDataRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataPoolRegistry: objects(filter: { type: "0x50c1::mydata::MyDataPoolRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  snapshotAnchorRegistry: objects(filter: { type: "0x50c1::mydata::SnapshotAnchorRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  mydataClaimVault: objects(filter: { type: "0x50c1::mydata::MyDataClaimVault", ownerKind: SHARED }, first: 1) { nodes { address } }
  distributionRegistry: objects(filter: { type: "0x50c1::mydata::DistributionRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  memoryConfig: objects(filter: { type: "0x50c1::memory::MemoryConfig", ownerKind: SHARED }, first: 1) { nodes { address } }
  ecosystemTreasury: objects(filter: { type: "0x50c1::profile::EcosystemTreasury", ownerKind: SHARED }, first: 1) { nodes { address } }
  blockListRegistry: objects(filter: { type: "0x50c1::block_list::BlockListRegistry", ownerKind: SHARED }, first: 1) { nodes { address } }
  platform: objects(filter: { type: "0x50c1::platform::Platform" }, last: 1) { nodes { address } }
  mydataAdminCap: objects(filter: { type: "0x50c1::mydata::MyDataAdminCap" }, last: 1) { nodes { address } }
  mydataPoolAdminCap: objects(filter: { type: "0x50c1::mydata::MyDataPoolAdminCap" }, last: 1) { nodes { address } }
}'

# Listing/pay/secrets only — never preserve GraphQL-owned shared objects or MemoryAccount
# (stale BlockListRegistry / MemoryConfig from a prior localnet reboot break purchase).
MYDATA_SESSION_PRESERVED_KEYS=(
    CLIENT_CONFIG KEY_SERVER_URL LISTING_ID SUB_POOL_ID PAY_COIN_ID
    MYDATA_ENCRYPTION_ID REVOKE_BUYER_ID MYDATA_SECRETS_FILE PUBLIC_KEY KEY_SERVER_OBJECT_ID
)

DO_REFRESH=0

CLIENT_CONFIG=''
PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
CLOCK_ID="$DEFAULT_CLOCK"
COIN_TYPE="$DEFAULT_COIN_TYPE"
KEY_SERVER_URL="$DEFAULT_KEY_SERVER_URL"

MYDATA_CONFIG_ID=''
MYDATA_REGISTRY_ID=''
POOL_REGISTRY_ID=''
ANCHOR_REGISTRY_ID=''
CLAIM_VAULT_ID=''
DIST_REGISTRY_ID=''
MYDATA_ADMIN_CAP_ID=''
POOL_ADMIN_CAP_ID=''
GAS_BUDGET=''
LISTING_ID=''
SUB_POOL_ID=''
PAY_COIN_ID=''
MYDATA_ENCRYPTION_ID=''
MEMORY_ACCOUNT_ID=''
REVOKE_BUYER_ID=''
MYDATA_SECRETS_FILE=''
PUBLIC_KEY=''
KEY_SERVER_OBJECT_ID=''

# Skip loading/writing session file (--no-session)
NO_SESSION_FILE=0

session_state_save_path() {
    if [[ -n "${MYDATA_MARKETPLACE_SESSION:-}" ]]; then
        printf '%s' "$MYDATA_MARKETPLACE_SESSION"
    else
        printf '%s' "$REPO_ROOT/network.config/mydata/marketplace-session.env"
    fi
}

collect_mydata_marketplace_gql_mappings() {
    local json="$1" alias val env_key
    for alias in mydataConfig mydataRegistry mydataPoolRegistry snapshotAnchorRegistry \
        mydataClaimVault distributionRegistry memoryConfig ecosystemTreasury blockListRegistry platform \
        mydataAdminCap mydataPoolAdminCap; do
        case "$alias" in
            mydataConfig) env_key=MYDATA_CONFIG_ID ;;
            mydataRegistry) env_key=MYDATA_REGISTRY_ID ;;
            mydataPoolRegistry) env_key=POOL_REGISTRY_ID ;;
            snapshotAnchorRegistry) env_key=ANCHOR_REGISTRY_ID ;;
            mydataClaimVault) env_key=CLAIM_VAULT_ID ;;
            distributionRegistry) env_key=DIST_REGISTRY_ID ;;
            memoryConfig) env_key=MEMORY_CONFIG_ID ;;
            ecosystemTreasury) env_key=ECOSYSTEM_TREASURY_ID ;;
            blockListRegistry) env_key=BLOCK_LIST_REGISTRY_ID ;;
            platform) env_key=PLATFORM_OBJECT_ID ;;
            mydataAdminCap) env_key=MYDATA_ADMIN_CAP_ID ;;
            mydataPoolAdminCap) env_key=POOL_ADMIN_CAP_ID ;;
            *) continue ;;
        esac
        val="$(gql_object_address "$json" "$alias")"
        [[ -n "$val" ]] || continue
        printf -v "$env_key" '%s' "$(normalize_hex_id "$val")"
        log_session_use "$env_key" "${!env_key}"
    done
}

resolve_owned_admin_cap() {
    local type_suffix="$1" active json
    active="$(resolve_myso_active_address)" || return 1
    json="$(myso client --client.config "$CLIENT_CONFIG" objects "$active" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r --arg suffix "$type_suffix" '
        def move_type:
            .type? // .objectType? // .object_type? // .data.type? // .data.objectType? //
            (
                if (.data.Move.type_? | type) == "string" then
                    .data.Move.type_
                elif (.data.Move.type_.Other? | type) == "object" then
                    ((.data.Move.type_.Other.module? // "") + "::" + (.data.Move.type_.Other.name? // ""))
                else
                    ""
                end
            );
        .[]?
        | select((move_type | tostring) | endswith($suffix))
        | .data.objectId // .objectId // .object_id // .address // empty
    ' | head -n1
}

bind_admin_caps_to_active_address() {
    local cap
    cap="$(resolve_owned_admin_cap 'mydata::MyDataAdminCap')" || cap=''
    if [[ -n "$cap" ]]; then
        MYDATA_ADMIN_CAP_ID="$(normalize_hex_id "$cap")"
    else
        MYDATA_ADMIN_CAP_ID=''
        echo "No MyDataAdminCap owned by the active administrator; refusing an arbitrary GraphQL cap." >&2
    fi
    cap="$(resolve_owned_admin_cap 'mydata::MyDataPoolAdminCap')" || cap=''
    if [[ -n "$cap" ]]; then
        POOL_ADMIN_CAP_ID="$(normalize_hex_id "$cap")"
    else
        POOL_ADMIN_CAP_ID=''
        echo "No MyDataPoolAdminCap owned by the active administrator; refusing an arbitrary GraphQL cap." >&2
    fi
}

gql_mydata_owner() {
    local listing_id="$1" vars resp owner
    listing_id="$(normalize_hex_id "$listing_id")" || return 1
    vars="$(jq -nc --arg id "$listing_id" '{id: $id}')" || return 1
    resp="$(graphql_post \
        'query MyDataOwner($id: ID!) { mydataRecord(id: $id) { owner } }' \
        "$vars" 2>/dev/null)" || return 1
    owner="$(echo "$resp" | jq -r '.data.mydataRecord.owner // empty' 2>/dev/null)"
    [[ -n "$owner" ]] || return 1
    normalize_hex_id "$owner"
}

# Abort early with a clear message instead of Move ESelfPurchase (abort 4).
assert_buyer_is_not_listing_owner() {
    local listing_id="$1" buyer owner
    listing_id="$(normalize_hex_id "$listing_id")" || return 1
    buyer="$(resolve_myso_active_address)" || {
        echo "Could not resolve active-address for self-purchase check" >&2
        return 1
    }
    buyer="$(normalize_hex_id "$buyer")" || return 1
    owner="$(gql_mydata_owner "$listing_id" 2>/dev/null)" || {
        echo "Warning: could not resolve listing owner via GraphQL; skipping self-purchase check." >&2
        return 0
    }
    if [[ "$buyer" == "$owner" ]]; then
        echo "Error: active-address $buyer owns listing $listing_id (Move ESelfPurchase)." >&2
        echo "  Switch to a different buyer wallet (myso client switch --address <buyer>), then retry." >&2
        return 1
    fi
    log_session_use "Listing owner (not buyer)" "$owner"
    return 0
}

# Always prompt — same pattern as listing / payment coin (Enter keeps session default).
resolve_purchase_memory_account_id() {
    local account current="${MEMORY_ACCOUNT_ID:-}" label="${1:-MemoryAccount object id}"
    if [[ -n "$current" ]]; then
        log_session_use "$label" "$current"
    else
        echo "  [session] $label=<unset>" >&2
    fi
    echo "  Enter the MemoryAccount object id to pass to Move (buyer account for purchase;" >&2
    echo "  listing-owner account for approve / agent flows). Find it on the profile or menu [s]." >&2
    account="$(prompt_or_default "$label (empty = use saved)" "$current")"
    [[ -n "$account" ]] || account="$current"
    [[ -n "$account" ]] || {
        echo "MemoryAccount object id is required (set in menu [s] or enter here)." >&2
        return 1
    }
    account="$(normalize_hex_id "$account")" || return 1
    MEMORY_ACCOUNT_ID="$account"
    printf '%s' "$account"
}

# Drop stale GraphQL-owned shared ids and refresh so purchase/approve cannot pass dead objects.
ensure_mydata_purchase_shared_ids() {
    local need_refresh=0 name
    for name in "$@"; do
        if ! session_value_set "$name"; then
            need_refresh=1
            continue
        fi
        if ! object_exists_on_fullnode "${!name}"; then
            echo "Session $name=${!name} missing on fullnode; will refresh from GraphQL." >&2
            printf -v "$name" '%s' ''
            need_refresh=1
        fi
    done
    if [[ "$need_refresh" != 1 ]]; then
        return 0
    fi
    refresh_mydata_marketplace_session_from_graphql || return 1
    require_session_fields "$@" || return 1
    for name in "$@"; do
        if ! object_exists_on_fullnode "${!name}"; then
            echo "After GraphQL refresh, $name=${!name} still missing on fullnode." >&2
            return 1
        fi
    done
}

refresh_active_profile_memory_account() {
    # Optional session hint only — purchase/approve always prompt; do not overwrite a set value.
    [[ -z "${MEMORY_ACCOUNT_ID:-}" ]] || return 0
    command -v jq >/dev/null 2>&1 || return 0
    local active vars resp account
    active="$(resolve_myso_active_address)" || return 0
    vars="$(jq -nc --arg addr "$(normalize_hex_id "$active")" '{addr: $addr}')" || return 0
    resp="$(graphql_post 'query ActiveProfileMemory($addr: MySoAddress!) { profile(address: $addr) { memoryAccountId } }' "$vars" 2>/dev/null)" || return 0
    account="$(echo "$resp" | jq -r '.data.profile.memoryAccountId // empty' 2>/dev/null)"
    [[ -n "$account" ]] || return 0
    MEMORY_ACCOUNT_ID="$(normalize_hex_id "$account")"
    log_session_use "MEMORY_ACCOUNT_ID (session default hint)" "$MEMORY_ACCOUNT_ID"
}

refresh_mydata_marketplace_session_from_graphql() {
    command -v curl >/dev/null 2>&1 || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }
    [[ "${NO_SESSION_FILE:-0}" == 1 ]] && { echo "Cannot refresh session with --no-session" >&2; return 1; }

    local f preserve_file="" key json
    f="$(session_state_save_path)"

    if [[ -f "$f" ]]; then
        preserve_file="$(mktemp)"
        # shellcheck disable=SC1090
        source "$f"
        {
            for key in "${MYDATA_SESSION_PRESERVED_KEYS[@]}"; do
                session_value_set "$key" && printf '%s=%q\n' "$key" "${!key}"
            done
        } > "$preserve_file"
    fi

    log_step "Refreshing MyData marketplace session from GraphQL ($GRAPHQL_URL)"
    json="$(graphql_post "$MYDATA_MARKETPLACE_GQL_EXTRAS")" || {
        rm -f "$preserve_file"
        return 1
    }

    PKG_SOCIAL="$SOCIAL_DEFAULT_PKG"
    CLOCK_ID="$SOCIAL_DEFAULT_CLOCK"
    COIN_TYPE="$SOCIAL_DEFAULT_COIN_TYPE"
    apply_session_defaults
    if [[ -z "${CLIENT_CONFIG:-}" ]]; then
        CLIENT_CONFIG="$PWD/network.config/client.yaml"
    fi
    collect_mydata_marketplace_gql_mappings "$json"
    bind_admin_caps_to_active_address || true

    if [[ -n "$preserve_file" && -s "$preserve_file" ]]; then
        # shellcheck disable=SC1090
        source "$preserve_file"
        rm -f "$preserve_file"
    fi

    refresh_active_profile_memory_account || true
    hydrate_encrypt_from_secrets || true
    save_session_state
    show_context
}

maybe_auto_refresh_mydata_session() {
    [[ "${MYDATA_NO_AUTO_REFRESH:-0}" == 1 ]] && return 0
    [[ "${NO_SESSION_FILE:-0}" == 1 ]] && return 0
    if session_value_set MYDATA_CONFIG_ID && session_value_set MYDATA_REGISTRY_ID \
        && session_value_set POOL_REGISTRY_ID && session_value_set MYDATA_ADMIN_CAP_ID; then
        return 0
    fi
    refresh_mydata_marketplace_session_from_graphql
}

apply_session_defaults() {
    [[ -n "${PKG_SOCIAL:-}" ]] || PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$DEFAULT_CLOCK"
    [[ -n "${COIN_TYPE:-}" ]] || COIN_TYPE="$DEFAULT_COIN_TYPE"
    [[ -n "${KEY_SERVER_URL:-}" ]] || KEY_SERVER_URL="$DEFAULT_KEY_SERVER_URL"
    [[ -n "${GAS_BUDGET:-}" ]] || GAS_BUDGET="$DEFAULT_GAS_BUDGET"
}

session_field_count() {
    local key count=0
        for key in CLIENT_CONFIG MYDATA_CONFIG_ID MYDATA_REGISTRY_ID POOL_REGISTRY_ID ANCHOR_REGISTRY_ID \
        CLAIM_VAULT_ID DIST_REGISTRY_ID MYDATA_ADMIN_CAP_ID POOL_ADMIN_CAP_ID GAS_BUDGET \
        LISTING_ID SUB_POOL_ID PAY_COIN_ID MYDATA_ENCRYPTION_ID MEMORY_ACCOUNT_ID MYDATA_SECRETS_FILE PUBLIC_KEY KEY_SERVER_OBJECT_ID \
        MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID BLOCK_LIST_REGISTRY_ID PLATFORM_OBJECT_ID; do
        session_value_set "$key" && count=$((count + 1))
    done
    printf '%s' "$count"
}

load_session_state() {
    [[ "${NO_SESSION_FILE:-0}" == 1 ]] && return 0
    local paths p loaded=0
    paths=()
    [[ -n "${MYDATA_MARKETPLACE_SESSION:-}" ]] && paths+=("$MYDATA_MARKETPLACE_SESSION")
    paths+=("$PWD/network.config/mydata/marketplace-session.env")
    paths+=("$REPO_ROOT/network.config/mydata/marketplace-session.env")
    for p in "${paths[@]}"; do
        [[ -n "$p" && -f "$p" ]] || continue
        # shellcheck disable=SC1090
        source "$p"
        loaded=1
        break
    done
    unset GAS_COIN_ID
    apply_session_defaults
    hydrate_encrypt_from_secrets
    if [[ "$loaded" == 1 ]]; then
        echo "Loaded MyData marketplace session ($(( $(session_field_count) )) fields set) from: $p" >&2
    fi
}

save_session_state() {
    [[ "${NO_SESSION_FILE:-0}" == 1 ]] && return 0
    [[ "${MYDATA_MARKETPLACE_NO_SAVE:-}" == 1 ]] && return 0
    local f
    f="$(session_state_save_path)"
    mkdir -p "$(dirname "$f")"
    local old_umask
    old_umask="$(umask)"
    umask 077
    {
        echo "# Local session for scripts/mydata-marketplace-runnable.sh — paths/ids only; do not commit if sensitive."
        local key
        for key in CLIENT_CONFIG PKG_SOCIAL CLOCK_ID COIN_TYPE KEY_SERVER_URL MYDATA_CONFIG_ID MYDATA_REGISTRY_ID POOL_REGISTRY_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID DIST_REGISTRY_ID MYDATA_ADMIN_CAP_ID POOL_ADMIN_CAP_ID GAS_BUDGET LISTING_ID SUB_POOL_ID PAY_COIN_ID MYDATA_ENCRYPTION_ID MEMORY_ACCOUNT_ID REVOKE_BUYER_ID MYDATA_SECRETS_FILE PUBLIC_KEY KEY_SERVER_OBJECT_ID MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID BLOCK_LIST_REGISTRY_ID PLATFORM_OBJECT_ID; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    umask "$old_umask"
    echo "Saved session to: $f" >&2
}

usage() {
    sed -n '2,75p' "$0" | sed 's/^# \?//'
}

strip_0x() {
    local x="${1:-}"
    x="${x#0x}"
    x="${x#0X}"
    printf '%s' "$x"
}

# Comma- or space-separated object IDs -> [0x..., 0x...] for myso client vector<ID> args.
format_sub_pool_ids_vector() {
    local input="$1"
    local normalized vec="" first=1 p

    input="${input//,/ }"
    for p in $input; do
        p="${p//[[:space:]]/}"
        [[ -n "$p" ]] || continue
        normalized="$(strip_0x "$p")"
        if [[ ${#normalized} -ne 64 ]]; then
            echo "Invalid sub-pool id: $p (expected 32-byte hex object id)" >&2
            return 1
        fi
        if [[ $first -eq 1 ]]; then
            vec="[0x${normalized}"
            first=0
        else
            vec+=", 0x${normalized}"
        fi
    done
    if [[ $first -eq 1 ]]; then
        echo "sub_pool_ids must be non-empty (one ID or comma-separated list)." >&2
        return 1
    fi
    vec+="]"
    printf '%s' "$vec"
}

resolve_sub_pool_id() {
    local label="${1:-sub_pool_id}"
    if [[ -n "${SUB_POOL_ID:-}" && "${MYDATA_FORCE_PROMPT:-}" != 1 ]]; then
        log_session_use "$label" "$SUB_POOL_ID"
        printf '%s' "$SUB_POOL_ID"
    else
        prompt_with_default "$label" "${SUB_POOL_ID:-}"
    fi
}


prompt_with_default() {
    local label="$1"
    local default="$2"
    local _read
    if [[ -n "$default" ]]; then
        read -r -p "${label} [${default}]: " _read || true
        printf '%s' "${_read:-$default}"
    else
        read -r -p "${label}: " _read
        printf '%s' "$_read"
    fi
}

session_value_set() {
    local var_name="$1"
    [[ -n "${!var_name:-}" ]]
}

log_session_use() {
    local label="$1"
    local value="$2"
    echo "  [session] ${label}=${value}" >&2
}

resolve_session_or_prompt() {
    local var_name="$1"
    local label="$2"
    local default="${3:-}"
    local current="${!var_name:-}"
    if [[ -n "$current" && "${MYDATA_FORCE_PROMPT:-}" != 1 ]]; then
        log_session_use "$label" "$current"
        printf '%s' "$current"
        return 0
    fi
    prompt_with_default "$label" "${current:-$default}"
}

prompt_or_default() {
    local label="$1"
    local default="$2"
    if [[ "${ASSUME_YES:-}" == 1 ]]; then
        printf '%s' "$default"
        return 0
    fi
    prompt_with_default "$label" "$default"
}

require_session_fields() {
    local name missing=()
    for name in "$@"; do
        session_value_set "$name" || missing+=("$name")
    done
    if [[ ${#missing[@]} -gt 0 ]]; then
        echo "Missing session field(s): ${missing[*]}. Run menu [0] to refresh from GraphQL or [s] for manual setup." >&2
        return 1
    fi
    return 0
}

# Purchase PTBs pass PAY_COIN_ID as a Move arg; auto gas selection may reuse that coin and fail dry-run.
resolve_purchase_listing_id() {
    local listing current="${LISTING_ID:-}"
    if [[ -n "$current" ]]; then
        log_session_use "MyData listing object id" "$current"
    else
        echo "  [session] MyData listing object id=<unset>" >&2
    fi
    listing="$(prompt_or_default "MyData listing object id (empty = use saved)" "$current")"
    [[ -n "$listing" ]] || listing="$current"
    [[ -n "$listing" ]] || { echo "MyData listing object id is required (set in menu 0 or enter here)." >&2; return 1; }
    LISTING_ID="$listing"
    printf '%s' "$listing"
}

list_active_address_gas_coins() {
    local myso json id
    myso="$(resolve_myso)"
    [[ -n "$myso" && -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || return 0
    json="$("$myso" client --client.config "$CLIENT_CONFIG" gas --json 2>/dev/null)" || return 0
    [[ -n "$json" && "$json" != "[]" ]] || return 0
    echo "  Coin<MYSO> owned by active-address (from \`myso client gas\`):" >&2
    while IFS= read -r id; do
        [[ -n "$id" ]] && echo "    $id" >&2
    done < <(printf '%s' "$json" | grep -Eo '"gasCoinId"[[:space:]]*:[[:space:]]*"0x[0-9a-fA-F]+"' \
        | grep -Eo '0x[0-9a-fA-F]+')
}

resolve_purchase_pay_coin() {
    local pay current="${PAY_COIN_ID:-}"
    list_active_address_gas_coins
    if [[ -n "$current" ]]; then
        log_session_use "Payment Coin<MYSO> object id" "$current"
    else
        echo "  [session] Payment Coin<MYSO> object id=<unset>" >&2
    fi
    pay="$(prompt_or_default "Payment Coin<MYSO> object id (CLI auto-selects gas; use a separate coin when possible)" "$current")"
    [[ -n "$pay" ]] || { echo "Payment coin object id is required." >&2; return 1; }
    PAY_COIN_ID="$pay"
    printf '%s' "$pay"
}

resolve_myso_active_address() {
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" && -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || return 1
    "$myso" client --client.config "$CLIENT_CONFIG" active-address 2>/dev/null
}

object_address_owner() {
    local myso object_id json
    myso="$(resolve_myso)"
    object_id="$1"
    [[ -n "$myso" && -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || return 1
    json="$("$myso" client --client.config "$CLIENT_CONFIG" object "$object_id" --json 2>/dev/null)" || return 1
    printf '%s' "$json" | grep -Eo '"AddressOwner"[[:space:]]*:[[:space:]]*"0x[0-9a-fA-F]+"' | head -n1 \
        | grep -Eo '0x[0-9a-fA-F]+' | head -n1
}

validate_purchase_coin_ownership() {
    local pay_coin="$1"
    local active pay_owner
    active="$(resolve_myso_active_address)" || {
        echo "Warning: could not read active-address from client config; skipping coin ownership check." >&2
        return 0
    }
    log_session_use "Client active-address" "$active"
    pay_owner="$(object_address_owner "$pay_coin")" || {
        echo "Warning: could not fetch owner for payment coin $pay_coin." >&2
        return 0
    }
    if [[ "$pay_owner" != "$active" ]]; then
        echo "Error: payment coin $pay_coin is owned by $pay_owner, not active-address $active." >&2
        echo "  Use a payment coin owned by the active address, or run: myso client switch --address <owner>" >&2
        return 1
    fi
    return 0
}

validate_g2_public_key_hex() {
    local pk="${1:-}"
    local naked len
    naked="$(strip_0x "$pk")"
    len="${#naked}"
    if [[ "$len" -ne "$G2_PUBLIC_KEY_HEX_LEN" ]]; then
        echo "PUBLIC_KEY invalid: ${len} hex chars after 0x strip; encrypt-hmac requires ${G2_PUBLIC_KEY_HEX_LEN} hex chars (compressed BLS12-381 G2)." >&2
        return 1
    fi
    if [[ ! "$naked" =~ ^[0-9a-fA-F]+$ ]]; then
        echo "PUBLIC_KEY invalid: non-hex characters after 0x strip." >&2
        return 1
    fi
    return 0
}

public_key_status_line() {
    local pk="${PUBLIC_KEY:-}"
    if [[ -z "$pk" ]]; then
        echo "<unset>"
        return 0
    fi
    if validate_g2_public_key_hex "$pk" 2>/dev/null; then
        echo "valid (${G2_PUBLIC_KEY_HEX_LEN} hex)"
    else
        local naked len
        naked="$(strip_0x "$pk")"
        len="${#naked}"
        echo "INVALID (${len} hex, need ${G2_PUBLIC_KEY_HEX_LEN})"
    fi
}

default_secrets_env_path() {
    if [[ -f "$PWD/$DEFAULT_SECRETS_REL" ]]; then
        printf '%s' "$PWD/$DEFAULT_SECRETS_REL"
    elif [[ -f "$REPO_ROOT/$DEFAULT_SECRETS_REL" ]]; then
        printf '%s' "$REPO_ROOT/$DEFAULT_SECRETS_REL"
    else
        printf '%s' "$PWD/$DEFAULT_SECRETS_REL"
    fi
}

resolve_secrets_env_path() {
    local path="${1:-}"
    local dir sibling
    if [[ -z "$path" ]]; then
        default_secrets_env_path
        return 0
    fi
    if [[ "$path" == *.yaml || "$path" == *.yml ]]; then
        dir="$(dirname "$path")"
        sibling="${dir}/local-mydata-secrets.env"
        if [[ -f "$sibling" ]]; then
            echo "Note: MYDATA_SECRETS_FILE points at yaml; using sibling ${sibling}" >&2
            printf '%s' "$sibling"
            return 0
        fi
        echo "Warning: ${path} is yaml (no PUBLIC_KEY); expected local-mydata-secrets.env beside it." >&2
    fi
    printf '%s' "$path"
}

hydrate_encrypt_from_secrets_file() {
    local sec_file="$1"
    local overwrite_pk="${2:-0}"
    [[ -f "$sec_file" ]] || return 0
    local pk_from_file ks_from_file u_from_file
    pk_from_file="$(parse_env_file_value "$sec_file" PUBLIC_KEY 2>/dev/null || true)"
    ks_from_file="$(parse_env_file_value "$sec_file" KEY_SERVER_OBJECT_ID 2>/dev/null || true)"
    u_from_file="$(parse_env_file_value "$sec_file" KEY_SERVER_URL 2>/dev/null || true)"
    if [[ "$overwrite_pk" == 1 ]] || [[ -z "${PUBLIC_KEY:-}" ]] || ! validate_g2_public_key_hex "${PUBLIC_KEY:-}" 2>/dev/null; then
        if [[ -n "${pk_from_file:-}" ]] && validate_g2_public_key_hex "$pk_from_file" 2>/dev/null; then
            PUBLIC_KEY="$pk_from_file"
        fi
    fi
    if [[ -z "${KEY_SERVER_OBJECT_ID:-}" && -n "${ks_from_file:-}" ]]; then
        KEY_SERVER_OBJECT_ID="$ks_from_file"
    fi
    if [[ -n "${u_from_file:-}" ]]; then
        KEY_SERVER_URL="$u_from_file"
    fi
}

hydrate_encrypt_from_secrets() {
    local sec_path resolved
    if [[ -n "${MYDATA_SECRETS_FILE:-}" ]]; then
        sec_path="$(resolve_secrets_env_path "$MYDATA_SECRETS_FILE")"
    else
        sec_path="$(default_secrets_env_path)"
    fi
    resolved="$(resolve_secrets_env_path "$sec_path")"
    if [[ -f "$resolved" ]]; then
        MYDATA_SECRETS_FILE="$resolved"
        local overwrite=0
        if [[ -n "${PUBLIC_KEY:-}" ]] && ! validate_g2_public_key_hex "${PUBLIC_KEY:-}" 2>/dev/null; then
            overwrite=1
        fi
        hydrate_encrypt_from_secrets_file "$resolved" "$overwrite"
    fi
}

resolve_mydata_repo_root() {
    local root="${MYDATA_REPO:-}"
    if [[ -z "$root" ]]; then
        if [[ -n "${MYSO_MYDATA_REPO:-}" ]]; then
            root="$MYSO_MYDATA_REPO"
        elif [[ -d "$REPO_ROOT/../myso-mydata" && -f "$REPO_ROOT/../myso-mydata/Cargo.toml" ]]; then
            root="$(cd "$REPO_ROOT/../myso-mydata" && pwd)"
        fi
    fi
    printf '%s' "${root:-}"
}

resolve_mydata() {
    if [[ -n "${MYDATA:-}" && -x "${MYDATA}" ]]; then
        echo "$MYDATA"
        return 0
    fi
    if command -v mydata &>/dev/null; then
        command -v mydata
        return 0
    fi
    local root cand
    root="$(resolve_mydata_repo_root)"
    if [[ -n "$root" ]]; then
        for cand in "$root/target/release/mydata" "$root/target/debug/mydata"; do
            if [[ -x "$cand" ]]; then
                echo "$cand"
                return 0
            fi
        done
        for cand in "$root/target/release/mydata-cli" "$root/target/debug/mydata-cli"; do
            if [[ -x "$cand" ]]; then
                echo "Note: using legacy binary name mydata-cli (prefer mydata)" >&2
                echo "$cand"
                return 0
            fi
        done
    fi
    echo ""
}

confirm_run() {
    if [[ "${ASSUME_YES:-}" == 1 ]]; then
        return 0
    fi
    read -r -p "Execute this command? [y/N] " ans
    [[ "${ans:-}" == [yY] || "${ans:-}" == [yY][eE][sS] ]]
}

extra_gas_budget() {
    local budget="${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}"
    printf '%s\n' '--gas-budget' "$budget"
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

parse_env_file_value() {
    local file="$1"
    local key="$2"
    [[ -f "$file" ]] || return 1
    local line
    line="$(grep -E "^[[:space:]]*${key}=" "$file" | tail -n1)" || return 1
    [[ -n "$line" ]] || return 1
    line="${line#*=}"
    line="${line//$'\r'/}"
    if [[ "$line" =~ ^\"(.*)\"$ ]]; then line="${BASH_REMATCH[1]}"; fi
    if [[ "$line" =~ ^\'(.*)\'$ ]]; then line="${BASH_REMATCH[1]}"; fi
    printf '%s' "$line"
}

probe_key_server() {
    local base="${1%/}"
    local code curl_ec=0
    code="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 5 "${base}/service" 2>/dev/null)" || curl_ec=$?
    if [[ "$curl_ec" -ne 0 || -z "$code" || "$code" == "000" ]]; then
        echo "Warning: could not reach ${base} (curl exit ${curl_ec}, http_code=${code:-?})." >&2
        read -r -p "Continue anyway? [y/N] " ans
        [[ "${ans:-}" == [yY]* ]] || return 1
        return 0
    fi
    # 4xx (e.g. 400) means the process responded; bare GET /service may not match the API contract.
    if [[ "$code" -ge 500 ]]; then
        echo "Warning: ${base}/service returned HTTP ${code}." >&2
        read -r -p "Continue anyway? [y/N] " ans
        [[ "${ans:-}" == [yY]* ]] || return 1
    fi
    return 0
}

# Outputs hex of serialized EncryptedObject via global ENCRYPT_OUT_HEX; id as ENCRYPT_ID_HEX (lowercase hex, no 0x)
run_encrypt_hmac_cli() {
    local mydata_cli="$1"
    local msg_hex="$2"
    local package_id="$3"
    local id_hex="$4"
    local threshold="$5"
    local pk_naked_hex="$6"
    local ks_object_id="$7"
    local aad_hex="${8:-}"

    local -a cmd
    cmd=("$mydata_cli" encrypt-hmac --message "$msg_hex")
    if [[ -n "$aad_hex" ]]; then
        cmd+=(--aad "$aad_hex")
    fi
    cmd+=(--package-id "$package_id" --id "$id_hex" --threshold "$threshold" "$pk_naked_hex" -- "$ks_object_id")

    local out ec
    set +e
    out="$("${cmd[@]}" 2>&1)"
    ec=$?
    set -e
    if [[ $ec -ne 0 ]]; then
        echo "mydata encrypt-hmac failed:" >&2
        echo "$out" >&2
        return "$ec"
    fi

    local hex_line best cand len
    hex_line=""
    best=0
    while IFS= read -r cand; do
        cand="$(strip_0x "$cand")"
        [[ "$cand" == "$(strip_0x "$id_hex")" ]] && continue
        [[ "$cand" == "$pk_naked_hex" ]] && continue
        len="${#cand}"
        if [[ "$len" -gt "$best" && "$len" -ge 64 && $((len % 2)) -eq 0 ]]; then
            best="$len"
            hex_line="$cand"
        fi
    done < <(printf '%s' "$out" | grep -Eo '(0x)?[0-9a-fA-F]+' || true)
    if [[ ${#hex_line} -lt 64 ]]; then
        echo "Could not parse encrypt-hmac ciphertext hex (need >= 32 bytes). Raw output:" >&2
        echo "$out" >&2
        return 1
    fi
    ENCRYPT_OUT_HEX="$hex_line"
    ENCRYPT_ID_HEX="$(printf '%s' "$id_hex" | tr 'A-F' 'a-f')"
    return 0
}

show_context() {
    echo "=== Session context ==="
    echo "  Session save path:   $(session_state_save_path)"
    echo "  CLIENT_CONFIG:       ${CLIENT_CONFIG:-<unset>}"
    echo "  PKG_SOCIAL:          $PKG_SOCIAL"
    echo "  CLOCK_ID:            $CLOCK_ID"
    echo "  COIN_TYPE:           $COIN_TYPE"
    echo "  KEY_SERVER_URL:      $KEY_SERVER_URL"
    echo "  MYDATA_CONFIG_ID:    ${MYDATA_CONFIG_ID:-<unset>}"
    echo "  MYDATA_REGISTRY_ID:  ${MYDATA_REGISTRY_ID:-<unset>}"
    echo "  POOL_REGISTRY_ID:    ${POOL_REGISTRY_ID:-<unset>}"
    echo "  ANCHOR_REGISTRY_ID:  ${ANCHOR_REGISTRY_ID:-<unset>}"
    echo "  CLAIM_VAULT_ID:      ${CLAIM_VAULT_ID:-<unset>}"
    echo "  DIST_REGISTRY_ID:    ${DIST_REGISTRY_ID:-<unset>}"
    echo "  MYDATA_ADMIN_CAP_ID: ${MYDATA_ADMIN_CAP_ID:-<unset>}"
    echo "  POOL_ADMIN_CAP_ID:   ${POOL_ADMIN_CAP_ID:-<unset>}"
    echo "  Gas payment:         CLI auto-select"
    echo "  GAS_BUDGET:          ${GAS_BUDGET:-$DEFAULT_GAS_BUDGET} (MIST)"
    echo "  LISTING_ID:          ${LISTING_ID:-<unset>}"
    echo "  SUB_POOL_ID:         ${SUB_POOL_ID:-<unset>}"
    echo "  PAY_COIN_ID:         ${PAY_COIN_ID:-<unset>}"
    echo "  MYDATA_ENCRYPTION_ID: ${MYDATA_ENCRYPTION_ID:-<unset>}"
    echo "  MEMORY_ACCOUNT_ID:   ${MEMORY_ACCOUNT_ID:-<unset>}"
    echo "  REVOKE_BUYER_ID:     ${REVOKE_BUYER_ID:-<unset>}"
    echo "  MYDATA_SECRETS_FILE: ${MYDATA_SECRETS_FILE:-<unset>}"
    echo "  PUBLIC_KEY:          $(public_key_status_line)"
    echo "  KEY_SERVER_OBJECT_ID: ${KEY_SERVER_OBJECT_ID:-<unset>}"
    echo "  MYSO:               $(resolve_myso || true)"
    echo "  MYDATA:             $(resolve_mydata || true)"
    echo "======================="
}

# Sets globals: pk ks MYDATA_SECRETS_FILE KEY_SERVER_URL. Returns 1 on unrecoverable credential errors.
resolve_encrypt_credentials() {
    local sec_path default_sec pk ks
    if [[ -n "${MYDATA_SECRETS_FILE:-}" ]]; then
        default_sec="$(resolve_secrets_env_path "$MYDATA_SECRETS_FILE")"
    else
        default_sec="$(default_secrets_env_path)"
    fi

    if session_value_set PUBLIC_KEY && session_value_set KEY_SERVER_OBJECT_ID && \
        validate_g2_public_key_hex "${PUBLIC_KEY:-}" 2>/dev/null; then
        sec_path="${MYDATA_SECRETS_FILE:-$default_sec}"
        if [[ -f "$sec_path" ]]; then
            log_session_use "secrets env" "$sec_path"
            MYDATA_SECRETS_FILE="$sec_path"
        fi
        pk="${PUBLIC_KEY}"
        ks="${KEY_SERVER_OBJECT_ID}"
        log_session_use "PUBLIC_KEY" "valid (${G2_PUBLIC_KEY_HEX_LEN} hex)"
        log_session_use "KEY_SERVER_OBJECT_ID" "$ks"
    else
        if session_value_set MYDATA_SECRETS_FILE; then
            sec_path="$(resolve_secrets_env_path "$MYDATA_SECRETS_FILE")"
        else
            sec_path="$default_sec"
        fi
        if [[ -f "$sec_path" ]]; then
            hydrate_encrypt_from_secrets_file "$sec_path" 1
            MYDATA_SECRETS_FILE="$sec_path"
            log_session_use "secrets env" "$sec_path"
        elif ! session_value_set MYDATA_SECRETS_FILE; then
            sec_path="$(resolve_session_or_prompt MYDATA_SECRETS_FILE "local-mydata-secrets.env path (NOT key-server-config.yaml)" "$default_sec")"
            sec_path="$(resolve_secrets_env_path "$sec_path")"
            MYDATA_SECRETS_FILE="$sec_path"
            if [[ -f "$sec_path" ]]; then
                hydrate_encrypt_from_secrets_file "$sec_path" 1
            fi
        fi
        pk="${PUBLIC_KEY:-}"
        ks="${KEY_SERVER_OBJECT_ID:-}"
        if [[ -n "${pk:-}" ]] && validate_g2_public_key_hex "$pk" 2>/dev/null; then
            log_session_use "PUBLIC_KEY" "valid (${G2_PUBLIC_KEY_HEX_LEN} hex)"
        elif [[ -z "${pk:-}" ]]; then
            pk="$(prompt_with_default "PUBLIC_KEY (0x..., IBE G2 from genkey / key server; ${G2_PUBLIC_KEY_HEX_LEN} hex chars)" "")"
        fi
        if [[ -z "${ks:-}" ]]; then
            ks="$(prompt_with_default "KEY_SERVER_OBJECT_ID (on-chain KeyServer)" "")"
        elif session_value_set KEY_SERVER_OBJECT_ID; then
            log_session_use "KEY_SERVER_OBJECT_ID" "$ks"
        fi
    fi

    [[ -n "${ks:-}" ]] || { echo "KEY_SERVER_OBJECT_ID is required." >&2; return 1; }

    if ! validate_g2_public_key_hex "${pk:-}"; then
        echo "  Correct source: local-mydata-secrets.env from \`myso start --with-mydata\` (PUBLIC_KEY=…)." >&2
        echo "  Re-run menu [0] or delete PUBLIC_KEY from marketplace-session.env to auto-load from secrets file." >&2
        return 1
    fi

    if session_value_set KEY_SERVER_URL; then
        log_session_use "KEY_SERVER_URL" "$KEY_SERVER_URL"
    else
        KEY_SERVER_URL="$(prompt_with_default "Key server URL (probe before encrypt)" "$KEY_SERVER_URL")"
    fi

    PUBLIC_KEY="$pk"
    KEY_SERVER_OBJECT_ID="$ks"
    return 0
}

set_context_interactive() {
    echo "Set session values (Enter keeps default in brackets)."
    echo "Values are written to $(session_state_save_path) when you finish (override with MYDATA_MARKETPLACE_SESSION)."
    apply_session_defaults
    if [[ -z "${CLIENT_CONFIG:-}" ]]; then
        CLIENT_CONFIG="$PWD/network.config/client.yaml"
    fi
    echo "Using fixed defaults (set CLIENT_CONFIG / PKG_SOCIAL / CLOCK_ID / COIN_TYPE in the session file or env to override):"
    echo "  client.yaml path:           $CLIENT_CONFIG"
    echo "  Social package (MySoSocial): $PKG_SOCIAL"
    echo "  Clock object id:             $CLOCK_ID"
    echo "  MYSO coin type tag:          $COIN_TYPE"
    KEY_SERVER_URL="$(prompt_with_default "Key server base URL" "$KEY_SERVER_URL")"
    MYDATA_CONFIG_ID="$(prompt_with_default "MyDataConfig object id" "${MYDATA_CONFIG_ID:-}")"
    MYDATA_REGISTRY_ID="$(prompt_with_default "MyDataRegistry object id" "${MYDATA_REGISTRY_ID:-}")"
    POOL_REGISTRY_ID="$(prompt_with_default "MyDataPoolRegistry object id" "${POOL_REGISTRY_ID:-}")"
    ANCHOR_REGISTRY_ID="$(prompt_with_default "SnapshotAnchorRegistry object id" "${ANCHOR_REGISTRY_ID:-}")"
    CLAIM_VAULT_ID="$(prompt_with_default "MyDataClaimVault object id" "${CLAIM_VAULT_ID:-}")"
    DIST_REGISTRY_ID="$(prompt_with_default "DistributionRegistry object id" "${DIST_REGISTRY_ID:-}")"
    MYDATA_ADMIN_CAP_ID="$(prompt_with_default "MyDataAdminCap object id" "${MYDATA_ADMIN_CAP_ID:-}")"
    POOL_ADMIN_CAP_ID="$(prompt_with_default "MyDataPoolAdminCap object id" "${POOL_ADMIN_CAP_ID:-}")"
    MEMORY_CONFIG_ID="$(prompt_with_default "MemoryConfig object id" "${MEMORY_CONFIG_ID:-}")"
    ECOSYSTEM_TREASURY_ID="$(prompt_with_default "EcosystemTreasury object id" "${ECOSYSTEM_TREASURY_ID:-}")"
    BLOCK_LIST_REGISTRY_ID="$(prompt_with_default "BlockListRegistry object id" "${BLOCK_LIST_REGISTRY_ID:-}")"
    PLATFORM_OBJECT_ID="$(prompt_with_default "Platform object id (optional)" "${PLATFORM_OBJECT_ID:-}")"
    GAS_BUDGET="$(prompt_with_default "Gas budget in MIST (default 1 MYSO)" "${GAS_BUDGET:-$DEFAULT_GAS_BUDGET}")"
    LISTING_ID="$(prompt_with_default "Default listing (MyData) object id" "${LISTING_ID:-}")"
    SUB_POOL_ID="$(prompt_with_default "Default sub-pool id (menus 13–15; from menu 12 tx effects)" "${SUB_POOL_ID:-}")"
    PAY_COIN_ID="$(prompt_with_default "Default payment Coin<MYSO> id" "${PAY_COIN_ID:-}")"
    MYDATA_ENCRYPTION_ID="$(prompt_with_default "Default encryption_id (0x hex; from menu 2)" "${MYDATA_ENCRYPTION_ID:-}")"
    echo "  MemoryAccount id (used as default when menus 3/4/7 prompt):"
    MEMORY_ACCOUNT_ID="$(prompt_with_default "MemoryAccount object id (purchase/approve)" "${MEMORY_ACCOUNT_ID:-}")"
    MYDATA_SECRETS_FILE="$(prompt_with_default "local-mydata-secrets.env path (NOT key-server-config.yaml)" "${MYDATA_SECRETS_FILE:-}")"
    if [[ -n "${MYDATA_SECRETS_FILE:-}" ]]; then
        MYDATA_SECRETS_FILE="$(resolve_secrets_env_path "$MYDATA_SECRETS_FILE")"
    fi
    PUBLIC_KEY="$(prompt_with_default "PUBLIC_KEY for encrypt (optional; can use secrets file)" "${PUBLIC_KEY:-}")"
    KEY_SERVER_OBJECT_ID="$(prompt_with_default "KEY_SERVER_OBJECT_ID (optional)" "${KEY_SERVER_OBJECT_ID:-}")"
    if [[ -n "${PUBLIC_KEY:-}" ]] && ! validate_g2_public_key_hex "${PUBLIC_KEY:-}"; then
        echo "Warning: PUBLIC_KEY will not be saved — fix length or leave empty to load from secrets file." >&2
        PUBLIC_KEY=''
    fi
    show_context
    apply_session_defaults
    save_session_state
}

run_myso_call() {
    local func="$1"
    shift
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" ]] || { echo "myso binary not found. Set MYSO or build the myso crate." >&2; return 1; }
    [[ -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || { echo "Set a valid CLIENT_CONFIG (menu 0)." >&2; return 1; }

    local -a cmd
    cmd=("$myso" client --client.config "$CLIENT_CONFIG" call --package "$PKG_SOCIAL" --module mydata --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")

    echo "---"
    printf ' %q' "${cmd[@]}"
    echo
    echo "---"
    if [[ "${SKIP_CONFIRM_RUN:-}" == 1 ]]; then
        MYDATA_LAST_CALL_EXECUTED=1
        "${cmd[@]}"
    else
        if ! confirm_run; then
            MYDATA_LAST_CALL_EXECUTED=0
            echo "[skipped] command not executed (confirmation declined)." >&2
            return 0
        fi
        MYDATA_LAST_CALL_EXECUTED=1
        "${cmd[@]}"
    fi
}

menu_update_config() {
    require_session_fields MYDATA_ADMIN_CAP_ID MYDATA_CONFIG_ID CLOCK_ID || return 1
    local en max_tags max_sub max_grants max_enc_id max_data max_tag_bytes max_metadata
    local max_payment_ref max_pool_assignments max_proof_depth max_paid_entries claim_window
    local p2p_plat p2p_eco md_plat md_eco np_creator np_treasury
    load_mydata_config_params_from_graphql || true
    en="$(prompt_or_default "marketplace_enabled (new query/pool snapshots; true/false)" "${MYDATA_CFG_MARKETPLACE_ENABLED:-false}")"
    max_tags="$(prompt_or_default "max_tags" "${MYDATA_CFG_MAX_TAGS:-$DEFAULT_MAX_TAGS}")"
    max_sub="$(prompt_or_default "max_subscription_days" "${MYDATA_CFG_MAX_SUBSCRIPTION_DAYS:-$DEFAULT_MAX_SUBSCRIPTION_DAYS}")"
    max_grants="$(prompt_or_default "max_free_access_grants" "${MYDATA_CFG_MAX_FREE_ACCESS_GRANTS:-$DEFAULT_MAX_FREE_ACCESS_GRANTS}")"
    max_enc_id="$(prompt_or_default "max_encryption_id_bytes" "${MYDATA_CFG_MAX_ENCRYPTION_ID_BYTES:-$DEFAULT_MAX_ENCRYPTION_ID_BYTES}")"
    max_data="$(prompt_or_default "max_encrypted_data_bytes" "${MYDATA_CFG_MAX_ENCRYPTED_DATA_BYTES:-$DEFAULT_MAX_ENCRYPTED_DATA_BYTES}")"
    max_tag_bytes="$(prompt_or_default "max_tag_bytes" "${MYDATA_CFG_MAX_TAG_BYTES:-$DEFAULT_MAX_TAG_BYTES}")"
    max_metadata="$(prompt_or_default "max_metadata_bytes" "${MYDATA_CFG_MAX_METADATA_BYTES:-$DEFAULT_MAX_METADATA_BYTES}")"
    max_payment_ref="$(prompt_or_default "max_payment_reference_bytes" "${MYDATA_CFG_MAX_PAYMENT_REFERENCE_BYTES:-$DEFAULT_MAX_PAYMENT_REFERENCE_BYTES}")"
    max_pool_assignments="$(prompt_or_default "max_pool_assignments" "${MYDATA_CFG_MAX_POOL_ASSIGNMENTS:-$DEFAULT_MAX_POOL_ASSIGNMENTS}")"
    max_proof_depth="$(prompt_or_default "max_merkle_proof_depth" "${MYDATA_CFG_MAX_MERKLE_PROOF_DEPTH:-$DEFAULT_MAX_MERKLE_PROOF_DEPTH}")"
    max_paid_entries="$(prompt_or_default "max_paid_access_entries" "${MYDATA_CFG_MAX_PAID_ACCESS_ENTRIES:-$DEFAULT_MAX_PAID_ACCESS_ENTRIES}")"
    claim_window="$(prompt_or_default "default_claim_window_ms" "${MYDATA_CFG_DEFAULT_CLAIM_WINDOW_MS:-$DEFAULT_CLAIM_WINDOW_MS}")"
    p2p_plat="$(prompt_or_default "p2p_platform_fee_bps" "${MYDATA_CFG_P2P_PLATFORM_FEE_BPS:-$DEFAULT_P2P_PLATFORM_FEE_BPS}")"
    p2p_eco="$(prompt_or_default "p2p_ecosystem_fee_bps" "${MYDATA_CFG_P2P_ECOSYSTEM_FEE_BPS:-$DEFAULT_P2P_ECOSYSTEM_FEE_BPS}")"
    md_plat="$(prompt_or_default "mydata_marketplace_platform_fee_bps" "${MYDATA_CFG_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS:-$DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS}")"
    md_eco="$(prompt_or_default "mydata_marketplace_ecosystem_fee_bps" "${MYDATA_CFG_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS:-$DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS}")"
    np_creator="$(prompt_or_default "non_platform_platform_to_creator_bps" "${MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS:-$DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS}")"
    np_treasury="$(prompt_or_default "non_platform_platform_to_treasury_bps" "${MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS:-$DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS}")"
    run_update_mydata_config_call "$en" "$max_tags" "$max_sub" "$max_grants" "$max_enc_id" \
        "$max_data" "$max_tag_bytes" "$max_metadata" "$max_payment_ref" "$max_pool_assignments" \
        "$max_proof_depth" "$max_paid_entries" "$claim_window" \
        "$p2p_plat" "$p2p_eco" "$md_plat" "$md_eco" "$np_creator" "$np_treasury"
}

gql_mydata_configuration_snapshot() {
    graphql_post "$MYDATA_CONFIG_GQL"
}

load_mydata_config_params_from_graphql() {
    local resp
    resp="$(gql_mydata_configuration_snapshot 2>/dev/null)" || return 1
    MYDATA_CFG_MARKETPLACE_ENABLED="$(echo "$resp" | jq -r '.data.mydataConfiguration.marketplaceEnabled // empty')"
    MYDATA_CFG_MAX_TAGS="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxTags // empty')"
    MYDATA_CFG_MAX_SUBSCRIPTION_DAYS="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxSubscriptionDays // empty')"
    MYDATA_CFG_MAX_FREE_ACCESS_GRANTS="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxFreeAccessGrants // empty')"
    MYDATA_CFG_MAX_ENCRYPTION_ID_BYTES="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxEncryptionIdBytes // empty')"
    MYDATA_CFG_MAX_ENCRYPTED_DATA_BYTES="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxEncryptedDataBytes // empty')"
    MYDATA_CFG_MAX_TAG_BYTES="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxTagBytes // empty')"
    MYDATA_CFG_MAX_METADATA_BYTES="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxMetadataBytes // empty')"
    MYDATA_CFG_MAX_PAYMENT_REFERENCE_BYTES="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxPaymentReferenceBytes // empty')"
    MYDATA_CFG_MAX_POOL_ASSIGNMENTS="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxPoolAssignments // empty')"
    MYDATA_CFG_MAX_MERKLE_PROOF_DEPTH="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxMerkleProofDepth // empty')"
    MYDATA_CFG_MAX_PAID_ACCESS_ENTRIES="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxPaidAccessEntries // empty')"
    MYDATA_CFG_DEFAULT_CLAIM_WINDOW_MS="$(echo "$resp" | jq -r '.data.mydataConfiguration.defaultClaimWindowMs // empty')"
    MYDATA_CFG_P2P_PLATFORM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.p2PPlatformFeeBps // empty')"
    MYDATA_CFG_P2P_ECOSYSTEM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.p2PEcosystemFeeBps // empty')"
    MYDATA_CFG_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.mydataMarketplacePlatformFeeBps // empty')"
    MYDATA_CFG_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.mydataMarketplaceEcosystemFeeBps // empty')"
    MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.nonPlatformPlatformToCreatorBps // empty')"
    MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.nonPlatformPlatformToTreasuryBps // empty')"
}

run_update_mydata_config_call() {
    local marketplace_enabled="$1" max_tags="$2" max_sub="$3" max_grants="$4" max_enc_id="$5"
    local max_data="$6" max_tag_bytes="$7" max_metadata="$8" max_payment_ref="$9"
    local max_pool_assignments="${10}" max_proof_depth="${11}" max_paid_entries="${12}" claim_window="${13}"
    local p2p_plat="${14}" p2p_eco="${15}" md_plat="${16}" md_eco="${17}" np_creator="${18}" np_treasury="${19}"
    require_session_fields MYDATA_ADMIN_CAP_ID MYDATA_CONFIG_ID CLOCK_ID || return 1
    run_myso_call update_mydata_config \
        --args "$MYDATA_ADMIN_CAP_ID" "$MYDATA_CONFIG_ID" "$marketplace_enabled" \
        "$max_tags" "$max_sub" "$max_grants" "$max_enc_id" \
        "$max_data" "$max_tag_bytes" "$max_metadata" "$max_payment_ref" \
        "$max_pool_assignments" "$max_proof_depth" "$max_paid_entries" "$claim_window" \
        "$p2p_plat" "$p2p_eco" "$md_plat" "$md_eco" "$np_creator" "$np_treasury" \
        "$CLOCK_ID"
}

menu_create_and_share() {
    require_session_fields MYDATA_CONFIG_ID MYDATA_REGISTRY_ID || return 1

    local mydata_bin sec_file pk ks plaintext aad_opt
    mydata_bin="$(resolve_mydata)"
    [[ -n "$mydata_bin" ]] || {
        echo "mydata not found. Build myso-mydata (cargo build -p mydata-cli); set MYDATA, MYDATA_REPO, or use sibling ../myso-mydata." >&2
        return 1
    }

    resolve_encrypt_credentials || return 1
    pk="${PUBLIC_KEY}"
    ks="${KEY_SERVER_OBJECT_ID}"

    probe_key_server "$KEY_SERVER_URL" || return 1

    plaintext="$(prompt_or_default "Plaintext to encrypt" "Hello from MyData marketplace demo")"
    aad_opt="$(prompt_or_default "Optional encrypt-hmac --aad as hex (empty to skip)" "")"

    local msg_hex enc_id
    msg_hex="$(printf '%s' "$plaintext" | xxd -p -c 65536 | tr -d '\n')"
    enc_id="$(openssl rand -hex 32)"

    local pk_naked
    pk_naked="$(strip_0x "$pk")"

    echo ""
    echo "Running mydata encrypt-hmac (threshold=1, package-id=$PKG_SOCIAL, key server object=$ks)"
    ENCRYPT_OUT_HEX=''
    ENCRYPT_ID_HEX=''
    if [[ -n "$aad_opt" ]]; then
        run_encrypt_hmac_cli "$mydata_bin" "$msg_hex" "$PKG_SOCIAL" "$enc_id" 1 "$pk_naked" "$ks" "$(strip_0x "$aad_opt")"
    else
        run_encrypt_hmac_cli "$mydata_bin" "$msg_hex" "$PKG_SOCIAL" "$enc_id" 1 "$pk_naked" "$ks"
    fi

    local enc_arg id_arg
    enc_arg="\"0x${ENCRYPT_OUT_HEX}\""
    id_arg="\"0x${ENCRYPT_ID_HEX}\""

    local model media tags_json platform_opt tstart tend price subdur_raw geo dq sample coll upd freq function_name
    model="$(prompt_or_default "listing model (profile|one-time|recurring)" "one-time")"
    case "$model" in
        profile|one-time|recurring) ;;
        *) echo "listing model must be profile, one-time, or recurring" >&2; return 1 ;;
    esac
    media="$(prompt_or_default "media_type" "demo:bf-hmac-encrypt-hmac")"
    tags_json="$(prompt_or_default 'tags (JSON array of strings)' '["cli-demo"]')"
    platform_opt="$(prompt_or_default 'platform_id Option<address> — [] or ["0x..."]' '[]')"
    tstart="$(prompt_or_default "timestamp_start (u64)" "0")"
    tend="$(prompt_or_default 'timestamp_end Option — [] or ["123"]' '[]')"
    price=''
    subdur_raw=''
    if [[ "$model" == one-time ]]; then
        price="$(prompt_or_default 'one_time_price (MIST)' '1000000000')"
    elif [[ "$model" == recurring ]]; then
        price="$(prompt_or_default 'subscription_price (MIST)' '500000000')"
        subdur_raw="$(prompt_or_default "subscription_duration_days" "30")"
    fi
    geo="$(prompt_or_default "geographic_region Option<String> — [] or [\"US-CA\"]" '[]')"
    dq="$(prompt_or_default "data_quality Option<String> — [] or [\"high\"] (not a number)" '[]')"
    sample="$(prompt_or_default "sample_size Option<u64> — [] or [1000]" '[]')"
    coll="$(prompt_or_default "collection_method Option<String> — [] or [\"cli\"]" '[]')"
    upd="$(prompt_or_default "is_updating (true/false)" "false")"
    freq="$(prompt_or_default "update_frequency Option" '[]')"

    local out digest created_listing
    case "$model" in
        profile)
            function_name=create_and_share_profile_subscription_mydata
            out="$(run_myso_call "$function_name" \
                --args "$MYDATA_CONFIG_ID" "$MYDATA_REGISTRY_ID" "\"$media\"" "$tags_json" "$platform_opt" "$tstart" "$tend" \
                "$enc_arg" "$id_arg" "$geo" "$dq" "$sample" "$coll" "$upd" "$freq" "$CLOCK_ID")" || return 1
            ;;
        one-time)
            function_name=create_and_share_marketplace_one_time_mydata
            out="$(run_myso_call "$function_name" \
                --args "$MYDATA_CONFIG_ID" "$MYDATA_REGISTRY_ID" "\"$media\"" "$tags_json" "$platform_opt" "$tstart" "$tend" \
                "$enc_arg" "$id_arg" "$price" "$geo" "$dq" "$sample" "$coll" "$upd" "$freq" "$CLOCK_ID")" || return 1
            ;;
        recurring)
            function_name=create_and_share_marketplace_recurring_mydata
            out="$(run_myso_call "$function_name" \
                --args "$MYDATA_CONFIG_ID" "$MYDATA_REGISTRY_ID" "\"$media\"" "$tags_json" "$platform_opt" "$tstart" "$tend" \
                "$enc_arg" "$id_arg" "$price" "$subdur_raw" "$geo" "$dq" "$sample" "$coll" "$upd" "$freq" "$CLOCK_ID")" || return 1
            ;;
    esac

    digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    if [[ -n "$digest" ]]; then
        created_listing="$(extract_created_object_by_type "$digest" "mydata::MyData" 2>/dev/null || true)"
        [[ -n "$created_listing" ]] || created_listing="$(extract_created_object_by_type "$digest" "MyData" 2>/dev/null || true)"
        if [[ -n "$created_listing" ]]; then
            LISTING_ID="$(normalize_hex_id "$created_listing")"
            log_session_use "LISTING_ID" "$LISTING_ID"
        fi
    fi
    MYDATA_ENCRYPTION_ID="0x${ENCRYPT_ID_HEX:-}"
    log_session_use "MYDATA_ENCRYPTION_ID" "$MYDATA_ENCRYPTION_ID"

    apply_session_defaults
    save_session_state
    print_mydata_operation_summary "${function_name} (encrypt + list)" \
        "Listing model" "$model" \
        "Media type" "$media" \
        "Price" "${price:-n/a}" \
        "Subscription days" "${subdur_raw:-n/a}" \
        "Listing" "${LISTING_ID:-<unknown>}" \
        "Encrypted object id" "0x${ENCRYPT_ID_HEX:-}" \
        "Key server object" "$ks"
}

require_query_marketplace_enabled() {
    if ! load_mydata_config_params_from_graphql; then
        echo "Warning: could not read MyData configuration; the on-chain call will enforce marketplace_enabled." >&2
        return 0
    fi
    if [[ "${MYDATA_CFG_MARKETPLACE_ENABLED:-false}" != "true" ]]; then
        echo "New query/pool marketplace snapshots are disabled by MyDataConfig.marketplace_enabled." >&2
        echo "Use menu [1] to enable the query marketplace before recording a snapshot anchor." >&2
        return 1
    fi
}

menu_purchase_one_time() {
    ensure_mydata_purchase_shared_ids MYDATA_CONFIG_ID BLOCK_LIST_REGISTRY_ID MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID || return 1
    local listing pay account
    listing="$(resolve_purchase_listing_id)" || return 1
    assert_buyer_is_not_listing_owner "$listing" || return 1
    pay="$(resolve_purchase_pay_coin)" || return 1
    account="$(resolve_purchase_memory_account_id "MemoryAccount object id")" || return 1
    validate_purchase_coin_ownership "$pay" || return 1
    LISTING_ID="$listing"
    PAY_COIN_ID="$pay"
    MEMORY_ACCOUNT_ID="$account"
    save_session_state
    run_myso_call purchase_one_time --args "$MYDATA_CONFIG_ID" "$BLOCK_LIST_REGISTRY_ID" "$MEMORY_CONFIG_ID" \
        "$listing" "$ECOSYSTEM_TREASURY_ID" "$pay" "$account" "$CLOCK_ID"
    print_mydata_operation_summary "purchase_one_time" \
        "Listing" "$listing" \
        "Payment coin" "$pay" \
        "Memory account" "$account" \
        "Buyer / signer" "$(myso client active-address 2>/dev/null || echo '<active>')"
}

menu_purchase_sub() {
    ensure_mydata_purchase_shared_ids MYDATA_CONFIG_ID BLOCK_LIST_REGISTRY_ID MEMORY_CONFIG_ID ECOSYSTEM_TREASURY_ID || return 1
    local listing pay account
    listing="$(resolve_purchase_listing_id)" || return 1
    assert_buyer_is_not_listing_owner "$listing" || return 1
    pay="$(resolve_purchase_pay_coin)" || return 1
    account="$(resolve_purchase_memory_account_id "MemoryAccount object id")" || return 1
    validate_purchase_coin_ownership "$pay" || return 1
    LISTING_ID="$listing"
    PAY_COIN_ID="$pay"
    MEMORY_ACCOUNT_ID="$account"
    save_session_state
    run_myso_call purchase_subscription --args "$MYDATA_CONFIG_ID" "$BLOCK_LIST_REGISTRY_ID" "$MEMORY_CONFIG_ID" \
        "$listing" "$ECOSYSTEM_TREASURY_ID" "$pay" "$account" "$CLOCK_ID"
    print_mydata_operation_summary "purchase_subscription" \
        "Listing" "$listing" \
        "Payment coin" "$pay" \
        "Memory account" "$account" \
        "Buyer / signer" "$(myso client active-address 2>/dev/null || echo '<active>')"
}

menu_update_pricing() {
    require_session_fields MYDATA_CONFIG_ID || return 1
    local listing
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    local o sp dur
    o="$(prompt_or_default 'new_one_time_price Option' '["1500000000"]')"
    sp="$(prompt_or_default 'new_subscription_price Option' '["750000000"]')"
    dur="$(prompt_or_default 'new_subscription_duration_days Option' '["45"]')"
    run_myso_call update_pricing --args "$MYDATA_CONFIG_ID" "$listing" "$o" "$sp" "$dur" "$CLOCK_ID"
}

menu_update_content() {
    require_session_fields MYDATA_CONFIG_ID || return 1
    local listing ed eid tags
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    ed="$(prompt_or_default 'new_encrypted_data Option — [] or ["0x..."]' '[]')"
    eid="$(prompt_or_default 'new_encryption_id Option — [] or ["0x..."] (required with encrypted data)' '[]')"
    tags="$(prompt_or_default 'new_tags Option' '[]')"
    if [[ "$ed" == '[]' && "$eid" != '[]' ]] || [[ "$ed" != '[]' && "$eid" == '[]' ]]; then
        echo "new_encrypted_data and new_encryption_id must both be set or both be []" >&2
        return 1
    fi
    run_myso_call update_content --args "$MYDATA_CONFIG_ID" "$listing" "$ed" "$eid" "$tags" "$CLOCK_ID"
}

menu_mydata_approve() {
    ensure_mydata_purchase_shared_ids BLOCK_LIST_REGISTRY_ID MEMORY_CONFIG_ID || return 1
    local listing idv account
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    idv="$(prompt_with_default "encryption_id (0x hex, matches listing)" "${MYDATA_ENCRYPTION_ID:-}")"
    [[ -n "$idv" ]] || { echo "encryption id required." >&2; return 1; }
    idv="0x$(strip_0x "$idv")"
    MYDATA_ENCRYPTION_ID="$idv"
    account="$(resolve_purchase_memory_account_id "Listing owner's MemoryAccount object id")" || return 1
    MEMORY_ACCOUNT_ID="$account"
    save_session_state
    run_myso_call mydata_approve --args "\"$idv\"" "$BLOCK_LIST_REGISTRY_ID" "$MEMORY_CONFIG_ID" "$listing" "$account" "$CLOCK_ID"
    print_mydata_operation_summary "mydata_approve (access grant)" \
        "Listing" "$listing" \
        "Encryption id" "$idv" \
        "Memory account" "$account"
}

menu_grant_access() {
    require_session_fields MYDATA_CONFIG_ID || return 1
    local listing user at sd
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    user="$(prompt_with_default "beneficiary address" "")"
    at="$(prompt_or_default "access_type (0=one_time, 1=subscription)" "0")"
    sd="$(prompt_or_default "subscription_days Option" '[]')"
    run_myso_call grant_access --args "$MYDATA_CONFIG_ID" "$listing" "$user" "$at" "$sd" "$CLOCK_ID"
    print_mydata_operation_summary "grant_access" \
        "Listing" "$listing" \
        "Beneficiary" "$user" \
        "Access type" "$at" \
        "Subscription days" "$sd"
}

menu_revoke_access() {
    local listing user at
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    user="$(resolve_session_or_prompt REVOKE_BUYER_ID "Buyer address to revoke")"
    REVOKE_BUYER_ID="$user"
    at="$(prompt_or_default "access_type (0=one_time, 1=subscription, 2=both)" "0")"
    echo "Sign as listing owner. Revoked buyers cannot pass mydata_approve; fetch_key returns NoAccess." >&2
    run_myso_call revoke_access --args "$listing" "$user" "$at" "$CLOCK_ID"
    save_session_state
    print_mydata_operation_summary "revoke_access" \
        "Listing" "$listing" \
        "Revoked buyer" "$user" \
        "Access type" "$at"
}

menu_register() {
    require_session_fields MYDATA_REGISTRY_ID || return 1
    local listing
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    run_myso_call register_in_registry --args "$MYDATA_REGISTRY_ID" "$listing" "$CLOCK_ID"
}

menu_unregister() {
    require_session_fields MYDATA_REGISTRY_ID || return 1
    local ip
    ip="$(prompt_with_default "ip_id (listing address)" "")"
    run_myso_call unregister_from_registry --args "$MYDATA_REGISTRY_ID" "$ip" "$CLOCK_ID"
}

menu_create_broad_pool() {
    require_session_fields MYDATA_CONFIG_ID POOL_ADMIN_CAP_ID POOL_REGISTRY_ID || return 1
    local n d platform
    n="$(prompt_or_default "pool name" "demo-pool")"
    d="$(prompt_or_default "description" "CLI demo")"
    platform="$(prompt_or_default "bind platform? (y/N)" "n")"
    if [[ "$platform" == [yY]* ]]; then
        require_session_fields PLATFORM_OBJECT_ID || return 1
        run_myso_call create_broad_pool_with_platform --args "$MYDATA_CONFIG_ID" "$POOL_ADMIN_CAP_ID" "$POOL_REGISTRY_ID" \
            "$PLATFORM_OBJECT_ID" "\"$n\"" "\"$d\"" "$CLOCK_ID"
    else
        run_myso_call create_broad_pool --args "$MYDATA_CONFIG_ID" "$POOL_ADMIN_CAP_ID" "$POOL_REGISTRY_ID" "\"$n\"" "\"$d\"" "$CLOCK_ID"
    fi
}

menu_create_sub_pool() {
    require_session_fields MYDATA_CONFIG_ID POOL_ADMIN_CAP_ID POOL_REGISTRY_ID || return 1
    local bid n d
    bid="$(prompt_with_default "broad_pool_id (ID value)" "")"
    n="$(prompt_or_default "sub pool name" "demo-sub")"
    d="$(prompt_or_default "description" "CLI demo sub")"
    run_myso_call create_sub_pool --args "$MYDATA_CONFIG_ID" "$POOL_ADMIN_CAP_ID" "$POOL_REGISTRY_ID" "$bid" "\"$n\"" "\"$d\"" [] "$CLOCK_ID"
    echo "After tx succeeds, copy the sub-pool id from effects and save as SUB_POOL_ID (menu 0)." >&2
}

menu_assign_to_pools() {
    require_session_fields MYDATA_CONFIG_ID POOL_REGISTRY_ID || return 1
    local listing sub_raw sub_ids_vec
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    sub_raw="$(resolve_sub_pool_id "sub_pool_id(s), comma-separated")"
    [[ -n "$sub_raw" ]] || { echo "sub_pool_id is required." >&2; return 1; }
    SUB_POOL_ID="$sub_raw"
    sub_ids_vec="$(format_sub_pool_ids_vector "$sub_raw")" || return 1
    echo "Sign as listing owner (assign_mydata_to_pools checks mydata.owner)." >&2
    run_myso_call assign_mydata_to_pools \
        --args "$MYDATA_CONFIG_ID" "$listing" "$POOL_REGISTRY_ID" "$sub_ids_vec" "$CLOCK_ID"
    echo "Pool membership recorded on-chain (get_mydata_sub_pools / MyDataAssignedToSubPoolEvent)." >&2
}

menu_remove_from_pool() {
    require_session_fields POOL_REGISTRY_ID || return 1
    local listing sub_id
    listing="$(resolve_session_or_prompt LISTING_ID "MyData listing id")"
    LISTING_ID="$listing"
    sub_id="$(resolve_sub_pool_id "sub_pool_id to remove")"
    [[ -n "$sub_id" ]] || { echo "sub_pool_id is required." >&2; return 1; }
    SUB_POOL_ID="$sub_id"
    echo "Sign as listing owner (remove_mydata_from_sub_pools checks mydata.owner)." >&2
    run_myso_call remove_mydata_from_sub_pools \
        --args "$listing" "$POOL_REGISTRY_ID" "$sub_id" "$CLOCK_ID"
}

menu_record_anchor() {
    require_session_fields MYDATA_CONFIG_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID POOL_REGISTRY_ID || return 1
    require_query_marketplace_enabled || return 1
    local b_sub pay mf mf_hex pr pr_hex pc
    b_sub="$(prompt_with_default "source_pool_id (broad pool)" "")"
    pay="$(resolve_sub_pool_id "source_sub_pool_id")"
    [[ -n "$pay" ]] || { echo "source_sub_pool_id is required." >&2; return 1; }
    SUB_POOL_ID="$pay"
    mf="$(prompt_or_default "manifest_hash (exactly 32-byte hex)" "$(openssl rand -hex 32)")"
    mf_hex="$(strip_0x "$mf")"
    [[ "$mf_hex" =~ ^[0-9a-fA-F]{64}$ ]] || { echo "manifest_hash must be exactly 32 bytes (64 hex characters)" >&2; return 1; }
    pr="$(prompt_or_default "payment_reference (non-empty hex, max ${DEFAULT_MAX_PAYMENT_REFERENCE_BYTES} bytes)" "$(openssl rand -hex 16)")"
    pr_hex="$(strip_0x "$pr")"
    [[ "$pr_hex" =~ ^[0-9a-fA-F]+$ && $(( ${#pr_hex} % 2 )) -eq 0 && ${#pr_hex} -le $((DEFAULT_MAX_PAYMENT_REFERENCE_BYTES * 2)) ]] || {
        echo "payment_reference must be non-empty even-length hex within the configured byte limit" >&2; return 1;
    }
    pc="$(prompt_with_default "Coin<MYSO> object id" "")"
    validate_purchase_coin_ownership "$pc" || return 1
    run_myso_call record_snapshot_anchor \
        --args "$MYDATA_CONFIG_ID" "$ANCHOR_REGISTRY_ID" "$CLAIM_VAULT_ID" "$POOL_REGISTRY_ID" "$b_sub" "$pay" \
        "\"0x${mf_hex}\"" "\"0x${pr_hex}\"" "$pc" "$CLOCK_ID"
}

menu_deposit_snapshot_escrow() {
    require_session_fields POOL_ADMIN_CAP_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID || return 1
    local sid coin
    sid="$(prompt_with_default "snapshot_id" "")"
    coin="$(prompt_with_default "additional escrow Coin<MYSO> object id" "")"
    validate_purchase_coin_ownership "$coin" || return 1
    run_myso_call deposit_snapshot_escrow --args "$POOL_ADMIN_CAP_ID" "$ANCHOR_REGISTRY_ID" "$CLAIM_VAULT_ID" "$sid" "$coin" "$CLOCK_ID"
}

menu_publish_distribution() {
    require_session_fields MYDATA_CONFIG_ID POOL_ADMIN_CAP_ID ANCHOR_REGISTRY_ID DIST_REGISTRY_ID CLAIM_VAULT_ID || return 1
    local sid root root_hex total contributors
    sid="$(prompt_with_default "snapshot_id" "")"
    root="$(prompt_with_default "root_hash (32 bytes hex, with or without 0x)" "")"
    root_hex="$(strip_0x "$root")"
    [[ "$root_hex" =~ ^[0-9a-fA-F]{64}$ ]] || { echo "root_hash must be exactly 32 bytes" >&2; return 1; }
    total="$(prompt_with_default "total allocation (must equal funded escrow)" "")"
    contributors="$(prompt_with_default "positive contributor_count" "")"
    [[ "$total" =~ ^[1-9][0-9]*$ && "$contributors" =~ ^[1-9][0-9]*$ ]] || { echo "total and contributor_count must be positive integers" >&2; return 1; }
    run_myso_call publish_distribution --args "$MYDATA_CONFIG_ID" "$POOL_ADMIN_CAP_ID" "$ANCHOR_REGISTRY_ID" \
        "$DIST_REGISTRY_ID" "$CLAIM_VAULT_ID" "$sid" "\"0x${root_hex}\"" "$total" "$contributors" "$CLOCK_ID"
}

prompt_claim_inputs() {
    CLAIM_SNAPSHOT_ID="$(prompt_with_default "snapshot_id" "")"
    CLAIM_AMOUNT="$(prompt_with_default "positive claim amount" "")"
    CLAIM_LEAF_INDEX="$(prompt_with_default "leaf_index" "")"
    CLAIM_PROOF="$(prompt_or_default 'Merkle proof JSON vector, e.g. ["0x<64 hex>"]' '[]')"
    [[ "$CLAIM_AMOUNT" =~ ^[1-9][0-9]*$ && "$CLAIM_LEAF_INDEX" =~ ^[0-9]+$ ]] || { echo "claim amount must be positive and leaf_index non-negative" >&2; return 1; }
    echo "$CLAIM_PROOF" | jq -e 'type == "array" and all(.[]; type == "string" and test("^0x[0-9a-fA-F]{64}$"))' >/dev/null || {
        echo "proof must be a JSON array of 32-byte 0x-prefixed hashes" >&2; return 1;
    }
    local depth
    depth="$(echo "$CLAIM_PROOF" | jq 'length')"
    (( depth <= ${MYDATA_CFG_MAX_MERKLE_PROOF_DEPTH:-$DEFAULT_MAX_MERKLE_PROOF_DEPTH} )) || { echo "proof exceeds configured max depth" >&2; return 1; }
}

resolve_snapshot_platform_from_graphql() {
    local sid="$1" resp
    resp="$(graphql_post "query { mydataSnapshotAnchor(snapshotId: \"$sid\") { platformId } }")" || return 1
    echo "$resp" | jq -r '.data.mydataSnapshotAnchor.platformId // empty'
}

menu_claim() {
    require_session_fields MYDATA_CONFIG_ID DIST_REGISTRY_ID CLAIM_VAULT_ID ECOSYSTEM_TREASURY_ID || return 1
    load_mydata_config_params_from_graphql || true
    prompt_claim_inputs || return 1
    run_myso_call claim --args "$MYDATA_CONFIG_ID" "$DIST_REGISTRY_ID" "$CLAIM_VAULT_ID" "$ECOSYSTEM_TREASURY_ID" \
        "$CLAIM_SNAPSHOT_ID" "$CLAIM_AMOUNT" "$CLAIM_LEAF_INDEX" "$CLAIM_PROOF" "$CLOCK_ID"
}

menu_claim_with_platform() {
    require_session_fields MYDATA_CONFIG_ID DIST_REGISTRY_ID CLAIM_VAULT_ID ECOSYSTEM_TREASURY_ID PLATFORM_OBJECT_ID || return 1
    load_mydata_config_params_from_graphql || true
    prompt_claim_inputs || return 1
    local indexed_platform
    indexed_platform="$(resolve_snapshot_platform_from_graphql "$CLAIM_SNAPSHOT_ID")" || return 1
    [[ -n "$indexed_platform" ]] || { echo "snapshot is not platform-bound; use the non-platform claim path" >&2; return 1; }
    [[ "$(normalize_hex_id "$indexed_platform")" == "$(normalize_hex_id "$PLATFORM_OBJECT_ID")" ]] || {
        echo "session platform $PLATFORM_OBJECT_ID does not match indexed snapshot platform $indexed_platform" >&2; return 1;
    }
    run_myso_call claim_with_platform --args "$MYDATA_CONFIG_ID" "$DIST_REGISTRY_ID" "$CLAIM_VAULT_ID" "$ECOSYSTEM_TREASURY_ID" \
        "$PLATFORM_OBJECT_ID" "$CLAIM_SNAPSHOT_ID" "$CLAIM_AMOUNT" "$CLAIM_LEAF_INDEX" "$CLAIM_PROOF" "$CLOCK_ID"
}

menu_reclaim_expired() {
    require_session_fields ANCHOR_REGISTRY_ID DIST_REGISTRY_ID CLAIM_VAULT_ID || return 1
    local sid
    sid="$(prompt_with_default "snapshot_id (sign as original buyer after deadline)" "")"
    run_myso_call reclaim_expired_snapshot_escrow --args "$ANCHOR_REGISTRY_ID" "$DIST_REGISTRY_ID" "$CLAIM_VAULT_ID" "$sid" "$CLOCK_ID"
}

main_menu() {
    while true; do
        echo ""
        echo "MyData marketplace (social_contracts::mydata)"
        echo " 0) Refresh session from GraphQL"
        echo " s) Set / show session context (manual secrets + listing ids)"
        echo " 1) update_mydata_config"
        echo " 2) create and share listing (profile, one-time, or recurring)"
        echo " 3) purchase_one_time"
        echo " 4) purchase_subscription"
        echo " 5) update_pricing"
        echo " 6) update_content"
        echo " 7) mydata_approve"
        echo " 8) grant_access"
        echo " 9) register_in_registry"
        echo "10) unregister_from_registry"
        echo "11) create_broad_pool"
        echo "12) create_sub_pool"
        echo "13) assign_mydata_to_pools (owner)"
        echo "14) remove_mydata_from_sub_pools (owner)"
        echo "15) record_snapshot_anchor"
        echo "16) deposit_snapshot_escrow"
        echo "17) publish_distribution (atomic root + allocation)"
        echo "18) claim"
        echo "19) claim_with_platform (platform derived from indexed snapshot)"
        echo "20) reclaim_expired_snapshot_escrow (buyer)"
        echo "21) revoke_access (owner; blocks future fetch_key for buyer)"
        echo " q) Quit"
        local c
        read -r -p "Choice: " c || break
        case "$c" in
            0) refresh_mydata_marketplace_session_from_graphql ;;
            s|S) set_context_interactive ;;
            1) menu_update_config ;;
            2) menu_create_and_share ;;
            3) menu_purchase_one_time ;;
            4) menu_purchase_sub ;;
            5) menu_update_pricing ;;
            6) menu_update_content ;;
            7) menu_mydata_approve ;;
            8) menu_grant_access ;;
            9) menu_register ;;
            10) menu_unregister ;;
            11) menu_create_broad_pool ;;
            12) menu_create_sub_pool ;;
            13) menu_assign_to_pools ;;
            14) menu_remove_from_pool ;;
            15) menu_record_anchor ;;
            16) menu_deposit_snapshot_escrow ;;
            17) menu_publish_distribution ;;
            18) menu_claim ;;
            19) menu_claim_with_platform ;;
            20) menu_reclaim_expired ;;
            21) menu_revoke_access ;;
            q|Q) break ;;
            *) echo "Unknown choice." ;;
        esac
    done
}

mydata_marketplace_main() {
    local arg
    for arg in "$@"; do
        case "$arg" in
            -h|--help)
                usage
                return 0
                ;;
            -y)
                ASSUME_YES=1
                ;;
            --no-session)
                NO_SESSION_FILE=1
                ;;
            --refresh-session)
                DO_REFRESH=1
                ;;
            --no-auto-refresh)
                MYDATA_NO_AUTO_REFRESH=1
                ;;
        esac
    done

    load_session_state
    maybe_auto_refresh_mydata_session
    load_session_state

    if [[ "$DO_REFRESH" == 1 ]]; then
        refresh_mydata_marketplace_session_from_graphql
        return 0
    fi

    main_menu
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    mydata_marketplace_main "$@"
fi
