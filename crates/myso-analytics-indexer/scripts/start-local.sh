#!/usr/bin/env bash
# Start ch-ui. Needs ClickHouse on localhost:8123. Run from crates/myso-analytics-indexer.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INDEXER_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CH_UI_PID="$INDEXER_DIR/ch-ui-server.pid"

cd "$INDEXER_DIR"

# Stop any existing ch-ui first
"$SCRIPT_DIR/stop-local.sh" 2>/dev/null || true
sleep 2

ch-ui server start --detach --clickhouse-url http://localhost:8123 --pid-file "$CH_UI_PID"
echo "ch-ui at http://localhost:3488"
