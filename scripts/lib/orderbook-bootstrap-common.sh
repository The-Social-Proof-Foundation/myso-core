#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared helpers for scripts/orderbook-bootstrap.sh.
# Source after scripts/lib/social-runtime-common.sh.
# All on-chain IDs are GraphQL/wallet discovered — none are required as user input.
# Env overrides (PKG_MYUSD, ORDERBOOK_REGISTRY_ID, …) are debug-only.

if [[ -n "${_ORDERBOOK_BOOTSTRAP_COMMON_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_ORDERBOOK_BOOTSTRAP_COMMON_SOURCED=1

readonly ORDERBOOK_DEFAULT_PKG='0x0000000000000000000000000000000000000000000000000000000000000b0c'
readonly ORDERBOOK_DEFAULT_REGISTRY='0x0000000000000000000000000000000000000000000000000000000000000010'
readonly ORDERBOOK_DEFAULT_API_URL='http://127.0.0.1:9008'
readonly ORDERBOOK_DEFAULT_ADMIN_TOKEN='bearer_admin_token'
readonly MYSO_COIN_TYPE='0x2::myso::MYSO'
readonly MYSO_DECIMALS=9
readonly MYUSD_DECIMALS=6
readonly BTC_DECIMALS=8
readonly ETH_DECIMALS=8

ORDERBOOK_PACKAGE_ID="${ORDERBOOK_PACKAGE_ID:-$ORDERBOOK_DEFAULT_PKG}"
ORDERBOOK_API_URL="${ORDERBOOK_API_URL:-$ORDERBOOK_DEFAULT_API_URL}"
ORDERBOOK_ADMIN_TOKEN="${ORDERBOOK_ADMIN_TOKEN:-$ORDERBOOK_DEFAULT_ADMIN_TOKEN}"

ORDERBOOK_REGISTRY_ID="${ORDERBOOK_REGISTRY_ID:-}"
ORDERBOOK_ADMIN_CAP_ID="${ORDERBOOK_ADMIN_CAP_ID:-}"
COIN_CREATION_ADMIN_CAP_ID="${COIN_CREATION_ADMIN_CAP_ID:-}"
PACKAGE_PUBLISH_ADMIN_CAP_ID="${PACKAGE_PUBLISH_ADMIN_CAP_ID:-}"

PKG_MYUSD="${PKG_MYUSD:-}"
PKG_BTC="${PKG_BTC:-}"
PKG_ETH="${PKG_ETH:-}"
MYUSD_COIN_TYPE="${MYUSD_COIN_TYPE:-}"
BTC_COIN_TYPE="${BTC_COIN_TYPE:-}"
ETH_COIN_TYPE="${ETH_COIN_TYPE:-}"
MYUSD_TREASURY_CAP_ID="${MYUSD_TREASURY_CAP_ID:-}"
BTC_TREASURY_CAP_ID="${BTC_TREASURY_CAP_ID:-}"
ETH_TREASURY_CAP_ID="${ETH_TREASURY_CAP_ID:-}"
MYSO_MYUSD_POOL_ID="${MYSO_MYUSD_POOL_ID:-}"
BTC_MYUSD_POOL_ID="${BTC_MYUSD_POOL_ID:-}"
ETH_MYUSD_POOL_ID="${ETH_MYUSD_POOL_ID:-}"
MYUSD_STABLECOIN_REGISTERED="${MYUSD_STABLECOIN_REGISTERED:-}"

# Move requires lot_size >= 1000, powers of ten, and min_size % lot_size == 0.
# Catalog examples used min_size=100; that aborts on-chain, so min is 1000.
# MYSO/MYUSD tick must be 1000 so a $0.0047 mid (internal 4700) can quote.
readonly TICK_SIZE_MYSO="${TICK_SIZE_MYSO:-1000}"
TICK_SIZE_BTC="${TICK_SIZE_BTC:-100}"
TICK_SIZE_ETH="${TICK_SIZE_ETH:-100}"
LOT_SIZE="${LOT_SIZE:-1000}"
MIN_SIZE="${MIN_SIZE:-1000}"

MYUSD_MINT_AMOUNT="${MYUSD_MINT_AMOUNT:-1000000000}"
# 8-decimal base units: 100 BTC, 100 ETH.
BTC_MINT_AMOUNT="${BTC_MINT_AMOUNT:-10000000000}"
ETH_MINT_AMOUNT="${ETH_MINT_AMOUNT:-10000000000}"

# Move build environment for packages with [environments] in Move.toml (e.g. myusd).
# Use -e localnet so publish works after a localnet wipe (CLI chain id != Move.toml id).
# Set ORDERBOOK_PUBLISH_ENV= to skip passing -e entirely.
ORDERBOOK_PUBLISH_ENV="${ORDERBOOK_PUBLISH_ENV:-localnet}"
# Rewrite Move.toml for publish (strip local framework deps, ensure env slot), then restore.
ORDERBOOK_MANAGE_MOVE_TOML="${ORDERBOOK_MANAGE_MOVE_TOML:-1}"
ORDERBOOK_MOVE_TOML_BAK_SUFFIX='.orderbook-bak'

PYTH_PACKAGE_ID="${PYTH_PACKAGE_ID:-}"
MYUSD_PRICE_INFO_OBJECT_ID="${MYUSD_PRICE_INFO_OBJECT_ID:-}"
MYSO_PRICE_INFO_OBJECT_ID="${MYSO_PRICE_INFO_OBJECT_ID:-}"
BTC_PRICE_INFO_OBJECT_ID="${BTC_PRICE_INFO_OBJECT_ID:-}"
ETH_PRICE_INFO_OBJECT_ID="${ETH_PRICE_INFO_OBJECT_ID:-}"
ORACLE_PRIVATE_KEY="${ORACLE_PRIVATE_KEY:-}"
ORACLE_ADDRESS="${ORACLE_ADDRESS:-}"
PRIVATE_KEY="${PRIVATE_KEY:-}"
DEPLOYER_ADDRESS="${DEPLOYER_ADDRESS:-}"
MM_POOLS="${MM_POOLS:-}"

ORDERBOOK_SANDBOX_DIR="${ORDERBOOK_SANDBOX_DIR:-$REPO_ROOT/../orderbook-sandbox-main/sandbox}"
ORDERBOOK_DB_URL="${ORDERBOOK_DB_URL:-postgresql://postgres@localhost:5432/orderbook}"
# Override MM_MYSO_MYUSD_FALLBACK_MID (internal quote units: 4700 ≈ $0.0047 MYUSD/MYSO).

ORDERBOOK_SESSION_KEYS=(
    ORDERBOOK_PACKAGE_ID CLOCK_ID COIN_TYPE GAS_BUDGET
    ORDERBOOK_REGISTRY_ID ORDERBOOK_ADMIN_CAP_ID
    COIN_CREATION_ADMIN_CAP_ID PACKAGE_PUBLISH_ADMIN_CAP_ID
    PKG_MYUSD PKG_BTC PKG_ETH
    MYUSD_COIN_TYPE BTC_COIN_TYPE ETH_COIN_TYPE
    MYUSD_TREASURY_CAP_ID BTC_TREASURY_CAP_ID ETH_TREASURY_CAP_ID
    MYSO_MYUSD_POOL_ID BTC_MYUSD_POOL_ID ETH_MYUSD_POOL_ID
    MYUSD_STABLECOIN_REGISTERED
    PYTH_PACKAGE_ID MYUSD_PRICE_INFO_OBJECT_ID MYSO_PRICE_INFO_OBJECT_ID
    BTC_PRICE_INFO_OBJECT_ID ETH_PRICE_INFO_OBJECT_ID ORACLE_PRIVATE_KEY ORACLE_ADDRESS
    PRIVATE_KEY DEPLOYER_ADDRESS MM_POOLS
)

readonly ORDERBOOK_GQL_BATCH='query OrderbookBootstrapObjects($active: MySoAddress!) {
  orderbookRegistry: objects(
    filter: { type: "0xb0c::registry::Registry", ownerKind: SHARED }, first: 1
  ) { nodes { address } }

  coinCreationAdminCap: objects(
    filter: { type: "0x2::coin::CoinCreationAdminCap", ownerKind: ADDRESS, owner: $active }, last: 1
  ) { nodes { address } }

  packagePublishingAdminCap: objects(
    filter: { type: "0x2::package::PackagePublishingAdminCap", ownerKind: ADDRESS, owner: $active }, last: 1
  ) { nodes { address } }

  orderbookAdminCap: objects(
    filter: { type: "0xb0c::registry::OrderbookAdminCap", ownerKind: ADDRESS, owner: $active }, last: 1
  ) { nodes { address } }

}'

readonly ORDERBOOK_GQL_TREASURIES='query OrderbookOwnedTreasuries($active: MySoAddress!) {
  ownedTreasuries: objects(
    filter: { type: "0x2::coin::TreasuryCap", ownerKind: ADDRESS, owner: $active }, last: 20
  ) { nodes { address contents { type { repr } } } }
}'

orderbook_apply_defaults() {
    [[ -n "${ORDERBOOK_PACKAGE_ID:-}" ]] || ORDERBOOK_PACKAGE_ID="$ORDERBOOK_DEFAULT_PKG"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$SOCIAL_DEFAULT_CLOCK"
    [[ -n "${COIN_TYPE:-}" ]] || COIN_TYPE="$MYSO_COIN_TYPE"
    [[ -n "${GAS_BUDGET:-}" ]] || GAS_BUDGET="$SOCIAL_DEFAULT_GAS_BUDGET"
    [[ -n "${ORDERBOOK_API_URL:-}" ]] || ORDERBOOK_API_URL="$ORDERBOOK_DEFAULT_API_URL"
    [[ -n "${ORDERBOOK_ADMIN_TOKEN:-}" ]] || ORDERBOOK_ADMIN_TOKEN="$ORDERBOOK_DEFAULT_ADMIN_TOKEN"
}

