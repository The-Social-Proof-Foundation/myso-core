#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Publish MyUSD/BTC/ETH, create three MyUSD-quoted orderbook pools, and register
# the assets + pools in the local orderbook catalog (localhost:9008).
#
# All object IDs, package IDs, cap IDs, coin types, and pool IDs are discovered
# automatically from GraphQL + the active wallet. Do not paste addresses.
# Optional env vars (PKG_MYUSD, ORDERBOOK_REGISTRY_ID, …) are debug overrides only.
# ORDERBOOK_PUBLISH_ENV=localnet (default) always passes -e localnet on publish.
# ORDERBOOK_CLEAR_PUBLICATION=1 (default) deletes Published.toml / Pub.*.toml before each publish.
# ORDERBOOK_MANAGE_MOVE_TOML=1 (default) temporarily strips explicit MoveStdlib/MySo deps and
# ensures a [environments] localnet slot, then restores the original Move.toml.
#
# Prerequisites:
#   - myso start --with-indexer --with-orderbook
#   - ./scripts/bootstrap.sh completed (caps owned by the active address)
#   - myso, curl, jq, python3 on PATH
#
# Session (write-only output): network.config/orderbook/orderbook-session.env
#
# Usage:
#   ./scripts/orderbook-bootstrap.sh
#   ./scripts/orderbook-bootstrap.sh --refresh-session
#   ./scripts/orderbook-bootstrap.sh --skip-mint
#   ./scripts/orderbook-bootstrap.sh --skip-mm
#   ./scripts/orderbook-bootstrap.sh --skip-oracle
#   ./scripts/orderbook-bootstrap.sh --e2e-test
#   ASSUME_YES=1 ./scripts/orderbook-bootstrap.sh
#   ./scripts/orderbook-bootstrap.sh -y
#
# MYSO/MYUSD localnet mid is live from MYSO HTTP oracle + Pyth (no static seed by default).
# ORDERBOOK_ORACLE_SEED_STATIC=1 + MM_ALLOW_FALLBACK=1 = offline fake prices only.
# ORDERBOOK_MYSO_RESEED=1 (default) cancels leftover MYSO/MYUSD book orders before MM start.
# After MM is ready, bootstrap runs a BTC/MYUSD market buy demo via orderbook-demo-btc-trades.ts.
# ORDERBOOK_DEMO_ROUND_TRIP=1 adds sell leg; ORDERBOOK_SKIP_DEMO_TRADE=1 skips demo.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="${ORDERBOOK_SESSION_SAVE_PATH:-$REPO_ROOT/network.config/orderbook/orderbook-session.env}"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/orderbook-bootstrap-common.sh
source "${SCRIPT_DIR}/lib/orderbook-bootstrap-common.sh"
# shellcheck source=lib/orderbook-mm-supervisor.sh
source "${SCRIPT_DIR}/lib/orderbook-mm-supervisor.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

SKIP_CONFIRM_RUN=1
ASSUME_YES="${ASSUME_YES:-1}"
DO_REFRESH=0
SKIP_MINT=0
SKIP_MM=0
SKIP_ORACLE=0
E2E_TEST=0

usage() {
    sed -n '2,24p' "$0" | sed 's/^# \?//'
}

ensure_token_published() {
    local kind="$1"
    local pkg_var="$2"
    local type_var="$3"
    local module_path="$4"
    local pkg_dir="$5"
    local pkg
    if [[ -n "${!pkg_var:-}" ]] && object_exists_on_fullnode "${!pkg_var}"; then
        log_step "Reusing published $kind package ${!pkg_var}"
        printf -v "$type_var" '%s::%s' "$(normalize_hex_id "${!pkg_var}")" "$module_path"
        return 0
    fi
    log_step "Publishing $kind from $pkg_dir"
    pkg="$(orderbook_publish_token_with_retry "$pkg_dir")" || {
        echo "Failed to publish $kind" >&2
        return 1
    }
    printf -v "$pkg_var" '%s' "$pkg"
    printf -v "$type_var" '%s::%s' "$pkg" "$module_path"
    log_session_use "$pkg_var" "$pkg"
    log_session_use "$type_var" "${!type_var}"
    orderbook_save_session
}

ensure_token_initialized() {
    local kind="$1"
    local treasury_var="$2"
    shift 2
    local cap
    if [[ -n "${!treasury_var:-}" ]] && object_exists_on_fullnode "${!treasury_var}"; then
        log_step "Reusing $kind TreasuryCap ${!treasury_var}"
        return 0
    fi
    log_step "Initializing $kind coin"
    cap="$("$@")" || {
        echo "Failed to init $kind" >&2
        return 1
    }
    printf -v "$treasury_var" '%s' "$cap"
    log_session_use "$treasury_var" "$cap"
    orderbook_save_session
}

