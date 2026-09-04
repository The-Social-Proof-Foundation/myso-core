#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Foreground supervisor: oracle-service (background) + market-maker (foreground).
# Soft landing on SIGINT/SIGTERM — MM cancels resting orders, oracle stops.

if [[ -n "${_ORDERBOOK_MM_SUPERVISOR_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_ORDERBOOK_MM_SUPERVISOR_SOURCED=1

ORDERBOOK_SANDBOX_DIR="${ORDERBOOK_SANDBOX_DIR:-$REPO_ROOT/../orderbook-sandbox-main/sandbox}"
ORACLE_STATUS_PORT="${ORACLE_STATUS_PORT:-9010}"
MM_HEALTH_PORT="${MM_HEALTH_CHECK_PORT:-3012}"
ORDERBOOK_MM_BACKGROUND="${ORDERBOOK_MM_BACKGROUND:-0}"
ORDERBOOK_MYSO_RESEED="${ORDERBOOK_MYSO_RESEED:-1}"

ORACLE_PID=''
MM_PID=''
SUPERVISOR_SHUTDOWN=0

orderbook_sandbox_has_pnpm() {
    [[ -d "$ORDERBOOK_SANDBOX_DIR/node_modules" ]] || {
        echo "Missing sandbox node_modules — run: (cd \"$ORDERBOOK_SANDBOX_DIR\" && pnpm install)" >&2
        return 1
    }
    command -v pnpm >/dev/null 2>&1 || {
        echo "pnpm required for oracle-service / market-maker" >&2
        return 1
    }
}

orderbook_load_pyth_api_key() {
    if [[ -n "${PYTH_API_KEY:-}" ]]; then
        return 0
    fi
    local env_file="$ORDERBOOK_SANDBOX_DIR/.env"
    if [[ -f "$env_file" ]]; then
        PYTH_API_KEY="$(grep -E '^PYTH_API_KEY=' "$env_file" | head -1 | cut -d= -f2- | tr -d '"' | tr -d "'")"
    fi
    if [[ -z "${PYTH_API_KEY:-}" ]]; then
        echo "PYTH_API_KEY required for live oracle (set env or add to $env_file)" >&2
        return 1
    fi
    export PYTH_API_KEY
}

orderbook_wait_oracle_ready() {
    local attempt max="${1:-20}"
    for ((attempt = 1; attempt <= max; attempt++)); do
        if curl -sf --max-time 2 "http://127.0.0.1:${ORACLE_STATUS_PORT}/" 2>/dev/null \
            | jq -e '.status == "ok"' >/dev/null 2>&1; then
            log_step "Oracle service listening (${ORACLE_STATUS_PORT})"
            return 0
        fi
        [[ "$attempt" == 1 || $((attempt % 5)) -eq 0 ]] \
            && log_wait_progress "oracle service" "$attempt" "$max"
        sleep 1
    done
    echo "Timed out waiting for oracle service on :${ORACLE_STATUS_PORT}" >&2
    return 1
}

orderbook_wait_oracle_live() {
    local attempt max="${1:-25}" btc_raw btc_num status_json updates
    for ((attempt = 1; attempt <= max; attempt++)); do
        status_json="$(curl -sf --max-time 2 "http://127.0.0.1:${ORACLE_STATUS_PORT}/" 2>/dev/null)" || status_json=''
        btc_raw="$(jq -r '.prices.btc // empty' <<<"$status_json" 2>/dev/null)" || btc_raw=''
        updates="$(jq -r '.updates // 0' <<<"$status_json" 2>/dev/null)" || updates='0'
        if [[ -n "$btc_raw" && "$updates" -ge 1 ]]; then
            btc_num="${btc_raw#\$}"
            if awk -v p="$btc_num" 'BEGIN { exit !(p+0 > 50000) }'; then
                log_step "Live oracle BTC price: ${btc_raw} (updates=${updates})"
                return 0
            fi
        fi
        [[ "$attempt" == 1 || $((attempt % 5)) -eq 0 ]] \
            && log_wait_progress "live oracle BTC price" "$attempt" "$max"
        sleep 1
    done
    echo "Timed out waiting for live oracle update on :${ORACLE_STATUS_PORT} (need updates>=1 and BTC > \$50,000)" >&2
    return 1
}

orderbook_seed_oracle_prices() {
    local out_file rc=0
    orderbook_sandbox_has_pnpm || return 1
    out_file="$(mktemp)"
    log_step "Seeding on-chain oracle prices (OFFLINE — ORDERBOOK_ORACLE_SEED_STATIC=1)"
    (
        cd "$ORDERBOOK_SANDBOX_DIR"
        export ORDERBOOK_ORACLE_SEED_STATIC=1 \
            PYTH_PACKAGE_ID ORACLE_PRIVATE_KEY \
            MYUSD_PRICE_INFO_OBJECT_ID MYSO_PRICE_INFO_OBJECT_ID \
            BTC_PRICE_INFO_OBJECT_ID ETH_PRICE_INFO_OBJECT_ID \
            RPC_URL="${RPC_URL:-http://127.0.0.1:9000}"
        pnpm tsx scripts/orderbook-seed-oracle-prices.ts
    ) >"$out_file" || rc=$?
    if [[ "$rc" != 0 ]]; then
        cat "$out_file" >&2
        rm -f "$out_file"
        return 1
    fi
    grep -q 'ORDERBOOK_ORACLE_SEEDED=1' "$out_file" || {
        cat "$out_file" >&2
        rm -f "$out_file"
        return 1
    }
    rm -f "$out_file"
}

orderbook_wait_mm_ready() {
    local attempt max="${1:-45}"
    for ((attempt = 1; attempt <= max; attempt++)); do
        if curl -sf --max-time 2 "http://127.0.0.1:${MM_HEALTH_PORT}/ready" 2>/dev/null \
            | jq -e '.ready == true' >/dev/null 2>&1; then
            log_step "Market maker ready (${MM_HEALTH_PORT})"
            return 0
        fi
        [[ "$attempt" == 1 || $((attempt % 5)) -eq 0 ]] \
            && log_wait_progress "market maker readiness" "$attempt" "$max"
        sleep 1
    done
    echo "Timed out waiting for market maker on :${MM_HEALTH_PORT}/ready" >&2
    return 1
}

orderbook_mm_supervisor_cleanup() {
    local sig="${1:-TERM}"
    [[ "$SUPERVISOR_SHUTDOWN" == 1 ]] && return 0
    SUPERVISOR_SHUTDOWN=1
    log_step "Shutting down orderbook MM supervisor (signal ${sig})"
    if [[ -n "$MM_PID" ]] && kill -0 "$MM_PID" 2>/dev/null; then
        kill -"$sig" "$MM_PID" 2>/dev/null || true
    fi
    if [[ -n "$ORACLE_PID" ]] && kill -0 "$ORACLE_PID" 2>/dev/null; then
        kill -TERM "$ORACLE_PID" 2>/dev/null || true
    fi
    [[ -n "$MM_PID" ]] && wait "$MM_PID" 2>/dev/null || true
    [[ -n "$ORACLE_PID" ]] && wait "$ORACLE_PID" 2>/dev/null || true
    ORACLE_PID=''
    MM_PID=''
}

orderbook_btc_ask_on_catalog() {
    local ask
    ask="$(curl -sf --max-time 2 "${ORDERBOOK_API_URL}/summary" 2>/dev/null \
        | jq -r '.[] | select(.trading_pairs == "BTC_MYUSD") | .lowest_ask // empty')" || ask=''
    [[ -n "$ask" ]] && awk -v p="$ask" 'BEGIN { exit !(p+0 > 0) }'
}

orderbook_wait_mm_liquidity() {
    local attempt max="${1:-15}"
    for ((attempt = 1; attempt <= max; attempt++)); do
        if curl -sf --max-time 2 "http://127.0.0.1:${MM_HEALTH_PORT}/orders" 2>/dev/null \
            | jq -e '.pools[]? | select(.pair | test("BTC")) | select((.orders | length) > 0)' >/dev/null 2>&1; then
            log_step "MM liquidity detected on BTC/MYUSD"
            return 0
        fi
        if curl -sf --max-time 2 "http://127.0.0.1:${MM_HEALTH_PORT}/health" 2>/dev/null \
            | jq -e '.pools.BTC_MYUSD.orders? // .pools["BTC/MYUSD"].orders? | tonumber > 0' >/dev/null 2>&1; then
            log_step "MM health reports BTC/MYUSD orders"
            return 0
        fi
        if orderbook_btc_ask_on_catalog; then
            log_step "BTC/MYUSD ask on catalog (proceeding with demo)"
            return 0
        fi
        [[ "$attempt" == 1 || $((attempt % 5)) -eq 0 ]] \
            && log_wait_progress "MM BTC liquidity" "$attempt" "$max"
        sleep 1
    done
    echo "Timed out waiting for BTC/MYUSD ask liquidity (${max}s)" >&2
    return 1
}

orderbook_run_test_trade() {
    orderbook_run_demo_btc_trades
}

orderbook_export_btc_trade_env() {
    export ORDERBOOK_PACKAGE_ID BTC_MYUSD_POOL_ID BTC_COIN_TYPE MYUSD_COIN_TYPE \
        RPC_URL="${RPC_URL:-http://127.0.0.1:9000}" \
        BTC_DEMO_QUANTITY_BASE="${BTC_DEMO_QUANTITY_BASE:-1000}" \
        MYUSD_DEPOSIT="${MYUSD_DEPOSIT:-50000000}" \
        MIN_SIZE="${MIN_SIZE:-1000}"
    # Session pool IDs beat stale MM_POOLS / coin types in sandbox .env.
    export MM_POOLS=
    export PRIVATE_KEY="${TEST_TRADER_PRIVATE_KEY:?TEST_TRADER_PRIVATE_KEY required for spot demo}"
}

orderbook_run_demo_btc_trades() {
    local out_file rc=0
    orderbook_sandbox_has_pnpm || return 1
    out_file="$(mktemp)"
    log_step "BTC/MYUSD spot demo: market buy (min lot)"
    (
        cd "$ORDERBOOK_SANDBOX_DIR"
        orderbook_export_btc_trade_env
        pnpm tsx scripts/orderbook-demo-btc-trades.ts >"$out_file"
    ) || rc=$?
    if [[ "$rc" == 0 ]] && grep -q 'ORDERBOOK_DEMO_TRADES_OK=1' "$out_file" 2>/dev/null; then
        log_step "Spot demo trade succeeded (market buy)"
        rm -f "$out_file"
        return 0
    fi
    echo "Spot demo trades failed:" >&2
    cat "$out_file" >&2 || true
    rm -f "$out_file"
    return 1
}

orderbook_maybe_run_btc_spot_demo() {
    local strict="${1:-0}"
    local liq_wait="${ORDERBOOK_DEMO_LIQ_WAIT:-}"
    [[ -z "$liq_wait" ]] && liq_wait="$([[ "$strict" == 1 ]] && echo 30 || echo 15)"
    [[ "${ORDERBOOK_SKIP_DEMO_TRADE:-0}" == 1 ]] && return 0
    orderbook_wait_mm_liquidity "$liq_wait" || {
        [[ "$strict" == 1 ]] && return 1
        log_step "Skipping spot demo — no BTC ask liquidity (${liq_wait}s)"
        return 0
    }
    sleep "${ORDERBOOK_DEMO_SETTLE_SECS:-2}"
    orderbook_run_demo_btc_trades || {
        [[ "$strict" == 1 ]] && return 1
        log_step "Spot demo failed (non-fatal — MM still running)"
        return 0
    }
    return 0
}

orderbook_mm_supervisor_run() {
    local skip_oracle="${1:-0}" run_test="${2:-0}" background="${ORDERBOOK_MM_BACKGROUND:-0}"

    orderbook_sandbox_has_pnpm || return 1
    require_session_fields ORDERBOOK_PACKAGE_ID PYTH_PACKAGE_ID \
        MYUSD_PRICE_INFO_OBJECT_ID MYSO_PRICE_INFO_OBJECT_ID \
        BTC_PRICE_INFO_OBJECT_ID ETH_PRICE_INFO_OBJECT_ID \
        ORACLE_PRIVATE_KEY PRIVATE_KEY MM_POOLS DEPLOYER_ADDRESS || return 1

    trap 'orderbook_mm_supervisor_cleanup TERM' INT TERM

    if [[ "$skip_oracle" != 1 ]]; then
        if [[ "${ORDERBOOK_ORACLE_SEED_STATIC:-0}" == 1 ]]; then
            orderbook_seed_oracle_prices || return 1
        else
            orderbook_load_pyth_api_key || return 1
            # Cancel stale ~9700 BTC / ~1 MYSO grids before oracle+MM share the deployer gas coin.
            orderbook_clear_stale_mm_book_orders || true
        fi
        log_step "Starting oracle-service (background, live HTTP updates)"
        (
            cd "$ORDERBOOK_SANDBOX_DIR"
            export PYTH_PACKAGE_ID MYUSD_PRICE_INFO_OBJECT_ID MYSO_PRICE_INFO_OBJECT_ID \
                BTC_PRICE_INFO_OBJECT_ID ETH_PRICE_INFO_OBJECT_ID ORACLE_PRIVATE_KEY \
            ORACLE_STATUS_PORT \
            ORDERBOOK_ORACLE_HTTP_UPDATES="${ORDERBOOK_ORACLE_HTTP_UPDATES:-1}" \
            ORACLE_UPDATE_INTERVAL_MS="${ORACLE_UPDATE_INTERVAL_MS:-15000}" \
            PYTH_API_KEY \
                MYSO_ORACLE_SOURCE_URL="${MYSO_ORACLE_SOURCE_URL:-}" \
                RPC_URL="${RPC_URL:-http://127.0.0.1:9000}"
            exec pnpm exec tsx scripts/oracle-service/index.ts
        ) &
        ORACLE_PID=$!
        orderbook_wait_oracle_ready 30 || {
            orderbook_mm_supervisor_cleanup TERM
            return 1
        }
        if [[ "${ORDERBOOK_ORACLE_HTTP_UPDATES:-1}" != 0 ]]; then
            orderbook_wait_oracle_live 25 || {
                orderbook_mm_supervisor_cleanup TERM
                return 1
            }
        fi
    fi

    sleep 5

    log_step "Starting market-maker"
    (
        cd "$ORDERBOOK_SANDBOX_DIR"
        export ORDERBOOK_PACKAGE_ID PYTH_PACKAGE_ID PRIVATE_KEY MM_POOLS DEPLOYER_ADDRESS \
            MM_LEVELS_PER_SIDE="${MM_LEVELS_PER_SIDE:-3}" \
            MM_REBALANCE_INTERVAL_MS="${MM_REBALANCE_INTERVAL_MS:-10000}" \
            MM_HEALTH_CHECK_PORT="$MM_HEALTH_PORT" \
            MM_FORCE_NEW_BALANCE_MANAGERS="${MM_FORCE_NEW_BALANCE_MANAGERS:-}" \
            RPC_URL="${RPC_URL:-http://127.0.0.1:9000}"
        if [[ "$background" == 1 ]]; then
            exec pnpm market-maker
        else
            exec pnpm market-maker
        fi
    ) &
    MM_PID=$!

    orderbook_wait_mm_ready 45 || {
        orderbook_mm_supervisor_cleanup TERM
        return 1
    }

    orderbook_maybe_run_btc_spot_demo "$run_test" || {
        orderbook_mm_supervisor_cleanup TERM
        return 1
    }

    if [[ "$run_test" == 1 ]]; then
        orderbook_verify_live_prices || {
            orderbook_mm_supervisor_cleanup TERM
            return 1
        }
        if orderbook_assert_ticker_last_price "BTC_MYUSD" 20; then
            log_step "Indexer ticker reflects demo trade"
        else
            log_step "Ticker not indexed yet (indexer lag) — on-chain demo trade already succeeded"
        fi
        log_step "E2E test passed — oracle PID=${ORACLE_PID:-} MM PID=${MM_PID:-} still running"
    fi

    if [[ "$background" == 1 ]]; then
        # Survive bootstrap script exit (avoid SIGHUP to oracle/MM children).
        [[ -n "$ORACLE_PID" ]] && disown -h "$ORACLE_PID" 2>/dev/null || true
        [[ -n "$MM_PID" ]] && disown -h "$MM_PID" 2>/dev/null || true
        log_step "Oracle PID=${ORACLE_PID} MM PID=${MM_PID} (background, disowned)"
        trap - INT TERM
        return 0
    fi

    log_step "Oracle + market maker running — Ctrl+C to stop (MM cancel-all, then oracle exit)"
    wait "$MM_PID" || true
    orderbook_mm_supervisor_cleanup TERM
    trap - INT TERM
}
