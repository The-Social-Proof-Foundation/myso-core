#!/usr/bin/env bash
set -euo pipefail

TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_DIR/../.." && pwd)"
SCRIPT_DIR="$REPO_ROOT/scripts"

log_step() { :; }
log_wait_progress() { :; }
sleep() { :; }

# shellcheck source=../lib/orderbook-mm-supervisor.sh
source "$SCRIPT_DIR/lib/orderbook-mm-supervisor.sh"

assert_eq() {
    local expected="$1" actual="$2" message="$3"
    [[ "$actual" == "$expected" ]] || {
        printf '%s\nexpected: %s\nactual:   %s\n' "$message" "$expected" "$actual" >&2
        return 1
    }
}

unset ORDERBOOK_MM_READY_WAIT ORDERBOOK_MM_READY_GRACE
assert_eq 120 "$(orderbook_mm_ready_wait_seconds)" "default MM ready wait is 120s"
assert_eq 30 "$(orderbook_mm_ready_grace_seconds)" "default MM ready grace is 30s"

ORDERBOOK_MM_READY_WAIT=90
ORDERBOOK_MM_READY_GRACE=15
assert_eq 90 "$(orderbook_mm_ready_wait_seconds)" "ORDERBOOK_MM_READY_WAIT overrides wait"
assert_eq 15 "$(orderbook_mm_ready_grace_seconds)" "ORDERBOOK_MM_READY_GRACE overrides grace"
unset ORDERBOOK_MM_READY_WAIT ORDERBOOK_MM_READY_GRACE

READY_HITS=0
orderbook_mm_ready_http() {
    READY_HITS=$((READY_HITS + 1))
    [[ "$READY_HITS" -ge 3 ]]
}
orderbook_mm_health_http() { return 0; }
orderbook_mm_process_alive() { return 0; }

ORDERBOOK_MM_READY_WAIT=2
ORDERBOOK_MM_READY_GRACE=3
if ! orderbook_wait_mm_ready; then
    echo "grace path must succeed once /ready flips after the main budget" >&2
    exit 1
fi
assert_eq 3 "$READY_HITS" "main wait of 2 then first grace poll should be ready"

READY_HITS=0
orderbook_mm_ready_http() {
    READY_HITS=$((READY_HITS + 1))
    return 1
}
orderbook_mm_process_alive() { return 1; }
if orderbook_wait_mm_ready 2>/dev/null; then
    echo "dead MM must not enter grace" >&2
    exit 1
fi
assert_eq 2 "$READY_HITS" "dead MM should only poll the main wait"

READY_HITS=0
orderbook_mm_process_alive() { return 0; }
orderbook_mm_health_http() { return 1; }
if orderbook_wait_mm_ready 2>/dev/null; then
    echo "health-down MM must not enter grace" >&2
    exit 1
fi
assert_eq 2 "$READY_HITS" "health-down MM should only poll the main wait"

echo "orderbook mm supervisor helper tests passed"
