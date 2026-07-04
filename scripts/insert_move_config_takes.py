#!/usr/bin/env python3
"""Insert missing memory_config / profile_config take_shared in tx blocks."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TEST_DIR = ROOT / "crates/myso-framework/packages/myso-social/tests"

CONFIGS = {
    "memory_config": "MemoryConfig",
    "profile_config": "ProfileConfig",
    "platform_config": "PlatformConfig",
}


def scenario_var_in_block(block: str) -> str | None:
    for var in ("&scenario", "&mut scenario", "&scen", "&mut scen", "sc"):
        if f"({var})" in block or f"({var}," in block:
            return var
    return None


def insert_config_takes(content: str) -> str:
    lines = content.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        out.append(line)
        # Start of tx block
        if "test_scenario::next_tx(" in line or re.match(r"^\s+\{\s*$", line):
            block_start = len(out) - 1
            depth = line.count("{") - line.count("}")
            j = i + 1
            block_lines = [line]
            while j < len(lines) and depth > 0:
                bl = lines[j]
                block_lines.append(bl)
                depth += bl.count("{") - bl.count("}")
                j += 1
            block = "".join(block_lines)
            scen = scenario_var_in_block(block)
            inserts: list[tuple[int, str]] = []
            if scen:
                for cfg_var, typ in CONFIGS.items():
                    if f"&{cfg_var}" in block and f"let {cfg_var} = test_scenario::take_shared<{typ}>" not in block:
                        # Find first take_shared line offset within block
                        for k, bl in enumerate(block_lines[1:], start=1):
                            if "test_scenario::take_shared<" in bl:
                                indent = re.match(r"^(\s*)", bl).group(1)  # type: ignore[union-attr]
                                insert_line = f"{indent}let {cfg_var} = test_scenario::take_shared<{typ}>({scen});\n"
                                inserts.append((k, insert_line))
                                break
                        else:
                            # No take_shared yet — insert after opening brace line
                            indent = re.match(r"^(\s*)", block_lines[1] if len(block_lines) > 1 else "            ").group(1)  # type: ignore[union-attr]
                            if not indent.strip():
                                indent = "            "
                            inserts.append((1, f"{indent}let {cfg_var} = test_scenario::take_shared<{typ}>({scen});\n"))

                # Add return_shared if missing
                for cfg_var, typ in CONFIGS.items():
                    if f"let {cfg_var} = test_scenario::take_shared<{typ}>" in block or any(
                        cfg_var in ins[1] for ins in inserts
                    ):
                        if f"return_shared({cfg_var})" not in block:
                            for k in range(len(block_lines) - 1, 0, -1):
                                if "test_scenario::return_shared(" in block_lines[k]:
                                    indent = re.match(r"^(\s*)", block_lines[k]).group(1)  # type: ignore[union-attr]
                                    inserts.append(
                                        (k + 1, f"{indent}test_scenario::return_shared({cfg_var});\n")
                                    )
                                    break

            if inserts:
                # Apply inserts to block_lines
                for pos, ins in sorted(inserts, key=lambda x: -x[0]):
                    block_lines.insert(pos, ins)
                # Replace out segment
                out = out[:block_start] + block_lines
            i = j
            continue
        i += 1
    return "".join(out)


def fix_memory_org_helpers(content: str) -> str:
    """Add memory_config param to helper fns in memory_organization_tests."""
    if "memory_organization_tests" not in content and "register_root_agent" not in content:
        return content

    # register_root_agent without config param
    content = re.sub(
        r"fun register_root_agent\(\n(\s+)memory_account:",
        r"fun register_root_agent(\n\1memory_config: &MemoryConfig,\n\1memory_account:",
        content,
    )
    content = re.sub(
        r"fun register_child_agent\(\n(\s+)memory_account:",
        r"fun register_child_agent(\n\1memory_config: &MemoryConfig,\n\1memory_account:",
        content,
    )
    content = re.sub(
        r"fun register_peer_agent\(\n(\s+)memory_account:",
        r"fun register_peer_agent(\n\1memory_config: &MemoryConfig,\n\1memory_account:",
        content,
    )
    # Fix calls to register_root_agent without config - add &memory_config as first arg after (
    content = re.sub(
        r"register_root_agent\(\n(\s+)&?mut memory_account",
        r"register_root_agent(\n\1&memory_config,\n\1&mut memory_account",
        content,
    )
    content = re.sub(
        r"register_child_agent\(\n(\s+)&?mut memory_account",
        r"register_child_agent(\n\1&memory_config,\n\1&mut memory_account",
        content,
    )
    content = re.sub(
        r"register_peer_agent\(\n(\s+)&?mut memory_account",
        r"register_peer_agent(\n\1&memory_config,\n\1&mut memory_account",
        content,
    )
    return content


def main() -> None:
    changed = []
    for path in sorted(TEST_DIR.glob("*.move")):
        orig = path.read_text()
        content = insert_config_takes(orig)
        if "memory_organization_tests" in path.name:
            content = fix_memory_org_helpers(content)
        if content != orig:
            path.write_text(content)
            changed.append(path.name)
    print(f"Updated {len(changed)} files: {changed}")


if __name__ == "__main__":
    main()
