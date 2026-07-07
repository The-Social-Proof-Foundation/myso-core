#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Human-readable E2E success summaries for runnable scripts.

# 1 MYSO = 1_000_000_000 MIST
format_mist_to_myslo() {
    local mist="${1:-0}"
    python3 - "$mist" <<'PY'
import sys
mist = int(sys.argv[1])
whole = mist // 1_000_000_000
frac = mist % 1_000_000_000
if frac == 0:
    print(f"{whole:,}")
else:
    print(f"{whole:,}.{frac:09d}".rstrip('0').rstrip('.'))
PY
}

format_mist_with_units() {
    local mist="${1:-0}"
    printf '%s MIST (%s MYSO)' "$mist" "$(format_mist_to_myslo "$mist")"
}

print_run_summary_header() {
    local title="$1"
    echo "" >&2
    echo "══════════════════════════════════════════════════════════════" >&2
    echo "  $title" >&2
    echo "══════════════════════════════════════════════════════════════" >&2
}

print_run_summary_line() {
    local label="$1"
    local value="$2"
    printf '  %-28s %s\n' "$label:" "$value" >&2
}

print_run_summary_footer() {
    echo "══════════════════════════════════════════════════════════════" >&2
    echo "" >&2
}
