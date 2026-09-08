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

echo "orderbook bootstrap helper tests passed"
