#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared helpers for scripts/bridge-bootstrap.sh.
# Source after scripts/lib/social-runtime-common.sh.
#
# Attach-only: never start/stop the user's MySo stack. Optional
# --start-anvil / --start-bridge-node spawn on non-default ports
# and do not install cleanup traps.

if [[ -n "${_BRIDGE_BOOTSTRAP_COMMON_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_BRIDGE_BOOTSTRAP_COMMON_SOURCED=1

readonly BRIDGE_OBJECT_ID='0x0000000000000000000000000000000000000000000000000000000000000009'
readonly BRIDGE_PACKAGE_ID='0x000000000000000000000000000000000000000000000000000000000000000b'
readonly BRIDGE_MYSO_CHAIN_ID=2
readonly BRIDGE_ETH_CHAIN_ID=12
readonly BRIDGE_BOOTSTRAP_NATIVE_MIST=50000000000000000
# Genesis gives ~15M MYSO per keystore address (5 × 3M coins). Sibling gather needs ≥4 others.
readonly BRIDGE_MIN_SIBLING_ADDRESSES=4
readonly BRIDGE_AUTHORITY_MIN_MIST=10000000000
readonly BRIDGE_DEFAULT_NODE_PORT=19291
readonly BRIDGE_DEFAULT_METRICS_PORT=19292
readonly BRIDGE_DEFAULT_ANVIL_PORT=18545
readonly BRIDGE_ANVIL_TEST_PK='0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80'
readonly BRIDGE_TOKEN_PRICE_BTC=500000000
readonly BRIDGE_TOKEN_PRICE_ETH=30000000
readonly BRIDGE_TOKEN_PRICE_USDC=1000
readonly BRIDGE_TOKEN_PRICE_USDT=1000
readonly BRIDGE_TOKEN_PRICE_MYSO=47000
readonly BRIDGE_EVM_TOKEN_PRICES='12800,432518900,25969600,10000,10000'

: "${BRIDGE_DIR:=$REPO_ROOT/network.config/bridge}"
: "${SOCIAL_SESSION_SAVE_PATH:=$REPO_ROOT/network.config/bridge/bridge-session.env}"
: "${MYSO_RPC_URL:=http://127.0.0.1:9000}"
: "${ETH_RPC_URL:=http://127.0.0.1:8545}"
: "${BRIDGE_AUTHORITY_URL:=}"
: "${BRIDGE_NODE_PORT:=$BRIDGE_DEFAULT_NODE_PORT}"
: "${BRIDGE_METRICS_PORT:=$BRIDGE_DEFAULT_METRICS_PORT}"
: "${BRIDGE_ANVIL_PORT:=$BRIDGE_DEFAULT_ANVIL_PORT}"
: "${BRIDGE_EVM_PRIVATE_KEY:=$BRIDGE_ANVIL_TEST_PK}"
: "${BRIDGE_COMMITTEE_WAIT_SECS:=180}"

BRIDGE_AUTHORITY_KEY_PATH="${BRIDGE_AUTHORITY_KEY_PATH:-}"
BRIDGE_CLIENT_KEY_PATH="${BRIDGE_CLIENT_KEY_PATH:-}"
BRIDGE_AUTHORITY_MYSO_ADDRESS="${BRIDGE_AUTHORITY_MYSO_ADDRESS:-}"
BRIDGE_AUTHORITY_ETH_ADDRESS="${BRIDGE_AUTHORITY_ETH_ADDRESS:-}"
BRIDGE_CLIENT_MYSO_ADDRESS="${BRIDGE_CLIENT_MYSO_ADDRESS:-}"
BRIDGE_CLIENT_ETH_ADDRESS="${BRIDGE_CLIENT_ETH_ADDRESS:-}"
BRIDGE_NODE_CONFIG_PATH="${BRIDGE_NODE_CONFIG_PATH:-}"
BRIDGE_CLIENT_CONFIG_PATH="${BRIDGE_CLIENT_CONFIG_PATH:-}"
BRIDGE_COMMITTEE_CONFIG_PATH="${BRIDGE_COMMITTEE_CONFIG_PATH:-}"
BRIDGE_DB_PATH="${BRIDGE_DB_PATH:-}"
BRIDGE_PROXY="${BRIDGE_PROXY:-}"
BRIDGE_COMMITTEE="${BRIDGE_COMMITTEE:-}"
BRIDGE_CONFIG="${BRIDGE_CONFIG:-}"
BRIDGE_LIMITER="${BRIDGE_LIMITER:-}"
BRIDGE_VAULT="${BRIDGE_VAULT:-}"
BRIDGE_WETH="${BRIDGE_WETH:-}"
BRIDGE_BTC="${BRIDGE_BTC:-}"
BRIDGE_USDC="${BRIDGE_USDC:-}"
BRIDGE_USDT="${BRIDGE_USDT:-}"
BRIDGE_KA="${BRIDGE_KA:-}"
PKG_BRIDGE_BTC="${PKG_BRIDGE_BTC:-}"
PKG_BRIDGE_ETH="${PKG_BRIDGE_ETH:-}"
PKG_BRIDGE_USDC="${PKG_BRIDGE_USDC:-}"
PKG_BRIDGE_USDT="${PKG_BRIDGE_USDT:-}"
BRIDGE_BTC_TYPE="${BRIDGE_BTC_TYPE:-}"
BRIDGE_ETH_TYPE="${BRIDGE_ETH_TYPE:-}"
BRIDGE_USDC_TYPE="${BRIDGE_USDC_TYPE:-}"
BRIDGE_USDT_TYPE="${BRIDGE_USDT_TYPE:-}"
MYSO_CHAIN_IDENTIFIER="${MYSO_CHAIN_IDENTIFIER:-}"
NATIVE_MYSO_BOOTSTRAPPED="${NATIVE_MYSO_BOOTSTRAPPED:-}"
COMMITTEE_REGISTERED="${COMMITTEE_REGISTERED:-}"
COMMITTEE_FINALIZED="${COMMITTEE_FINALIZED:-}"
TOKENS_REGISTERED_MYSO="${TOKENS_REGISTERED_MYSO:-}"
TOKENS_REGISTERED_EVM="${TOKENS_REGISTERED_EVM:-}"
COIN_CREATION_ADMIN_CAP_ID="${COIN_CREATION_ADMIN_CAP_ID:-}"
PACKAGE_PUBLISH_ADMIN_CAP_ID="${PACKAGE_PUBLISH_ADMIN_CAP_ID:-}"

BRIDGE_SESSION_KEYS=(
    MYSO_RPC_URL GRAPHQL_URL ETH_RPC_URL
    BRIDGE_DIR BRIDGE_AUTHORITY_KEY_PATH BRIDGE_CLIENT_KEY_PATH
    BRIDGE_AUTHORITY_MYSO_ADDRESS BRIDGE_AUTHORITY_ETH_ADDRESS
    BRIDGE_CLIENT_MYSO_ADDRESS BRIDGE_CLIENT_ETH_ADDRESS
    BRIDGE_NODE_PORT BRIDGE_METRICS_PORT BRIDGE_AUTHORITY_URL
    BRIDGE_NODE_CONFIG_PATH BRIDGE_CLIENT_CONFIG_PATH
    BRIDGE_COMMITTEE_CONFIG_PATH BRIDGE_DB_PATH
    BRIDGE_PROXY BRIDGE_COMMITTEE BRIDGE_CONFIG BRIDGE_LIMITER BRIDGE_VAULT
    BRIDGE_WETH BRIDGE_BTC BRIDGE_USDC BRIDGE_USDT BRIDGE_KA
    PKG_BRIDGE_BTC PKG_BRIDGE_ETH PKG_BRIDGE_MYUSD PKG_BRIDGE_USDC PKG_BRIDGE_USDT
    BRIDGE_BTC_TYPE BRIDGE_ETH_TYPE BRIDGE_MYUSD_TYPE BRIDGE_USDC_TYPE BRIDGE_USDT_TYPE
    MYSO_CHAIN_IDENTIFIER NATIVE_MYSO_BOOTSTRAPPED COMMITTEE_REGISTERED COMMITTEE_FINALIZED
    TOKENS_REGISTERED_MYSO TOKENS_REGISTERED_EVM
    COIN_CREATION_ADMIN_CAP_ID PACKAGE_PUBLISH_ADMIN_CAP_ID
)

bridge_apply_defaults() {
    [[ -n "${MYSO_RPC_URL:-}" ]] || MYSO_RPC_URL='http://127.0.0.1:9000'
    [[ -n "${GRAPHQL_URL:-}" ]] || GRAPHQL_URL='http://127.0.0.1:9125/graphql'
    [[ -n "${ETH_RPC_URL:-}" ]] || ETH_RPC_URL='http://127.0.0.1:8545'
    [[ -n "${BRIDGE_DIR:-}" ]] || BRIDGE_DIR="$REPO_ROOT/network.config/bridge"
    [[ -n "${BRIDGE_NODE_PORT:-}" ]] || BRIDGE_NODE_PORT="$BRIDGE_DEFAULT_NODE_PORT"
    [[ -n "${BRIDGE_METRICS_PORT:-}" ]] || BRIDGE_METRICS_PORT="$BRIDGE_DEFAULT_METRICS_PORT"
    [[ -n "${BRIDGE_AUTHORITY_KEY_PATH:-}" ]] || BRIDGE_AUTHORITY_KEY_PATH="$BRIDGE_DIR/bridge-authority.key"
    [[ -n "${BRIDGE_CLIENT_KEY_PATH:-}" ]] || BRIDGE_CLIENT_KEY_PATH="$BRIDGE_DIR/bridge-client.key"
    [[ -n "${BRIDGE_NODE_CONFIG_PATH:-}" ]] || BRIDGE_NODE_CONFIG_PATH="$BRIDGE_DIR/bridge-node.yaml"
    [[ -n "${BRIDGE_CLIENT_CONFIG_PATH:-}" ]] || BRIDGE_CLIENT_CONFIG_PATH="$BRIDGE_DIR/bridge-client-config.yaml"
    [[ -n "${BRIDGE_COMMITTEE_CONFIG_PATH:-}" ]] || BRIDGE_COMMITTEE_CONFIG_PATH="$BRIDGE_DIR/bridge-committee.yaml"
    [[ -n "${BRIDGE_DB_PATH:-}" ]] || BRIDGE_DB_PATH="$BRIDGE_DIR/client-db"
    [[ -n "${BRIDGE_AUTHORITY_URL:-}" ]] || BRIDGE_AUTHORITY_URL="http://127.0.0.1:${BRIDGE_NODE_PORT}"
}

bridge_save_session() {
    social_save_session "${BRIDGE_SESSION_KEYS[@]}"
}

bridge_load_session() {
    social_load_session
    bridge_apply_defaults
}

bridge_require_cmd() {
    local name="$1"
    command -v "$name" >/dev/null 2>&1 || {
        echo "Required command not on PATH: $name" >&2
        return 1
    }
}

bridge_http_ok() {
    local url="$1"
    curl -sf --connect-timeout 3 --max-time 5 "$url" >/dev/null 2>&1
}

bridge_rpc_reachable() {
    local url="${1:-$MYSO_RPC_URL}"
    [[ -n "$url" ]] || return 1
    curl -sf --connect-timeout 3 --max-time 8 -X POST "$url" \
        -H 'Content-Type: application/json' \
        -d '{"jsonrpc":"2.0","id":1,"method":"myso_getChainIdentifier","params":[]}' \
        >/dev/null 2>&1 \
        || myso client active-address >/dev/null 2>&1
}

bridge_eth_rpc_reachable() {
    local url="${1:-$ETH_RPC_URL}"
    [[ -n "$url" ]] || return 1
    curl -sf --connect-timeout 3 --max-time 5 -X POST "$url" \
        -H 'Content-Type: application/json' \
        -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}' \
        >/dev/null 2>&1
}

bridge_port_in_use() {
    local port="$1"
    if command -v lsof >/dev/null 2>&1; then
        lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1
        return $?
    fi
    nc -z 127.0.0.1 "$port" >/dev/null 2>&1
}

bridge_pick_free_port() {
    local start="${1:-9200}"
    local port="$start"
    while [[ "$port" -lt 65000 ]]; do
        if ! bridge_port_in_use "$port"; then
            printf '%s' "$port"
            return 0
        fi
        port=$((port + 1))
    done
    echo "No free TCP port found at or above $start" >&2
    return 1
}

bridge_ensure_ports() {
    if [[ "${BRIDGE_NODE_PORT:-}" == "$BRIDGE_DEFAULT_NODE_PORT" ]] && bridge_port_in_use "$BRIDGE_NODE_PORT"; then
        BRIDGE_NODE_PORT="$(bridge_pick_free_port 9200)" || return 1
        log_step "Default bridge node port busy; using $BRIDGE_NODE_PORT"
    fi
    if [[ "${BRIDGE_METRICS_PORT:-}" == "$BRIDGE_DEFAULT_METRICS_PORT" ]] && bridge_port_in_use "$BRIDGE_METRICS_PORT"; then
        BRIDGE_METRICS_PORT="$(bridge_pick_free_port $((BRIDGE_NODE_PORT + 1)))" || return 1
        log_step "Default bridge metrics port busy; using $BRIDGE_METRICS_PORT"
    fi
    BRIDGE_AUTHORITY_URL="http://127.0.0.1:${BRIDGE_NODE_PORT}"
}

bridge_myso_config_dir() {
    if [[ -n "${MYSO_CONFIG_DIR:-}" ]]; then
        printf '%s' "$MYSO_CONFIG_DIR"
        return 0
    fi
    printf '%s' "${HOME}/.myso/myso_config"
}

bridge_client_config() {
    local dir="${1:-}" client
    dir="${dir:-$(bridge_myso_config_dir)}"
    client="$dir/client.yaml"
    [[ -f "$client" ]] || return 1
    printf '%s' "$client"
}

# macOS has no GNU timeout. Kill a hung CLI (checkpoint wait) after $1 seconds.
bridge_run_with_deadline() {
    local sec="$1"
    shift
    python3 - "$sec" "$@" <<'PY'
import subprocess, sys
sec = int(sys.argv[1])
cmd = sys.argv[2:]
proc = subprocess.Popen(cmd)
try:
    sys.exit(proc.wait(timeout=sec))
except subprocess.TimeoutExpired:
    proc.terminate()
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait()
    sys.exit(124)
PY
}

bridge_myso_cli() {
    local cfg
    if cfg="$(bridge_client_config 2>/dev/null)"; then
        myso --client.config "$cfg" "$@"
    else
        myso "$@"
    fi
}

bridge_resolve_anvil() {
    local candidate
    if command -v anvil >/dev/null 2>&1; then
        command -v anvil
        return 0
    fi
    for candidate in \
        "${HOME}/.foundry/bin/anvil" \
        "${HOME}/.cargo/bin/anvil" \
        /opt/homebrew/bin/anvil; do
        if [[ -x "$candidate" ]]; then
            printf '%s' "$candidate"
            return 0
        fi
    done
    return 1
}

bridge_network_yaml() {
    local dir
    dir="$(bridge_myso_config_dir)"
    if [[ -f "$dir/network.yaml" ]]; then
        printf '%s' "$dir/network.yaml"
        return 0
    fi
    return 1
}

bridge_cli() {
    local bin=''
    if [[ -n "${REPO_ROOT:-}" ]]; then
        if [[ -x "$REPO_ROOT/target/debug/myso-bridge" ]]; then
            bin="$REPO_ROOT/target/debug/myso-bridge"
        elif [[ -x "$REPO_ROOT/target/release/myso-bridge" ]]; then
            bin="$REPO_ROOT/target/release/myso-bridge"
        fi
    fi
    if [[ -n "$bin" ]]; then
        "$bin" "$@"
        return
    fi
    if command -v myso-bridge >/dev/null 2>&1; then
        myso-bridge "$@"
        return
    fi
    echo "myso-bridge CLI not found. Build with: cargo build -p myso-bridge-cli --bin myso-bridge" >&2
    return 1
}

bridge_node_bin() {
    if command -v myso-bridge-node >/dev/null 2>&1; then
        printf '%s' 'myso-bridge-node'
        return 0
    fi
    printf '%s' ''
    return 1
}

bridge_preflight() {
    log_step "Preflight (attach-only; will not start or stop services)"
    bridge_require_cmd myso || return 1
    bridge_require_cmd jq || return 1
    bridge_require_cmd curl || return 1
    bridge_require_cmd python3 || return 1
    if ! bridge_rpc_reachable "$MYSO_RPC_URL"; then
        echo "MySo RPC is not reachable at $MYSO_RPC_URL." >&2
        echo "This script does not start localnet. Point MYSO_RPC_URL at your running fullnode." >&2
        return 1
    fi
    log_session_use "MYSO_RPC_URL" "$MYSO_RPC_URL"
    if ! object_exists_on_fullnode "$BRIDGE_OBJECT_ID"; then
        echo "Bridge object $BRIDGE_OBJECT_ID is missing. Wait for an epoch boundary after myso start (do not regenesis from this script)." >&2
        return 1
    fi
    log_session_use "BRIDGE_OBJECT_ID" "$BRIDGE_OBJECT_ID"
    if graphql_is_reachable "$GRAPHQL_URL"; then
        log_session_use "GRAPHQL_URL" "$GRAPHQL_URL"
    else
        echo "GraphQL not reachable at ${GRAPHQL_URL} — continuing with CLI discovery only." >&2
    fi
    if bridge_eth_rpc_reachable "$ETH_RPC_URL"; then
        log_session_use "ETH_RPC_URL" "$ETH_RPC_URL"
    else
        echo "Ethereum RPC not reachable at ${ETH_RPC_URL}. EVM deploy will be skipped unless you start anvil yourself." >&2
    fi
    local vals
    vals="$(bridge_live_validators 2>/dev/null || true)"
    if [[ -n "$vals" ]]; then
        log_step "Live validators:"
        printf '%s\n' "$vals" | sed 's/^/  /' >&2
    fi
}

bridge_rpc_json() {
    local method="$1"
    curl -sf --connect-timeout 3 --max-time 10 -X POST "$MYSO_RPC_URL" \
        -H 'Content-Type: application/json' \
        -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"${method}\",\"params\":[]}"
}

bridge_live_chain_id() {
    local raw
    raw="$(bridge_rpc_json myso_getChainIdentifier 2>/dev/null || true)"
    printf '%s' "$raw" | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    sys.exit(1)
r = d.get("result")
if isinstance(r, str) and r:
    print(r)
    sys.exit(0)
sys.exit(1)
'
}

bridge_client_yaml_chain_id() {
    local yaml="$1"
    [[ -f "$yaml" ]] || return 1
    python3 - "$yaml" <<'PY'
import sys
text = open(sys.argv[1], encoding="utf-8", errors="replace").read().splitlines()
chain = ""
for line in text:
    s = line.strip()
    if s.startswith("chain_id:"):
        chain = s.split(":", 1)[1].strip().strip('"').strip("'")
print(chain)
PY
}

# Prefer a repo network.yaml whose validator count matches the live chain.
# ~/.myso/myso_config/network.yaml is often a leftover 3-validator file.
bridge_repo_matching_network_yaml() {
    local live_n="$1" live_chain="${2:-}" yaml n sibling_client sibling_chain fallback=""
    [[ -n "${REPO_ROOT:-}" && -n "$live_n" ]] || return 1
    shopt -s nullglob
    for yaml in "$REPO_ROOT"/network.config/*/network.yaml "$REPO_ROOT"/network.config/network.yaml; do
        [[ -f "$yaml" ]] || continue
        n="$(bridge_network_yaml_validator_count "$yaml")"
        [[ "$n" == "$live_n" ]] || continue
        sibling_client="$(dirname "$yaml")/client.yaml"
        if [[ -n "$live_chain" && -f "$sibling_client" ]]; then
            sibling_chain="$(bridge_client_yaml_chain_id "$sibling_client" || true)"
            if [[ "$sibling_chain" == "$live_chain" ]]; then
                printf '%s' "$yaml"
                return 0
            fi
        fi
        fallback="$yaml"
    done
    if [[ -n "$fallback" ]]; then
        printf '%s' "$fallback"
        return 0
    fi
    return 1
}

bridge_live_validators() {
    bridge_rpc_json mysox_getLatestMySoSystemState | python3 -c '
import json, sys
d = json.load(sys.stdin)
r = d.get("result") or {}
vals = r.get("activeValidators") or r.get("active_validators") or []
for v in vals:
    addr = v.get("mysoAddress") or v.get("myso_address") or ""
    if addr:
        print(addr)
'
}

bridge_network_yaml_validator_count() {
    local yaml="$1"
    [[ -f "$yaml" ]] || { echo 0; return 0; }
    python3 - "$yaml" <<'PY'
import re, sys
text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
print(len(re.findall(r"(?m)^  - protocol-key-pair:", text)))
PY
}

# Read-only: never write ~/.myso. Prefer a yaml whose validator count
# matches the live committee. ~/.myso/myso_config/network.yaml is often a
# leftover 3-validator file and must not be used against 1-validator localnet.
bridge_matching_network_yaml() {
    local live_n yaml n live_chain
    live_n="$(bridge_live_validators | wc -l | tr -d ' ')"
    [[ "${live_n:-0}" -gt 0 ]] || return 1
    live_chain="$(bridge_live_chain_id || true)"
    if [[ -n "${BRIDGE_NETWORK_YAML:-}" && -f "${BRIDGE_NETWORK_YAML}" ]]; then
        n="$(bridge_network_yaml_validator_count "$BRIDGE_NETWORK_YAML")"
        if [[ "$n" == "$live_n" ]]; then
            printf '%s' "$BRIDGE_NETWORK_YAML"
            return 0
        fi
        echo "BRIDGE_NETWORK_YAML=$BRIDGE_NETWORK_YAML has $n validators; live chain has $live_n. Not using it." >&2
        return 1
    fi
    yaml="$(bridge_network_yaml || true)"
    if [[ -n "$yaml" ]]; then
        n="$(bridge_network_yaml_validator_count "$yaml")"
        if [[ "$n" == "$live_n" ]]; then
            printf '%s' "$yaml"
            return 0
        fi
        echo "Ignoring $yaml ($n validators) — live localnet has $live_n. File is read-only and left untouched." >&2
    fi
    yaml="$(bridge_repo_matching_network_yaml "$live_n" "$live_chain" || true)"
    if [[ -n "$yaml" ]]; then
        echo "Using $yaml (validator count $live_n matches live chain)." >&2
        printf '%s' "$yaml"
        return 0
    fi
    return 1
}

bridge_inner_json() {
    local wrapper inner field_id
    wrapper="$(graphql_post '{
      object(address: "0x9") {
        asMoveObject { contents { json } }
      }
    }' '{}' 2>/dev/null)" || return 1
    inner="$(printf '%s' "$wrapper" | python3 -c '
import json, sys
d = json.load(sys.stdin)
j = ((((d.get("data") or {}).get("object") or {}).get("asMoveObject") or {}).get("contents") or {}).get("json") or {}
print((j.get("inner") or {}).get("id") or "")
')" || return 1
    [[ -n "$inner" ]] || return 1
    field_id="$(myso client dynamic-field "$inner" --json 2>/dev/null | python3 -c '
import json, sys
d = json.load(sys.stdin)
fields = d.get("dynamicFields") or []
if not fields:
    sys.exit(1)
fo = fields[0].get("fieldObject") or {}
print(fo.get("objectId") or fields[0].get("fieldId") or "")
')" || return 1
    [[ -n "$field_id" ]] || return 1
    graphql_post "{
      object(address: \"${field_id}\") {
        asMoveObject { contents { json } }
      }
    }" '{}' 2>/dev/null | python3 -c '
import json, sys
d = json.load(sys.stdin)
j = ((((d.get("data") or {}).get("object") or {}).get("asMoveObject") or {}).get("contents") or {}).get("json") or {}
print(json.dumps(j.get("value") or j))
'
}

bridge_live_chain_identifier() {
    local gql rpc
    gql="$(bridge_graphql_chain_identifier 2>/dev/null || true)"
    [[ -n "$gql" ]] && {
        printf '%s' "$gql"
        return 0
    }
    rpc="$(bridge_rpc_json myso_getChainIdentifier 2>/dev/null \
        | jq -r '.result // empty' 2>/dev/null || true)"
    [[ -n "$rpc" ]] && printf '%s' "$rpc"
}

bridge_graphql_chain_identifier() {
    local json
    graphql_is_reachable "$GRAPHQL_URL" || return 1
    json="$(graphql_post '{ chainIdentifier }' '{}' 2>/dev/null)" || return 1
    printf '%s' "$(printf '%s' "$json" | jq -r '.data.chainIdentifier // empty' 2>/dev/null)"
}

bridge_committee_total_voting_power() {
    local inner
    inner="$(bridge_inner_json 2>/dev/null)" || return 1
    printf '%s' "$inner" | python3 -c '
import json, sys
d = json.load(sys.stdin)
members = (((d.get("committee") or {}).get("members") or {}).get("contents") or [])
total = 0
for entry in members:
    if not isinstance(entry, dict):
        continue
    v = entry.get("value") if isinstance(entry.get("value"), dict) else entry
    total += int(v.get("voting_power") or 0)
print(total)
'
}

bridge_committee_node_ready() {
    local total min=7500
    bridge_committee_finalized || return 1
    total="$(bridge_committee_total_voting_power 2>/dev/null || echo 0)"
    [[ "${total:-0}" -ge "$min" ]]
}

bridge_normalize_type_name() {
    python3 - "$1" <<'PY'
import sys
raw = sys.argv[1].strip()
if not raw:
    sys.exit(1)
body = raw[2:] if raw.startswith("0x") else raw
parts = body.split("::")
if len(parts) < 3:
    print(raw)
    raise SystemExit(0)
print("0x" + parts[0].lower().zfill(64) + "::" + "::".join(parts[1:]))
PY
}

bridge_supported_token_types_from_rpc() {
    local raw
    raw="$(bridge_rpc_json mysox_getLatestBridge 2>/dev/null || true)"
    printf '%s' "$raw" | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    sys.exit(0)
t = (d.get("result") or {}).get("treasury") or {}
rows = t.get("supportedTokens") or t.get("supported_tokens") or []
for row in rows:
    if isinstance(row, (list, tuple)) and row:
        print(row[0])
    elif isinstance(row, dict):
        typ = row.get("type") or row.get("tokenType") or row.get("token_type") or ""
        if typ:
            print(typ)
'
}

bridge_supported_token_ids_from_rpc() {
    local raw
    raw="$(bridge_rpc_json mysox_getLatestBridge 2>/dev/null || true)"
    printf '%s' "$raw" | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    sys.exit(0)
t = (d.get("result") or {}).get("treasury") or {}
rows = t.get("idTokenTypeMap") or t.get("id_token_type_map") or []
for row in rows:
    if isinstance(row, (list, tuple)) and row:
        print(row[0])
    elif isinstance(row, dict):
        token_id = row.get("id") or row.get("tokenId") or row.get("token_id")
        if token_id is not None:
            print(token_id)
'
}

# True when every non-empty argument type is already in supported_tokens.
bridge_types_supported_on_chain() {
    local have want n have_n
    [[ $# -gt 0 ]] || return 1
    have="$(bridge_supported_token_types_from_rpc)"
    have_n="$(printf '%s\n' "$have" | python3 -c '
import sys
def norm(raw):
    raw = raw.strip()
    if not raw:
        return ""
    body = raw[2:] if raw.startswith("0x") else raw
    parts = body.split("::")
    if len(parts) < 3:
        return raw.lower()
    return "0x" + parts[0].lower().zfill(64) + "::" + "::".join(parts[1:])
print("\n".join(norm(line) for line in sys.stdin if line.strip()))
')"
    for want in "$@"; do
        [[ -n "$want" ]] || return 1
        n="$(bridge_normalize_type_name "$want")" || return 1
        printf '%s\n' "$have_n" | grep -Fxq "$n" || return 1
    done
    return 0
}

bridge_foreign_tokens_supported_on_chain() {
    local rail_type="${BRIDGE_MYUSD_TYPE:-${BRIDGE_USDC_TYPE:-}}"
    local have_ids
    [[ -n "${BRIDGE_BTC_TYPE:-}" && -n "${BRIDGE_ETH_TYPE:-}" && -n "$rail_type" ]] || return 1
    BRIDGE_USDC_TYPE="$rail_type"
    BRIDGE_USDT_TYPE="$rail_type"
    bridge_types_supported_on_chain \
        "$BRIDGE_BTC_TYPE" "$BRIDGE_ETH_TYPE" "$rail_type" || return 1
    have_ids="$(bridge_supported_token_ids_from_rpc)"
    for want in 1 2 3 4; do
        printf '%s\n' "$have_ids" | grep -Fxq "$want" || return 1
    done
    return 0
}

bridge_find_object_by_type() {
    local move_type="$1" owner="${2:-}" query vars json addr
    graphql_is_reachable "$GRAPHQL_URL" || return 1
    if [[ -n "$owner" ]]; then
        owner="$(normalize_hex_id "$owner")" || return 1
        query='query Obj($owner: MySoAddress!, $typ: String!) {
          objects(filter: { type: $typ, ownerKind: ADDRESS, owner: $owner }, last: 5) {
            nodes { address }
          }
        }'
        vars="$(jq -nc --arg owner "$owner" --arg typ "$move_type" '{owner: $owner, typ: $typ}')"
    else
        query='query Obj($typ: String!) {
          objects(filter: { type: $typ }, last: 5) {
            nodes { address }
          }
        }'
        vars="$(jq -nc --arg typ "$move_type" '{typ: $typ}')"
    fi
    json="$(graphql_post "$query" "$vars" 2>/dev/null)" || return 1
    addr="$(printf '%s' "$json" | jq -r '.data.objects.nodes[0].address // empty')"
    [[ -n "$addr" ]] || return 1
    normalize_hex_id "$addr"
}

bridge_register_existing_foreign_token() {
    local type_name="$1" pkg active tc md uc out rc=0
    [[ -n "$type_name" ]] || return 1
    pkg="${type_name%%::*}"
    active="$(resolve_myso_active_address)" || return 1
    tc="$(bridge_find_object_by_type "0x2::coin::TreasuryCap<${type_name}>" "$active" || true)"
    [[ -n "$tc" ]] || tc="$(bridge_find_object_by_type "0x2::coin::TreasuryCap<${type_name}>" || true)"
    md="$(bridge_find_object_by_type "0x2::coin::CoinMetadata<${type_name}>" || true)"
    uc="$(bridge_resolve_upgrade_cap_for_package "$pkg" "$active" 2>/dev/null || true)"
    [[ -n "$tc" && -n "$md" && -n "$uc" ]] || {
        echo "Cannot re-register $type_name (treasury=$tc metadata=$md upgrade=$uc)" >&2
        return 1
    }
    log_step "register_foreign_token $type_name (existing caps)"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
        --move-call "${BRIDGE_PACKAGE_ID}::bridge::register_foreign_token" "<${type_name}>" \
        "@${BRIDGE_OBJECT_ID}" \
        "@$(normalize_hex_id "$tc")" \
        "@$(normalize_hex_id "$uc")" \
        "@$(normalize_hex_id "$md")")" || rc=$?
    if assert_tx_success "$out"; then
        return 0
    fi
    if echo "$out" | grep -qiE 'already|EUnsupportedTokenType|waiting|ETokenSupplyNonZero'; then
        log_step "register_foreign_token $type_name already applied or waiting"
        return 0
    fi
    return "${rc:-1}"
}

bridge_ensure_waiting_room_tokens() {
    local kind type_name
    for kind in btc eth myusd; do
        case "$kind" in
            btc) type_name="${BRIDGE_BTC_TYPE:-}" ;;
            eth) type_name="${BRIDGE_ETH_TYPE:-}" ;;
            myusd) type_name="${BRIDGE_MYUSD_TYPE:-${BRIDGE_USDC_TYPE:-}}" ;;
        esac
        [[ -n "$type_name" ]] || continue
        if bridge_types_supported_on_chain "$type_name"; then
            continue
        fi
        bridge_register_existing_foreign_token "$type_name" || true
    done
}

bridge_canonical_type_no_0x() {
    python3 - "$1" <<'PY'
import sys
raw = sys.argv[1].strip()
body = raw[2:] if raw.startswith("0x") else raw
parts = body.split("::")
if len(parts) < 3:
    print(body)
    raise SystemExit(0)
print(parts[0].lower().zfill(64) + "::" + "::".join(parts[1:]))
PY
}

bridge_node_sign_add_tokens_on_myso() {
    local types prices url
    types="$(printf '%s,%s,%s,%s' \
        "$(bridge_normalize_type_name "$BRIDGE_BTC_TYPE")" \
        "$(bridge_normalize_type_name "$BRIDGE_ETH_TYPE")" \
        "$(bridge_normalize_type_name "$BRIDGE_USDC_TYPE")" \
        "$(bridge_normalize_type_name "$BRIDGE_USDT_TYPE")")"
    prices="${BRIDGE_TOKEN_PRICE_BTC},${BRIDGE_TOKEN_PRICE_ETH},${BRIDGE_TOKEN_PRICE_USDC},${BRIDGE_TOKEN_PRICE_USDT}"
    url="${BRIDGE_AUTHORITY_URL%/}/sign/add_tokens_on_myso/${BRIDGE_MYSO_CHAIN_ID}/0/0/1,2,3,4/${types}/${prices}"
    curl -sf --connect-timeout 3 --max-time 15 "$url"
}

bridge_add_tokens_on_myso_via_node() {
    local signed sig_vec names_csv active out
    [[ -n "${BRIDGE_BTC_TYPE:-}" ]] || return 1
    if ! bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        return 1
    fi
    signed="$(bridge_node_sign_add_tokens_on_myso)" || {
        echo "Node did not sign add_tokens_on_myso" >&2
        return 1
    }
    sig_vec="$(printf '%s' "$signed" | python3 -c '
import json, sys, base64
d = json.load(sys.stdin)
sig = ((d.get("auth_signature") or {}).get("signature") or "")
raw = base64.b64decode(sig)
print("vector[vector[" + ",".join(f"{b}u8" for b in raw) + "]]")
')"
    [[ -n "$sig_vec" && "$sig_vec" != "vector[vector[]]" ]] || {
        echo "Could not decode add_tokens_on_myso signature" >&2
        return 1
    }
    names_csv="$(python3 - "$BRIDGE_BTC_TYPE" "$BRIDGE_ETH_TYPE" "$BRIDGE_USDC_TYPE" "$BRIDGE_USDT_TYPE" <<'PY'
import sys
out = []
for raw in sys.argv[1:]:
    body = raw[2:] if raw.startswith("0x") else raw
    parts = body.split("::")
    out.append(parts[0].lower().zfill(64) + "::" + "::".join(parts[1:]))
print(",".join(out))
PY
)"
    active="$(resolve_myso_active_address)" || return 1
    log_step "execute_system_message add_tokens_on_myso via node signature"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
        --move-call "${BRIDGE_PACKAGE_ID}::message::create_add_tokens_on_myso_message" \
        "${BRIDGE_MYSO_CHAIN_ID}u8" "0u64" false \
        "vector[1u8,2u8,3u8,4u8]" \
        "$(literal_move_vector_from_csv "$names_csv")" \
        "vector[${BRIDGE_TOKEN_PRICE_BTC}u64,${BRIDGE_TOKEN_PRICE_ETH}u64,${BRIDGE_TOKEN_PRICE_USDC}u64,${BRIDGE_TOKEN_PRICE_USDT}u64]" \
        --assign add_tokens_msg \
        --move-call "${BRIDGE_PACKAGE_ID}::bridge::execute_system_message" \
        "@${BRIDGE_OBJECT_ID}" add_tokens_msg "$sig_vec")" || true
    assert_tx_success "$out"
}

bridge_clear_on_chain_session_flags() {
    NATIVE_MYSO_BOOTSTRAPPED=''
    COMMITTEE_REGISTERED=''
    COMMITTEE_FINALIZED=''
    TOKENS_REGISTERED_MYSO=''
    TOKENS_REGISTERED_EVM=''
    BRIDGE_PROXY=''
    BRIDGE_COMMITTEE=''
    BRIDGE_CONFIG=''
    BRIDGE_LIMITER=''
    BRIDGE_VAULT=''
    BRIDGE_WETH=''
    BRIDGE_BTC=''
    BRIDGE_USDC=''
    BRIDGE_USDT=''
    BRIDGE_KA=''
    PKG_BRIDGE_BTC=''
    PKG_BRIDGE_ETH=''
    PKG_BRIDGE_USDC=''
    PKG_BRIDGE_USDT=''
    BRIDGE_BTC_TYPE=''
    BRIDGE_ETH_TYPE=''
    BRIDGE_USDC_TYPE=''
    BRIDGE_USDT_TYPE=''
    COIN_CREATION_ADMIN_CAP_ID=''
    PACKAGE_PUBLISH_ADMIN_CAP_ID=''
    BRIDGE_CAP_OWNER=''
}

bridge_sync_session_with_chain() {
    local live="${1:-$(bridge_live_chain_identifier)}"
    [[ -n "$live" ]] || return 0
    if [[ -n "${MYSO_CHAIN_IDENTIFIER:-}" && "$MYSO_CHAIN_IDENTIFIER" != "$live" ]]; then
        log_step "Chain identifier changed ($MYSO_CHAIN_IDENTIFIER -> $live); clearing stale bridge session flags"
        bridge_clear_on_chain_session_flags
    fi
    MYSO_CHAIN_IDENTIFIER="$live"
}

bridge_reset_fresh_chain_state() {
    log_step "Fresh chain reset: removing bridge session and client-db (keeps bridge keys)"
    rm -f "$SOCIAL_SESSION_SAVE_PATH"
    rm -rf "${BRIDGE_DB_PATH:-$BRIDGE_DIR/client-db}"
    bridge_clear_on_chain_session_flags
    MYSO_CHAIN_IDENTIFIER=''
}

bridge_native_ready() {
    local inner ready=1
    inner="$(bridge_inner_json 2>/dev/null)" || ready=0
    if [[ "$ready" == 1 ]] && printf '%s' "$inner" | python3 -c '
import json, sys
d = json.load(sys.stdin)
t = d.get("treasury") or {}
v = t.get("native_bridge_initialized")
sys.exit(0 if v in (True, "true", 1, "1") else 1)
'; then
        NATIVE_MYSO_BOOTSTRAPPED=1
        return 0
    fi
    if [[ "${NATIVE_MYSO_BOOTSTRAPPED:-}" == 1 ]]; then
        log_step "Session had NATIVE_MYSO_BOOTSTRAPPED=1 but chain is not initialized; retrying bootstrap"
        NATIVE_MYSO_BOOTSTRAPPED=''
    fi
    return 1
}

bridge_pick_largest_myso_coin() {
    local addr="$1" json
    addr="$(normalize_hex_id "$addr")" || return 1
    json="$(resolve_gas_coins_json_for_address "$addr")" || return 1
    echo "$json" | jq -r '
        def oid: (.gasCoinId // .coinObjectId);
        max_by(.mistBalance | tonumber)
        | "\(oid) \(.mistBalance)"
    '
}

bridge_list_keystore_addresses() {
    bridge_myso_cli client addresses --json 2>/dev/null \
        | jq -r '.addresses[]? | if type == "array" then .[1] else .address // .mysoAddress // empty end'
}

bridge_merge_all_myso_coins() {
    local addr="$1" before after i
    addr="$(normalize_hex_id "$addr")" || return 1
    for i in $(seq 1 200); do
        before="$(resolve_gas_coins_json_for_address "$addr" 2>/dev/null | jq 'length' 2>/dev/null || echo 0)"
        [[ "${before:-0}" -le 1 ]] && return 0
        bridge_merge_myso_coins_to_largest "$addr" || break
        after="$(resolve_gas_coins_json_for_address "$addr" 2>/dev/null | jq 'length' 2>/dev/null || echo "$before")"
        [[ "${after:-0}" -ge "${before:-0}" ]] && break
    done
}

bridge_merge_until_largest_at_least() {
    local addr="$1" need="$2" i before after largest
    addr="$(normalize_hex_id "$addr")" || return 1
    for i in $(seq 1 200); do
        largest="$(resolve_max_coin_balance "$addr")"
        [[ "${largest:-0}" -ge "$need" ]] && return 0
        before="$(resolve_gas_coins_json_for_address "$addr" 2>/dev/null | jq 'length' 2>/dev/null || echo 0)"
        [[ "${before:-0}" -le 1 ]] && break
        bridge_merge_myso_coins_to_largest "$addr" || break
        after="$(resolve_gas_coins_json_for_address "$addr" 2>/dev/null | jq 'length' 2>/dev/null || echo "$before")"
        [[ "${after:-0}" -ge "${before:-0}" ]] && break
    done
    largest="$(resolve_max_coin_balance "$addr")"
    [[ "${largest:-0}" -ge "$need" ]]
}

bridge_transfer_donor_coins_to_active() {
    local donor="$1" active="$2"
    local json gas_coin coin_ids coin_id out coin_count
    donor="$(normalize_hex_id "$donor")" || return 1
    active="$(normalize_hex_id "$active")" || return 1
    switch_wallet "$donor" || return 1
    bridge_merge_all_myso_coins "$donor" || true
    json="$(resolve_gas_coins_json_for_address "$donor")" || {
        restore_wallet
        return 1
    }
    coin_count="$(echo "$json" | jq 'length')"
    [[ "${coin_count:-0}" -gt 0 ]] || {
        restore_wallet
        return 0
    }
    gas_coin="$(echo "$json" | jq -r 'min_by(.mistBalance | tonumber) | .gasCoinId // .coinObjectId // empty')"
    if [[ "${coin_count:-0}" -le 1 ]]; then
        coin_id="$(echo "$json" | jq -r '.[0] | .gasCoinId // .coinObjectId')"
        log_step "Transferring consolidated coin $(normalize_hex_id "$coin_id") from $donor to $active"
        if bridge_myso_cli client transfer-myso --to "$active" --myso-coin-object-id "$(normalize_hex_id "$coin_id")" >/dev/null 2>&1 \
            || bridge_myso_cli client transfer-myso --to "$active" --myso-coin-object-id "$(normalize_hex_id "$coin_id")" >&2; then
            restore_wallet
            return 0
        fi
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$donor" \
            --transfer-objects "[@$(normalize_hex_id "$coin_id")]" "@${active}")" || true
        assert_tx_success "$out" || true
        restore_wallet
        return 0
    fi
    coin_ids="$(echo "$json" | jq -r 'sort_by(.mistBalance | tonumber) | reverse | .[] | .gasCoinId // .coinObjectId')"
    while IFS= read -r coin_id; do
        [[ -n "$coin_id" ]] || continue
        [[ "$(normalize_hex_id "$coin_id")" == "$(normalize_hex_id "$gas_coin")" ]] && continue
        log_step "Transferring coin $(normalize_hex_id "$coin_id") from $donor to $active"
        PTB_GAS_COIN_ID="$gas_coin" out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$donor" \
            --transfer-objects "[@$(normalize_hex_id "$coin_id")]" "@${active}")" || true
        PTB_GAS_COIN_ID=''
        assert_tx_success "$out" || true
    done <<< "$coin_ids"
    restore_wallet
}

bridge_count_keystore_siblings() {
    local active="$1" count=0 addr
    active="$(normalize_hex_id "$active")" || return 1
    while IFS= read -r addr; do
        [[ -z "$addr" ]] && continue
        addr="$(normalize_hex_id "$addr" 2>/dev/null)" || continue
        [[ "$addr" == "$active" ]] && continue
        count=$((count + 1))
    done < <(bridge_list_keystore_addresses)
    printf '%s' "$count"
}

bridge_gather_myso_from_keystore_siblings() {
    local active need addr bal before after siblings
    active="$(resolve_myso_active_address)" || return 1
    active="$(normalize_hex_id "$active")" || return 1
    need=$((BRIDGE_BOOTSTRAP_NATIVE_MIST + GAS_BUDGET))
    [[ "$(resolve_total_coin_balance "$active")" -ge "$need" ]] \
        && [[ "$(resolve_max_coin_balance "$active")" -ge "$need" ]] && return 0
    siblings="$(bridge_count_keystore_siblings "$active")"
    if [[ "${siblings:-0}" -lt "$BRIDGE_MIN_SIBLING_ADDRESSES" ]]; then
        echo "Need ≥${BRIDGE_MIN_SIBLING_ADDRESSES} other keystore addresses to gather ~50M MYSO (have ${siblings:-0})." >&2
        echo "Re-run myso genesis --force without wiping client.yaml/myso.keystore so all genesis accounts stay in the keystore." >&2
        return 1
    fi
    log_step "Gathering MYSO from ${siblings} sibling keystore addresses (native MYSO cannot be mint_and_transfer'd)"
    while IFS= read -r addr; do
        [[ -z "$addr" ]] && continue
        addr="$(normalize_hex_id "$addr" 2>/dev/null)" || continue
        [[ "$addr" == "$active" ]] && continue
        bal="$(resolve_total_coin_balance "$addr" 2>/dev/null || echo 0)"
        [[ "${bal:-0}" -le 1000000000000 ]] && continue
        bridge_transfer_donor_coins_to_active "$addr" "$active" || true
        [[ "$(resolve_total_coin_balance "$active")" -ge "$need" ]] && break
    done < <(bridge_list_keystore_addresses)
    bridge_merge_until_largest_at_least "$active" "$need" || true
}

bridge_merge_myso_coins_to_largest() {
    local active="$1" json dest_id merge_list out sources coin_count
    active="$(normalize_hex_id "$active")" || return 1
    json="$(resolve_gas_coins_json_for_address "$active")" || return 1
    coin_count="$(echo "$json" | jq 'length')"
    [[ "${coin_count:-0}" -le 1 ]] && return 0

    dest_id="$(echo "$json" | jq -r 'max_by(.mistBalance | tonumber) | .gasCoinId // .coinObjectId')"
    # Dest cannot be both --gas-coin and --merge-coins dest. Pay gas with the
    # largest coin and merge the rest into `gas`.
    sources="$(echo "$json" | jq -r --arg d "$dest_id" \
        '.[] | select((.gasCoinId // .coinObjectId) != $d) | "@\(.gasCoinId // .coinObjectId)"' | head -n 50)"
    [[ -n "$sources" ]] || return 0
    merge_list="[$(echo "$sources" | awk 'NF {printf "%s%s", (n++?", ":""), $0}')]"
    log_step "Merging MYSO coins into $(normalize_hex_id "$dest_id") (dest pays gas; merge into gas)"
    PTB_GAS_COIN_ID="$dest_id"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
        --merge-coins gas "$merge_list")" || true
    PTB_GAS_COIN_ID=''
    assert_tx_success "$out"
}

bridge_ensure_funding_for_native_bootstrap() {
    local active need total largest tap max_taps
    need=$((BRIDGE_BOOTSTRAP_NATIVE_MIST + GAS_BUDGET))
    max_taps="${BRIDGE_FAUCET_ATTEMPTS:-3}"
    active="$(resolve_myso_active_address)" || return 1
    bridge_gather_myso_from_keystore_siblings || true

    total="$(resolve_total_coin_balance "$active")"
    largest="$(resolve_max_coin_balance "$active")"
    if [[ "${total:-0}" -ge "$need" && "${largest:-0}" -ge "$need" ]]; then
        return 0
    fi

    if [[ "${total:-0}" -lt "$need" ]]; then
        for tap in $(seq 1 "$max_taps"); do
            log_step "Faucet top-up ($tap/$max_taps): total=${total:-0} largest=${largest:-0} need=$need"
            bridge_myso_cli client faucet --address "$active" >/dev/null 2>&1 \
                || bridge_myso_cli client faucet --address "$active" >&2 \
                || true
            [[ "$tap" -lt "$max_taps" ]] && sleep 1
            total="$(resolve_total_coin_balance "$active")"
            largest="$(resolve_max_coin_balance "$active")"
            [[ "${total:-0}" -ge "$need" ]] && break
        done
    else
        log_step "Total MYSO sufficient (${total:-0} mist) but largest coin is ${largest:-0}; merging (skipping faucet)"
    fi

    total="$(resolve_total_coin_balance "$active")"
    if [[ "${total:-0}" -lt "$need" ]]; then
        echo "Wallet $active has total=${total:-0} mist after gather/faucet; need $need for native bootstrap." >&2
        echo "Ensure myso genesis --force kept the existing keystore (≥5 addresses, ~15M MYSO each)." >&2
        return 1
    fi

    if ! bridge_merge_until_largest_at_least "$active" "$need"; then
        largest="$(resolve_max_coin_balance "$active")"
        echo "Largest MYSO coin still below $need mist after merge (largest=${largest:-0}, total=${total:-0})." >&2
        return 1
    fi
}

bridge_bootstrap_native_myso() {
    local active coin_id bal out digest new_coin pair
    if bridge_native_ready; then
        log_step "Native MYSO already bootstrapped on $BRIDGE_OBJECT_ID"
        NATIVE_MYSO_BOOTSTRAPPED=1
        return 0
    fi
    active="$(resolve_myso_active_address)" || {
        echo "No active myso address" >&2
        return 1
    }
    if ! bridge_ensure_funding_for_native_bootstrap; then
        echo "Could not fund active wallet for native MYSO bootstrap." >&2
        return 1
    fi
    pair="$(bridge_pick_largest_myso_coin "$active")" || {
        echo "Could not list MYSO coins for $active" >&2
        return 1
    }
    read -r coin_id bal <<<"$pair"
    new_coin="$(echo "$(resolve_gas_coins_json_for_address "$active")" | jq -r --argjson want "$BRIDGE_BOOTSTRAP_NATIVE_MIST" '
        def oid: (.gasCoinId // .coinObjectId);
        .[] | select((.mistBalance | tonumber) == $want) | oid
    ' | head -n1)"
    if [[ -z "$new_coin" ]]; then
        if [[ "${bal:-0}" -lt $((BRIDGE_BOOTSTRAP_NATIVE_MIST + GAS_BUDGET)) ]]; then
            echo "Skipping native MYSO bootstrap: $active has $bal mist, need ${BRIDGE_BOOTSTRAP_NATIVE_MIST} + gas." >&2
            return 0
        fi
        # One large coin cannot be both --split-coins dest and an implicit other
        # gas object. Pay gas with it and split from `gas`.
        log_step "Splitting ${BRIDGE_BOOTSTRAP_NATIVE_MIST} mist from $coin_id (source pays gas; split from gas)"
        PTB_GAS_COIN_ID="$coin_id"
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
            --split-coins gas "[${BRIDGE_BOOTSTRAP_NATIVE_MIST}]" \
            --assign bootstrap_coin \
            --transfer-objects "[bootstrap_coin]" "@${active}")" || true
        PTB_GAS_COIN_ID=''
        if ! assert_tx_success "$out"; then
            echo "split-coins for native MYSO bootstrap failed" >&2
            return 1
        fi
        digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
        new_coin="$(echo "$(resolve_gas_coins_json_for_address "$active")" | jq -r --argjson want "$BRIDGE_BOOTSTRAP_NATIVE_MIST" '
            def oid: (.gasCoinId // .coinObjectId);
            .[] | select((.mistBalance | tonumber) == $want) | oid
        ' | head -n1)"
        if [[ -z "$new_coin" && -n "$digest" ]]; then
            new_coin="$(extract_created_object_by_type "$digest" "Coin" 2>/dev/null || true)"
        fi
    else
        log_step "Reusing existing ${BRIDGE_BOOTSTRAP_NATIVE_MIST} mist coin $new_coin"
        out=''
        digest=''
    fi
    [[ -n "$new_coin" ]] || {
        echo "Could not find the 50M MYSO coin after split" >&2
        return 1
    }
    log_step "Calling 0xb::bridge::bootstrap_native_myso"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
        --move-call "${BRIDGE_PACKAGE_ID}::bridge::bootstrap_native_myso" \
        "@${BRIDGE_OBJECT_ID}" "@$(normalize_hex_id "$new_coin")")" || true
    if assert_tx_success "$out"; then
        NATIVE_MYSO_BOOTSTRAPPED=1
        return 0
    fi
    if echo "$out" | grep -qiE 'ENativeBridgeAlreadyInitialized|already initialized|native_bridge'; then
        log_step "Native MYSO bootstrap already completed on-chain"
        NATIVE_MYSO_BOOTSTRAPPED=1
        return 0
    fi
    echo "bootstrap_native_myso failed" >&2
    return 1
}

bridge_parse_examine_key() {
    local out="$1"
    local eth myso
    eth="$(printf '%s' "$out" | sed -n 's/.*Corresponding Ethereum address: *//p' | head -n1 | tr -d '[:space:]')"
    myso="$(printf '%s' "$out" | sed -n 's/.*Corresponding MySo address: *//p' | head -n1 | tr -d '[:space:]')"
    [[ -n "$eth" && -n "$myso" ]] || return 1
    printf '%s %s' "$eth" "$myso"
}

bridge_ensure_keys() {
    local refresh="${1:-0}" out pair
    mkdir -p "$BRIDGE_DIR"
    if [[ "$refresh" == 1 ]]; then
        if [[ -f "$BRIDGE_AUTHORITY_KEY_PATH" || -f "$BRIDGE_CLIENT_KEY_PATH" ]]; then
            if ! confirm_run; then
                echo "Refusing to overwrite existing bridge keys without confirmation." >&2
                return 1
            fi
        fi
        rm -f "$BRIDGE_AUTHORITY_KEY_PATH" "$BRIDGE_CLIENT_KEY_PATH"
    fi
    if [[ ! -f "$BRIDGE_AUTHORITY_KEY_PATH" ]]; then
        log_step "Creating bridge authority key at $BRIDGE_AUTHORITY_KEY_PATH"
        bridge_require_cmd myso-bridge || return 1
        bridge_cli create-bridge-validator-key "$BRIDGE_AUTHORITY_KEY_PATH" >/dev/null
    else
        log_step "Reusing authority key $BRIDGE_AUTHORITY_KEY_PATH"
    fi
    if [[ ! -f "$BRIDGE_CLIENT_KEY_PATH" ]]; then
        log_step "Creating bridge client ecdsa key at $BRIDGE_CLIENT_KEY_PATH"
        bridge_cli create-bridge-client-key "$BRIDGE_CLIENT_KEY_PATH" --use-ecdsa >/dev/null
    else
        log_step "Reusing client key $BRIDGE_CLIENT_KEY_PATH"
    fi
    out="$(bridge_cli examine-key "$BRIDGE_AUTHORITY_KEY_PATH" --is-validator-key 2>&1)" || {
        echo "$out" >&2
        echo "examine-key failed for authority key" >&2
        return 1
    }
    pair="$(bridge_parse_examine_key "$out")" || {
        echo "Could not parse authority examine-key output:" >&2
        echo "$out" >&2
        return 1
    }
    read -r BRIDGE_AUTHORITY_ETH_ADDRESS BRIDGE_AUTHORITY_MYSO_ADDRESS <<<"$pair"
    BRIDGE_AUTHORITY_MYSO_ADDRESS="$(normalize_hex_id "$BRIDGE_AUTHORITY_MYSO_ADDRESS")"
    out="$(bridge_cli examine-key "$BRIDGE_CLIENT_KEY_PATH" --is-validator-key 2>&1)" || {
        echo "$out" >&2
        echo "examine-key failed for client key" >&2
        return 1
    }
    pair="$(bridge_parse_examine_key "$out")" || {
        echo "Could not parse client examine-key output:" >&2
        echo "$out" >&2
        return 1
    }
    read -r BRIDGE_CLIENT_ETH_ADDRESS BRIDGE_CLIENT_MYSO_ADDRESS <<<"$pair"
    BRIDGE_CLIENT_MYSO_ADDRESS="$(normalize_hex_id "$BRIDGE_CLIENT_MYSO_ADDRESS")"
    log_session_use "BRIDGE_AUTHORITY_MYSO_ADDRESS" "$BRIDGE_AUTHORITY_MYSO_ADDRESS"
    log_session_use "BRIDGE_AUTHORITY_ETH_ADDRESS" "$BRIDGE_AUTHORITY_ETH_ADDRESS"
    log_session_use "BRIDGE_CLIENT_MYSO_ADDRESS" "$BRIDGE_CLIENT_MYSO_ADDRESS"
    log_session_use "BRIDGE_CLIENT_ETH_ADDRESS" "$BRIDGE_CLIENT_ETH_ADDRESS"
}

bridge_pad_type_address() {
    python3 - "$1" <<'PY'
import sys
s = sys.argv[1]
if s.startswith("0x"):
    body = s[2:]
else:
    body = s
print("0x" + body.zfill(64))
PY
}

bridge_yaml_token_struct() {
    local type_name="$1"
    python3 - "$type_name" <<'PY'
import sys
t = sys.argv[1]
addr, module, name = t.split("::")
body = addr[2:] if addr.startswith("0x") else addr
print(f'''        - Struct:
            address: "0x{body.zfill(64)}"
            module: "{module}"
            name: "{name}"
            type_params: []''')
PY
}

bridge_write_configs() {
    local proxy="${BRIDGE_PROXY:-0x0000000000000000000000000000000000000000}"
    local evm0 evm1 evm2 evm3 evm4
    evm0="${BRIDGE_PROXY:+0x0000000000000000000000000000000000000000}"
    evm0="${evm0:-0x0000000000000000000000000000000000000000}"
    evm1="${BRIDGE_BTC:-0x0000000000000000000000000000000000000000}"
    evm2="${BRIDGE_WETH:-0x0000000000000000000000000000000000000000}"
    evm3="${BRIDGE_USDC:-0x0000000000000000000000000000000000000000}"
    evm4="${BRIDGE_USDT:-0x0000000000000000000000000000000000000000}"
    mkdir -p "$BRIDGE_DIR" "$BRIDGE_DB_PATH"

    local myso_types=""
    local rail_type="${BRIDGE_MYUSD_TYPE:-${BRIDGE_USDC_TYPE:-}}"
    if [[ -n "${BRIDGE_BTC_TYPE:-}" && -n "${BRIDGE_ETH_TYPE:-}" && -n "$rail_type" ]]; then
        myso_types="$(cat <<EOF
  - AddTokensOnMySoAction:
      nonce: 0
      chain_id: MySoCustom
      native: false
      token_ids: [1, 2, 3, 4]
      token_type_names:
$(bridge_yaml_token_struct "$BRIDGE_BTC_TYPE")
$(bridge_yaml_token_struct "$BRIDGE_ETH_TYPE")
$(bridge_yaml_token_struct "$rail_type")
$(bridge_yaml_token_struct "$rail_type")
      token_prices: [${BRIDGE_TOKEN_PRICE_BTC}, ${BRIDGE_TOKEN_PRICE_ETH}, ${BRIDGE_TOKEN_PRICE_USDC}, ${BRIDGE_TOKEN_PRICE_USDT}]
EOF
)"
    else
        myso_types="$(cat <<EOF
  - AddTokensOnMySoAction:
      nonce: 0
      chain_id: MySoCustom
      native: true
      token_ids: [0]
      token_type_names:
$(bridge_yaml_token_struct "0x2::myso::MYSO")
      token_prices: [${BRIDGE_TOKEN_PRICE_MYSO}]
EOF
)"
    fi

    cat > "$BRIDGE_NODE_CONFIG_PATH" <<EOF
