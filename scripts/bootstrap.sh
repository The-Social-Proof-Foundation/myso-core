#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Usage: ./scripts/bootstrap.sh
#
# Claims all social admin capabilities, then seeds the localnet with the
# myso_admin profile, approved DAO platforms (DripDrop, SoFiSwap, Chatr),
# and localnet-friendly Social Proof Tokens config thresholds.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/social-runtime-common.sh
source "$SCRIPT_DIR/lib/social-runtime-common.sh"

GRAPHQL_URL="${GRAPHQL_URL:-http://127.0.0.1:9125/graphql}"
SKIP_CONFIRM_RUN=1
ASSUME_YES=1

GQL='query BootstrapKey {
  bootstrap: objects(
    filter: { type: "0x2::bootstrap_key::BootstrapKey", ownerKind: SHARED }
    first: 1
  ) {
    nodes { address }
  }
}'

# Admin profile (hardcoded)
readonly ADMIN_USERNAME='myso_admin'
readonly ADMIN_DISPLAY_NAME='MySo Admin'
readonly ADMIN_BIO='Hello world!!!!!'
readonly ADMIN_AVATAR_URL='https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/05dbb735-d8f2-4be5-cd6e-f9c8cf4b2900/256'
readonly ADMIN_COVER_URL='https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/9c49cfe8-83b5-41a7-63cf-5713b9000300/public'

# Platform media (hardcoded — not env vars)
readonly DRIPDROP_LOGO_URL='https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/908ab73e-50c9-477f-870d-2f1ed6b93d00/256'
readonly DRIPDROP_COVER_URL='https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/e4359209-29b6-4bd2-6560-ee45cde98300/256'
readonly SOFISWAP_LOGO_URL='https://imagedelivery.net/pWxk5Gk3_hWKUypXIAwVgg/b8bdf45f-f221-4e3a-9259-c1aeedc8d200/public'
readonly SOFISWAP_COVER_URL='https://imagedelivery.net/pWxk5Gk3_hWKUypXIAwVgg/26308053-9da7-4378-c707-19b1c2a6d700/public'
readonly CHATR_LOGO_URL='https://imagedelivery.net/pWxk5Gk3_hWKUypXIAwVgg/ac67dfce-8331-4dc9-5af3-eb1c3fefe200/public'
readonly CHATR_COVER_URL='https://imagedelivery.net/pWxk5Gk3_hWKUypXIAwVgg/f76bb698-3cf3-441e-43f3-080e42080b00/public'
readonly PLATFORM_MEDIA_PREVIEWS_CSV='https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/0c39eedb-4efe-4b1c-bcae-2fd2af5c4b00/256,https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/2048368f-34ee-42f7-263a-c3bd2ff3dd00/256,https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/1ff79c02-ab1b-4939-e273-0e98436e4700/256,https://imagedelivery.net/lP4RIXSxux_dD7_Y8Y9OGw/992b0e40-abfc-47ee-97f2-079496a7e800/256'
readonly PLATFORM_TERMS_URL='https://docs.google.com/document/d/1qxKECZAOfgaZxl49Y3PhP9oAxB1yOsKJLasPEU-b6GY/edit?pli=1&tab=t.0'
readonly PLATFORM_PRIVACY_URL='https://docs.google.com/document/d/1_lFu0GsqmcsyiuKrlGF-RBz6nd4Gm3vGluxhALiXYQA/'
readonly PLATFORM_PLATFORMS_CSV='iOS'
readonly PLATFORM_STATUS=0
readonly PLATFORM_RELEASE_DATE='2026-09-01'
readonly PLATFORM_DAO_DELEGATE_COUNT=3
readonly PLATFORM_DAO_DELEGATE_TERM_EPOCHS=90
readonly PLATFORM_DAO_PROPOSAL_SUBMISSION_COST=10
readonly PLATFORM_DAO_MAX_VOTES_PER_USER=10
readonly PLATFORM_DAO_QUADRATIC_BASE_COST=500
readonly PLATFORM_DAO_VOTING_PERIOD_EPOCHS=7
readonly PLATFORM_DAO_QUORUM_VOTES=7

