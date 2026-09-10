#!/usr/bin/env bash
set -euo pipefail

TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_DIR/../.." && pwd)"
SCRIPT_DIR="$REPO_ROOT/scripts"

normalize_hex_id() {
    local value="${1#0x}"
    printf '0x%064s' "$value" | tr ' ' '0'
}

log_step() { :; }
log_session_use() { :; }

LIVE_PKG="$(normalize_hex_id aabe796d580727144366594ca1ac258a77159d2307971d7f2270187c7e94f9f1)"
STALE_PKG="$(normalize_hex_id c416d904381a76d3053c58a76b972435b43ac9692fde05a11ebd9ea0f14d7a5d)"
LIVE_TYPE="${LIVE_PKG}::myusd::MYUSD"
STALE_TYPE="${STALE_PKG}::myusd::MYUSD"

object_exists_on_fullnode() {
    local id="$1"
    id="$(normalize_hex_id "$id")" || return 1
    [[ "$id" == "$LIVE_PKG" ]]
}

# shellcheck source=../lib/subscription-test-common.sh
source "$SCRIPT_DIR/lib/subscription-test-common.sh"

extracted="$(myusd_package_id_from_coin_type "$STALE_TYPE")"
[[ "$extracted" == "$STALE_PKG" ]] || {
    echo "package extract failed: $extracted" >&2
    exit 1
}

if myusd_coin_type_is_on_chain "$STALE_TYPE"; then
    echo "stale MYUSD package must not count as on-chain" >&2
    exit 1
fi
myusd_coin_type_is_on_chain "$LIVE_TYPE"

ORDERBOOK_TMP="$(mktemp)"
cat >"$ORDERBOOK_TMP" <<EOF
PKG_MYUSD=${LIVE_PKG}
MYUSD_COIN_TYPE=${LIVE_TYPE}
SERVICE_ID=0xshouldnotimport
EOF

MYUSD_COIN_TYPE="$STALE_TYPE"
resolved="$(subscription_resolve_live_myusd_coin_type)"
# resolve will try stale first, fail on-chain, then need orderbook file at default path.
# Call import directly against the temp session:
MYUSD_COIN_TYPE="$STALE_TYPE"
subscription_import_live_myusd_from_orderbook "$ORDERBOOK_TMP"
[[ "$MYUSD_COIN_TYPE" == "$LIVE_TYPE" ]] || {
    echo "orderbook import did not replace stale MYUSD type" >&2
    rm -f "$ORDERBOOK_TMP"
    exit 1
}

# Import must not require sourcing the whole orderbook session.
[[ "${SERVICE_ID:-}" != "0xshouldnotimport" ]] || {
    echo "orderbook import sourced the whole session" >&2
    rm -f "$ORDERBOOK_TMP"
    exit 1
}

rm -f "$ORDERBOOK_TMP"
echo "dripdrop subscription MYUSD helper tests passed"
