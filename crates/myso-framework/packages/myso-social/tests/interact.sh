#!/bin/bash

# MySocial Contract Interaction Script
# Uses myso client call for public entry functions and myso client ptb for public fun
# (e.g. post::create_post, platform::create_platform). See docs/content/references/cli/ptb.mdx.
#
# PTB notes:
#   - Shared object args must be valid 0x… IDs; empty vars expand to bare @ and fail PTB parsing.
#   - Keep vector literals single-quoted for zsh (e.g. 'vector[]').
#
# Optional local addresses file (sourced on startup if present):
#   interact_addrs.env  — same directory as this script; see keys in record_saved_addresses().

set -eo pipefail

INTERACT_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INTERACT_ADDRS_FILE="${INTERACT_SCRIPT_DIR}/interact_addrs.env"

# Package ID of the published MySocialContracts (social_contracts) package
PACKAGE_ID="${PACKAGE_ID:-0x50c1}"
# Published Orderbook package ID (only if your CLI requires it for typed args; usually object IDs suffice)
ORDERBOOK_PACKAGE_ID="${ORDERBOOK_PACKAGE_ID:-}"
GAS_BUDGET="${GAS_BUDGET:-1000000000}"
CLOCK_ID="${CLOCK_ID:-0x6}"

# Optional: set in interact_addrs.env after bootstrap or from explorer
BOOTSTRAP_KEY_ID="${BOOTSTRAP_KEY_ID:-}"
ORDERBOOK_REGISTRY_ID="${ORDERBOOK_REGISTRY_ID:-}"
USERNAME_REGISTRY_ID="${USERNAME_REGISTRY_ID:-}"
ECOSYSTEM_TREASURY_ID="${ECOSYSTEM_TREASURY_ID:-}"
BLOCK_LIST_REGISTRY_ID="${BLOCK_LIST_REGISTRY_ID:-}"
SOCIAL_GRAPH_ID="${SOCIAL_GRAPH_ID:-}"
PLATFORM_REGISTRY_ID="${PLATFORM_REGISTRY_ID:-}"
PLATFORM_CONFIG_ID="${PLATFORM_CONFIG_ID:-}"
MYDATA_REGISTRY_ID="${MYDATA_REGISTRY_ID:-}"
MYDATA_CONFIG_ID="${MYDATA_CONFIG_ID:-}"
GOVERNANCE_ECOSYSTEM_REGISTRY_ID="${GOVERNANCE_ECOSYSTEM_REGISTRY_ID:-}"
GOVERNANCE_POC_REGISTRY_ID="${GOVERNANCE_POC_REGISTRY_ID:-}"
POST_CONFIG_ID="${POST_CONFIG_ID:-}"
SOCIAL_PROOF_TOKENS_CONFIG_ID="${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}"
MESSAGE_REGISTRY_ID="${MESSAGE_REGISTRY_ID:-}"
SPOT_CONFIG_ID="${SPOT_CONFIG_ID:-}"
INSURANCE_CONFIG_ID="${INSURANCE_CONFIG_ID:-}"
# Shared Platform object (for posts/comments/reactions — not the PlatformRegistry ID)
PLATFORM_OBJECT_ID="${PLATFORM_OBJECT_ID:-}"
PLATFORM_ADMIN_CAP_ID="${PLATFORM_ADMIN_CAP_ID:-}"
MEMORY_ACCOUNT_ID="${MEMORY_ACCOUNT_ID:-}"
MODERATORS_GROUP_ID="${MODERATORS_GROUP_ID:-}"
# social_proof_tokens::TokenRegistry shared object
TOKEN_REGISTRY_ID="${TOKEN_REGISTRY_ID:-}"

if [ -f "${INTERACT_ADDRS_FILE}" ]; then
    # shellcheck disable=SC1090
    source "${INTERACT_ADDRS_FILE}"
fi

# Published package default if env file clears or omits PACKAGE_ID
PACKAGE_ID="${PACKAGE_ID:-0x50c1}"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "\n${BLUE}==== $1 ====${NC}\n"
}

print_success() {
    echo -e "${GREEN}$1${NC}"
}

print_info() {
    echo -e "${YELLOW}$1${NC}"
}

press_enter() {
    read -r -p "Press Enter to continue..."
}

# Run a programmable transaction block (one or more --move-call steps)
invoke_ptb() {
    myso client ptb "$@" --gas-budget "$GAS_BUDGET"
}

# Move UTF-8 string for PTB / CLI: '"text"' with backslash and double-quote escaped
literal_move_string() {
    local s=$1
    s="${s//\\/\\\\}"
    s="${s//\"/\\\"}"
    s="${s//$'\n'/\\n}"
    s="${s//$'\r'/}"
    printf "'\"%s\"'" "$s"
}

normalize_hex_id() {
    local id="$1"
    id="${id#@}"
    [[ -n "$id" ]] || return 1
    case "$id" in
        0x*) printf '%s' "$id" ;;
        *) printf '0x%s' "$id" ;;
    esac
}

ptb_shared_ref() {
    local id normalized
    id="$1"
    normalized="$(normalize_hex_id "$id")" || {
        echo "PTB shared object id is empty or invalid (got: '${id:-<empty>}')" >&2
        return 1
    }
    printf '@%s' "$normalized"
}

literal_move_vector_empty() {
    printf '%s' "'vector[]'"
}

literal_move_vector_from_csv() {
    local csv="$1"
    if [ -z "$csv" ]; then
        literal_move_vector_empty
        return 0
    fi
    local acc="" s2="" p
    IFS=',' read -r -a _VA <<<"$csv"
    for p in "${_VA[@]}"; do
        p="${p## }"
        p="${p%% }"
        acc="${acc}${s2}$(literal_move_string "$p")"
        s2=", "
    done
    printf 'vector[%s]' "$acc"
}

extract_tx_digest() {
    local out="$1" digest
    if command -v jq >/dev/null 2>&1; then
        digest="$(echo "$out" | jq -r '
            .effects.V2.transaction_digest //
            .effects.transaction_digest //
            .transaction_digest //
            empty
        ' 2>/dev/null | head -n1)"
        if [ -n "$digest" ]; then
            printf '%s' "$digest"
            return 0
        fi
    fi
    echo "$out" | grep -Eo 'Transaction Digest: [0-9a-zA-Z+/=_-]+' | head -n1 | awk '{print $3}' \
        || echo "$out" | grep -Eo '[A-Za-z0-9+/]{43,44}=' | head -n1
}

extract_created_object_by_type() {
    local digest="$1" type_substring="$2" json result
    [ -n "$digest" ] && [ -n "$type_substring" ] || return 1
    command -v jq >/dev/null 2>&1 || return 1
    json="$(myso client tx-block "$digest" --json 2>/dev/null)" || return 1
    result="$(echo "$json" | jq -r --arg t "$type_substring" '
        def suffix_match($ot):
            ($ot | tostring) | endswith("::" + $t);
        def object_type($o):
            ($o.objectType? // $o.object_type? // $o.type? // "") | tostring;
        def object_id($o):
            ($o.objectId? // $o.object_id? // $o.reference?.objectId? // "") | tostring;
        (
            (.changed_objects // .changedObjects // [])[]
            | if type == "array" then empty else . end
            | select(suffix_match(object_type(.)))
            | object_id(.)
        ),
        (
            .. | objects
            | select(suffix_match(object_type(.)))
            | object_id(.)
        )
        | select(. != null and . != "")
    ' | head -n1)"
    [ -n "$result" ] || return 1
    printf '%s' "$result"
}

invoke_ptb_capture() {
    local out ec
    out="$(myso client ptb "$@" --gas-budget "$GAS_BUDGET" 2>&1)"
    ec=$?
    echo "$out" >&2
    [ "$ec" -eq 0 ] || return 1
    printf '%s' "$out"
}

# ---- Main menu ----

show_menu() {
    print_header "MySocial Contract Interaction Menu"
    echo "1. Profile Management"
    echo "2. Content Management (PTB: post)"
    echo "3. Social Graph"
    echo "4. IP / MyData (create_and_share)"
    echo "5. Platform Management (PTB: create_platform)"
    echo "6. Block List Management"
    echo "7. Governance (client call entries)"
    echo "8. Social proof tokens"
    echo "9. View Object Details"
    echo "10. Upgrade Management"
    echo "11. Bootstrap & saved addresses"
    echo "0. Exit"
    echo ""
    read -r -p "Select an option [0-11]: " choice

    case $choice in
        1) profile_menu ;;
        2) content_menu ;;
        3) social_graph_menu ;;
        4) ip_menu ;;
        5) platform_menu ;;
        6) block_list_menu ;;
        7) governance_menu ;;
        8) token_exchange_menu ;;
        9) view_object ;;
        10) upgrade_menu ;;
        11) bootstrap_menu ;;
        0) exit 0 ;;
        *) echo "Invalid option" && show_menu ;;
    esac
}

# ---- Profile ----

