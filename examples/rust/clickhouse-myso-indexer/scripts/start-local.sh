#!/usr/bin/env bash
# Start all three services for local development: ClickHouse, indexer, ch-ui.
# Run from examples/rust/clickhouse-myso-indexer. Stop with ./scripts/stop-local.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INDEXER_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$INDEXER_DIR"

PID_FILE="$INDEXER_DIR/.local-dev.pids"
REMOTE_STORE="${REMOTE_STORE_URL:-https://storage.googleapis.com/mysocial-testnet-archive}"
FIRST_CP="${FIRST_CHECKPOINT:-2587781}"
# gRPC streaming keeps indexer fast when approaching chain head (HTTP-only slows down at the end)
STREAMING_URL="${STREAMING_URL:-http://fullnode.testnet.mysocial.network:9000}"

# Default data path: ~/clickhouse-data (works on macOS without sudo)
CLICKHOUSE_DATA_PATH="${CLICKHOUSE_DATA_PATH:-$HOME/clickhouse-data}"
mkdir -p "$CLICKHOUSE_DATA_PATH"

echo "=== 1. Starting ClickHouse ==="
# Stop any existing ClickHouse so we can bind ports
pkill -f "clickhouse server" 2>/dev/null || true
sleep 2

if command -v clickhouse &>/dev/null; then
  echo "Data path: $CLICKHOUSE_DATA_PATH"
  clickhouse server -- --path="$CLICKHOUSE_DATA_PATH" &
  CH_PID=$!
  echo "$CH_PID" > "$PID_FILE"
  CH_USER=default
else
  echo "ClickHouse binary not found. Use Docker:"
  echo "  docker run -d --name clickhouse-dev -p 8123:8123 -p 9000:9000 --ulimit nofile=262144:262144 clickhouse/clickhouse-server"
  echo "  docker exec clickhouse-dev clickhouse-client --query \"CREATE USER IF NOT EXISTS dev IDENTIFIED WITH no_password\""
  echo "  docker exec clickhouse-dev clickhouse-client --query \"GRANT CREATE, INSERT, SELECT, ALTER, UPDATE, DELETE, DROP TABLE ON *.* TO dev\""
  echo "Then run this script again."
  exit 1
fi

echo "Waiting for ClickHouse..."
until curl -s "http://localhost:8123/?user=$CH_USER&query=SELECT%201" 2>/dev/null | grep -q 1; do
  sleep 1
done
echo "ClickHouse ready."

echo ""
echo "=== 2. Starting indexer ==="
# Use rpc-api-url + streaming-url for gRPC (faster, avoids "outside buffer" stalls)
INDEXER_ARGS=(run --rpc-api-url "$STREAMING_URL" --streaming-url "$STREAMING_URL" --first-checkpoint="$FIRST_CP")
# Indexer uses native protocol (port 9000); ch-ui uses HTTP (8123)
CLICKHOUSE_USER=$CH_USER CLICKHOUSE_HOST="${CLICKHOUSE_HOST:-localhost}" CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-9000}" cargo run -- "${INDEXER_ARGS[@]}" &
echo $! >> "$PID_FILE"

echo ""
echo "=== 3. Starting ch-ui ==="
CH_UI_PID_FILE="$INDEXER_DIR/ch-ui-server.pid"
if command -v ch-ui &>/dev/null; then
  (cd "$INDEXER_DIR" && ch-ui server stop --pid-file "$CH_UI_PID_FILE" 2>/dev/null) || true
  if [[ -f "$CH_UI_PID_FILE" ]]; then
    kill -9 "$(cat "$CH_UI_PID_FILE")" 2>/dev/null || true
    rm -f "$CH_UI_PID_FILE"
  fi
  pkill -9 -f "ch-ui" 2>/dev/null || true
  if command -v lsof &>/dev/null; then
    lsof -ti:3488 | xargs kill -9 2>/dev/null || true
  fi
  sleep 2
  (cd "$INDEXER_DIR" && ch-ui server start --detach --clickhouse-url http://localhost:8123 --pid-file "$CH_UI_PID_FILE") && echo "ch-ui at http://localhost:3488" || echo "ch-ui failed to start (optional)"
else
  echo "ch-ui binary not found. Run manually:"
  echo "  CLICKHOUSE_URL=http://localhost:8123 ch-ui"
  echo "Or use Docker:"
  echo "  docker run --rm -p 5521:3488 -v ch-ui-data:/app/data -e CLICKHOUSE_URL=http://host.docker.internal:8123 ghcr.io/caioricciuti/ch-ui:latest"
  echo "  Then open http://localhost:5521"
fi

echo ""
echo "All services started. PIDs in $PID_FILE"
echo "Stop with: ./scripts/stop-local.sh"
