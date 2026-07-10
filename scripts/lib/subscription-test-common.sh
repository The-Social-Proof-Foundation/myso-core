#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared helpers for profile subscription E2E scripts.

wait_for_rest_subscription_access() {
    local subscriber="$1" service_id="$2" expected="$3"
    local url
    subscriber="$(normalize_hex_id "$subscriber")" || return 1
    service_id="$(normalize_hex_id "$service_id")" || return 1
    url="${SOCIAL_SERVER_URL}/subscription-access/${subscriber}/${service_id}"
    wait_for_rest_json "$url" '.hasAccess // .has_access // empty' "$expected"
}

wait_for_gql_subscription_access() {
    local subscriber="$1" service_id="$2" expected="$3"
    local resp vars attempt=0 max="${GQL_WAIT_MAX:-15}"
    subscriber="$(normalize_hex_id "$subscriber")" || return 1
    service_id="$(normalize_hex_id "$service_id")" || return 1
    vars="$(jq -nc --arg sub "$subscriber" --arg sid "$service_id" \
        '{subscriber: $sub, serviceId: $sid}')"
    while (( attempt < max )); do
        attempt=$((attempt + 1))
        log_wait_progress "GQL subscriptionAccess" "$attempt" "$max" "expected=$expected"
        resp="$(graphql_post \
            'query SubAccess($subscriber: MySoAddress!, $serviceId: ID!) {
                subscriptionAccess(subscriber: $subscriber, serviceId: $serviceId) {
                    hasAccess
                }
            }' "$vars")" || { sleep 2; continue; }
        local got
        got="$(echo "$resp" | jq -r '
            if .data.subscriptionAccess.hasAccess == true then "true"
            elif .data.subscriptionAccess.hasAccess == false then "false"
            else empty end
        ')"
        if [[ "$got" == "$expected" ]]; then
            return 0
        fi
        sleep 2
    done
    echo "GQL subscriptionAccess expected=$expected for subscriber=$subscriber service=$service_id" >&2
    echo "$resp" | jq . >&2 || true
    return 1
}

wait_for_gql_profile_subscription_field() {
    local subscription_id="$1" jq_path="$2" expected="$3"
    local resp vars attempt=0 max="${GQL_WAIT_MAX:-15}"
    subscription_id="$(normalize_hex_id "$subscription_id")" || return 1
    vars="$(jq -nc --arg id "$subscription_id" '{id: $id}')"
    while (( attempt < max )); do
        attempt=$((attempt + 1))
        log_wait_progress "GQL profileSubscription" "$attempt" "$max" "$jq_path expected=$expected"
        resp="$(graphql_post \
            'query Sub($id: ID!) {
                profileSubscription(subscriptionId: $id) {
                    subscriptionId expiresAt renewalBalance cancelledAt active
                }
            }' "$vars")" || { sleep 2; continue; }
        local got
        got="$(echo "$resp" | jq -r "$jq_path")"
        [[ "$got" == "null" ]] && got=''
        if [[ -n "$got" && "$got" == "$expected" ]]; then
            return 0
        fi
        sleep 2
    done
    echo "GQL profileSubscription $jq_path expected=$expected id=$subscription_id got=${got:-}" >&2
    return 1
}

gql_profile_subscription_services() {
    local owner="$1" limit="${2:-10}"
    owner="$(normalize_hex_id "$owner")" || return 1
    graphql_post \
        'query Services($owner: MySoAddress!, $limit: Int) {
            profile(address: $owner) {
                profileId
            }
        }' "$(jq -nc --arg owner "$owner" --argjson limit "$limit" '{owner: $owner, limit: $limit}')"
}

verify_subscription_layers() {
    local subscriber="$1" service_id="$2" expected_access="$3" subscription_id="${4:-}"
    wait_for_gql_subscription_access "$subscriber" "$service_id" "$expected_access" || return 1
    if [[ -n "${subscription_id}" && "$expected_access" == "true" ]]; then
        wait_for_gql_profile_subscription_field "$subscription_id" '.data.profileSubscription.active' 'true' || return 1
    fi
    if [[ "${VERIFY_REST_LAYERS:-0}" == 1 ]]; then
        if ! wait_for_rest_subscription_access "$subscriber" "$service_id" "$expected_access"; then
            echo "  (REST subscription-access lagged or unavailable; GQL verified)" >&2
        fi
    fi
}

