#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Arrow-key + number picker for runnable start menus.
# Each item is "value|label". Always writes INTERACTIVE_SELECT_RESULT
# (do not printf -v into a caller `local` — that is a no-op on bash 3.2).
# Returns 0 on a selection, 1 on quit (q).
# Non-TTY stdin falls back to a numbered prompt (same as other runnables).

if [[ -n "${_INTERACTIVE_SELECT_SOURCED:-}" ]]; then
    return 0 2>/dev/null || exit 0
fi
_INTERACTIVE_SELECT_SOURCED=1

INTERACTIVE_SELECT_RESULT=

_interactive_select_restore() {
    stty echo icanon 2>/dev/null || true
    printf '\033[?25h' >&2
}

_interactive_select_draw() {
    local title="$1" idx="$2"
    shift 2
    local labels=("$@") i prefix
    local lines=$(( ${#labels[@]} + 4 ))
    if [[ "${_INTERACTIVE_SELECT_DRAWN_LINES:-0}" -gt 0 ]]; then
        printf '\033[%dA\033[1G\033[J' "$_INTERACTIVE_SELECT_DRAWN_LINES" >&2
    fi
    printf '%s\n\n' "$title" >&2
    for i in "${!labels[@]}"; do
        if [[ "$i" -eq "$idx" ]]; then
            prefix='  > '
        else
            prefix='    '
        fi
        printf '%s%s\n' "$prefix" "${labels[$i]}" >&2
    done
    printf '\n  ↑/↓ and Enter · number · q to quit\n' >&2
    _INTERACTIVE_SELECT_DRAWN_LINES=$lines
}

# interactive_select DEST_VAR TITLE ITEM [ITEM...]
# DEST_VAR is ignored for assignment; read INTERACTIVE_SELECT_RESULT after a 0 return.
# ITEM format: value|label
interactive_select() {
    local _dest_unused="$1" title="$2"
    shift 2
    local items=("$@") values=() labels=() item value label
    local idx=0 n choice key key2 key3 i

    [[ "$#" -ge 1 ]] || {
        echo "interactive_select requires at least one item" >&2
        return 1
    }

    for item in "${items[@]}"; do
        value="${item%%|*}"
        label="${item#*|}"
        if [[ "$label" == "$item" ]]; then
            label="$item"
        fi
        values+=("$value")
        labels+=("$label")
    done
    n="${#values[@]}"
    _INTERACTIVE_SELECT_DRAWN_LINES=0

    if [[ ! -t 0 || ! -t 2 ]]; then
        printf '%s\n' "$title" >&2
        for i in "${!labels[@]}"; do
            printf '  %s\n' "${labels[$i]}" >&2
        done
        read -r -p "Choice: " choice || choice='q'
        case "$choice" in
            [Qq]|'')
                INTERACTIVE_SELECT_RESULT=
                return 1
                ;;
            [1-9])
                i=$((choice - 1))
                if [[ "$i" -ge 0 && "$i" -lt "$n" ]]; then
                    INTERACTIVE_SELECT_RESULT="${values[$i]}"
                    return 0
                fi
                ;;
        esac
        echo "Invalid choice" >&2
        INTERACTIVE_SELECT_RESULT=
        return 1
    fi

    stty -echo -icanon min 1 time 0 2>/dev/null || true
    printf '\033[?25l' >&2
    _interactive_select_draw "$title" "$idx" "${labels[@]}"

    while true; do
        IFS= read -rsn1 key || key='q'
        if [[ "$key" == $'\x1b' ]]; then
            IFS= read -rsn1 key2 || key2=''
            if [[ "$key2" == '[' ]]; then
                IFS= read -rsn1 key3 || key3=''
                case "$key3" in
                    A)
                        idx=$(((idx - 1 + n) % n))
                        _interactive_select_draw "$title" "$idx" "${labels[@]}"
                        ;;
                    B)
                        idx=$(((idx + 1) % n))
                        _interactive_select_draw "$title" "$idx" "${labels[@]}"
                        ;;
                esac
            fi
            continue
        fi
        case "$key" in
            ''|$'\n'|$'\r')
                INTERACTIVE_SELECT_RESULT="${values[$idx]}"
                _interactive_select_restore
                printf '\n' >&2
                return 0
                ;;
            [Qq])
                INTERACTIVE_SELECT_RESULT=
                _interactive_select_restore
                printf '\n' >&2
                return 1
                ;;
            [1-9])
                i=$((key - 1))
                if [[ "$i" -ge 0 && "$i" -lt "$n" ]]; then
                    INTERACTIVE_SELECT_RESULT="${values[$i]}"
                    _interactive_select_restore
                    printf '\n' >&2
                    return 0
                fi
                ;;
            k)
                idx=$(((idx - 1 + n) % n))
                _interactive_select_draw "$title" "$idx" "${labels[@]}"
                ;;
            j)
                idx=$(((idx + 1) % n))
                _interactive_select_draw "$title" "$idx" "${labels[@]}"
                ;;
        esac
    done
}