# SPT localnet config (matches admin UI)
readonly SPT_POST_THRESHOLD='10000000000'
readonly SPT_PROFILE_THRESHOLD='10000000000'
readonly SPT_MAX_INDIVIDUAL_RESERVATION_BPS='200000'
readonly SPT_MAX_HOLD_PERCENT_BPS='500000'

# Unchanged defaults from social_proof_tokens.move bootstrap_init
readonly SPT_TRADING_CREATOR_FEE_BPS='100'
readonly SPT_TRADING_PLATFORM_FEE_BPS='25'
readonly SPT_TRADING_TREASURY_FEE_BPS='25'
readonly SPT_RESERVATION_CREATOR_FEE_BPS='100'
readonly SPT_RESERVATION_PLATFORM_FEE_BPS='25'
readonly SPT_RESERVATION_TREASURY_FEE_BPS='25'
readonly SPT_BASE_PRICE='1000000000'
readonly SPT_QUADRATIC_COEFFICIENT='100000'
readonly SPT_MAX_RESERVERS_PER_POOL='1000'
readonly SPT_NON_PLATFORM_TO_CREATOR_BPS='5000'
readonly SPT_NON_PLATFORM_TO_TREASURY_BPS='5000'

SPT_ADMIN_CAP_ID=''
SOCIAL_PROOF_TOKENS_CONFIG_ID=''

bootstrap_refresh_session_with_retry() {
    local attempt max=12
    for ((attempt = 1; attempt <= max; attempt++)); do
        log_step "Refreshing social session from GraphQL (attempt $attempt/$max)"
        if social_refresh_session_from_graphql; then
            if [[ -n "${USERNAME_REGISTRY_ID:-}" && -n "${PLATFORM_REGISTRY_ID:-}" \
                && -n "${PLATFORM_ADMIN_CAP_ID:-}" && -n "${AI_CREDIT_CONFIG_ID:-}" ]]; then
                return 0
            fi
            echo "Session refresh incomplete (missing registry/cap ids)" >&2
        else
            echo "Session refresh failed" >&2
        fi
        sleep 2
    done
    echo "Could not refresh social session after claim" >&2
    return 1
}

bootstrap_append_session_ids() {
    mkdir -p "$(dirname "$SOCIAL_SESSION_SAVE_PATH")"
    {
        printf '%s=%q\n' ADMIN_PROFILE_ID "${ADMIN_PROFILE_ID:-}"
        printf '%s=%q\n' DRIPDROP_PLATFORM_ID "${DRIPDROP_PLATFORM_ID:-}"
        printf '%s=%q\n' SOFISWAP_PLATFORM_ID "${SOFISWAP_PLATFORM_ID:-}"
        printf '%s=%q\n' CHATR_PLATFORM_ID "${CHATR_PLATFORM_ID:-}"
        printf '%s=%q\n' PLATFORM_OBJECT_ID "${PLATFORM_OBJECT_ID:-}"
        printf '%s=%q\n' SPT_ADMIN_CAP_ID "${SPT_ADMIN_CAP_ID:-}"
        printf '%s=%q\n' SOCIAL_PROOF_TOKENS_CONFIG_ID "${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}"
    } >> "$SOCIAL_SESSION_SAVE_PATH"
    chmod 600 "$SOCIAL_SESSION_SAVE_PATH" 2>/dev/null || true
}

bootstrap_create_admin_profile() {
    local sender lines
    sender="$(resolve_myso_active_address)" || {
        echo "Could not resolve active address" >&2
        return 1
    }
    sender="$(normalize_hex_id "$sender")" || return 1
    log_step "Creating admin profile username=$ADMIN_USERNAME for $sender"
    lines="$(create_profile_with_media_for_address \
        "$sender" \
        "$ADMIN_DISPLAY_NAME" \
        "$ADMIN_USERNAME" \
        "$ADMIN_BIO" \
        "$ADMIN_AVATAR_URL" \
        "$ADMIN_COVER_URL")" || return 1
    ADMIN_PROFILE_ID="$(normalize_hex_id "$(echo "$lines" | sed -n '1p')")"
    [[ -n "$ADMIN_PROFILE_ID" ]] || {
        echo "Admin profile create did not return an id" >&2
        return 1
    }
    log_session_use "ADMIN_PROFILE_ID" "$ADMIN_PROFILE_ID"
}

