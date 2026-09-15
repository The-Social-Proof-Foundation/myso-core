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
orderbook_filter_mm_pools_shared
assert_eq 2 "$(echo "$MM_POOLS" | jq -r length)" \
    "default MM may omit BTC when it is not required"

assert_eq "MYSO_MYUSD_POOL_ID" "$(orderbook_mm_required_pool_id_vars)" \
    "default MM required pool is MYSO only"
assert_eq "0xabcdef" "$(orderbook_coin_pkg '0xabcdef::btc::BTC')" \
    "coin pkg is the address prefix"

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

DEPLOYER_ADDRESS=''
PRIVATE_KEY=''
resolve_myso_active_address() {
    printf '%s' '0xabc'
}
orderbook_export_active_private_key() {
    printf '%s' 'mysoprivkey1test'
}
orderbook_save_session() { :; }
log_session_use() { :; }
orderbook_ensure_deployer_identity
assert_eq "$(normalize_hex_id abc)" "$DEPLOYER_ADDRESS" \
    "ensure should set DEPLOYER_ADDRESS from active-address"
assert_eq "mysoprivkey1test" "$PRIVATE_KEY" \
    "ensure should export PRIVATE_KEY when session is empty"

timeout_dump='ChangedObject { object_id: Some("0x02c0b2923372c636a53b503df5a554d1855da0881e241e16935907a13e3445ad"), object_type: Some("0x2::package::UpgradeCap") } ChangedObject { object_id: Some("0x131643887b867ca4b970ee7029432769f8eb96db5f245b51b8be1e2d3f7982ad"), output_state: Some(PackageWrite), object_type: Some("package") }'
extracted="$(orderbook_package_id_from_publish_output "$timeout_dump")"
assert_eq "$(normalize_hex_id 131643887b867ca4b970ee7029432769f8eb96db5f245b51b8be1e2d3f7982ad)" \
    "$extracted" "checkpoint-timeout dump should yield the published package id"

object_exists_on_fullnode() { return 0; }

parsed="$(orderbook_parse_pool_type_args \
    '0xb0c::pool::Pool<0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH, 0xcafba0::myusd::MYUSD>')"
assert_eq "0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH	0xcafba0::myusd::MYUSD" \
    "$parsed" "pool type args should split base and quote"

if ! orderbook_coin_types_equal \
    '0x2::myso::MYSO' \
    '0x0000000000000000000000000000000000000000000000000000000000000002::myso::MYSO'; then
    echo "padded and unpadded MYSO types must compare equal" >&2
    exit 1
fi

ETH_COIN_TYPE=''
ETH_MYUSD_POOL_ID="$(normalize_hex_id 99)"
orderbook_apply_bridge_coin_type ETH \
    '0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH'
assert_eq '' "$ETH_MYUSD_POOL_ID" \
    "empty old ETH type should clear leftover pool id"

ETH_COIN_TYPE='0x296ab26f70f617a11157b71de7d2930300b116a779571b6360bb79ded3fb9365::eth::ETH'
ETH_MYUSD_POOL_ID="$(normalize_hex_id 99)"
orderbook_apply_bridge_coin_type ETH \
    '0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH'
assert_eq '' "$ETH_MYUSD_POOL_ID" \
    "ETH type change should clear leftover pool id"

bridge_eth='0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH'
stale_catalog="$(jq -nc --arg old '0x296ab26f70f617a11157b71de7d2930300b116a779571b6360bb79ded3fb9365::eth::ETH' \
    '[{pool_name:"ETH_MYUSD",pool_id:"0x11",base_asset_id:$old}]')"
stale_id="$(orderbook_catalog_stale_pool_id_from_json "$stale_catalog" ETH_MYUSD "$bridge_eth")"
assert_eq "$(normalize_hex_id 11)" "$stale_id" \
    "catalog ETH_MYUSD with a different base_asset_id is stale"

fresh_catalog="$(jq -nc --arg t "$bridge_eth" \
    '[{pool_name:"ETH_MYUSD",pool_id:"0x22",base_asset_id:$t}]')"
if orderbook_catalog_stale_pool_id_from_json "$fresh_catalog" ETH_MYUSD "$bridge_eth" >/dev/null; then
    echo "matching catalog ETH type must not be stale" >&2
    exit 1
fi

found="$(orderbook_catalog_pool_id_from_json "$fresh_catalog" \
    '0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH')"
assert_eq "$(normalize_hex_id 22)" "$found" \
    "catalog lookup should match hex-normalized ETH type"

assert_eq "MYSO_MYUSD" "$(orderbook_mm_required_catalog_pool_names)" \
    "default required catalog pool is MYSO only"
if ! orderbook_catalog_has_required_mm_pools '[{"pool_name":"MYSO_MYUSD"}]'; then
    echo "MYSO-only catalog should satisfy default MM required pools" >&2
    exit 1
fi
if orderbook_catalog_has_required_mm_pools '[{"pool_name":"ETH_MYUSD"}]'; then
    echo "ETH-only catalog must not satisfy default MYSO required pool" >&2
    exit 1
fi

sandbox_dir="$(mktemp -d)"
printf '%s\n' 'PYTH_API_KEY=keepme' 'ETH_COIN_TYPE=old-eth' > "$sandbox_dir/.env"
ORDERBOOK_SANDBOX_DIR="$sandbox_dir"
ETH_COIN_TYPE="$bridge_eth"
BTC_COIN_TYPE='0xa4523ab352b50d73a156ba0f6754de5e7c8ef2de0355314fbfa9689d112659c5::btc::BTC'
MYUSD_COIN_TYPE='0xcafba0d41504b7850a39768c32f6112765208bb82e6a368930c5815d80376205::myusd::MYUSD'
MM_POOLS='[{"poolId":"0x1"}]'
orderbook_sync_sandbox_env
grep -q '^PYTH_API_KEY=keepme$' "$sandbox_dir/.env" || {
    echo "sandbox env upsert must leave PYTH_API_KEY alone" >&2
    exit 1
}
grep -q "$bridge_eth" "$sandbox_dir/.env" || {
    echo "sandbox env upsert must write bridge ETH type" >&2
    exit 1
}
rm -rf "$sandbox_dir"

shared_myusd='0x3f964cdf3eaf8f9322c51f319ec9869021a5cdcf4c95d548675c0046de3e48d7::myusd::MYUSD'
BRIDGE_MYUSD_TYPE=''
BRIDGE_USDC_TYPE="$shared_myusd"
BRIDGE_USDT_TYPE="$shared_myusd"
got="$(orderbook_bridge_myusd_type)"
assert_eq "$shared_myusd" "$got" "orderbook reuses published MYUSD from aliased USDC/USDT rail types"
assert_eq "$BRIDGE_USDC_TYPE" "$BRIDGE_USDT_TYPE" "ids 3 and 4 must share one MYUSD type"

echo "orderbook bootstrap helper tests passed"
