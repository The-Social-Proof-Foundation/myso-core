#!/usr/bin/env bash
# Copyright (c) Mysten Labs, Inc.
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Interactive helper for social_contracts::mydata Move calls via `myso client call`.
#
# Prerequisites:
#   - MySocial bootstrap has run (social_contracts::bootstrap claim_all_admin_capabilities).
#   - Shared MyData objects exist (mydata::bootstrap_init). Resolve IDs from GraphQL / explorer
#     (e.g. types ...::mydata::MyDataConfig, MyDataRegistry, MyDataPoolRegistry, ...).
#   - For listings: MyDataConfig.enable_flag must be true (menu 1).
#
# Production-like encryption (menu 2):
#   - Requires `mydata-cli` from the myso-mydata repo (same as myso start --with-mydata).
#   - Runs `mydata-cli encrypt-hmac` per crates/myso-framework/.../bf_hmac_encryption.move comments.
#   - Default key server http://127.0.0.1:2024: probes /service (HTTP 4xx still counts as reachable).
#   - Loads PUBLIC_KEY and KEY_SERVER_OBJECT_ID from network.config/mydata/local-mydata-secrets.env
#     (written by myso start --with-mydata) or prompts.
#   - --package-id for encrypt is the MySoSocial package (0x50c1); ciphertext must match create_and_share
#     EncryptedObject.package_id for permissioned key-server flows.
#
# Environment:
#   MYSO              Path to myso binary (optional)
#   MYDATA_REPO / MYSO_MYDATA_REPO   Path to myso-mydata for mydata-cli
#   DRY_RUN=1         Pass --dry-run to myso client
#   MYDATA_MARKETPLACE_SESSION   Path to saved session env file (optional).
#                                   Default save: <repo>/network.config/mydata/marketplace-session.env
#                                   Load tries: this var, then ./network.config/.../ then repo path.
#   MYDATA_MARKETPLACE_NO_SAVE=1 Skip writing session file after menu 0 / encrypt flow.
#   ASSUME_YES=1 / -y          Non-interactive yes for confirm_run and enable_flag prompt.
#
# Usage: ./scripts/mydata-marketplace-runnable.sh   [-y] [--help] [--no-session]
# Rename/link as runnable.sh if you prefer.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

readonly DEFAULT_PKG_SOCIAL='0x00000000000000000000000000000000000000000000000000000000000050c1'
readonly DEFAULT_CLOCK='0x0000000000000000000000000000000000000000000000000000000000000006'
readonly DEFAULT_COIN_TYPE='0x2::myso::MYSO'
readonly DEFAULT_KEY_SERVER_URL='http://127.0.0.1:2024'
readonly DEFAULT_SECRETS_REL='network.config/mydata/local-mydata-secrets.env'

CLIENT_CONFIG=''
PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
CLOCK_ID="$DEFAULT_CLOCK"
COIN_TYPE="$DEFAULT_COIN_TYPE"
KEY_SERVER_URL="$DEFAULT_KEY_SERVER_URL"

MYDATA_CONFIG_ID=''
MYDATA_REGISTRY_ID=''
POOL_REGISTRY_ID=''
ANCHOR_REGISTRY_ID=''
CLAIM_VAULT_ID=''
DIST_REGISTRY_ID=''
MYDATA_ADMIN_CAP_ID=''
POOL_ADMIN_CAP_ID=''
GAS_COIN_ID=''
LISTING_ID=''
PAY_COIN_ID=''
MYDATA_SECRETS_FILE=''
PUBLIC_KEY=''
KEY_SERVER_OBJECT_ID=''

# Skip loading/writing session file (--no-session)
NO_SESSION_FILE=0

session_state_save_path() {
    if [[ -n "${MYDATA_MARKETPLACE_SESSION:-}" ]]; then
        printf '%s' "$MYDATA_MARKETPLACE_SESSION"
    else
        printf '%s' "$REPO_ROOT/network.config/mydata/marketplace-session.env"
    fi
}

apply_session_defaults() {
    [[ -n "${PKG_SOCIAL:-}" ]] || PKG_SOCIAL="$DEFAULT_PKG_SOCIAL"
    [[ -n "${CLOCK_ID:-}" ]] || CLOCK_ID="$DEFAULT_CLOCK"
    [[ -n "${COIN_TYPE:-}" ]] || COIN_TYPE="$DEFAULT_COIN_TYPE"
    [[ -n "${KEY_SERVER_URL:-}" ]] || KEY_SERVER_URL="$DEFAULT_KEY_SERVER_URL"
}

