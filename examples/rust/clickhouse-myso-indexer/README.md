# ClickHouse MySo Analytics Indexer

A production-ready MySo analytics indexer that writes transaction data to ClickHouse for OLAP queries, dashboards, and analytics.

## One Command (Local Dev)

From this directory:

```bash
./scripts/start-local.sh
```

Starts ClickHouse (data in `~/clickhouse-data`), the indexer, and ch-ui. Stop with `./scripts/stop-local.sh`. Requires `clickhouse` and `ch-ui` binaries.

## Local Development: Three Services

The stack has three services. **ClickHouse must start first**; the indexer and ch-ui both depend on it.

### Port Reference

| Service | Port(s) | Purpose |
|---------|--------|---------|
| ClickHouse | 8123 (HTTP), 9000 (native) | Database; indexer uses native (9000), ch-ui uses HTTP (8123) |
| ch-ui | 3488 (default) or 5521 | Web UI for querying ClickHouse |
| Indexer | — | Client only; connects to ClickHouse via native protocol (9000) |

### Startup Order

**1. ClickHouse** (must be running before the others)

Native (recommended on macOS for speed). Use a dedicated data path to avoid storing in the project directory (slower on synced/network drives):

```bash
clickhouse server -- --path=/var/lib/clickhouse
# Or: CLICKHOUSE_DATA_PATH=/var/lib/clickhouse clickhouse server
```

Without `--path`, data is stored in the current directory.

Or Docker:

```bash
docker run -d --name clickhouse-dev -p 8123:8123 -p 9000:9000 --ulimit nofile=262144:262144 clickhouse/clickhouse-server
docker exec clickhouse-dev clickhouse-client --query "CREATE USER IF NOT EXISTS dev IDENTIFIED WITH no_password"
docker exec clickhouse-dev clickhouse-client --query "GRANT CREATE, INSERT, SELECT, ALTER, UPDATE, DELETE, DROP TABLE ON *.* TO dev"
```

**2. Indexer** (from this directory)

```bash
cd examples/rust/clickhouse-myso-indexer
CLICKHOUSE_USER=default cargo run -- run --remote-store-url https://storage.googleapis.com/mysocial-testnet-archive \
  --streaming-url http://fullnode.testnet.mysocial.network:9000 --first-checkpoint=2587781
```

Use `CLICKHOUSE_USER=dev` if using Docker ClickHouse with the `dev` user.

**3. ch-ui** (Web UI)

Native binary:

```bash
CLICKHOUSE_URL=http://localhost:8123 ch-ui
```

Or Docker (must set `CLICKHOUSE_URL` so the container can reach ClickHouse on the host):

```bash
docker run --rm -p 5521:3488 -v ch-ui-data:/app/data \
  -e CLICKHOUSE_URL=http://host.docker.internal:8123 \
  ghcr.io/caioricciuti/ch-ui:latest
```

Open http://localhost:3488 (native) or http://localhost:5521 (Docker). Log in with `default` (native) or `dev` (Docker ClickHouse).

### Shutdown Order

1. Stop ch-ui (Ctrl+C or `docker stop ch-ui`)
2. Stop the indexer (Ctrl+C)
3. Stop ClickHouse (Ctrl+C or `docker stop clickhouse-dev`)

### Verify ClickHouse Before Starting ch-ui or Indexer

```bash
curl -s "http://localhost:8123/?user=default&query=SELECT%201"
# Should return: 1
```

If you see `connection refused`, ClickHouse is not running. Start it first.

### Optional: Start/Stop Scripts

From this directory:

```bash
./scripts/start-local.sh   # Start all three (native ClickHouse, indexer, ch-ui)
./scripts/stop-local.sh    # Stop all
```

The start script stops any existing ClickHouse, uses `~/clickhouse-data` by default, and waits for readiness before starting the indexer. Override with env: `CLICKHOUSE_DATA_PATH`, `FIRST_CHECKPOINT`, `STREAMING_URL`, `CLICKHOUSE_HOST`, `CLICKHOUSE_PORT`.

---

## Quick Start

> **Note:** Run all commands from this directory (`examples/rust/clickhouse-myso-indexer`). The package is excluded from the workspace root, so `cargo run -p clickhouse-myso-indexer` from the repo root will fail.

### 1. Start ClickHouse

```bash
docker run -d --name clickhouse-dev -p 8123:8123 -p 9000:9000 --ulimit nofile=262144:262144 clickhouse/clickhouse-server
```

### 2. Set up database user

```bash
docker exec clickhouse-dev clickhouse-client --query "CREATE USER IF NOT EXISTS dev IDENTIFIED WITH no_password"
docker exec clickhouse-dev clickhouse-client --query "GRANT CREATE, INSERT, SELECT, ALTER, UPDATE, DELETE, DROP TABLE ON *.* TO dev"
```

