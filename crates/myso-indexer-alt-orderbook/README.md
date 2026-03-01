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