bootstrap_create_seed_platform() {
    local name="$1" tagline="$2" description="$3"
    local primary_category="$4" secondary_category="$5" links_csv="$6"
    local logo_url="$7" cover_url="$8"
    create_and_approve_platform \
        "$name" \
        "$tagline" \
        "$description" \
        "$logo_url" \
        "$cover_url" \
        "$PLATFORM_MEDIA_PREVIEWS_CSV" \
        "$PLATFORM_TERMS_URL" \
        "$PLATFORM_PRIVACY_URL" \
        "$PLATFORM_PLATFORMS_CSV" \
        "$links_csv" \
        "$primary_category" \
        "$secondary_category" \
        "$PLATFORM_STATUS" \
        "$PLATFORM_RELEASE_DATE" \
        true \
        "$PLATFORM_DAO_DELEGATE_COUNT" \
        "$PLATFORM_DAO_DELEGATE_TERM_EPOCHS" \
        "$PLATFORM_DAO_PROPOSAL_SUBMISSION_COST" \
        "$PLATFORM_DAO_MAX_VOTES_PER_USER" \
        "$PLATFORM_DAO_QUADRATIC_BASE_COST" \
        "$PLATFORM_DAO_VOTING_PERIOD_EPOCHS" \
        "$PLATFORM_DAO_QUORUM_VOTES"
}

bootstrap_update_spt_config() {
    local admin_addr out
    SPT_ADMIN_CAP_ID="$(gql_live_object_address_for_type \
        '0x50c1::social_proof_tokens::SocialProofTokensAdminCap' ANY)" || SPT_ADMIN_CAP_ID=''
    SOCIAL_PROOF_TOKENS_CONFIG_ID="$(gql_live_object_address_for_type \
        '0x50c1::social_proof_tokens::SocialProofTokensConfig' SHARED)" || SOCIAL_PROOF_TOKENS_CONFIG_ID=''
    [[ -n "$SPT_ADMIN_CAP_ID" && -n "$SOCIAL_PROOF_TOKENS_CONFIG_ID" ]] || {
        echo "Could not resolve SPT admin cap or config object ids" >&2
        return 1
    }
    admin_addr="$(object_address_owner "$SPT_ADMIN_CAP_ID" 2>/dev/null)" || admin_addr=''
    admin_addr="$(normalize_hex_id "${admin_addr:-$(resolve_myso_active_address)}")" || return 1
    ensure_wallet_funded "$admin_addr" "$SOCIAL_DEFAULT_GAS_BUDGET" || return 1
    log_step "Updating SPT config (post/profile threshold=10 MYSO, max reservation=2000%, max hold=5000%)"
    log_session_use "SPT_ADMIN_CAP_ID" "$SPT_ADMIN_CAP_ID"
    log_session_use "SOCIAL_PROOF_TOKENS_CONFIG_ID" "$SOCIAL_PROOF_TOKENS_CONFIG_ID"
    out="$(run_myso_call_as_capture "$admin_addr" social_proof_tokens update_social_proof_tokens_config \
        "@$(normalize_hex_id "$SPT_ADMIN_CAP_ID")" \
        "@$(normalize_hex_id "$SOCIAL_PROOF_TOKENS_CONFIG_ID")" \
        "$SPT_TRADING_CREATOR_FEE_BPS" \
        "$SPT_TRADING_PLATFORM_FEE_BPS" \
        "$SPT_TRADING_TREASURY_FEE_BPS" \
        "$SPT_RESERVATION_CREATOR_FEE_BPS" \
        "$SPT_RESERVATION_PLATFORM_FEE_BPS" \
        "$SPT_RESERVATION_TREASURY_FEE_BPS" \
        "$SPT_BASE_PRICE" \
        "$SPT_QUADRATIC_COEFFICIENT" \
        "$SPT_MAX_HOLD_PERCENT_BPS" \
        "$SPT_POST_THRESHOLD" \
        "$SPT_PROFILE_THRESHOLD" \
        "$SPT_MAX_INDIVIDUAL_RESERVATION_BPS" \
        "$SPT_MAX_RESERVERS_PER_POOL" \
        "$SPT_NON_PLATFORM_TO_CREATOR_BPS" \
        "$SPT_NON_PLATFORM_TO_TREASURY_BPS" \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
}

