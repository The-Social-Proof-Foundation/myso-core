#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# E2E helper for the Task Market (social_contracts::agent_task): escrowed offers between a
# human buyer and a seller agent, delivery, settlement, receipts, bidirectional star reviews,
# reputation, and the social-server read endpoints behind the Task Market page.
#
# Actors (all created and funded for you, then remembered in the state file):
#   buyer   a human principal with a profile + MemoryAccount; funds the escrow
#   seller  a second principal with a profile, an agentic organization and a root sub-agent
#   agent   the seller's sub-agent; a real keypair, so it can sign accept / deliver / review
#
# Scenarios (each one is its own task, so a failure in one does not hide the others):
#   happy     offer -> accept -> deliver -> settle -> buyer reviews seller -> seller reviews buyer
#   decline   offer -> seller agent declines (escrow returned)
#   cancel    offer -> buyer cancels (escrow returned)
#   refund    offer -> accept -> deliver -> seller proposes refund -> buyer accepts (mutual refund)
#   guards    self-dealing, wrong terms digest, delivery by a non-agent, double review (all abort)
#
# Not covered here (they need the keeper / jury roster stack and wall-clock waits):
#   disputes and jury verdicts, release_task after the grace period, expire_offer, refund_task
#   after a missed delivery deadline. The script prints /jury/cases so you can see what exists.
#   Agent rooms are messaging groups made by the chat app; set ROOM_GROUP_ID to attach one to
#   the happy-path task and the script checks `/tasks?room_group_id=` and `/agents/rooms`.
#
# Prerequisites:
#   - ./scripts/bootstrap.sh completed (TaskConfig published and indexed)
#   - Local chain RPC http://127.0.0.1:9000, faucet enabled
#   - Social indexer GraphQL http://127.0.0.1:9125/graphql
#   - Social server       http://127.0.0.1:9126  (this branch: needs /task/stats and /agents/top)
#   - `myso`, `curl`, `jq`, `python3` on PATH
#
# Session:  network.config/task-market/task-market-session.env   (shared object ids)
# State:    network.config/task-market/task-market-state.env     (wallets, accounts, agent)
#
# Usage:
#   ./scripts/task-market-runnable.sh --refresh-session
#   ASSUME_YES=1 ./scripts/task-market-runnable.sh --run-all
#   ./scripts/task-market-runnable.sh --scenario happy
#   ./scripts/task-market-runnable.sh --setup-only
#   ./scripts/task-market-runnable.sh --reads          # print every Task Market endpoint
#   ./scripts/task-market-runnable.sh --fresh --run-all   # forget saved actors, make new ones
#   ./scripts/task-market-runnable.sh                  # interactive menu
#
# Environment:
#   ESCROW_MIST      escrow per task (default 100000000 = 0.1 MYSO)
#   ROOM_GROUP_ID    optional messaging group id attached to the happy-path task
#   REST_WAIT_MAX    seconds to wait for the indexer per assertion (default 40)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/task-market/task-market-session.env"
# shellcheck source=lib/social-runtime-common.sh
source "${SCRIPT_DIR}/lib/social-runtime-common.sh"
# shellcheck source=lib/runnable-summary-common.sh
source "${SCRIPT_DIR}/lib/runnable-summary-common.sh"

readonly STATE_FILE="$REPO_ROOT/network.config/task-market/task-market-state.env"
readonly CAP_TASK_TRADE='131072'
readonly CLASS_DELEGATED_AI='1'
readonly REGISTER_SCOPE='3'
readonly MS_HOUR=3600000
readonly STATUS_OFFERED=0 STATUS_ACCEPTED=1 STATUS_DELIVERED=2 STATUS_SETTLED=3 STATUS_REFUNDED=4 STATUS_CANCELLED=5

ESCROW_MIST="${ESCROW_MIST:-100000000}"
ROOM_GROUP_ID="${ROOM_GROUP_ID:-}"
REST_WAIT_MAX="${REST_WAIT_MAX:-40}"
GAS_BUDGET="${GAS_BUDGET:-300000000}"

SOCIAL_RUN_ID="$(date +%s)"
RUN_MODE=''
SCENARIO=''
FRESH=0
ASSUME_YES="${ASSUME_YES:-0}"

BUYER_ADDRESS=''
SELLER_ADDRESS=''
AGENT_ADDRESS=''
BUYER_PROFILE_ID=''
SELLER_PROFILE_ID=''
BUYER_ACCOUNT_ID=''
SELLER_ACCOUNT_ID=''
ORG_ID=''
AGENT_OBJECT_ID=''

LAST_OUT=''
LAST_DIGEST=''
LAST_TASK_ID=''
HAPPY_TASK_ID=''
CHECKS_PASSED=0
CHECKS_FAILED=0
FAILED_CHECKS=()

STATE_KEYS=(
    BUYER_ADDRESS SELLER_ADDRESS AGENT_ADDRESS BUYER_PROFILE_ID SELLER_PROFILE_ID
    BUYER_ACCOUNT_ID SELLER_ACCOUNT_ID ORG_ID AGENT_OBJECT_ID
)
SESSION_KEYS=(
    PKG_SOCIAL CLOCK_ID COIN_TYPE GAS_BUDGET
    USERNAME_REGISTRY_ID PROFILE_CONFIG_ID AI_CREDIT_CONFIG_ID MEMORY_REGISTRY_ID MEMORY_CONFIG_ID
    ECOSYSTEM_TREASURY_ID PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID PLATFORM_OBJECT_ID
    PLATFORM_ADMIN_CAP_ID BLOCK_LIST_REGISTRY_ID TASK_CONFIG_ID TASK_MARKET_CAP_ID
    JURY_CONFIG_ID JUROR_ROSTER_ID JURY_ADMIN_CAP_ID
)