server-listen-port: ${BRIDGE_NODE_PORT}
metrics-port: ${BRIDGE_METRICS_PORT}
bridge-authority-key-path: ${BRIDGE_AUTHORITY_KEY_PATH}
run-client: true
db-path: ${BRIDGE_DB_PATH}
approved-governance-actions:
${myso_types}
  - AddTokensOnEvmAction:
      nonce: 0
      chain_id: EthCustom
      native: true
      token_ids: [0, 1, 2, 3, 4]
      token_addresses: ["${evm0}", "${evm1}", "${evm2}", "${evm3}", "${evm4}"]
      token_myso_decimals: [9, 8, 8, 6, 6]
      token_prices: [12800, 432518900, 25969600, 10000, 10000]
myso:
  myso-rpc-url: "${MYSO_RPC_URL}"
  myso-bridge-chain-id: ${BRIDGE_MYSO_CHAIN_ID}
  bridge-client-key-path: ${BRIDGE_CLIENT_KEY_PATH}
eth:
  eth-rpc-urls:
    - "${ETH_RPC_URL}"
  eth-rpc-quorum: 1
  eth-bridge-proxy-address: "${proxy}"
  eth-bridge-chain-id: ${BRIDGE_ETH_CHAIN_ID}
  eth-contracts-start-block-fallback: 0