profile_menu() {
    print_header "Profile Management"
    echo "1. Create Profile (registers username)"
    echo "2. About usernames"
    echo "3. Update Profile"
    echo "4. Create Username Listing"
    echo "5. Create Username Offer"
    echo "6. Accept Username Offer"
    echo "7. Reject/Revoke Username Offer"
    echo "8. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-8]: " choice

    case $choice in
        1) create_profile ;;
        2) about_usernames ;;
        3) update_profile ;;
        4) create_username_listing ;;
        5) create_username_offer ;;
        6) accept_username_offer ;;
        7) reject_username_offer ;;
        8) show_menu ;;
        *) echo "Invalid option" && profile_menu ;;
    esac
}

about_usernames() {
    print_header "Usernames"
    print_info "profile::register_username is test-only and not published for production."
    print_info "Use Create Profile: it creates the profile and reserves the username against the UsernameRegistry."
    print_info "Saved default registry: ${USERNAME_REGISTRY_ID:-not set in interact_addrs.env}"
    press_enter
    profile_menu
}

create_profile() {
    print_header "Creating Profile"

    read -r -p "Enter UsernameRegistry object ID [${USERNAME_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$USERNAME_REGISTRY_ID}"
    read -r -p "Enter display name: " display_name
    read -r -p "Enter username: " username
    read -r -p "Enter bio: " bio
    read -r -p "Enter profile picture URL (bytes): " profile_pic
    read -r -p "Enter cover image URL (bytes, leave empty if none): " cover_image

    if [ -z "$cover_image" ]; then
        cover_image='""'
    fi

    print_info "Creating profile..."
    myso client call --package "$PACKAGE_ID" --module profile --function create_profile \
        --args "$registry_id" "$display_name" "$username" "$bio" "$profile_pic" "$cover_image" --gas-budget "$GAS_BUDGET"

    print_success "Profile created. Note the Profile object ID from the transaction output."
    press_enter
    profile_menu
}

update_profile() {
    print_header "Updating Profile"

    read -r -p "Enter profile object ID: " profile_id
    read -r -p "Enter new display name: " display_name
    read -r -p "Enter new bio: " bio
    read -r -p "Enter new profile picture URL (bytes): " profile_pic
    read -r -p "Enter new cover image URL (bytes): " cover_image

    print_info "Updating profile..."
    myso client call --package "$PACKAGE_ID" --module profile --function update_profile \
        --args "$profile_id" "$display_name" "$bio" "$profile_pic" "$cover_image" "option::none()" --gas-budget "$GAS_BUDGET"

    print_success "Profile updated."
    press_enter
    profile_menu
}

create_username_listing() {
    print_header "Creating Username Listing"

    read -r -p "Enter UsernameMarketplace object ID [${USERNAME_MARKETPLACE_ID:-}]: " marketplace_id
    marketplace_id="${marketplace_id:-$USERNAME_MARKETPLACE_ID}"
    read -r -p "Enter UsernameRegistry object ID [${USERNAME_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$USERNAME_REGISTRY_ID}"
    read -r -p "Enter seller profile object ID: " profile_id
    read -r -p "Enter username to list: " username
    read -r -p "Enter minimum offer amount (MYSO base units): " min_price

    print_info "Creating username listing..."
    myso client call --package "$PACKAGE_ID" --module profile --function create_username_listing \
        --args "$marketplace_id" "$registry_id" "$profile_id" "$username" "$min_price" --gas-budget "$GAS_BUDGET"

    print_success "Username listing created."
    press_enter
    profile_menu
}

create_username_offer() {
    print_header "Creating Username Offer"

    read -r -p "Enter UsernameMarketplace object ID [${USERNAME_MARKETPLACE_ID:-}]: " marketplace_id
    marketplace_id="${marketplace_id:-$USERNAME_MARKETPLACE_ID}"
    read -r -p "Enter UsernameRegistry ID [${USERNAME_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$USERNAME_REGISTRY_ID}"
    read -r -p "Enter listed username: " username
    read -r -p "Enter coin object ID for payment: " coin_id
    read -r -p "Enter offer amount (MYSO base units): " amount

    print_info "Creating username offer..."
    myso client call --package "$PACKAGE_ID" --module profile --function create_username_offer \
        --args "$marketplace_id" "$registry_id" "$username" "$coin_id" "$amount" --gas-budget "$GAS_BUDGET"

    print_success "Username offer created."
    press_enter
    profile_menu
}

accept_username_offer() {
    print_header "Accepting Username Offer"

    read -r -p "Enter UsernameMarketplace object ID [${USERNAME_MARKETPLACE_ID:-}]: " marketplace_id
    marketplace_id="${marketplace_id:-$USERNAME_MARKETPLACE_ID}"
    read -r -p "Enter UsernameRegistry ID [${USERNAME_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$USERNAME_REGISTRY_ID}"
    read -r -p "Enter seller profile object ID: " profile_id
    read -r -p "Enter listed username: " username
    read -r -p "Enter buyer address: " buyer_address
    read -r -p "Enter replacement username for seller: " replacement_username
    read -r -p "Enter ProfileConfig object ID [${PROFILE_CONFIG_ID:-}]: " config_id
    config_id="${config_id:-$PROFILE_CONFIG_ID}"
    read -r -p "Enter EcosystemTreasury object ID [${ECOSYSTEM_TREASURY_ID:-}]: " treasury_id
    treasury_id="${treasury_id:-$ECOSYSTEM_TREASURY_ID}"

    print_info "Accepting username offer..."
    myso client call --package "$PACKAGE_ID" --module profile --function accept_username_offer \
        --args "$marketplace_id" "$registry_id" "$profile_id" "$username" "$buyer_address" "$replacement_username" "$config_id" "$treasury_id" --gas-budget "$GAS_BUDGET"

    print_success "Username offer accepted."
    press_enter
    profile_menu
}

reject_username_offer() {
    print_header "Rejecting/Revoking Username Offer"

    read -r -p "Enter UsernameMarketplace object ID [${USERNAME_MARKETPLACE_ID:-}]: " marketplace_id
    marketplace_id="${marketplace_id:-$USERNAME_MARKETPLACE_ID}"
    read -r -p "Enter seller profile object ID: " profile_id
    read -r -p "Enter listed username: " username
    read -r -p "Enter buyer address: " buyer_address

    print_info "Rejecting/revoking username offer..."
    myso client call --package "$PACKAGE_ID" --module profile --function reject_or_revoke_username_offer \
        --args "$marketplace_id" "$profile_id" "$username" "$buyer_address" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    profile_menu
}

# ---- Content (post:: public fun via PTB) ----

content_menu() {
    print_header "Content Management Menu"
    echo "1. Create post (post::create_post PTB)"
    echo "2. Create comment (post::create_comment PTB)"
    echo "3. React to post (post::react_to_post PTB)"
    echo "4. Moderate post — platform mod (post::set_moderation_status PTB)"
    echo "5. Delete post (post::delete_post PTB — owned Post objects only)"
    echo "6. Delete comment (post::delete_comment PTB — owned Comment objects only)"
    echo "7. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-7]: " choice

    case $choice in
        1) ptb_create_post ;;
        2) ptb_create_comment ;;
        3) ptb_react_to_post ;;
        4) ptb_moderate_post ;;
        5) ptb_delete_post ;;
        6) ptb_delete_comment ;;
        7) show_menu ;;
        *) echo "Invalid option" && content_menu ;;
    esac
}