usage() {
    sed -n '2,46p' "$0" | sed 's/^# \?//'
}

# ---------------------------------------------------------------------------
# State and checks
# ---------------------------------------------------------------------------

save_state() {
    local key
    mkdir -p "$(dirname "$STATE_FILE")"
    {
        echo "# Task Market actors — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
        for key in "${STATE_KEYS[@]}"; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${STATE_FILE}.tmp"
    chmod 600 "${STATE_FILE}.tmp" 2>/dev/null || true
    mv "${STATE_FILE}.tmp" "$STATE_FILE"
}

load_state() {
    if [[ "$FRESH" == 1 ]]; then
        rm -f "$STATE_FILE"
        return 0
    fi
    if [[ -f "$STATE_FILE" ]]; then
        # shellcheck disable=SC1090
        source "$STATE_FILE"
        echo "Loaded actors from: $STATE_FILE" >&2
    fi
}

save_session() {
    social_save_session "${SESSION_KEYS[@]}"
}

pass() {
    CHECKS_PASSED=$((CHECKS_PASSED + 1))
    echo "  PASS  $1" >&2
}

fail() {
    CHECKS_FAILED=$((CHECKS_FAILED + 1))
    FAILED_CHECKS+=("$1")
    echo "  FAIL  $1" >&2
}

# check "<description>" <command...>   passes when the command succeeds
check() {
    local desc="$1"
    shift
    if "$@" >/dev/null 2>&1; then pass "$desc"; else fail "$desc"; fi
}

check_eq() {
    local desc="$1" actual="$2" expected="$3"
    if [[ "$actual" == "$expected" ]]; then
        pass "$desc ($actual)"
    else
        fail "$desc (got '${actual:-<empty>}', want '$expected')"
    fi
}

# ---------------------------------------------------------------------------
# REST helpers (social server)
# ---------------------------------------------------------------------------

rest_get() {
    curl -sf --connect-timeout 3 --max-time 10 "${SOCIAL_SERVER_URL}$1"
}

# rest_wait <path> <jq filter> <expected>   polls until the filter equals expected
rest_wait() {
    local path="$1" filter="$2" expected="$3" attempt resp val
    for attempt in $(seq 1 "$REST_WAIT_MAX"); do
        resp="$(rest_get "$path" 2>/dev/null)" || resp=''
        val="$(printf '%s' "$resp" | jq -r "$filter" 2>/dev/null || true)"
        if [[ "$val" == "$expected" ]]; then
            printf '%s' "$resp"
            return 0
        fi
        sleep 1
    done
    echo "  (timed out: GET $path  $filter = '${val:-<none>}', wanted '$expected')" >&2
    return 1
}

rest_field() {
    rest_get "$1" 2>/dev/null | jq -r "$2" 2>/dev/null || true
}

rest_preflight() {
    command -v myso >/dev/null || { echo "myso not on PATH" >&2; return 1; }
    command -v jq >/dev/null || { echo "jq required" >&2; return 1; }
    command -v curl >/dev/null || { echo "curl required" >&2; return 1; }
    command -v python3 >/dev/null || { echo "python3 required" >&2; return 1; }
    if ! rest_get /task/config >/dev/null 2>&1; then
        echo "GET ${SOCIAL_SERVER_URL}/task/config failed." >&2
        echo "Start the social indexer + server (myso start --with-social-indexer ...) and run" >&2
        echo "./scripts/bootstrap.sh so TaskConfig exists and is indexed." >&2
        return 1
    fi
    if ! rest_get "/task/stats" >/dev/null 2>&1; then
        echo "GET /task/stats failed: the running social server predates the Task Market page endpoints." >&2
        echo "Rebuild and restart it from this branch." >&2
        return 1
    fi
}

# ---------------------------------------------------------------------------
# Transaction helpers
# ---------------------------------------------------------------------------

move_vector_u8_from_hex() {
    python3 - "$1" <<'PY'
import sys
data = bytes.fromhex(sys.argv[1].removeprefix("0x"))
print("vector[" + ",".join(f"{b}u8" for b in data) + "]")
PY
}

# Deterministic 32-byte digest (hex) for a label, so a rerun with a new run id gets a new scope.
digest_hex_for() {
    python3 - "$1" <<'PY'
import hashlib, sys
print(hashlib.sha256(sys.argv[1].encode()).hexdigest())
PY
}

# 32-byte raw public key (hex) of a keystore address: the keytool key is `flag || pubkey`.
raw_pubkey_hex_for_address() {
    local addr="$1" b64
    b64="$(myso keytool list --json 2>/dev/null \
        | jq -r --arg a "$addr" 'map(select(.mysoAddress == $a))[0].publicBase64Key // empty')"
    [[ -n "$b64" ]] || { echo "No keystore key for $addr" >&2; return 1; }
    python3 - "$b64" <<'PY'
import base64, sys
raw = base64.b64decode(sys.argv[1])
print(raw[1:].hex())
PY
}

# tm_tx <sender> <ptb args...>   runs one PTB; sets LAST_OUT / LAST_DIGEST; returns 1 on failure
tm_tx() {
    local sender="$1"
    shift
    LAST_OUT=''
    LAST_DIGEST=''
    LAST_OUT="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" "$@" 2>/dev/null)" || {
        echo "$LAST_OUT" | tail -n 20 >&2
        return 1
    }
    LAST_DIGEST="$(extract_tx_digest "$LAST_OUT" 2>/dev/null || true)"
    assert_tx_success "$LAST_OUT" "$LAST_DIGEST" || {
        echo "$LAST_OUT" | tail -n 20 >&2
        return 1
    }
}

