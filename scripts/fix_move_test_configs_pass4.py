#!/usr/bin/env python3
"""Pass 4: surgical fixes for dynamic config test compile errors."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TEST_DIRS = [
    ROOT / "crates/myso-framework/packages/myso-social/tests",
    ROOT / "crates/myso-framework/packages/messaging/tests",
]


def fix_double_commas(content: str) -> str:
    while ",," in content:
        content = content.replace(",,", ",")
    return content


def fix_duplicate_platform_config(content: str) -> str:
    """Remove double-inserted &platform_config in assign_badge calls."""
    return re.sub(
        r"(&platform_config,\s*\n\s*)&platform_config,\s*\n(\s*&platform,)",
        r"\1\2",
        content,
    )


def fix_platform_test_init_without_clock(content: str) -> str:
    """Ensure clock exists before platform::test_init(&clock, ...)."""
    pattern = (
        r"(test_scenario::next_tx\((scenario|&mut scen)\);\s*\n\s*\{\s*\n)"
        r"(\s*)platform::test_init\(&clock,"
    )

    def repl(m: re.Match) -> str:
        prefix = m.group(1)
        indent = m.group(3)
        var = m.group(2)
        if var == "scenario":
            ctx_arg = "scenario"
        else:
            ctx_arg = "&mut scen"
        return (
            f"{prefix}"
            f"{indent}let clock = clock::create_for_testing(test_scenario::ctx({ctx_arg}));\n"
            f"{indent}platform::test_init(&clock,"
        )

    return re.sub(pattern, repl, content)


def fix_vest_myso_linear_signature(content: str) -> str:
    """Add profile_config param to vest_myso_linear helper."""
    old = """    fun vest_myso_linear(
        coin: Coin<MYSO>,
        recipient: address,"""
    new = """    fun vest_myso_linear(
        profile_config: &ProfileConfig,
        coin: Coin<MYSO>,
        recipient: address,"""
    if old in content and new not in content:
        content = content.replace(old, new)
    # Remove duplicate &profile_config inside body if present
    content = re.sub(
        r"(profile::vest_myso\(\s*\n\s*)&profile_config,\s*\n(\s*&profile_config,)",
        r"\1\2",
        content,
    )
    return content


def fix_memory_helper_take_internally(content: str, fn_name: str) -> str:
    """For helpers with memory_config param but callers omit it: take internally."""
    # Match function with trailing memory_config param
    pat = rf"(    fun {fn_name}\([^)]*)\n        memory_config: &MemoryConfig\n    \) \{{"
    m = re.search(pat, content, re.DOTALL)
    if not m:
        return content

    body_start = m.end()
    # Find matching closing brace for function
    depth = 1
    i = body_start
    while i < len(content) and depth > 0:
        if content[i] == "{":
            depth += 1
        elif content[i] == "}":
            depth -= 1
        i += 1
    body = content[body_start : i - 1]

    if "let memory_config = test_scenario::take_shared<MemoryConfig>" in body:
        # Already takes internally — just remove param
        new_sig = m.group(1) + "\n    ) {"
        content = content[: m.start()] + new_sig + content[body_start - 1 :]
        return content

    # Insert take at start of body, return before end
    indent = "        "
    take_line = f"{indent}let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);\n"
    return_line = f"{indent}test_scenario::return_shared(memory_config);\n"

    new_body = take_line + body
    if "return_shared(memory_config)" not in body:
        # Insert return before last closing operations
        new_body = new_body.rstrip() + "\n" + return_line

    new_sig = m.group(1) + "\n    ) {"
    content = content[: m.start()] + new_sig + new_body + content[i - 1 :]
    return content


def fix_create_org_in_tx_helper(content: str) -> str:
    """memory_test_helpers: remove erroneous memory_config param, keep internal take."""
    content = re.sub(
        r"(public fun create_org_in_tx\(\s*\n\s*sc: &mut test_scenario::Scenario,\s*\n\s*org_type: u8),?\s*\n\s*memory_config: &MemoryConfig\s*\n",
        r"\1\n",
        content,
    )
    return content


def fix_create_org_for_limit_test(content: str) -> str:
    content = re.sub(
        r"(fun create_org_for_limit_test\(\s*\n\s*scenario: &mut test_scenario::Scenario),?\s*\n\s*memory_config: &MemoryConfig\s*\n",
        r"\1\n",
        content,
    )
    content = re.sub(
        r"(#\[test\]\s*\n\s*#\[expected_failure[^\]]*\]\s*\n\s*fun test_org_limit_exceeded\()\s*memory_config: &MemoryConfig\s*\n",
        r"\1",
        content,
    )
    return content


def fix_register_child_of_agent(content: str) -> str:
    content = re.sub(
        r"(fun register_child_of_agent\(scenario: &mut test_scenario::Scenario),?\s*\n\s*memory_config: &MemoryConfig\s*\n",
        r"\1\n",
        content,
    )
    return content


def fix_memory_register_helpers(content: str) -> str:
    """Add &MemoryConfig first arg to local register_root/child/peer helpers."""

    def patch_fn(text: str, fn: str, call: str) -> str:
        sig_pat = rf"(    fun {fn}\(\s*\n        memory_account: &mut MemoryAccount,)"
        if not re.search(sig_pat, text):
            return text
        if re.search(rf"fun {fn}\([^)]*config: &MemoryConfig", text):
            return text

        text_new = re.sub(
            sig_pat,
            rf"    fun {fn}(\n        config: &MemoryConfig,\n        memory_account: &mut MemoryAccount,",
            text,
            count=1,
        )
        if text_new == text:
            return text
        text = text_new

        call_pat = rf"(memory::{call}\(\s*\n\s*)memory_account,"
        text = re.sub(
            call_pat,
            rf"\1&config,\n            memory_account,",
            text,
            count=1,
        )
        return text

    for fn, call in (
        ("register_root_agent", "register_sub_agent"),
        ("register_child_agent", "register_sub_agent_delegated"),
        ("register_peer_agent", "register_sub_agent_delegated"),
    ):
        content = patch_fn(content, fn, call)

    # Update call sites for register_root_agent / register_child_agent / register_peer_agent
    for fn in ("register_root_agent", "register_child_agent", "register_peer_agent"):
        content = re.sub(
            rf"({fn}\(\s*\n\s*)&mut memory_account,",
            rf"\1&memory_config,\n                &mut memory_account,",
            content,
        )
        content = re.sub(
            rf"({fn}\(\s*)&mut memory_account,",
            rf"\1&memory_config, &mut memory_account,",
            content,
        )

    # register_root_from_created_org needs to take memory_config
    if "fun register_root_from_created_org(" in content:
        block_pat = r"(fun register_root_from_created_org\(\s*\n\s*scenario: &mut test_scenario::Scenario,\s*\n\s*\) \{\s*\n)(        let mut org)"
        if re.search(block_pat, content) and "take_shared<MemoryConfig>" not in re.search(
            block_pat, content
        ).group(0):  # type: ignore[union-attr]
            content = re.sub(
                block_pat,
                r"\1        let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);\n\2",
                content,
                count=1,
            )
            content = re.sub(
                r"(register_root_from_created_org[\s\S]*?test_scenario::return_shared\(clock\);)",
                r"\1\n        test_scenario::return_shared(memory_config);",
                content,
                count=1,
            )

    return content


def fix_helper_scenario_param_refs(content: str) -> str:
    """When `scenario/scen: &mut Scenario` is a fn param, use (var) not (&var) for &Scenario APIs."""

    read_only_ops = (
        "take_shared",
        "take_shared_by_id",
        "take_from_sender",
        "take_from_sender_by_id",
        "take_from_address",
        "return_to_sender",
        "return_to_address",
        "has_most_recent_shared",
    )

    parts = re.split(r"(?=\n    fun )", content)
    if len(parts) == 1:
        return content

    result = [parts[0]]
    for part in parts[1:]:
        header = part.split("{", 1)[0]
        var_match = re.search(
            r"(scenario|scen):\s*&mut\s+(?:test_scenario::)?Scenario",
            header,
        )
        if var_match:
            var = var_match.group(1)
            for op in read_only_ops:
                part = re.sub(
                    rf"test_scenario::{op}(<[^>]+>)?\(&{var}\)",
                    rf"test_scenario::{op}\1({var})",
                    part,
                )
                part = re.sub(
                    rf"test_scenario::{op}(<[^>]+>)?\(&{var},",
                    rf"test_scenario::{op}\1({var},",
                    part,
                )
            part = re.sub(
                rf"test_scenario::ctx\(&mut {var}\)",
                f"test_scenario::ctx({var})",
                part,
            )
            part = re.sub(
                rf"test_scenario::next_tx\(&mut {var},",
                f"test_scenario::next_tx({var},",
                part,
            )
        result.append(part)
    return "".join(result)


def fix_define_custom_org_role_calls(content: str) -> str:
    if "memory::define_custom_org_role(" not in content:
        return content

    def repl(m: re.Match) -> str:
        block = m.group(0)
        if "&memory_config," in block.split("\n", 3)[0:4]:
            return block
        indent = m.group(1)
        return f"memory::define_custom_org_role(\n{indent}&memory_config,\n{indent}{m.group(2)}"

    content = re.sub(
        r"memory::define_custom_org_role\(\s*\n(\s*)(&memory_account|memory_account)",
        repl,
        content,
    )

    # Ensure memory_config take in tx blocks that call define_custom_org_role
    lines = content.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    while i < len(lines):
        out.append(lines[i])
        if "memory::define_custom_org_role(" in lines[i] and i > 0:
            # scan back to block start for memory_config take
            j = len(out) - 1
            depth = 0
            block_lines: list[str] = []
            while j >= 0:
                block_lines.insert(0, out[j])
                depth += out[j].count("}") - out[j].count("{")
                if depth < 0 or "test_scenario::next_tx" in out[j]:
                    break
                j -= 1
            block = "".join(block_lines)
            if "take_shared<MemoryConfig>" not in block:
                # find scenario var from next_tx
                scen_match = re.search(
                    r"test_scenario::next_tx\(([^,)]+),", block
                )
                scen_var = scen_match.group(1).strip() if scen_match else "&scenario"
                insert = f"            let memory_config = test_scenario::take_shared<MemoryConfig>({scen_var});\n"
                # insert after opening brace
                for k in range(len(out) - 1, max(0, len(out) - 40), -1):
                    if re.match(r"\s+\{\s*$", out[k]):
                        out.insert(k + 1, insert)
                        break
        i += 1

    new_content = "".join(out)
    # add return_shared(memory_config) before block ends
    if "define_custom_org_role(\n" in new_content:
        new_content = re.sub(
            r"(test_scenario::return_shared\(clock\);)(\s*\n\s*\};)",
            r"test_scenario::return_shared(memory_config);\n            \1\2",
            new_content,
        )
    return new_content


def fix_vest_myso_linear_body(content: str) -> str:
    return content.replace(
        "        profile::vest_myso(\n            &profile_config,",
        "        profile::vest_myso(\n            profile_config,",
    )


def fix_is_descendant_single_line(content: str) -> str:
    """Fix single-line is_descendant_agent calls missing config."""
    if "memory::is_descendant_agent(&memory_account," not in content:
        return content

    # For test blocks using single-line calls, add memory_config take in the block
    lines = content.splitlines(keepends=True)
    result: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if (
            "memory::is_descendant_agent(&memory_account," in line
            and "&memory_config," not in line
        ):
            # Look back in result for memory_config take in current block
            block_start = len(result) - 1
            while block_start > 0 and "test_scenario::next_tx" not in result[block_start]:
                block_start -= 1
            block_text = "".join(result[block_start:])
            if "take_shared<MemoryConfig>" not in block_text:
                # Insert after opening brace or memory_account take
                for j in range(len(result) - 1, block_start, -1):
                    if "take_shared<MemoryAccount>" in result[j]:
                        result.insert(
                            j + 1,
                            "            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);\n",
                        )
                        break
            line = line.replace(
                "memory::is_descendant_agent(&memory_account,",
                "memory::is_descendant_agent(&memory_config, &memory_account,",
            )
        result.append(line)
        i += 1

    new_content = "".join(result)
    # Add return_shared(memory_config) before block end if we added takes
    if "is_descendant_agent(&memory_config," in new_content:
        new_content = re.sub(
            r"(test_scenario::return_shared\(memory_account\);)",
            r"test_scenario::return_shared(memory_config);\n            \1",
            new_content,
            count=1,
        )
    return new_content


def process_file(path: Path) -> bool:
    original = path.read_text()
    content = original
    content = fix_double_commas(content)
    content = fix_duplicate_platform_config(content)
    content = fix_platform_test_init_without_clock(content)

    content = fix_helper_scenario_param_refs(content)
    if "memory::define_custom_org_role(" in content:
        content = fix_define_custom_org_role_calls(content)

    if path.name == "profile_tests.move":
        content = fix_vest_myso_linear_signature(content)
        content = fix_vest_myso_linear_body(content)

    if path.name == "memory_test_helpers.move":
        content = fix_create_org_in_tx_helper(content)

    if path.name == "memory_organization_tests.move":
        content = fix_create_org_for_limit_test(content)
        content = fix_memory_register_helpers(content)

    if path.name == "memory_hierarchy_tests.move":
        content = fix_memory_register_helpers(content)

    if path.name == "ai_credit_approval_tests.move":
        content = fix_register_child_of_agent(content)
        content = fix_is_descendant_single_line(content)

    for fn in (
        "register_test_agent",
        "register_test_agent_with_spend",
        "register_mydata_agent",
        "register_placeholder_agent",
        "register_root_agent_for_org",
        "register_member_agent_via_root",
    ):
        if f"fun {fn}(" in content:
            content = fix_memory_helper_take_internally(content, fn)

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
