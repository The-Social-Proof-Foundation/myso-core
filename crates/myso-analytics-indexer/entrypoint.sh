#!/bin/sh
set -e

echo "=== MySocial Analytics Indexer ==="
echo "Starting analytics indexer..."

# Support both RPC_API_URL and REST_URL for backward compatibility
RPC_URL="${RPC_API_URL:-$REST_URL}"
if [ -z "$RPC_URL" ]; then
  echo "ERROR: RPC_API_URL or REST_URL environment variable is not set"
  echo "Expected: RPC_API_URL=http://fullnode.testnet.mysocial.network:9000"
  exit 1
fi

# ClickHouse output: set CLICKHOUSE_HOST (and optionally CLICKHOUSE_PORT, CLICKHOUSE_USER)
# GCS output: set REMOTE_STORE_BUCKET and GCS_SERVICE_ACCOUNT_JSON
USE_CLICKHOUSE=false
if [ -n "$CLICKHOUSE_HOST" ]; then
  USE_CLICKHOUSE=true
  CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-9000}"
  CLICKHOUSE_USER="${CLICKHOUSE_USER:-default}"
fi

if [ "$USE_CLICKHOUSE" = "false" ] && [ -z "$REMOTE_STORE_BUCKET" ]; then
  echo "ERROR: Set REMOTE_STORE_BUCKET for GCS, or CLICKHOUSE_HOST for ClickHouse"
  exit 1
fi

# Create GCS service account key from environment variables (only when not using ClickHouse)
if [ "$USE_CLICKHOUSE" = "false" ] && [ -n "$GCS_SERVICE_ACCOUNT_JSON" ]; then
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
elif [ "$USE_CLICKHOUSE" = "false" ]; then
  echo "WARNING: GCS_SERVICE_ACCOUNT_JSON not set - uploads may fail"
fi

# When using ClickHouse, default to Transaction,Event,MoveCall pipelines
case "${FILE_TYPE:-checkpoint}" in
  checkpoint) DEFAULT_PIPELINE="Checkpoint" ;;
  transaction) DEFAULT_PIPELINE="Transaction" ;;
  *) DEFAULT_PIPELINE="Checkpoint" ;;
esac
if [ "$USE_CLICKHOUSE" = "true" ]; then
  PIPELINE_TYPES="${PIPELINES:-Transaction,Event,MoveCall}"
  CLICKHOUSE_BATCH_ROWS="${CLICKHOUSE_BATCH_ROWS:-100}"
  CLICKHOUSE_FORCE_CUT_SECS="${CLICKHOUSE_FORCE_CUT_SECS:-5}"
else
  PIPELINE_TYPES="${PIPELINES:-$DEFAULT_PIPELINE}"
fi

# Generate YAML config from environment variables
# Use PORT from Railway (or default 9184) so health checks reach the metrics server
METRICS_PORT="${PORT:-9184}"
CHECKPOINT_INTERVAL="${CHECKPOINT_INTERVAL:-10000}"
TIME_INTERVAL_S="${TIME_INTERVAL_S:-600}"
FILE_FORMAT="${FILE_FORMAT:-parquet}"
REMOTE_STORE_URL="${REMOTE_STORE_URL:-https://mysocial-testnet-checkpoints.storage.googleapis.com}"
# Default FIRST_CHECKPOINT near chain head to avoid massive backfill (update as chain grows)
FIRST_CHECKPOINT="${FIRST_CHECKPOINT:-1159000}"

# When STREAMING_URL is set, use gRPC for checkpoint ingestion (streaming primary, remote_store fallback).
USE_GRPC=false
if [ -n "$STREAMING_URL" ]; then
  USE_GRPC=true
fi

cat > /app/config.yaml << EOF
client_metric_host: "0.0.0.0"
client_metric_port: $METRICS_PORT
rpc_api_url: "$RPC_URL"
EOF

if [ "$USE_GRPC" = "true" ]; then
  echo "streaming_url: \"$STREAMING_URL\"" >> /app/config.yaml
  # When streaming is set, ingestion uses gRPC GetCheckpoint - no remote_store_url needed
else
  echo "WARNING: STREAMING_URL not set - using HTTP only (slower). Set STREAMING_URL for gRPC streaming."
  echo "remote_store_url: \"$REMOTE_STORE_URL\"" >> /app/config.yaml
fi

if [ "$USE_CLICKHOUSE" = "true" ]; then
  cat >> /app/config.yaml << EOF

output_store:
  type: clickhouse
  host: "$CLICKHOUSE_HOST"
  port: $CLICKHOUSE_PORT
  user: "$CLICKHOUSE_USER"
pipelines:
EOF
else
  cat >> /app/config.yaml << EOF

output_store:
  type: gcs
  bucket: "$REMOTE_STORE_BUCKET"
  service_account_path: "/app/credentials/gcs-key.json"
pipelines:
EOF
fi

# Add each pipeline (comma-separated)
# ClickHouse mode: use rows-based batching (100 rows, 5 sec) to match clickhouse-myso-indexer example
# GCS mode: use checkpoint-based batching
echo "$PIPELINE_TYPES" | tr ',' '\n' | while read -r p; do
  p=$(echo "$p" | tr -d ' ')
  [ -z "$p" ] && continue
  if [ "$USE_CLICKHOUSE" = "true" ]; then
    cat >> /app/config.yaml << PIPELINE
  - pipeline: $p
    file_format: $FILE_FORMAT
    batch_size:
      rows: $CLICKHOUSE_BATCH_ROWS
    force_batch_cut_after_secs: $CLICKHOUSE_FORCE_CUT_SECS
PIPELINE
  else
    cat >> /app/config.yaml << PIPELINE
  - pipeline: $p
    file_format: $FILE_FORMAT
    batch_size:
      checkpoints: $CHECKPOINT_INTERVAL
    force_batch_cut_after_secs: $TIME_INTERVAL_S
PIPELINE
  fi
done

# Add first_checkpoint (default 765000 to avoid massive backfill)
echo "first_checkpoint: $FIRST_CHECKPOINT" >> /app/config.yaml
if [ -n "$LAST_CHECKPOINT" ]; then
  echo "last_checkpoint: $LAST_CHECKPOINT" >> /app/config.yaml
fi

echo "=== Configuration ==="
echo "RPC_URL: $RPC_URL"
if [ "$USE_CLICKHOUSE" = "true" ]; then
  echo "OUTPUT: ClickHouse $CLICKHOUSE_HOST:$CLICKHOUSE_PORT"
else
  echo "BUCKET: $REMOTE_STORE_BUCKET"
fi
echo "PIPELINES: $PIPELINE_TYPES"
echo "METRICS_PORT: $METRICS_PORT"
echo "CHECKPOINT_INTERVAL: $CHECKPOINT_INTERVAL"
echo "FIRST_CHECKPOINT: $FIRST_CHECKPOINT"
if [ "$USE_GRPC" = "true" ]; then
  echo "STREAMING_URL (gRPC): $STREAMING_URL"
  echo "Ingestion: streaming + gRPC GetCheckpoint fallback"
else
  echo "REMOTE_STORE_URL: $REMOTE_STORE_URL"
  echo "Ingestion: HTTP object store"
fi
echo ""
echo "=== Generated config.yaml ==="
cat /app/config.yaml
echo "============================="
echo ""
echo "Starting: myso-analytics-indexer /app/config.yaml"
echo ""

exec myso-analytics-indexer /app/config.yaml
