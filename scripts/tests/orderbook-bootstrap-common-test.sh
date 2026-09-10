#!/usr/bin/env bash
set -euo pipefail

TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_DIR/../.." && pwd)"
SCRIPT_DIR="$REPO_ROOT/scripts"

normalize_hex_id() {
    local value="${1#0x}"
    printf '0x%064s' "$value" | tr ' ' '0'
}

tx_event_field() { return 1; }
log_step() { :; }

# shellcheck source=../lib/orderbook-bootstrap-common.sh
source "$SCRIPT_DIR/lib/orderbook-bootstrap-common.sh"

INNER_ID="$(normalize_hex_id 11)"
POOL_ID="$(normalize_hex_id 22)"
MYSO_MYUSD_POOL_ID="$(normalize_hex_id 31)"
BTC_MYUSD_POOL_ID="$POOL_ID"
ETH_MYUSD_POOL_ID="$(normalize_hex_id 33)"

myso() {
    if [[ "${1:-}" == client && "${2:-}" == tx-block ]]; then
        jq -nc --arg inner "$INNER_ID" --arg pool "$POOL_ID" '{
            changed_objects: [
                {
                    objectId: $inner,
                    objectType: "0x2::dynamic_field::Field<u64,0xb0c::pool::PoolInner<0x1::btc::BTC,0x2::myusd::MYUSD>>"
                },
                {
                    objectId: $pool,
                    objectType: "0xb0c::pool::Pool<0x1::btc::BTC,0x2::myusd::MYUSD>"
                }
            ]
        }'
        return 0
    fi
    if [[ "${1:-}" == client && "${2:-}" == object ]]; then
        case "${3:-}" in
            "$INNER_ID")
                jq -nc '{
                    owner: {ObjectOwner: "0xparent"},
                    previous_transaction: "create-digest",
                    data: {Move: {type_: {Other: {module: "dynamic_field", name: "Field"}}}}
                }'
                ;;
            "$POOL_ID"|"$MYSO_MYUSD_POOL_ID"|"$ETH_MYUSD_POOL_ID")
                jq -nc '{
                    owner: {Shared: {initial_shared_version: 1}},
                    data: {Move: {type_: {Other: {module: "pool", name: "Pool"}}}}
                }'
                ;;
            *) return 1 ;;
        esac
        return 0
    fi
    return 1
}

assert_eq() {
    local expected="$1" actual="$2" message="$3"
    [[ "$actual" == "$expected" ]] || {
        printf '%s\nexpected: %s\nactual:   %s\n' "$message" "$expected" "$actual" >&2
        return 1
    }
}

extracted="$(orderbook_extract_pool_id create-digest)"
assert_eq "$POOL_ID" "$extracted" "pool extraction selected PoolInner instead of Pool"

if orderbook_pool_is_shared "$INNER_ID"; then
    echo "PoolInner must not pass shared Pool validation" >&2
    exit 1
fi
orderbook_pool_is_shared "$POOL_ID"

recovered="$(orderbook_resolve_shared_pool_id "$INNER_ID")"
assert_eq "$POOL_ID" "$recovered" "failed to recover outer Pool from PoolInner transaction"

# Catalog cleanup must be safe to repeat. The public catalog read can lag the
# writer, so DELETE may correctly report 404 after GET still exposed the row.
ADMIN_DELETE_STATUS=404
orderbook_admin_curl() {
    printf '{"status":"missing"}\n%s' "$ADMIN_DELETE_STATUS"
}
orderbook_admin_delete "/admin/pools/${INNER_ID}" >/dev/null

ADMIN_DELETE_STATUS=500
if orderbook_admin_delete "/admin/pools/${INNER_ID}" >/dev/null 2>&1; then
    echo "admin delete must still fail for non-idempotent server errors" >&2
    exit 1
fi

MM_POOLS="$(jq -nc \
    --arg myso "$MYSO_MYUSD_POOL_ID" \
    --arg btc "$BTC_MYUSD_POOL_ID" \
    --arg eth "$ETH_MYUSD_POOL_ID" \
    '[{poolId: $myso}, {poolId: $btc}, {poolId: $eth}]')"
orderbook_filter_mm_pools_shared
assert_eq 3 "$(echo "$MM_POOLS" | jq -r length)" "all required pools should remain configured"

MM_POOLS="$(jq -nc \
    --arg myso "$MYSO_MYUSD_POOL_ID" \
    --arg eth "$ETH_MYUSD_POOL_ID" \
    '[{poolId: $myso}, {poolId: $eth}]')"
if orderbook_filter_mm_pools_shared 2>/dev/null; then
    echo "market-maker validation must fail when BTC_MYUSD is missing" >&2
    exit 1
fi

assert_eq "MYSO_MYUSD" "$(orderbook_normalize_mm_pool_filter MYSO)" "MYSO alias should normalize"
assert_eq "BTC_MYUSD" "$(orderbook_normalize_mm_pool_filter btc_myusd)" "btc_myusd should normalize"
assert_eq "" "$(orderbook_normalize_mm_pool_filter '')" "empty filter stays empty"
if orderbook_normalize_mm_pool_filter NOPE >/dev/null 2>&1; then
    echo "unknown pool filter must fail" >&2
    exit 1
fi

MM_POOL_FILTER=MYSO_MYUSD
MM_POOLS="$(jq -nc \
    --arg myso "$MYSO_MYUSD_POOL_ID" \
    --arg btc "$BTC_MYUSD_POOL_ID" \
    --arg eth "$ETH_MYUSD_POOL_ID" \
    '[{poolId: $myso}, {poolId: $btc}, {poolId: $eth}]')"
