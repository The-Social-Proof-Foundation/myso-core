#!/usr/bin/env python3
"""Second-pass fixes for Move test config threading."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TEST_DIRS = [
    ROOT / "crates/myso-framework/packages/myso-social/tests",
    ROOT / "crates/myso-framework/packages/messaging/tests",
]


def fix_file(content: str) -> str:
    # platform::test_init needs clock when clock is in scope
    content = re.sub(
        r"social_contracts::platform::test_init\(test_scenario::ctx",
        r"social_contracts::platform::test_init(&clock, test_scenario::ctx",
        content,
    )
    content = re.sub(
        r"(?<![\w:])platform::test_init\(test_scenario::ctx",
        r"platform::test_init(&clock, test_scenario::ctx",
        content,
    )
    content = re.sub(
        r"platform::test_init\(ts::ctx",
        r"platform::test_init(&clock, ts::ctx",
        content,
    )

    # SPT reserve/withdraw: insert min_vault_deposit when missing after config
    for fn in (
        "reserve_towards_post",
        "reserve_towards_post_with_platform",
        "withdraw_reservation_for_post",
        "withdraw_reservation_with_platform_for_post",
    ):
        def spt_fix(m: re.Match, fn=fn) -> str:
            if re.match(r"\s*\d+,", m.group(3)):
                return m.group(0)
            return f"{m.group(1)}{m.group(2)}\n{m.group(1)}1,\n{m.group(1)}{m.group(3)}"

        content = re.sub(
            rf"(social_proof_tokens::{fn}\(\s*\n\s*&mut \w+,\s*\n\s*&config,\s*\n)(\s*)(&mut \w+,)",
            spt_fix,
            content,
        )

    # Remove spurious profile_config returns without take in file
    if "take_shared<ProfileConfig>" not in content:
        content = re.sub(r"\s*test_scenario::return_shared\(profile_config\);\n", "\n", content)
        content = re.sub(r"\s*ts::return_shared\(profile_config\);\n", "\n", content)

    # Remove spurious platform_config returns without take
    if "take_shared<PlatformConfig>" not in content:
        content = re.sub(r"\s*test_scenario::return_shared\(platform_config\);\n", "\n", content)
        content = re.sub(r"\s*ts::return_shared\(platform_config\);\n", "\n", content)

    # SPoT single-line update_spot_config missing betting/reasoning fields
    content = re.sub(
        r"spot::update_spot_config\((&admin_cap, &mut cfg, true, (\d+), 0, 0, 0, )(\d+), (\d+), ([^,]+), (\d+), (\d+), ([^,]+), (&clock)",
        r"spot::update_spot_config(\1\2, \3, 2, 10, 1, 1000, 10, \4, \5, \6, \7, \8",
        content,
    )

    # memory register_sub_agent when still missing config (account first line)
    content = re.sub(
        r"memory::register_sub_agent\(\s*\n(\s*)(&mut memory_account,)(?!\s*\n\s*&memory_config,)",
        r"memory::register_sub_agent(\n\1&memory_config,\n\1\2",
        content,
    )
    content = re.sub(
        r"memory::register_sub_agent_delegated\(\s*\n(\s*)(&mut memory_account,)(?!\s*\n\s*&memory_config,)",
        r"memory::register_sub_agent_delegated(\n\1&memory_config,\n\1\2",
        content,
    )

    return content


def main() -> None:
    for test_dir in TEST_DIRS:
        if not test_dir.exists():
            continue
        for path in sorted(test_dir.glob("*.move")):
            original = path.read_text()
            updated = fix_file(original)
            if updated != original:
                path.write_text(updated)
                print(f"fixed {path.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
