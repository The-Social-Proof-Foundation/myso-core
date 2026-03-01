#!/bin/sh
set -e

echo "=== MySocial Social Server ==="

export RUST_LOG="${RUST_LOG:-info}"

API_PORT="${PORT:-9009}"

if [ -z "$DATABASE_URL" ]; then
  echo "ERROR: DATABASE_URL is not set"
  exit 1
fi

echo "Starting: myso-social-server --server-port $API_PORT --metrics-address 0.0.0.0:9186 --database-url <redacted>"
exec myso-social-server \
  --server-port "$API_PORT" \
  --metrics-address "0.0.0.0:9186" \
  --database-url "$DATABASE_URL"