# tm_expect_abort <description> <sender> <ptb args...>   passes when the transaction aborts
tm_expect_abort() {
    local desc="$1" sender="$2" out digest
    shift 2
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" "$@" 2>&1)" || true
    digest="$(extract_tx_digest "$out" 2>/dev/null || true)"
    if [[ -n "$out" ]] && assert_tx_aborts "$out" "$digest" 2>/dev/null; then
        pass "$desc (aborted as expected)"
    else
        fail "$desc (transaction was NOT rejected)"
    fi
}

ref() { ptb_shared_ref "$1"; }

# ---------------------------------------------------------------------------
# Setup: shared objects, wallets, profiles, organization, agent
# ---------------------------------------------------------------------------

ensure_session() {
    log_step "Resolving shared object ids from GraphQL"
    social_refresh_session_from_graphql || return 1
    require_session_fields TASK_CONFIG_ID MEMORY_CONFIG_ID MEMORY_REGISTRY_ID CLOCK_ID || {
        echo "TaskConfig / MemoryConfig not found. Run ./scripts/bootstrap.sh and wait for the indexer." >&2
        return 1
    }
    if ! session_value_set PLATFORM_OBJECT_ID; then
        log_step "No Platform object found; creating a test platform for settle_task"
        create_test_platform || return 1
    fi
    log_session_use "TASK_CONFIG_ID" "$TASK_CONFIG_ID"
    log_session_use "MEMORY_CONFIG_ID" "$MEMORY_CONFIG_ID"
    log_session_use "PLATFORM_OBJECT_ID" "$PLATFORM_OBJECT_ID"
    save_session
}

actor_on_chain() {
    [[ -n "${1:-}" ]] && object_exists_on_fullnode "$1"
}

fund() {
    # Buyer pays escrow for five tasks plus gas; the others only need gas.
    ensure_wallet_funded "$1" "$2"
}

ensure_wallets() {
    if [[ -z "$BUYER_ADDRESS" ]]; then
        BUYER_ADDRESS="$(create_ephemeral_wallet "tm_buyer_${SOCIAL_RUN_ID}")" || return 1
    fi
    if [[ -z "$SELLER_ADDRESS" ]]; then
        SELLER_ADDRESS="$(create_ephemeral_wallet "tm_seller_${SOCIAL_RUN_ID}")" || return 1
    fi
    if [[ -z "$AGENT_ADDRESS" ]]; then
        AGENT_ADDRESS="$(create_ephemeral_wallet "tm_agent_${SOCIAL_RUN_ID}")" || return 1
    fi
    log_session_use "BUYER_ADDRESS" "$BUYER_ADDRESS"
    log_session_use "SELLER_ADDRESS" "$SELLER_ADDRESS"
    log_session_use "AGENT_ADDRESS" "$AGENT_ADDRESS"
    save_state
    fund "$BUYER_ADDRESS" $((ESCROW_MIST * 6 + GAS_BUDGET * 12)) || return 1
    fund "$SELLER_ADDRESS" $((GAS_BUDGET * 6)) || return 1
    fund "$AGENT_ADDRESS" $((GAS_BUDGET * 12)) || return 1
}

ensure_profile() {
    # ensure_profile <role: BUYER|SELLER>
    local role="$1" addr_var="${1}_ADDRESS" profile_var="${1}_PROFILE_ID" acct_var="${1}_ACCOUNT_ID"
    local created profile mem lower
    if actor_on_chain "${!acct_var}"; then
        log_session_use "${acct_var} (existing)" "${!acct_var}"
        return 0
    fi
    lower="$(printf '%s' "$role" | tr 'A-Z' 'a-z')"
    created="$(create_profile_for_address "${!addr_var}" "Task Market ${role}" "tm${lower}${SOCIAL_RUN_ID}")" || return 1
    profile="$(printf '%s\n' "$created" | sed -n 1p)"
    mem="$(printf '%s\n' "$created" | sed -n 2p)"
    [[ -n "$profile" && -n "$mem" ]] || { echo "create_profile gave no MemoryAccount for $role" >&2; return 1; }
    printf -v "$profile_var" '%s' "$profile"
    printf -v "$acct_var" '%s' "$mem"
    log_session_use "$profile_var" "$profile"
    log_session_use "$acct_var" "$mem"
    save_state
}