deposits:
  enabled: true
EOF

    cat > "$BRIDGE_CLIENT_CONFIG_PATH" <<EOF
myso-rpc-url: "${MYSO_RPC_URL}"
eth-rpc-url: "${ETH_RPC_URL}"
eth-bridge-proxy-address: "${proxy}"
myso-key-path: ${BRIDGE_CLIENT_KEY_PATH}
eth-key-path: ${BRIDGE_CLIENT_KEY_PATH}
EOF

    cat > "$BRIDGE_COMMITTEE_CONFIG_PATH" <<EOF
bridge-authority-port-and-key-path:
  - - ${BRIDGE_NODE_PORT}
    - ${BRIDGE_AUTHORITY_KEY_PATH}
EOF

    log_step "Wrote configs under $BRIDGE_DIR"
    log_session_use "BRIDGE_NODE_CONFIG_PATH" "$BRIDGE_NODE_CONFIG_PATH"
    log_session_use "BRIDGE_CLIENT_CONFIG_PATH" "$BRIDGE_CLIENT_CONFIG_PATH"
}

bridge_start_anvil_opt_in() {
    local logf anvil_bin
    anvil_bin="$(bridge_resolve_anvil || true)"
    [[ -n "$anvil_bin" ]] || {
        echo "anvil not found (install Foundry: curl -L https://foundry.paradigm.xyz | bash && foundryup)" >&2
        return 1
    }
    if bridge_port_in_use "$BRIDGE_ANVIL_PORT"; then
        echo "Port $BRIDGE_ANVIL_PORT already in use; not starting another anvil." >&2
        ETH_RPC_URL="http://127.0.0.1:${BRIDGE_ANVIL_PORT}"
        return 0
    fi
    mkdir -p "$BRIDGE_DIR"
    logf="$BRIDGE_DIR/anvil.log"
    log_step "Starting anvil on $BRIDGE_ANVIL_PORT (opt-in; no cleanup trap)"
    nohup "$anvil_bin" --port "$BRIDGE_ANVIL_PORT" --block-time 1 --slots-in-an-epoch 1 \
        >"$logf" 2>&1 &
    echo "anvil pid $!  log $logf" >&2
    ETH_RPC_URL="http://127.0.0.1:${BRIDGE_ANVIL_PORT}"
    local i
    for i in $(seq 1 20); do
        bridge_eth_rpc_reachable "$ETH_RPC_URL" && return 0
        sleep 0.25
    done
    echo "anvil did not become reachable at $ETH_RPC_URL" >&2
    return 1
}

