#!/usr/bin/env bash
# Stop services started by start-local.sh. Run from examples/rust/clickhouse-myso-indexer.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INDEXER_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PID_FILE="$INDEXER_DIR/.local-dev.pids"

cd "$INDEXER_DIR"

if [[ ! -f "$PID_FILE" ]]; then
  echo "No .local-dev.pids found. Nothing to stop."
  exit 0
fi

echo "Stopping services..."
while read -r pid; do
  if kill -0 "$pid" 2>/dev/null; then
    kill "$pid" 2>/dev/null || true
  fi
done < "$PID_FILE"
rm -f "$PID_FILE"

# Also stop clickhouse if running (it may have a different process name)
pkill -f "clickhouse server" 2>/dev/null || true

# Stop ch-ui (use same pid file as start-local.sh)
CH_UI_PID_FILE="$INDEXER_DIR/ch-ui-server.pid"
(cd "$INDEXER_DIR" && ch-ui server stop --pid-file "$CH_UI_PID_FILE" 2>/dev/null) || true
pkill -f "ch-ui" 2>/dev/null || true

echo "Stopped. If using Docker ClickHouse, run: docker stop clickhouse-dev"
