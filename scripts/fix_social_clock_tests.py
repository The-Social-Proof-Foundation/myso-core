#!/usr/bin/env python3
"""Fix myso-social Move tests for clock timestamp unification."""

from __future__ import annotations

import re
from pathlib import Path

TESTS_DIR = (
    Path(__file__).resolve().parents[1]
    / "crates/myso-framework/packages/myso-social/tests"
)

CLOCK_IMPORT = "    use myso::clock::{Self, Clock};\n"

INIT_SUFFIXES = (
    "profile::init_for_testing",
    "profile::test_init",
    "block_list::test_init",
    "mydata::test_init",
    "social_graph::init_for_testing",
    "proof_of_creativity::test_init",
)

NEEDS_CLOCK = (
    "profile::create_profile",
    "profile::update_profile",
    "profile::register_username",
    "profile::create_offer",
    "profile::accept_offer",
    "profile::accept_offer_with_memory",
    "profile::reject_or_revoke_offer",
    "profile::admin_set_profile_x_username",
    "profile::transfer_profile_with_memory",
    "post::test_create_post",
    "post::test_create_post_with_revenue_redirect",
    "post::test_create_post_with_escrow_redirect",
    "post::test_create_post_with_spot",
    "post::test_create_comment",
    "post::create_test_promoted_post",
    "post::delete_post",
    "post::delete_comment",
    "post::tip_post",
    "post::tip_repost",
    "post::tip_comment",
    "platform::assign_badge",
    "platform::revoke_badge",
    "platform::create_platform",
    "platform::update_platform",
    "platform::leave_platform",
    "social_proof_tokens::update_social_proof_tokens_config",
    "social_proof_tokens::enable_spt_for_post",
    "social_proof_tokens::create_reservation_pool_for_profile",
    "social_proof_tokens::reserve_towards_post",
    "social_proof_tokens::reserve_towards_profile",
    "social_proof_tokens::reserve_towards_post_with_platform",
    "social_proof_tokens::reserve_towards_profile_with_platform",
    "social_proof_tokens::withdraw_reservation_for_post",
    "social_proof_tokens::withdraw_reservation_for_profile",
    "social_proof_tokens::withdraw_reservation_with_platform_for_post",
    "social_proof_tokens::withdraw_reservation_with_platform_for_profile",
    "social_proof_tokens::create_social_proof_token",
    "social_proof_tokens::sync_token_pool_poc_from_post",
    "social_proof_tokens::can_create_auction",
    "insurance::init_config",
    "insurance::set_enable_flag",
    "spot::update_spot_config",
    "spot::withdraw_spot_bet",
    "proof_of_creativity::update_poc_config",
    "governance::bootstrap_init",
    "governance::create_platform_governance",
)


def ensure_clock_import(content: str) -> str:
    if "use myso::clock" in content:
        return content
    if not any(x in content for x in ("clock::", "<Clock>", "&Clock", "Clock,")):
        return content
    m = re.search(r"(use myso::test_scenario;\n)", content)
    if m:
        return content[: m.end()] + CLOCK_IMPORT + content[m.end() :]
    return content


def init_line_pattern(suffix: str) -> re.Pattern[str]:
    return re.compile(
        rf"^(?P<indent>[ \t]*)(?P<module>(?:[\w]+::)*)"
        rf"{re.escape(suffix)}\(test_scenario::ctx\((?P<ctx>[^)]+)\)\);",
        re.MULTILINE,
    )


def fix_init_calls(content: str) -> str:
    for suffix in INIT_SUFFIXES:
        pat = init_line_pattern(suffix)

        # Reorder init-before-clock
        content = re.sub(
            rf"^(?P<indent>[ \t]*)(?P<module>(?:[\w]+::)*)"
            rf"{re.escape(suffix)}\(test_scenario::ctx\((?P<ctx>[^)]+)\)\);\s*\n"
            rf"(?:[ \t]*//[^\n]*\n\s*)?"
            rf"[ \t]*let clock = clock::create_for_testing\(test_scenario::ctx\((?P=ctx)\)\);",
            lambda m, s=suffix: (
                f"{m.group('indent')}let clock = clock::create_for_testing(test_scenario::ctx({m.group('ctx')}));\n"
                f"{m.group('indent')}{m.group('module')}{s}(&clock, test_scenario::ctx({m.group('ctx')}));\n"
            ),
            content,
            flags=re.MULTILINE,
        )

        def add_init(m: re.Match[str], s=suffix) -> str:
            indent = m.group("indent")
            module = m.group("module")
            ctx = m.group("ctx")
            func = f"{module}{s}"
            return (
                f"{indent}let clock = clock::create_for_testing(test_scenario::ctx({ctx}));\n"
                f"{indent}{func}(&clock, test_scenario::ctx({ctx}));\n"
                f"{indent}clock::share_for_testing(clock);\n"
            )

        content = pat.sub(add_init, content)
    return content