ptb_create_post() {
    print_header "Create post (PTB)"
    print_info "Needs shared objects from bootstrap (see interact_addrs.env)."
    read -r -p "UsernameRegistry [${USERNAME_REGISTRY_ID:-}]: " ur
    ur="${ur:-$USERNAME_REGISTRY_ID}"
    read -r -p "PlatformRegistry [${PLATFORM_REGISTRY_ID:-}]: " pr
    pr="${pr:-$PLATFORM_REGISTRY_ID}"
    read -r -p "Platform shared object [${PLATFORM_OBJECT_ID:-}]: " plat
    plat="${plat:-$PLATFORM_OBJECT_ID}"
    read -r -p "BlockListRegistry [${BLOCK_LIST_REGISTRY_ID:-}]: " blr
    blr="${blr:-$BLOCK_LIST_REGISTRY_ID}"
    read -r -p "PostConfig [${POST_CONFIG_ID:-}]: " cfg
    cfg="${cfg:-$POST_CONFIG_ID}"
    read -r -p "MyDataRegistry [${MYDATA_REGISTRY_ID:-}]: " mr
    mr="${mr:-$MYDATA_REGISTRY_ID}"
    read -r -p "MemoryAccount [${MEMORY_ACCOUNT_ID:-}]: " mem
    mem="${mem:-$MEMORY_ACCOUNT_ID}"
    read -r -p "Post body (UTF-8; avoid unescaped double-quotes): " body
    CONTENT_LIT="$(literal_move_string "$body")"
    if [ -z "$ur" ] || [ -z "$pr" ] || [ -z "$plat" ] || [ -z "$blr" ] || [ -z "$cfg" ] || [ -z "$mr" ] || [ -z "$mem" ]; then
        print_info "Missing required object id."
        press_enter
        content_menu
        return
    fi
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mr ref_mem ref_clk
    ref_ur="$(ptb_shared_ref "$ur")" || { press_enter; content_menu; return; }
    ref_pr="$(ptb_shared_ref "$pr")" || { press_enter; content_menu; return; }
    ref_plat="$(ptb_shared_ref "$plat")" || { press_enter; content_menu; return; }
    ref_blr="$(ptb_shared_ref "$blr")" || { press_enter; content_menu; return; }
    ref_cfg="$(ptb_shared_ref "$cfg")" || { press_enter; content_menu; return; }
    ref_mr="$(ptb_shared_ref "$mr")" || { press_enter; content_menu; return; }
    ref_mem="$(ptb_shared_ref "$mem")" || { press_enter; content_menu; return; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { press_enter; content_menu; return; }
    print_info "Running myso client ptb --move-call ${PACKAGE_ID}::post::create_post ..."
    invoke_ptb --move-call "${PACKAGE_ID}::post::create_post" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" \
        "${CONTENT_LIT}" \
        none none none none none none none none \
        none some\(true\) none none \
        "$ref_mr" "$ref_mem" "$ref_clk"
    print_success "Submitted."
    press_enter
    content_menu
}

ptb_create_comment() {
    print_header "Create comment (PTB)"
    read -r -p "UsernameRegistry [${USERNAME_REGISTRY_ID:-}]: " ur
    ur="${ur:-$USERNAME_REGISTRY_ID}"
    read -r -p "PlatformRegistry [${PLATFORM_REGISTRY_ID:-}]: " pr
    pr="${pr:-$PLATFORM_REGISTRY_ID}"
    read -r -p "Platform shared object [${PLATFORM_OBJECT_ID:-}]: " plat
    plat="${plat:-$PLATFORM_OBJECT_ID}"
    read -r -p "BlockListRegistry [${BLOCK_LIST_REGISTRY_ID:-}]: " blr
    blr="${blr:-$BLOCK_LIST_REGISTRY_ID}"
    read -r -p "PostConfig [${POST_CONFIG_ID:-}]: " cfg
    cfg="${cfg:-$POST_CONFIG_ID}"
    read -r -p "MemoryAccount [${MEMORY_ACCOUNT_ID:-}]: " mem
    mem="${mem:-$MEMORY_ACCOUNT_ID}"
    read -r -p "Parent post (mutable Post object) ID: " pp
    read -r -p "Parent comment address (empty if top-level comment reply): " pc
    read -r -p "Comment body: " body
    BODY_LIT="$(literal_move_string "$body")"
    if [ -z "$pc" ]; then
        PC_ARG=none
    else
        PC_ARG="some(@$(normalize_hex_id "$pc"))"
    fi
    if [ -z "$ur" ] || [ -z "$pr" ] || [ -z "$plat" ] || [ -z "$blr" ] || [ -z "$cfg" ] || [ -z "$mem" ] || [ -z "$pp" ]; then
        print_info "Missing required id."
        press_enter
        content_menu
        return
    fi
    local ref_ur ref_pr ref_plat ref_blr ref_cfg ref_mem ref_pp ref_clk
    ref_ur="$(ptb_shared_ref "$ur")" || { press_enter; content_menu; return; }
    ref_pr="$(ptb_shared_ref "$pr")" || { press_enter; content_menu; return; }
    ref_plat="$(ptb_shared_ref "$plat")" || { press_enter; content_menu; return; }
    ref_blr="$(ptb_shared_ref "$blr")" || { press_enter; content_menu; return; }
    ref_cfg="$(ptb_shared_ref "$cfg")" || { press_enter; content_menu; return; }
    ref_mem="$(ptb_shared_ref "$mem")" || { press_enter; content_menu; return; }
    ref_pp="$(ptb_shared_ref "$pp")" || { press_enter; content_menu; return; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { press_enter; content_menu; return; }
    invoke_ptb --move-call "${PACKAGE_ID}::post::create_comment" \
        "$ref_ur" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" \
        "$ref_mem" "$ref_pp" \
        "${PC_ARG}" \
        "${BODY_LIT}" \
        none none none \
        "$ref_clk"
    print_success "Submitted."
    press_enter
    content_menu
}

ptb_react_to_post() {
    print_header "React to post (PTB)"
    read -r -p "UsernameRegistry [${USERNAME_REGISTRY_ID:-}]: " ur
    ur="${ur:-$USERNAME_REGISTRY_ID}"
    read -r -p "Post shared object ID (mutable Post): " post_id
    read -r -p "PlatformRegistry [${PLATFORM_REGISTRY_ID:-}]: " pr
    pr="${pr:-$PLATFORM_REGISTRY_ID}"
    read -r -p "Platform shared object [${PLATFORM_OBJECT_ID:-}]: " plat
    plat="${plat:-$PLATFORM_OBJECT_ID}"
    read -r -p "BlockListRegistry [${BLOCK_LIST_REGISTRY_ID:-}]: " blr
    blr="${blr:-$BLOCK_LIST_REGISTRY_ID}"
    read -r -p "PostConfig [${POST_CONFIG_ID:-}]: " cfg
    cfg="${cfg:-$POST_CONFIG_ID}"
    read -r -p "MemoryAccount [${MEMORY_ACCOUNT_ID:-}]: " mem
    mem="${mem:-$MEMORY_ACCOUNT_ID}"
    read -r -p "Reaction string (emoji/text): " reaction
    RX_LIT="$(literal_move_string "$reaction")"
    if [ -z "$ur" ] || [ -z "$post_id" ] || [ -z "$pr" ] || [ -z "$plat" ] || [ -z "$blr" ] || [ -z "$cfg" ] || [ -z "$mem" ]; then
        print_info "Missing required id."
        press_enter
        content_menu
        return
    fi
    local ref_ur ref_post ref_pr ref_plat ref_blr ref_cfg ref_mem ref_clk
    ref_ur="$(ptb_shared_ref "$ur")" || { press_enter; content_menu; return; }
    ref_post="$(ptb_shared_ref "$post_id")" || { press_enter; content_menu; return; }
    ref_pr="$(ptb_shared_ref "$pr")" || { press_enter; content_menu; return; }
    ref_plat="$(ptb_shared_ref "$plat")" || { press_enter; content_menu; return; }
    ref_blr="$(ptb_shared_ref "$blr")" || { press_enter; content_menu; return; }
    ref_cfg="$(ptb_shared_ref "$cfg")" || { press_enter; content_menu; return; }
    ref_mem="$(ptb_shared_ref "$mem")" || { press_enter; content_menu; return; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { press_enter; content_menu; return; }
    invoke_ptb --move-call "${PACKAGE_ID}::post::react_to_post" \
        "$ref_ur" "$ref_post" "$ref_pr" "$ref_plat" "$ref_blr" "$ref_cfg" \
        "$ref_mem" "${RX_LIT}" "$ref_clk"
    print_success "Submitted."
    press_enter
    content_menu
}

ptb_moderate_post() {
    print_header "Moderate post — platform moderator (PTB)"
    print_info "Uses post::set_moderation_status (caller must be platform dev/mod)."
    read -r -p "Post mutable object ID (shared Post): " post_id
    read -r -p "Platform shared object [${PLATFORM_OBJECT_ID:-}]: " plat
    plat="${plat:-$PLATFORM_OBJECT_ID}"
    read -r -p "Moderators group shared object ID [${MODERATORS_GROUP_ID:-}]: " group_id
    group_id="${group_id:-$MODERATORS_GROUP_ID}"
    read -r -p "PlatformRegistry [${PLATFORM_REGISTRY_ID:-}]: " preg
    preg="${preg:-$PLATFORM_REGISTRY_ID}"
    read -r -p "Status — 1=MODERATION_APPROVED, 2=MODERATION_FLAGGED (remove): " st
    read -r -p "Reason (optional Move string, empty for none): " reason_raw
    if [ -z "$reason_raw" ]; then
        R_ARG=none
    else
        rl="$(literal_move_string "$reason_raw")"
        R_ARG="some(${rl})"
    fi
    if [ -z "$post_id" ] || [ -z "$plat" ] || [ -z "$group_id" ] || [ -z "$preg" ]; then
        print_info "Missing required id."
        press_enter
        content_menu
        return
    fi
    local ref_post ref_plat ref_group ref_preg ref_clk
    ref_post="$(ptb_shared_ref "$post_id")" || { press_enter; content_menu; return; }
    ref_plat="$(ptb_shared_ref "$plat")" || { press_enter; content_menu; return; }
    ref_group="$(ptb_shared_ref "$group_id")" || { press_enter; content_menu; return; }
    ref_preg="$(ptb_shared_ref "$preg")" || { press_enter; content_menu; return; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { press_enter; content_menu; return; }
    invoke_ptb --move-call "${PACKAGE_ID}::post::set_moderation_status" \
        "$ref_post" "$ref_plat" "$ref_group" "$ref_preg" "${st}" "${R_ARG}" "$ref_clk"
    print_success "Submitted."
    press_enter
    content_menu
}

ptb_delete_post() {
    print_header "Delete post (post::delete_post PTB)"
    print_info "On-chain Posts are normally SHARED; delete_post takes an owned Post by value."
    print_info "This call succeeds only if the Post object is truly owned (e.g. test flows)."
    read -r -p "Owned Post object ID: " post_id
    if [ -z "$post_id" ]; then
        print_info "Missing post id."
        press_enter
        content_menu
        return
    fi
    local ref_post ref_clk
    ref_post="$(ptb_shared_ref "$post_id")" || { press_enter; content_menu; return; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { press_enter; content_menu; return; }
    invoke_ptb --move-call "${PACKAGE_ID}::post::delete_post" "$ref_post" "$ref_clk"
    print_success "Submitted."
    press_enter
    content_menu
}

ptb_delete_comment() {
    print_header "Delete comment (post::delete_comment PTB)"
    print_info "Comments are normally SHARED; delete_comment takes an owned Comment by value."
    read -r -p "Parent post (mutable shared Post) ID: " post_id
    read -r -p "Comment object ID (must be owned for this entry): " comment_id
    if [ -z "$post_id" ] || [ -z "$comment_id" ]; then
        print_info "Missing required id."
        press_enter
        content_menu
        return
    fi
    local ref_post ref_comment ref_clk
    ref_post="$(ptb_shared_ref "$post_id")" || { press_enter; content_menu; return; }
    ref_comment="$(ptb_shared_ref "$comment_id")" || { press_enter; content_menu; return; }
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || { press_enter; content_menu; return; }
    invoke_ptb --move-call "${PACKAGE_ID}::post::delete_comment" \
        "$ref_post" "$ref_comment" "$ref_clk"
    print_success "Submitted."
    press_enter
    content_menu
}

# ---- Social graph ----

social_graph_menu() {
    print_header "Social Graph Menu"
    echo "1. Follow User"
    echo "2. Unfollow User"
    echo "3. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-3]: " choice

    case $choice in
        1) follow_user ;;
        2) unfollow_user ;;
        3) show_menu ;;
        *) echo "Invalid option" && social_graph_menu ;;
    esac
}

follow_user() {
    print_header "Following User"

    read -r -p "Enter SocialGraph shared object ID [${SOCIAL_GRAPH_ID:-}]: " graph_id
    graph_id="${graph_id:-$SOCIAL_GRAPH_ID}"
    read -r -p "Enter wallet address to follow: " address_to_follow

    print_info "Following user..."
    myso client call --package "$PACKAGE_ID" --module social_graph --function follow \
        --args "$graph_id" "$address_to_follow" --gas-budget "$GAS_BUDGET"

    print_success "Follow transaction submitted."
    press_enter
    social_graph_menu
}

unfollow_user() {
    print_header "Unfollowing User"

    read -r -p "Enter SocialGraph shared object ID [${SOCIAL_GRAPH_ID:-}]: " graph_id
    graph_id="${graph_id:-$SOCIAL_GRAPH_ID}"
    read -r -p "Enter wallet address to unfollow: " address_to_unfollow

    print_info "Unfollowing user..."
    myso client call --package "$PACKAGE_ID" --module social_graph --function unfollow \
        --args "$graph_id" "$address_to_unfollow" --gas-budget "$GAS_BUDGET"

    print_success "Unfollow transaction submitted."
    press_enter
    social_graph_menu
}

# ---- IP / MyData (mydata::create_and_share entry) ----

ip_menu() {
    print_header "MyData Menu"
    echo "1. create_and_share (public entry)"
    echo "2. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-2]: " choice

    case $choice in
        1) mydata_create_and_share ;;
        2) show_menu ;;
        *) echo "Invalid option" && ip_menu ;;
    esac
}