ensure_pool() {
    local name="$1"
    local id_var="$2"
    local base_type="$3"
    local tick="$4"
    local pool catalog_pool
    if [[ -n "${!id_var:-}" ]] && object_exists_on_fullnode "${!id_var}"; then
        if orderbook_pool_is_shared "${!id_var}"; then
            log_step "Reusing pool $name ${!id_var}"
            return 0
        fi
        log_step "Pool $name ${!id_var} exists but is not a shared Pool object — MM will skip it"
        return 0
    fi
    if catalog_pool="$(orderbook_catalog_pool_id_for_base "$base_type" 2>/dev/null)" \
        && [[ -n "$catalog_pool" ]] \
        && object_exists_on_fullnode "$catalog_pool"; then
        printf -v "$id_var" '%s' "$catalog_pool"
        log_session_use "$id_var" "$catalog_pool"
        orderbook_save_session
        if orderbook_pool_is_shared "$catalog_pool"; then
            log_step "Reusing catalog pool $name $catalog_pool"
            return 0
        fi
        log_step "Catalog pool $name $catalog_pool is not shared — MM will skip it"
        return 0
    fi
    log_step "Creating pool $name ($base_type / $MYUSD_COIN_TYPE)"
    pool="$(orderbook_create_pool "$base_type" "$tick" "$LOT_SIZE" "$MIN_SIZE")" || {
        if catalog_pool="$(orderbook_catalog_pool_id_for_base "$base_type" 2>/dev/null)" \
            && [[ -n "$catalog_pool" ]]; then
            printf -v "$id_var" '%s' "$catalog_pool"
            log_session_use "$id_var" "$catalog_pool"
            orderbook_save_session
            log_step "Pool $name already registered in catalog ($catalog_pool)"
            return 0
        fi
        echo "Failed to create pool $name" >&2
        return 1
    }
    printf -v "$id_var" '%s' "$pool"
    log_session_use "$id_var" "$pool"
    orderbook_save_session
}

