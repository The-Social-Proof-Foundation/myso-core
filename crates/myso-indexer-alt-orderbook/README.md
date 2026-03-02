# Orderbook Indexer

The Orderbook Indexer uses the myso-indexer-alt framework to index Orderbook Move events from the MySo blockchain. It processes checkpoints and extracts event data for orders, trades, pools, margin trading, and governance.

## Getting Started

### Prerequisites

- **Rust** (latest stable)
- **PostgreSQL** (13+)

### Running Locally

```bash
DATABASE_URL="postgresql://user:pass@localhost/orderbook" \
cargo run -p myso-indexer-alt-orderbook --bin myso-orderbook-indexer -- \
  --env testnet \
  --packages orderbook orderbook-margin
```

### Parameters

- `--env` (required) – MySo network:
  - `testnet` – Development and testing
  - `mainnet` – Production (margin trading not yet on mainnet)

- `--packages` (optional, default: both) – Event types to index:
  - `orderbook` – Core events (orders, trades, pools, governance)
  - `orderbook-margin` – Margin events (lending, borrowing, liquidations)
  - Example: `--packages orderbook orderbook-margin`

- `--database-url` (optional) – PostgreSQL connection string. Also via `DATABASE_URL`.

- `--metrics-address` (optional, default: `0.0.0.0:9184`) – Prometheus metrics and health endpoint.

- `--streaming-url` (optional) – gRPC endpoint for checkpoint streaming. When unset, uses remote store HTTP ingestion.

- `--remote-store-url` (optional, `REMOTE_STORE_URL`) – HTTP checkpoint store URL. When set, used for ingestion instead of gRPC. Useful for testnet when fullnode GetCheckpoint returns NotFound.

- `--first-checkpoint` (optional) – Start ingestion from a specific checkpoint (e.g. for backfill).

### Examples

**Core Orderbook on testnet:**
```bash
DATABASE_URL="postgresql://user:pass@localhost/orderbook" \
cargo run -p myso-indexer-alt-orderbook --bin myso-orderbook-indexer -- \
  --env testnet --packages orderbook
```

**Core + margin on testnet:**
```bash
DATABASE_URL="postgresql://user:pass@localhost/orderbook" \
cargo run -p myso-indexer-alt-orderbook --bin myso-orderbook-indexer -- \
  --env testnet --packages orderbook orderbook-margin
```

**With gRPC streaming:**
```bash
DATABASE_URL="postgresql://user:pass@localhost/orderbook" \
cargo run -p myso-indexer-alt-orderbook --bin myso-orderbook-indexer -- \
  --env testnet --streaming-url http://fullnode.testnet.mysocial.network:9000
```

### Notes

- **Mainnet margin**: `--packages orderbook-margin` is not supported on mainnet yet.
- **Migrations**: Run automatically on startup.
- **Health**: Metrics service exposes `/health` and `/metrics` on the metrics address.

### Troubleshooting

**Ingestion failing / Not-Found Retries**

If Grafana shows `orderbook_indexer_total_ingested_not_found_retries` growing while checkpoint gauges stay at 0, the root cause is fullnode gRPC GetCheckpoint returning NotFound.

**Fix:** Set `REMOTE_STORE_URL` to a working HTTP checkpoint store (e.g. GCS bucket populated by checkpoint-blob-indexer). If no HTTP store has data, infra must either populate a GCS bucket or fix fullnode gRPC GetCheckpoint.

**Verification:** After deploying with `REMOTE_STORE_URL`, confirm `orderbook_indexer_total_ingested_checkpoints` increases and `orderbook_indexer_total_ingested_not_found_retries` stops growing.