ptb_option_address_from_arg() {
    local arg="$1"
    if [[ "$arg" == none ]]; then
        printf 'none'
        return 0
    fi
    if [[ "$arg" =~ ^some\((0x[0-9a-fA-F]+)\)$ ]]; then
        printf 'some(@%s)' "${BASH_REMATCH[1]}"
        return 0
    fi
    printf '%s' "$arg"
}

subscription_marketplace_session_path() {
    local repo_root="${REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
    printf '%s/network.config/username-marketplace/marketplace-session.env' "$repo_root"
}

subscription_import_creator_from_marketplace() {
    local role="${SUBSCRIPTION_CREATOR:-marketplace-seller}" mp_session
    mp_session="$(subscription_marketplace_session_path)"
    [[ -f "$mp_session" ]] || {
        echo "Marketplace session not found: $mp_session (run username-marketplace E2E first)" >&2
        return 1
    }
    # shellcheck disable=SC1090
    source "$mp_session"
    case "$role" in
        marketplace-seller|seller)
            require_session_fields SELLER_ADDRESS SELLER_PROFILE_ID || return 1
            CREATOR_ADDRESS="$(normalize_hex_id "$SELLER_ADDRESS")" || return 1
            CREATOR_PROFILE_ID="$(normalize_hex_id "$SELLER_PROFILE_ID")" || return 1
            if [[ -n "${SELLER_MEMORY_ACCOUNT_ID:-}" ]]; then
                MEMORY_ACCOUNT_ID="$(normalize_hex_id "$SELLER_MEMORY_ACCOUNT_ID")"
            fi
            ;;
        marketplace-buyer|buyer)
            require_session_fields BUYER_ADDRESS BUYER_PROFILE_ID || return 1
            CREATOR_ADDRESS="$(normalize_hex_id "$BUYER_ADDRESS")" || return 1
            CREATOR_PROFILE_ID="$(normalize_hex_id "$BUYER_PROFILE_ID")" || return 1
            ;;
        *)
            echo "Unknown SUBSCRIPTION_CREATOR=$role (use marketplace-seller or marketplace-buyer)" >&2
            return 1
            ;;
    esac
    log_step "Using marketplace $role as subscription creator (wallet=$CREATOR_ADDRESS profile=$CREATOR_PROFILE_ID)"
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
    log_session_use "CREATOR_PROFILE_ID" "$CREATOR_PROFILE_ID"
    [[ -n "${MEMORY_ACCOUNT_ID:-}" ]] && log_session_use "MEMORY_ACCOUNT_ID" "$MEMORY_ACCOUNT_ID"
    return 0
}

subscription_sync_creator_address_from_profile() {
    local profile_id="$1" owner
    profile_id="$(normalize_hex_id "$profile_id")" || return 1
    owner="$(object_address_owner "$profile_id" 2>/dev/null)" || owner=''
    [[ -n "$owner" ]] || return 1
    owner="$(normalize_hex_id "$owner")" || return 1
    if [[ -n "${CREATOR_ADDRESS:-}" && "$(normalize_hex_id "$CREATOR_ADDRESS")" != "$owner" ]]; then
        log_step "CREATOR_ADDRESS $(normalize_hex_id "$CREATOR_ADDRESS") does not own profile $profile_id — switching to $owner"
    fi
    CREATOR_ADDRESS="$owner"
    log_session_use "CREATOR_ADDRESS" "$CREATOR_ADDRESS"
    return 0
}

