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
    local resp vars attempt=0 max="${GQL_WAIT_MAX:-60}"
    subscriber="$(normalize_hex_id "$subscriber")" || return 1
    service_id="$(normalize_hex_id "$service_id")" || return 1
    vars="$(jq -nc --arg sub "$subscriber" --arg sid "$service_id" \
        '{subscriber: $sub, serviceId: $sid}')"
    while (( attempt < max )); do
        resp="$(graphql_post \
            'query SubAccess($subscriber: MySoAddress!, $serviceId: ID!) {
                subscriptionAccess(subscriber: $subscriber, serviceId: $serviceId) {
                    hasAccess
                }
            }' "$vars")" || { sleep 2; attempt=$((attempt + 1)); continue; }
        local got
        got="$(echo "$resp" | jq -r '.data.subscriptionAccess.hasAccess // empty')"
        if [[ "$got" == "$expected" ]]; then
            return 0
        fi
        sleep 2
        attempt=$((attempt + 1))
    done
    echo "GQL subscriptionAccess expected=$expected for subscriber=$subscriber service=$service_id" >&2
    echo "$resp" | jq . >&2 || true
    return 1
}

wait_for_gql_profile_subscription_field() {
    local subscription_id="$1" jq_path="$2" expected="$3"
    local resp vars attempt=0 max="${GQL_WAIT_MAX:-60}"
    subscription_id="$(normalize_hex_id "$subscription_id")" || return 1
    vars="$(jq -nc --arg id "$subscription_id" '{id: $id}')"
    while (( attempt < max )); do
        resp="$(graphql_post \
            'query Sub($id: ID!) {
                profileSubscription(subscriptionId: $id) {
                    subscriptionId expiresAt renewalBalance cancelledAt active
                }
            }' "$vars")" || { sleep 2; attempt=$((attempt + 1)); continue; }
        local got
        got="$(echo "$resp" | jq -r "$jq_path // empty")"
        if [[ -n "$got" && "$got" == "$expected" ]]; then
            return 0
        fi
        sleep 2
        attempt=$((attempt + 1))
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
    wait_for_rest_subscription_access "$subscriber" "$service_id" "$expected_access" || return 1
    wait_for_gql_subscription_access "$subscriber" "$service_id" "$expected_access" || return 1
    if [[ -n "$subscription_id" && "$expected_access" == "true" ]]; then
        wait_for_gql_profile_subscription_field "$subscription_id" '.data.profileSubscription.active' 'true' || return 1
    fi
}

pick_split_coin_for_amount() {
    local owner="$1" amount="$2"
    owner="$(normalize_hex_id "$owner")" || return 1
    myso client gas --owner "$owner" --amount "$amount" 2>/dev/null | head -n1
}

subscription_require_session_objects() {
    require_session_fields \
        SUBSCRIPTION_CONFIG_ID ECOSYSTEM_TREASURY_ID CLOCK_ID \
        "${@}"
}