load_session_state() {
    [[ "${NO_SESSION_FILE:-0}" == 1 ]] && return 0
    local paths p loaded=0
    paths=()
    [[ -n "${MYDATA_MARKETPLACE_SESSION:-}" ]] && paths+=("$MYDATA_MARKETPLACE_SESSION")
    paths+=("$PWD/network.config/mydata/marketplace-session.env")
    paths+=("$REPO_ROOT/network.config/mydata/marketplace-session.env")
    for p in "${paths[@]}"; do
        [[ -n "$p" && -f "$p" ]] || continue
        # shellcheck disable=SC1090
        source "$p"
        echo "Loaded MyData marketplace session from: $p" >&2
        loaded=1
        break
    done
    apply_session_defaults
    _secrets_merge=''
    if [[ -f "$PWD/$DEFAULT_SECRETS_REL" ]]; then
        _secrets_merge="$PWD/$DEFAULT_SECRETS_REL"
    elif [[ -f "$REPO_ROOT/$DEFAULT_SECRETS_REL" ]]; then
        _secrets_merge="$REPO_ROOT/$DEFAULT_SECRETS_REL"
    fi
    if [[ -n "$_secrets_merge" ]]; then
        _u="$(parse_env_file_value "$_secrets_merge" KEY_SERVER_URL 2>/dev/null || true)"
        [[ -n "${_u:-}" ]] && KEY_SERVER_URL="$_u"
    fi
}

save_session_state() {
    [[ "${NO_SESSION_FILE:-0}" == 1 ]] && return 0
    [[ "${MYDATA_MARKETPLACE_NO_SAVE:-}" == 1 ]] && return 0
    local f
    f="$(session_state_save_path)"
    mkdir -p "$(dirname "$f")"
    local old_umask
    old_umask="$(umask)"
    umask 077
    {
        echo "# Local session for scripts/mydata-marketplace-runnable.sh — paths/ids only; do not commit if sensitive."
        local key
        for key in CLIENT_CONFIG PKG_SOCIAL CLOCK_ID COIN_TYPE KEY_SERVER_URL MYDATA_CONFIG_ID MYDATA_REGISTRY_ID POOL_REGISTRY_ID ANCHOR_REGISTRY_ID CLAIM_VAULT_ID DIST_REGISTRY_ID MYDATA_ADMIN_CAP_ID POOL_ADMIN_CAP_ID GAS_COIN_ID LISTING_ID PAY_COIN_ID MYDATA_SECRETS_FILE PUBLIC_KEY KEY_SERVER_OBJECT_ID; do
            printf '%s=%q\n' "$key" "${!key-}"
        done
    } > "${f}.tmp"
    mv "${f}.tmp" "$f"
    umask "$old_umask"
    echo "Saved session to: $f" >&2
}

usage() {
    sed -n '2,40p' "$0" | sed 's/^# \?//'
}

strip_0x() {
    local x="${1:-}"
    x="${x#0x}"
    x="${x#0X}"
    printf '%s' "$x"
}

prompt_with_default() {
    local label="$1"
    local default="$2"
    local _read
    if [[ -n "$default" ]]; then
        read -r -p "${label} [${default}]: " _read || true
        printf '%s' "${_read:-$default}"
    else
        read -r -p "${label}: " _read
        printf '%s' "$_read"
    fi
}

confirm_run() {
    if [[ "${ASSUME_YES:-}" == 1 ]]; then
        return 0
    fi
    read -r -p "Execute this command? [y/N] " ans
    [[ "${ans:-}" == [yY] || "${ans:-}" == [yY][eE][sS] ]]
}

extra_gas() {
    if [[ -n "${GAS_COIN_ID:-}" ]]; then
        printf '%s\n' '--gas' "$GAS_COIN_ID"
    fi
}

extra_dry() {
    if [[ "${DRY_RUN:-0}" == 1 ]]; then
        printf '%s\n' '--dry-run'
    fi
}

resolve_myso() {
    if [[ -n "${MYSO:-}" ]]; then
        echo "$MYSO"
        return
    fi
    if command -v myso &>/dev/null; then
        command -v myso
        return
    fi
    for cand in "$REPO_ROOT/target/debug/myso" "$REPO_ROOT/target/release/myso"; do
        if [[ -x "$cand" ]]; then
            echo "$cand"
            return
        fi
    done
    echo ""
}

