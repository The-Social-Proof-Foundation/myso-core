#!/usr/bin/env bash
# Lightweight call-order / menu-shape checks for username-admin + spot-insurance runnables.
# Does not require a live localnet.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PASS=0
FAIL=0

assert_contains() {
    local file="$1" needle="$2" label="$3"
    if grep -qF -- "$needle" "$file"; then
        PASS=$((PASS + 1))
    else
        echo "FAIL: $label — expected to find: $needle" >&2
        FAIL=$((FAIL + 1))
    fi
}

assert_not_contains() {
    local file="$1" needle="$2" label="$3"
    if grep -qF -- "$needle" "$file"; then
        echo "FAIL: $label — expected NOT to find: $needle" >&2
        FAIL=$((FAIL + 1))
    else
        PASS=$((PASS + 1))
    fi
}

assert_file_executable_syntax() {
    local file="$1"
    if bash -n "$file"; then
        PASS=$((PASS + 1))
    else
        echo "FAIL: bash -n $file" >&2
        FAIL=$((FAIL + 1))
    fi
}

UA="$REPO_ROOT/scripts/username-admin-runnable.sh"
SI="$REPO_ROOT/scripts/spot-insurance-runnable.sh"

assert_file_executable_syntax "$UA"
assert_file_executable_syntax "$SI"

# Menu shape: 0 = refresh, 1 = primary run
assert_contains "$UA" '0) Refresh session from GraphQL' 'username-admin menu 0'
assert_contains "$UA" '1) Run rename flow (--run-all)' 'username-admin menu 1'
assert_contains "$SI" '0) Refresh session from GraphQL' 'spot-insurance menu 0'
assert_contains "$SI" '1) Run insurance walkthrough (--run-all)' 'spot-insurance menu 1'

# Username admin: single-profile rename (no secondary / source_replacement)
assert_contains "$UA" 'profile admin_reassign_username' 'reassign call'
assert_not_contains "$UA" 'admin_revoke_username' 'no revoke entry'
assert_not_contains "$UA" 'SOURCE_REPLACEMENT_USERNAME' 'no source replacement'
assert_not_contains "$UA" 'SECONDARY_PROFILE_ID' 'no secondary profile'
assert_contains "$UA" 'NEW_USERNAME' 'new username session key'
assert_contains "$UA" 'resolve_new_username' 'prompts new username'
assert_contains "$UA" 'New unclaimed username for primary profile' 'new username prompt label'
assert_contains "$UA" 'PRIMARY_PRIOR_USERNAME' 'tracks freed primary prior'
assert_contains "$UA" 'UsernameAdminCap' 'UsernameAdminCap reference'
assert_contains "$UA" 'create_profile_for_address' 'profile creation'
assert_contains "$UA" 'assert_on_chain_registry_username_absent' 'prior username on-chain absent'
assert_contains "$UA" 'assert_on_chain_registry_username' 'rename on-chain assert'
assert_contains "$UA" 'prior username freed for reclaim' 'summary asserts prior freed'

# Insurance E2E reuses SPOT + insurance helpers in the documented order
assert_contains "$SI" 'spot-oracle-post-runnable.sh' 'creates spot post'
assert_contains "$SI" 'spot_place_bet_for_post' 'places spot bet'
assert_contains "$SI" 'spot_insurance_e2e_prepare' 'prepares insurance'
assert_contains "$SI" 'spot_insurance_buy_coverage' 'buys coverage'
assert_contains "$SI" 'spot_insurance_claim' 'claims insurance'
assert_contains "$SI" 'spot_insurance_assert_duplicate_claim_fails' 'duplicate claim assert'
assert_contains "$SI" 'spot_claim_payout' 'claims spot payout'
assert_contains "$SI" 'wait_for_market_resolved' 'waits for resolve'
assert_contains "$SI" 'network.config/spot-insurance/spot-insurance-session.env' 'insurance session path'

# Session paths
assert_contains "$UA" 'network.config/username-admin/username-admin-session.env' 'username-admin session path'

if [[ "$FAIL" -gt 0 ]]; then
    echo "username-admin / spot-insurance runnable tests: FAIL ($FAIL failed, $PASS passed)" >&2
    exit 1
fi
echo "username-admin / spot-insurance runnable tests: PASS ($PASS checks)"