ensure_seller_org_and_agent() {
    local pk_hex pk_vec created
    if actor_on_chain "$AGENT_OBJECT_ID"; then
        log_session_use "AGENT_OBJECT_ID (existing)" "$AGENT_OBJECT_ID"
        return 0
    fi
    if ! actor_on_chain "$ORG_ID"; then
        log_step "Seller creates an agentic organization"
        tm_tx "$SELLER_ADDRESS" \
            --move-call "${PKG_SOCIAL}::memory::create_agentic_organization" \
            "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$SELLER_ACCOUNT_ID")" 0 \
            "some($(literal_move_string "Task Market Seller Org"))" \
            "some($(literal_move_string "task market e2e"))" \
            "$(ref "$CLOCK_ID")" || return 1
        created="$(extract_created_object_by_type "$LAST_DIGEST" "memory::AgenticOrganization")" || created=''
        [[ -n "$created" ]] || { echo "No AgenticOrganization in tx $LAST_DIGEST" >&2; return 1; }
        ORG_ID="$(normalize_hex_id "$created")"
        log_session_use "ORG_ID" "$ORG_ID"
        save_state
    fi
    pk_hex="$(raw_pubkey_hex_for_address "$AGENT_ADDRESS")" || return 1
    pk_vec="$(move_vector_u8_from_hex "$pk_hex")"
    log_step "Seller registers the sub-agent (CAP_TASK_TRADE) with its own signing key"
    tm_tx "$SELLER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::memory::register_sub_agent" \
        "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$ORG_ID")" \
        "$pk_vec" "@${AGENT_ADDRESS}" \
        "$(literal_move_string "task-market-e2e-agent")" \
        "$CLASS_DELEGATED_AI" 0 \
        "$CAP_TASK_TRADE" "$CAP_TASK_TRADE" \
        "$REGISTER_SCOPE" 0 \
        none none none \
        "$(ref "$CLOCK_ID")" || return 1
    created="$(extract_created_object_by_type "$LAST_DIGEST" "memory::SubAgent")" || created=''
    [[ -n "$created" ]] || { echo "No SubAgent in tx $LAST_DIGEST" >&2; return 1; }
    AGENT_OBJECT_ID="$(normalize_hex_id "$created")"
    log_session_use "AGENT_OBJECT_ID" "$AGENT_OBJECT_ID"
    save_state
}

setup_actors() {
    ensure_session || return 1
    rest_preflight || return 1
    log_step "Task config"
    rest_get /task/config | jq '{active, platformFeeBps, ecosystemFeeBps, minEscrowMist, acceptWindowMinMs, deliveryWindowMinMs, settleWindowMinMs}' >&2
    check_eq "task market is active" "$(rest_field /task/config '.active')" "true"
    ensure_wallets || return 1
    ensure_profile BUYER || return 1
    ensure_profile SELLER || return 1
    ensure_seller_org_and_agent || return 1
    log_step "Actors ready"
    print_run_summary_header "Task Market actors"
    print_run_summary_line "buyer" "$BUYER_ADDRESS"
    print_run_summary_line "buyer account" "$BUYER_ACCOUNT_ID"
    print_run_summary_line "seller" "$SELLER_ADDRESS"
    print_run_summary_line "seller account" "$SELLER_ACCOUNT_ID"
    print_run_summary_line "seller agent (signer)" "$AGENT_ADDRESS"
    print_run_summary_line "seller agent object" "$AGENT_OBJECT_ID"
    print_run_summary_footer
}

# ---------------------------------------------------------------------------
# Task actions
# ---------------------------------------------------------------------------

# create_task <title> [room group id]   -> sets LAST_TASK_ID
create_task_offer() {
    local title="$1" room="${2:-}" scope_hex scope_vec room_arg created
    scope_hex="$(digest_hex_for "scope:${title}:${SOCIAL_RUN_ID}:${RANDOM}")"
    scope_vec="$(move_vector_u8_from_hex "$scope_hex")"
    if [[ -n "$room" ]]; then
        room_arg="some(@$(normalize_hex_id "$room"))"
    else
        room_arg='none'
    fi
    log_step "Buyer offers: ${title} (${ESCROW_MIST} MIST)"
    tm_tx "$BUYER_ADDRESS" \
        --split-coins gas "[${ESCROW_MIST}]" \
        --assign pay \
        --move-call "${PKG_SOCIAL}::agent_task::create_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        pay.0 "$ESCROW_MIST" "$scope_vec" "$(literal_move_string "$title")" \
        "$((1 * MS_HOUR))" "$((1 * MS_HOUR))" "$((1 * MS_HOUR))" \
        "$room_arg" none "$(ref "$CLOCK_ID")" \
        --transfer-objects "[pay.0]" "@${BUYER_ADDRESS}" || return 1
    created="$(extract_created_object_by_type "$LAST_DIGEST" "agent_task::TaskEscrow")" || created=''
    [[ -n "$created" ]] || { echo "No TaskEscrow in tx $LAST_DIGEST" >&2; return 1; }
    LAST_TASK_ID="$(normalize_hex_id "$created")"
    log_session_use "task" "$LAST_TASK_ID"
}

task_status() {
    rest_field "/tasks/$1" '.status'
}

wait_task_status() {
    local task="$1" want="$2" label="$3"
    if rest_wait "/tasks/${task}" '.status | tostring' "$want" >/dev/null; then
        pass "indexer shows task ${label} (status ${want})"
    else
        fail "indexer shows task ${label} (status ${want}); last status '$(task_status "$task")'"
    fi
}

terms_digest_of() {
    rest_field "/tasks/$1" '.termsDigest'
}