bridge_print_manual_forge() {
    cat >&2 <<EOF
Start anvil yourself, then re-run without --skip-evm, or deploy manually:

  OVERRIDE_CONFIG_PATH=/tmp/myso-bridge-bootstrap/sol_deploy_config.json \\
  PRIVATE_KEY=${BRIDGE_EVM_PRIVATE_KEY} \\
  forge script script/deploy_bridge.s.sol --fork-url ${ETH_RPC_URL} --broadcast --ffi --chain 31337

Working directory: ${REPO_ROOT}/bridge/evm
EOF
}

bridge_write_evm_deploy_json() {
    local path="$1" member
    member="${BRIDGE_AUTHORITY_ETH_ADDRESS:-}"
    [[ -n "$member" ]] || {
        echo "Authority EVM address is empty; generate keys first" >&2
        return 1
    }
    case "$member" in
        0x*|0X*) ;;
        *) member="0x${member}" ;;
    esac
    mkdir -p "$(dirname "$path")"
    cat > "$path" <<EOF
{
  "committeeMemberStake": [10000],
  "committeeMembers": ["${member}"],
  "minCommitteeStakeRequired": 10000,
  "sourceChainId": 12,
  "supportedChainIds": [1, 2, 3],
  "supportedChainLimitsInDollars": [1000000000000000, 1000000000000000, 1000000000000000],
  "supportedTokens": [],
  "tokenPrices": [12800, 432518900, 25969600, 10000, 10000],
  "tokenIds": [],
  "mysoDecimals": [],
  "weth": "0x0000000000000000000000000000000000000000"
}
EOF
}