gql_subscription_service_for_profile() {
    local service_id="$1" resp vars
    service_id="$(normalize_hex_id "$service_id")" || return 1
    vars="$(jq -nc --arg sid "$service_id" '{sid: $sid}')"
    resp="$(graphql_post \
        'query Svc($sid: ID!) {
            profileSubscriptionService(serviceId: $sid) {
                serviceId
                profileId
                profileOwner
                active
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

gql_subscription_service_matches_profile() {
    local service_id="$1" profile_id="$2" resp match_pid
    service_id="$(normalize_hex_id "$service_id")" || return 1
    profile_id="$(normalize_hex_id "$profile_id")" || return 1
    resp="$(gql_subscription_service_for_profile "$service_id")" || return 1
    match_pid="$(echo "$resp" | jq -r '.data.profileSubscriptionService.profileId // empty')"
    [[ -n "$match_pid" && "$(normalize_hex_id "$match_pid")" == "$profile_id" ]]
}

subscription_resolve_existing_service_for_profile() {
    local profile_id="$1"
    profile_id="$(normalize_hex_id "$profile_id")" || return 1
    if [[ -n "${SERVICE_ID:-}" ]] \
        && object_exists_on_fullnode "$SERVICE_ID" \
        && gql_subscription_service_matches_profile "$SERVICE_ID" "$profile_id"; then
        normalize_hex_id "$SERVICE_ID"
        return 0
    fi
    return 1
}

subscription_load_mydata_secrets() {
    local lib_dir
    lib_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    # shellcheck source=lib/mydata-test-common.sh
    source "${lib_dir}/mydata-test-common.sh"
    local default_sec
    default_sec="$(mydata_default_secrets_env_path)"
    if [[ -n "${MYDATA_SECRETS_FILE:-}" ]]; then
        MYDATA_SECRETS_FILE="$(mydata_resolve_secrets_env_path "$MYDATA_SECRETS_FILE")"
    elif [[ -f "$default_sec" ]]; then
        MYDATA_SECRETS_FILE="$default_sec"
    fi
    if [[ -f "${MYDATA_SECRETS_FILE:-}" ]]; then
        # shellcheck disable=SC1090
        source "$MYDATA_SECRETS_FILE"
        mydata_hydrate_encrypt_from_secrets_file "$MYDATA_SECRETS_FILE" 1
    fi
    [[ -n "${KEY_SERVER_URL:-}" ]] || KEY_SERVER_URL="$MYDATA_DEFAULT_KEY_SERVER_URL"
    log_session_use "KEY_SERVER_URL" "$KEY_SERVER_URL"
    log_session_use "KEY_SERVER_OBJECT_ID" "${KEY_SERVER_OBJECT_ID:-}"
}

subscription_require_mydata_stack() {
    local lib_dir
    lib_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    # shellcheck source=lib/mydata-test-common.sh
    source "${lib_dir}/mydata-test-common.sh"
    local mydata_bin
    mydata_bin="$(mydata_resolve_mydata)"
    [[ -n "$mydata_bin" ]] || {
        echo "mydata CLI required; build myso-mydata (cargo build -p mydata-cli)" >&2
        return 1
    }
    subscription_load_mydata_secrets || return 1
    mydata_resolve_encrypt_credentials || return 1
    mydata_probe_key_server "${KEY_SERVER_URL:-$MYDATA_DEFAULT_KEY_SERVER_URL}" || return 1
    require_session_fields MYDATA_CONFIG_ID MYDATA_REGISTRY_ID MEMORY_CONFIG_ID || return 1
}

wait_for_gql_post_subscription_fields() {
    local post_id="$1" expect_encrypted="${2:-0}"
    local resp vars attempt=0 max="${GQL_WAIT_MAX:-15}"
    post_id="$(normalize_hex_id "$post_id")" || return 1
    vars="$(jq -nc --arg id "$post_id" '{id: $id}')"
    while (( attempt < max )); do
        attempt=$((attempt + 1))
        log_wait_progress "GQL post subscription gate" "$attempt" "$max" "post=$post_id"
        resp="$(graphql_post \
            'query PostSubGate($id: ID!) {
                post(id: $id) {
                    requiresSubscription
                    subscriptionServiceId
                    mydataId
                }
            }' "$vars")" || { sleep 2; continue; }
        local req_sub svc_id
        req_sub="$(echo "$resp" | jq -r '.data.post.requiresSubscription // empty')"
        svc_id="$(echo "$resp" | jq -r '.data.post.subscriptionServiceId // empty')"
        if [[ "$req_sub" == "true" && -n "$svc_id" ]]; then
            if [[ "$expect_encrypted" == 1 ]]; then
                local md
                md="$(echo "$resp" | jq -r '.data.post.mydataId // empty')"
                if [[ -n "$md" ]]; then
                    return 0
                fi
            else
                return 0
            fi
        fi
        sleep 2
    done
    echo "GQL post subscription fields not ready for post=$post_id (encrypted=$expect_encrypted)" >&2
    echo "$resp" | jq . >&2 || true
    if [[ "${LENIENT_OFFCHAIN:-0}" == 1 ]]; then
        echo "  (lenient-offchain: continuing)" >&2
        return 0
    fi
    return 1
}

subscription_dry_run_mydata_policy() {
    local sender="$1"
    local id_arg ref_mcfg ref_mydata ref_mem ref_svc ref_sub ref_clk out
    subscription_require_session_objects \
        MEMORY_CONFIG_ID MYDATA_ID MEMORY_ACCOUNT_ID SERVICE_ID SUBSCRIPTION_ID || return 1
    [[ -n "${ENCRYPTION_ID_HEX:-}" ]] || {
        echo "ENCRYPTION_ID_HEX required for policy dry-run" >&2
        return 1
    }
    id_arg="$(literal_move_vector_u8_from_hex "$ENCRYPTION_ID_HEX")"
    ref_mcfg="$(ptb_shared_ref "$MEMORY_CONFIG_ID")" || return 1
    ref_mydata="$(ptb_shared_ref "$MYDATA_ID")" || return 1
    ref_mem="$(ptb_shared_ref "$MEMORY_ACCOUNT_ID")" || return 1
    ref_svc="$(ptb_shared_ref "$SERVICE_ID")" || return 1
    ref_sub="@$(normalize_hex_id "$SUBSCRIPTION_ID")"
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1
    out="$(DRY_RUN=1 SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$sender" \
        --move-call "${PKG_SOCIAL}::mydata::mydata_approve_profile_subscription" \
        "$id_arg" "$ref_mcfg" "$ref_mydata" "$ref_mem" "$ref_svc" "$ref_sub" "$ref_clk")" || return 1
    assert_tx_success "$out"
}

