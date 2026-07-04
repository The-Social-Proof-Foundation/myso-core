#!/usr/bin/env python3
"""Pass 3: fix scen refs, corrupted SPT args, missing profile_config takes, import typos."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TEST_DIRS = [
    ROOT / "crates/myso-framework/packages/myso-social/tests",
    ROOT / "crates/myso-framework/packages/messaging/tests",
]

SCENARIO_FUNCS = (
    "take_shared",
    "take_shared_by_id",
    "take_from_sender",
    "take_from_address",
    "return_shared",
    "return_to_sender",
    "return_to_address",
    "ctx",
)

# Functions that take owned Scenario (do NOT prefix scen with &)
OWNED_SCENARIO_FUNCS = {"end", "begin"}


def fix_scen_refs(content: str) -> str:
    """Fix test_scenario::* calls that pass local `scen` without reference."""
    for func in SCENARIO_FUNCS:
        pattern = rf"(test_scenario::{func}(?:<[^>]+>)?)\((?!&)(scen)\b"
        if func == "ctx":
            repl = rf"\1(&mut scen"
        else:
            repl = rf"\1(&scen"
        content = re.sub(pattern, repl, content)
    content = re.sub(
        r"test_scenario::next_tx\((?!&mut )(scen)\b",
        r"test_scenario::next_tx(&mut scen",
        content,
    )
    return content


def fix_corrupted_spt_args(content: str) -> str:
    content = content.replace(
        "h00, // non_platform_platform_to_creator_bps",
        "5000, // non_platform_platform_to_creator_bps",
    )
    content = content.replace(
        "è00, // non_platform_platform_to_treasury_bps",
        "5000, // non_platform_platform_to_treasury_bps",
    )
    return content


def fix_import_typos(content: str) -> str:
    content = content.replace("UsernameAdminCap,,", "UsernameAdminCap,")
    content = re.sub(r",\n\s*,\n\s*MemoryConfig\}", ",\n        MemoryConfig}", content)
    content = re.sub(r",\n\s*,\n\s*ProfileConfig\}", ",\n        ProfileConfig}", content)
    content = re.sub(r",\n\s*,\n\s*PlatformConfig\}", ",\n        PlatformConfig}", content)
    return content


def remove_spurious_return_shared(content: str) -> str:
    """Remove return_shared for configs never taken in the same block."""
    lines = content.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        # Detect tx block start via next_tx or opening brace after fun
        out.append(line)
        i += 1
    return content


def add_profile_config_in_blocks(content: str) -> str:
    """Before create_profile using &profile_config, ensure take_shared exists in same `{` block."""

    if "profile::create_profile(" not in content or "&profile_config" not in content:
        return content

    lines = content.splitlines(keepends=True)
    result: list[str] = []
    brace_depth = 0
    block_start = 0
    in_block = False

    for idx, line in enumerate(lines):
        if "test_scenario::next_tx(" in line or (
            brace_depth == 0 and re.search(r"^\s+\{\s*$", line)
        ):
            in_block = True
            block_start = len(result)

        if in_block:
            brace_depth += line.count("{") - line.count("}")

        result.append(line)

        if in_block and brace_depth <= 0 and "}" in line:
            # Analyze block [block_start:len(result))
            block = "".join(result[block_start:])
            if (
                "profile::create_profile(" in block
                and "&profile_config" in block
                and "let profile_config = test_scenario::take_shared<ProfileConfig>" not in block
            ):
                # Find scenario var used in block
                scen_var = None
                for var in ("&scenario", "&scen", "sc", "&mut scenario", "&mut scen"):
                    if f"take_shared<" in block and f"({var})" in block:
                        scen_var = var
                        break
                if scen_var is None:
                    if "(&scenario)" in block:
                        scen_var = "&scenario"
                    elif "(&scen)" in block:
                        scen_var = "&scen"
                    elif "(sc)" in block:
                        scen_var = "sc"
                if scen_var:
                    insert = f"            let profile_config = test_scenario::take_shared<ProfileConfig>({scen_var});\n"
                    # Insert after first take_shared in block
                    for j in range(block_start, len(result)):
                        if "test_scenario::take_shared<" in result[j]:
                            result.insert(j + 1, insert)
                            # Add return before block end if missing
                            block_text = "".join(result[block_start:])
                            if "return_shared(profile_config)" not in block_text:
                                for k in range(len(result) - 1, block_start, -1):
                                    if "test_scenario::return_shared(" in result[k]:
                                        result.insert(
                                            k + 1,
                                            "            test_scenario::return_shared(profile_config);\n",
                                        )
                                        break
                            break
            in_block = False
            brace_depth = 0

    return "".join(result)


def remove_unmatched_config_returns(content: str) -> str:
    """Remove return_shared(profile_config/platform_config) when no take in same block."""
    lines = content.splitlines(keepends=True)
    result: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if re.match(
            r"\s+test_scenario::return_shared\((profile_config|platform_config|memory_config)\);\s*\n?",
            line,
        ):
            # Walk back to find block start
            j = len(result) - 1
            depth = 0
            block_lines: list[str] = []
            while j >= 0:
                block_lines.insert(0, result[j])
                depth += result[j].count("}") - result[j].count("{")
                if depth < 0:
                    break
                j -= 1
            block = "".join(block_lines) + line
            cfg = re.search(r"return_shared\((\w+)\)", line).group(1)  # type: ignore[union-attr]
            if f"take_shared<{cfg.replace('_config', '').title()}Config>" not in block and f"take_shared<{cfg[0].upper() + cfg[1:]}" not in block:
                # Check ProfileConfig pattern
                type_map = {
                    "profile_config": "ProfileConfig",
                    "platform_config": "PlatformConfig",
                    "memory_config": "MemoryConfig",
                }
                tname = type_map.get(cfg, "")
                if tname and f"take_shared<{tname}>" not in block:
                    i += 1
                    continue
        result.append(line)
        i += 1
    return "".join(result)


def process_file(path: Path) -> bool:
    original = path.read_text()
    content = original
    content = fix_import_typos(content)
    content = fix_corrupted_spt_args(content)
    content = fix_scen_refs(content)
    content = add_profile_config_in_blocks(content)
    content = remove_unmatched_config_returns(content)
    if content != original:
        path.write_text(content)
        return True
    return False


def main() -> None:
    changed = []
    for d in TEST_DIRS:
        if not d.exists():
            continue
        for path in sorted(d.glob("*.move")):
            if process_file(path):
                changed.append(path.relative_to(ROOT))
    print(f"Updated {len(changed)} files:")
    for p in changed:
        print(f"  {p}")


if __name__ == "__main__":
    main()
