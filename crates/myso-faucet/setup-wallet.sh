#!/bin/bash

# Install network tools for debugging
apt-get update -qq && apt-get install -y -qq curl iputils-ping

# Create config directory in the default MySocial location
mkdir -p /root/.myso/myso_config

# Check if we have the required environment variables
if [ -z "$WALLET_ADDRESS" ]; then
    echo "❌ Missing required WALLET_ADDRESS environment variable"
    exit 1
fi

# Check that we have either private key OR mnemonic
if [ -z "$WALLET_PRIVATE_KEY" ] && [ -z "$WALLET_MNEMONIC" ]; then
    echo "❌ Must provide either WALLET_PRIVATE_KEY or WALLET_MNEMONIC"
    exit 1
fi

# Set default network if not provided
NETWORK_URL=${NETWORK_URL:-"http://fullnode.testnet.mysocial.network:9000"}
NETWORK_ALIAS=${NETWORK_ALIAS:-"testnet"}

echo "🔧 Setting up wallet configuration..."
echo "📍 Network: $NETWORK_URL"
echo "💳 Address: $WALLET_ADDRESS"

# Test multiple MySocial endpoints to find one that works
echo "🌐 Testing MySocial network connectivity..."

ENDPOINTS=(
    "http://fullnode.testnet.mysocial.network:9000"
)

WORKING_ENDPOINT=""

for endpoint in "${ENDPOINTS[@]}"; do
    echo "🔍 Testing: $endpoint"
    if curl -s --connect-timeout 5 --max-time 10 "$endpoint" > /dev/null 2>&1; then
        echo "✅ Connection successful to: $endpoint"
        WORKING_ENDPOINT="$endpoint"
        break
    else
        echo "❌ Failed to connect to: $endpoint"
    fi
done

if [ -z "$WORKING_ENDPOINT" ]; then
    echo "🚨 No MySocial endpoints are reachable from Railway infrastructure"
    echo "🔄 Proceeding anyway - this may cause connection failures"
else
    echo "🎯 Using working endpoint: $WORKING_ENDPOINT"
    NETWORK_URL="$WORKING_ENDPOINT"
fi

# Create client.yaml
cat > /root/.myso/myso_config/client.yaml << EOF
keystore:
  File: /root/.myso/myso_config/myso.keystore
envs:
- alias: $NETWORK_ALIAS
  rpc: $NETWORK_URL
  ws: null
  basic_auth: null
active_env: $NETWORK_ALIAS
active_address: $WALLET_ADDRESS
EOF

# Create keystore file - MySocial format is just an array of base64-encoded keypairs
if [ ! -z "$WALLET_PRIVATE_KEY" ]; then
    echo "🔑 Using provided private key"
    cat > /root/.myso/myso_config/myso.keystore << EOF
[
  "$WALLET_PRIVATE_KEY"
]
EOF
elif [ ! -z "$WALLET_MNEMONIC" ]; then
    echo "🎯 Using mnemonic - faucet will derive keypair at runtime"
    # Faucet derives key from mnemonic internally; create empty keystore placeholder
    echo "[]" > /root/.myso/myso_config/myso.keystore
fi

echo "✅ Created client.yaml"
echo "✅ Created myso.keystore"
echo "🚀 Starting MySocial faucet..."

# Start the faucet with correct host binding for Railway
exec ./bin/myso-faucet --write-ahead-log /app/faucet.wal --host-ip 0.0.0.0 --port ${PORT:-5003}