subscription_subscriber_decrypt_encrypted_post() {
    local sender="$1"
    local mydata_bin rpc_url decrypted expect
    subscription_require_mydata_stack || return 1
    subscription_require_session_objects \
        MYDATA_ID SERVICE_ID SUBSCRIPTION_ID MEMORY_ACCOUNT_ID || return 1
    [[ -n "${ENCRYPT_CIPHERTEXT_HEX:-}" && -n "${ENCRYPTION_ID_HEX:-}" ]] || {
        echo "ENCRYPT_CIPHERTEXT_HEX and ENCRYPTION_ID_HEX required" >&2
        return 1
    }
    mydata_bin="$(mydata_resolve_mydata)"
    rpc_url="${MYSO_RPC_URL:-http://127.0.0.1:9000}"
    expect="${ENCRYPTED_PLAINTEXT_EXPECTED:-}"
    log_step "Decrypt via key server (fetch-key-profile-subscription) sender=$sender"
    decrypted="$("$mydata_bin" fetch-key-profile-subscription \
        --key-server-url "$KEY_SERVER_URL" \
        --key-server-object-id "$KEY_SERVER_OBJECT_ID" \
        --server-public-key-hex "$PUBLIC_KEY" \
        --package-id "$PKG_SOCIAL" \
        --encrypted-object-hex "0x${ENCRYPT_CIPHERTEXT_HEX}" \
        --encryption-id-hex "$ENCRYPTION_ID_HEX" \
        --sender "$sender" \
        --rpc-url "$rpc_url" \
        --memory-config-id "$MEMORY_CONFIG_ID" \
        --mydata-id "$MYDATA_ID" \
        --memory-account-id "$MEMORY_ACCOUNT_ID" \
        --service-id "$SERVICE_ID" \
        --subscription-id "$SUBSCRIPTION_ID" \
        --clock-id "$CLOCK_ID")" || return 1
    decrypted="${decrypted//$'\r'/}"
    [[ -n "$decrypted" ]] || {
        echo "fetch-key-profile-subscription returned empty plaintext" >&2
        return 1
    }
    if [[ -n "$expect" && "$decrypted" != "$expect" ]]; then
        echo "Decrypted plaintext mismatch." >&2
        echo "  expected: $expect" >&2
        echo "  got:      $decrypted" >&2
        return 1
    fi
    printf '%s' "$decrypted"
}