### 3. Run the indexer

```bash
cargo run -- run --remote-store-url https://storage.googleapis.com/mysocial-testnet-checkpoints --last-checkpoint=10
```

That's it! The indexer will:
- Create the necessary tables automatically
- Fetch checkpoints from the MySo testnet
- Write transaction data to ClickHouse

### gRPC streaming (required for live sync)

When a fullnode gRPC endpoint is available, **always use `--streaming-url`** for real-time checkpoint delivery. Streaming pushes checkpoints as they are produced. Without it, the indexer will freeze when it catches up to the chain head (ingestion waits forever for checkpoints that do not exist yet).

```bash
cargo run -- run --remote-store-url https://storage.googleapis.com/mysocial-testnet-archive \
  --streaming-url http://fullnode.testnet.mysocial.network:9000
```

**gRPC backfill (faster than HTTP):** Use `--rpc-api-url` instead of `--remote-store-url` to fetch historical checkpoints from the fullnode over gRPC. You must still pass `--streaming-url` so the indexer can switch to streaming at the chain head:

```bash
cargo run -- run --rpc-api-url http://fullnode.testnet.mysocial.network:9000 \
  --streaming-url http://fullnode.testnet.mysocial.network:9000 --first-checkpoint=3162102
```

Use `STREAMING_URL` env var or `--streaming-url` directly. If omitted, the indexer uses only ingestion and will freeze when it reaches the chain head.

## Verify Data

Check that data was written:

```bash
# Row count
docker exec clickhouse-dev clickhouse-client --user=dev --query "SELECT COUNT(*) FROM transactions"

# Recent transactions
docker exec clickhouse-dev clickhouse-client --user=dev --query "SELECT * FROM transactions ORDER BY checkpoint_sequence_number DESC LIMIT 10"

# By status (0=success, 1=failure)
docker exec clickhouse-dev clickhouse-client --user=dev --query "SELECT status, COUNT(*) FROM transactions GROUP BY status"

# Throughput by checkpoint
docker exec clickhouse-dev clickhouse-client --user=dev --query "SELECT checkpoint_sequence_number, count() as tx_count FROM transactions GROUP BY checkpoint_sequence_number ORDER BY checkpoint_sequence_number DESC LIMIT 20"
```

## Analytics Queries

Daily transaction volume with success/failure and gas:

```sql
SELECT
  toDate(toDateTime64(timestamp_ms / 1000, 3, 'UTC')) AS day,
  count() AS tx_count,
  countIf(status = 0) AS success_count,
  uniq(sender) AS unique_senders,
  sum(gas_computation_cost + gas_storage_cost - gas_storage_rebate) AS total_gas_cost
FROM transactions
GROUP BY day
ORDER BY day;
```

30-day active wallets:

```sql
SELECT uniq(sender) AS active_wallets_30d
FROM transactions
WHERE timestamp_ms >= (toUnixTimestamp64Milli(now64()) - 30 * 24 * 60 * 60 * 1000);
```

## Resetting / Re-indexing

If you need to re-run the indexer for the same checkpoint range (e.g. after fixing schema issues or migrating from Docker to native ClickHouse), reset the tables first. Otherwise the indexer will skip ingestion because it thinks it has already processed those checkpoints.

**Using the indexer's built-in reset** (recommended; works with both Docker and native ClickHouse):

```bash
# From examples/rust/clickhouse-myso-indexer
CLICKHOUSE_USER=default cargo run -- reset
```

**Using ClickHouse CLI directly:**

Docker:
```bash
docker exec clickhouse-dev clickhouse-client --query "DROP TABLE IF EXISTS watermarks"
docker exec clickhouse-dev clickhouse-client --query "DROP TABLE IF EXISTS transactions"
```

Native ClickHouse:
```bash
clickhouse client --query "DROP TABLE IF EXISTS watermarks"
clickhouse client --query "DROP TABLE IF EXISTS transactions"
```

Then run the indexer again: `cargo run -- run --remote-store-url ... --first-checkpoint=<N>`

## Clean Up

Stop and remove the ClickHouse container:

```bash
docker stop clickhouse-dev && docker rm clickhouse-dev
```

## Running ClickHouse Natively (Outside Docker)

Running ClickHouse natively avoids Docker VM overhead and can significantly speed up indexing on Apple Silicon.

### 1. Install ClickHouse

```bash
brew install --cask clickhouse
```

If you hit a developer verification error on macOS, run:

```bash
xattr -d com.apple.quarantine $(which clickhouse)
```

### 2. Start ClickHouse server

```bash
clickhouse server
```

Leave this running in a terminal. Data is stored in the current directory.

### 3. Run the indexer with default user