resolve_mydata_cli() {
    local root="${MYDATA_REPO:-}"
    if [[ -z "$root" ]]; then
        if [[ -n "${MYSO_MYDATA_REPO:-}" ]]; then
            root="$MYSO_MYDATA_REPO"
        elif [[ -d "$REPO_ROOT/../myso-mydata" && -f "$REPO_ROOT/../myso-mydata/Cargo.toml" ]]; then
            root="$(cd "$REPO_ROOT/../myso-mydata" && pwd)"
        fi
    fi
    if [[ -z "${root:-}" ]]; then
        echo ""
        return
    fi
    for cand in "$root/target/release/mydata-cli" "$root/target/debug/mydata-cli"; do
        if [[ -x "$cand" ]]; then
            echo "$cand"
            return
        fi
    done
    echo ""
}

parse_env_file_value() {
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

probe_key_server() {
    local base="${1%/}"
    local code curl_ec=0
    code="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 5 "${base}/service" 2>/dev/null)" || curl_ec=$?
    if [[ "$curl_ec" -ne 0 || -z "$code" || "$code" == "000" ]]; then
        echo "Warning: could not reach ${base} (curl exit ${curl_ec}, http_code=${code:-?})." >&2
        read -r -p "Continue anyway? [y/N] " ans
        [[ "${ans:-}" == [yY]* ]] || return 1
        return 0
    fi
    # 4xx (e.g. 400) means the process responded; bare GET /service may not match the API contract.
    if [[ "$code" -ge 500 ]]; then
        echo "Warning: ${base}/service returned HTTP ${code}." >&2
        read -r -p "Continue anyway? [y/N] " ans
        [[ "${ans:-}" == [yY]* ]] || return 1
    fi
    return 0
}

# Outputs hex of serialized EncryptedObject via global ENCRYPT_OUT_HEX; id as ENCRYPT_ID_HEX (lowercase hex, no 0x)
run_encrypt_hmac_cli() {
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
        echo "mydata-cli encrypt-hmac failed:" >&2
        echo "$out" >&2
        return "$ec"
    fi

    local hex_line best cand len
    hex_line=""
    best=0
    while IFS= read -r cand; do
        cand="$(strip_0x "$cand")"
        [[ "$cand" == "$(strip_0x "$id_hex")" ]] && continue
        [[ "$cand" == "$pk_naked_hex" ]] && continue
        len="${#cand}"
        if [[ "$len" -gt "$best" && "$len" -ge 64 && $((len % 2)) -eq 0 ]]; then
            best="$len"
            hex_line="$cand"
        fi
    done < <(printf '%s' "$out" | grep -Eo '(0x)?[0-9a-fA-F]+' || true)
    if [[ ${#hex_line} -lt 64 ]]; then
        echo "Could not parse encrypt-hmac ciphertext hex (need >= 32 bytes). Raw output:" >&2
        echo "$out" >&2
        return 1
    fi
    ENCRYPT_OUT_HEX="$hex_line"
    ENCRYPT_ID_HEX="$(printf '%s' "$id_hex" | tr 'A-F' 'a-f')"
    return 0
}

show_context() {
    echo "=== Session context ==="
    echo "  Session save path:   $(session_state_save_path)"
    echo "  CLIENT_CONFIG:       ${CLIENT_CONFIG:-<unset>}"
    echo "  PKG_SOCIAL:          $PKG_SOCIAL"
    echo "  CLOCK_ID:            $CLOCK_ID"
    echo "  COIN_TYPE:           $COIN_TYPE"
    echo "  KEY_SERVER_URL:      $KEY_SERVER_URL"
    echo "  MYDATA_CONFIG_ID:    ${MYDATA_CONFIG_ID:-<unset>}"
    echo "  MYDATA_REGISTRY_ID:  ${MYDATA_REGISTRY_ID:-<unset>}"
    echo "  POOL_REGISTRY_ID:    ${POOL_REGISTRY_ID:-<unset>}"
    echo "  ANCHOR_REGISTRY_ID:  ${ANCHOR_REGISTRY_ID:-<unset>}"
    echo "  CLAIM_VAULT_ID:      ${CLAIM_VAULT_ID:-<unset>}"
    echo "  DIST_REGISTRY_ID:    ${DIST_REGISTRY_ID:-<unset>}"
    echo "  MYDATA_ADMIN_CAP_ID: ${MYDATA_ADMIN_CAP_ID:-<unset>}"
    echo "  POOL_ADMIN_CAP_ID:   ${POOL_ADMIN_CAP_ID:-<unset>}"
    echo "  GAS_COIN_ID:         ${GAS_COIN_ID:-<auto>}"
    echo "  LISTING_ID:          ${LISTING_ID:-<unset>}"
    echo "  PAY_COIN_ID:         ${PAY_COIN_ID:-<unset>}"
    echo "  MYDATA_SECRETS_FILE: ${MYDATA_SECRETS_FILE:-<unset>}"
    if [[ -n "${PUBLIC_KEY:-}" ]]; then
        echo "  PUBLIC_KEY:          <set, ${#PUBLIC_KEY} chars>"
    else
        echo "  PUBLIC_KEY:          <unset>"
    fi
    echo "  KEY_SERVER_OBJECT_ID: ${KEY_SERVER_OBJECT_ID:-<unset>}"
    echo "  MYSO:               $(resolve_myso || true)"
    echo "  MYDATA_CLI:         $(resolve_mydata_cli || true)"
    echo "======================="
}

set_context_interactive() {
    echo "Set session values (Enter keeps default in brackets)."
    echo "Values are written to $(session_state_save_path) when you finish (override with MYDATA_MARKETPLACE_SESSION)."
    CLIENT_CONFIG="$(prompt_with_default "Path to client.yaml" "${CLIENT_CONFIG:-$PWD/network.config/client.yaml}")"
    PKG_SOCIAL="$(prompt_with_default "Social package id (MySoSocial)" "$PKG_SOCIAL")"
    CLOCK_ID="$(prompt_with_default "Clock object id" "$CLOCK_ID")"
    COIN_TYPE="$(prompt_with_default "MYSO coin type tag" "$COIN_TYPE")"
    KEY_SERVER_URL="$(prompt_with_default "Key server base URL" "$KEY_SERVER_URL")"
    MYDATA_CONFIG_ID="$(prompt_with_default "MyDataConfig object id" "${MYDATA_CONFIG_ID:-}")"
    MYDATA_REGISTRY_ID="$(prompt_with_default "MyDataRegistry object id" "${MYDATA_REGISTRY_ID:-}")"
    POOL_REGISTRY_ID="$(prompt_with_default "MyDataPoolRegistry object id" "${POOL_REGISTRY_ID:-}")"
    ANCHOR_REGISTRY_ID="$(prompt_with_default "SnapshotAnchorRegistry object id" "${ANCHOR_REGISTRY_ID:-}")"
    CLAIM_VAULT_ID="$(prompt_with_default "MyDataClaimVault object id" "${CLAIM_VAULT_ID:-}")"
    DIST_REGISTRY_ID="$(prompt_with_default "DistributionRegistry object id" "${DIST_REGISTRY_ID:-}")"
    MYDATA_ADMIN_CAP_ID="$(prompt_with_default "MyDataAdminCap object id" "${MYDATA_ADMIN_CAP_ID:-}")"
    POOL_ADMIN_CAP_ID="$(prompt_with_default "MyDataPoolAdminCap object id" "${POOL_ADMIN_CAP_ID:-}")"
    GAS_COIN_ID="$(prompt_with_default "Gas coin object id (empty = auto)" "${GAS_COIN_ID:-}")"
    LISTING_ID="$(prompt_with_default "Default listing (MyData) object id" "${LISTING_ID:-}")"
    PAY_COIN_ID="$(prompt_with_default "Default payment Coin<MYSO> id" "${PAY_COIN_ID:-}")"
    MYDATA_SECRETS_FILE="$(prompt_with_default "local-mydata-secrets.env path (optional)" "${MYDATA_SECRETS_FILE:-}")"
    PUBLIC_KEY="$(prompt_with_default "PUBLIC_KEY for encrypt (optional; can use secrets file)" "${PUBLIC_KEY:-}")"
    KEY_SERVER_OBJECT_ID="$(prompt_with_default "KEY_SERVER_OBJECT_ID (optional)" "${KEY_SERVER_OBJECT_ID:-}")"
    show_context
    apply_session_defaults
    save_session_state
}

run_myso_call() {
    local func="$1"
    shift
    local myso
    myso="$(resolve_myso)"
    [[ -n "$myso" ]] || { echo "myso binary not found. Set MYSO or build the myso crate." >&2; return 1; }
    [[ -n "${CLIENT_CONFIG:-}" && -f "$CLIENT_CONFIG" ]] || { echo "Set a valid CLIENT_CONFIG (menu 0)." >&2; return 1; }

    local -a cmd
    cmd=("$myso" client --client.config "$CLIENT_CONFIG" call --package "$PKG_SOCIAL" --module mydata --function "$func")
    local g
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_gas)
    while IFS= read -r g; do [[ -n "$g" ]] && cmd+=("$g"); done < <(extra_dry)
    cmd+=("$@")

    echo "---"
    printf ' %q' "${cmd[@]}"
    echo
    echo "---"
    if [[ "${SKIP_CONFIRM_RUN:-}" == 1 ]]; then
        "${cmd[@]}"
    else
        confirm_run || return 0
        "${cmd[@]}"
    fi
}

menu_update_config() {
    [[ -n "${MYDATA_ADMIN_CAP_ID:-}" ]] || { echo "Set MYDATA_ADMIN_CAP_ID (menu 0)." >&2; return 1; }
    [[ -n "${MYDATA_CONFIG_ID:-}" ]] || { echo "Set MYDATA_CONFIG_ID (menu 0)." >&2; return 1; }
    local en max_tags max_sub max_grants
    en="$(prompt_with_default "enable_flag (true/false)" "true")"
    max_tags="$(prompt_with_default "max_tags" "10")"
    max_sub="$(prompt_with_default "max_subscription_days" "365")"
    max_grants="$(prompt_with_default "max_free_access_grants" "100000")"
    run_myso_call update_mydata_config \
        --args "$MYDATA_ADMIN_CAP_ID" "$MYDATA_CONFIG_ID" "$en" "$max_tags" "$max_sub" "$max_grants"
}

# Ensures create_and_share won't hit EDisabled (abort 11). Optional non-interactive: ASSUME_YES=1 skips prompt and always enables.
ensure_mydata_enabled_for_listing() {
    [[ -n "${MYDATA_CONFIG_ID:-}" ]] || return 0
    [[ -n "${MYDATA_ADMIN_CAP_ID:-}" ]] || {
        echo "" >&2
        echo "Note: MYDATA_ADMIN_CAP is not set. If create_and_share fails with abort 11 (EDisabled)," >&2
        echo "      run menu [1] update_mydata_config with enable_flag true (need admin cap in menu 0)." >&2
        return 0
    }
    echo "" >&2
    echo ">>> Listings require MyDataConfig.enable_flag=true (otherwise Move abort 11 / EDisabled)." >&2
    local run_en='y'
    if [[ "${ASSUME_YES:-}" != 1 ]]; then
        read -r -p "Run update_mydata_config with enable_flag=true now (limits 10 / 365d / 100k grants)? [Y/n] " run_en
    fi
    if [[ -z "${run_en:-}" || "${run_en}" == [yY]* ]]; then
        echo "(Submitting update_mydata_config — enable_flag true...)" >&2
        set +e
        SKIP_CONFIRM_RUN=1 run_myso_call update_mydata_config \
            --args "$MYDATA_ADMIN_CAP_ID" "$MYDATA_CONFIG_ID" true 10 365 100000
        local ec=$?
        set -e
        if [[ $ec -ne 0 ]]; then
            echo "update_mydata_config failed (exit $ec). Fix and run menu [1], or retry." >&2
            read -r -p "Continue to encrypt/listing anyway? [y/N] " cont
            [[ "${cont:-}" == [yY]* ]] || return 1
        fi
    else
        echo "Skipping on-chain enable — ensure you already ran menu [1] or enabled elsewhere." >&2
    fi
}

menu_create_and_share() {
    [[ -n "${MYDATA_CONFIG_ID:-}" && -n "${MYDATA_REGISTRY_ID:-}" ]] || {
        echo "Set MYDATA_CONFIG_ID and MYDATA_REGISTRY_ID (menu 0)." >&2
        return 1
    }

    ensure_mydata_enabled_for_listing

    local mydata_cli sec_file pk ks plaintext aad_opt
    mydata_cli="$(resolve_mydata_cli)"
    [[ -n "$mydata_cli" ]] || {
        echo "mydata-cli not found. Build myso-mydata (cargo build -p mydata-cli); set MYDATA_REPO or use sibling ../myso-mydata." >&2
        return 1
    }

    local default_sec="${MYDATA_SECRETS_FILE:-}"
    if [[ -z "$default_sec" ]]; then
        if [[ -f "$PWD/$DEFAULT_SECRETS_REL" ]]; then
            default_sec="$PWD/$DEFAULT_SECRETS_REL"
        elif [[ -f "$REPO_ROOT/$DEFAULT_SECRETS_REL" ]]; then
            default_sec="$REPO_ROOT/$DEFAULT_SECRETS_REL"
        else
            default_sec="$PWD/$DEFAULT_SECRETS_REL"
        fi
    fi
    sec_file="$(prompt_with_default "Path to local-mydata-secrets.env" "$default_sec")"
    pk="${PUBLIC_KEY:-}"
    ks="${KEY_SERVER_OBJECT_ID:-}"
    if [[ -f "$sec_file" ]]; then
        MYDATA_SECRETS_FILE="$sec_file"
        [[ -z "${pk:-}" ]] && pk="$(parse_env_file_value "$sec_file" PUBLIC_KEY || true)"
        [[ -z "${ks:-}" ]] && ks="$(parse_env_file_value "$sec_file" KEY_SERVER_OBJECT_ID || true)"
        local u
        u="$(parse_env_file_value "$sec_file" KEY_SERVER_URL || true)"
        [[ -n "$u" ]] && KEY_SERVER_URL="$u"
    fi
    if [[ -z "${pk:-}" ]]; then
        pk="$(prompt_with_default "PUBLIC_KEY (0x..., IBE G2 from genkey / key server)" "")"
    fi
    if [[ -z "${ks:-}" ]]; then
        ks="$(prompt_with_default "KEY_SERVER_OBJECT_ID (on-chain KeyServer)" "")"
    fi
    [[ -n "$pk" && -n "$ks" ]] || { echo "PUBLIC_KEY and KEY_SERVER_OBJECT_ID are required." >&2; return 1; }

    KEY_SERVER_URL="$(prompt_with_default "Key server URL (probe before encrypt)" "$KEY_SERVER_URL")"
    probe_key_server "$KEY_SERVER_URL" || return 1

    plaintext="$(prompt_with_default "Plaintext to encrypt" "Hello from MyData marketplace demo")"
    aad_opt="$(prompt_with_default "Optional encrypt-hmac --aad as hex (empty to skip)" "")"

    local msg_hex enc_id
    msg_hex="$(printf '%s' "$plaintext" | xxd -p -c 65536 | tr -d '\n')"
    enc_id="$(openssl rand -hex 32)"

    local pk_naked
    pk_naked="$(strip_0x "$pk")"
    [[ ${#pk_naked} -ge 64 ]] || { echo "PUBLIC_KEY hex too short after stripping 0x." >&2; return 1; }

    echo ""
    echo "Running mydata-cli encrypt-hmac (threshold=1, package-id=$PKG_SOCIAL, key server object=$ks)"
    ENCRYPT_OUT_HEX=''
    ENCRYPT_ID_HEX=''
    if [[ -n "$aad_opt" ]]; then
        run_encrypt_hmac_cli "$mydata_cli" "$msg_hex" "$PKG_SOCIAL" "$enc_id" 1 "$pk_naked" "$ks" "$(strip_0x "$aad_opt")"
    else
        run_encrypt_hmac_cli "$mydata_cli" "$msg_hex" "$PKG_SOCIAL" "$enc_id" 1 "$pk_naked" "$ks"
    fi

    local enc_arg id_arg
    enc_arg="\"0x${ENCRYPT_OUT_HEX}\""
    id_arg="\"0x${ENCRYPT_ID_HEX}\""

    local media tags_json tstart tend otp sp subdur_raw geo dq sample coll upd freq
    media="$(prompt_with_default "media_type" "demo:bf-hmac-encrypt-hmac")"
    tags_json="$(prompt_with_default 'tags (JSON array of strings)' '["cli-demo"]')"
    tstart="$(prompt_with_default "timestamp_start (u64)" "0")"
    tend="$(prompt_with_default 'timestamp_end Option — [] or ["123"]' '[]')"
    otp="$(prompt_with_default 'one_time_price Option — [] or ["1000000000"]' '["1000000000"]')"
    sp="$(prompt_with_default 'subscription_price Option — [] or ["500000000"]' '["500000000"]')"
    subdur_raw="$(prompt_with_default "subscription_duration_days" "30")"
    geo="$(prompt_with_default "geographic_region Option<String> — [] or [\"US-CA\"]" '[]')"
    dq="$(prompt_with_default "data_quality Option<String> — [] or [\"high\"] (not a number)" '[]')"
    sample="$(prompt_with_default "sample_size Option<u64> — [] or [1000]" '[]')"
    coll="$(prompt_with_default "collection_method Option<String> — [] or [\"cli\"]" '[]')"
    upd="$(prompt_with_default "is_updating (true/false)" "false")"
    freq="$(prompt_with_default "update_frequency Option" '[]')"

    run_myso_call create_and_share \
        --args "$MYDATA_CONFIG_ID" "$MYDATA_REGISTRY_ID" "\"$media\"" "$tags_json" [] "$tstart" "$tend" \
        "$enc_arg" "$id_arg" "$otp" "$sp" "$subdur_raw" "$geo" "$dq" "$sample" "$coll" "$upd" "$freq" "$CLOCK_ID"

    PUBLIC_KEY="$pk"
    KEY_SERVER_OBJECT_ID="$ks"
    apply_session_defaults
    save_session_state
}

menu_purchase_one_time() {
    local listing pay
    listing="$(prompt_with_default "MyData listing object id" "${LISTING_ID:-}")"
    pay="$(prompt_with_default "Payment Coin<MYSO> object id" "${PAY_COIN_ID:-}")"
    run_myso_call purchase_one_time --type-args "$COIN_TYPE" --args "$MYDATA_CONFIG_ID" "$listing" "$pay" "$CLOCK_ID"
}

menu_purchase_sub() {
    local listing pay
    listing="$(prompt_with_default "MyData listing object id" "${LISTING_ID:-}")"
    pay="$(prompt_with_default "Payment Coin<MYSO> object id" "${PAY_COIN_ID:-}")"
    run_myso_call purchase_subscription --type-args "$COIN_TYPE" --args "$MYDATA_CONFIG_ID" "$listing" "$pay" "$CLOCK_ID"
}

menu_update_pricing() {
    local listing
    listing="$(prompt_with_default "MyData listing id" "${LISTING_ID:-}")"
    local o sp dur
    o="$(prompt_with_default 'new_one_time_price Option' '["1500000000"]')"
    sp="$(prompt_with_default 'new_subscription_price Option' '["750000000"]')"
    dur="$(prompt_with_default 'new_subscription_duration_days Option' '["45"]')"
    run_myso_call update_pricing --args "$listing" "$o" "$sp" "$dur" "$CLOCK_ID"
}

menu_update_content() {
    local listing ed tags
    listing="$(prompt_with_default "MyData listing id" "${LISTING_ID:-}")"
    ed="$(prompt_with_default 'new_encrypted_data Option — [] or "0x..."' '[]')"
    tags="$(prompt_with_default 'new_tags Option' '[]')"
    run_myso_call update_content --args "$listing" "$ed" "$tags" "$CLOCK_ID"
}

menu_mydata_approve() {
    local listing idv
    listing="$(prompt_with_default "MyData listing id" "${LISTING_ID:-}")"
    idv="$(prompt_with_default "encryption_id (0x hex, matches listing)" "")"
    [[ -n "$idv" ]] || { echo "encryption id required." >&2; return 1; }
    idv="0x$(strip_0x "$idv")"
    run_myso_call mydata_approve --args "\"$idv\"" "$listing" "$CLOCK_ID"
}

menu_grant_access() {
    local listing user at sd
    listing="$(prompt_with_default "MyData listing id" "${LISTING_ID:-}")"
    user="$(prompt_with_default "beneficiary address" "")"
    at="$(prompt_with_default "access_type (0=one_time, 1=subscription)" "0")"
    sd="$(prompt_with_default "subscription_days Option" '[]')"
    run_myso_call grant_access --args "$MYDATA_CONFIG_ID" "$listing" "$user" "$at" "$sd" "$CLOCK_ID"
}

menu_register() {
    local listing
    listing="$(prompt_with_default "MyData listing id" "${LISTING_ID:-}")"
    run_myso_call register_in_registry --args "$MYDATA_REGISTRY_ID" "$listing" "$CLOCK_ID"
}

menu_unregister() {
    local ip
    ip="$(prompt_with_default "ip_id (listing address)" "")"
    run_myso_call unregister_from_registry --args "$MYDATA_REGISTRY_ID" "$ip" "$CLOCK_ID"
}

menu_create_broad_pool() {
    local n d
    n="$(prompt_with_default "pool name" "demo-pool")"
    d="$(prompt_with_default "description" "CLI demo")"
    run_myso_call create_broad_pool --args "$POOL_ADMIN_CAP_ID" "$POOL_REGISTRY_ID" "\"$n\"" "\"$d\"" "$CLOCK_ID"
}

menu_create_sub_pool() {
    local bid n d
    bid="$(prompt_with_default "broad_pool_id (ID value)" "")"
    n="$(prompt_with_default "sub pool name" "demo-sub")"
    d="$(prompt_with_default "description" "CLI demo sub")"
    run_myso_call create_sub_pool --args "$POOL_ADMIN_CAP_ID" "$POOL_REGISTRY_ID" "$bid" "\"$n\"" "\"$d\"" [] "$CLOCK_ID"
}

menu_record_anchor() {
    local b_sub pay mf pr pc
    b_sub="$(prompt_with_default "source_pool_id" "")"
    pay="$(prompt_with_default "source_sub_pool_id" "")"
    mf="$(prompt_with_default "manifest_hash (JSON string bytes)" "\"01020304\"")"
    pr="$(prompt_with_default "payment_reference (JSON string bytes)" "\"05060708\"")"
    pc="$(prompt_with_default "Coin<MYSO> object id" "")"
    run_myso_call record_snapshot_anchor --type-args "$COIN_TYPE" \
        --args "$ANCHOR_REGISTRY_ID" "$CLAIM_VAULT_ID" "$POOL_REGISTRY_ID" "$b_sub" "$pay" "$mf" "$pr" "$pc" "$CLOCK_ID"
}

menu_publish_merkle() {
    local sid root
    sid="$(prompt_with_default "snapshot_id" "")"
    root="$(prompt_with_default "root_hash (32 bytes hex, with or without 0x)" "")"
    root="0x$(strip_0x "$root")"
    run_myso_call publish_merkle_root --args "$POOL_ADMIN_CAP_ID" "$ANCHOR_REGISTRY_ID" "$CLAIM_VAULT_ID" "$sid" "\"${root}\"" "$CLOCK_ID"
}

menu_claim_hint() {
    echo "claim() needs Merkle proof (vector<vector<u8>>), leaf_index, amount — construct offline or via SDK."
}

main_menu() {
    while true; do
        echo ""
        echo "MyData marketplace (social_contracts::mydata)"
        echo " 0) Set / show session context (saved when done)"
        echo " 1) update_mydata_config"
        echo " 2) create_and_share (encrypt + list; auto-offers enable if admin cap set)"
        echo " 3) purchase_one_time"
        echo " 4) purchase_subscription"
        echo " 5) update_pricing"
        echo " 6) update_content"
        echo " 7) mydata_approve"
        echo " 8) grant_access"
        echo " 9) register_in_registry"
        echo "10) unregister_from_registry"
        echo "11) create_broad_pool"
        echo "12) create_sub_pool"
        echo "13) record_snapshot_anchor"
        echo "14) publish_merkle_root"
        echo "15) claim (help only)"
        echo " q) Quit"
        local c
        read -r -p "Choice: " c || break
        case "$c" in
            0) set_context_interactive ;;
            1) menu_update_config ;;
            2) menu_create_and_share ;;
            3) menu_purchase_one_time ;;
            4) menu_purchase_sub ;;
            5) menu_update_pricing ;;
            6) menu_update_content ;;
            7) menu_mydata_approve ;;
            8) menu_grant_access ;;
            9) menu_register ;;
            10) menu_unregister ;;
            11) menu_create_broad_pool ;;
            12) menu_create_sub_pool ;;
            13) menu_record_anchor ;;
            14) menu_publish_merkle ;;
            15) menu_claim_hint ;;
            q|Q) break ;;
            *) echo "Unknown choice." ;;
        esac
    done
}

for arg in "$@"; do
    case "$arg" in
        -h|--help)
            usage
            exit 0
            ;;
        -y)
            ASSUME_YES=1
            ;;
        --no-session)
            NO_SESSION_FILE=1
            ;;
    esac
done

load_session_state

main_menu
