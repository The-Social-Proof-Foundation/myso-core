#!/usr/bin/env bash
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0
#
# Run myso-spot-oracle locally (postgres via docker compose + cargo run).
#
# Session: network.config/spot-oracle/spot-oracle-session.env
#
# Usage:
#   ./scripts/run-spot-oracle.sh
#   ./scripts/run-spot-oracle.sh --refresh-session

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

SOCIAL_SESSION_SAVE_PATH="$REPO_ROOT/network.config/spot-oracle/spot-oracle-session.env"
# shellcheck source=lib/spot-oracle-common.sh
source "${SCRIPT_DIR}/lib/spot-oracle-common.sh"

COMPOSE_FILE="$REPO_ROOT/crates/myso-spot-oracle/docker-compose.yml"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --help|-h)
            sed -n '2,12p' "$0" | sed 's/^# \?//'
            exit 0
            ;;
        --refresh-session)
            refresh_spot_oracle_session_from_graphql
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

log_step "Starting spot-oracle postgres"
docker compose -f "$COMPOSE_FILE" up -d spot-oracle-postgres

for i in $(seq 1 60); do
    if docker compose -f "$COMPOSE_FILE" exec -T spot-oracle-postgres pg_isready -U spot -d spot_oracle >/dev/null 2>&1; then
        break
    fi
    sleep 1
done

export_spot_oracle_env
log_step "Running myso-spot-oracle"
exec cargo run -p myso-spot-oracle
