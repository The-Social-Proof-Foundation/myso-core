#!/usr/bin/env python3
"""
DEPRECATED: OpenRouter catalog drift is handled by myso-ai-credit-oracle when
AI_CREDIT_CATALOG_SYNC_ENABLED=true and AI_CREDIT_OPENROUTER_API_KEY are set.
See docs/content/guides/developer/ai-credit-integration.mdx.

Legacy diff tool — compares OpenRouter model pricing against pricing_catalog.toml.

Usage:
  OPENROUTER_API_KEY=... python3 scripts/sync_ai_credit_catalog.py [--myso-usd 1.0]

Prints a human-readable diff. Does not modify the catalog file — operators review
and merge changes into crates/myso-ai-credit-oracle/config/pricing_catalog.toml.

Conversion: mist_per_1m = usd_per_1m_tokens * (MIST_PER_MYSO / myso_usd_price)
Default assumes 1 MYSO = $1 USD for catalog alignment.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import urllib.request
from pathlib import Path

MIST_PER_MYSO = 1_000_000_000
CATALOG_PATH = Path(__file__).resolve().parents[1] / "crates/myso-ai-credit-oracle/config/pricing_catalog.toml"


def fetch_openrouter_models() -> list[dict]:
    api_key = os.environ.get("OPENROUTER_API_KEY")
    if not api_key:
        print("OPENROUTER_API_KEY not set — skipping live fetch; parsing local catalog only.")
        return []
    req = urllib.request.Request(
        "https://openrouter.ai/api/v1/models",
        headers={"Authorization": f"Bearer {api_key}"},
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.load(resp)
    return data.get("data", [])


def parse_catalog_aliases(path: Path) -> dict[str, tuple[int, int]]:
    """Return alias -> (input_mist_per_1m, output_mist_per_1m) from local TOML."""
    text = path.read_text()
    models: dict[str, tuple[int, int]] = {}
    current_aliases: list[str] = []
    input_mist = 0
    output_mist = 0
    in_models = False
    for line in text.splitlines():
        if line.strip().startswith("[[models]]"):
            in_models = True
            current_aliases = []
            input_mist = 0
            output_mist = 0
            continue
        if in_models and line.strip().startswith("[[") and not line.strip().startswith("[[models]]"):
            in_models = False
        if not in_models:
            continue
        if line.strip().startswith("aliases"):
            m = re.search(r"\[(.*)\]", line)
            if m:
                current_aliases = [
                    a.strip().strip('"').strip("'")
                    for a in m.group(1).split(",")
                ]
        if "input_mist_per_1m" in line:
            input_mist = int(re.search(r"(\d+)", line).group(1))
        if "output_mist_per_1m" in line:
            output_mist = int(re.search(r"(\d+)", line).group(1))
            for alias in current_aliases:
                models[alias.lower()] = (input_mist, output_mist)
    return models


def usd_to_mist_per_1m(usd: float, myso_usd: float) -> int:
    if myso_usd <= 0:
        raise ValueError("myso_usd must be positive")
    return int(round(usd * (MIST_PER_MYSO / myso_usd)))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--myso-usd", type=float, default=1.0, help="USD price of 1 MYSO")
    parser.add_argument("--catalog", type=Path, default=CATALOG_PATH)
    args = parser.parse_args()

    local = parse_catalog_aliases(args.catalog)
    remote_models = fetch_openrouter_models()

    if not remote_models:
        print(f"Local catalog: {len(local)} model aliases at {args.catalog}")
        return 0

    diffs = 0
    for entry in remote_models:
        model_id = entry.get("id", "").lower()
        if model_id not in local:
            continue
        pricing = entry.get("pricing") or {}
        try:
            prompt_usd = float(pricing.get("prompt", 0))
            completion_usd = float(pricing.get("completion", 0))
        except (TypeError, ValueError):
            continue
        # OpenRouter prices are per token; convert to per 1M tokens.
        prompt_per_1m = prompt_usd * 1_000_000
        completion_per_1m = completion_usd * 1_000_000
        expected_in = usd_to_mist_per_1m(prompt_per_1m, args.myso_usd)
        expected_out = usd_to_mist_per_1m(completion_per_1m, args.myso_usd)
        local_in, local_out = local[model_id]
        if local_in != expected_in or local_out != expected_out:
            diffs += 1
            print(f"\n{model_id}:")
            print(f"  catalog in/out:  {local_in:,} / {local_out:,} MIST per 1M")
            print(f"  openrouter in/out: {expected_in:,} / {expected_out:,} MIST per 1M (myso_usd={args.myso_usd})")

    if diffs == 0:
        print("No diffs for models present in both OpenRouter and local catalog.")
    else:
        print(f"\n{diffs} model(s) differ — update catalog version + effective_date after merge.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