orderbook_save_session() {
    social_save_session "${ORDERBOOK_SESSION_KEYS[@]}"
}

orderbook_load_session() {
    social_load_session
    orderbook_apply_defaults
}

orderbook_clear_stale_id() {
    local env_key="$1"
    local id="${!env_key:-}"
    [[ -n "$id" ]] || return 0
    if ! object_exists_on_fullnode "$id"; then
        echo "Clearing stale ${env_key}=${id} (not on fullnode)" >&2
        printf -v "$env_key" '%s' ''
    else
        printf -v "$env_key" '%s' "$(normalize_hex_id "$id")"
    fi
}

orderbook_validate_session_ids() {
    local key
    for key in ORDERBOOK_REGISTRY_ID ORDERBOOK_ADMIN_CAP_ID \
        COIN_CREATION_ADMIN_CAP_ID PACKAGE_PUBLISH_ADMIN_CAP_ID \
        PKG_MYUSD PKG_BTC PKG_ETH \
        MYUSD_TREASURY_CAP_ID BTC_TREASURY_CAP_ID ETH_TREASURY_CAP_ID \
        MYSO_MYUSD_POOL_ID BTC_MYUSD_POOL_ID ETH_MYUSD_POOL_ID \
        PYTH_PACKAGE_ID MYUSD_PRICE_INFO_OBJECT_ID MYSO_PRICE_INFO_OBJECT_ID \
        BTC_PRICE_INFO_OBJECT_ID ETH_PRICE_INFO_OBJECT_ID; do
        orderbook_clear_stale_id "$key"
    done
    if [[ -n "${PKG_MYUSD:-}" ]]; then
        MYUSD_COIN_TYPE="${PKG_MYUSD}::myusd::MYUSD"
    else
        MYUSD_COIN_TYPE=''
        MYUSD_TREASURY_CAP_ID=''
    fi
    if [[ -n "${PKG_BTC:-}" ]]; then
        BTC_COIN_TYPE="${PKG_BTC}::btc::BTC"
    else
        BTC_COIN_TYPE=''
        BTC_TREASURY_CAP_ID=''
    fi
    if [[ -n "${PKG_ETH:-}" ]]; then
        ETH_COIN_TYPE="${PKG_ETH}::eth::ETH"
    else
        ETH_COIN_TYPE=''
        ETH_TREASURY_CAP_ID=''
    fi
    if [[ -n "${MYUSD_TREASURY_CAP_ID:-}" ]] && ! object_exists_on_fullnode "$MYUSD_TREASURY_CAP_ID"; then
        MYUSD_TREASURY_CAP_ID=''
    fi
    if [[ -n "${BTC_TREASURY_CAP_ID:-}" ]] && ! object_exists_on_fullnode "$BTC_TREASURY_CAP_ID"; then
        BTC_TREASURY_CAP_ID=''
    fi
    if [[ -n "${ETH_TREASURY_CAP_ID:-}" ]] && ! object_exists_on_fullnode "$ETH_TREASURY_CAP_ID"; then
        ETH_TREASURY_CAP_ID=''
    fi
}

orderbook_cap_owned_by() {
    local cap="$1" expected="$2" owner
    cap="$(normalize_hex_id "$cap")" || return 1
    expected="$(normalize_hex_id "$expected")" || return 1
    owner="$(object_address_owner "$cap")" || return 1
    [[ "$(normalize_hex_id "$owner")" == "$expected" ]]
}

