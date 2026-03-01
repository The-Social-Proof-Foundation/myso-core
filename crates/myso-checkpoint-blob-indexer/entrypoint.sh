#!/bin/sh
set -e

echo "=== MySocial Checkpoint Blob Indexer ==="
echo "Starting checkpoint blob indexer..."

# Support both RPC_API_URL and REST_URL for backward compatibility
RPC_URL="${RPC_API_URL:-$REST_URL}"
if [ -z "$RPC_URL" ]; then
  echo "ERROR: RPC_API_URL or REST_URL environment variable is not set"
  echo "Expected: RPC_API_URL=http://fullnode.testnet.mysocial.network:9000"
  exit 1
fi

if [ -z "$REMOTE_STORE_BUCKET" ]; then
  echo "ERROR: REMOTE_STORE_BUCKET environment variable is not set"
  echo "Expected: REMOTE_STORE_BUCKET=mysocial-testnet-archive"
  exit 1
fi

# Create GCS service account key from environment variables
if [ -n "$GCS_SERVICE_ACCOUNT_JSON" ]; then
  echo "Creating GCS service account key file..."
  printf "%s" "$GCS_SERVICE_ACCOUNT_JSON" > /app/credentials/gcs-key.json
  export GOOGLE_APPLICATION_CREDENTIALS="/app/credentials/gcs-key.json"
  echo "✓ GCS authentication configured"

  if [ -f "/app/credentials/gcs-key.json" ]; then
    if jq empty /app/credentials/gcs-key.json 2>/dev/null; then
      if jq -e ".type and .project_id and .private_key and .client_email" /app/credentials/gcs-key.json >/dev/null 2>&1; then
        echo "✓ GCS key file valid"
      else
        echo "✗ GCS key file missing required fields"
        exit 1
      fi
    else
      echo "✗ GCS key file is not valid JSON"
      exit 1
    fi
  else
    echo "✗ GCS key file not found"
    exit 1
  fi
else
  echo "WARNING: GCS_SERVICE_ACCOUNT_JSON not set - uploads may fail"
fi

# Use PORT from Railway (or default 9184) so health checks reach the metrics server
METRICS_PORT="${PORT:-9184}"
REMOTE_STORE_URL="${REMOTE_STORE_URL:-https://storage.googleapis.com/mysocial-testnet-checkpoints}"

# Build CLI args - when streaming is set, ingestion uses gRPC fallback (no remote-store-url needed)
set -- --gcs "$REMOTE_STORE_BUCKET" \
  --metrics-address "0.0.0.0:$METRICS_PORT"

if [ -n "$STREAMING_URL" ]; then
  set -- "$@" --streaming-url "$STREAMING_URL" --rpc-api-url "$STREAMING_URL"
else
  set -- "$@" --remote-store-url "$REMOTE_STORE_URL"
fi

if [ -n "$FIRST_CHECKPOINT" ]; then
  set -- "$@" --first-checkpoint "$FIRST_CHECKPOINT"
fi

if [ -n "$LAST_CHECKPOINT" ]; then
  set -- "$@" --last-checkpoint "$LAST_CHECKPOINT"
fi

echo "=== Configuration ==="
echo "RPC_URL: $RPC_URL"
echo "BUCKET: $REMOTE_STORE_BUCKET"
echo "REMOTE_STORE_URL: $REMOTE_STORE_URL"
echo "METRICS_PORT: $METRICS_PORT"
if [ -n "$STREAMING_URL" ]; then
  echo "STREAMING_URL (gRPC): $STREAMING_URL"
  echo "Ingestion: streaming + gRPC GetCheckpoint fallback"
else
  echo "Ingestion: HTTP object store"
fi
echo ""
echo "Starting: myso-checkpoint-blob-indexer $*"
echo ""

exec myso-checkpoint-blob-indexer "$@"