bridge_parse_forge_deployed() {
    local log="$1"
    python3 - "$log" <<'PY'
import re, sys
text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
found = {}
for m in re.finditer(r"\[Deployed\]\s*([^:]+):\s*(0x[0-9a-fA-F]+)", text):
    found[m.group(1).strip()] = m.group(2)
for name in ("MySoBridge", "BridgeCommittee", "BridgeConfig", "BridgeLimiter", "BridgeVault", "BTC", "ETH", "USDC", "USDT", "KA", "WETH"):
    if name in found:
        print(f"{name}={found[name]}")
PY
}

bridge_evm_deps_present() {
    [[ -f "$REPO_ROOT/bridge/evm/dependencies/forge-std-1.9.2/src/Script.sol" ]] \
        && [[ -f "$REPO_ROOT/bridge/evm/dependencies/openzeppelin-foundry-upgrades-0.3.1/src/Upgrades.sol" ]] \
        && [[ -d "$REPO_ROOT/bridge/evm/dependencies/@openzeppelin-contracts-5.0.1" ]]
}

bridge_install_evm_deps_from_soldeer_lock() {
    local lock="$REPO_ROOT/bridge/evm/soldeer.lock"
    [[ -f "$lock" ]] || {
        echo "Missing $lock — cannot install bridge/evm dependencies." >&2
        return 1
    }
    log_step "Installing bridge/evm deps from soldeer.lock S3 archives (soldeer registry fallback)"
    python3 - "$REPO_ROOT/bridge/evm" "$lock" <<'PY'
import re, sys, zipfile, urllib.request
from pathlib import Path

root = Path(sys.argv[1])
lock = Path(sys.argv[2]).read_text(encoding="utf-8")
deps = []
for block in re.split(r"\n\s*\n", lock.strip()):
    name = version = source = None
    for line in block.splitlines():
        line = line.strip()
        if line.startswith("name ="):
            name = line.split("=", 1)[1].strip().strip('"')
        elif line.startswith("version ="):
            version = line.split("=", 1)[1].strip().strip('"')
        elif line.startswith("source ="):
            source = line.split("=", 1)[1].strip().strip('"')
    if name and version and source:
        deps.append((name, version, source))

deps_dir = root / "dependencies"
deps_dir.mkdir(parents=True, exist_ok=True)
for name, version, url in deps:
    dest = deps_dir / f"{name}-{version}"
    marker = dest / ".soldeer-installed"
    if marker.exists():
        print(f"skip {dest.name}", flush=True)
        continue
    print(f"fetch {url}", flush=True)
    data = urllib.request.urlopen(url, timeout=120).read()
    tmp = deps_dir / f".{name}-{version}.zip"
    tmp.write_bytes(data)
    dest.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(tmp) as zf:
        for member in zf.namelist():
            if member.endswith("/"):
                continue
            rel = member.lstrip("/")
            if not rel:
                continue
            out = dest / rel
            out.parent.mkdir(parents=True, exist_ok=True)
            out.write_bytes(zf.read(member))
    tmp.unlink(missing_ok=True)
    marker.write_text(url + "\n", encoding="utf-8")
    print(f"installed {dest.name}", flush=True)
PY
}

bridge_ensure_evm_deps() {
    if bridge_evm_deps_present; then
        return 0
    fi
    log_step "Bridge EVM dependencies missing under bridge/evm/dependencies/"
    (
        cd "$REPO_ROOT/bridge/evm"
        if forge soldeer update; then
            exit 0
        fi
        echo "forge soldeer update failed; using soldeer.lock S3 fallback" >&2
    ) || true
    if ! bridge_evm_deps_present; then
        bridge_install_evm_deps_from_soldeer_lock || return 1
    fi
    if ! bridge_evm_deps_present; then
        echo "bridge/evm dependencies still missing after install." >&2
        echo "Run manually: cd bridge/evm && forge soldeer update" >&2
        return 1
    fi
    log_step "Bridge EVM dependencies ready"
}

bridge_deploy_evm() {
    local cfg out_dir logf parsed
    if [[ -n "${BRIDGE_PROXY:-}" ]]; then
        log_step "Reusing deployed bridge proxy ${BRIDGE_PROXY}"
        return 0
    fi
    if ! bridge_eth_rpc_reachable "$ETH_RPC_URL"; then
        echo "Ethereum RPC not reachable at $ETH_RPC_URL — skipping EVM deploy." >&2
        bridge_print_manual_forge
        return 0
    fi
    bridge_require_cmd forge || {
        echo "forge not on PATH — skipping EVM deploy." >&2
        bridge_print_manual_forge
        return 0
    }
    if ! bridge_ensure_evm_deps; then
        echo "Could not install bridge/evm Foundry dependencies — skipping EVM deploy." >&2
        bridge_print_manual_forge
        return 0
    fi
    cfg="/tmp/myso-bridge-bootstrap/sol_deploy_config.json"
    bridge_write_evm_deploy_json "$cfg" || return 1
    out_dir="out-bridge-bootstrap"
    logf="$BRIDGE_DIR/forge-deploy.log"
    mkdir -p "$BRIDGE_DIR"
    log_step "Deploying EVM bridge via forge against $ETH_RPC_URL"
    (
        cd "$REPO_ROOT/bridge/evm"
        if [[ "${BRIDGE_FORGE_CLEAN:-0}" == 1 ]]; then
            forge clean
        fi
        OVERRIDE_CONFIG_PATH="$cfg" \
        PRIVATE_KEY="${BRIDGE_EVM_PRIVATE_KEY}" \
        ETHERSCAN_API_KEY="${ETHERSCAN_API_KEY:-n/a}" \
        FOUNDRY_OUT="$out_dir" \
            forge script script/deploy_bridge.s.sol \
                --fork-url "$ETH_RPC_URL" \
                --broadcast --ffi --chain 31337
    ) >"$logf" 2>&1 || {
        echo "forge deploy failed; see $logf" >&2
        tail -n 40 "$logf" >&2
        return 1
    }
    parsed="$(bridge_parse_forge_deployed "$logf")"
    eval "$parsed"
    BRIDGE_PROXY="${MySoBridge:-}"
    BRIDGE_COMMITTEE="${BridgeCommittee:-}"
    BRIDGE_CONFIG="${BridgeConfig:-}"
    BRIDGE_LIMITER="${BridgeLimiter:-}"
    BRIDGE_VAULT="${BridgeVault:-}"
    BRIDGE_BTC="${BTC:-}"
    BRIDGE_WETH="${ETH:-${WETH:-}}"
    BRIDGE_USDC="${USDC:-}"
    BRIDGE_USDT="${USDT:-}"
    BRIDGE_KA="${KA:-}"
    [[ -n "$BRIDGE_PROXY" ]] || {
        echo "forge succeeded but no [Deployed] MySoBridge line parsed from $logf" >&2
        tail -n 40 "$logf" >&2
        return 1
    }
    log_session_use "BRIDGE_PROXY" "$BRIDGE_PROXY"
}

bridge_fund_address() {
    local dest="$1" min_mist="$2"
    local dest_n active bal out
    dest_n="$(normalize_hex_id "$dest")" || return 1
    bal="$(resolve_total_coin_balance "$dest_n" 2>/dev/null || echo 0)"
    if [[ -n "$bal" && "$bal" -ge "$min_mist" ]]; then
        log_step "Address $dest_n already has $bal mist"
        return 0
    fi
    active="$(resolve_myso_active_address)" || return 1
    log_step "Funding $dest_n with ${min_mist} mist from $active"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
        --split-coins gas "[${min_mist}]" \
        --assign fund_coin \
        --transfer-objects "[fund_coin]" "@${dest_n}")" || true
    assert_tx_success "$out"
}

bridge_committee_registered() {
    local inner
    inner="$(bridge_inner_json 2>/dev/null)" || return 1
    printf '%s' "$inner" | python3 -c '
import json, sys
d = json.load(sys.stdin)
regs = (((d.get("committee") or {}).get("member_registrations") or {}).get("contents") or [])
members = (((d.get("committee") or {}).get("members") or {}).get("contents") or [])
sys.exit(0 if regs or members else 1)
'
}

bridge_committee_pending() {
    local inner
    inner="$(bridge_inner_json 2>/dev/null)" || return 1
    printf '%s' "$inner" | python3 -c '
import json, sys
d = json.load(sys.stdin)
regs = (((d.get("committee") or {}).get("member_registrations") or {}).get("contents") or [])
members = (((d.get("committee") or {}).get("members") or {}).get("contents") or [])
sys.exit(0 if regs and not members else 1)
'
}

bridge_committee_finalized() {
    local inner
    inner="$(bridge_inner_json 2>/dev/null)" || return 1
    printf '%s' "$inner" | python3 -c '
import json, sys
d = json.load(sys.stdin)
members = (((d.get("committee") or {}).get("members") or {}).get("contents") or [])
sys.exit(0 if members else 1)
'
}

# Pending registration or a seated member — do not re-init committee.
bridge_committee_has_seat() {
    bridge_committee_finalized || bridge_committee_registered
}