mydata_create_and_share() {
    print_header "mydata::create_and_share"
    read -r -p "MyDataConfig shared object ID [${MYDATA_CONFIG_ID:-}]: " cfg_id
    cfg_id="${cfg_id:-$MYDATA_CONFIG_ID}"
    read -r -p "MyDataRegistry (mutable) ID [${MYDATA_REGISTRY_ID:-}]: " reg_id
    reg_id="${reg_id:-$MYDATA_REGISTRY_ID}"
    read -r -p "media_type (ASCII string hint): " media_type
    read -r -p "comma-separated tags (empty for vector[]): " tags_in
    if [ -z "$tags_in" ]; then
        TAGS_ARG="vector[]"
    else
        IFS=',' read -r -a TA <<<"$tags_in"
        TV=""
        sep=""
        for t in "${TA[@]}"; do
            t_trim="${t## }"
            t_trim="${t_trim%% }"
            lit="$(literal_move_string "$t_trim")"
            TV="${TV}${sep}${lit}"
            sep=", "
        done
        TAGS_ARG="vector[${TV}]"
    fi
    read -r -p "platform_id as address (optional, empty for none): " plat_opt
    if [ -z "$plat_opt" ]; then
        PLAT_ARG="none"
    else
        PLAT_ARG="some(@${plat_opt})"
    fi
    read -r -p "timestamp_start (u64 epoch ms): " ts
    ts="${ts:-0}"
    read -r -p "timestamp_end (optional ms, empty none): " te
    if [ -z "$te" ]; then
        TE_ARG="none"
    else
        TE_ARG="some($te)"
    fi
    print_info "Using vector[] placeholders for encrypted_data/encryption_id; edit PTB manually for real payloads."
    EDATA="vector[]"
    EID="vector[]"
    read -r -p "one_time_price optional u64 (empty none): " otp
    if [ -z "$otp" ]; then
        OTP_ARG="none"
    else
        OTP_ARG="some($otp)"
    fi
    read -r -p "subscription_price optional (empty none): " sp
    if [ -z "$sp" ]; then
        SP_ARG="none"
    else
        SP_ARG="some($sp)"
    fi
    read -r -p "subscription_duration_days (u64): " sdd
    sdd="${sdd:-0}"
    read -r -p "geographic_region (optional, empty none): " gr
    if [ -z "$gr" ]; then
        GR_ARG="none"
    else
        grl="$(literal_move_string "$gr")"
        GR_ARG="some(${grl})"
    fi
    read -r -p "data_quality optional: " dq
    if [ -z "$dq" ]; then
        DQ_ARG="none"
    else
        dql="$(literal_move_string "$dq")"
        DQ_ARG="some(${dql})"
    fi
    read -r -p "sample_size optional u64 (empty none): " ss
    if [ -z "$ss" ]; then
        SSA="none"
    else
        SSA="some($ss)"
    fi
    read -r -p "collection_method optional (empty none): " cm
    if [ -z "$cm" ]; then
        CM_ARG="none"
    else
        cml="$(literal_move_string "$cm")"
        CM_ARG="some(${cml})"
    fi
    read -r -p "is_updating (true/false): " iu
    read -r -p "update_frequency optional (empty none): " uf
    if [ -z "$uf" ]; then
        UF_ARG="none"
    else
        ufl="$(literal_move_string "$uf")"
        UF_ARG="some(${ufl})"
    fi

    if [ -z "$cfg_id" ] || [ -z "$reg_id" ]; then
        print_info "MYDATA_CONFIG_ID and MYDATA_REGISTRY_ID required."
        press_enter
        ip_menu
        return
    fi

    print_info "Calling mydata::create_and_share ..."
    myso client call --package "$PACKAGE_ID" --module mydata --function create_and_share \
        --args \
        "$cfg_id" \
        "$reg_id" \
        "$(literal_move_string "$media_type")" \
        "$TAGS_ARG" \
        "$PLAT_ARG" \
        "$ts" \
        "$TE_ARG" \
        "$EDATA" \
        "$EID" \
        "$OTP_ARG" \
        "$SP_ARG" \
        "$sdd" \
        "$GR_ARG" \
        "$DQ_ARG" \
        "$SSA" \
        "$CM_ARG" \
        "$iu" \
        "$UF_ARG" \
        "$CLOCK_ID" \
        --gas-budget "$GAS_BUDGET"

    print_success "Submitted."
    press_enter
    ip_menu
}

# ---- Platform (platform::create_platform PTB) ----

