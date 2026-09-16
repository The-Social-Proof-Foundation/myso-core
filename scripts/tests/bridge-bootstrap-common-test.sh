#!/usr/bin/env bash
set -euo pipefail

TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_DIR/../.." && pwd)"
SCRIPT_DIR="$REPO_ROOT/scripts"
SOCIAL_SESSION_SAVE_PATH="/tmp/myso-bridge-bootstrap-test-session.env"

# shellcheck source=../lib/social-runtime-common.sh
source "$SCRIPT_DIR/lib/social-runtime-common.sh"
# shellcheck source=../lib/bridge-bootstrap-common.sh
source "$SCRIPT_DIR/lib/bridge-bootstrap-common.sh"

assert_eq() {
    local expected="$1" actual="$2" message="$3"
    [[ "$actual" == "$expected" ]] || {
        printf '%s\nexpected: %s\nactual:   %s\n' "$message" "$expected" "$actual" >&2
        return 1
    }
}

want='0xa4523ab352b50d73a156ba0f6754de5e7c8ef2de0355314fbfa9689d112659c5::btc::BTC'
got="$(bridge_normalize_type_name '0xa4523ab352b50d73a156ba0f6754de5e7c8ef2de0355314fbfa9689d112659c5::btc::BTC')"
assert_eq "$want" "$got" "already-padded type stays padded"

got="$(bridge_normalize_type_name 'a4523ab352b50d73a156ba0f6754de5e7c8ef2de0355314fbfa9689d112659c5::btc::BTC')"
assert_eq "$want" "$got" "missing 0x is padded"

MYUSD_TYPE='0x3f964cdf3eaf8f9322c51f319ec9869021a5cdcf4c95d548675c0046de3e48d7::myusd::MYUSD'
bridge_rpc_json() {
    cat <<JSON
{"jsonrpc":"2.0","id":1,"result":{"treasury":{"supportedTokens":[
  ["0xa4523ab352b50d73a156ba0f6754de5e7c8ef2de0355314fbfa9689d112659c5::btc::BTC",{"id":1}],
  ["0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH",{"id":2}],
  ["${MYUSD_TYPE}",{"id":3}]
],"idTokenTypeMap":[
  [1,"0xa4523ab352b50d73a156ba0f6754de5e7c8ef2de0355314fbfa9689d112659c5::btc::BTC"],
  [2,"0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH"],
  [3,"${MYUSD_TYPE}"],
  [4,"${MYUSD_TYPE}"]
],"railReserve":[[3,1000],[4,0]]}}}
JSON
}

BRIDGE_BTC_TYPE='0xa4523ab352b50d73a156ba0f6754de5e7c8ef2de0355314fbfa9689d112659c5::btc::BTC'
BRIDGE_ETH_TYPE='0x0e8ba9f351924f9f053dbecb740f6aebed0f4dd38b42914636212a038886f450::eth::ETH'
BRIDGE_MYUSD_TYPE="$MYUSD_TYPE"
BRIDGE_USDC_TYPE="$MYUSD_TYPE"
BRIDGE_USDT_TYPE="$MYUSD_TYPE"

bridge_foreign_tokens_supported_on_chain

BRIDGE_MYUSD_TYPE='0xdead::myusd::MYUSD'
BRIDGE_USDC_TYPE="$BRIDGE_MYUSD_TYPE"
if bridge_foreign_tokens_supported_on_chain; then
    echo "expected missing MYUSD type to fail on-chain check" >&2
    exit 1
fi
BRIDGE_MYUSD_TYPE="$MYUSD_TYPE"
BRIDGE_USDC_TYPE="$MYUSD_TYPE"
BRIDGE_USDT_TYPE="$MYUSD_TYPE"

yaml_usdc="$(bridge_yaml_token_struct "$BRIDGE_USDC_TYPE")"
yaml_usdt="$(bridge_yaml_token_struct "$BRIDGE_USDT_TYPE")"
assert_eq "$yaml_usdc" "$yaml_usdt" "approved-action yaml lists the same MYUSD type for ids 3 and 4"
printf '%s\n' "$yaml_usdc" | grep -q 'myusd' || {
    echo "approved-action yaml must use published myusd::MYUSD for rails 3/4" >&2
    exit 1
}

assert_eq 'vector["a::btc::BTC", "b::eth::ETH"]' \
    "$(literal_move_vector_from_csv 'a::btc::BTC,b::eth::ETH')" \
    "csv type names become a Move vector"

yaml="$(bridge_yaml_token_struct '0x33ba7869553ea2cb3e7110cdd203c2b663a2969e503e0030fb44d647e1dd8dce::btc::BTC')"
printf '%s\n' "$yaml" | grep -q '0x33ba7869553ea2cb3e7110cdd203c2b663a2969e503e0030fb44d647e1dd8dce' || {
    echo "approved-action yaml must keep the published BTC package address" >&2
    exit 1
}

assert_eq '3=1000 4=0' "$(bridge_rail_reserve_from_rpc)" \
    "RPC summary railReserve is parsed as id=amount pairs"

deposit_tokens="$(bridge_yaml_deposit_supported_tokens \
    '0x0000000000000000000000000000000000000000' \
    '0x5fc748f1FEb28d7b76fa1c6B07D8ba2d5535177c' \
    '0x38a024C0b412B9d1db8BC398140D00F5Af3093D4' \
    '0xB82008565FdC7e44609fA118A4a681E92581e680' \
    '0x2a810409872AfC346F9B5b26571Fd6eC42EA4849')"
printf '%s\n' "$deposit_tokens" | grep -q '0x5fc748f1FEb28d7b76fa1c6B07D8ba2d5535177c' || {
    echo "deposit supported-tokens must include BRIDGE_BTC" >&2
    exit 1
}
printf '%s\n' "$deposit_tokens" | grep -q '0x38a024C0b412B9d1db8BC398140D00F5Af3093D4' || {
    echo "deposit supported-tokens must include BRIDGE_WETH" >&2
    exit 1
}
printf '%s\n' "$deposit_tokens" | grep -q '0xB82008565FdC7e44609fA118A4a681E92581e680' || {
    echo "deposit supported-tokens must include BRIDGE_USDC" >&2
    exit 1
}
printf '%s\n' "$deposit_tokens" | grep -q '0x2a810409872AfC346F9B5b26571Fd6eC42EA4849' || {
    echo "deposit supported-tokens must include BRIDGE_USDT" >&2
    exit 1
}
if printf '%s\n' "$deposit_tokens" | grep -qi '0x0000000000000000000000000000000000000000'; then
    echo "deposit supported-tokens must omit native 0x0" >&2
    exit 1
fi

relayer_yaml="$(bridge_yaml_deposit_relayer_eth_key "$BRIDGE_ANVIL_TEST_PK")"
printf '%s\n' "$relayer_yaml" | grep -q 'relayer-eth-private-key:' || {
    echo "deposit yaml must include relayer-eth-private-key" >&2
    exit 1
}
printf '%s\n' "$relayer_yaml" | grep -q "$BRIDGE_ANVIL_TEST_PK" || {
    echo "deposit relayer-eth-private-key must use BRIDGE_EVM_PRIVATE_KEY / Anvil #0" >&2
    exit 1
}

echo "bridge-bootstrap-common-test ok"