accept_task_as_agent() {
    local task="$1" terms
    terms="$(terms_digest_of "$task")"
    [[ -n "$terms" && "$terms" != null ]] || { echo "No terms digest for $task yet" >&2; return 1; }
    log_step "Seller agent accepts the exact terms"
    tm_tx "$AGENT_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::accept_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(move_vector_u8_from_hex "$terms")" "$(ref "$CLOCK_ID")"
}

deliver_task_as_agent() {
    local task="$1" deliverable_hex
    deliverable_hex="$(digest_hex_for "deliverable:${task}")"
    log_step "Seller agent records the deliverable"
    tm_tx "$AGENT_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::mark_delivered" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(move_vector_u8_from_hex "$deliverable_hex")" \
        "$(literal_move_string "https://example.com/deliverables/${task}")" \
        "$(ref "$CLOCK_ID")"
}

settle_task_as_buyer() {
    local task="$1"
    log_step "Buyer settles (pays the seller)"
    tm_tx "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::settle_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$PLATFORM_OBJECT_ID")" \
        "$(ref "$CLOCK_ID")"
}

# ---------------------------------------------------------------------------
# Scenarios
# ---------------------------------------------------------------------------

scenario_happy() {
    local task row amount pfee efee net score_before count
    print_run_summary_header "Scenario: happy path"
    create_task_offer "Summarise the Task Market contracts ${SOCIAL_RUN_ID}" "$ROOM_GROUP_ID" || { fail "create_task"; return 0; }
    task="$LAST_TASK_ID"
    HAPPY_TASK_ID="$task"
    wait_task_status "$task" "$STATUS_OFFERED" "offered"

    row="$(rest_get "/tasks/${task}" 2>/dev/null || true)"
    check_eq "offer amount" "$(echo "$row" | jq -r '.amountMist')" "$ESCROW_MIST"
    check_eq "buyer principal" "$(normalize_hex_id "$(echo "$row" | jq -r '.buyerPrincipal')")" "$(normalize_hex_id "$BUYER_ADDRESS")"
    check_eq "seller agent" "$(normalize_hex_id "$(echo "$row" | jq -r '.sellerAgentId')")" "$(normalize_hex_id "$AGENT_OBJECT_ID")"
    check "task is in the open-offers feed (/tasks?status=0)" \
        bash -c "curl -sf '${SOCIAL_SERVER_URL}/tasks?status=0&limit=200' | jq -e --arg t '${task}' 'map(select(.taskId == \$t)) | length == 1'"
    check "an unscoped, status-less listing is refused" \
        bash -c "[[ \"\$(curl -s -o /dev/null -w '%{http_code}' '${SOCIAL_SERVER_URL}/tasks')\" == 400 ]]"
    if [[ -n "$ROOM_GROUP_ID" ]]; then
        check "task is listed under its room (/tasks?room_group_id=)" \
            bash -c "curl -sf '${SOCIAL_SERVER_URL}/tasks?room_group_id=${ROOM_GROUP_ID}' | jq -e --arg t '${task}' 'map(select(.taskId == \$t)) | length == 1'"
    fi

    accept_task_as_agent "$task" || { fail "accept_task"; return 0; }
    wait_task_status "$task" "$STATUS_ACCEPTED" "accepted"

    deliver_task_as_agent "$task" || { fail "mark_delivered"; return 0; }
    wait_task_status "$task" "$STATUS_DELIVERED" "delivered"
    check "deliverable uri is indexed" \
        bash -c "curl -sf '${SOCIAL_SERVER_URL}/tasks/${task}' | jq -e '.deliverableUri != null'"

    score_before="$(rest_field "/agents/${AGENT_OBJECT_ID}/reputation" '.asSeller.reviewCount')"
    settle_task_as_buyer "$task" || { fail "settle_task"; return 0; }
    wait_task_status "$task" "$STATUS_SETTLED" "settled"

    row="$(rest_get "/tasks/${task}" 2>/dev/null || true)"
    amount="$(echo "$row" | jq -r '.amountMist')"
    pfee="$(echo "$row" | jq -r '.platformFeeMist // 0')"
    efee="$(echo "$row" | jq -r '.ecosystemFeeMist // 0')"
    net="$(echo "$row" | jq -r '.netAmountMist // 0')"
    check_eq "fees + net add back to the escrow" "$((pfee + efee + net))" "$amount"
    check "the seller was paid something (net > 0)" test "$net" -gt 0
    echo "  amount=$amount platformFee=$pfee ecosystemFee=$efee net=$net" >&2

    check "receipt appears for the agent (/agents/:id/receipts)" \
        bash -c "for i in \$(seq 1 ${REST_WAIT_MAX}); do curl -sf '${SOCIAL_SERVER_URL}/agents/${AGENT_OBJECT_ID}/receipts' | jq -e --arg t '${task}' 'map(select(.taskId == \$t)) | length == 1' >/dev/null && exit 0; sleep 1; done; exit 1"

    log_step "Buyer rates the seller agent 5 stars"
    if tm_tx "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::review_seller" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        5u8 "$(ref "$CLOCK_ID")"; then
        pass "review_seller executed"
    else
        fail "review_seller executed"
    fi

    log_step "Seller agent rates the buyer 4 stars"
    if tm_tx "$AGENT_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::review_buyer_principal" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        4u8 "$(ref "$CLOCK_ID")"; then
        pass "review_buyer_principal executed"
    else
        fail "review_buyer_principal executed"
    fi

    count="$(( ${score_before:-0} + 1 ))"
    if rest_wait "/agents/${AGENT_OBJECT_ID}/reputation" '.asSeller.reviewCount | tostring' "$count" >/dev/null; then
        pass "agent seller review count is ${count}"
    else
        fail "agent seller review count is ${count}"
    fi
    check "both reviews are on the task detail" \
        bash -c "for i in \$(seq 1 ${REST_WAIT_MAX}); do curl -sf '${SOCIAL_SERVER_URL}/tasks/${task}' | jq -e '.reviews | length == 2' >/dev/null && exit 0; sleep 1; done; exit 1"
    check "buyer principal has a human-buyer score (/principals/:address/reputation)" \
        bash -c "for i in \$(seq 1 ${REST_WAIT_MAX}); do curl -sf '${SOCIAL_SERVER_URL}/principals/${BUYER_ADDRESS}/reputation' | jq -e '.asHumanBuyer.reviewCount >= 1' >/dev/null && exit 0; sleep 1; done; exit 1"
    check "the agent is ranked in /agents/top" \
        bash -c "curl -sf '${SOCIAL_SERVER_URL}/agents/top?min_reviews=1&limit=200' | jq -e --arg a '${AGENT_OBJECT_ID}' 'map(select(.agentObjectId == \$a)) | length == 1'"
    check "a settled task is listed under status 3 (Receipts tab feed)" \
        bash -c "curl -sf '${SOCIAL_SERVER_URL}/tasks?status=3&limit=200' | jq -e --arg t '${task}' 'map(select(.taskId == \$t)) | length == 1'"
}

