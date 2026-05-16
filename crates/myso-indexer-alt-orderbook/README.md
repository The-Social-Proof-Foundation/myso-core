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

- `--env` (required) – logical network for package resolution:
  - `local` – `myso start` + local checkpoint blobs; uses genesis orderbook addresses from `myso_types` (same as other values; no remote implied).
  - `testnet` / `mainnet` – public remote checkpoint stores (see below).

- **Mainnet margin**: `--packages orderbook-margin` is not supported on mainnet yet.
- **Migrations**: Run automatically on startup.
- **Health**: Metrics service exposes `/health` and `/metrics` on the metrics address.

### Local / `myso start`

When you run `myso start --with-indexer --with-orderbook`, the embedded orderbook indexer uses:

- `OrderbookEnv::Local`
- Checkpoint blobs under `<myso config dir>/data_ingestion/` (same directory the fullnode writes; persisted across restarts so Postgres watermarks stay aligned with files on disk)
- A separate Postgres database (often named `orderbook`) — confirm you query the same DB URL the process logs

**Metrics:** `orderbook_indexer_total_ingested_events` should grow when checkpoint transactions include events. If it stays near zero while `orderbook_indexer_total_ingested_checkpoints` grows, check logs at target `orderbook_indexer` for warnings about orderbook transactions missing events.

### Data written vs Orderbook REST `pools` table

The indexer inserts **event** rows (`pool_created`, fills, etc.). The Orderbook REST API’s **`pools` table** is a separate **catalog** filled via the orderbook server **admin API**, not by this indexer. See [orderbook-server README](../myso-orderbook-server/README.md#database-indexer-vs-catalog-important-for-operators).

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

- **Standalone `myso-orderbook-indexer` with `--env local`:** you must pass **`--local-ingestion-path`** (full node checkpoint dir) or **`--streaming-url`** / **`--remote-store-url`** — the binary will not guess a path.
- **Mainnet margin**: `--packages orderbook-margin` is not supported on mainnet yet.

### Troubleshooting

**Ingestion failing / Not-Found Retries**

If Grafana shows `orderbook_indexer_total_ingested_not_found_retries` growing while checkpoint gauges stay at 0, the root cause is fullnode gRPC GetCheckpoint returning NotFound.

**Fix:** Set `REMOTE_STORE_URL` to a working HTTP checkpoint store (e.g. GCS bucket populated by checkpoint-blob-indexer). If no HTTP store has data, infra must either populate a GCS bucket or fix fullnode gRPC GetCheckpoint.

**Verification:** After deploying with `REMOTE_STORE_URL`, confirm `orderbook_indexer_total_ingested_checkpoints` increases and `orderbook_indexer_total_ingested_not_found_retries` stops growing.