# Temp client.yaml only (never writes ~/.myso). Keystore path stays the user's file.
bridge_temp_client_yaml() {
    local validator="$1"
    local dest="/tmp/myso-bridge-bootstrap/client.yaml"
    local src
    src="$(bridge_myso_config_dir)/client.yaml"
    [[ -f "$src" ]] || return 1
    mkdir -p /tmp/myso-bridge-bootstrap
    python3 - "$src" "$dest" "$validator" "$MYSO_RPC_URL" <<'PY'
import sys
src, dest, validator, rpc = sys.argv[1:5]
text = open(src, encoding="utf-8").read()
# Keep the original keystore File: path. Only override active address + localnet rpc.
lines = []
for line in text.splitlines():
    if line.startswith("active_address:"):
        lines.append(f'active_address: "{validator}"')
    else:
        lines.append(line)
open(dest, "w", encoding="utf-8").write("\n".join(lines) + "\n")
PY
    printf '%s' "$dest"
}

bridge_keystore_has_address_in_dir() {
    local config_dir="$1" addr="$2"
    MYSO_CONFIG_DIR="$config_dir" bridge_keystore_has_address "$addr"
}

bridge_keystore_has_address() {
    local addr="$1"
    myso client addresses --json 2>/dev/null | python3 -c '
import json, sys
want = sys.argv[1].lower()
if want.startswith("0x"):
    want = want[2:]
want = want.zfill(64)
raw = sys.stdin.read()
try:
    data = json.loads(raw)
except Exception:
    sys.exit(1)
items = data.get("addresses") or []
found = []
for item in items:
    if isinstance(item, dict):
        found.append(str(item.get("address") or item.get("mysoAddress") or ""))
    elif isinstance(item, (list, tuple)) and len(item) >= 2:
        found.append(str(item[1]))
    else:
        found.append(str(item))
for a in found:
    body = a.lower()[2:] if a.lower().startswith("0x") else a.lower()
    if body.zfill(64) == want:
        sys.exit(0)
sys.exit(1)
' "$(normalize_hex_id "$addr")"
}

bridge_register_committee() {
    local yaml out validator client_yaml client_cfg rc=0
    if [[ "${COMMITTEE_FINALIZED:-}" == 1 ]] && ! bridge_committee_finalized; then
        log_step "Session had COMMITTEE_FINALIZED=1 but chain is not finalized; waiting again"
        COMMITTEE_FINALIZED=''
        COMMITTEE_REGISTERED=''
    fi
    if [[ "${COMMITTEE_FINALIZED:-}" == 1 ]] && bridge_committee_finalized; then
        log_step "Bridge committee already finalized"
        COMMITTEE_REGISTERED=1
        COMMITTEE_FINALIZED=1
        return 0
    fi
    if bridge_committee_registered; then
        log_step "Bridge authority already registered"
        COMMITTEE_REGISTERED=1
        return 0
    fi
    if [[ "${COMMITTEE_REGISTERED:-}" == 1 ]]; then
        log_step "Session had COMMITTEE_REGISTERED=1 but chain has no registration; re-registering"
        COMMITTEE_REGISTERED=''
    fi

    validator="$(bridge_live_validators | head -n1)" || validator=''
    if [[ -n "$validator" ]]; then
        log_step "Live validator $validator (from mysox_getLatestMySoSystemState)"
    fi

    if [[ -n "$validator" ]]; then
        bridge_fund_address "$validator" "${BRIDGE_AUTHORITY_MIN_MIST:-10000000000}" || true
    fi

    # Prefer the validator CLI when the live key is in this keystore. The
    # bridge-committee-init command waits on checkpoint finality and can hang.
    if [[ -n "$validator" ]] && bridge_keystore_has_address "$validator"; then
        client_yaml="$(bridge_temp_client_yaml "$validator")" || client_yaml=''
        if [[ -n "$client_yaml" ]]; then
            log_step "Registering bridge committee as $validator (temp client.yaml; ~/.myso untouched)"
            if bridge_run_with_deadline "${BRIDGE_COMMITTEE_INIT_TIMEOUT_SECS:-90}" \
                myso --client.config "$client_yaml" validator register-bridge-committee \
                --bridge-authority-key-path "$BRIDGE_AUTHORITY_KEY_PATH" \
                --bridge-authority-url "$BRIDGE_AUTHORITY_URL" >&2; then
                if bridge_committee_registered; then
                    COMMITTEE_REGISTERED=1
                    return 0
                fi
            fi
            echo "register-bridge-committee did not land a registration; trying bridge-committee-init" >&2
        fi
    elif [[ -n "$validator" ]]; then
        echo "Live validator $validator is not in $(bridge_myso_config_dir) keystore." >&2
    fi

    yaml="$(bridge_matching_network_yaml || true)"
    if [[ -n "$yaml" ]]; then
        log_step "Registering committee via myso bridge-committee-init ($yaml)"
        client_cfg="$(bridge_client_config "$(bridge_myso_config_dir)" 2>/dev/null || true)"
        rc=0
        if [[ -n "$client_cfg" ]]; then
            bridge_run_with_deadline "${BRIDGE_COMMITTEE_INIT_TIMEOUT_SECS:-90}" \
                myso bridge-committee-init \
                --network.config "$yaml" \
                --client.config "$client_cfg" \
                --bridge_committee.config "$BRIDGE_COMMITTEE_CONFIG_PATH" >&2 || rc=$?
        else
            bridge_run_with_deadline "${BRIDGE_COMMITTEE_INIT_TIMEOUT_SECS:-90}" \
                myso bridge-committee-init \
                --network.config "$yaml" \
                --bridge_committee.config "$BRIDGE_COMMITTEE_CONFIG_PATH" >&2 || rc=$?
        fi
        if [[ "$rc" == 124 ]]; then
            echo "bridge-committee-init timed out after ${BRIDGE_COMMITTEE_INIT_TIMEOUT_SECS:-90}s — checking chain" >&2
        fi
        if bridge_committee_registered; then
            COMMITTEE_REGISTERED=1
            return 0
        fi
        echo "bridge-committee-init did not register a member (exit ${rc})." >&2
    fi

    if [[ -z "$validator" ]]; then
        echo "Could not discover the live validator. Skipping committee registration." >&2
        return 0
    fi
    echo "Committee registration not on-chain yet; epoch/token publish can still continue." >&2
    return 0
}

bridge_wait_committee_finalized() {
    local waited=0 max="${BRIDGE_COMMITTEE_WAIT_SECS}" step=5 total
    if bridge_committee_node_ready; then
        COMMITTEE_REGISTERED=1
        COMMITTEE_FINALIZED=1
        total="$(bridge_committee_total_voting_power 2>/dev/null || echo 0)"
        log_step "Bridge committee has a seated node (voting power ${total}/10000)"
        return 0
    fi
    if ! bridge_committee_pending && [[ "${COMMITTEE_REGISTERED:-}" != 1 ]] && ! bridge_committee_registered; then
        echo "No committee seat or pending registration on-chain yet." >&2
        return 1
    fi
    COMMITTEE_REGISTERED=1
    log_step "Waiting up to ${max}s for the epoch to seat the committee (voting power >= 7500)"
    while (( waited < max )); do
        if bridge_committee_node_ready; then
            COMMITTEE_FINALIZED=1
            total="$(bridge_committee_total_voting_power 2>/dev/null || echo 0)"
            log_step "Bridge committee seated (voting power ${total}/10000)"
            return 0
        fi
        waited=$((waited + step))
        log_wait_progress "bridge committee epoch seat" "$((waited / step))" "$((max / step))" \
            "need voting power >= 7500"
        sleep "$step"
    done
    echo "Committee still pending after ${max}s — do not start the node until voting power >= 7500 (re-run after the next epoch)." >&2
    return 1
}

bridge_print_node_start() {
    echo "" >&2
    echo "Start the bridge node only after the committee is seated (voting power >= 7500):" >&2
    echo "  cargo run -p myso-bridge --bin myso-bridge-node -- --config-path ${BRIDGE_NODE_CONFIG_PATH}" >&2
    echo "Authority URL: ${BRIDGE_AUTHORITY_URL}" >&2
}

bridge_stop_stale_node() {
    if bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        return 0
    fi
    bridge_stop_running_node
}

bridge_stop_running_node() {
    local port pid cmd
    for port in "$BRIDGE_NODE_PORT" "$BRIDGE_METRICS_PORT"; do
        [[ -n "$port" ]] || continue
        while IFS= read -r pid; do
            [[ -n "$pid" ]] || continue
            cmd="$(ps -p "$pid" -o args= 2>/dev/null || true)"
            if printf '%s' "$cmd" | grep -q 'myso-bridge-node'; then
                log_step "Stopping myso-bridge-node pid $pid on :$port"
                kill "$pid" 2>/dev/null || true
            fi
        done < <(lsof -nP -iTCP:"$port" -sTCP:LISTEN -t 2>/dev/null || true)
    done
    local i
    for i in $(seq 1 15); do
        bridge_node_reachable "$BRIDGE_AUTHORITY_URL" || return 0
        sleep 1
    done
}

bridge_reload_node_for_token_governance() {
    if ! bridge_committee_node_ready; then
        echo "Not starting the node: committee voting power is still below 7500." >&2
        return 1
    fi
    if bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        log_step "Restarting node so approved-governance-actions match published token types"
        bridge_stop_running_node
    fi
    bridge_start_node_opt_in
}

bridge_node_reachable() {
    local url="${1:-$BRIDGE_AUTHORITY_URL}"
    curl -sf --connect-timeout 2 --max-time 4 "${url%/}/ping" >/dev/null 2>&1 \
        || curl -sf --connect-timeout 2 --max-time 4 "${url%/}/" >/dev/null 2>&1
}

bridge_start_node_opt_in() {
    local bin logf node_bin wait_secs="${BRIDGE_NODE_START_WAIT_SECS:-300}"
    if ! bridge_committee_node_ready; then
        echo "Not starting the node: committee voting power is still below 7500." >&2
        return 1
    fi
    bridge_stop_stale_node
    if bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        log_step "Bridge node already reachable at $BRIDGE_AUTHORITY_URL"
        return 0
    fi
    if bridge_port_in_use "$BRIDGE_NODE_PORT" || bridge_port_in_use "$BRIDGE_METRICS_PORT"; then
        echo "Port ${BRIDGE_NODE_PORT}/${BRIDGE_METRICS_PORT} still in use after stale cleanup; not starting another node." >&2
        return 1
    fi
    bin="$(bridge_node_bin || true)"
    node_bin="${REPO_ROOT}/target/debug/myso-bridge-node"
    if [[ -z "$bin" && ! -x "$node_bin" ]]; then
        log_step "Building myso-bridge-node (one-time)"
        (cd "$REPO_ROOT" && cargo build -p myso-bridge --bin myso-bridge-node) >>"$BRIDGE_DIR/bridge-node-build.log" 2>&1 \
            || {
                echo "Failed to build myso-bridge-node; see $BRIDGE_DIR/bridge-node-build.log" >&2
                return 1
            }
    fi
    mkdir -p "$BRIDGE_DIR" "$BRIDGE_DB_PATH"
    logf="$BRIDGE_DIR/bridge-node.log"
    log_step "Starting bridge node on $BRIDGE_NODE_PORT (log $BRIDGE_DIR/bridge-node.log)"
    local launch_bin launch_args
    if [[ -n "$bin" ]]; then
        launch_bin="$(command -v myso-bridge-node)"
        launch_args=("--config-path" "$BRIDGE_NODE_CONFIG_PATH")
    elif [[ -x "$node_bin" ]]; then
        launch_bin="$node_bin"
        launch_args=("--config-path" "$BRIDGE_NODE_CONFIG_PATH")
    else
        launch_bin="$(command -v cargo)"
        launch_args=("run" "-p" "myso-bridge" "--bin" "myso-bridge-node" "--" "--config-path" "$BRIDGE_NODE_CONFIG_PATH")
    fi
    local node_pid
    node_pid="$(python3 - "$launch_bin" "$logf" "${launch_args[@]}" <<'PY'
import os, subprocess, sys
bin_path, logf, *args = sys.argv[1:]
log = open(logf, "w")
proc = subprocess.Popen(
    [bin_path, *args],
    stdin=subprocess.DEVNULL,
    stdout=log,
    stderr=subprocess.STDOUT,
    start_new_session=True,
    close_fds=True,
)
print(proc.pid)
PY
)"
    echo "$node_pid" >"$BRIDGE_DIR/bridge-node.pid"
    echo "bridge-node pid $node_pid  log $logf" >&2
    local i
    for i in $(seq 1 "$wait_secs"); do
        bridge_node_reachable "$BRIDGE_AUTHORITY_URL" && return 0
        sleep 1
    done
    echo "Bridge node did not become reachable; see $logf" >&2
    tail -n 30 "$logf" >&2
    return 1
}

readonly BRIDGE_GQL_ACTIVE_CAPS='query Caps($active: MySoAddress!) {
  coinCreationAdminCap: objects(
    filter: { type: "0x2::coin::CoinCreationAdminCap", ownerKind: ADDRESS, owner: $active }, last: 1
  ) { nodes { address } }
  packagePublishingAdminCap: objects(
    filter: { type: "0x2::package::PackagePublishingAdminCap", ownerKind: ADDRESS, owner: $active }, last: 1
  ) { nodes { address } }
}'

readonly BRIDGE_GQL_GLOBAL_CAPS='query {
  coinCreationAdminCap: objects(filter: { type: "0x2::coin::CoinCreationAdminCap" }, last: 1) { nodes { address } }
  packagePublishingAdminCap: objects(filter: { type: "0x2::package::PackagePublishingAdminCap" }, last: 1) { nodes { address } }
}'

bridge_clear_stale_cap_id() {
    local env_key="$1"
    local id="${!env_key:-}"
    [[ -n "$id" ]] || return 0
    if ! object_exists_on_fullnode "$id"; then
        echo "Clearing stale ${env_key}=${id} (not on fullnode)" >&2
        printf -v "$env_key" '%s' ''
    else
        printf -v "$env_key" '%s' "$(normalize_hex_id "$id")"
    fi
}

bridge_cap_owned_by() {
    local cap="$1" expected="$2" owner
    cap="$(normalize_hex_id "$cap")" || return 1
    expected="$(normalize_hex_id "$expected")" || return 1
    owner="$(object_address_owner "$cap")" || return 1
    [[ "$(normalize_hex_id "$owner")" == "$expected" ]]
}

bridge_bind_admin_cap() {
    local env_key="$1" gql_id="$2" active="$3"
    local candidate
    if [[ -z "$gql_id" || -z "$active" ]]; then
        printf -v "$env_key" '%s' ''
        return 1
    fi
    candidate="$(normalize_hex_id "$gql_id")" || candidate=''
    if [[ -n "$candidate" ]] && object_exists_on_fullnode "$candidate" \
        && bridge_cap_owned_by "$candidate" "$active"; then
        printf -v "$env_key" '%s' "$candidate"
        return 0
    fi
    if [[ -n "$candidate" ]]; then
        echo "Ignoring ${env_key}=${candidate} (not owned by active ${active})" >&2
    fi
    printf -v "$env_key" '%s' ''
    return 1
}

