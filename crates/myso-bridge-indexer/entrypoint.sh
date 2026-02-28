#!/bin/sh
set -e

echo "=== MySocial Bridge Indexer ==="
echo "Starting bridge indexer..."

DB_URL="${DATABASE_URL:-$DB_URL}"
if [ -z "$DB_URL" ]; then
  echo "ERROR: DATABASE_URL or DB_URL environment variable is not set"
  echo "Expected: DATABASE_URL=postgres://user:pass@host:5432/bridge"
  exit 1
fi

if [ -z "$ETH_RPC_URL" ]; then
  echo "ERROR: ETH_RPC_URL environment variable is not set"
  echo "Expected: ETH_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY"
  exit 1
fi

if [ -z "$MYSO_RPC_URL" ]; then
  echo "ERROR: MYSO_RPC_URL environment variable is not set"
  echo "Expected: MYSO_RPC_URL=http://fullnode.testnet.mysocial.network:9000"
  exit 1
fi

if [ -z "$ETH_MYSO_BRIDGE_CONTRACT_ADDRESS" ]; then
  echo "ERROR: ETH_MYSO_BRIDGE_CONTRACT_ADDRESS environment variable is not set"
  echo "Expected (testnet): ETH_MYSO_BRIDGE_CONTRACT_ADDRESS=0xAE68F87938439afEEDd6552B0E83D2CbC2473623"
  exit 1
fi

REMOTE_STORE_URL="${REMOTE_STORE_URL:-https://storage.googleapis.com/mysocial-testnet-checkpoints}"
ETH_WS_URL="${ETH_WS_URL:-$(echo "$ETH_RPC_URL" | sed 's|^https://|wss://|;s|^http://|ws://|')}"
METRIC_PORT="${PORT:-${METRIC_PORT:-9184}}"
CONCURRENCY="${CONCURRENCY:-500}"
MYSO_BRIDGE_GENESIS_CHECKPOINT="${MYSO_BRIDGE_GENESIS_CHECKPOINT:-43917829}"
ETH_BRIDGE_GENESIS_BLOCK="${ETH_BRIDGE_GENESIS_BLOCK:-5997013}"

CONFIG_PATH="/myso/config.yaml"
mkdir -p "$(dirname "$CONFIG_PATH")"

cat > "$CONFIG_PATH" << EOF
remote_store_url: "$REMOTE_STORE_URL"
myso_rpc_url: "$MYSO_RPC_URL"
eth_rpc_url: "$ETH_RPC_URL"
eth_ws_url: "$ETH_WS_URL"
db_url: "$DB_URL"
concurrency: $CONCURRENCY
myso_bridge_genesis_checkpoint: $MYSO_BRIDGE_GENESIS_CHECKPOINT
eth_bridge_genesis_block: $ETH_BRIDGE_GENESIS_BLOCK
eth_myso_bridge_contract_address: "$ETH_MYSO_BRIDGE_CONTRACT_ADDRESS"
metric_port: $METRIC_PORT
EOF

if [ -n "$CHECKPOINTS_PATH" ]; then
  echo "checkpoints_path: \"$CHECKPOINTS_PATH\"" >> "$CONFIG_PATH"
fi

echo "=== Configuration ==="
echo "REMOTE_STORE_URL: $REMOTE_STORE_URL"
echo "MYSO_RPC_URL: $MYSO_RPC_URL"
echo "ETH_RPC_URL: (set)"
echo "ETH_WS_URL: (set)"
echo "DB_URL: (set)"
echo "CONCURRENCY: $CONCURRENCY"
echo "MYSO_BRIDGE_GENESIS_CHECKPOINT: $MYSO_BRIDGE_GENESIS_CHECKPOINT"
echo "ETH_BRIDGE_GENESIS_BLOCK: $ETH_BRIDGE_GENESIS_BLOCK"
echo "ETH_MYSO_BRIDGE_CONTRACT_ADDRESS: $ETH_MYSO_BRIDGE_CONTRACT_ADDRESS"
echo "METRIC_PORT: $METRIC_PORT"
echo ""
echo "=== Generated config.yaml ==="
cat "$CONFIG_PATH"
echo "============================="
echo ""
echo "Starting: bridge-indexer --config-path $CONFIG_PATH"
echo ""

exec bridge-indexer --config-path "$CONFIG_PATH"
