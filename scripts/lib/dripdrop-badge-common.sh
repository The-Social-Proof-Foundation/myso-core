#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared helpers for DripDrop Premium+ platform badge grant/assign scripts.
# Source after social-runtime-common.sh.

if [[ -n "${_DRIPDROP_BADGE_COMMON_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_DRIPDROP_BADGE_COMMON_SOURCED=1

readonly DRIPDROP_PREMIUM_BADGE_NAME_DEFAULT='Premium+'
readonly DRIPDROP_PREMIUM_BADGE_DESCRIPTION_DEFAULT='DripDrop Premium+ subscriber'
readonly DRIPDROP_PREMIUM_BADGE_ASSET_URL_DEFAULT='https://pub-847d2d55c78d4b0abf39262364ac42de.r2.dev/vid_msjx2r68e494aebcda8a6135/thumbnail.webp'
readonly DRIPDROP_PREMIUM_BADGE_TYPE_DEFAULT='1'
readonly DRIPDROP_BADGE_SECRETS_REL='network.config/dripdrop/local-premium-badge-secrets.env'

dripdrop_apply_badge_defaults() {
    PREMIUM_BADGE_NAME="${PREMIUM_BADGE_NAME:-$DRIPDROP_PREMIUM_BADGE_NAME_DEFAULT}"
    PREMIUM_BADGE_DESCRIPTION="${PREMIUM_BADGE_DESCRIPTION:-$DRIPDROP_PREMIUM_BADGE_DESCRIPTION_DEFAULT}"
    PREMIUM_BADGE_ICON_URL="${PREMIUM_BADGE_ICON_URL:-$DRIPDROP_PREMIUM_BADGE_ASSET_URL_DEFAULT}"
    PREMIUM_BADGE_MEDIA_URL="${PREMIUM_BADGE_MEDIA_URL:-$PREMIUM_BADGE_ICON_URL}"
    PREMIUM_BADGE_TYPE="${PREMIUM_BADGE_TYPE:-$DRIPDROP_PREMIUM_BADGE_TYPE_DEFAULT}"
}

dripdrop_load_badge_secrets() {
    local path="${DRIPDROP_BADGE_SECRETS_FILE:-$REPO_ROOT/$DRIPDROP_BADGE_SECRETS_REL}"
    if [[ -f "$path" ]]; then
        # shellcheck disable=SC1090
        source "$path"
        log_step "Loaded badge secrets from $path"
    fi
}

dripdrop_premium_badge_id() {
    local platform_id="$1" name="${2:-${PREMIUM_BADGE_NAME:-$DRIPDROP_PREMIUM_BADGE_NAME_DEFAULT}}"
    platform_id="$(normalize_hex_id "$platform_id")" || return 1
    printf 'badge_%s_%s' "$platform_id" "$name"
}

dripdrop_gql_platform_badge_context() {
    local platform_id="$1" user="$2" resp vars
    platform_id="$(normalize_hex_id "$platform_id")" || return 1
    user="$(normalize_hex_id "$user")" || return 1
    vars="$(jq -nc --arg id "$platform_id" --arg user "$user" '{id: $id, user: $user}')"
    resp="$(graphql_post \
        'query PlatformBadgeCtx($id: ID!, $user: MySoAddress!) {
            platform(id: $id) {
                developerAddress
                moderatorsGroupId
                userAccess(user: $user) {
                    canManageBadges
                    moderatorPermissions
                }
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

dripdrop_resolve_moderators_group_id() {
    local platform_id="$1" group_id
    if [[ -n "${PERMISSIONED_GROUP_ID:-}" ]]; then
        normalize_hex_id "$PERMISSIONED_GROUP_ID"
        return 0
    fi
    platform_id="$(normalize_hex_id "$platform_id")" || return 1
    group_id="$(gql_platform_moderators_group_id "$platform_id")" || group_id=''
    [[ -n "$group_id" ]] || {
        echo "Could not resolve DripDrop moderatorsGroupId for $platform_id" >&2
        return 1
    }
    normalize_hex_id "$group_id"
}

dripdrop_wallet_can_manage_badges() {
    local platform_id="$1" user="$2" resp
    resp="$(dripdrop_gql_platform_badge_context "$platform_id" "$user")" || return 1
    echo "$resp" | jq -e '.data.platform.userAccess.canManageBadges == true' >/dev/null
}

dripdrop_platform_developer_address() {
    local platform_id="$1" resp vars
    platform_id="$(normalize_hex_id "$platform_id")" || return 1
    vars="$(jq -nc --arg id "$platform_id" '{id: $id}')"
    resp="$(graphql_post \
        'query PlatformDeveloper($id: ID!) { platform(id: $id) { developerAddress } }' \
        "$vars")" || return 1
    echo "$resp" | jq -r '.data.platform.developerAddress // empty'
}

dripdrop_grant_badge_admin() {
    local admin="$1" platform_id="$2" group_id="$3" member="$4" out
    admin="$(normalize_hex_id "$admin")" || return 1
    platform_id="$(normalize_hex_id "$platform_id")" || return 1
    group_id="$(normalize_hex_id "$group_id")" || return 1
    member="$(normalize_hex_id "$member")" || return 1
    require_session_fields PKG_SOCIAL || return 1
    log_step "Granting PlatformBadgeAdmin to $member on $platform_id"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$admin" \
        --move-call "${PKG_SOCIAL}::platform::grant_moderator_permission<${PKG_SOCIAL}::platform::PlatformBadgeAdmin>" \
        "@${platform_id}" \
        "@${group_id}" \
        "$member")" || {
        if echo "$out" | grep -qiE 'already|EUnauthorized|has_permission'; then
            log_step "Grant skipped or already present for $member"
            return 0
        fi
        echo "grant_moderator_permission<PlatformBadgeAdmin> failed" >&2
        return 1
    }
    if [[ "${DRY_RUN:-0}" == 1 ]]; then
        assert_tx_success "$out" || return 1
        return 0
    fi
    assert_tx_success "$out" || return 1
}

dripdrop_assign_premium_badge() {
    local signer="$1" profile_id="$2" out
    signer="$(normalize_hex_id "$signer")" || return 1
    profile_id="$(normalize_hex_id "$profile_id")" || return 1
    dripdrop_apply_badge_defaults
    require_session_fields PKG_SOCIAL PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID \
        DRIPDROP_PLATFORM_ID PERMISSIONED_GROUP_ID CLOCK_ID || return 1
    require_hex_ids PLATFORM_REGISTRY_ID PLATFORM_CONFIG_ID DRIPDROP_PLATFORM_ID \
        PERMISSIONED_GROUP_ID CLOCK_ID || return 1
    log_step "Assigning Premium+ badge to profile $profile_id"
    out="$(SKIP_CONFIRM_RUN=1 invoke_ptb_as_capture "$signer" \
        --move-call "${PKG_SOCIAL}::platform::assign_badge" \
        "@$(normalize_hex_id "$PLATFORM_REGISTRY_ID")" \
        "@$(normalize_hex_id "$PLATFORM_CONFIG_ID")" \
        "@$(normalize_hex_id "$DRIPDROP_PLATFORM_ID")" \
        "@$(normalize_hex_id "$PERMISSIONED_GROUP_ID")" \
        "@${profile_id}" \
        "$(literal_move_string "$PREMIUM_BADGE_NAME")" \
        "$(literal_move_string "$PREMIUM_BADGE_DESCRIPTION")" \
        "$(literal_move_string "$PREMIUM_BADGE_MEDIA_URL")" \
        "$(literal_move_string "$PREMIUM_BADGE_ICON_URL")" \
        "$PREMIUM_BADGE_TYPE" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || {
        if echo "$out" | grep -qE 'EBadgeAlreadyExists|abort code: 14|Abort.*14'; then
            log_step "Badge already exists on $profile_id"
            return 0
        fi
        echo "platform::assign_badge failed" >&2
        return 1
    }
    if echo "$out" | grep -qE 'EBadgeAlreadyExists|abort code: 14|Abort.*14'; then
        log_step "Badge already exists on $profile_id"
        return 0
    fi
    assert_tx_success "$out" || return 1
    if [[ "${DRY_RUN:-0}" != 1 ]]; then
        local digest
        digest="$(extract_tx_digest "$out")" || true
        if [[ -n "$digest" ]]; then
            tx_has_event_named "$digest" "BadgeAssignedEvent" || true
        fi
    fi
    printf '%s' "$out"
}

dripdrop_gql_profile_badge_ids() {
    local address="$1" resp vars
    address="$(normalize_hex_id "$address")" || return 1
    vars="$(jq -nc --arg addr "$address" '{addr: $addr}')"
    resp="$(graphql_post \
        'query ProfileBadges($addr: MySoAddress!) {
            profile(address: $addr) {
                profileId
                badges(limit: 50) { badgeId badgeName }
            }
        }' \
        "$vars")" || return 1
    printf '%s' "$resp"
}

dripdrop_wait_for_profile_badge() {
    local address="$1" badge_id="$2"
    local resp attempt=0 max="${GQL_WAIT_MAX:-15}" got
    address="$(normalize_hex_id "$address")" || return 1
    [[ -n "$badge_id" ]] || return 1
    while (( attempt < max )); do
        attempt=$((attempt + 1))
        log_wait_progress "GQL profile.badges" "$attempt" "$max" "badgeId=$badge_id"
        resp="$(dripdrop_gql_profile_badge_ids "$address" 2>/dev/null)" || resp='{}'
        got="$(echo "$resp" | jq -r --arg id "$badge_id" '
            [.data.profile.badges[]? | .badgeId] | map(ascii_downcase)
            | index($id | ascii_downcase) // empty
        ')"
        if [[ -n "$got" ]]; then
            return 0
        fi
        sleep 2
    done
    echo "Timed out waiting for badge $badge_id on $address" >&2
    echo "$resp" | jq . >&2 || true
    return 1
}