maybe_approve_platform() {
    local platform_id="$1"
    local admin_cap="${PLATFORM_ADMIN_CAP_ID:-}"
    local preg="${PLATFORM_REGISTRY_ID:-}"
    local pcfg="${PLATFORM_CONFIG_ID:-}"
    local ref_preg ref_pcfg ref_cap platform_addr out digest

    [ -n "$platform_id" ] || return 0
    [ -n "$admin_cap" ] || {
        print_info "PLATFORM_ADMIN_CAP_ID unset — skip toggle_platform_approval."
        return 0
    }
    [ -n "$pcfg" ] || {
        print_info "PLATFORM_CONFIG_ID unset — skip toggle_platform_approval."
        return 0
    }
    ref_preg="$(ptb_shared_ref "$preg")" || return 1
    ref_pcfg="$(ptb_shared_ref "$pcfg")" || return 1
    ref_cap="$(ptb_shared_ref "$admin_cap")" || return 1
    platform_addr="$(normalize_hex_id "$platform_id")" || return 1
    print_info "Approving platform via toggle_platform_approval ..."
    invoke_ptb --move-call "${PACKAGE_ID}::platform::toggle_platform_approval" \
        "$ref_preg" "$ref_pcfg" "$(ptb_shared_ref "$platform_addr")" "$ref_cap" none
}

invoke_platform_create_ptb() {
    local preg="$1"
    local nl="$2" tg="$3" ds="$4" lg="$5" tm="$6" pv="$7"
    local pl_vec="$8" lk_vec="$9"
    shift 9
    local pc="$1" sc_arg="$2" st="$3" rd="$4" wdao="$5"
    local dc_a="$6" dt_a="$7" psc_a="$8" mv_a="$9" qb_a="${10}" vp_a="${11}" qv_a="${12}"
    local cp_arg="${13}" mp_arg="${14}"
    local ref_preg ref_pcfg ref_clk out digest platform_id

    ref_preg="$(ptb_shared_ref "$preg")" || return 1
    if [ -z "${PLATFORM_CONFIG_ID:-}" ]; then
        print_info "PLATFORM_CONFIG_ID required (platform::PlatformConfig shared object)."
        return 1
    fi
    ref_pcfg="$(ptb_shared_ref "$PLATFORM_CONFIG_ID")" || return 1
    ref_clk="$(ptb_shared_ref "$CLOCK_ID")" || return 1

    out="$(invoke_ptb_capture --move-call "${PACKAGE_ID}::platform::create_platform" \
        "$ref_preg" \
        "$ref_pcfg" \
        "$nl" "$tg" "$ds" "$lg" "$tm" "$pv" \
        "$pl_vec" "$lk_vec" \
        "$pc" "$sc_arg" \
        "$st" \
        "$rd" \
        "$wdao" \
        "$dc_a" "$dt_a" "$psc_a" "$mv_a" "$qb_a" "$vp_a" "$qv_a" \
        "$cp_arg" "$mp_arg" \
        "$ref_clk")" || return 1

    digest="$(extract_tx_digest "$out")"
    platform_id="$(extract_created_object_by_type "$digest" "platform::Platform")"
    [ -n "$platform_id" ] || platform_id="$(extract_created_object_by_type "$digest" "Platform")"

    if [ -n "$platform_id" ]; then
        print_success "Created platform: $platform_id"
        maybe_approve_platform "$platform_id" || return 1
        read -r -p "Save as PLATFORM_OBJECT_ID in interact_addrs.env? [y/N]: " save_plat
        if [ "${save_plat}" = "y" ] || [ "${save_plat}" = "Y" ]; then
            PLATFORM_OBJECT_ID="$platform_id"
            if [ -f "${INTERACT_ADDRS_FILE}" ]; then
                if grep -q '^PLATFORM_OBJECT_ID=' "${INTERACT_ADDRS_FILE}"; then
                    sed -i.bak "s|^PLATFORM_OBJECT_ID=.*|PLATFORM_OBJECT_ID=${platform_id}|" "${INTERACT_ADDRS_FILE}"
                    rm -f "${INTERACT_ADDRS_FILE}.bak"
                else
                    printf '\nPLATFORM_OBJECT_ID=%s\n' "$platform_id" >>"${INTERACT_ADDRS_FILE}"
                fi
            else
                printf 'PLATFORM_OBJECT_ID=%s\n' "$platform_id" >"${INTERACT_ADDRS_FILE}"
            fi
            print_success "Updated ${INTERACT_ADDRS_FILE}"
        fi
    else
        print_info "Platform created; could not auto-detect object id from tx digest."
    fi
    return 0
}

platform_menu() {
    print_header "Platform Management Menu"
    echo "1. create_platform (interactive PTB)"
    echo "2. create_platform with test defaults (+ approve if admin cap set)"
    echo "3. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-3]: " choice

    case $choice in
        1) ptb_platform_create ;;
        2) ptb_platform_create_with_defaults ;;
        3) show_menu ;;
        *) echo "Invalid option" && platform_menu ;;
    esac
}

ptb_platform_create_with_defaults() {
    local preg run_id
    preg="${PLATFORM_REGISTRY_ID:-}"
    if [ -z "$preg" ]; then
        read -r -p "PlatformRegistry (mutable shared) ID: " preg
    fi
    if [ -z "${PLATFORM_CONFIG_ID:-}" ]; then
        read -r -p "PlatformConfig (shared) ID: " PLATFORM_CONFIG_ID
    fi
    if [ -z "$preg" ] || [ -z "${PLATFORM_CONFIG_ID:-}" ]; then
        print_info "PLATFORM_REGISTRY_ID and PLATFORM_CONFIG_ID required."
        press_enter
        platform_menu
        return
    fi
    run_id="$(date +%s)"
    invoke_platform_create_ptb \
        "$preg" \
        "$(literal_move_string "Test Platform ${run_id}")" \
        "$(literal_move_string 'A test platform')" \
        "$(literal_move_string 'This is a test platform for badge testing')" \
        "$(literal_move_string 'https://pub-1f3749a8084a44c3abbd97a4875268a1.r2.dev/Logo%20Regular%20-%20Accent-1.png')" \
        "$(literal_move_string 'https://example.com/terms')" \
        "$(literal_move_string 'https://example.com/privacy')" \
        "$(literal_move_vector_empty)" \
        "$(literal_move_vector_from_csv 'https://example.com')" \
        "$(literal_move_string 'Social Network')" \
        none 2 \
        "$(literal_move_string '2023-01-01')" \
        false \
        none none none none none none none \
        "some($(literal_move_string 'https://pub-1f3749a8084a44c3abbd97a4875268a1.r2.dev/mysocial-banner.png'))" none \
        || { press_enter; platform_menu; return; }
    print_success "Submitted."
    press_enter
    platform_menu
}

ptb_platform_create() {
    print_header "platform::create_platform (PTB)"
    read -r -p "PlatformRegistry (mutable shared) ID [${PLATFORM_REGISTRY_ID:-}]: " preg
    preg="${preg:-$PLATFORM_REGISTRY_ID}"
    if [ -z "${PLATFORM_CONFIG_ID:-}" ]; then
        read -r -p "PlatformConfig (shared) ID: " PLATFORM_CONFIG_ID
    fi
    read -r -p "platform name String: " n
    read -r -p "tagline String: " tag
    read -r -p "description String: " desc
    read -r -p "logo_url String (URL text): " logo
    read -r -p "terms_of_service String: " terms
    read -r -p "privacy_policy String: " priv
    read -r -p "additional platform links as comma-separated strings (optional, empty vector[]): " plat_links_in
    if [ -z "$plat_links_in" ]; then
        PL_VEC="$(literal_move_vector_empty)"
    else
        PL_VEC="$(literal_move_vector_from_csv "$plat_links_in")"
    fi
    read -r -p "social links comma-separated (optional, empty vector[]): " lk_in
    if [ -z "$lk_in" ]; then
        LK_VEC="$(literal_move_vector_empty)"
    else
        LK_VEC="$(literal_move_vector_from_csv "$lk_in")"
    fi
    read -r -p "primary_category String: " pc
    read -r -p "secondary_category optional (empty none): " sc
    if [ -z "$sc" ]; then SC_ARG="none"
    else
        scl="$(literal_move_string "$sc")"
        SC_ARG="some(${scl})"
    fi
    read -r -p "status u8 (see platform STATUS_* in platform.move): " st
    read -r -p "release_date String (YYYY-MM-DD): " rd
    read -r -p "wants_dao_governance (true/false): " wdao
    if [ "${wdao}" = "true" ]; then
        read -r -p "delegate_count optional u64: " dc
        read -r -p "delegate_term_epochs optional u64: " dt
        read -r -p "proposal_submission_cost optional u64: " psc
        read -r -p "max_votes_per_user optional u64: " mv
        read -r -p "quadratic_base_cost optional u64: " qb
        read -r -p "voting_period_epochs optional u64: " vp
        read -r -p "quorum_votes optional u64: " qv
        dc_a="some(${dc})"; dt_a="some(${dt})"; psc_a="some(${psc})"; mv_a="some(${mv})"
        qb_a="some(${qb})"; vp_a="some(${vp})"; qv_a="some(${qv})"
    else
        dc_a=none; dt_a=none; psc_a=none; mv_a=none; qb_a=none; vp_a=none; qv_a=none
    fi
    read -r -p "cover_photo optional URL (empty none): " cp
    if [ -z "$cp" ]; then CP_ARG="none"
    else
        cpl="$(literal_move_string "$cp")"
        CP_ARG="some(${cpl})"
    fi
    read -r -p "media_previews comma-separated URLs (empty none): " mp_in
    if [ -z "$mp_in" ]; then MP_ARG="none"
    else
        IFS=',' read -r -a MA <<<"$mp_in"
        acc=""
        s2=""
        for p in "${MA[@]}"; do
            p="${p## }"; p="${p%% }"
            acc="${acc}${s2}$(literal_move_string "$p")"
            s2=", "
        done
        MP_ARG="some(vector[${acc}])"
    fi

    if [ -z "$preg" ] || [ -z "${PLATFORM_CONFIG_ID:-}" ]; then
        print_info "PLATFORM_REGISTRY_ID and PLATFORM_CONFIG_ID required."
        press_enter
        platform_menu
        return
    fi

    invoke_platform_create_ptb \
        "$preg" \
        "$(literal_move_string "$n")" \
        "$(literal_move_string "$tag")" \
        "$(literal_move_string "$desc")" \
        "$(literal_move_string "$logo")" \
        "$(literal_move_string "$terms")" \
        "$(literal_move_string "$priv")" \
        "$PL_VEC" "$LK_VEC" \
        "$(literal_move_string "$pc")" "$SC_ARG" \
        "${st}" \
        "$(literal_move_string "$rd")" \
        "${wdao}" \
        "$dc_a" "$dt_a" "$psc_a" "$mv_a" "$qb_a" "$vp_a" "$qv_a" \
        "$CP_ARG" "$MP_ARG" \
        || { press_enter; platform_menu; return; }

    print_success "Submitted."
    press_enter
    platform_menu
}

