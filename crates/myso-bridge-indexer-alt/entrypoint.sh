#!/bin/sh
set -e

echo "=== MySocial Bridge Indexer Alt ==="
echo "Starting bridge indexer alt..."

if [ -z "$DATABASE_URL" ]; then
  echo "ERROR: DATABASE_URL environment variable is not set"
  echo "Expected: DATABASE_URL=postgres://user:pass@host:5432/bridge"
  exit 1
fi

if [ -z "$REMOTE_STORE_URL" ]; then
  echo "ERROR: REMOTE_STORE_URL environment variable is not set"
  echo "Expected: REMOTE_STORE_URL=https://storage.googleapis.com/mysocial-testnet-checkpoint-blobs"
  exit 1
fi

METRICS_PORT="${PORT:-9184}"

set -- \
  --database-url "$DATABASE_URL" \
  --remote-store-url "$REMOTE_STORE_URL" \
  --metrics-address "0.0.0.0:$METRICS_PORT"

if [ -n "$STREAMING_URL" ]; then
  set -- "$@" --streaming-url "$STREAMING_URL"
  echo "STREAMING_URL (gRPC): $STREAMING_URL"
fi

if [ -n "$FIRST_CHECKPOINT" ]; then
  set -- "$@" --first-checkpoint "$FIRST_CHECKPOINT"
fi

if [ -n "$LAST_CHECKPOINT" ]; then
  set -- "$@" --last-checkpoint "$LAST_CHECKPOINT"
fi

echo "=== Configuration ==="
echo "DATABASE_URL: (set)"
echo "REMOTE_STORE_URL: $REMOTE_STORE_URL"
echo "METRICS_PORT: $METRICS_PORT"
echo "FIRST_CHECKPOINT: ${FIRST_CHECKPOINT:-not set}"
echo "LAST_CHECKPOINT: ${LAST_CHECKPOINT:-not set}"
echo ""
echo "Starting: bridge-indexer-alt $*"
echo ""

exec bridge-indexer-alt "$@"