bridge_refresh_admin_caps_from_graphql() {
    local active="${1:-}"
    [[ -n "$active" ]] || active="$(resolve_myso_active_address || true)"
    [[ -n "$active" ]] || return 1
    active="$(normalize_hex_id "$active")" || return 1
    graphql_is_reachable "$GRAPHQL_URL" || return 1
    bridge_import_admin_caps
    if [[ -n "${COIN_CREATION_ADMIN_CAP_ID:-}" && -n "${PACKAGE_PUBLISH_ADMIN_CAP_ID:-}" ]]; then
        return 0
    fi
    bridge_resolve_global_admin_caps
}

bridge_refresh_session_from_graphql() {
    local active attempt max=12
    command -v curl >/dev/null 2>&1 || { echo "curl required" >&2; return 1; }
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; return 1; }
    if ! graphql_is_reachable "$GRAPHQL_URL"; then
        echo "GraphQL unreachable at $GRAPHQL_URL — start myso with --with-indexer --with-graphql" >&2
        return 1
    fi
    active="$(resolve_myso_active_address)" || {
        echo "Could not resolve myso client active-address" >&2
        return 1
    }
    active="$(normalize_hex_id "$active")" || return 1
    for ((attempt = 1; attempt <= max; attempt++)); do
        log_step "Refreshing bridge admin caps from GraphQL (attempt $attempt/$max)"
        COIN_CREATION_ADMIN_CAP_ID=''
        PACKAGE_PUBLISH_ADMIN_CAP_ID=''
        BRIDGE_CAP_OWNER=''
        if bridge_refresh_admin_caps_from_graphql "$active"; then
            log_session_use "COIN_CREATION_ADMIN_CAP_ID" "$COIN_CREATION_ADMIN_CAP_ID"
            log_session_use "PACKAGE_PUBLISH_ADMIN_CAP_ID" "$PACKAGE_PUBLISH_ADMIN_CAP_ID"
            return 0
        fi
        sleep 2
    done
    echo "GraphQL refresh incomplete; admin caps not found on-chain" >&2
    return 1
}

bridge_import_admin_caps() {
    local active json gql_coin gql_pkg
    bridge_clear_stale_cap_id COIN_CREATION_ADMIN_CAP_ID
    bridge_clear_stale_cap_id PACKAGE_PUBLISH_ADMIN_CAP_ID

    active="$(resolve_myso_active_address || true)"
    if [[ -n "$active" ]]; then
        for key in COIN_CREATION_ADMIN_CAP_ID PACKAGE_PUBLISH_ADMIN_CAP_ID; do
            local id="${!key:-}"
            if [[ -n "$id" ]] && ! bridge_cap_owned_by "$id" "$active"; then
                echo "Clearing ${key}=${id} (not owned by active ${active})" >&2
                printf -v "$key" '%s' ''
            fi
        done
    fi

    if graphql_is_reachable "$GRAPHQL_URL" && [[ -n "$active" ]]; then
        json="$(graphql_post "$BRIDGE_GQL_ACTIVE_CAPS" \
            "$(jq -nc --arg active "$(normalize_hex_id "$active")" '{active: $active}')" 2>/dev/null)" || json=''
        if [[ -n "$json" ]]; then
            gql_coin="$(gql_object_address "$json" "coinCreationAdminCap")"
            gql_pkg="$(gql_object_address "$json" "packagePublishingAdminCap")"
            bridge_bind_admin_cap COIN_CREATION_ADMIN_CAP_ID "$gql_coin" "$active" || true
            bridge_bind_admin_cap PACKAGE_PUBLISH_ADMIN_CAP_ID "$gql_pkg" "$active" || true
        fi
    fi
}

bridge_resolve_global_admin_caps() {
    local json gql_coin gql_pkg coin_owner pkg_owner
    [[ -n "${COIN_CREATION_ADMIN_CAP_ID:-}" && -n "${PACKAGE_PUBLISH_ADMIN_CAP_ID:-}" ]] && return 0
    graphql_is_reachable "$GRAPHQL_URL" || return 1
    json="$(graphql_post "$BRIDGE_GQL_GLOBAL_CAPS" '{}' 2>/dev/null)" || return 1
    gql_coin="$(gql_object_address "$json" "coinCreationAdminCap")"
    gql_pkg="$(gql_object_address "$json" "packagePublishingAdminCap")"
    [[ -n "$gql_coin" && -n "$gql_pkg" ]] || return 1
    if ! object_exists_on_fullnode "$gql_coin" || ! object_exists_on_fullnode "$gql_pkg"; then
        return 1
    fi
    coin_owner="$(object_address_owner "$gql_coin")" || return 1
    pkg_owner="$(object_address_owner "$gql_pkg")" || return 1
    if [[ "$(normalize_hex_id "$coin_owner")" != "$(normalize_hex_id "$pkg_owner")" ]]; then
        echo "CoinCreationAdminCap and PackagePublishingAdminCap have different owners on-chain" >&2
        return 1
    fi
    COIN_CREATION_ADMIN_CAP_ID="$(normalize_hex_id "$gql_coin")"
    PACKAGE_PUBLISH_ADMIN_CAP_ID="$(normalize_hex_id "$gql_pkg")"
    BRIDGE_CAP_OWNER="$(normalize_hex_id "$coin_owner")"
}

bridge_ensure_admin_caps_for_publish() {
    local cap_owner active
    BRIDGE_CAP_OWNER=''
    bridge_import_admin_caps
    if [[ -n "${COIN_CREATION_ADMIN_CAP_ID:-}" && -n "${PACKAGE_PUBLISH_ADMIN_CAP_ID:-}" ]]; then
        return 0
    fi

    if ! bridge_resolve_global_admin_caps; then
        cap_owner=''
    else
        cap_owner="$BRIDGE_CAP_OWNER"
    fi
    if [[ -z "$cap_owner" ]]; then
        echo "CoinCreationAdminCap / PackagePublishingAdminCap not found on-chain." >&2
        echo "Run ./scripts/bootstrap.sh once per chain (bootstrap is one-time)." >&2
        return 1
    fi

    active="$(resolve_myso_active_address || true)"
    if [[ -n "$active" && "$(normalize_hex_id "$active")" == "$cap_owner" ]]; then
        return 0
    fi

    if bridge_keystore_has_address "$cap_owner"; then
        log_step "Switching active wallet to ${cap_owner} (owns bootstrap admin caps)"
        switch_wallet "$cap_owner"
        return 0
    fi

    local default_dir="${HOME}/.myso/myso_config"
    if [[ -n "${MYSO_CONFIG_DIR:-}" && "$MYSO_CONFIG_DIR" != "$default_dir" ]] \
        && bridge_keystore_has_address_in_dir "$default_dir" "$cap_owner"; then
        log_step "Using bootstrap wallet from ${default_dir} for token publish (caps owned by ${cap_owner})"
        BRIDGE_PUBLISH_MYSO_CONFIG_DIR="$default_dir"
        BRIDGE_PUBLISH_CAP_OWNER="$cap_owner"
        return 0
    fi

    echo "Bootstrap admin caps exist on ${cap_owner} but that address is not in the active keystore." >&2
    echo "Either:" >&2
    echo "  1. Run ./scripts/bootstrap.sh with the same MYSO_CONFIG_DIR before bridge bootstrap, or" >&2
    echo "  2. Import the bootstrap wallet key (${cap_owner}) into your localnet keystore." >&2
    COIN_CREATION_ADMIN_CAP_ID=''
    PACKAGE_PUBLISH_ADMIN_CAP_ID=''
    return 1
}

bridge_patch_token_move_toml() {
    local toml="$1"
    local chain="${2:-$(bridge_live_chain_identifier)}"
    [[ -f "$toml" && -n "$chain" ]] || return 0
    python3 - "$toml" "$chain" <<'PY'
import re, sys
path, chain = sys.argv[1], sys.argv[2].strip().lower()
text = open(path, encoding="utf-8").read()
if re.search(r"(?m)^\[environments\]", text):
    if re.search(r"(?m)^localnet\s*=", text):
        text = re.sub(r"(?m)^localnet\s*=.*$", f'localnet = "{chain}"', text, count=1)
    else:
        text = re.sub(
            r"(?m)^(\[environments\]\s*\n)",
            rf'\1localnet = "{chain}"\n',
            text,
            count=1,
        )
else:
    text = text.rstrip() + f'\n\n[environments]\nlocalnet = "{chain}"\n'
open(path, "w", encoding="utf-8").write(text)
PY
}

bridge_copy_token_pkg() {
    local kind="$1"
    local src="$REPO_ROOT/bridge/move/tokens/$kind"
    local dest="$BRIDGE_DIR/token-packages/$kind"
    [[ -d "$src/sources" ]] || {
        echo "Missing token sources: $src" >&2
        return 1
    }
    mkdir -p "$dest"
    cp -R "$src/sources" "$dest/"
    cp "$src/Move.toml" "$dest/Move.toml"
    rm -f "$dest/Published.toml" "$dest"/Pub.*.toml "$dest/Move.lock"
    bridge_patch_token_move_toml "$dest/Move.toml"
    printf '%s' "$dest"
}

BRIDGE_LAST_PUBLISH_JSON=''
BRIDGE_LAST_PUBLISH_DIGEST=''
BRIDGE_PUBLISH_MYSO_CONFIG_DIR=''
BRIDGE_PUBLISH_CAP_OWNER=''
BRIDGE_CAP_OWNER=''

