#!/bin/sh
set -e

echo "=== MySocial Analytics Indexer ==="
echo "Starting analytics indexer..."

# Support both RPC_API_URL and REST_URL for backward compatibility
RPC_URL="${RPC_API_URL:-$REST_URL}"
if [ -z "$RPC_URL" ]; then
  echo "ERROR: RPC_API_URL or REST_URL environment variable is not set"
  echo "Expected: RPC_API_URL=https://fullnode.testnet.mysocial.network:9000"
  exit 1
fi

if [ -z "$REMOTE_STORE_BUCKET" ]; then
  echo "ERROR: REMOTE_STORE_BUCKET environment variable is not set"
  echo "Expected: REMOTE_STORE_BUCKET=mysocial-testnet-analytics"
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

# Resolve pipeline types: PIPELINES env or FILE_TYPE (legacy) or default Checkpoint
# FILE_TYPE: checkpoint -> Checkpoint, transaction -> Transaction
case "${FILE_TYPE:-checkpoint}" in
  checkpoint) DEFAULT_PIPELINE="Checkpoint" ;;
  transaction) DEFAULT_PIPELINE="Transaction" ;;
  *) DEFAULT_PIPELINE="Checkpoint" ;;
esac
PIPELINE_TYPES="${PIPELINES:-$DEFAULT_PIPELINE}"

# Generate YAML config from environment variables
# Use PORT from Railway (or default 9184) so health checks reach the metrics server
METRICS_PORT="${PORT:-9184}"
CHECKPOINT_INTERVAL="${CHECKPOINT_INTERVAL:-10000}"
TIME_INTERVAL_S="${TIME_INTERVAL_S:-600}"
FILE_FORMAT="${FILE_FORMAT:-parquet}"
REMOTE_STORE_URL="${REMOTE_STORE_URL:-https://mysocial-testnet-checkpoints.storage.googleapis.com}"

cat > /app/config.yaml << EOF
client_metric_host: "0.0.0.0"
client_metric_port: $METRICS_PORT
rpc_api_url: "$RPC_URL"
remote_store_url: "$REMOTE_STORE_URL"
output_store:
  type: gcs
  bucket: "$REMOTE_STORE_BUCKET"
  service_account_path: "/app/credentials/gcs-key.json"
pipelines:
EOF

# Add each pipeline (comma-separated)
echo "$PIPELINE_TYPES" | tr ',' '\n' | while read -r p; do
  p=$(echo "$p" | tr -d ' ')
  [ -z "$p" ] && continue
  cat >> /app/config.yaml << PIPELINE
  - pipeline: $p
    file_format: $FILE_FORMAT
    batch_size:
      checkpoints: $CHECKPOINT_INTERVAL
    force_batch_cut_after_secs: $TIME_INTERVAL_S
PIPELINE
done

# Add optional first_checkpoint if set
if [ -n "$FIRST_CHECKPOINT" ]; then
  echo "first_checkpoint: $FIRST_CHECKPOINT" >> /app/config.yaml
fi
if [ -n "$LAST_CHECKPOINT" ]; then
  echo "last_checkpoint: $LAST_CHECKPOINT" >> /app/config.yaml
fi

echo "=== Configuration ==="
echo "RPC_URL: $RPC_URL"
echo "BUCKET: $REMOTE_STORE_BUCKET"
echo "PIPELINES: $PIPELINE_TYPES"
echo "METRICS_PORT: $METRICS_PORT"
echo "CHECKPOINT_INTERVAL: $CHECKPOINT_INTERVAL"
echo ""
echo "Starting: myso-analytics-indexer /app/config.yaml"
echo ""

exec myso-analytics-indexer /app/config.yaml