subscription_print_decrypted_post_body() {
    local sender="${1:-$SUBSCRIBER_ADDRESS}" decrypted
    subscription_require_session_objects POST_ID MYDATA_ID SERVICE_ID || return 1
    subscription_ensure_active_subscription flow_subscribe || return 1
    log_step "Decrypting MyData body for subscriber (post=$POST_ID mydata=$MYDATA_ID)"
    decrypted="$(subscription_subscriber_decrypt_encrypted_post "$sender")" || return 1
    log_step "Decrypted OK (${#decrypted} bytes)"
    echo ""
    echo "──────── Decrypted MyData post body (PRIVATE) ────────"
    printf '%s\n' "$decrypted"
    echo "────────────────────────────────────────────────────"
    echo "(DB/API posts.content still shows the public teaser only)"
    echo ""
}

pick_split_coin_for_amount() {
    local owner="$1" amount="$2" pay_coin gas_coin
    owner="$(normalize_hex_id "$owner")" || return 1
    ensure_two_gas_coins_for_address "$owner" || return 1
    read -r pay_coin gas_coin <<<"$(pick_payment_and_gas_coins_for_address "$owner" "$amount")" || return 1
    PAY_COIN_ID="$(normalize_hex_id "$pay_coin")"
    log_session_use "PAY_COIN_ID" "$PAY_COIN_ID"
    printf '@%s' "$PAY_COIN_ID"
}