scenario_decline() {
    local task
    print_run_summary_header "Scenario: seller agent declines"
    create_task_offer "Declined job ${SOCIAL_RUN_ID}" || { fail "create_task"; return 0; }
    task="$LAST_TASK_ID"
    wait_task_status "$task" "$STATUS_OFFERED" "offered"
    log_step "Seller agent declines the offer"
    if tm_tx "$AGENT_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::decline_task" \
        "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(ref "$CLOCK_ID")"; then
        pass "decline_task executed"
    else
        fail "decline_task executed"
        return 0
    fi
    wait_task_status "$task" "$STATUS_CANCELLED" "cancelled"
    check_eq "close kind is seller-declined" "$(rest_field "/tasks/${task}" '.closeKind')" "1"
}

scenario_cancel() {
    local task
    print_run_summary_header "Scenario: buyer cancels the offer"
    create_task_offer "Cancelled job ${SOCIAL_RUN_ID}" || { fail "create_task"; return 0; }
    task="$LAST_TASK_ID"
    wait_task_status "$task" "$STATUS_OFFERED" "offered"
    log_step "Buyer cancels before acceptance"
    if tm_tx "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::cancel_offer" \
        "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$CLOCK_ID")"; then
        pass "cancel_offer executed"
    else
        fail "cancel_offer executed"
        return 0
    fi
    wait_task_status "$task" "$STATUS_CANCELLED" "cancelled"
}

scenario_refund() {
    local task
    print_run_summary_header "Scenario: mutual refund of delivered work"
    create_task_offer "Refunded job ${SOCIAL_RUN_ID}" || { fail "create_task"; return 0; }
    task="$LAST_TASK_ID"
    wait_task_status "$task" "$STATUS_OFFERED" "offered"
    accept_task_as_agent "$task" || { fail "accept_task"; return 0; }
    deliver_task_as_agent "$task" || { fail "mark_delivered"; return 0; }
    wait_task_status "$task" "$STATUS_DELIVERED" "delivered"
    log_step "Seller agent proposes a full refund"
    if tm_tx "$AGENT_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::propose_refund" \
        "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(ref "$CLOCK_ID")"; then
        pass "propose_refund executed"
    else
        fail "propose_refund executed"
        return 0
    fi
    if rest_wait "/tasks/${task}" '.refundProposer | tostring' "2" >/dev/null; then
        pass "indexer shows the seller's pending refund proposal"
    else
        fail "indexer shows the seller's pending refund proposal (refundProposer=$(rest_field "/tasks/${task}" '.refundProposer'))"
    fi
    log_step "Buyer accepts the refund"
    if tm_tx "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::accept_refund" \
        "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(ref "$CLOCK_ID")"; then
        pass "accept_refund executed"
    else
        fail "accept_refund executed"
        return 0
    fi
    wait_task_status "$task" "$STATUS_REFUNDED" "refunded"
}