echo ">>> Faucet"
myso client faucet

echo ">>> Resolving BootstrapKey from GraphQL"
bootstrap_key_id="$(
  curl -sS -X POST "$GRAPHQL_URL" \
    -H 'Content-Type: application/json' \
    -d "$(jq -nc --arg q "$GQL" '{query: $q}')" \
  | jq -r '.data.bootstrap.nodes[0].address'
)"
echo "BootstrapKey: $bootstrap_key_id"
[[ -n "$bootstrap_key_id" && "$bootstrap_key_id" != "null" ]] || {
    echo "BootstrapKey not found via GraphQL at $GRAPHQL_URL" >&2
    exit 1
}

echo ">>> claim_all_admin_capabilities"
myso client call \
  --package 0x50c1 \
  --module bootstrap \
  --function claim_all_admin_capabilities \
  --args 0x10 "$bootstrap_key_id" 0x6

echo ">>> Refresh social session + seed admin profile / platforms"
bootstrap_refresh_session_with_retry

bootstrap_create_admin_profile

DRIPDROP_PLATFORM_ID="$(bootstrap_create_seed_platform \
    'DripDrop' \
    'The most fun 24 sec social video economy.' \
    "We're building the most fun 24-second video economy, all on-chain. Featuring Social Proof Tokens, fair ownership, & unlimited ways to earn" \
    'Social Network' \
    'Video Streaming' \
    'https://dripdrop.social' \
    "$DRIPDROP_LOGO_URL" \
    "$DRIPDROP_COVER_URL")"
SOFISWAP_PLATFORM_ID="$(bootstrap_create_seed_platform \
    'SoFiSwap' \
    'Social finance swap for the MySo ecosystem.' \
    "We're building the fastest and most fun SocialFi + InfoFi decentralized exchange for Social Proof Tokens and MyData." \
    'Decentralized Exchange' \
    '' \
    'https://example.com' \
    "$SOFISWAP_LOGO_URL" \
    "$SOFISWAP_COVER_URL")"
CHATR_PLATFORM_ID="$(bootstrap_create_seed_platform \
    'Chatr' \
    'Free speech that rewards attention, powered by truth and transparency.' \
    "We're building the most important free speech platform that rewards attention through truth, transparency, and accountability, all on-chain." \
    'Social Network' \
    '' \
    'https://example.com' \
    "$CHATR_LOGO_URL" \
    "$CHATR_COVER_URL")"

PLATFORM_OBJECT_ID="$DRIPDROP_PLATFORM_ID"

echo ">>> Updating SPT config for localnet"
bootstrap_update_spt_config

bootstrap_append_session_ids

echo "ADMIN_PROFILE_ID=$ADMIN_PROFILE_ID"
echo "DRIPDROP_PLATFORM_ID=$DRIPDROP_PLATFORM_ID"
echo "SOFISWAP_PLATFORM_ID=$SOFISWAP_PLATFORM_ID"
echo "CHATR_PLATFORM_ID=$CHATR_PLATFORM_ID"
echo "SPT_ADMIN_CAP_ID=$SPT_ADMIN_CAP_ID"
echo "SOCIAL_PROOF_TOKENS_CONFIG_ID=$SOCIAL_PROOF_TOKENS_CONFIG_ID"
echo "Bootstrap complete."
