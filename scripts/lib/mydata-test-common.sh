#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Shared MyData encrypt / key-server helpers for E2E scripts (subscription test, marketplace).

if [[ -n "${_MYDATA_TEST_COMMON_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_MYDATA_TEST_COMMON_SOURCED=1

: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

readonly MYDATA_DEFAULT_KEY_SERVER_URL='http://127.0.0.1:2024'
readonly MYDATA_DEFAULT_SECRETS_REL='network.config/mydata/local-mydata-secrets.env'
readonly MYDATA_G2_PUBLIC_KEY_HEX_LEN=192

readonly MYDATA_DEFAULT_MAX_TAGS='10'
readonly MYDATA_DEFAULT_MAX_SUBSCRIPTION_DAYS='365'
readonly MYDATA_DEFAULT_MAX_FREE_ACCESS_GRANTS='100000'
readonly MYDATA_DEFAULT_MAX_ENCRYPTION_ID_BYTES='1024'
readonly MYDATA_DEFAULT_P2P_PLATFORM_FEE_BPS='250'
readonly MYDATA_DEFAULT_P2P_ECOSYSTEM_FEE_BPS='250'
readonly MYDATA_DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS='250'
readonly MYDATA_DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS='250'
readonly MYDATA_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS='0'
readonly MYDATA_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS='10000'

readonly MYDATA_CONFIG_GQL='query MyDataConfiguration {
  mydataConfiguration {
    marketplaceEnabled
    maxTags
    maxSubscriptionDays
    maxFreeAccessGrants
    maxEncryptionIdBytes
    p2PPlatformFeeBps
    p2PEcosystemFeeBps
    mydataMarketplacePlatformFeeBps
    mydataMarketplaceEcosystemFeeBps
    nonPlatformPlatformToCreatorBps
    nonPlatformPlatformToTreasuryBps
  }
}'

KEY_SERVER_URL="${KEY_SERVER_URL:-$MYDATA_DEFAULT_KEY_SERVER_URL}"
MYDATA_SECRETS_FILE="${MYDATA_SECRETS_FILE:-}"
PUBLIC_KEY="${PUBLIC_KEY:-}"
KEY_SERVER_OBJECT_ID="${KEY_SERVER_OBJECT_ID:-}"
MYDATA_CONFIG_ID="${MYDATA_CONFIG_ID:-}"
MYDATA_ADMIN_CAP_ID="${MYDATA_ADMIN_CAP_ID:-}"
MYDATA_ID="${MYDATA_ID:-}"
ENCRYPTION_ID_HEX="${ENCRYPTION_ID_HEX:-}"
ENCRYPT_OUT_HEX="${ENCRYPT_OUT_HEX:-}"
ENCRYPT_CIPHERTEXT_HEX="${ENCRYPT_CIPHERTEXT_HEX:-}"
ENCRYPTED_PLAINTEXT_EXPECTED="${ENCRYPTED_PLAINTEXT_EXPECTED:-}"

MYDATA_CFG_MARKETPLACE_ENABLED=''
MYDATA_CFG_MAX_TAGS=''
MYDATA_CFG_MAX_SUBSCRIPTION_DAYS=''
MYDATA_CFG_MAX_FREE_ACCESS_GRANTS=''
MYDATA_CFG_MAX_ENCRYPTION_ID_BYTES=''
MYDATA_CFG_P2P_PLATFORM_FEE_BPS=''
MYDATA_CFG_P2P_ECOSYSTEM_FEE_BPS=''
MYDATA_CFG_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS=''
MYDATA_CFG_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS=''
MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS=''
MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS=''

mydata_strip_0x() {
    local x="${1:-}"
    x="${x#0x}"
    x="${x#0X}"
    printf '%s' "$x"
}

mydata_parse_env_file_value() {
    local file="$1"
    local key="$2"
    [[ -f "$file" ]] || return 1
    local line
    line="$(grep -E "^[[:space:]]*${key}=" "$file" | tail -n1)" || return 1
    [[ -n "$line" ]] || return 1
    line="${line#*=}"
    line="${line//$'\r'/}"
    if [[ "$line" =~ ^\"(.*)\"$ ]]; then line="${BASH_REMATCH[1]}"; fi
    if [[ "$line" =~ ^\'(.*)\'$ ]]; then line="${BASH_REMATCH[1]}"; fi
    printf '%s' "$line"
}

mydata_default_secrets_env_path() {
    if [[ -f "$PWD/$MYDATA_DEFAULT_SECRETS_REL" ]]; then
        printf '%s' "$PWD/$MYDATA_DEFAULT_SECRETS_REL"
    elif [[ -f "$REPO_ROOT/$MYDATA_DEFAULT_SECRETS_REL" ]]; then
        printf '%s' "$REPO_ROOT/$MYDATA_DEFAULT_SECRETS_REL"
    else
        printf '%s' "$REPO_ROOT/$MYDATA_DEFAULT_SECRETS_REL"
    fi
}

mydata_resolve_secrets_env_path() {
    local path="${1:-}"
    local dir sibling
    if [[ -z "$path" ]]; then
        mydata_default_secrets_env_path
        return 0
    fi
    if [[ "$path" == *.yaml || "$path" == *.yml ]]; then
        dir="$(dirname "$path")"
        sibling="${dir}/local-mydata-secrets.env"
        if [[ -f "$sibling" ]]; then
            printf '%s' "$sibling"
            return 0
        fi
    fi
    printf '%s' "$path"
}

mydata_validate_g2_public_key_hex() {
    local pk="${1:-}"
    local naked len
    naked="$(mydata_strip_0x "$pk")"
    len="${#naked}"
    if [[ "$len" -ne "$MYDATA_G2_PUBLIC_KEY_HEX_LEN" ]]; then
        echo "PUBLIC_KEY invalid: ${len} hex chars; need ${MYDATA_G2_PUBLIC_KEY_HEX_LEN}" >&2
        return 1
    fi
    [[ "$naked" =~ ^[0-9a-fA-F]+$ ]]
}

mydata_hydrate_encrypt_from_secrets_file() {
    local sec_file="$1"
    local overwrite_pk="${2:-0}"
    [[ -f "$sec_file" ]] || return 0
    local pk_from_file ks_from_file u_from_file
    pk_from_file="$(mydata_parse_env_file_value "$sec_file" PUBLIC_KEY 2>/dev/null || true)"
    ks_from_file="$(mydata_parse_env_file_value "$sec_file" KEY_SERVER_OBJECT_ID 2>/dev/null || true)"
    u_from_file="$(mydata_parse_env_file_value "$sec_file" KEY_SERVER_URL 2>/dev/null || true)"
    if [[ "$overwrite_pk" == 1 ]] || [[ -z "${PUBLIC_KEY:-}" ]] || ! mydata_validate_g2_public_key_hex "${PUBLIC_KEY:-}" 2>/dev/null; then
        if [[ -n "${pk_from_file:-}" ]] && mydata_validate_g2_public_key_hex "$pk_from_file" 2>/dev/null; then
            PUBLIC_KEY="$pk_from_file"
        fi
    fi
    if [[ -z "${KEY_SERVER_OBJECT_ID:-}" && -n "${ks_from_file:-}" ]]; then
        KEY_SERVER_OBJECT_ID="$ks_from_file"
    fi
    if [[ -n "${u_from_file:-}" ]]; then
        KEY_SERVER_URL="$u_from_file"
    fi
}

mydata_hydrate_encrypt_from_secrets() {
    local sec_path resolved
    if [[ -n "${MYDATA_SECRETS_FILE:-}" ]]; then
        sec_path="$(mydata_resolve_secrets_env_path "$MYDATA_SECRETS_FILE")"
    else
        sec_path="$(mydata_default_secrets_env_path)"
    fi
    resolved="$(mydata_resolve_secrets_env_path "$sec_path")"
    if [[ -f "$resolved" ]]; then
        MYDATA_SECRETS_FILE="$resolved"
        local overwrite=0
        if [[ -n "${PUBLIC_KEY:-}" ]] && ! mydata_validate_g2_public_key_hex "${PUBLIC_KEY:-}" 2>/dev/null; then
            overwrite=1
        fi
        mydata_hydrate_encrypt_from_secrets_file "$resolved" "$overwrite"
    fi
}

mydata_resolve_mydata_repo_root() {
    local root="${MYDATA_REPO:-}"
    if [[ -z "$root" ]]; then
        if [[ -n "${MYSO_MYDATA_REPO:-}" ]]; then
            root="$MYSO_MYDATA_REPO"
        elif [[ -d "$REPO_ROOT/../myso-mydata" && -f "$REPO_ROOT/../myso-mydata/Cargo.toml" ]]; then
            root="$(cd "$REPO_ROOT/../myso-mydata" && pwd)"
        fi
    fi
    printf '%s' "${root:-}"
}

mydata_resolve_mydata() {
    if [[ -n "${MYDATA:-}" && -x "${MYDATA}" ]]; then
        echo "$MYDATA"
        return 0
    fi
    local root cand
    root="$(mydata_resolve_mydata_repo_root)"
    if [[ -n "$root" ]]; then
        for cand in "$root/target/release/mydata" "$root/target/debug/mydata"; do
            if [[ -x "$cand" ]]; then
                echo "$cand"
                return 0
            fi
        done
    fi
    if command -v mydata &>/dev/null; then
        command -v mydata
        return 0
    fi
    echo ""
}

mydata_probe_key_server() {
    local base="${1%/}"
    local code curl_ec=0
    code="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 5 "${base}/service" 2>/dev/null)" || curl_ec=$?
    if [[ "$curl_ec" -ne 0 || -z "$code" || "$code" == "000" ]]; then
        echo "Key server unreachable at ${base} (curl exit ${curl_ec})" >&2
        return 1
    fi
    if [[ "$code" -ge 500 ]]; then
        echo "Key server ${base}/service returned HTTP ${code}" >&2
        return 1
    fi
    return 0
}

# Sets ENCRYPT_OUT_HEX and ENCRYPTION_ID_HEX
mydata_run_encrypt_hmac_cli() {
    local mydata_cli="$1"
    local msg_hex="$2"
    local package_id="$3"
    local id_hex="$4"
    local threshold="$5"
    local pk_naked_hex="$6"
    local ks_object_id="$7"
    local aad_hex="${8:-}"

    local -a cmd
    cmd=("$mydata_cli" encrypt-hmac --message "$msg_hex")
    if [[ -n "$aad_hex" ]]; then
        cmd+=(--aad "$aad_hex")
    fi
    cmd+=(--package-id "$package_id" --id "$id_hex" --threshold "$threshold" "$pk_naked_hex" -- "$ks_object_id")

    local out ec
    set +e
    out="$("${cmd[@]}" 2>&1)"
    ec=$?
    set -e
    if [[ $ec -ne 0 ]]; then
        echo "mydata encrypt-hmac failed:" >&2
        echo "$out" >&2
        return "$ec"
    fi

    local bcs_hex
    bcs_hex="$(printf '%s' "$out" | sed -n 's/^Encrypted object (bcs): //p' | head -n1)"
    if [[ -n "$bcs_hex" ]]; then
        ENCRYPT_OUT_HEX="$(mydata_strip_0x "$bcs_hex")"
    else
        local hex_line best cand len
        hex_line=""
        best=0
        while IFS= read -r cand; do
            cand="$(mydata_strip_0x "$cand")"
            [[ "$cand" == "$(mydata_strip_0x "$id_hex")" ]] && continue
            [[ "$cand" == "$pk_naked_hex" ]] && continue
            len="${#cand}"
            if [[ "$len" -gt "$best" && "$len" -ge 64 && $((len % 2)) -eq 0 ]]; then
                best="$len"
                hex_line="$cand"
            fi
        done < <(printf '%s' "$out" | grep -Eo '(0x)?[0-9a-fA-F]+' || true)
        ENCRYPT_OUT_HEX="$hex_line"
    fi
    if [[ ${#ENCRYPT_OUT_HEX} -lt 64 ]]; then
        echo "Could not parse encrypt-hmac ciphertext hex. Raw output:" >&2
        echo "$out" >&2
        return 1
    fi
    ENCRYPTION_ID_HEX="$(printf '%s' "$id_hex" | tr 'A-F' 'a-f')"
    ENCRYPT_CIPHERTEXT_HEX="$ENCRYPT_OUT_HEX"
    return 0
}

mydata_resolve_encrypt_credentials() {
    local sec_path default_sec
    default_sec="$(mydata_default_secrets_env_path)"
    if session_value_set PUBLIC_KEY && session_value_set KEY_SERVER_OBJECT_ID && \
        mydata_validate_g2_public_key_hex "${PUBLIC_KEY:-}" 2>/dev/null; then
        if [[ -f "${MYDATA_SECRETS_FILE:-$default_sec}" ]]; then
            MYDATA_SECRETS_FILE="$(mydata_resolve_secrets_env_path "${MYDATA_SECRETS_FILE:-$default_sec}")"
        fi
        return 0
    fi
    if [[ -n "${MYDATA_SECRETS_FILE:-}" ]]; then
        sec_path="$(mydata_resolve_secrets_env_path "$MYDATA_SECRETS_FILE")"
    else
        sec_path="$default_sec"
    fi
    if [[ -f "$sec_path" ]]; then
        mydata_hydrate_encrypt_from_secrets_file "$sec_path" 1
        MYDATA_SECRETS_FILE="$sec_path"
    fi
    [[ -n "${KEY_SERVER_URL:-}" ]] || KEY_SERVER_URL="$MYDATA_DEFAULT_KEY_SERVER_URL"
    [[ -n "${KEY_SERVER_OBJECT_ID:-}" ]] || {
        echo "KEY_SERVER_OBJECT_ID is required (from myso start --with-mydata secrets)" >&2
        return 1
    }
    mydata_validate_g2_public_key_hex "${PUBLIC_KEY:-}" || return 1
}

mydata_load_mydata_config_params_from_graphql() {
    local resp
    resp="$(graphql_post "$MYDATA_CONFIG_GQL" 2>/dev/null)" || return 1
    MYDATA_CFG_MARKETPLACE_ENABLED="$(echo "$resp" | jq -r '.data.mydataConfiguration.marketplaceEnabled // empty')"
    MYDATA_CFG_MAX_TAGS="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxTags // empty')"
    MYDATA_CFG_MAX_SUBSCRIPTION_DAYS="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxSubscriptionDays // empty')"
    MYDATA_CFG_MAX_FREE_ACCESS_GRANTS="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxFreeAccessGrants // empty')"
    MYDATA_CFG_MAX_ENCRYPTION_ID_BYTES="$(echo "$resp" | jq -r '.data.mydataConfiguration.maxEncryptionIdBytes // empty')"
    MYDATA_CFG_P2P_PLATFORM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.p2PPlatformFeeBps // empty')"
    MYDATA_CFG_P2P_ECOSYSTEM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.p2PEcosystemFeeBps // empty')"
    MYDATA_CFG_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.mydataMarketplacePlatformFeeBps // empty')"
    MYDATA_CFG_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.mydataMarketplaceEcosystemFeeBps // empty')"
    MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.nonPlatformPlatformToCreatorBps // empty')"
    MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS="$(echo "$resp" | jq -r '.data.mydataConfiguration.nonPlatformPlatformToTreasuryBps // empty')"
}

mydata_run_update_config_call() {
    local sender="$1"
    local marketplace_enabled="$2" max_tags="$3" max_sub="$4" max_grants="$5" max_enc_id="$6"
    local p2p_plat="$7" p2p_eco="$8" md_plat="$9" md_eco="${10}" np_creator="${11}" np_treasury="${12}"
    require_session_fields MYDATA_ADMIN_CAP_ID MYDATA_CONFIG_ID CLOCK_ID || return 1
    run_myso_call_as_capture "$sender" mydata update_mydata_config \
        "@$(normalize_hex_id "$MYDATA_ADMIN_CAP_ID")" \
        "@$(normalize_hex_id "$MYDATA_CONFIG_ID")" \
        "$marketplace_enabled" \
        "$max_tags" "$max_sub" "$max_grants" "$max_enc_id" \
        "$p2p_plat" "$p2p_eco" "$md_plat" "$md_eco" "$np_creator" "$np_treasury" \
        "@$(normalize_hex_id "$CLOCK_ID")"
}

# Args: creator_address plaintext
mydata_ensure_marketplace_enabled() {
    local sender="$1"
    [[ -n "${MYDATA_CONFIG_ID:-}" ]] || return 0
    [[ -n "${MYDATA_ADMIN_CAP_ID:-}" ]] || {
        echo "Note: MYDATA_ADMIN_CAP_ID unset; create_and_share may abort if marketplace_enabled=false" >&2
        return 0
    }
    mydata_load_mydata_config_params_from_graphql || true
    if [[ "${MYDATA_CFG_MARKETPLACE_ENABLED:-false}" == "true" ]]; then
        return 0
    fi
    log_step "Enabling MyDataConfig.marketplace_enabled for create_and_share"
    SKIP_CONFIRM_RUN=1 mydata_run_update_config_call "$sender" true \
        "${MYDATA_CFG_MAX_TAGS:-$MYDATA_DEFAULT_MAX_TAGS}" \
        "${MYDATA_CFG_MAX_SUBSCRIPTION_DAYS:-$MYDATA_DEFAULT_MAX_SUBSCRIPTION_DAYS}" \
        "${MYDATA_CFG_MAX_FREE_ACCESS_GRANTS:-$MYDATA_DEFAULT_MAX_FREE_ACCESS_GRANTS}" \
        "${MYDATA_CFG_MAX_ENCRYPTION_ID_BYTES:-$MYDATA_DEFAULT_MAX_ENCRYPTION_ID_BYTES}" \
        "${MYDATA_CFG_P2P_PLATFORM_FEE_BPS:-$MYDATA_DEFAULT_P2P_PLATFORM_FEE_BPS}" \
        "${MYDATA_CFG_P2P_ECOSYSTEM_FEE_BPS:-$MYDATA_DEFAULT_P2P_ECOSYSTEM_FEE_BPS}" \
        "${MYDATA_CFG_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS:-$MYDATA_DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS}" \
        "${MYDATA_CFG_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS:-$MYDATA_DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS}" \
        "${MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS:-$MYDATA_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS}" \
        "${MYDATA_CFG_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS:-$MYDATA_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS}" \
        || return 1
}

# Args: creator_address plaintext
# Sets MYDATA_ID, ENCRYPTION_ID_HEX, ENCRYPT_CIPHERTEXT_HEX, ENCRYPTED_PLAINTEXT_EXPECTED
mydata_create_and_share_encrypted() {
    local creator="$1"
    local plaintext="$2"
    local mydata_bin enc_id msg_hex pk_naked out digest

    require_session_fields MYDATA_CONFIG_ID MYDATA_REGISTRY_ID PKG_SOCIAL CLOCK_ID || return 1
    mydata_resolve_encrypt_credentials || return 1
    mydata_bin="$(mydata_resolve_mydata)"
    [[ -n "$mydata_bin" ]] || {
        echo "mydata CLI not found; build myso-mydata (cargo build -p mydata-cli)" >&2
        return 1
    }
    mydata_probe_key_server "$KEY_SERVER_URL" || return 1
    mydata_ensure_marketplace_enabled "$creator" || return 1

    ENCRYPTED_PLAINTEXT_EXPECTED="$plaintext"
    msg_hex="$(printf '%s' "$plaintext" | xxd -p -c 65536 | tr -d '\n')"
    enc_id="$(openssl rand -hex 32)"
    pk_naked="$(mydata_strip_0x "$PUBLIC_KEY")"

    log_step "Encrypting post body (encrypt-hmac, package=$PKG_SOCIAL)"
    mydata_run_encrypt_hmac_cli "$mydata_bin" "$msg_hex" "$PKG_SOCIAL" "$enc_id" 1 "$pk_naked" "$KEY_SERVER_OBJECT_ID" || return 1

    local enc_arg id_arg
    enc_arg="\"0x${ENCRYPT_OUT_HEX}\""
    id_arg="\"0x${ENCRYPTION_ID_HEX}\""

    log_step "create_and_share MyData (subscription-gated, no marketplace prices)"
    out="$(SKIP_CONFIRM_RUN=1 run_myso_call_as_capture "$creator" mydata create_and_share \
        "@$(normalize_hex_id "$MYDATA_CONFIG_ID")" \
        "@$(normalize_hex_id "$MYDATA_REGISTRY_ID")" \
        "$(literal_move_string "subscription-post:bf-hmac")" \
        '["subscription-e2e"]' \
        '[]' 0 '[]' \
        "$enc_arg" "$id_arg" '[]' '[]' 30 '[]' '[]' '[]' '[]' false '[]' \
        "@$(normalize_hex_id "$CLOCK_ID")")" || return 1
    assert_tx_success "$out" || return 1
    digest="$(extract_tx_digest "$out")"
    MYDATA_ID="$(extract_created_object_by_type "$digest" "mydata::MyData")" || return 1
    log_session_use "MYDATA_ID" "$MYDATA_ID"
    log_session_use "ENCRYPTION_ID_HEX" "$ENCRYPTION_ID_HEX"
    return 0
}
