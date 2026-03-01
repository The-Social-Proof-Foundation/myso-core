#!/bin/sh
set -e

echo "=== MySocial Orderbook Server ==="

export RUST_LOG="${RUST_LOG:-info}"

API_PORT="${PORT:-9008}"
RPC_URL="${RPC_URL:-$RPC_API_URL}"

if [ -z "$DATABASE_URL" ]; then
  echo "ERROR: DATABASE_URL is not set"
  exit 1
fi

set -- \
  --server-port "$API_PORT" \
  --metrics-address "0.0.0.0:9184" \
  --database-url "$DATABASE_URL" \
  --rpc-url "${RPC_URL:-http://fullnode.testnet.mysocial.network:9000}"

[ -n "$ORDERBOOK_PACKAGE_ID" ] && set -- "$@" --orderbook-package-id "$ORDERBOOK_PACKAGE_ID"
[ -n "$MYSO_TOKEN_PACKAGE_ID" ] && set -- "$@" --myso-token-package-id "$MYSO_TOKEN_PACKAGE_ID"
[ -n "$MYSO_TREASURY_ID" ] && set -- "$@" --myso-treasury-id "$MYSO_TREASURY_ID"
[ -n "$MARGIN_PACKAGE_ID" ] && set -- "$@" --margin-package-id "$MARGIN_PACKAGE_ID"
[ -n "$ADMIN_TOKENS" ] && set -- "$@" --admin-tokens "$ADMIN_TOKENS"

echo "Starting: myso-orderbook-server $*"
exec myso-orderbook-server "$@"