Native ClickHouse uses the `default` user with no password. Set the env var and run:

```bash
CLICKHOUSE_USER=default cargo run -- run --remote-store-url https://storage.googleapis.com/mysocial-testnet-archive --first-checkpoint=2587781
```

**To see progress** (commit logs), run with `RUST_LOG=info`:

```bash
CLICKHOUSE_USER=default RUST_LOG=info cargo run -- run --remote-store-url https://storage.googleapis.com/mysocial-testnet-archive --first-checkpoint=2587781
```

Without this, the indexer runs silently after the startup message.

### 4. Data path (avoid storing in project directory)

By default, ClickHouse stores data in the current directory. To use a dedicated data path outside the project:

```bash
clickhouse server -- --path=/var/lib/clickhouse
```

Or use a config override (see `config.d/path.xml.example`). Set `CLICKHOUSE_DATA_PATH` before starting if using the example config.

### 5. Query with clickhouse-client

```bash
clickhouse client --query "SELECT count() FROM transactions"
clickhouse client --query "SELECT * FROM transactions ORDER BY timestamp_ms DESC LIMIT 10"
```

### Migrating from Docker

If you have existing data in Docker, you'll need to re-index. Stop Docker ClickHouse, start native ClickHouse, drop tables if any exist, and run the indexer from your desired `--first-checkpoint`.

### Operation timeout / Broken pipe

If you see `Error writing batch: Connection error: operation timeout` or `I/O error: Broken pipe`, inserts are taking too long (>60s) or the connection was closed. The indexer automatically reconnects on these errors.

To reduce insert latency:

1. **Data path**: Run ClickHouse with `--path=/var/lib/clickhouse` (or another fast disk). Avoid storing data in the project directory or on network/synced drives.
2. **Batch size**: The indexer uses small batches (100–2000 rows). If ClickHouse is under merge pressure, consider lowering `MAX_PENDING_ROWS` in `handlers.rs`.
3. **ClickHouse config**: For high throughput, consider `async_insert=1` or tuning MergeTree settings. Check ClickHouse logs for slow merges.

## Production Deployment

### Checklist

- **ClickHouse connection**: Use `--clickhouse-host` / `--clickhouse-port` or `CLICKHOUSE_HOST` / `CLICKHOUSE_PORT` (default: localhost:9000 for native protocol).
- **Data path**: Run ClickHouse with `--path` outside the project directory (e.g. `/var/lib/clickhouse`). See `config.d/path.xml.example`.
- **Docker with volume**: `docker run -d -v /var/lib/clickhouse:/var/lib/clickhouse -p 8123:8123 clickhouse/clickhouse-server`
- **Schema changes**: If upgrading from an older schema, run `cargo run -- reset` and re-index, or use `ALTER TABLE` to add new columns manually.

### Deploy Order (same as local)

1. Start ClickHouse
2. Start the indexer (as a systemd service, Docker container, or supervisor process)
3. Start ch-ui (optional; for web UI access)

## Schema (transactions table)

| Column | Type | Description |
|--------|------|-------------|
| checkpoint_sequence_number | UInt64 | Checkpoint containing this transaction |
| transaction_digest | String | Transaction digest |
| sender | String | Sender address |
| timestamp_ms | Int64 | Checkpoint timestamp (ms) |
| tx_kind | LowCardinality(String) | Transaction kind (e.g. ProgrammableTransaction, ChangeEpoch) |
| gas_computation_cost | UInt64 | Gas computation cost |
| gas_storage_cost | UInt64 | Gas storage cost |
| gas_storage_rebate | UInt64 | Gas storage rebate |
| status | UInt8 | 0=success, 1=failure |
| epoch | UInt64 | Epoch number |
| gas_price | UInt64 | Gas price |
| gas_budget | UInt64 | Gas budget |
| gas_owner | String | Gas owner (for sponsored txns) |
| is_sponsored | UInt8 | 1 if sponsored |
| created_objects | UInt32 | Number of created objects |
| mutated_objects | UInt32 | Number of mutated objects |
| execution_error | Nullable(String) | Error message if status=1 |
| indexed_at | DateTime64 | When the row was indexed |

## What This Indexer Provides

- **Custom Store Implementation**: Implements the `Store` trait for ClickHouse
- **Concurrent Pipeline**: Processes checkpoints out-of-order with reader, committer, and pruner
- **Watermark Management**: Tracks indexer progress for resumability
- **Rich Transaction Data**: Epoch, tx kind, gas, sponsorship, object counts, execution errors

## Architecture

```
MySo Network → Checkpoints → Concurrent Pipeline → ClickHouse Store → ClickHouse DB
```

The indexer uses a concurrent pipeline that processes checkpoints out-of-order with separate reader, committer, and pruner components.