scenario_guards() {
    local task terms bad_hex
    print_run_summary_header "Scenario: guards (every one must abort)"
    # The seller principal cannot buy from its own agent.
    tm_expect_abort "self-dealing: seller buys from its own agent" "$SELLER_ADDRESS" \
        --split-coins gas "[${ESCROW_MIST}]" --assign pay \
        --move-call "${PKG_SOCIAL}::agent_task::create_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        pay.0 "$ESCROW_MIST" "$(move_vector_u8_from_hex "$(digest_hex_for guard-self)")" \
        "$(literal_move_string "self deal")" \
        "$MS_HOUR" "$MS_HOUR" "$MS_HOUR" none none "$(ref "$CLOCK_ID")" \
        --transfer-objects "[pay.0]" "@${SELLER_ADDRESS}"

    # An escrow below the configured minimum.
    tm_expect_abort "escrow below the configured minimum" "$BUYER_ADDRESS" \
        --split-coins gas "[1]" --assign pay \
        --move-call "${PKG_SOCIAL}::agent_task::create_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        pay.0 1 "$(move_vector_u8_from_hex "$(digest_hex_for guard-min)")" \
        "$(literal_move_string "too small")" \
        "$MS_HOUR" "$MS_HOUR" "$MS_HOUR" none none "$(ref "$CLOCK_ID")" \
        --transfer-objects "[pay.0]" "@${BUYER_ADDRESS}"

    # An acceptance window shorter than the minimum.
    tm_expect_abort "accept window below the configured minimum" "$BUYER_ADDRESS" \
        --split-coins gas "[${ESCROW_MIST}]" --assign pay \
        --move-call "${PKG_SOCIAL}::agent_task::create_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        pay.0 "$ESCROW_MIST" "$(move_vector_u8_from_hex "$(digest_hex_for guard-window)")" \
        "$(literal_move_string "short window")" \
        1000 "$MS_HOUR" "$MS_HOUR" none none "$(ref "$CLOCK_ID")" \
        --transfer-objects "[pay.0]" "@${BUYER_ADDRESS}"

    create_task_offer "Guard target ${SOCIAL_RUN_ID}" || { fail "create_task for guards"; return 0; }
    task="$LAST_TASK_ID"
    wait_task_status "$task" "$STATUS_OFFERED" "offered"

    bad_hex="$(digest_hex_for "not-the-terms")"
    tm_expect_abort "accept with a different terms digest" "$AGENT_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::accept_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(move_vector_u8_from_hex "$bad_hex")" "$(ref "$CLOCK_ID")"

    tm_expect_abort "the buyer cannot accept on the seller's behalf" "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::accept_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(move_vector_u8_from_hex "$(terms_digest_of "$task")")" "$(ref "$CLOCK_ID")"

    tm_expect_abort "delivering before acceptance" "$AGENT_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::mark_delivered" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$AGENT_OBJECT_ID")" \
        "$(move_vector_u8_from_hex "$(digest_hex_for early)")" \
        "$(literal_move_string "https://example.com/early")" "$(ref "$CLOCK_ID")"

    tm_expect_abort "a stranger cannot cancel the buyer's offer" "$SELLER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::cancel_offer" \
        "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$CLOCK_ID")"

    tm_expect_abort "expire_offer before the accept deadline" "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::expire_offer" "$(ref "$task")" "$(ref "$CLOCK_ID")"

    tm_expect_abort "settle before anything was delivered" "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::settle_task" \
        "$(ref "$TASK_CONFIG_ID")" "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" \
        "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$PLATFORM_OBJECT_ID")" \
        "$(ref "$CLOCK_ID")"

    # Clean up the guard target so its escrow is returned.
    tm_tx "$BUYER_ADDRESS" \
        --move-call "${PKG_SOCIAL}::agent_task::cancel_offer" \
        "$(ref "$task")" "$(ref "$MEMORY_CONFIG_ID")" "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$CLOCK_ID")" \
        >/dev/null 2>&1 || true

    if [[ -n "$HAPPY_TASK_ID" ]]; then
        tm_expect_abort "reviewing the same receipt twice (buyer -> seller)" "$BUYER_ADDRESS" \
            --move-call "${PKG_SOCIAL}::agent_task::review_seller" \
            "$(ref "$TASK_CONFIG_ID")" "$(ref "$HAPPY_TASK_ID")" "$(ref "$MEMORY_CONFIG_ID")" \
            "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$SELLER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
            3u8 "$(ref "$CLOCK_ID")"
        tm_expect_abort "an out-of-range star rating" "$AGENT_ADDRESS" \
            --move-call "${PKG_SOCIAL}::agent_task::review_buyer_principal" \
            "$(ref "$TASK_CONFIG_ID")" "$(ref "$HAPPY_TASK_ID")" "$(ref "$BUYER_ACCOUNT_ID")" "$(ref "$AGENT_OBJECT_ID")" \
            9u8 "$(ref "$CLOCK_ID")"
    else
        echo "  (skipping the double-review guards: run the happy scenario first)" >&2
    fi
}

# ---------------------------------------------------------------------------
# Read endpoints (what the Task Market page calls)
# ---------------------------------------------------------------------------

