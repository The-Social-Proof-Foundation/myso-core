#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Usage: ./scripts/bootstrap.sh

set -euo pipefail

GRAPHQL_URL='http://127.0.0.1:9125/graphql'

GQL='query BootstrapKey {
  bootstrap: objects(
    filter: { type: "0x2::bootstrap_key::BootstrapKey", ownerKind: SHARED }
    first: 1
  ) {
    nodes { address }
  }
}'

echo ">>> Faucet"
myso client faucet

echo ">>> Resolving BootstrapKey from GraphQL"
bootstrap_key_id="$(
  curl -sS -X POST "$GRAPHQL_URL" \
    -H 'Content-Type: application/json' \
    -d "$(jq -nc --arg q "$GQL" '{query: $q}')" \
  | jq -r '.data.bootstrap.nodes[0].address'
)"
echo "BootstrapKey: $bootstrap_key_id"

echo ">>> claim_all_admin_capabilities"
myso client call \
  --package 0x50c1 \
  --module bootstrap \
  --function claim_all_admin_capabilities \
  --args 0x10 "$bootstrap_key_id" 0x6

echo "Bootstrap complete."