run_bootstrap() {
    local active
    active="$(resolve_myso_active_address)" || {
        echo "Could not resolve active address" >&2
        return 1
    }
    active="$(normalize_hex_id "$active")" || return 1

    log_step "Faucet"
    myso client faucet || true

    orderbook_load_session
    orderbook_refresh_session_from_graphql || return 1
    orderbook_validate_session_ids
    require_session_fields ORDERBOOK_REGISTRY_ID ORDERBOOK_ADMIN_CAP_ID \
        COIN_CREATION_ADMIN_CAP_ID PACKAGE_PUBLISH_ADMIN_CAP_ID \
        ORDERBOOK_PACKAGE_ID CLOCK_ID || return 1

    if ! orderbook_admin_health; then
        echo "Orderbook admin API not reachable at ${ORDERBOOK_API_URL}/admin/health" >&2
        echo "Start localnet with: myso start --with-indexer --with-orderbook" >&2
        return 1
    fi

    ensure_token_published MYUSD PKG_MYUSD MYUSD_COIN_TYPE "myusd::MYUSD" \
        "$REPO_ROOT/bridge/move/tokens/myusd" || return 1
    ensure_token_published BTC PKG_BTC BTC_COIN_TYPE "btc::BTC" \
        "$REPO_ROOT/bridge/move/tokens/btc" || return 1
    ensure_token_published ETH PKG_ETH ETH_COIN_TYPE "eth::ETH" \
        "$REPO_ROOT/bridge/move/tokens/eth" || return 1

    ensure_token_initialized MYUSD MYUSD_TREASURY_CAP_ID \
        orderbook_init_myusd "$PKG_MYUSD" "$active" || return 1
    ensure_token_initialized BTC BTC_TREASURY_CAP_ID \
        orderbook_init_bridged_token "$PKG_BTC" btc || return 1
    ensure_token_initialized ETH ETH_TREASURY_CAP_ID \
        orderbook_init_bridged_token "$PKG_ETH" eth || return 1

    if [[ "$SKIP_MINT" != 1 ]]; then
        log_step "Minting test supply"
        orderbook_mint_token "$MYUSD_COIN_TYPE" "$MYUSD_TREASURY_CAP_ID" "$MYUSD_MINT_AMOUNT" "$active" || return 1
        orderbook_mint_token "$BTC_COIN_TYPE" "$BTC_TREASURY_CAP_ID" "$BTC_MINT_AMOUNT" "$active" || return 1
        orderbook_mint_token "$ETH_COIN_TYPE" "$ETH_TREASURY_CAP_ID" "$ETH_MINT_AMOUNT" "$active" || return 1
        orderbook_save_session
    else
        log_step "Skipping mint (--skip-mint)"
    fi

    log_step "Registering MyUSD as orderbook stablecoin"
    orderbook_add_myusd_stablecoin || return 1
    orderbook_save_session

    ensure_pool MYSO_MYUSD MYSO_MYUSD_POOL_ID "$MYSO_COIN_TYPE" "$TICK_SIZE_MYSO" || return 1
    orderbook_ensure_mysousd_tick || return 1
    ensure_pool BTC_MYUSD BTC_MYUSD_POOL_ID "$BTC_COIN_TYPE" "$TICK_SIZE_BTC" || return 1
    ensure_pool ETH_MYUSD ETH_MYUSD_POOL_ID "$ETH_COIN_TYPE" "$TICK_SIZE_ETH" || return 1

    log_step "Registering assets and pools in orderbook catalog"
    orderbook_register_assets || return 1
    orderbook_register_pools || return 1
    orderbook_save_session

    if [[ "$SKIP_MM" == 1 ]]; then
        log_step "Skipping Pyth / oracle / MM (--skip-mm)"
        print_run_summary_header "Orderbook bootstrap complete (catalog only)"
        print_run_summary_line "Active address" "$active"
        print_run_summary_line "Catalog API" "$ORDERBOOK_API_URL"
        print_run_summary_line "Session" "$SOCIAL_SESSION_SAVE_PATH"
        print_run_summary_footer
        return 0
    fi

    orderbook_run_pyth_setup || return 1
    if [[ -n "${ORACLE_ADDRESS:-}" ]]; then
        log_step "Funding oracle wallet gas ($ORACLE_ADDRESS)"
        orderbook_fund_address "$ORACLE_ADDRESS" 300000000 || return 1
    fi
    orderbook_ensure_demo_trade_funds || return 1
    orderbook_fetch_mm_pools_json >/dev/null || return 1

    print_run_summary_header "Orderbook bootstrap complete"
    print_run_summary_line "Active address" "$active"
    print_run_summary_line "Registry" "$ORDERBOOK_REGISTRY_ID"
    print_run_summary_line "MYUSD package" "$PKG_MYUSD"
    print_run_summary_line "MYUSD type" "$MYUSD_COIN_TYPE"
    print_run_summary_line "MYUSD treasury" "$MYUSD_TREASURY_CAP_ID"
    print_run_summary_line "BTC package" "$PKG_BTC"
    print_run_summary_line "BTC type" "$BTC_COIN_TYPE"
    print_run_summary_line "ETH package" "$PKG_ETH"
    print_run_summary_line "ETH type" "$ETH_COIN_TYPE"
    print_run_summary_line "MYSO_MYUSD pool" "$MYSO_MYUSD_POOL_ID"
    print_run_summary_line "BTC_MYUSD pool" "$BTC_MYUSD_POOL_ID"
    print_run_summary_line "ETH_MYUSD pool" "$ETH_MYUSD_POOL_ID"
    print_run_summary_line "Pyth package" "$PYTH_PACKAGE_ID"
    print_run_summary_line "Oracle PIO (BTC)" "$BTC_PRICE_INFO_OBJECT_ID"
    print_run_summary_line "MM pools" "$(echo "$MM_POOLS" | jq -r 'map(.poolId) | join(", ")')"
    print_run_summary_line "Catalog API" "$ORDERBOOK_API_URL"
    print_run_summary_line "Session" "$SOCIAL_SESSION_SAVE_PATH"
    print_run_summary_footer

    if [[ "$E2E_TEST" == 1 ]]; then
        ORDERBOOK_MM_BACKGROUND=1
        orderbook_mm_supervisor_run "$SKIP_ORACLE" 1 || return 1
        log_step "E2E test passed — oracle PID=${ORACLE_PID:-} MM PID=${MM_PID:-} still running"
        return 0
    fi

    orderbook_mm_supervisor_run "$SKIP_ORACLE" 0 || return 1
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --refresh-session)
            DO_REFRESH=1
            shift
            ;;
        --skip-mint)
            SKIP_MINT=1
            shift
            ;;
        --skip-mm)
            SKIP_MM=1
            shift
            ;;
        --skip-oracle)
            SKIP_ORACLE=1
            shift
            ;;
        --e2e-test)
            E2E_TEST=1
            ORDERBOOK_MM_BACKGROUND=1
            shift
            ;;
        -y|--yes)
            ASSUME_YES=1
            SKIP_CONFIRM_RUN=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage
            exit 1
            ;;
    esac
done

# --refresh-session is the default path (GraphQL always runs). The flag forces
# a re-validate of any previously saved artifact IDs against fullnode.
if [[ "$DO_REFRESH" == 1 && -f "$SOCIAL_SESSION_SAVE_PATH" ]]; then
    log_step "Forced session re-validation (--refresh-session)"
fi

run_bootstrap
