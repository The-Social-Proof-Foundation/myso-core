#!/usr/bin/env bash
# Stop ch-ui. Run from crates/myso-analytics-indexer.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INDEXER_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CH_UI_PID="$INDEXER_DIR/ch-ui-server.pid"

cd "$INDEXER_DIR"

ch-ui server stop --pid-file "$CH_UI_PID" 2>/dev/null || true
pkill -f "ch-ui" 2>/dev/null || true
lsof -ti:3488 | xargs kill -9 2>/dev/null || true
rm -f "$CH_UI_PID"
echo "ch-ui stopped"
