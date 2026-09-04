#!/usr/bin/env bash
# Reset orderbook indexer Postgres state after localnet regenesis.
# Watermarks are read only at myso startup — restart myso after running this.
set -euo pipefail

ORDERBOOK_DB_URL="${ORDERBOOK_DB_URL:-postgresql://postgres@localhost:5432/orderbook}"

echo "Resetting orderbook indexer DB at ${ORDERBOOK_DB_URL}" >&2
psql "$ORDERBOOK_DB_URL" -v ON_ERROR_STOP=1 <<'SQL'
UPDATE watermarks SET
  checkpoint_hi_inclusive = 0,
  epoch_hi_inclusive = 0,
  tx_hi = 0,
  timestamp_ms_hi_inclusive = 0,
  reader_lo = 0,
  pruner_hi = 0;
TRUNCATE order_fills, order_updates RESTART IDENTITY;
SQL
echo "Done. Restart myso (same flags, no hot reload) so the orderbook indexer re-reads watermarks." >&2