orderbook_resolve_owned_cap_wallet() {
    local addr="$1" type_fragment="$2"
    local json
    json="$(myso client objects "$addr" --json 2>/dev/null)" || return 1
    echo "$json" | jq -r --arg t "$type_fragment" '
        def move_type:
            .type? // .objectType? // .object_type? // .data.type? // .data.objectType? //
            (
                if (.data.Move.type_? | type) == "string" then
                    .data.Move.type_
                elif (.data.Move.type_.Other? | type) == "object" then
                    ((.data.Move.type_.Other.address? // "") + "::" +
                     (.data.Move.type_.Other.module? // "") + "::" +
                     (.data.Move.type_.Other.name? // ""))
                else
                    ""
                end
            );
        .[]?
        | select((move_type | tostring) | contains($t))
        | .data.objectId // .objectId // .object_id // .address // empty
    ' | head -n1
}

orderbook_bind_cap() {
    local env_key="$1" gql_id="$2" type_fragment="$3" active="$4"
    local candidate
    if [[ -n "$gql_id" ]]; then
        candidate="$(normalize_hex_id "$gql_id")" || candidate=''
        if [[ -n "$candidate" ]] && orderbook_cap_owned_by "$candidate" "$active"; then
            printf -v "$env_key" '%s' "$candidate"
            log_session_use "$env_key" "${!env_key}"
            return 0
        fi
        if [[ -n "$candidate" ]]; then
            echo "GraphQL ${env_key}=${candidate} not owned by ${active}; trying wallet" >&2
        fi
    fi
    candidate="$(orderbook_resolve_owned_cap_wallet "$active" "$type_fragment")" || candidate=''
    if [[ -n "$candidate" ]]; then
        candidate="$(normalize_hex_id "$candidate")"
        if orderbook_cap_owned_by "$candidate" "$active"; then
            printf -v "$env_key" '%s' "$candidate"
            log_session_use "$env_key" "${!env_key}"
            return 0
        fi
    fi
    printf -v "$env_key" '%s' ''
    return 1
}

orderbook_discover_published_tokens() {
    local json="$1" active="$2"
    local addr typ inner pkg
    while IFS=$'\t' read -r addr typ; do
        [[ -n "$addr" && -n "$typ" ]] || continue
        inner="$(printf '%s' "$typ" | sed -n 's/.*TreasuryCap<\(.*\)>.*/\1/p')"
        [[ -n "$inner" ]] || continue
        pkg="${inner%%::*}"
        pkg="$(normalize_hex_id "$pkg" 2>/dev/null || true)"
        [[ -n "$pkg" ]] || continue
        addr="$(normalize_hex_id "$addr")" || continue
        object_exists_on_fullnode "$addr" || continue
        case "$inner" in
            *::myusd::MYUSD)
                PKG_MYUSD="$pkg"
                MYUSD_COIN_TYPE="${pkg}::myusd::MYUSD"
                MYUSD_TREASURY_CAP_ID="$addr"
                log_session_use "MYUSD_TREASURY_CAP_ID" "$addr"
                ;;
            *::btc::BTC)
                PKG_BTC="$pkg"
                BTC_COIN_TYPE="${pkg}::btc::BTC"
                BTC_TREASURY_CAP_ID="$addr"
                log_session_use "BTC_TREASURY_CAP_ID" "$addr"
                ;;
            *::eth::ETH)
                PKG_ETH="$pkg"
                ETH_COIN_TYPE="${pkg}::eth::ETH"
                ETH_TREASURY_CAP_ID="$addr"
                log_session_use "ETH_TREASURY_CAP_ID" "$addr"
                ;;
        esac
    done < <(echo "$json" | jq -r '
        (.data.ownedTreasuries.nodes // [])[]
        | [(.address // ""), (.contents.type.repr // "")]
        | @tsv
    ')
    orderbook_discover_treasuries_from_wallet "$active"
}

orderbook_discover_treasuries_from_wallet() {
    local active="$1" json
    [[ -n "${MYUSD_TREASURY_CAP_ID:-}" && -n "${BTC_TREASURY_CAP_ID:-}" && -n "${ETH_TREASURY_CAP_ID:-}" ]] && return 0
    json="$(myso client objects "$active" --json 2>/dev/null)" || return 0
    local addr typ inner pkg
    while IFS=$'\t' read -r addr typ; do
        [[ -n "$addr" && -n "$typ" ]] || continue
        inner="$(printf '%s' "$typ" | sed -n 's/.*TreasuryCap<\(.*\)>.*/\1/p')"
        [[ -n "$inner" ]] || continue
        pkg="${inner%%::*}"
        pkg="$(normalize_hex_id "$pkg" 2>/dev/null || true)"
        [[ -n "$pkg" ]] || continue
        addr="$(normalize_hex_id "$addr")" || continue
        case "$inner" in
            *::myusd::MYUSD)
                [[ -n "${MYUSD_TREASURY_CAP_ID:-}" ]] && continue
                PKG_MYUSD="$pkg"
                MYUSD_COIN_TYPE="${pkg}::myusd::MYUSD"
                MYUSD_TREASURY_CAP_ID="$addr"
                log_session_use "MYUSD_TREASURY_CAP_ID" "$addr"
                ;;
            *::btc::BTC)
                [[ -n "${BTC_TREASURY_CAP_ID:-}" ]] && continue
                PKG_BTC="$pkg"
                BTC_COIN_TYPE="${pkg}::btc::BTC"
                BTC_TREASURY_CAP_ID="$addr"
                log_session_use "BTC_TREASURY_CAP_ID" "$addr"
                ;;
            *::eth::ETH)
                [[ -n "${ETH_TREASURY_CAP_ID:-}" ]] && continue
                PKG_ETH="$pkg"
                ETH_COIN_TYPE="${pkg}::eth::ETH"
                ETH_TREASURY_CAP_ID="$addr"
                log_session_use "ETH_TREASURY_CAP_ID" "$addr"
                ;;
        esac
    done < <(echo "$json" | jq -r '
        def move_type:
            .type? // .objectType? // .object_type? // .data.type? // .data.objectType? //
            (
                if (.data.Move.type_? | type) == "string" then
                    .data.Move.type_
                elif (.data.Move.type_.Other? | type) == "object" then
                    ((.data.Move.type_.Other.address? // "") + "::" +
                     (.data.Move.type_.Other.module? // "") + "::" +
                     (.data.Move.type_.Other.name? // ""))
                else
                    ""
                end
            );
        .[]?
        | [( .data.objectId // .objectId // .object_id // .address // ""), (move_type | tostring)]
        | select(.[1] | contains("TreasuryCap"))
        | @tsv
    ')
}

orderbook_resolve_admin_caps() {
    local active="$1" json="${2:-}"
    local gql_coin gql_pkg gql_ob
    gql_coin="$(gql_object_address "${json:-{\}}" "coinCreationAdminCap")"
    gql_pkg="$(gql_object_address "${json:-{\}}" "packagePublishingAdminCap")"
    gql_ob="$(gql_object_address "${json:-{\}}" "orderbookAdminCap")"
    orderbook_bind_cap COIN_CREATION_ADMIN_CAP_ID "$gql_coin" "CoinCreationAdminCap" "$active" || true
    orderbook_bind_cap PACKAGE_PUBLISH_ADMIN_CAP_ID "$gql_pkg" "PackagePublishingAdminCap" "$active" || true
    orderbook_bind_cap ORDERBOOK_ADMIN_CAP_ID "$gql_ob" "OrderbookAdminCap" "$active" || true
    if [[ -z "${COIN_CREATION_ADMIN_CAP_ID:-}" || -z "${PACKAGE_PUBLISH_ADMIN_CAP_ID:-}" \
        || -z "${ORDERBOOK_ADMIN_CAP_ID:-}" ]]; then
        echo "Missing admin cap(s) for $active — run ./scripts/bootstrap.sh and wait for indexer sync" >&2
        echo "  CoinCreationAdminCap=${COIN_CREATION_ADMIN_CAP_ID:-<unset>}" >&2
        echo "  PackagePublishingAdminCap=${PACKAGE_PUBLISH_ADMIN_CAP_ID:-<unset>}" >&2
        echo "  OrderbookAdminCap=${ORDERBOOK_ADMIN_CAP_ID:-<unset>}" >&2
        echo "IDs are auto-discovered from GraphQL/wallet; do not paste them into env files." >&2
        return 1
    fi
}

orderbook_graphql_refresh_once() {
    local active="$1" vars json registry
    vars="$(jq -nc --arg active "$active" '{active: $active}')" || return 1
    json="$(graphql_post "$ORDERBOOK_GQL_BATCH" "$vars")" || return 1
    registry="$(gql_object_address "$json" "orderbookRegistry")"
    if [[ -n "$registry" ]] && object_exists_on_fullnode "$registry"; then
        ORDERBOOK_REGISTRY_ID="$(normalize_hex_id "$registry")"
    elif [[ -n "${ORDERBOOK_REGISTRY_ID:-}" ]] && object_exists_on_fullnode "$ORDERBOOK_REGISTRY_ID"; then
        ORDERBOOK_REGISTRY_ID="$(normalize_hex_id "$ORDERBOOK_REGISTRY_ID")"
    elif object_exists_on_fullnode "$ORDERBOOK_DEFAULT_REGISTRY"; then
        ORDERBOOK_REGISTRY_ID="$ORDERBOOK_DEFAULT_REGISTRY"
    else
        echo "Could not resolve live ORDERBOOK_REGISTRY_ID from GraphQL or genesis 0x10" >&2
        return 1
    fi
    log_session_use "ORDERBOOK_REGISTRY_ID" "$ORDERBOOK_REGISTRY_ID"
    orderbook_resolve_admin_caps "$active" "$json" || return 1
    local treasuries
    treasuries="$(graphql_post "$ORDERBOOK_GQL_TREASURIES" "$vars" 2>/dev/null)" || treasuries='{}'
    orderbook_discover_published_tokens "$treasuries" "$active"
    return 0
}

orderbook_refresh_session_from_graphql() {
    local active attempt max=12
    command -v curl >/dev/null 2>&1 || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }
    active="$(resolve_myso_active_address)" || {
        echo "Could not resolve myso client active-address" >&2
        return 1
    }
    active="$(normalize_hex_id "$active")" || return 1
    if ! graphql_is_reachable; then
        echo "GraphQL unreachable at $GRAPHQL_URL — start myso start --with-indexer --with-orderbook" >&2
        return 1
    fi
    for ((attempt = 1; attempt <= max; attempt++)); do
        log_step "Refreshing orderbook session from GraphQL (attempt $attempt/$max)"
        if orderbook_graphql_refresh_once "$active"; then
            orderbook_apply_defaults
            return 0
        fi
        sleep 2
    done
    echo "GraphQL refresh incomplete; trying wallet-only cap resolution" >&2
    if object_exists_on_fullnode "$ORDERBOOK_DEFAULT_REGISTRY"; then
        ORDERBOOK_REGISTRY_ID="$ORDERBOOK_DEFAULT_REGISTRY"
    fi
    orderbook_resolve_admin_caps "$active" '{}' || return 1
    orderbook_discover_published_tokens '{}' "$active"
    orderbook_apply_defaults
}

orderbook_extract_json() {
    local raw="$1"
    if printf '%s' "$raw" | jq -e . >/dev/null 2>&1; then
        printf '%s' "$raw"
        return 0
    fi
    python3 -c '
import json, sys
text = sys.argv[1]
last = None
i = 0
while True:
    i = text.find("{", i)
    if i < 0:
        break
    try:
        obj, end = json.JSONDecoder().raw_decode(text[i:])
        last = obj
        i += end
    except Exception:
        i += 1
if last is None:
    sys.exit(1)
json.dump(last, sys.stdout)
' "$raw"
}

orderbook_package_id_from_json() {
    local json="$1"
    echo "$json" | jq -r '
        (
            (.objectChanges // .object_changes // [])[]
            | select((.type // "") == "published")
            | .packageId // .package_id // empty
        ),
        (
            (.changed_objects // .changedObjects // [])[]
            | select((.objectType // .object_type // "") == "package")
            | select((.idOperation // .id_operation // "") | test("CREATED"; "i"))
            | .objectId // .object_id // empty
        )
    ' | head -n1
}

orderbook_extract_created_containing() {
    local digest="$1" needle="$2"
    local json result
    [[ -n "$digest" && -n "$needle" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    result="$(echo "$json" | jq -r --arg t "$needle" '
        def object_type($o):
            ($o.objectType? // $o.object_type? // $o.type? // "") | tostring;
        def object_id($o):
            ($o.objectId? // $o.object_id? // $o.reference?.objectId? // "") | tostring;
        (.changed_objects // .changedObjects // [])[]
        | if type == "array" then empty else . end
        | select(object_type(.) | contains($t))
        | object_id(.)
        | select(length > 0)
    ' | head -n1)"
    [[ -n "$result" ]] || return 1
    normalize_hex_id "$result"
}

orderbook_myso_call_capture() {
    local package="$1" module="$2" func="$3"
    shift 3
    local -a cmd type_args=() call_args=()
    local g arg
    if [[ "${1:-}" == "--type-args" ]]; then
        shift
        while [[ $# -gt 0 && "$1" != "--args" ]]; do
            type_args+=("$1")
            shift
        done
    fi
    if [[ "${1:-}" == "--args" ]]; then
        shift
    fi
    while IFS= read -r -d '' arg; do call_args+=("$arg"); done < <(normalize_client_call_args "$@")
    cmd=(myso client call --package "$package" --module "$module" --function "$func")
    if ((${#type_args[@]})); then
        cmd+=(--type-args)
        cmd+=("${type_args[@]}")
    fi
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    if ((${#call_args[@]})); then
        cmd+=(--args)
        cmd+=("${call_args[@]}")
    fi
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" == 1 ]] || confirm_run; then
        local rc=0 out
        out="$(run_with_timeout "${MYSO_CMD_TIMEOUT_SEC:-180}" "${cmd[@]}" 2>&1)" || rc=$?
        if [[ "$rc" == 124 ]]; then
            echo "Timed out after ${MYSO_CMD_TIMEOUT_SEC:-180}s: ${cmd[*]}" >&2
        fi
        echo "$out" >&2
        printf '%s' "$out"
        return "$rc"
    fi
    return 0
}

orderbook_clear_stale_publication_metadata() {
    local pkg_path="$1" f rel
    [[ "${ORDERBOOK_CLEAR_PUBLICATION:-1}" == 1 ]] || return 0
    for f in \
        "$pkg_path/Published.toml" \
        "$pkg_path/Pub.localnet.toml" \
        "$pkg_path/Move.lock"; do
        [[ -f "$f" ]] || continue
        rel="${f#${REPO_ROOT}/}"
        [[ "$rel" == "$f" ]] && rel="$f"
        log_step "Removing stale publication metadata: $rel"
        rm -f "$f"
    done
    for f in "$pkg_path"/Pub.*.toml; do
        [[ -e "$f" ]] || continue
        rel="${f#${REPO_ROOT}/}"
        [[ "$rel" == "$f" ]] && rel="$f"
        log_step "Removing stale publication metadata: $rel"
        rm -f "$f"
    done
}

orderbook_publish_env_args() {
    local pkg_path="$1" env="${ORDERBOOK_PUBLISH_ENV:-localnet}"
    [[ -n "$env" ]] || return 0
    [[ -f "$pkg_path/Move.toml" ]] || return 0
    printf '%s\n' '-e' "$env"
}

orderbook_move_toml_bak() {
    printf '%s/Move.toml%s' "$1" "$ORDERBOOK_MOVE_TOML_BAK_SUFFIX"
}

orderbook_restore_move_toml() {
    local pkg_path="$1"
    local toml="$pkg_path/Move.toml"
    local bak
    bak="$(orderbook_move_toml_bak "$pkg_path")"
    [[ -f "$bak" ]] || return 0
    mv -f "$bak" "$toml"
}

orderbook_rewrite_move_toml_for_publish() {
    local toml="$1"
    local env_name="${2:-localnet}"
    local env_value="${3:-7fe1c64b}"
    python3 - "$toml" "$env_name" "$env_value" <<'PY'
import re
import sys

path, env_name, env_value = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(path, encoding="utf-8").read()
names = ("MoveStdlib", "MySo")
lines = text.splitlines(keepends=True)
out = []
i = 0
in_deps = False
stripped = False
while i < len(lines):
    raw = lines[i]
    code = raw.split("#", 1)[0]
    if re.match(r"^\s*\[dependencies\]\s*$", code):
        in_deps = True
        out.append(raw)
        i += 1
        continue
    if re.match(r"^\s*\[", code):
        in_deps = False
    if in_deps and re.match(r"^\s*(?:%s)\s*=" % "|".join(names), code):
        stripped = True
        braces = raw.count("{") - raw.count("}")
        i += 1
        while braces > 0 and i < len(lines):
            nxt = lines[i]
            braces += nxt.count("{") - nxt.count("}")
            i += 1
        continue
    out.append(raw)
    i += 1
text = "".join(out)
added_env = False
if not re.search(r"(?m)^\s*\[environments\]", text):
    text = text.rstrip() + f'\n\n[environments]\n{env_name} = "{env_value}"\n'
    added_env = True
elif not re.search(rf"(?m)^\s*{re.escape(env_name)}\s*=", text):
    text, n = re.subn(
        r"(?m)^(\s*\[environments\][^\n]*\n)",
        rf'\1{env_name} = "{env_value}"\n',
        text,
        count=1,
    )
    added_env = n > 0
if text != open(path, encoding="utf-8").read():
    open(path, "w", encoding="utf-8").write(text)
print(f"stripped={int(stripped)} added_env={int(added_env)}")
PY
}

orderbook_prepare_move_toml_for_publish() {
    local pkg_path="$1"
    local toml="$pkg_path/Move.toml"
    local bak env="${ORDERBOOK_PUBLISH_ENV:-localnet}" result rel
    [[ "${ORDERBOOK_MANAGE_MOVE_TOML:-1}" == 1 ]] || return 0
    [[ -f "$toml" ]] || return 0
    bak="$(orderbook_move_toml_bak "$pkg_path")"
    cp -p "$toml" "$bak"
    result="$(orderbook_rewrite_move_toml_for_publish "$toml" "$env" "7fe1c64b")" || {
        orderbook_restore_move_toml "$pkg_path"
        echo "Failed to rewrite Move.toml for publish: $toml" >&2
        return 1
    }
    if cmp -s "$toml" "$bak"; then
        rm -f "$bak"
        return 0
    fi
    rel="${toml#${REPO_ROOT}/}"
    [[ "$rel" == "$toml" ]] && rel="$toml"
    if [[ "$result" == *stripped=1* ]]; then
        log_step "Temporarily clearing explicit MoveStdlib/MySo deps for CLI auto-inject: $rel"
    fi
    if [[ "$result" == *added_env=1* ]]; then
        log_step "Adding [environments] ${env} slot for publish: $rel"
    fi
}

orderbook_with_prepared_move_toml() {
    local pkg_path="$1"
    shift
    local rc=0
    orderbook_prepare_move_toml_for_publish "$pkg_path" || return 1
    "$@" || rc=$?
    orderbook_restore_move_toml "$pkg_path"
    return "$rc"
}

orderbook_package_has_local_framework_deps() {
    local toml="$1/Move.toml"
    [[ -f "$toml" ]] || return 1
    grep -qE 'local[[:space:]]*=[[:space:]]*".*myso-framework' "$toml"
}

orderbook_publish_is_compile_error() {
    echo "$1" | grep -qiE 'unbound module|E03002|E03004'
}

orderbook_publish_is_unpublished_dep_error() {
    echo "$1" | grep -qiE 'unpublished depend|Unpublished dependencies|No modules found'
}

orderbook_publish_token_package() {
    local pkg_path="$1"
    shift
    local -a extra_flags=("$@")
    local -a cmd out json pkg env_arg
    cmd=(myso client publish "$pkg_path" --publish-admin-cap "$PACKAGE_PUBLISH_ADMIN_CAP_ID")
    local g
    while IFS= read -r env_arg; do
        [[ -n "$env_arg" ]] && cmd+=("$env_arg")
    done < <(orderbook_publish_env_args "$pkg_path")
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    if ((${#extra_flags[@]})); then
        local flag
        for flag in "${extra_flags[@]}"; do
            [[ -n "$flag" ]] && cmd+=("$flag")
        done
    fi
    cmd+=(--json)
    echo "---" >&2
    printf ' %q\n' "${cmd[@]}" >&2
    echo "---" >&2
    if [[ "${SKIP_CONFIRM_RUN:-0}" != 1 ]] && ! confirm_run; then
        return 0
    fi
    local rc=0
    out="$(run_with_timeout "${MYSO_CMD_TIMEOUT_SEC:-300}" "${cmd[@]}" 2>&1)" || rc=$?
    echo "$out" >&2
    if [[ "$rc" != 0 ]]; then
        return "$rc"
    fi
    json="$(orderbook_extract_json "$out")" || {
        echo "Publish succeeded but JSON parse failed for $pkg_path" >&2
        return 1
    }
    pkg="$(orderbook_package_id_from_json "$json")"
    pkg="$(normalize_hex_id "$pkg")" || {
        echo "Publish did not return a packageId for $pkg_path" >&2
        return 1
    }
    printf '%s' "$pkg"
}

orderbook_publish_token_with_retry_inner() {
    local pkg_path="$1" pkg err_file err
    orderbook_clear_stale_publication_metadata "$pkg_path"
    err_file="$(mktemp)"
    if pkg="$(orderbook_publish_token_package "$pkg_path" 2>"$err_file")"; then
        cat "$err_file" >&2
        rm -f "$err_file"
        [[ -n "$pkg" ]] && {
            printf '%s' "$pkg"
            return 0
        }
    fi
    err="$(cat "$err_file")"
    rm -f "$err_file"
    [[ -n "$err" ]] && echo "$err" >&2
    if orderbook_publish_is_compile_error "$err"; then
        return 1
    fi
    if ! orderbook_package_has_local_framework_deps "$pkg_path"; then
        return 1
    fi
    if ! orderbook_publish_is_unpublished_dep_error "$err"; then
        return 1
    fi
    log_step "Retrying publish with --with-unpublished-dependencies: $pkg_path"
    orderbook_clear_stale_publication_metadata "$pkg_path"
    orderbook_publish_token_package "$pkg_path" --with-unpublished-dependencies
}

orderbook_publish_token_with_retry() {
    local pkg_path="$1"
    orderbook_with_prepared_move_toml "$pkg_path" \
        orderbook_publish_token_with_retry_inner "$pkg_path"
}

orderbook_init_myusd() {
    local pkg="$1" active="$2" out digest cap
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
        --move-call "${pkg}::myusd::init_coin" \
        "@$(normalize_hex_id "$COIN_CREATION_ADMIN_CAP_ID")")" || return 1
    assert_tx_success "$out" || {
        echo "myusd::init_coin failed" >&2
        return 1
    }
    digest="$(extract_tx_digest "$out")"
    cap="$(orderbook_extract_created_containing "$digest" "TreasuryCap")" || \
        cap="$(extract_created_object_by_type "$digest" "TreasuryCap")" || true
    [[ -n "$cap" ]] || {
        echo "init_coin did not yield TreasuryCap<MYUSD>" >&2
        return 1
    }
    normalize_hex_id "$cap"
}

orderbook_init_bridged_token() {
    local pkg="$1" module="$2" out digest cap
    out="$(orderbook_myso_call_capture "$pkg" "$module" init_coin \
        --args "@$(normalize_hex_id "$COIN_CREATION_ADMIN_CAP_ID")")" || return 1
    assert_tx_success "$out" || {
        echo "${module}::init_coin failed" >&2
        return 1
    }
    digest="$(extract_tx_digest "$out")"
    cap="$(orderbook_extract_created_containing "$digest" "TreasuryCap")" || \
        cap="$(extract_created_object_by_type "$digest" "TreasuryCap")" || true
    [[ -n "$cap" ]] || {
        echo "init_coin did not yield TreasuryCap for $module" >&2
        return 1
    }
    normalize_hex_id "$cap"
}

orderbook_mint_token() {
    local coin_type="$1" treasury="$2" amount="$3" recipient="$4" out
    out="$(orderbook_myso_call_capture 0x2 coin mint_and_transfer \
        --type-args "$coin_type" \
        --args "@$(normalize_hex_id "$treasury")" "$amount" "$(normalize_hex_id "$recipient")")" || return 1
    assert_tx_success "$out" || {
        echo "mint_and_transfer failed for $coin_type" >&2
        return 1
    }
}

orderbook_add_myusd_stablecoin() {
    local out active
    if [[ "${MYUSD_STABLECOIN_REGISTERED:-}" == 1 ]]; then
        log_step "MyUSD already marked stablecoin-registered in session"
        return 0
    fi
    out="$(orderbook_myso_call_capture "$ORDERBOOK_PACKAGE_ID" registry add_stablecoin \
        --type-args "$MYUSD_COIN_TYPE" \
        --args "@$(normalize_hex_id "$ORDERBOOK_REGISTRY_ID")" \
               "@$(normalize_hex_id "$ORDERBOOK_ADMIN_CAP_ID")")" || true
    if ! assert_tx_success "$out"; then
        active="$(resolve_myso_active_address)" || return 1
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
            --move-call "${ORDERBOOK_PACKAGE_ID}::registry::add_stablecoin<${MYUSD_COIN_TYPE}>" \
            "$(ptb_shared_ref "$ORDERBOOK_REGISTRY_ID")" \
            "@$(normalize_hex_id "$ORDERBOOK_ADMIN_CAP_ID")")" || true
    fi
    if assert_tx_success "$out"; then
        MYUSD_STABLECOIN_REGISTERED=1
        return 0
    fi
    if echo "$out" | grep -qiE 'ECoinAlreadyWhitelisted|already|MoveAbort|abort'; then
        log_step "MyUSD already whitelisted as stablecoin"
        MYUSD_STABLECOIN_REGISTERED=1
        return 0
    fi
    echo "add_stablecoin<MYUSD> failed" >&2
    return 1
}

orderbook_extract_pool_id() {
    local digest="$1" pool
    pool="$(orderbook_extract_created_containing "$digest" "pool::Pool")" || pool=''
    [[ -n "$pool" ]] || pool="$(tx_event_field "$digest" "PoolCreated" "pool_id")" || pool=''
    [[ -n "$pool" ]] || return 1
    normalize_hex_id "$pool"
}

orderbook_create_pool() {
    local base_type="$1" tick="$2" lot="$3" min="$4"
    local out digest pool active
    out="$(orderbook_myso_call_capture "$ORDERBOOK_PACKAGE_ID" pool create_pool_admin \
        --type-args "$base_type" "$MYUSD_COIN_TYPE" \
        --args \
            "@$(normalize_hex_id "$ORDERBOOK_REGISTRY_ID")" \
            "$tick" "$lot" "$min" \
            false false \
            "@$(normalize_hex_id "$ORDERBOOK_ADMIN_CAP_ID")")" || true
    if ! assert_tx_success "$out"; then
        active="$(resolve_myso_active_address)" || return 1
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
            --move-call "${ORDERBOOK_PACKAGE_ID}::pool::create_pool_admin<${base_type},${MYUSD_COIN_TYPE}>" \
            "$(ptb_shared_ref "$ORDERBOOK_REGISTRY_ID")" \
            "$tick" "$lot" "$min" \
            false false \
            "@$(normalize_hex_id "$ORDERBOOK_ADMIN_CAP_ID")")" || return 1
    fi
    assert_tx_success "$out" || {
        echo "create_pool_admin failed for $base_type / $MYUSD_COIN_TYPE" >&2
        return 1
    }
    digest="$(extract_tx_digest "$out")"
    pool="$(orderbook_extract_pool_id "$digest")" || {
        echo "Could not extract pool id for $base_type" >&2
        return 1
    }
    printf '%s' "$pool"
}

orderbook_read_pool_tick() {
    local pool_id="$1" base_type="$2" quote_type="$3" out tick
    pool_id="$(normalize_hex_id "$pool_id")" || return 1
    out="$(myso client call \
        --package "$ORDERBOOK_PACKAGE_ID" --module pool --function pool_book_params \
        --type-args "$base_type" "$quote_type" \
        --args "@${pool_id}" \
        --dry-run --json 2>/dev/null)" || return 1
    tick="$(echo "$out" | jq -r '
        (.command_outputs[0].returnValues[0].json
         // .commandResults[0].returnValues[0].json
         // empty)
        | if type == "array" then .[0] else . end
        | if type == "number" or type == "string" then . else empty end
    ' 2>/dev/null)" || tick=''
    if [[ -z "$tick" || "$tick" == "null" ]]; then
        tick="$(echo "$out" | python3 -c '
import json, sys
data = json.load(sys.stdin)
outputs = data.get("command_outputs") or data.get("commandResults") or []
if not outputs:
    sys.exit(1)
rvs = outputs[0].get("returnValues") or []
if not rvs:
    sys.exit(1)
js = rvs[0].get("json")
if isinstance(js, list) and js:
    print(js[0]); raise SystemExit(0)
if isinstance(js, (int, str)) and str(js).isdigit():
    print(js); raise SystemExit(0)
val = rvs[0].get("value") or {}
raw = val.get("value") if isinstance(val, dict) else None
if not raw:
    sys.exit(1)
import base64, struct
b = base64.b64decode(raw)
if len(b) < 8:
    sys.exit(1)
print(struct.unpack_from("<Q", b, 0)[0])
' 2>/dev/null)" || return 1
    fi
    [[ -n "$tick" && "$tick" != "null" ]] || return 1
    printf '%s' "$tick"
}

orderbook_adjust_pool_tick() {
    local pool_id="$1" base_type="$2" quote_type="$3" new_tick="$4" out active
    pool_id="$(normalize_hex_id "$pool_id")" || return 1
    [[ -n "${ORDERBOOK_ADMIN_CAP_ID:-}" && -n "${CLOCK_ID:-}" ]] || return 1
    out="$(orderbook_myso_call_capture "$ORDERBOOK_PACKAGE_ID" pool adjust_tick_size_admin \
        --type-args "$base_type" "$quote_type" \
        --args \
            "@${pool_id}" \
            "$new_tick" \
            "@$(normalize_hex_id "$ORDERBOOK_ADMIN_CAP_ID")" \
            "@$(normalize_hex_id "$CLOCK_ID")")" || true
    if ! assert_tx_success "$out"; then
        active="$(resolve_myso_active_address)" || return 1
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
            --move-call "${ORDERBOOK_PACKAGE_ID}::pool::adjust_tick_size_admin<${base_type},${quote_type}>" \
            "$(ptb_shared_ref "$pool_id")" \
            "$new_tick" \
            "@$(normalize_hex_id "$ORDERBOOK_ADMIN_CAP_ID")" \
            "$(ptb_shared_ref "$CLOCK_ID")")" || true
    fi
    assert_tx_success "$out" || {
        echo "adjust_tick_size_admin failed for $pool_id (wanted tick $new_tick)" >&2
        return 1
    }
}

orderbook_sync_catalog_tick() {
    local pool_id="$1" tick="$2"
    pool_id="$(normalize_hex_id "$pool_id")" || return 1
    command -v psql >/dev/null 2>&1 || {
        echo "psql not found — cannot sync catalog tick_size for $pool_id" >&2
        return 1
    }
    psql "$ORDERBOOK_DB_URL" -v ON_ERROR_STOP=1 -c \
        "UPDATE pools SET tick_size = ${tick} WHERE pool_id = '${pool_id}' OR pool_name = 'MYSO_MYUSD';" \
        >/dev/null
}

orderbook_ensure_mysousd_tick() {
    local pool_id="${MYSO_MYUSD_POOL_ID:-}" on_chain wanted="$TICK_SIZE_MYSO"
    [[ -n "$pool_id" ]] || return 0
    if ! orderbook_pool_is_shared "$pool_id"; then
        log_step "Skipping MYSO tick adjust — pool $pool_id is not shared"
        return 0
    fi
    require_session_fields MYUSD_COIN_TYPE ORDERBOOK_PACKAGE_ID ORDERBOOK_ADMIN_CAP_ID CLOCK_ID || return 1
    on_chain="$(orderbook_read_pool_tick "$pool_id" "$MYSO_COIN_TYPE" "$MYUSD_COIN_TYPE" 2>/dev/null)" || on_chain=''
    if [[ -n "$on_chain" && "$on_chain" == "$wanted" ]]; then
        log_step "MYSO_MYUSD on-chain tick already $wanted"
    else
        if [[ -n "$on_chain" ]]; then
            log_step "Adjusting MYSO_MYUSD tick $on_chain → $wanted"
        else
            log_step "Adjusting MYSO_MYUSD tick → $wanted (on-chain read unavailable)"
        fi
        orderbook_adjust_pool_tick "$pool_id" "$MYSO_COIN_TYPE" "$MYUSD_COIN_TYPE" "$wanted" || return 1
    fi
    orderbook_sync_catalog_tick "$pool_id" "$wanted" || {
        echo "Failed to sync catalog tick_size=$wanted for MYSO_MYUSD" >&2
        return 1
    }
    log_step "Catalog tick_size=$wanted for MYSO_MYUSD"
}

orderbook_cancel_all_orders() {
    local pool_id="$1" bm_id="$2" base_type="$3" quote_type="$4" active out attempt
    pool_id="$(normalize_hex_id "$pool_id")" || return 1
    bm_id="$(normalize_hex_id "$bm_id")" || return 1
    active="$(resolve_myso_active_address)" || return 1
    for attempt in 1 2 3; do
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
            --move-call "${ORDERBOOK_PACKAGE_ID}::balance_manager::generate_proof_as_owner" \
            "@${bm_id}" \
            --assign proof \
            --move-call "${ORDERBOOK_PACKAGE_ID}::pool::cancel_all_orders<${base_type},${quote_type}>" \
            "$(ptb_shared_ref "$pool_id")" \
            "@${bm_id}" \
            proof \
            "$(ptb_shared_ref "$CLOCK_ID")")" || true
        if assert_tx_success "$out"; then
            return 0
        fi
        if [[ "$attempt" -lt 3 ]] \
            && grep -qE 'not available for consumption|already locked' <<<"$out"; then
            sleep 2
            continue
        fi
        break
    done
    echo "cancel_all_orders failed for BM $bm_id on pool $pool_id" >&2
    return 1
}

orderbook_owned_balance_manager_ids() {
    local active vars json
    active="$(resolve_myso_active_address)" || return 1
    active="$(normalize_hex_id "$active")" || return 1
    vars="$(jq -nc --arg addr "$active" '{addr: $addr}')"
    json="$(graphql_post \
        'query OwnedBMs($addr: MySoAddress!) {
          objects(filter: { type: "0xb0c::balance_manager::BalanceManager", ownerKind: ADDRESS, owner: $addr }, first: 50) {
            nodes { address }
          }
        }' "$vars" 2>/dev/null)" || return 1
    echo "$json" | jq -r '.data.objects.nodes[]?.address // empty'
}

orderbook_clear_pool_book_orders() {
    local pool_id="$1" base_type="$2" quote_type="$3" label="$4" bm
    [[ -n "$pool_id" && -n "$base_type" && -n "$quote_type" ]] || return 0
    if ! orderbook_pool_is_shared "$pool_id"; then
        log_step "Skipping ${label} cancel — pool $pool_id is not shared"
        return 0
    fi
    require_session_fields ORDERBOOK_PACKAGE_ID CLOCK_ID || return 1
    log_step "Canceling leftover ${label} orders on owned BalanceManagers"
    while IFS= read -r bm; do
        [[ -n "$bm" ]] || continue
        orderbook_cancel_all_orders "$pool_id" "$bm" "$base_type" "$quote_type" \
            || log_step "No cancelable orders on BM $bm (${label})"
        sleep 1
    done < <(orderbook_owned_balance_manager_ids || true)
}

orderbook_clear_myso_book_orders() {
    orderbook_clear_pool_book_orders \
        "${MYSO_MYUSD_POOL_ID:-}" "$MYSO_COIN_TYPE" "${MYUSD_COIN_TYPE:-}" "MYSO/MYUSD"
}

orderbook_clear_btc_book_orders() {
    orderbook_clear_pool_book_orders \
        "${BTC_MYUSD_POOL_ID:-}" "${BTC_COIN_TYPE:-}" "${MYUSD_COIN_TYPE:-}" "BTC/MYUSD"
}

orderbook_clear_stale_mm_book_orders() {
    [[ "${ORDERBOOK_MYSO_RESEED:-1}" == 1 ]] || return 0
    log_step "Clearing stale MM book orders before live oracle (ORDERBOOK_MYSO_RESEED=1)"
    orderbook_clear_btc_book_orders || true
    orderbook_clear_myso_book_orders || true
    sleep 2
}

orderbook_ensure_demo_trade_funds() {
    local trader_addr myusd_mint
    [[ "${ORDERBOOK_SKIP_DEMO_TRADE:-0}" == 1 ]] && return 0
    trader_addr="${TEST_TRADER_ADDRESS:-${DEPLOYER_ADDRESS:-}}"
    [[ -n "$trader_addr" ]] || {
        echo "TEST_TRADER_ADDRESS or DEPLOYER_ADDRESS required for BTC spot demo" >&2
        return 1
    }
    myusd_mint="${ORDERBOOK_DEMO_MYUSD_MINT:-50000000}"
    log_step "Funding spot demo trader ($trader_addr): gas + ${myusd_mint} MYUSD base units"
    orderbook_fund_address "$trader_addr" 300000000 || return 1
    require_session_fields MYUSD_COIN_TYPE MYUSD_TREASURY_CAP_ID || return 1
    orderbook_mint_token "$MYUSD_COIN_TYPE" "$MYUSD_TREASURY_CAP_ID" "$myusd_mint" "$trader_addr" || return 1
    orderbook_mint_token "$BTC_COIN_TYPE" "$BTC_TREASURY_CAP_ID" 10000000 "$trader_addr" || return 1
}

orderbook_fund_address() {
    local addr="$1" min_mist="${2:-300000000}" attempt total_bal tap
    addr="$(normalize_hex_id "$addr")" || return 1
    total_bal="$(resolve_total_coin_balance "$addr")"
    if [[ -n "$total_bal" && "$total_bal" -ge "$min_mist" ]]; then
        return 0
    fi
    log_step "Funding $addr via faucet (need >= $min_mist MIST; no keystore switch)"
    for tap in $(seq 1 5); do
        total_bal="$(resolve_total_coin_balance "$addr")"
        if [[ -n "$total_bal" && "$total_bal" -ge "$min_mist" ]]; then
            return 0
        fi
        myso client faucet --address "$addr" >/dev/null 2>&1 \
            || myso client faucet --address "$addr" >&2 \
            || true
        [[ "$tap" -lt 5 ]] && sleep 2
    done
    local max_wait="${FAUCET_WAIT_MAX:-20}"
    for ((attempt = 1; attempt <= max_wait; attempt++)); do
        log_wait_progress "faucet balance for $addr" "$attempt" "$max_wait" "need >= $min_mist MIST"
        sleep 1
        total_bal="$(resolve_total_coin_balance "$addr")"
        [[ -n "$total_bal" && "$total_bal" -ge "$min_mist" ]] && return 0
    done
    echo "Wallet $addr not funded after faucet (total=${total_bal:-0}, need=$min_mist)" >&2
    return 1
}

orderbook_admin_curl() {
    local method="$1" path="$2" body="${3:-}"
    local -a cmd
    cmd=(curl -sS -X "$method" "${ORDERBOOK_API_URL}${path}"
        -H "Authorization: Bearer ${ORDERBOOK_ADMIN_TOKEN}"
        -w '\n%{http_code}')
    if [[ -n "$body" ]]; then
        cmd+=(-H 'Content-Type: application/json' -d "$body")
    fi
    "${cmd[@]}"
}

orderbook_admin_health() {
    local code
    code="$(curl -sS -o /dev/null -w '%{http_code}' --connect-timeout 3 --max-time 5 \
        "${ORDERBOOK_API_URL}/admin/health" 2>/dev/null || true)"
    [[ "$code" == "200" ]]
}

orderbook_admin_post() {
    local path="$1" body="$2"
    local resp http_code
    resp="$(orderbook_admin_curl POST "$path" "$body")" || return 1
    http_code="${resp##*$'\n'}"
    resp="${resp%$'\n'*}"
    if [[ "$http_code" != "200" && "$http_code" != "201" ]]; then
        echo "Admin POST ${path} HTTP ${http_code}: $resp" >&2
        return 1
    fi
    printf '%s' "$resp"
}

orderbook_admin_put() {
    local path="$1" body="$2"
    local resp http_code
    resp="$(orderbook_admin_curl PUT "$path" "$body")" || return 1
    http_code="${resp##*$'\n'}"
    resp="${resp%$'\n'*}"
    if [[ "$http_code" != "200" ]]; then
        echo "Admin PUT ${path} HTTP ${http_code}: $resp" >&2
        return 1
    fi
    printf '%s' "$resp"
}

orderbook_asset_registered() {
    local asset_type="$1" json
    json="$(curl -sf --max-time 10 "${ORDERBOOK_API_URL}/assets" 2>/dev/null)" || return 1
    echo "$json" | jq -e --arg t "$asset_type" '
        [.. | objects | select((.asset_type // "") == $t)] | length > 0
    ' >/dev/null 2>&1
}

orderbook_pool_is_shared() {
    local pool_id="$1" json
    [[ -n "$pool_id" ]] || return 1
    json="$(myso client object "$pool_id" --json 2>/dev/null)" || return 1
    echo "$json" | jq -e '.owner.Shared // .data.owner.Shared // .owner?.Shared' >/dev/null 2>&1
}

orderbook_ensure_pool_shared() {
    local name="$1"
    local id_var="$2"
    local pool_id="${!id_var:-}"
    if [[ -n "$pool_id" ]] && object_exists_on_fullnode "$pool_id" && orderbook_pool_is_shared "$pool_id"; then
        return 0
    fi
    if [[ -n "$pool_id" ]]; then
        echo "Clearing stale/non-shared pool ${name} id ${pool_id}" >&2
        printf -v "$id_var" '%s' ''
    fi
    return 1
}

orderbook_pool_registered() {
    local pool_id="$1" json
    json="$(curl -sf --max-time 10 "${ORDERBOOK_API_URL}/get_pools" 2>/dev/null)" || return 1
    echo "$json" | jq -e --arg id "$pool_id" '
        (if type == "array" then . else [] end)
        | map(.pool_id // "")
        | index($id) != null
    ' >/dev/null 2>&1
}

orderbook_catalog_pool_id_for_base() {
    local base_type="$1" json pool_id
    json="$(curl -sf --max-time 10 "${ORDERBOOK_API_URL}/get_pools" 2>/dev/null)" || return 1
    pool_id="$(echo "$json" | jq -r --arg base "$base_type" '
        (if type == "array" then . else [] end)
        | map(select((.base_asset_id // "") == $base))
        | .[0].pool_id // empty
    ')"
    [[ -n "$pool_id" && "$pool_id" != null ]] || return 1
    normalize_hex_id "$pool_id"
}

orderbook_register_asset() {
    local asset_type="$1" name="$2" symbol="$3" decimals="$4" package_id="${5:-}"
    local body
    if orderbook_asset_registered "$asset_type"; then
        log_step "Asset $symbol already in catalog"
        return 0
    fi
    if [[ -n "$package_id" ]]; then
        body="$(jq -nc \
            --arg asset_type "$asset_type" \
            --arg name "$name" \
            --arg symbol "$symbol" \
            --argjson decimals "$decimals" \
            --arg package_id "$package_id" \
            '{
                asset_type: $asset_type,
                name: $name,
                symbol: $symbol,
                decimals: $decimals,
                ucid: null,
                package_id: $package_id,
                package_address_url: null
            }')"
    else
        body="$(jq -nc \
            --arg asset_type "$asset_type" \
            --arg name "$name" \
            --arg symbol "$symbol" \
            --argjson decimals "$decimals" \
            '{
                asset_type: $asset_type,
                name: $name,
                symbol: $symbol,
                decimals: $decimals,
                ucid: null,
                package_id: null,
                package_address_url: null
            }')"
    fi
    if orderbook_admin_post /admin/assets "$body" >/dev/null; then
        log_step "Registered asset $symbol"
        return 0
    fi
    echo "Failed to register asset $symbol ($asset_type)" >&2
    return 1
}

orderbook_register_pool_row() {
    local pool_id="$1" pool_name="$2"
    local base_id="$3" base_decimals="$4" base_symbol="$5" base_name="$6"
    local quote_id="$7" quote_decimals="$8" quote_symbol="$9" quote_name="${10}"
    local min="${11}" lot="${12}" tick="${13}"
    local body
    body="$(jq -nc \
        --arg pool_id "$pool_id" \
        --arg pool_name "$pool_name" \
        --arg base_asset_id "$base_id" \
        --argjson base_asset_decimals "$base_decimals" \
        --arg base_asset_symbol "$base_symbol" \
        --arg base_asset_name "$base_name" \
        --arg quote_asset_id "$quote_id" \
        --argjson quote_asset_decimals "$quote_decimals" \
        --arg quote_asset_symbol "$quote_symbol" \
        --arg quote_asset_name "$quote_name" \
        --argjson min_size "$min" \
        --argjson lot_size "$lot" \
        --argjson tick_size "$tick" \
        '{
            pool_id: $pool_id,
            pool_name: $pool_name,
            base_asset_id: $base_asset_id,
            base_asset_decimals: $base_asset_decimals,
            base_asset_symbol: $base_asset_symbol,
            base_asset_name: $base_asset_name,
            quote_asset_id: $quote_asset_id,
            quote_asset_decimals: $quote_asset_decimals,
            quote_asset_symbol: $quote_asset_symbol,
            quote_asset_name: $quote_asset_name,
            min_size: $min_size,
            lot_size: $lot_size,
            tick_size: $tick_size
        }')"
    if orderbook_pool_registered "$pool_id"; then
        log_step "Pool $pool_name already in catalog; updating sizes"
        if orderbook_admin_put "/admin/pools/${pool_id}" "$(jq -nc \
            --arg pool_name "$pool_name" \
            --argjson min_size "$min" \
            --argjson lot_size "$lot" \
            --argjson tick_size "$tick" \
            '{pool_name: $pool_name, min_size: $min_size, lot_size: $lot_size, tick_size: $tick_size}')" >/dev/null; then
            return 0
        fi
        log_step "Catalog update skipped for $pool_name (pool present in get_pools; admin PUT unavailable)"
        return 0
    fi
    if orderbook_admin_post /admin/pools "$body" >/dev/null; then
        log_step "Registered catalog pool $pool_name"
        return 0
    fi
    log_step "POST /admin/pools failed for $pool_name; trying PUT"
    orderbook_admin_put "/admin/pools/${pool_id}" "$(jq -nc \
        --arg pool_name "$pool_name" \
        --argjson min_size "$min" \
        --argjson lot_size "$lot" \
        --argjson tick_size "$tick" \
        '{pool_name: $pool_name, min_size: $min_size, lot_size: $lot_size, tick_size: $tick_size}')" >/dev/null
}

orderbook_register_assets() {
    orderbook_register_asset "$MYSO_COIN_TYPE" "MySocial" "MYSO" "$MYSO_DECIMALS" "" || return 1
    orderbook_register_asset "$MYUSD_COIN_TYPE" "MyUSD" "MYUSD" "$MYUSD_DECIMALS" "$PKG_MYUSD" || return 1
    orderbook_register_asset "$BTC_COIN_TYPE" "Bitcoin" "BTC" "$BTC_DECIMALS" "$PKG_BTC" || return 1
    orderbook_register_asset "$ETH_COIN_TYPE" "Ethereum" "ETH" "$ETH_DECIMALS" "$PKG_ETH" || return 1
}

orderbook_register_pools() {
    orderbook_register_pool_row \
        "$MYSO_MYUSD_POOL_ID" "MYSO_MYUSD" \
        "$MYSO_COIN_TYPE" "$MYSO_DECIMALS" "MYSO" "MySocial" \
        "$MYUSD_COIN_TYPE" "$MYUSD_DECIMALS" "MYUSD" "MyUSD" \
        "$MIN_SIZE" "$LOT_SIZE" "$TICK_SIZE_MYSO" || return 1
    orderbook_register_pool_row \
        "$BTC_MYUSD_POOL_ID" "BTC_MYUSD" \
        "$BTC_COIN_TYPE" "$BTC_DECIMALS" "BTC" "Bitcoin" \
        "$MYUSD_COIN_TYPE" "$MYUSD_DECIMALS" "MYUSD" "MyUSD" \
        "$MIN_SIZE" "$LOT_SIZE" "$TICK_SIZE_BTC" || return 1
    orderbook_register_pool_row \
        "$ETH_MYUSD_POOL_ID" "ETH_MYUSD" \
        "$ETH_COIN_TYPE" "$ETH_DECIMALS" "ETH" "Ethereum" \
        "$MYUSD_COIN_TYPE" "$MYUSD_DECIMALS" "MYUSD" "MyUSD" \
        "$MIN_SIZE" "$LOT_SIZE" "$TICK_SIZE_ETH" || return 1
}

orderbook_export_active_private_key() {
    local active key_json
    active="$(resolve_myso_active_address)" || return 1
    key_json="$(myso keytool export --key-identity "$active" --json 2>/dev/null)" || {
        echo "Could not export private key for active address $active" >&2
        return 1
    }
    echo "$key_json" | jq -r '.exportedPrivateKey // empty'
}

orderbook_apply_pyth_setup_output() {
    local line key val
    while IFS= read -r line; do
        [[ -n "$line" ]] || continue
        [[ "$line" =~ ^[A-Z_][A-Z0-9_]*= ]] || continue
        key="${line%%=*}"
        val="$(echo "${line#*=}" | jq -r '.')"
        case "$key" in
            PYTH_PACKAGE_ID|MYUSD_PRICE_INFO_OBJECT_ID|MYSO_PRICE_INFO_OBJECT_ID| \
            BTC_PRICE_INFO_OBJECT_ID|ETH_PRICE_INFO_OBJECT_ID|ORACLE_PRIVATE_KEY| \
            ORACLE_ADDRESS|DEPLOYER_ADDRESS|TEST_TRADER_PRIVATE_KEY|TEST_TRADER_ADDRESS)
                printf -v "$key" '%s' "$val"
                log_session_use "$key" "${!key}"
                ;;
        esac
    done
}

orderbook_run_pyth_setup() {
    local out_file rc=0 pyth_dir
    command -v pnpm >/dev/null 2>&1 || {
        echo "pnpm required for orderbook-pyth-setup.ts" >&2
        return 1
    }
    [[ -d "$ORDERBOOK_SANDBOX_DIR/node_modules" ]] || {
        echo "Run: (cd \"$ORDERBOOK_SANDBOX_DIR\" && pnpm install)" >&2
        return 1
    }
    pyth_dir="$ORDERBOOK_SANDBOX_DIR/packages/pyth"
    if [[ -n "${PYTH_PACKAGE_ID:-}" ]] && object_exists_on_fullnode "$PYTH_PACKAGE_ID"; then
        log_step "Reusing Pyth package ${PYTH_PACKAGE_ID}"
    else
        log_step "Publishing Pyth from $pyth_dir"
        PYTH_PACKAGE_ID="$(orderbook_publish_token_with_retry "$pyth_dir")" || return 1
        log_session_use "PYTH_PACKAGE_ID" "$PYTH_PACKAGE_ID"
        orderbook_save_session
    fi

    PRIVATE_KEY="$(orderbook_export_active_private_key)" || return 1
    log_session_use "PRIVATE_KEY" "<exported>"
    out_file="$(mktemp)"
    log_step "PriceInfoObjects + oracle signer (sandbox orderbook-pyth-setup.ts)"
    (
        cd "$ORDERBOOK_SANDBOX_DIR"
        export PRIVATE_KEY PACKAGE_PUBLISH_ADMIN_CAP_ID PYTH_PACKAGE_ID \
            MYUSD_PRICE_INFO_OBJECT_ID MYSO_PRICE_INFO_OBJECT_ID \
            BTC_PRICE_INFO_OBJECT_ID ETH_PRICE_INFO_OBJECT_ID ORACLE_PRIVATE_KEY \
            ORDERBOOK_ORACLE_SEPARATE_WALLET="${ORDERBOOK_ORACLE_SEPARATE_WALLET:-1}" \
            RPC_URL="${RPC_URL:-http://127.0.0.1:9000}"
        pnpm exec tsx scripts/orderbook-pyth-setup.ts
    ) >"$out_file" || rc=$?
    if [[ "$rc" != 0 ]]; then
        cat "$out_file" >&2
        rm -f "$out_file"
        return 1
    fi
    orderbook_apply_pyth_setup_output <"$out_file"
    rm -f "$out_file"
    orderbook_save_session
}


orderbook_verify_live_prices() {
    local bid ask oracle_btc oracle_num
    bid="$(curl -sf --max-time 15 "${ORDERBOOK_API_URL}/summary" 2>/dev/null \
        | jq -r '.[] | select(.trading_pairs == "BTC_MYUSD") | .highest_bid // empty')" || bid=''
    ask="$(curl -sf --max-time 15 "${ORDERBOOK_API_URL}/summary" 2>/dev/null \
        | jq -r '.[] | select(.trading_pairs == "BTC_MYUSD") | .lowest_ask // empty')" || ask=''
    oracle_btc="$(curl -sf --max-time 2 "http://127.0.0.1:${ORACLE_STATUS_PORT:-9010}/" 2>/dev/null \
        | jq -r '.prices.btc // empty')" || oracle_btc=''
    oracle_num="${oracle_btc#\$}"
    if [[ -n "$ask" ]] && awk -v p="$ask" 'BEGIN { exit !(p+0 > 50000) }'; then
        log_step "Live price check OK: oracle=${oracle_btc:-unknown} summary_ask=${ask} bid=${bid:-0}"
        return 0
    fi
    if [[ -n "$bid" ]] && awk -v p="$bid" 'BEGIN { exit !(p+0 > 50000) }'; then
        log_step "Live price check OK: oracle=${oracle_btc:-unknown} summary_bid=${bid} ask=${ask:-0}"
        return 0
    fi
    if [[ -n "$oracle_num" ]] && awk -v p="$oracle_num" 'BEGIN { exit !(p+0 > 50000) }'; then
        log_step "Live price check OK: oracle=${oracle_btc} (catalog bid=${bid:-0} ask=${ask:-0})"
        return 0
    fi
    echo "BTC prices look stale: bid=${bid:-?} ask=${ask:-?} oracle=${oracle_btc:-?} (expected live > 50000)" >&2
    return 1
}

orderbook_assert_ticker_last_price() {
    local pool_name="${1:-BTC_MYUSD}" max="${2:-20}" attempt last_price
    for ((attempt = 1; attempt <= max; attempt++)); do
        last_price="$(curl -sf --max-time 15 "${ORDERBOOK_API_URL}/ticker" 2>/dev/null \
            | jq -r --arg p "$pool_name" '.[$p].last_price // 0')" || last_price='0'
        if awk -v p="$last_price" 'BEGIN { exit !(p+0 > 0) }'; then
            log_step "Ticker ${pool_name} last_price=${last_price}"
            return 0
        fi
        [[ "$attempt" == 1 || $((attempt % 5)) -eq 0 ]] \
            && log_wait_progress "ticker ${pool_name} last_price" "$attempt" "$max"
        sleep 1
    done
    echo "/ticker ${pool_name} last_price=${last_price} (expected > 0 after test trade)" >&2
    return 1
}

orderbook_mm_fallback_mid() {
    local static_default="$1"
    if [[ "${ORDERBOOK_ORACLE_SEED_STATIC:-0}" == 1 ]]; then
        printf '%s' "$static_default"
    else
        printf '0'
    fi
}

orderbook_mm_tuning_for_symbol() {
    local symbol="$1"
    case "$symbol" in
        MYSO)
            printf '%s\n' \
                "${MM_MYSO_MYUSD_ORDER_SIZE_BASE:-2000000000}" \
                "$(orderbook_mm_fallback_mid "${MM_MYSO_MYUSD_FALLBACK_MID:-4700}")" \
                "${MM_MYSO_MYUSD_BASE_DEPOSIT:-20000000000}" \
                "${MM_MYSO_MYUSD_QUOTE_DEPOSIT:-85000000}"
            ;;
        BTC)
            printf '%s\n' \
                "${MM_BTC_MYUSD_ORDER_SIZE_BASE:-1000000}" \
                "$(orderbook_mm_fallback_mid "${MM_BTC_MYUSD_FALLBACK_MID:-9700000000000}")" \
                "${MM_BTC_MYUSD_BASE_DEPOSIT:-500000000}" \
                "${MM_BTC_MYUSD_QUOTE_DEPOSIT:-500000000}"
            ;;
        ETH)
            printf '%s\n' \
                "${MM_ETH_MYUSD_ORDER_SIZE_BASE:-10000000}" \
                "$(orderbook_mm_fallback_mid "${MM_ETH_MYUSD_FALLBACK_MID:-350000000000}")" \
                "${MM_ETH_MYUSD_BASE_DEPOSIT:-5000000000}" \
                "${MM_ETH_MYUSD_QUOTE_DEPOSIT:-500000000}"
            ;;
        *)
            printf '%s\n' "1000" "$(orderbook_mm_fallback_mid "1000000")" "1000000" "1000000"
            ;;
    esac
}

orderbook_fetch_mm_pools_json() {
    local pools_json required
    pools_json="$(curl -sf --max-time 15 "${ORDERBOOK_API_URL}/get_pools")" || {
        echo "GET ${ORDERBOOK_API_URL}/get_pools failed" >&2
        return 1
    }
    if ! echo "$pools_json" | jq -e '
        (if type == "array" then . else [] end) as $p
        | ["BTC_MYUSD", "ETH_MYUSD"] as $need
        | ($need | map(. as $n | ($p | map(.pool_name) | index($n)) != null) | all)
    ' >/dev/null 2>&1; then
        echo "Catalog missing BTC_MYUSD / ETH_MYUSD pools (MYSO_MYUSD optional if broken)" >&2
        return 1
    fi
    require_session_fields MYUSD_PRICE_INFO_OBJECT_ID MYSO_PRICE_INFO_OBJECT_ID \
        BTC_PRICE_INFO_OBJECT_ID ETH_PRICE_INFO_OBJECT_ID || return 1

    MM_POOLS="$(echo "$pools_json" | jq -c --arg myso_pio "$MYSO_PRICE_INFO_OBJECT_ID" \
        --arg myusd_pio "$MYUSD_PRICE_INFO_OBJECT_ID" \
        --arg btc_pio "$BTC_PRICE_INFO_OBJECT_ID" \
        --arg eth_pio "$ETH_PRICE_INFO_OBJECT_ID" \
        --arg seed_static "${ORDERBOOK_ORACLE_SEED_STATIC:-0}" \
        --argjson myso_os "${MM_MYSO_MYUSD_ORDER_SIZE_BASE:-2000000000}" \
        --argjson myso_fb_off "${MM_MYSO_MYUSD_FALLBACK_MID:-4700}" \
        --argjson myso_bd "${MM_MYSO_MYUSD_BASE_DEPOSIT:-20000000000}" \
        --argjson myso_qd "${MM_MYSO_MYUSD_QUOTE_DEPOSIT:-85000000}" \
        --argjson btc_os "${MM_BTC_MYUSD_ORDER_SIZE_BASE:-100000}" \
        --argjson btc_fb_off "${MM_BTC_MYUSD_FALLBACK_MID:-9700000000000}" \
        --argjson btc_bd "${MM_BTC_MYUSD_BASE_DEPOSIT:-500000000}" \
        --argjson btc_qd "${MM_BTC_MYUSD_QUOTE_DEPOSIT:-500000000}" \
        --argjson eth_os "${MM_ETH_MYUSD_ORDER_SIZE_BASE:-10000000}" \
        --argjson eth_fb_off "${MM_ETH_MYUSD_FALLBACK_MID:-350000000000}" \
        --argjson eth_bd "${MM_ETH_MYUSD_BASE_DEPOSIT:-5000000000}" \
        --argjson eth_qd "${MM_ETH_MYUSD_QUOTE_DEPOSIT:-500000000}" \
        '
        def fallback($offline):
            if $seed_static == "1" then ($offline|tostring) else "0" end;
        def base_pio($sym):
            if $sym == "MYSO" then $myso_pio
            elif $sym == "BTC" then $btc_pio
            elif $sym == "ETH" then $eth_pio
            else null end;
        def tuning($sym):
            if $sym == "MYSO" then
                {orderSizeBase: ($myso_os|tostring), fallbackMidPrice: fallback($myso_fb_off),
                 baseDepositAmount: ($myso_bd|tostring), quoteDepositAmount: ($myso_qd|tostring)}
            elif $sym == "BTC" then
                {orderSizeBase: ($btc_os|tostring), fallbackMidPrice: fallback($btc_fb_off),
                 baseDepositAmount: ($btc_bd|tostring), quoteDepositAmount: ($btc_qd|tostring)}
            elif $sym == "ETH" then
                {orderSizeBase: ($eth_os|tostring), fallbackMidPrice: fallback($eth_fb_off),
                 baseDepositAmount: ($eth_bd|tostring), quoteDepositAmount: ($eth_qd|tostring)}
            else
                {orderSizeBase: "1000", fallbackMidPrice: fallback(1000000),
                 baseDepositAmount: "1000000", quoteDepositAmount: "1000000"}
            end;
        (if type == "array" then . else [] end)
        | map(
            . as $row
            | ($row.base_asset_symbol // "") as $sym
            | (base_pio($sym)) as $bpio
            | (tuning($sym)) as $t
            | {
                poolId: $row.pool_id,
                baseCoinType: $row.base_asset_id,
                quoteCoinType: $row.quote_asset_id,
                tickSize: ($row.tick_size | tostring),
                lotSize: ($row.lot_size | tostring),
                minSize: ($row.min_size | tostring),
                baseDecimals: $row.base_asset_decimals,
                quoteDecimals: $row.quote_asset_decimals,
                basePriceInfoObjectId: $bpio,
                quotePriceInfoObjectId: $myusd_pio,
                orderSizeBase: $t.orderSizeBase,
                fallbackMidPrice: $t.fallbackMidPrice,
                baseDepositAmount: $t.baseDepositAmount,
                quoteDepositAmount: $t.quoteDepositAmount
              }
          )
        ')" || return 1

    orderbook_filter_mm_pools_shared || return 1
    log_session_use "MM_POOLS" "<$(echo "$MM_POOLS" | jq 'length') pools>"
    orderbook_save_session
    printf '%s' "$MM_POOLS"
}

orderbook_filter_mm_pools_shared() {
    local row pool_id filtered='[]' count
    while IFS= read -r row; do
        [[ -n "$row" ]] || continue
        pool_id="$(echo "$row" | jq -r '.poolId')"
        if orderbook_pool_is_shared "$pool_id"; then
            filtered="$(echo "$filtered" | jq --argjson r "$row" '. + [$r]')"
        else
            echo "Skipping non-shared pool for MM: ${pool_id}" >&2
        fi
    done < <(echo "$MM_POOLS" | jq -c '.[]')
    count="$(echo "$filtered" | jq 'length')"
    if [[ "$count" -lt 1 ]]; then
        echo "No shared pools available for market maker" >&2
        return 1
    fi
    MM_POOLS="$filtered"
}
