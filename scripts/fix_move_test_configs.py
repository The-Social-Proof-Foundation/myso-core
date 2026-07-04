#!/usr/bin/env python3
"""Fix Move test files for dynamic ecosystem config API changes."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TEST_DIRS = [
    ROOT / "crates/myso-framework/packages/myso-social/tests",
    ROOT / "crates/myso-framework/packages/messaging/tests",
]

# Default PoC min vault deposit (matches proof_of_creativity::DEFAULT_MIN_VAULT_DEPOSIT_AMOUNT)
MIN_VAULT_DEPOSIT = "1"


def ensure_import(content: str, module_path: str, type_name: str) -> str:
    """Add a type to an existing `use module::{...}` block or create one."""
    if type_name in content:
        return content

    pattern = rf"(use {re.escape(module_path)}::\{{[^}}]*)(\}};)"
    match = re.search(pattern, content, re.DOTALL)
    if match:
        inner = match.group(1)
        if type_name not in inner:
            inner = inner.rstrip() + f",\n        {type_name}"
            content = content[: match.start()] + inner + match.group(2) + content[match.end() :]
        return content

    # Insert after first use block in module
    insert = f"    use {module_path}::{type_name};\n"
    mod_match = re.search(r"module [^{]+\{", content)
    if mod_match:
        pos = mod_match.end()
        return content[:pos] + "\n" + insert + content[pos:]
    return content


def add_shared_takes(content: str) -> str:
    """Insert take_shared for configs when calls need them but take is missing."""

    def insert_after(pattern: str, insert_line: str, guard: str) -> str:
        if guard in content:
            return content
        return re.sub(
            pattern,
            lambda m: m.group(0) + insert_line,
            content,
            count=1,
        )

    if "profile::create_profile(" in content and "take_shared<ProfileConfig>" not in content:
        for scenario_var in ("&scenario", "&mut scenario", "&scen", "&mut scen"):
            for reg_name in ("memory_registry", "username_registry"):
                pat = rf"(let mut {reg_name} = test_scenario::take_shared<[^>]+>\({scenario_var[1:]}\);)\n"
                if re.search(pat, content):
                    content = re.sub(
                        pat,
                        rf"\1\n            let profile_config = test_scenario::take_shared<ProfileConfig>({scenario_var});\n",
                        content,
                        count=1,
                    )
                    break
            if "take_shared<ProfileConfig>" in content:
                break

    if (
        any(
            f"memory::{fn}(" in content
            for fn in (
                "register_sub_agent",
                "register_sub_agent_delegated",
                "approve_org_key_policy",
                "is_descendant_agent",
                "define_custom_org_role",
                "approve_key_policy",
                "test_create_agentic_organization",
                "update_sub_agent_label",
            )
        )
        and "take_shared<MemoryConfig>" not in content
    ):
        for scenario_var in ("&scenario", "&mut scenario", "&scen", "&mut scen"):
            pat = rf"(let mut memory_registry = test_scenario::take_shared<MemoryRegistry>\({scenario_var[1:]}\);)\n"
            if re.search(pat, content):
                content = re.sub(
                    pat,
                    rf"\1\n            let memory_config = test_scenario::take_shared<MemoryConfig>({scenario_var});\n",
                    content,
                    count=1,
                )
                break

    if "platform::create_platform(" in content and "take_shared<PlatformConfig>" not in content:
        for scenario_var in ("&scenario", "&mut scenario", "&scen", "&mut scen", "scenario"):
            pat = rf"(let mut \w+ = test_scenario::take_shared<PlatformRegistry>\({scenario_var}\);)\n"
            if re.search(pat, content):
                sv = scenario_var if scenario_var.startswith("&") else f"&{scenario_var}"
                content = re.sub(
                    pat,
                    rf"\1\n            let platform_config = test_scenario::take_shared<PlatformConfig>({sv});\n",
                    content,
                    count=1,
                )
                break

    return content


def add_shared_returns(content: str) -> str:
    """Return shared configs before block ends when taken but not returned."""
    pairs = [
        ("profile_config", "ProfileConfig"),
        ("memory_config", "MemoryConfig"),
        ("platform_config", "PlatformConfig"),
        ("post_config", "PostConfig"),
    ]
    for var, _ in pairs:
        take_pat = rf"let {var} = test_scenario::take_shared"
        return_pat = rf"return_shared\({var}\)"
        if re.search(take_pat, content) and not re.search(return_pat, content):
            # Insert before first return_shared(clock) or end of block
            content = re.sub(
                r"(test_scenario::return_shared\(clock\);)",
                f"test_scenario::return_shared({var});\n            \\1",
                content,
                count=1,
            )
            if not re.search(return_pat, content):
                content = re.sub(
                    r"(test_scenario::return_shared\(registry\);)",
                    f"test_scenario::return_shared({var});\n            \\1",
                    content,
                    count=1,
                )
    return content


def fix_calls(content: str) -> str:
    """Apply call-site argument threading fixes."""

    # profile::create_profile — config is 2nd arg after registry
    def fix_create_profile(match: re.Match) -> str:
        body = match.group(0)
        if "&profile_config," in body.split("\n", 3)[0:4]:
            return body
        indent = match.group(1)
        reg = match.group(2)
        mem = match.group(3)
        return f"profile::create_profile(\n{indent}{reg}\n{indent}&profile_config,\n{indent}{mem}"

    content = re.sub(
        r"profile::create_profile\(\s*\n(\s*)(&mut \w+,)\s*\n\s*(&mut \w+,)",
        fix_create_profile,
        content,
    )

    # memory entry functions — config is 1st arg
    def fix_memory_fn(fn: str, first_arg_pattern: str) -> None:
        nonlocal content

        def repl(m: re.Match) -> str:
            body = m.group(0)
            if "&memory_config," in body.split("\n", 2)[0:3]:
                return body
            indent = m.group(1)
            first = m.group(2)
            return f"memory::{fn}(\n{indent}&memory_config,\n{indent}{first}"

        content = re.sub(
            rf"memory::{fn}\(\s*\n(\s*)({first_arg_pattern})",
            repl,
            content,
        )

    for fn in (
        "register_sub_agent",
        "register_sub_agent_delegated",
        "define_custom_org_role",
        "approve_key_policy",
        "test_create_agentic_organization",
        "update_sub_agent_label",
        "update_agentic_organization_metadata",
    ):
        fix_memory_fn(fn, r"&mut \w+,")

    def fix_approve_org(m: re.Match) -> str:
        if "&memory_config," in m.group(0).split("\n", 2)[0:3]:
            return m.group(0)
        indent = m.group(1)
        return f"memory::approve_org_key_policy(\n{indent}&memory_config,\n{indent}{m.group(2)}"

    content = re.sub(
        r"memory::approve_org_key_policy\(\s*\n(\s*)(\w+,|&\w+,|id,|vector)",
        fix_approve_org,
        content,
    )

    def fix_is_descendant(m: re.Match) -> str:
        if "&memory_config," in m.group(0).split("\n", 2)[0:3]:
            return m.group(0)
        indent = m.group(1)
        return f"memory::is_descendant_agent(\n{indent}&memory_config,\n{indent}{m.group(2)}"

    content = re.sub(
        r"memory::is_descendant_agent\(\s*\n(\s*)(&?\w+,)",
        fix_is_descendant,
        content,
    )

    # platform — config after registry
    def fix_platform_fn(fn: str) -> None:
        nonlocal content

        def repl(m: re.Match) -> str:
            if "&platform_config," in m.group(0).split("\n", 3)[0:4]:
                return m.group(0)
            indent = m.group(1)
            return f"platform::{fn}(\n{indent}{m.group(2)}\n{indent}&platform_config,\n{indent}{m.group(3)}"

        content = re.sub(
            rf"platform::{fn}\(\s*\n(\s*)(&mut \w+|&\w+,)\s*\n\s*(&mut \w+|&\w+,|string::)",
            repl,
            content,
        )

    for fn in ("create_platform", "assign_badge", "update_platform"):
        fix_platform_fn(fn)

    # profile vesting — config first
    for fn in ("vest_myso", "claim_vested_tokens"):
        content = re.sub(
            rf"profile::{fn}\(\s*\n(\s*)(coin::|&mut coin|recipient|start_time)",
            rf"profile::{fn}(\n\1&profile_config,\n\1\2",
            content,
        )
    content = re.sub(
        r"profile::claimable\(\s*\n(\s*)(&wallet|wallet,)",
        r"profile::claimable(\n\1&profile_config,\n\1\2",
        content,
    )
    content = re.sub(
        r"profile::claimable\((&wallet|wallet,)",
        r"profile::claimable(&profile_config, \1",
        content,
    )

    # ai_credit child budget functions
    for fn in ("set_child_agent_budget", "disable_child_agent_budget", "approve_child_agent_spend"):
        content = re.sub(
            rf"ai_credit::{fn}\(\s*\n(\s*)(&config,|&mut config,|config,)",
            rf"ai_credit::{fn}(\n\1\2\n\1&memory_config,\n\1",
            content,
        )
        # Fix double memory_config if already partially patched
        content = re.sub(
            rf"(ai_credit::{fn}\(\s*\n\s*&config,\s*\n\s*&memory_config,\s*\n\s*)&memory_config,\s*\n",
            r"\1",
            content,
        )

    # mydata — memory_config after mydata config
    for fn in ("purchase_one_time", "purchase_subscription"):
        content = re.sub(
            rf"mydata::{fn}\(\s*\n(\s*)(&config,|&mut config,|config,)\s*\n\s*(&mut mydata,|mydata,|&mydata,)",
            rf"mydata::{fn}(\n\1\2\n\1&memory_config,\n\1\3",
            content,
        )

    content = re.sub(
        r"mydata::mydata_approve\(\s*\n(\s*)(b\"|id,|vector|&id)",
        r"mydata::mydata_approve(\n\1&memory_config,\n\1\2",
        content,
    )

    # create_platform missing platform_config (registry then string::)
    def fix_create_platform(m: re.Match) -> str:
        if "&platform_config," in m.group(0).split("\n", 3)[0:4]:
            return m.group(0)
        indent = m.group(1)
        reg = m.group(2)
        return f"platform::create_platform(\n{indent}{reg}\n{indent}&platform_config,\n{indent}string::"

    content = re.sub(
        r"platform::create_platform\(\s*\n(\s*)(&mut \w+,)\s*\n\s*string::",
        fix_create_platform,
        content,
    )

    # vest_myso / claim_vested_tokens — config first (single-line and multi-line)
    content = re.sub(
        r"profile::claim_vested_tokens\((&mut \w+,)",
        r"profile::claim_vested_tokens(&profile_config, \1",
        content,
    )
    content = re.sub(
        r"profile::vest_myso\(\s*\n(\s*)(coin,|&mut coin|coin::)",
        r"profile::vest_myso(\n\1&profile_config,\n\1\2",
        content,
    )
    content = re.sub(
        r"profile::claimable\((&?\w+, &clock|&?\w+, clock)",
        r"profile::claimable(&profile_config, \1",
        content,
    )

    content = re.sub(
        r"vest_myso_linear\(\s*\n(\s*)(coin::|coin,)",
        r"vest_myso_linear(\n\1&profile_config,\n\1\2",
        content,
    )

    # register_sub_agent_delegated — same as register_sub_agent
    fix_memory_fn("register_sub_agent_delegated", r"&mut \w+,")

    # social_proof_tokens — min_vault_deposit after config
    for fn in (
        "reserve_towards_post",
        "reserve_towards_post_with_platform",
        "withdraw_reservation_for_post",
        "withdraw_reservation_with_platform_for_post",
    ):
        def spt_repl(m: re.Match) -> str:
            indent = m.group(1)
            return (
                f"social_proof_tokens::{fn}(\n{indent}{m.group(2)}\n"
                f"{indent}{m.group(3)}\n{indent}{MIN_VAULT_DEPOSIT},\n"
                f"{indent}{m.group(4)}"
            )

        content = re.sub(
            rf"social_proof_tokens::{fn}\(\s*\n(\s*)(&mut \w+|&\w+,)\s*\n\s*(&config,|&mut config,|&\w+,)\s*\n\s*(&mut \w+|&\w+,)",
            spt_repl,
            content,
        )

    # proof_of_creativity claim_username_beneficiary — profile_config after poc config
    content = re.sub(
        r"proof_of_creativity::claim_username_beneficiary\(\s*\n(\s*)(&config,|&mut config,|config,)\s*\n\s*(&mut directory,|directory,)",
        r"proof_of_creativity::claim_username_beneficiary(\n\1\2\n\1&profile_config,\n\1\3",
        content,
    )

    # post functions needing PostConfig + MemoryConfig
    def fix_react_to_post(m: re.Match) -> str:
        if "&post_config," in m.group(0):
            return m.group(0)
        indent = m.group(1)
        return (
            f"post::react_to_post(\n{indent}{m.group(2)}\n{indent}{m.group(3)}\n"
            f"{indent}{m.group(4)}\n{indent}{m.group(5)}\n{indent}{m.group(6)}\n"
            f"{indent}&post_config,\n{indent}&memory_config,\n{indent}{m.group(7)}"
        )

    content = re.sub(
        r"post::react_to_post\(\s*\n(\s*)(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)",
        fix_react_to_post,
        content,
    )

    for fn in ("create_post", "create_comment"):
        def post_repl(m: re.Match, fn=fn) -> str:
            if "&post_config," in m.group(0):
                return m.group(0)
            indent = m.group(1)
            return (
                f"post::{fn}(\n{indent}{m.group(2)}\n{indent}{m.group(3)}\n"
                f"{indent}{m.group(4)}\n{indent}{m.group(5)}\n{indent}{m.group(6)}\n"
                f"{indent}&post_config,\n{indent}&memory_config,\n{indent}{m.group(7)}"
            )

        content = re.sub(
            rf"post::{fn}\(\s*\n(\s*)(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)\s*\n\s*(&?\w+,)",
            post_repl,
            content,
        )

    # social_proof_tokens config update — add non-platform split bps before clock
    def fix_spt_config_update(m: re.Match) -> str:
        block = m.group(0)
        if "non_platform_platform_to_creator_bps" in block or re.search(
            r",\s*5000,\s*\n\s*5000,\s*\n\s*&clock", block
        ):
            return block
        return re.sub(
            r"(\s*)(\d+),\s*\n(\s*)(&clock,)",
            r"\1\2,\n\15000, // non_platform_platform_to_creator_bps\n\35000, // non_platform_platform_to_treasury_bps\n\3\4",
            block,
            count=1,
        )

    content = re.sub(
        r"social_proof_tokens::update_social_proof_tokens_config\([\s\S]*?&clock,",
        fix_spt_config_update,
        content,
    )

    # SPoT config update — replace legacy fee_bps + split args
    content = re.sub(
        r"(\d+),\s*// fee_bps[^\n]*\n\s*(\d+),\s*// platform split\n\s*(\w+),\s*// oracle_address",
        r"25,   // platform_fee_bps\n                25,   // ecosystem_fee_bps\n                2,    // min_betting_options\n                10,   // max_betting_options\n                1,    // min_reasoning_length\n                1000, // max_reasoning_length\n                10,   // max_evidence_urls\n                \3, // oracle_address",
        content,
    )
    content = re.sub(
        r"(spot::update_spot_config\([^)]*?, )(\d+), (\d+), ([^,]+), (\d+), (\d+), ([^,]+), (&clock)",
        r"\g<1>\2, \3, 2, 10, 1, 1000, 10, \4, \5, \6, \7, \8",
        content,
    )

    return content


def fix_imports(content: str) -> str:
    if "profile::create_profile(" in content or "profile::vest_myso(" in content or "profile::claimable(" in content:
        content = ensure_import(content, "social_contracts::profile", "ProfileConfig")

    if re.search(r"memory::(register_sub_agent|approve_org_key_policy|is_descendant_agent|test_create_agentic_organization)", content):
        content = ensure_import(content, "social_contracts::memory", "MemoryConfig")

    if "platform::create_platform(" in content or "platform::assign_badge(" in content:
        content = ensure_import(content, "social_contracts::platform", "PlatformConfig")

    if re.search(r"post::(create_post|create_comment|react_to_post)", content):
        content = ensure_import(content, "social_contracts::post", "PostConfig")
        content = ensure_import(content, "social_contracts::memory", "MemoryConfig")

    return content


def process_file(path: Path) -> bool:
    original = path.read_text()
    content = original
    content = fix_imports(content)
    content = fix_calls(content)
    content = add_shared_takes(content)
    content = add_shared_returns(content)
    if content != original:
        path.write_text(content)
        return True
    return False


def main() -> None:
    changed = []
    for test_dir in TEST_DIRS:
        if not test_dir.exists():
            continue
        for path in sorted(test_dir.glob("*.move")):
            if process_file(path):
                changed.append(path.relative_to(ROOT))
    print(f"Updated {len(changed)} files:")
    for p in changed:
        print(f"  {p}")


if __name__ == "__main__":
    main()