orderbook_filter_mm_pools_shared
assert_eq 1 "$(echo "$MM_POOLS" | jq -r length)" "MYSO filter should keep one pool"
assert_eq "$MYSO_MYUSD_POOL_ID" "$(echo "$MM_POOLS" | jq -r '.[0].poolId')" \
    "MYSO filter should keep the MYSO pool id"

MM_POOL_FILTER=MYSO_MYUSD
MM_POOLS="$(jq -nc --arg myso "$MYSO_MYUSD_POOL_ID" '[{poolId: $myso}]')"
orderbook_filter_mm_pools_shared
assert_eq 1 "$(echo "$MM_POOLS" | jq -r length)" \
    "MYSO-only MM_POOLS should pass when filter is MYSO_MYUSD"

MM_POOL_FILTER=

if orderbook_should_adjust_tick 1 1 1; then
    echo "should not adjust when on-chain tick already matches" >&2
    exit 1
fi
if orderbook_should_adjust_tick '' 1 1; then
    echo "should not adjust when catalog tick already matches and on-chain is unknown" >&2
    exit 1
fi
if ! orderbook_should_adjust_tick 1000 1 ''; then
    echo "should adjust when on-chain tick differs" >&2
    exit 1
fi
if ! orderbook_should_adjust_tick '' 1 ''; then
    echo "should adjust when neither on-chain nor catalog tick is known" >&2
    exit 1
fi

MM_POOLS="$(jq -nc --arg myso "$MYSO_MYUSD_POOL_ID" '[{poolId: $myso}]')"
if orderbook_mm_pools_include BTC_MYUSD; then
    echo "MYSO-only MM_POOLS must not include BTC_MYUSD" >&2
    exit 1
fi
if ! orderbook_mm_pools_include MYSO_MYUSD; then
    echo "MYSO-only MM_POOLS must include MYSO_MYUSD" >&2
    exit 1
fi

myso_only_status='{"updates":1,"prices":{"myso":"$0.0047","btc":null,"eth":null}}'
if ! orderbook_oracle_status_is_live_for_mm "$myso_only_status"; then
    echo "MYSO-only live check should pass without a BTC tick" >&2
    exit 1
fi
if orderbook_oracle_status_is_live_for_mm '{"updates":0,"prices":{"myso":"$0.0047"}}'; then
    echo "live check must require updates>=1" >&2
    exit 1
fi
MM_POOLS="$(jq -nc --arg btc "$BTC_MYUSD_POOL_ID" '[{poolId: $btc}]')"
if orderbook_oracle_status_is_live_for_mm "$myso_only_status"; then
    echo "BTC MM run must not pass on a MYSO-only status" >&2
    exit 1
fi
if ! orderbook_oracle_status_is_live_for_mm '{"updates":1,"prices":{"btc":"$97000.00"}}'; then
    echo "BTC live check should pass when BTC > 50000" >&2
    exit 1
fi

assert_eq 2100000000 "$(orderbook_mm_myusd_have 104486616 1995513384)" \
    "have must include BalanceManager MYUSD"
if orderbook_should_mint_mm_myusd 2100000000 2100000000; then
    echo "should not mint when wallet+BM already covers required" >&2
    exit 1
fi
if ! orderbook_should_mint_mm_myusd 104486616 2100000000; then
    echo "should mint when only wallet is counted and it is short" >&2
    exit 1
fi
assert_eq 42 "$(orderbook_parse_u64_return '{"commandResults":[{"returnValues":[{"json":42}]}]}')" \
    "dry-run u64 return should parse"

assert_eq true "$(orderbook_pool_whitelist_flag MYSO_MYUSD)" \
    "MYSO/MYUSD must be created whitelisted"
assert_eq false "$(orderbook_pool_whitelist_flag BTC_MYUSD)" \
    "BTC/MYUSD must not be whitelisted"
assert_eq false "$(orderbook_pool_whitelist_flag ETH_MYUSD)" \
    "ETH/MYUSD must not be whitelisted"

empty_mm='{"pools":[]}'
empty_catalog='[]'
if ! orderbook_should_skip_myso_fee_prices "$empty_mm" "$empty_catalog"; then
    echo "empty MYSO book must skip fee price points" >&2
    exit 1
fi

two_sided_mm="$(jq -nc '{
    pools: [{
        pair: "MYSO/MYUSD",
        orders: [
            {isBid: true, price: 0.0046, quantity: 1},
            {isBid: false, price: 0.0048, quantity: 1}
        ]
    }]
}')"
if orderbook_should_skip_myso_fee_prices "$two_sided_mm" "$empty_catalog"; then
    echo "two-sided MM MYSO book must not skip fee price points" >&2
    exit 1
fi

one_sided_mm="$(jq -nc '{
    pools: [{
        pair: "MYSO/MYUSD",
        orders: [{isBid: true, price: 0.0046, quantity: 1}]
    }]
}')"
if ! orderbook_should_skip_myso_fee_prices "$one_sided_mm" "$empty_catalog"; then
    echo "one-sided MYSO book must skip fee price points" >&2
    exit 1
fi

two_sided_catalog="$(jq -nc '[{trading_pairs:"MYSO_MYUSD",highest_bid:"0.0046",lowest_ask:"0.0048"}]')"
if orderbook_should_skip_myso_fee_prices "$empty_mm" "$two_sided_catalog"; then
    echo "catalog two-sided MYSO book must not skip fee price points" >&2
    exit 1
fi

echo "orderbook bootstrap helper tests passed"
