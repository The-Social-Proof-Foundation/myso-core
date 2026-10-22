#!/bin/sh
set -e

echo "=== MySocial Checkpoint Blob Indexer ==="
echo "Starting checkpoint blob indexer..."

# Support both RPC_API_URL and REST_URL for backward compatibility
RPC_URL="${RPC_API_URL:-$REST_URL}"
if [ -z "$RPC_URL" ]; then
  echo "ERROR: RPC_API_URL or REST_URL environment variable is not set"
  echo "Expected: RPC_API_URL=https://fullnode.testnet.mysocial.network:443"
  exit 1
fi

if [ -z "$REMOTE_STORE_BUCKET" ]; then
  echo "ERROR: REMOTE_STORE_BUCKET environment variable is not set"
  echo "Expected: REMOTE_STORE_BUCKET=checkpoints-archive"
  exit 1
fi

# AmazonS3Builder::from_env reads AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY,
# AWS_DEFAULT_REGION (or AWS_REGION), and optional AWS_ENDPOINT.
if [ -z "$AWS_ACCESS_KEY_ID" ] || [ -z "$AWS_SECRET_ACCESS_KEY" ]; then
  echo "ERROR: AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY must be set"
  exit 1
fi
if [ -z "${AWS_DEFAULT_REGION:-$AWS_REGION}" ]; then
  echo "ERROR: AWS_DEFAULT_REGION or AWS_REGION must be set"
  exit 1
fi
export AWS_DEFAULT_REGION="${AWS_DEFAULT_REGION:-$AWS_REGION}"

# Use PORT from Railway (or default 9184) so health checks reach the metrics server
METRICS_PORT="${PORT:-9184}"
COMPRESSION_LEVEL="${COMPRESSION_LEVEL:-3}"

# Build CLI args - when streaming is set, ingestion uses gRPC fallback (no remote-store-url needed)
set -- --s3 "$REMOTE_STORE_BUCKET" \
  --metrics-address "0.0.0.0:$METRICS_PORT" \
  --compression-level "$COMPRESSION_LEVEL"

if [ -n "$STREAMING_URL" ]; then
  set -- "$@" --streaming-url "$STREAMING_URL" --rpc-api-url "$STREAMING_URL"
else
  if [ -z "$REMOTE_STORE_URL" ]; then
    echo "ERROR: REMOTE_STORE_URL must be set when STREAMING_URL is unset"
    exit 1
  fi
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
echo "AWS_DEFAULT_REGION: $AWS_DEFAULT_REGION"
if [ -n "$AWS_ENDPOINT" ]; then
  echo "AWS_ENDPOINT: $AWS_ENDPOINT"
fi
echo "REMOTE_STORE_URL: ${REMOTE_STORE_URL:-unset}"
echo "METRICS_PORT: $METRICS_PORT"
echo "COMPRESSION_LEVEL: $COMPRESSION_LEVEL"
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