def consolidate_setup_block_clocks(content: str) -> str:
    block_re = re.compile(r"(\{)([^{}]*(?:\{[^{}]*\}[^{}]*)*)(\})", re.DOTALL)

    def fix_block(m: re.Match[str]) -> str:
        body = m.group(2)
        creates = list(
            re.finditer(
                r"^[ \t]*let clock = clock::create_for_testing\(test_scenario::ctx\([^)]+\)\);",
                body,
                re.MULTILINE,
            )
        )
        if len(creates) <= 1:
            return m.group(0)
        new_body = body
        for match in reversed(creates[1:]):
            new_body = new_body[: match.start()] + new_body[match.end() + 1 :]
        shares = list(
            re.finditer(r"^[ \t]*clock::share_for_testing\(clock\);", new_body, re.MULTILINE)
        )
        if len(shares) > 1:
            for match in reversed(shares[:-1]):
                new_body = new_body[: match.start()] + new_body[match.end() + 1 :]
        return m.group(1) + new_body + m.group(3)

    return block_re.sub(fix_block, content)


def insert_clock_arg(content: str) -> str:
    for func in NEEDS_CLOCK:
        # ctx as final argument inside the same call
        pat = re.compile(
            rf"({re.escape(func)}\((?P<args>[\s\S]*?)),\s*(?://[^\n]*)?\n([ \t]*)test_scenario::ctx\(",
            re.MULTILINE,
        )

        def repl(m: re.Match[str], f=func) -> str:
            args = m.group("args")
            if "&clock" in args:
                return m.group(0)
            indent = m.group(3)
            return f"{f}({args},\n{indent}&clock,\n{indent}test_scenario::ctx("

        content = pat.sub(repl, content)
    return content


def scenario_ref(ctx_arg: str) -> str:
    ctx_arg = ctx_arg.strip()
    if ctx_arg.startswith("&mut "):
        return "&" + ctx_arg[len("&mut ") :]
    if ctx_arg.startswith("&"):
        return ctx_arg
    return f"&{ctx_arg}"


def ensure_clock_in_next_tx_blocks(content: str) -> str:
    needs = re.compile("|".join(re.escape(f) for f in NEEDS_CLOCK))
    block_re = re.compile(
        r"(test_scenario::next_tx\([^;]+;\s*\n\s*\{)(.*?)(^\s*\};)",
        re.MULTILINE | re.DOTALL,
    )

    def fix(m: re.Match[str]) -> str:
        header, body, close = m.group(1), m.group(2), m.group(3)
        if not needs.search(body):
            return m.group(0)
        if re.search(r"let (mut )?clock =", body):
            body = insert_clock_arg(body)
            if (
                "return_shared(clock)" not in body
                and needs.search(body)
                and not re.search(r"\n[ \t]*[a-zA-Z0-9_:]+[ \t]*$", body.rstrip().split("\n")[-1])
            ):
                indent = re.match(r"^(\s*)\S", body, re.MULTILINE)
                indent = (indent.group(1) if indent else "            ")
                body = body.rstrip() + f"\n{indent}test_scenario::return_shared(clock);\n"
            return header + body + close
        ctx_m = re.search(r"test_scenario::ctx\(([^)]+)\)", body)
        if not ctx_m:
            return m.group(0)
        ref = scenario_ref(ctx_m.group(1))
        indent = re.match(r"^(\s*)\S", body, re.MULTILINE)
        indent = (indent.group(1) if indent else "            ")
        new_body = (
            f"\n{indent}let clock = test_scenario::take_shared<Clock>({ref});\n"
            + insert_clock_arg(body.lstrip("\n"))
        )
        if "return_shared(clock)" not in new_body:
            new_body = new_body.rstrip() + f"\n{indent}test_scenario::return_shared(clock);\n"
        return header + new_body + close

    return block_re.sub(fix, content)


def fix_governance_rescind_timestamp(content: str) -> str:
    content = content.replace(
        "let current_time = tx_context::epoch_timestamp_ms(ctx);",
        "let current_time = clock::timestamp_ms(&clock);",
    )
    content = re.sub(
        r"(// Successfully rescind proposal\n\s*test_scenario::next_tx\(&mut scenario, USER1\);\n\s*\{\n)"
        r"(\s*)let ctx = test_scenario::ctx\(&mut scenario\);",
        r"\1\2let clock = test_scenario::take_shared<Clock>(&scenario);\n\2let ctx = test_scenario::ctx(&mut scenario);",
        content,
        count=1,
    )
    return content.replace(
        "assert!(event_refund == STAKE_AMOUNT, 13);\n        };",
        "assert!(event_refund == STAKE_AMOUNT, 13);\n            test_scenario::return_shared(clock);\n        };",
        1,
    )


def process(path: Path) -> bool:
    original = path.read_text()
    content = original
    content = fix_init_calls(content)
    content = consolidate_setup_block_clocks(content)
    content = insert_clock_arg(content)
    content = ensure_clock_in_next_tx_blocks(content)
    if path.name == "governance_tests.move":
        content = fix_governance_rescind_timestamp(content)
    content = re.sub(r",(\s*,)+", r",\1", content)
    content = ensure_clock_import(content)
    if content != original:
        path.write_text(content)
        return True
    return False


def main() -> None:
    for path in sorted(TESTS_DIR.glob("*.move")):
        if process(path):
            print(path.name)


if __name__ == "__main__":
    main()
