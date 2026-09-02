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
TICK_SIZE_MYSO="${TICK_SIZE_MYSO:-10000}"
TICK_SIZE_BTC="${TICK_SIZE_BTC:-100}"
TICK_SIZE_ETH="${TICK_SIZE_ETH:-100}"
LOT_SIZE="${LOT_SIZE:-1000}"
MIN_SIZE="${MIN_SIZE:-1000}"

MYUSD_MINT_AMOUNT="${MYUSD_MINT_AMOUNT:-1000000000}"
BTC_MINT_AMOUNT="${BTC_MINT_AMOUNT:-100000000}"
ETH_MINT_AMOUNT="${ETH_MINT_AMOUNT:-1000000000}"

ORDERBOOK_SESSION_KEYS=(
    ORDERBOOK_PACKAGE_ID CLOCK_ID COIN_TYPE GAS_BUDGET
    ORDERBOOK_REGISTRY_ID ORDERBOOK_ADMIN_CAP_ID
    COIN_CREATION_ADMIN_CAP_ID PACKAGE_PUBLISH_ADMIN_CAP_ID
    PKG_MYUSD PKG_BTC PKG_ETH
    MYUSD_COIN_TYPE BTC_COIN_TYPE ETH_COIN_TYPE
    MYUSD_TREASURY_CAP_ID BTC_TREASURY_CAP_ID ETH_TREASURY_CAP_ID
    MYSO_MYUSD_POOL_ID BTC_MYUSD_POOL_ID ETH_MYUSD_POOL_ID
    MYUSD_STABLECOIN_REGISTERED
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
        MYSO_MYUSD_POOL_ID BTC_MYUSD_POOL_ID ETH_MYUSD_POOL_ID; do
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
        (.objectChanges // .object_changes // [])[]
        | select((.type // "") == "published")
        | .packageId // .package_id // empty
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

orderbook_publish_token_package() {
    local pkg_path="$1"
    local extra_flag="${2:-}"
    local -a cmd out json pkg
    cmd=(myso client publish "$pkg_path" --publish-admin-cap "$PACKAGE_PUBLISH_ADMIN_CAP_ID")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
    [[ -n "$extra_flag" ]] && cmd+=("$extra_flag")
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
        printf '%s' "$out"
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

orderbook_publish_token_with_retry() {
    local pkg_path="$1"
    local pkg
    pkg="$(orderbook_publish_token_package "$pkg_path")" && [[ -n "$pkg" ]] && {
        printf '%s' "$pkg"
        return 0
    }
    log_step "Retrying publish with --with-unpublished-dependencies: $pkg_path"
    orderbook_publish_token_package "$pkg_path" "--with-unpublished-dependencies"
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

orderbook_pool_registered() {
    local pool_id="$1" json
    json="$(curl -sf --max-time 10 "${ORDERBOOK_API_URL}/get_pools" 2>/dev/null)" || return 1
    echo "$json" | jq -e --arg id "$pool_id" '
        (if type == "array" then . else [] end)
        | map(.pool_id // "")
        | index($id) != null
    ' >/dev/null 2>&1
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
        orderbook_admin_put "/admin/pools/${pool_id}" "$(jq -nc \
            --arg pool_name "$pool_name" \
            --argjson min_size "$min" \
            --argjson lot_size "$lot" \
            --argjson tick_size "$tick" \
            '{pool_name: $pool_name, min_size: $min_size, lot_size: $lot_size, tick_size: $tick_size}')" >/dev/null \
            || true
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