show_reads() {
    local agent_arg=''
    print_run_summary_header "Task Market read endpoints"
    echo "GET /task/config" >&2
    rest_get /task/config | jq '.' >&2 || true
    echo "GET /task/stats" >&2
    rest_get /task/stats | jq '.' >&2 || true
    echo "GET /tasks?status=0 (open offers)" >&2
    rest_get '/tasks?status=0&limit=5' | jq 'map({taskId, title, amountMist, status, sellerAgentId})' >&2 || true
    echo "GET /tasks?status=1,2,6 (active)" >&2
    rest_get '/tasks?status=1,2,6&limit=5' | jq 'map({taskId, title, amountMist, status})' >&2 || true
    echo "GET /tasks?status=3 (settled)" >&2
    rest_get '/tasks?status=3&limit=5' | jq 'map({taskId, amountMist, settlementKind, settledAtMs})' >&2 || true
    echo "GET /agents/top?min_reviews=0" >&2
    rest_get '/agents/top?min_reviews=0&limit=5' | jq '.' >&2 || true
    if [[ -n "$AGENT_OBJECT_ID" ]]; then
        echo "GET /agents/${AGENT_OBJECT_ID}/reputation" >&2
        rest_get "/agents/${AGENT_OBJECT_ID}/reputation" | jq '.' >&2 || true
        echo "GET /agents/${AGENT_OBJECT_ID}/receipts" >&2
        rest_get "/agents/${AGENT_OBJECT_ID}/receipts?limit=5" | jq '.' >&2 || true
    fi
    if [[ -n "$SELLER_ADDRESS" ]]; then
        echo "GET /agents/rooms?principal=${SELLER_ADDRESS}" >&2
        rest_get "/agents/rooms?principal=${SELLER_ADDRESS}" | jq '.' >&2 || true
    fi
    if [[ -n "$BUYER_ADDRESS" ]]; then
        echo "GET /principals/${BUYER_ADDRESS}/reputation" >&2
        rest_get "/principals/${BUYER_ADDRESS}/reputation" | jq '.' >&2 || true
    fi
    echo "GET /jury/cases?subject_kind=1 (task disputes)" >&2
    rest_get '/jury/cases?subject_kind=1&limit=5' | jq 'map({caseId, subjectId, state, finalOutcome})' >&2 || true
    print_run_summary_footer
}

check_stats_reflect_runs() {
    local stats
    stats="$(rest_get /task/stats 2>/dev/null || true)"
    print_run_summary_header "Market stats after this run"
    echo "$stats" | jq '.' >&2 || true
    check "stats: at least one settled task" bash -c "echo '$stats' | jq -e '.settled >= 1'"
    check "stats: total settled volume is positive" bash -c "echo '$stats' | jq -e '.totalSettledMist > 0'"
    print_run_summary_footer
}

# ---------------------------------------------------------------------------
# Drivers
# ---------------------------------------------------------------------------

print_results() {
    local item
    print_run_summary_header "Task Market E2E results"
    print_run_summary_line "checks passed" "$CHECKS_PASSED"
    print_run_summary_line "checks failed" "$CHECKS_FAILED"
    if ((CHECKS_FAILED > 0)); then
        for item in "${FAILED_CHECKS[@]}"; do
            print_run_summary_line "  FAILED" "$item"
        done
    fi
    print_run_summary_footer
    echo "Open http://localhost:3000/task-market in mysocial-frontend to see the same data." >&2
}

run_scenario() {
    case "$1" in
        happy) scenario_happy ;;
        decline) scenario_decline ;;
        cancel) scenario_cancel ;;
        refund) scenario_refund ;;
        guards) scenario_guards ;;
        *) echo "Unknown scenario: $1 (happy|decline|cancel|refund|guards)" >&2; return 1 ;;
    esac
}

run_all_flow() {
    setup_actors || return 1
    scenario_happy
    scenario_decline
    scenario_cancel
    scenario_refund
    scenario_guards
    check_stats_reflect_runs
    show_reads
    print_results
    ((CHECKS_FAILED == 0))
}

show_menu() {
    cat >&2 <<'MENU'

Task Market E2E (scripts/task-market-runnable.sh)
  0) Refresh session from GraphQL
  1) Set up actors only (wallets, profiles, org, agent)
  2) Run everything
  3) Happy path
  4) Decline
  5) Cancel
  6) Mutual refund
  7) Guards (must-abort cases)
  8) Show read endpoints
  q) Quit
MENU
    local choice
    read -r -p "Choice: " choice
    case "${choice:-}" in
        0) social_refresh_session_from_graphql ;;
        1) setup_actors ;;
        2) run_all_flow || true ;;
        3) setup_actors && scenario_happy; print_results ;;
        4) setup_actors && scenario_decline; print_results ;;
        5) setup_actors && scenario_cancel; print_results ;;
        6) setup_actors && scenario_refund; print_results ;;
        7) setup_actors && scenario_guards; print_results ;;
        8) show_reads ;;
        [Qq]) exit 0 ;;
        *) echo "Invalid choice" >&2 ;;
    esac
    show_menu
}

main() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -h|--help) usage; exit 0 ;;
            -y) ASSUME_YES=1; shift ;;
            --fresh) FRESH=1; shift ;;
            --refresh-session) RUN_MODE=refresh; shift ;;
            --setup-only) RUN_MODE=setup; shift ;;
            --run-all) RUN_MODE=run_all; shift ;;
            --reads) RUN_MODE=reads; shift ;;
            --scenario) RUN_MODE=scenario; SCENARIO="${2:-}"; shift 2 || { usage; exit 1; } ;;
            *) echo "Unknown option: $1" >&2; usage; exit 1 ;;
        esac
    done

    social_load_session
    load_state

    case "${RUN_MODE:-}" in
        refresh) social_refresh_session_from_graphql; exit 0 ;;
        setup) setup_actors; exit $? ;;
        reads) show_reads; exit 0 ;;
        scenario)
            setup_actors || exit 1
            run_scenario "$SCENARIO" || exit 1
            print_results
            ((CHECKS_FAILED == 0)); exit $?
            ;;
        run_all) run_all_flow; exit $? ;;
        '') show_menu ;;
        *) echo "Unknown RUN_MODE: $RUN_MODE" >&2; exit 1 ;;
    esac
}

main "$@"
