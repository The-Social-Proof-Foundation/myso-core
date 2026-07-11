#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../mydata-marketplace-runnable.sh
source "$SCRIPT_DIR/mydata-marketplace-runnable.sh"

MYDATA_CONFIG_ID=cfg
BLOCK_LIST_REGISTRY_ID=blocklist
MEMORY_CONFIG_ID=memory-config
ECOSYSTEM_TREASURY_ID=treasury
POOL_ADMIN_CAP_ID=pool-cap
POOL_REGISTRY_ID=pool-registry
ANCHOR_REGISTRY_ID=anchor-registry
CLAIM_VAULT_ID=claim-vault
DIST_REGISTRY_ID=distribution-registry
PLATFORM_OBJECT_ID=platform
CLOCK_ID=clock
MEMORY_ACCOUNT_ID=memory-account
LISTING_ID=listing
SUB_POOL_ID=sub-pool

require_session_fields() { :; }
save_session_state() { :; }
print_mydata_operation_summary() { :; }
validate_purchase_coin_ownership() { :; }
load_mydata_config_params_from_graphql() { :; }
resolve_purchase_listing_id() { printf '%s' listing; }
resolve_purchase_pay_coin() { printf '%s' payment-coin; }
resolve_session_or_prompt() {
    case "$1" in
        LISTING_ID) printf '%s' listing ;;
        MEMORY_ACCOUNT_ID) printf '%s' memory-account ;;
        *) return 1 ;;
    esac
}
resolve_sub_pool_id() { printf '%s' sub-pool; }
prompt_or_default() {
    case "$1" in
        "pool name") printf '%s' pool-name ;;
        description) printf '%s' pool-description ;;
        "bind platform? (y/N)") printf '%s' n ;;
        "manifest_hash (exactly 32-byte hex)") printf '%064d' 0 ;;
        "payment_reference"*) printf '%032d' 0 ;;
        *) printf '%s' "$2" ;;
    esac
}
prompt_with_default() {
    case "$1" in
        "encryption_id"*) printf '%064d' 0 ;;
        "source_pool_id"*) printf '%s' broad-pool ;;
        "Coin<MYSO>"*) printf '%s' payment-coin ;;
        "additional escrow"*) printf '%s' payment-coin ;;
        snapshot_id*) printf '%s' snapshot ;;
        "root_hash"*) printf '%064d' 0 ;;
        "total allocation"*) printf '%s' 100 ;;
        "positive contributor_count"*) printf '%s' 1 ;;
        *) printf '%s' "${2:-}" ;;
    esac
}

declare -a CAPTURED_CALL=()
run_myso_call() { CAPTURED_CALL=("$@"); }

assert_call() {
    local expected="$1" actual
    actual="$(printf '%s ' "${CAPTURED_CALL[@]}")"
    actual="${actual% }"
    [[ "$actual" == "$expected" ]] || {
        printf 'expected: %s\nactual:   %s\n' "$expected" "$actual" >&2
        return 1
    }
}

menu_purchase_one_time
assert_call "purchase_one_time --args cfg blocklist memory-config listing treasury payment-coin memory-account clock"

menu_purchase_sub
assert_call "purchase_subscription --args cfg blocklist memory-config listing treasury payment-coin memory-account clock"

menu_mydata_approve
assert_call "mydata_approve --args \"0x$(printf '%064d' 0)\" blocklist memory-config listing memory-account clock"

menu_create_broad_pool
assert_call "create_broad_pool --args cfg pool-cap pool-registry \"pool-name\" \"pool-description\" clock"

menu_record_anchor
assert_call "record_snapshot_anchor --args cfg anchor-registry claim-vault pool-registry broad-pool sub-pool \"0x$(printf '%064d' 0)\" \"0x$(printf '%032d' 0)\" payment-coin clock"

menu_deposit_snapshot_escrow
assert_call "deposit_snapshot_escrow --args pool-cap anchor-registry claim-vault snapshot payment-coin clock"

menu_publish_distribution
assert_call "publish_distribution --args cfg pool-cap anchor-registry distribution-registry claim-vault snapshot \"0x$(printf '%064d' 0)\" 100 1 clock"

prompt_claim_inputs() {
    CLAIM_SNAPSHOT_ID=snapshot
    CLAIM_AMOUNT=100
    CLAIM_LEAF_INDEX=0
    CLAIM_PROOF='[]'
}
menu_claim
assert_call "claim --args cfg distribution-registry claim-vault treasury snapshot 100 0 [] clock"

resolve_snapshot_platform_from_graphql() { printf '%s' platform; }
normalize_hex_id() { printf '%s' "$1"; }
menu_claim_with_platform
assert_call "claim_with_platform --args cfg distribution-registry claim-vault treasury platform snapshot 100 0 [] clock"

menu_reclaim_expired
assert_call "reclaim_expired_snapshot_escrow --args anchor-registry distribution-registry claim-vault snapshot clock"

echo "mydata marketplace call-order tests: PASS"