subscription_require_session_objects() {
    if (($# > 0)); then
        require_session_fields \
            SUBSCRIPTION_CONFIG_ID ECOSYSTEM_TREASURY_ID CLOCK_ID \
            "$@"
    else
        require_session_fields \
            SUBSCRIPTION_CONFIG_ID ECOSYSTEM_TREASURY_ID CLOCK_ID
    fi
}

subscription_expires_at_ms() {
    local subscription_id="$1"
    subscription_id="$(normalize_hex_id "$subscription_id")" || return 1
    python3 - "$subscription_id" <<'PY'
import json, subprocess, sys
sub = sys.argv[1]
out = json.loads(subprocess.check_output(["myso", "client", "object", sub, "--json"], text=True))
b = bytes(out["data"]["Move"]["contents"])
# ProfileSubscription: UID(32) service_id(32) subscriber(32) created_at(8) expires_at(8)
print(int.from_bytes(b[104:112], "little"))
PY
}

subscription_on_chain_unexpired() {
    local subscription_id="$1" expires_at now_ms
    subscription_id="$(normalize_hex_id "$subscription_id")" || return 1
    expires_at="$(subscription_expires_at_ms "$subscription_id")" || return 1
    now_ms="$(($(date +%s) * 1000))"
    [[ "$expires_at" -gt "$now_ms" ]]
}

subscription_object_matches_service_and_subscriber() {
    local subscription_id="$1" service_id="$2" subscriber="$3"
    subscription_id="$(normalize_hex_id "$subscription_id")" || return 1
    service_id="$(normalize_hex_id "$service_id")" || return 1
    subscriber="$(normalize_hex_id "$subscriber")" || return 1
    python3 - "$subscription_id" "$service_id" "$subscriber" <<'PY'
import json, subprocess, sys
sub, want_svc, want_sub = sys.argv[1:4]
out = json.loads(subprocess.check_output(["myso", "client", "object", sub, "--json"], text=True))
b = bytes(out["data"]["Move"]["contents"])
svc = "0x" + b[32:64].hex()
owner = "0x" + b[64:96].hex()
raise SystemExit(0 if svc.lower() == want_svc.lower() and owner.lower() == want_sub.lower() else 1)
PY
}

subscription_find_owned_active_for_service() {
    local subscriber="$1" service_id="$2"
    local resp vars candidate now_ms json sub_id
    subscriber="$(normalize_hex_id "$subscriber")" || return 1
    service_id="$(normalize_hex_id "$service_id")" || return 1
    now_ms="$(($(date +%s) * 1000))"

    vars="$(jq -nc --arg sub "$subscriber" --arg svc "$service_id" '{sub: $sub, svc: $svc}')"
    resp="$(graphql_post \
        'query ActiveSubs($sub: MySoAddress!, $svc: ID!) {
            profileSubscriptions(subscriber: $sub, serviceId: $svc, limit: 10) {
                subscriptionId expiresAt active cancelledAt
            }
        }' "$vars" 2>/dev/null)" || resp=''
    if [[ -n "$resp" ]]; then
        candidate="$(echo "$resp" | jq -r --argjson now "$now_ms" '
            [.data.profileSubscriptions[]?
             | select(.active == true and (.cancelledAt // null) == null and (.expiresAt // 0) > $now)
             | .subscriptionId]
            | first // empty
        ')"
        if [[ -n "$candidate" ]] \
            && subscription_object_matches_service_and_subscriber "$candidate" "$service_id" "$subscriber" \
            && subscription_on_chain_unexpired "$candidate"; then
            normalize_hex_id "$candidate"
            return 0
        fi
    fi

    json="$(myso client objects "$subscriber" --json 2>/dev/null)" || return 1
    while read -r sub_id; do
        [[ -z "$sub_id" ]] && continue
        if subscription_object_matches_service_and_subscriber "$sub_id" "$service_id" "$subscriber" \
            && subscription_on_chain_unexpired "$sub_id"; then
            normalize_hex_id "$sub_id"
            return 0
        fi
    done < <(echo "$json" | jq -r '
        .[]? | select(
            ((.data.Move.type_.Other? // empty | .module == "subscription" and .name == "ProfileSubscription")
                or (.type? | tostring | contains("subscription::ProfileSubscription")))
        ) | (.data.objectId // .objectId // .object_id // empty)
    ')
    return 1
}

subscription_reuse_active_if_present() {
    local subscriber="${1:-$SUBSCRIBER_ADDRESS}" existing
    subscription_require_session_objects SERVICE_ID || return 1
    [[ -n "${subscriber:-}" ]] || return 1
    if [[ -n "${SUBSCRIPTION_ID:-}" ]] && subscription_is_active "$SUBSCRIPTION_ID"; then
        return 0
    fi
    existing="$(subscription_find_owned_active_for_service "$subscriber" "$SERVICE_ID" 2>/dev/null)" || existing=''
    [[ -n "$existing" ]] || return 1
    log_step "Reusing active subscription $existing (no new subscribe_to_profile payment)"
    SUBSCRIPTION_ID="$(normalize_hex_id "$existing")"
    SUBSCRIPTION_ACTIVE=1
    log_session_use "SUBSCRIPTION_ID" "$SUBSCRIPTION_ID"
    log_session_use "SUBSCRIPTION_ACTIVE" "$SUBSCRIPTION_ACTIVE"
    return 0
}

subscription_is_active() {
    local subscription_id="$1"
    subscription_id="$(normalize_hex_id "$subscription_id")" || return 1
    [[ "${SUBSCRIPTION_ACTIVE:-0}" == 1 ]] || return 1
    [[ -n "${SUBSCRIPTION_ID:-}" ]] || return 1
    [[ "$(normalize_hex_id "$SUBSCRIPTION_ID")" == "$subscription_id" ]] || return 1
    object_exists_on_fullnode "$subscription_id" || return 1
    subscription_on_chain_unexpired "$subscription_id"
}

subscription_ensure_active_subscription() {
    local ensure_fn="${1:-flow_subscribe}"
    subscription_require_session_objects SERVICE_ID || return 1
    if subscription_reuse_active_if_present "$SUBSCRIBER_ADDRESS"; then
        return 0
    fi
    log_step "Subscribing (no active subscription found for $SUBSCRIBER_ADDRESS)"
    SUBSCRIPTION_ID=''
    SUBSCRIPTION_ACTIVE=0
    "$ensure_fn" || return 1
}
