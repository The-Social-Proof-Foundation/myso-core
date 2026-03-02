#!/bin/sh
set -e

echo "=== MySocial Orderbook Indexer ==="

# Ensure RUST_LOG is set for Railway log visibility (variables may not always apply)
export RUST_LOG="${RUST_LOG:-info,myso_indexer_alt_framework=warn,myso_indexer_alt_orderbook=info}"

METRICS_PORT="${PORT:-9184}"
ENV="${ENV:-testnet}"

set -- --env "$ENV" --metrics-address "0.0.0.0:$METRICS_PORT" --database-url "$DATABASE_URL"

if [ -n "$STREAMING_URL" ]; then
  set -- "$@" --streaming-url "$STREAMING_URL"
  echo "Using gRPC streaming: $STREAMING_URL"
else
  echo "STREAMING_URL not set, using HTTP checkpoint store"
fi

if [ -n "$REMOTE_STORE_URL" ]; then
  set -- "$@" --remote-store-url "$REMOTE_STORE_URL"
  echo "Using HTTP remote store: $REMOTE_STORE_URL"
fi

if [ -n "$FIRST_CHECKPOINT" ]; then
  set -- "$@" --first-checkpoint "$FIRST_CHECKPOINT"
fi

echo "Starting: myso-orderbook-indexer $*"
echo ""

exec myso-orderbook-indexer "$@"