# ---- Block list ----

block_list_menu() {
    print_header "Block List Management Menu"
    echo "1. Block Wallet"
    echo "2. Unblock Wallet"
    echo "3. View blocked wallets (read-only note)"
    echo "4. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-4]: " choice

    case $choice in
        1) block_wallet ;;
        2) unblock_wallet ;;
        3) view_blocked_wallets ;;
        4) show_menu ;;
        *) echo "Invalid option" && block_list_menu ;;
    esac
}

block_wallet() {
    print_header "Blocking Wallet"

    read -r -p "Enter BlockListRegistry ID [${BLOCK_LIST_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$BLOCK_LIST_REGISTRY_ID}"
    read -r -p "Enter SocialGraph ID [${SOCIAL_GRAPH_ID:-}]: " social_graph_id
    social_graph_id="${social_graph_id:-$SOCIAL_GRAPH_ID}"
    read -r -p "Enter address to block: " address_to_block

    print_info "Blocking wallet..."
    myso client call --package "$PACKAGE_ID" --module block_list --function block_wallet \
        --args "$registry_id" "$social_graph_id" "$address_to_block" --gas-budget "$GAS_BUDGET"

    print_success "Block transaction submitted."
    press_enter
    block_list_menu
}

unblock_wallet() {
    print_header "Unblocking Wallet"

    read -r -p "Enter BlockListRegistry ID [${BLOCK_LIST_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$BLOCK_LIST_REGISTRY_ID}"
    read -r -p "Enter address to unblock: " address_to_unblock

    print_info "Unblocking wallet..."
    myso client call --package "$PACKAGE_ID" --module block_list --function unblock_wallet \
        --args "$registry_id" "$address_to_unblock" --gas-budget "$GAS_BUDGET"

    print_success "Unblock transaction submitted."
    press_enter
    block_list_menu
}

view_blocked_wallets() {
    print_header "Viewing Blocked Wallets"
    print_info "block_list::get_blocked_wallets is a view (public fun), not a tx entry."
    print_info "Inspect the BlockListRegistry object with the CLI or indexer, or compose a dev inspect call."
    print_info "Suggested registry id: ${BLOCK_LIST_REGISTRY_ID:-not set}"
    press_enter
    block_list_menu
}

# ---- Governance (public entry: myso client call) ----

governance_menu() {
    print_header "Governance Menu"
    echo "1. submit_ecosystem_proposal"
    echo "2. submit_proof_of_creativity_proposal"
    echo "3. rescind_proposal (delegate review phase)"
    echo "4. community_vote_on_proposal"
    echo "5. finalize_proposal (ecosystem / PoC registries)"
    echo "6. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-6]: " choice

    case $choice in
        1) gov_submit_ecosystem ;;
        2) gov_submit_poc ;;
        3) gov_rescind ;;
        4) gov_community_vote ;;
        5) gov_finalize ;;
        6) show_menu ;;
        *) echo "Invalid option" && governance_menu ;;
    esac
}

gov_submit_ecosystem() {
    print_header "governance::submit_ecosystem_proposal"
    read -r -p "Ecosystem GovernanceDAO registry [${GOVERNANCE_ECOSYSTEM_REGISTRY_ID:-}]: " rid
    rid="${rid:-$GOVERNANCE_ECOSYSTEM_REGISTRY_ID}"
    read -r -p "proposal title String: " t1
    read -r -p "description String: " d1
    read -r -p "reference_id as object ID hex (optional, empty none): " ref_i
    if [ -z "$ref_i" ]; then
        ref_arg="none"
    else
        ref_arg="some(@${ref_i})"
    fi
    read -r -p "metadata_json String (optional, empty none): " mj
    if [ -z "$mj" ]; then
        mj_arg="none"
    else
        mjl="$(literal_move_string "$mj")"
        mj_arg="some(${mjl})"
    fi
    read -r -p "Stake Coin<MYSO> object ID (mutable, splits stake fee): " coin_id

    myso client call --package "$PACKAGE_ID" --module governance --function submit_ecosystem_proposal \
        --args "@${rid}" \
        "$(literal_move_string "$t1")" \
        "$(literal_move_string "$d1")" \
        "${ref_arg}" \
        "${mj_arg}" \
        "@${coin_id}" \
        "$CLOCK_ID" \
        --gas-budget "$GAS_BUDGET"

    print_success "Submitted."
    press_enter
    governance_menu
}

gov_submit_poc() {
    print_header "governance::submit_proof_of_creativity_proposal"
    read -r -p "PoC GovernanceDAO registry [${GOVERNANCE_POC_REGISTRY_ID:-}]: " rid
    rid="${rid:-$GOVERNANCE_POC_REGISTRY_ID}"
    read -r -p "title String: " t1
    read -r -p "description String: " d1
    read -r -p "creative_content_id object ID hex (ID types use @ hex): " cid
    read -r -p "metadata_json optional (empty none): " mj
    if [ -z "$mj" ]; then
        mj_arg="none"
    else
        mjl="$(literal_move_string "$mj")"
        mj_arg="some(${mjl})"
    fi
    read -r -p "Stake Coin<MYSO> object ID: " coin_id

    myso client call --package "$PACKAGE_ID" --module governance --function submit_proof_of_creativity_proposal \
        --args "@${rid}" \
        "$(literal_move_string "$t1")" \
        "$(literal_move_string "$d1")" \
        "@${cid}" \
        "${mj_arg}" \
        "@${coin_id}" \
        "$CLOCK_ID" \
        --gas-budget "$GAS_BUDGET"

    print_success "Submitted."
    press_enter
    governance_menu
}

gov_rescind() {
    print_header "governance::rescind_proposal"
    read -r -p "GovernanceDAO registry (must match proposal type): " rid
    read -r -p "Proposal shared object ID: " pid
    myso client call --package "$PACKAGE_ID" --module governance --function rescind_proposal \
        --args "@${rid}" "@${pid}" "$CLOCK_ID" \
        --gas-budget "$GAS_BUDGET"
    print_success "Submitted."
    press_enter
    governance_menu
}

gov_community_vote() {
    print_header "governance::community_vote_on_proposal"
    print_info "proposal must be in STATUS_COMMUNITY_VOTING; coin pays quadratic vote cost when vote_count is greater than 1"
    read -r -p "GovernanceDAO registry ID: " rid
    read -r -p "Proposal shared object ID: " pid
    read -r -p "vote_count (u64, typically 1+): " vc
    read -r -p "approve true or false: " ap
    read -r -p "MYSO Coin object ID for quadratic vote fees: " coin_id
    myso client call --package "$PACKAGE_ID" --module governance --function community_vote_on_proposal \
        --args "@${rid}" "@${pid}" "$vc" "$ap" "@${coin_id}" "$CLOCK_ID" \
        --gas-budget "$GAS_BUDGET"
    print_success "Submitted."
    press_enter
    governance_menu
}

gov_finalize() {
    print_header "governance::finalize_proposal"
    read -r -p "Ecosystem/PoC GovernanceDAO registry ID: " rid
    read -r -p "Proposal shared object ID: " pid
    read -r -p "EcosystemTreasury shared object ID [${ECOSYSTEM_TREASURY_ID:-}]: " et
    et="${et:-$ECOSYSTEM_TREASURY_ID}"

    myso client call --package "$PACKAGE_ID" --module governance --function finalize_proposal \
        --args "@${rid}" "@${pid}" "@${et}" "$CLOCK_ID" \
        --gas-budget "$GAS_BUDGET"

    print_success "Submitted."
    press_enter
    governance_menu
}

