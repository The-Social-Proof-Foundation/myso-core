#!/usr/bin/env python3
"""Safely insert config take_shared per transaction block (no truncation)."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIRS = [
    ROOT / "crates/myso-framework/packages/myso-social/tests",
    ROOT / "crates/myso-framework/packages/messaging/tests",
]

CONFIGS = [
    ("profile_config", "ProfileConfig"),
    ("memory_config", "MemoryConfig"),
    ("platform_config", "PlatformConfig"),
    ("post_config", "PostConfig"),
]


def split_tx_blocks(content: str) -> list[tuple[str, str, str]]:
    """Return list of (prefix, block_body, suffix) for each next_tx block."""
    # Process entire file by finding next_tx { ... }; patterns
    results = []
    pattern = re.compile(
        r"(test_scenario::next_tx\([^)]+\);\s*\{)(.*?)(^\s*\};)",
        re.MULTILINE | re.DOTALL,
    )
    last = 0
    for m in pattern.finditer(content):
        results.append((content[last : m.start()], m.group(0), m))
        last = m.end()
    return content, results, last


def scenario_var(block: str) -> str | None:
    m = re.search(r"test_scenario::next_tx\(([^,)]+)", block)
    if m:
        return m.group(1).strip()
    for var in ("&scenario", "&mut scenario", "&scen", "&mut scen", "sc"):
        if f"({var})" in block:
            return var
    return None


def patch_block(block: str) -> str:
    scen = scenario_var(block)
    if not scen:
        return block

    lines = block.splitlines(keepends=True)
    modified = False

    for cfg_var, typ in CONFIGS:
        if f"&{cfg_var}" not in block:
            continue
        if f"let {cfg_var} = test_scenario::take_shared<{typ}>" in block:
            continue
        # Insert after opening `{` line
        for i, line in enumerate(lines):
            if line.strip() == "{":
                indent = re.match(r"^(\s*)", lines[i + 1] if i + 1 < len(lines) else "            ").group(1)  # type: ignore[union-attr]
                if not indent:
                    indent = "            "
                lines.insert(
                    i + 1,
                    f"{indent}let {cfg_var} = test_scenario::take_shared<{typ}>({scen});\n",
                )
                modified = True
                block = "".join(lines)
                break

        if modified and f"return_shared({cfg_var})" not in block:
            # Insert before closing `};`
            for j in range(len(lines) - 1, -1, -1):
                if lines[j].strip() == "};":
                    indent = re.match(r"^(\s*)", lines[j]).group(1)  # type: ignore[union-attr]
                    lines.insert(j, f"{indent}test_scenario::return_shared({cfg_var});\n")
                    block = "".join(lines)
                    break

    return "".join(lines) if modified else block


def process(content: str) -> str:
    pattern = re.compile(
        r"(test_scenario::next_tx\([^)]+\);\s*\{)(.*?)(^\s*\};)",
        re.MULTILINE | re.DOTALL,
    )

    def repl(m: re.Match[str]) -> str:
        full = m.group(0)
        return patch_block(full)

    return pattern.sub(repl, content)


def fix_memory_helper_params(content: str) -> str:
    """Thread MemoryConfig through helper fns that reference &memory_config."""
    if "&memory_config," not in content:
        return content

    # Skip if helpers already have memory_config param
    if re.search(r"fun \w+\([^)]*memory_config: &MemoryConfig", content):
        return content

    # Add param to fun definitions that body-use &memory_config
    fun_pattern = re.compile(
        r"(fun (\w+)\(([^)]*)\)\s*\{)(.*?)(^\s*\})",
        re.MULTILINE | re.DOTALL,
    )

    def fix_fun(m: re.Match[str]) -> str:
        header, name, params, body, close = m.group(1), m.group(2), m.group(3), m.group(4), m.group(5)
        if "&memory_config" not in body:
            return m.group(0)
        if "memory_config: &MemoryConfig" in params:
            return m.group(0)
        if params.strip():
            new_params = params.rstrip() + ",\n        memory_config: &MemoryConfig"
        else:
            new_params = "\n        memory_config: &MemoryConfig"
        new_body = body
        # Fix internal calls to this helper — add &memory_config first arg
        for callee in ("register_root_agent", "register_child_agent", "register_peer_agent", "register_test_agent"):
            new_body = re.sub(
                rf"({callee}\(\n\s+)",
                rf"\1&memory_config,\n            ",
                new_body,
            )
        return f"fun {name}({new_params}\n    ) {{{new_body}{close}"

    return fun_pattern.sub(fix_fun, content)


def main() -> None:
    changed = []
    for d in DIRS:
        if not d.exists():
            continue
        for path in sorted(d.glob("*.move")):
            orig = path.read_text()
            new = process(orig)
            if "memory" in path.name or "mydata" in path.name or "ai_credit" in path.name:
                new = fix_memory_helper_params(new)
            if new != orig:
                path.write_text(new)
                changed.append(path.name)
    print(f"Patched {len(changed)} files: {changed}")


if __name__ == "__main__":
    main()