bridge_resolve_upgrade_cap_for_package() {
    local pkg="$1" owner="$2" attempt json cap
    pkg="$(normalize_hex_id "$pkg")" || return 1
    owner="$(normalize_hex_id "$owner")" || return 1
    graphql_is_reachable "$GRAPHQL_URL" || return 1
    for attempt in $(seq 1 12); do
        json="$(graphql_post 'query UpgradeCaps($owner: MySoAddress!) {
          objects(filter: { type: "0x2::package::UpgradeCap", ownerKind: ADDRESS, owner: $owner }, last: 30) {
            nodes { address asMoveObject { contents { json } } }
          }
        }' "$(jq -nc --arg owner "$owner" '{owner: $owner}')" 2>/dev/null)" || json=''
        cap="$(echo "$json" | jq -r --arg pkg "$pkg" '
            def norm:
                (tostring | ltrimstr("0x") | ascii_downcase);
            .data.objects.nodes[]?
            | select((.asMoveObject.contents.json.package // "" | norm) == ($pkg | norm))
            | .address
        ' | head -n1)"
        if [[ -n "$cap" ]]; then
            normalize_hex_id "$cap"
            return 0
        fi
        sleep 1
    done
    return 1
}

bridge_extract_created_containing() {
    local digest="$1" needle="$2"
    local json result
    [[ -n "$digest" && -n "$needle" ]] || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    result="$(echo "$json" | jq -r --arg t "$needle" '
        def object_type($o):
            ($o.objectType? // $o.object_type? // $o.type? // "") | tostring;
        def object_id($o):
            ($o.objectId? // $o.object_id? // $o.reference?.objectId? // "") | tostring;
        (.changed_objects // .changedObjects // [])[]
        | if type == "array" then empty else . end
        | select(object_type(.) | contains($t))
        | object_id(.)
        | select(length > 0)
    ' | head -n1)"
    [[ -n "$result" ]] || return 1
    normalize_hex_id "$result"
}

bridge_extract_json() {
    local raw="$1"
    if printf '%s' "$raw" | jq -e . >/dev/null 2>&1; then
        printf '%s' "$raw"
        return 0
    fi
    python3 -c '
import json, sys
text = sys.argv[1]
last = None
i = 0
while True:
    i = text.find("{", i)
    if i < 0:
        break
    try:
        obj, end = json.JSONDecoder().raw_decode(text[i:])
        last = obj
        i += end
    except Exception:
        i += 1
if last is None:
    sys.exit(1)
json.dump(last, sys.stdout)
' "$raw"
}

bridge_package_id_from_json() {
    local json="$1"
    echo "$json" | jq -r '
        (
            (.objectChanges // .object_changes // [])[]
            | select((.type // "") == "published")
            | .packageId // .package_id // empty
        ),
        (
            (.changed_objects // .changedObjects // [])[]
            | select((.objectType // .object_type // "") == "package")
            | .objectId // .object_id // empty
        )
    ' | head -n1
}

bridge_publish_token() {
    local pkg_path="$1"
    local -a cmd
    local attempt max="${MYSO_TX_RETRIES:-8}" rc=0 out json pkg active g
    active="$(resolve_myso_active_address || true)"
    for ((attempt = 1; attempt <= max; attempt++)); do
        if [[ -n "$active" ]]; then
            bridge_refresh_admin_caps_from_graphql "$active" || true
        fi
        cmd=(myso client publish "$pkg_path")
        if [[ -n "${PACKAGE_PUBLISH_ADMIN_CAP_ID:-}" ]]; then
            cmd+=(--publish-admin-cap "$(normalize_hex_id "$PACKAGE_PUBLISH_ADMIN_CAP_ID")")
        fi
        cmd+=(-e localnet --json)
        while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas_budget)
        echo "---" >&2
        printf ' %q\n' "${cmd[@]}" >&2
        echo "---" >&2
        rc=0
        out="$(run_with_timeout "${MYSO_CMD_TIMEOUT_SEC:-300}" "${cmd[@]}" 2>&1)" || rc=$?
        echo "$out" >&2
        if myso_output_is_executed_success "$out" || [[ "$rc" == 0 ]]; then
            json="$(bridge_extract_json "$out")" || json=''
            if [[ -n "$json" ]]; then
                BRIDGE_LAST_PUBLISH_JSON="$json"
                BRIDGE_LAST_PUBLISH_DIGEST="$(extract_tx_digest "$json" 2>/dev/null || extract_tx_digest "$out" 2>/dev/null || true)"
                pkg="$(bridge_package_id_from_json "$json")"
                if pkg="$(normalize_hex_id "$pkg" 2>/dev/null)"; then
                    printf '%s' "$pkg"
                    return 0
                fi
            fi
        fi
        if myso_output_is_transient_tx_error "$out"; then
            echo "Transient object conflict on publish (attempt ${attempt}/${max}); retrying in 3s" >&2
            sleep 3
            continue
        fi
        return "${rc:-1}"
    done
    return 1
}

bridge_init_and_register_token() {
    local pkg="$1" module="$2" type_name upgrade_cap treasury metadata out digest active publish_digest
    local attempt max="${MYSO_TX_RETRIES:-8}" rc=0
    active="$(resolve_myso_active_address)" || return 1
    bridge_refresh_admin_caps_from_graphql "$active" || {
        echo "CoinCreationAdminCap missing; cannot init $module" >&2
        return 1
    }
    [[ -n "${COIN_CREATION_ADMIN_CAP_ID:-}" ]] || {
        echo "CoinCreationAdminCap missing; cannot init $module" >&2
        return 1
    }
    for ((attempt = 1; attempt <= max; attempt++)); do
        bridge_refresh_admin_caps_from_graphql "$active" || true
        rc=0
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
            --move-call "${pkg}::${module}::init_coin" \
            "@$(normalize_hex_id "$COIN_CREATION_ADMIN_CAP_ID")")" || rc=$?
        if assert_tx_success "$out"; then
            break
        fi
        if myso_output_is_transient_tx_error "$out"; then
            log_step "Transient object conflict on init_coin $module (attempt ${attempt}/${max}); retrying"
            sleep 3
            continue
        fi
        return "${rc:-1}"
    done
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    treasury="$(bridge_extract_created_containing "$digest" "TreasuryCap" || true)"
    [[ -n "$treasury" ]] || treasury="$(extract_created_object_by_type "$digest" "TreasuryCap" 2>/dev/null || true)"
    metadata="$(bridge_extract_created_containing "$digest" "CoinMetadata" || true)"
    [[ -n "$metadata" ]] || metadata="$(extract_created_object_by_type "$digest" "CoinMetadata" 2>/dev/null || true)"
    publish_digest="${BRIDGE_LAST_PUBLISH_DIGEST:-}"
    upgrade_cap="$(bridge_extract_created_containing "$publish_digest" "UpgradeCap" 2>/dev/null || true)"
    [[ -n "$upgrade_cap" ]] || upgrade_cap="$(extract_created_object_by_type "$publish_digest" "UpgradeCap" 2>/dev/null || true)"
    if [[ -z "$upgrade_cap" && -n "${BRIDGE_LAST_PUBLISH_JSON:-}" ]]; then
        upgrade_cap="$(echo "$BRIDGE_LAST_PUBLISH_JSON" | jq -r '
            (.objectChanges // .object_changes // [])[]
            | select((.type // "") | test("created|Created"))
            | select(((.objectType // .object_type // "") | tostring) | contains("UpgradeCap"))
            | .objectId // .object_id // empty
        ' | head -n1)"
    fi
    if [[ -z "$upgrade_cap" ]]; then
        upgrade_cap="$(bridge_resolve_upgrade_cap_for_package "$pkg" "$active" 2>/dev/null || true)"
    fi
    [[ -n "$treasury" && -n "$metadata" && -n "$upgrade_cap" ]] || {
        echo "Missing treasury/metadata/upgrade cap after init_coin $module (tc=$treasury md=$metadata uc=$upgrade_cap)" >&2
        return 1
    }
    type_name="${pkg}::${module}::$(echo "$module" | tr '[:lower:]' '[:upper:]')"
    case "$module" in
        btc) type_name="${pkg}::btc::BTC" ;;
        eth) type_name="${pkg}::eth::ETH" ;;
        usdc) type_name="${pkg}::usdc::USDC" ;;
        usdt) type_name="${pkg}::usdt::USDT" ;;
        myusd) type_name="${pkg}::myusd::MYUSD" ;;
    esac
    log_step "register_foreign_token $type_name"
    for ((attempt = 1; attempt <= max; attempt++)); do
        upgrade_cap="$(bridge_resolve_upgrade_cap_for_package "$pkg" "$active" 2>/dev/null || true)"
        [[ -n "$upgrade_cap" ]] || upgrade_cap="$(bridge_extract_created_containing "$publish_digest" "UpgradeCap" 2>/dev/null || true)"
        [[ -n "$upgrade_cap" ]] || {
            echo "Could not resolve UpgradeCap for $type_name before register_foreign_token" >&2
            return 1
        }
        rc=0
        out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$active" \
            --move-call "${BRIDGE_PACKAGE_ID}::bridge::register_foreign_token" "<${type_name}>" \
            "@${BRIDGE_OBJECT_ID}" \
            "@$(normalize_hex_id "$treasury")" \
            "@$(normalize_hex_id "$upgrade_cap")" \
            "@$(normalize_hex_id "$metadata")")" || rc=$?
        if assert_tx_success "$out"; then
            printf '%s' "$type_name"
            return 0
        fi
        if echo "$out" | grep -qiE 'already|EUnsupportedTokenType|waiting'; then
            log_step "register_foreign_token $type_name already applied"
            printf '%s' "$type_name"
            return 0
        fi
        if myso_output_is_transient_tx_error "$out"; then
            log_step "Transient object conflict on register_foreign_token (attempt ${attempt}/${max}); refreshing caps"
            bridge_refresh_admin_caps_from_graphql "$active" || true
            sleep 3
            continue
        fi
        echo "register_foreign_token failed for $type_name" >&2
        return "${rc:-1}"
    done
    echo "register_foreign_token failed for $type_name after ${max} stale-object retries" >&2
    return 1
}

bridge_register_myso_tokens() {
    local dest pkg kind type_name saved_config="${MYSO_CONFIG_DIR:-}"
    if bridge_foreign_tokens_supported_on_chain; then
        log_step "Foreign tokens already in supported_tokens"
        TOKENS_REGISTERED_MYSO=1
        return 0
    fi
    if [[ -n "${BRIDGE_BTC_TYPE:-}" && -n "${BRIDGE_ETH_TYPE:-}" && -n "${BRIDGE_MYUSD_TYPE:-}" ]]; then
        BRIDGE_USDC_TYPE="${BRIDGE_MYUSD_TYPE}"
        BRIDGE_USDT_TYPE="${BRIDGE_MYUSD_TYPE}"
        log_step "Reusing published bridge token types from session"
        return 0
    fi
    bridge_refresh_session_from_graphql || {
        echo "Could not refresh admin caps from GraphQL before token publish" >&2
        return 1
    }
    if ! bridge_ensure_admin_caps_for_publish; then
        echo "Skipping foreign token publish: admin caps unavailable for the active keystore." >&2
        return 0
    fi
    if [[ -n "${BRIDGE_PUBLISH_MYSO_CONFIG_DIR:-}" ]]; then
        export MYSO_CONFIG_DIR="$BRIDGE_PUBLISH_MYSO_CONFIG_DIR"
        switch_wallet "${BRIDGE_PUBLISH_CAP_OWNER}"
    fi
    for kind in btc eth myusd; do
        type_name=''
        case "$kind" in
            btc) type_name="${BRIDGE_BTC_TYPE:-}" ;;
            eth) type_name="${BRIDGE_ETH_TYPE:-}" ;;
            myusd) type_name="${BRIDGE_MYUSD_TYPE:-}" ;;
        esac
        if [[ -n "$type_name" ]]; then
            pkg="${type_name%%::*}"
            if object_exists_on_fullnode "$pkg"; then
                log_step "Reusing published bridge $kind $type_name"
                continue
            fi
            echo "Clearing stale $kind type $type_name (package not on fullnode)" >&2
        fi
        dest="$(bridge_copy_token_pkg "$kind")" || return 1
        log_step "Publishing bridge $kind from $dest"
        pkg="$(bridge_publish_token "$dest")" || {
            echo "Publish failed for $kind" >&2
            return 1
        }
        type_name="$(bridge_init_and_register_token "$pkg" "$kind")" || return 1
        case "$kind" in
            btc) PKG_BRIDGE_BTC="$pkg"; BRIDGE_BTC_TYPE="$type_name" ;;
            eth) PKG_BRIDGE_ETH="$pkg"; BRIDGE_ETH_TYPE="$type_name" ;;
            myusd)
                PKG_BRIDGE_MYUSD="$pkg"
                BRIDGE_MYUSD_TYPE="$type_name"
                BRIDGE_USDC_TYPE="$type_name"
                BRIDGE_USDT_TYPE="$type_name"
                ;;
        esac
        log_session_use "PKG_BRIDGE_${kind}" "$pkg"
        bridge_save_session
        sleep "${BRIDGE_TOKEN_STEP_DELAY_SECS:-2}"
    done
    restore_wallet
    if [[ -n "$saved_config" ]]; then
        export MYSO_CONFIG_DIR="$saved_config"
    else
        unset MYSO_CONFIG_DIR
    fi
    BRIDGE_PUBLISH_MYSO_CONFIG_DIR=''
    BRIDGE_PUBLISH_CAP_OWNER=''
}

bridge_governance_add_tokens_myso() {
    local attempt
    [[ -n "${BRIDGE_BTC_TYPE:-}" ]] || {
        echo "No foreign token types; skipping add-tokens-on-myso" >&2
        return 0
    }
    if bridge_foreign_tokens_supported_on_chain; then
        log_step "add-tokens-on-myso already applied on-chain"
        TOKENS_REGISTERED_MYSO=1
        return 0
    fi
    TOKENS_REGISTERED_MYSO=''
    if ! bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        echo "Bridge node not reachable at $BRIDGE_AUTHORITY_URL — skip MySo governance (start the node, then re-run)." >&2
        return 0
    fi
    for attempt in 1 2; do
        log_step "Governance add-tokens-on-myso (foreign BTC/ETH/USDC/USDT) attempt ${attempt}"
        if bridge_cli governance --config-path "$BRIDGE_CLIENT_CONFIG_PATH" --chain-id "$BRIDGE_MYSO_CHAIN_ID" \
            add-tokens-on-myso \
            --nonce 0 \
            --token-ids 1,2,3,4 \
            --token-type-names "${BRIDGE_BTC_TYPE},${BRIDGE_ETH_TYPE},${BRIDGE_MYUSD_TYPE:-$BRIDGE_USDC_TYPE},${BRIDGE_MYUSD_TYPE:-$BRIDGE_USDT_TYPE}" \
            --token-prices "${BRIDGE_TOKEN_PRICE_BTC},${BRIDGE_TOKEN_PRICE_ETH},${BRIDGE_TOKEN_PRICE_USDC},${BRIDGE_TOKEN_PRICE_USDT}"; then
            if bridge_foreign_tokens_supported_on_chain; then
                TOKENS_REGISTERED_MYSO=1
                return 0
            fi
        fi
        echo "CLI add-tokens-on-myso did not land; trying node signature + execute_system_message" >&2
        if bridge_add_tokens_on_myso_via_node && bridge_foreign_tokens_supported_on_chain; then
            TOKENS_REGISTERED_MYSO=1
            return 0
        fi
        echo "add-tokens-on-myso did not land supported_tokens; ensuring waiting_room then retrying" >&2
        bridge_ensure_waiting_room_tokens
    done
    if bridge_foreign_tokens_supported_on_chain; then
        TOKENS_REGISTERED_MYSO=1
        return 0
    fi
    echo "add-tokens-on-myso failed (node must have matching approved-governance-actions). Re-write configs and restart the node, then retry." >&2
    return 1
}

bridge_governance_add_tokens_evm() {
    if [[ "${TOKENS_REGISTERED_EVM:-}" == 1 ]]; then
        return 0
    fi
    [[ -n "${BRIDGE_PROXY:-}" ]] || {
        echo "No EVM proxy; skipping add-tokens-on-evm" >&2
        return 0
    }
    if ! bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        echo "Bridge node not reachable — skip EVM governance." >&2
        return 0
    fi
    local t0 t1 t2 t3 t4
    t0='0x0000000000000000000000000000000000000000'
    t1="${BRIDGE_BTC:-$t0}"
    t2="${BRIDGE_WETH:-$t0}"
    t3="${BRIDGE_USDC:-$t0}"
    t4="${BRIDGE_USDT:-$t0}"
    log_step "Governance add-tokens-on-evm"
    if bridge_cli governance --config-path "$BRIDGE_CLIENT_CONFIG_PATH" --chain-id "$BRIDGE_ETH_CHAIN_ID" \
        add-tokens-on-evm \
        --nonce 0 \
        --token-ids 0,1,2,3,4 \
        --token-type-names "${t0},${t1},${t2},${t3},${t4}" \
        --token-prices 12800,432518900,25969600,10000,10000 \
        --token-myso-decimals 9,8,8,6,6; then
        TOKENS_REGISTERED_EVM=1
    else
        echo "add-tokens-on-evm failed or tokens already present on the EVM config (local forge deploy registers them)." >&2
        TOKENS_REGISTERED_EVM=1
        return 0
    fi
}

bridge_demo_transfer() {
    local active coin pair out
    if ! bridge_node_reachable "$BRIDGE_AUTHORITY_URL"; then
        echo "Demo requires a running bridge node at $BRIDGE_AUTHORITY_URL" >&2
        return 1
    fi
    [[ -n "${BRIDGE_CLIENT_ETH_ADDRESS:-}" ]] || {
        echo "Client ETH address missing" >&2
        return 1
    }
    active="$(resolve_myso_active_address)" || return 1
    ensure_two_gas_coins_for_address "$active" || {
        SKIP_CONFIRM_RUN=1 invoke_ptb_as "$active" --split-coins gas "[${GAS_BUDGET}]" >/dev/null || true
    }
    pair="$(bridge_pick_largest_myso_coin "$active")" || return 1
    read -r coin _ <<<"$pair"
    log_step "Demo deposit-on-myso $coin -> ${BRIDGE_CLIENT_ETH_ADDRESS} (chain 12)"
    bridge_cli client --config-path "$BRIDGE_CLIENT_CONFIG_PATH" deposit-on-myso \
        --coin-object-id "$coin" \
        --coin-type '0x2::myso::MYSO' \
        --target-chain 12 \
        --recipient-address "$BRIDGE_CLIENT_ETH_ADDRESS" || return 1
    log_step "Demo claim-on-eth --seq-num 0 (adjust if the deposit used a later nonce)"
    bridge_cli client --config-path "$BRIDGE_CLIENT_CONFIG_PATH" claim-on-eth \
        --seq-num 0 --dry-run false || true
}

bridge_print_summary() {
    echo "" >&2
    echo "Bridge attach-only bootstrap summary" >&2
    echo "  session:        $SOCIAL_SESSION_SAVE_PATH" >&2
    echo "  authority myso: ${BRIDGE_AUTHORITY_MYSO_ADDRESS:-<unset>}" >&2
    echo "  authority eth:  ${BRIDGE_AUTHORITY_ETH_ADDRESS:-<unset>}" >&2
    echo "  node config:    ${BRIDGE_NODE_CONFIG_PATH:-<unset>}" >&2
    echo "  client config:  ${BRIDGE_CLIENT_CONFIG_PATH:-<unset>}" >&2
    echo "  authority url:  ${BRIDGE_AUTHORITY_URL:-<unset>}" >&2
    echo "  evm proxy:      ${BRIDGE_PROXY:-<not deployed>}" >&2
    echo "  native myso:    ${NATIVE_MYSO_BOOTSTRAPPED:-0}" >&2
    echo "  committee:      registered=${COMMITTEE_REGISTERED:-0} finalized=${COMMITTEE_FINALIZED:-0}" >&2
    echo "  myso tokens:    registered=${TOKENS_REGISTERED_MYSO:-0}" >&2
}