# ---- Social proof tokens (public entries) ----

token_exchange_menu() {
    print_header "Social proof tokens"
    echo "1. create_reservation_pool_for_post"
    echo "2. create_reservation_pool_for_profile"
    echo "3. create_social_proof_token"
    echo "4. split_social_token_entry"
    echo "5. merge_social_tokens_entry"
    echo "6. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-6]: " choice

    case $choice in
        1) spt_res_post ;;
        2) spt_res_prof ;;
        3) spt_create_token ;;
        4) spt_split ;;
        5) spt_merge ;;
        6) show_menu ;;
        *) echo "Invalid option" && token_exchange_menu ;;
    esac
}

spt_res_post() {
    print_header "social_proof_tokens::create_reservation_pool_for_post"
    read -r -p "TokenRegistry (mutable) [${TOKEN_REGISTRY_ID:-}]: " tr
    tr="${tr:-$TOKEN_REGISTRY_ID}"
    read -r -p "SocialProofTokensConfig shared object [${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}]: " cfg
    cfg="${cfg:-$SOCIAL_PROOF_TOKENS_CONFIG_ID}"
    read -r -p "Post shared object ID: " post_id
    myso client call --package "$PACKAGE_ID" --module social_proof_tokens --function create_reservation_pool_for_post \
        --args "@${tr}" "@${cfg}" "@${post_id}" \
        --gas-budget "$GAS_BUDGET"
    print_success "Submitted."
    press_enter
    token_exchange_menu
}

spt_res_prof() {
    print_header "social_proof_tokens::create_reservation_pool_for_profile"
    read -r -p "TokenRegistry [${TOKEN_REGISTRY_ID:-}]: " tr
    tr="${tr:-$TOKEN_REGISTRY_ID}"
    read -r -p "SocialProofTokensConfig [${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}]: " cfg
    cfg="${cfg:-$SOCIAL_PROOF_TOKENS_CONFIG_ID}"
    read -r -p "Owned Profile object ID (see profile module): " prof_id
    myso client call --package "$PACKAGE_ID" --module social_proof_tokens --function create_reservation_pool_for_profile \
        --args "@${tr}" "@${cfg}" "@${prof_id}" \
        --gas-budget "$GAS_BUDGET"
    print_success "Submitted."
    press_enter
    token_exchange_menu
}

spt_create_token() {
    print_header "social_proof_tokens::create_social_proof_token"
    read -r -p "TokenRegistry [${TOKEN_REGISTRY_ID:-}]: " tr
    tr="${tr:-$TOKEN_REGISTRY_ID}"
    read -r -p "SocialProofTokensConfig [${SOCIAL_PROOF_TOKENS_CONFIG_ID:-}]: " cfg
    cfg="${cfg:-$SOCIAL_PROOF_TOKENS_CONFIG_ID}"
    read -r -p "ReservationPoolObject mutable shared ID: " pool_id
    myso client call --package "$PACKAGE_ID" --module social_proof_tokens --function create_social_proof_token \
        --args "@${tr}" "@${cfg}" "@${pool_id}" \
        --gas-budget "$GAS_BUDGET"
    print_success "Submitted."
    press_enter
    token_exchange_menu
}

spt_split() {
    print_header "social_proof_tokens::split_social_token_entry"
    read -r -p "SocialToken mutable object ID: " tok
    read -r -p "split_amount u64: " sa
    myso client call --package "$PACKAGE_ID" --module social_proof_tokens --function split_social_token_entry \
        --args "@${tok}" "$sa" \
        --gas-budget "$GAS_BUDGET"
    print_success "Submitted."
    press_enter
    token_exchange_menu
}

spt_merge() {
    print_header "social_proof_tokens::merge_social_tokens_entry"
    read -r -p "token1 mutable object ID (merge into): " t1
    read -r -p "token2 owned object ID (consumed): " t2
    myso client call --package "$PACKAGE_ID" --module social_proof_tokens --function merge_social_tokens_entry \
        --args "@${t1}" "@${t2}" \
        --gas-budget "$GAS_BUDGET"
    print_success "Submitted."
    press_enter
    token_exchange_menu
}

view_object() {
    print_header "View Object Details"

    read -r -p "Enter object ID: " object_id

    print_info "Fetching object data..."
    myso client object "$object_id"

    press_enter
    show_menu
}

# ---- Bootstrap ----

bootstrap_menu() {
    print_header "Bootstrap & Saved Addresses"
    echo "1. Run bootstrap (claim_all_admin_capabilities)"
    echo "2. View saved addresses (interact_addrs.env)"
    echo "3. Record or update saved addresses interactively"
    echo "4. Write example interact_addrs.env template (asks before overwrite)"
    echo "5. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-5]: " choice

    case $choice in
        1) run_bootstrap ;;
        2) view_saved_addresses ;;
        3) record_saved_addresses ;;
        4) write_addrs_template_confirm ;;
        5) show_menu ;;
        *) echo "Invalid option" && bootstrap_menu ;;
    esac
}

run_bootstrap() {
    print_header "Bootstrap: claim_all_admin_capabilities"
    print_info "Calls social_contracts::bootstrap::claim_all_admin_capabilities with:"
    print_info "  - Orderbook Registry (mutable shared object)"
    print_info "  - BootstrapKey (mutable shared object)"
    print_info "  - Clock (${CLOCK_ID})"
    if [ -n "${ORDERBOOK_PACKAGE_ID}" ]; then
        print_info "ORDERBOOK_PACKAGE_ID is set (${ORDERBOOK_PACKAGE_ID}); object IDs normally resolve types without it."
    fi

    read -r -p "Orderbook Registry object ID [${ORDERBOOK_REGISTRY_ID:-}]: " ob_reg
    ob_reg="${ob_reg:-$ORDERBOOK_REGISTRY_ID}"
    read -r -p "BootstrapKey object ID [${BOOTSTRAP_KEY_ID:-}]: " bsk
    bsk="${bsk:-$BOOTSTRAP_KEY_ID}"

    if [ -z "$ob_reg" ] || [ -z "$bsk" ]; then
        print_info "Missing Orderbook registry or BootstrapKey id."
        press_enter
        bootstrap_menu
        return
    fi

    print_info "Submitting bootstrap transaction..."
    myso client call --package "$PACKAGE_ID" --module bootstrap --function claim_all_admin_capabilities \
        --args "$ob_reg" "$bsk" "$CLOCK_ID" --gas-budget "$GAS_BUDGET"

    print_success "Bootstrap transaction executed. Inspect effects for shared object IDs."
    print_info "Use menu option 3 to save addresses into ${INTERACT_ADDRS_FILE}"
    press_enter
    bootstrap_menu
}

view_saved_addresses() {
    print_header "Saved addresses (${INTERACT_ADDRS_FILE})"
    if [ ! -f "${INTERACT_ADDRS_FILE}" ]; then
        print_info "No saved addresses file found. Use option 3 or 4 in the Bootstrap menu to create one."
        press_enter
        bootstrap_menu
        return
    fi
    print_info "--- begin ${INTERACT_ADDRS_FILE} ---"
    cat "${INTERACT_ADDRS_FILE}"
    print_info "--- end ---"
    press_enter
    bootstrap_menu
}

record_saved_addresses() {
    print_header "Record addresses (writes ${INTERACT_ADDRS_FILE})"
    print_info "Press Enter on a line to omit that key from the written file."

    read_one() {
        local key="$1"
        local hint="$2"
        read -r -p "${key} (${hint}): " val
        if [ -n "$val" ]; then
            printf '%s=%s\n' "$key" "$val"
        fi
    }

    outfile="$(mktemp)"
    {
        echo "# interact_addrs.env — sourced by interact.sh"
        echo "# Generated interactively via interact.sh Bootstrap menu."
        echo ""
        read_one "PACKAGE_ID" "social_contracts package"
        read_one "ORDERBOOK_PACKAGE_ID" "optional published orderbook package"
        read_one "ORDERBOOK_REGISTRY_ID" "orderbook registry shared object"
        read_one "BOOTSTRAP_KEY_ID" "framework BootstrapKey shared object before seal"
        read_one "USERNAME_REGISTRY_ID" "profile UsernameRegistry"
        read_one "ECOSYSTEM_TREASURY_ID" "profile EcosystemTreasury"
        read_one "BLOCK_LIST_REGISTRY_ID" "block_list BlockListRegistry"
        read_one "SOCIAL_GRAPH_ID" "social_graph SocialGraph"
        read_one "PLATFORM_REGISTRY_ID" "platform PlatformRegistry"
        read_one "PLATFORM_CONFIG_ID" "platform PlatformConfig"
        read_one "PLATFORM_OBJECT_ID" "platform Platform shared object (for posts/comments)"
        read_one "PLATFORM_ADMIN_CAP_ID" "platform PlatformAdminCap (for approval PTB)"
        read_one "MEMORY_ACCOUNT_ID" "memory MemoryAccount (for post PTBs)"
        read_one "MYDATA_REGISTRY_ID" "mydata MyDataRegistry"
        read_one "MYDATA_CONFIG_ID" "mydata MyDataConfig"
        read_one "SUBSCRIPTION_CONFIG_ID" "subscription SubscriptionConfig"
        read_one "GOVERNANCE_ECOSYSTEM_REGISTRY_ID" "governance ecosystem GovernanceDAO"
        read_one "GOVERNANCE_POC_REGISTRY_ID" "governance PoC GovernanceDAO"
        read_one "POST_CONFIG_ID" "post PostConfig"
        read_one "SOCIAL_PROOF_TOKENS_CONFIG_ID" "SPT shared config object"
        read_one "TOKEN_REGISTRY_ID" "social_proof_tokens TokenRegistry"
        read_one "MESSAGE_REGISTRY_ID" "message registry if used"
        read_one "SPOT_CONFIG_ID" "social_proof_of_truth config"
        read_one "INSURANCE_CONFIG_ID" "insurance config"
    } >"${outfile}"

    mv "${outfile}" "${INTERACT_ADDRS_FILE}"
    chmod 600 "${INTERACT_ADDRS_FILE}" 2>/dev/null || true

    print_success "Saved to ${INTERACT_ADDRS_FILE}. Re-run the script or re-source this file manually to load variables."
    press_enter
    bootstrap_menu
}

write_addrs_template_confirm() {
    print_header "Example interact_addrs.env"
    if [ -f "${INTERACT_ADDRS_FILE}" ]; then
        read -r -p "File exists. Overwrite? [y/N]: " ok
        if [ "${ok}" != "y" ] && [ "${ok}" != "Y" ]; then
            bootstrap_menu
            return
        fi
    fi
    cat >"${INTERACT_ADDRS_FILE}" <<'EOF'
# Copy and fill IDs from chain explorer / genesis after publish and bootstrap.
# Lines are shell assignments; omit or comment keys you do not use.

PACKAGE_ID=
ORDERBOOK_PACKAGE_ID=
ORDERBOOK_REGISTRY_ID=
BOOTSTRAP_KEY_ID=
USERNAME_REGISTRY_ID=
ECOSYSTEM_TREASURY_ID=
BLOCK_LIST_REGISTRY_ID=
SOCIAL_GRAPH_ID=
PLATFORM_REGISTRY_ID=
PLATFORM_CONFIG_ID=
PLATFORM_OBJECT_ID=
PLATFORM_ADMIN_CAP_ID=
MEMORY_ACCOUNT_ID=
MYDATA_REGISTRY_ID=
MYDATA_CONFIG_ID=
GOVERNANCE_ECOSYSTEM_REGISTRY_ID=
GOVERNANCE_POC_REGISTRY_ID=
POST_CONFIG_ID=
SOCIAL_PROOF_TOKENS_CONFIG_ID=
TOKEN_REGISTRY_ID=
MESSAGE_REGISTRY_ID=
SPOT_CONFIG_ID=
INSURANCE_CONFIG_ID=
EOF
    print_success "Wrote template to ${INTERACT_ADDRS_FILE}"
    press_enter
    bootstrap_menu
}

# ---- Upgrade migrations (entry-verified names) ----
#
# Subscription & MyData platform/ecosystem fees (post-upgrade):
#   1. Call subscription::migrate_config and mydata::migrate_config on shared configs.
#   2. Client entry functions now require &EcosystemTreasury on payment paths:
#        subscription::{subscribe_to_profile,renew_subscription,auto_renew_subscription}
#        mydata::{purchase_one_time,purchase_subscription,claim}
#   3. Optional platform routing via *_with_platform variants:
#        subscription::{subscribe_to_profile_with_platform,...}
#        mydata::{purchase_one_time_with_platform,purchase_subscription_with_platform,claim_with_platform}

upgrade_menu() {
    print_header "Upgrade Management Menu"
    echo "1. Migrate MyData object"
    echo "2. Migrate MyData registry"
    echo "3. Migrate Social Graph"
    echo "4. Migrate UsernameRegistry (profile)"
    echo "5. Migrate Post config"
    echo "6. Migrate MyData config (fee fields)"
    echo "7. Migrate Subscription config (fee fields)"
    echo "8. Back to Main Menu"
    echo ""
    read -r -p "Select an option [1-8]: " choice

    case $choice in
        1) migrate_mydata ;;
        2) migrate_mydata_registry ;;
        3) migrate_social_graph ;;
        4) migrate_username_registry ;;
        5) migrate_post_config ;;
        6) migrate_mydata_config ;;
        7) migrate_subscription_config ;;
        8) show_menu ;;
        *) echo "Invalid option" && upgrade_menu ;;
    esac
}

migrate_mydata() {
    print_header "Migrating MyData"

    read -r -p "Enter MyData object ID: " mydata_id
    read -r -p "Enter UpgradeAdminCap object ID: " admin_cap_id

    print_info "Migrating MyData..."
    myso client call --package "$PACKAGE_ID" --module mydata --function migrate_mydata \
        --args "$mydata_id" "$admin_cap_id" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    upgrade_menu
}

migrate_mydata_registry() {
    print_header "Migrating MyData Registry"

    read -r -p "Enter MyDataRegistry ID [${MYDATA_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$MYDATA_REGISTRY_ID}"
    read -r -p "Enter UpgradeAdminCap object ID: " admin_cap_id

    print_info "Migrating registry..."
    myso client call --package "$PACKAGE_ID" --module mydata --function migrate_registry \
        --args "$registry_id" "$admin_cap_id" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    upgrade_menu
}

migrate_social_graph() {
    print_header "Migrating Social Graph"

    read -r -p "Enter SocialGraph ID [${SOCIAL_GRAPH_ID:-}]: " graph_id
    graph_id="${graph_id:-$SOCIAL_GRAPH_ID}"
    read -r -p "Enter UpgradeAdminCap object ID: " admin_cap_id

    print_info "Migrating social graph..."
    myso client call --package "$PACKAGE_ID" --module social_graph --function migrate_social_graph \
        --args "$graph_id" "$admin_cap_id" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    upgrade_menu
}

migrate_username_registry() {
    print_header "Migrating UsernameRegistry"

    read -r -p "Enter UsernameRegistry ID [${USERNAME_REGISTRY_ID:-}]: " registry_id
    registry_id="${registry_id:-$USERNAME_REGISTRY_ID}"
    read -r -p "Enter UpgradeAdminCap object ID: " admin_cap_id

    print_info "Migrating UsernameRegistry..."
    myso client call --package "$PACKAGE_ID" --module profile --function migrate_registry \
        --args "$registry_id" "$admin_cap_id" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    upgrade_menu
}

migrate_post_config() {
    print_header "Migrating Post Config"

    read -r -p "Enter PostConfig object ID [${POST_CONFIG_ID:-}]: " config_id
    config_id="${config_id:-$POST_CONFIG_ID}"
    read -r -p "Enter UpgradeAdminCap object ID: " admin_cap_id

    print_info "Migrating PostConfig..."
    myso client call --package "$PACKAGE_ID" --module post --function migrate_post_config \
        --args "$config_id" "$admin_cap_id" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    upgrade_menu
}

migrate_mydata_config() {
    print_header "Migrating MyData Config (fee bps fields)"

    read -r -p "Enter MyDataConfig ID [${MYDATA_CONFIG_ID:-}]: " config_id
    config_id="${config_id:-$MYDATA_CONFIG_ID}"
    read -r -p "Enter UpgradeAdminCap object ID: " admin_cap_id

    print_info "Calling mydata::migrate_config ..."
    myso client call --package "$PACKAGE_ID" --module mydata --function migrate_config \
        --args "$config_id" "$admin_cap_id" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    upgrade_menu
}

migrate_subscription_config() {
    print_header "Migrating Subscription Config (fee bps fields)"

    read -r -p "Enter SubscriptionConfig ID [${SUBSCRIPTION_CONFIG_ID:-}]: " config_id
    config_id="${config_id:-$SUBSCRIPTION_CONFIG_ID}"
    read -r -p "Enter UpgradeAdminCap object ID: " admin_cap_id

    print_info "Calling subscription::migrate_config ..."
    myso client call --package "$PACKAGE_ID" --module subscription --function migrate_config \
        --args "$config_id" "$admin_cap_id" --gas-budget "$GAS_BUDGET"

    print_success "Done."
    press_enter
    upgrade_menu
}

# ---- Entry ----

print_header "MySocial Contract Interaction Tool"
print_info "social_contracts PACKAGE_ID: ${PACKAGE_ID}"
if [ -f "${INTERACT_ADDRS_FILE}" ]; then
    print_info "Loaded addresses from ${INTERACT_ADDRS_FILE}"
else
    print_info "No ${INTERACT_ADDRS_FILE}; optional. Bootstrap menu can create it."
fi
show_menu
