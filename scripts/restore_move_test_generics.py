#!/usr/bin/env python3
"""Restore generic type args stripped from test_scenario calls."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIRS = [
    ROOT / "crates/myso-framework/packages/myso-social/tests",
    ROOT / "crates/myso-framework/packages/messaging/tests",
]

# (variable_pattern, type) — first match wins
INFER_RULES: list[tuple[str, str]] = [
    (r"let\s+(?:mut\s+)?clock\b", "Clock"),
    (r"let\s+platform_config\b", "PlatformConfig"),
    (r"let\s+profile_config\b", "ProfileConfig"),
    (r"let\s+memory_config\b", "memory::MemoryConfig"),
    (r"let\s+mut\s+preg\b", "PlatformRegistry"),
    (r"let\s+mut\s+registry\b", "UsernameRegistry"),
    (r"let\s+mut\s+memory_registry\b", "memory::MemoryRegistry"),
    (r"let\s+mut\s+ai_credit_config\b", "AiCreditConfig"),
    (r"let\s+mut\s+cfg\b", "spot::SpotConfig"),
    (r"let\s+cfg\b", "spot::SpotConfig"),
    (r"let\s+spot_cfg\b", "spot::SpotConfig"),
    (r"let\s+mut\s+config\b", "insurance::InsuranceConfig"),
    (r"let\s+config\b", "insurance::InsuranceConfig"),
    (r"let\s+mut\s+vault\b", "insurance::UnderwriterVault"),
    (r"let\s+mut\s+rec\b", "spot::SpotRecord"),
    (r"let\s+post_ref\b", "Post"),
    (r"let\s+mut\s+p\b", "Post"),
    (r"let\s+mut\s+beneficiary\b", "PoCUsernameBeneficiary"),
    (r"let\s+mut\s+directory\b", "PoCUsernameBeneficiaryDirectory"),
    (r"let\s+directory\b", "PoCUsernameBeneficiaryDirectory"),
    (r"let\s+treasury\b", "EcosystemTreasury"),
    (r"let\s+mut\s+vault_dir\b", "PoCVaultDirectory"),
    (r"let\s+mut\s+shard\b", "PoCUsernameBeneficiaryShard"),
    (r"let\s+mut\s+spot_config\b", "spot::SpotConfig"),
    (r"let\s+spot_config\b", "spot::SpotConfig"),
    (r"let\s+mut\s+spot_cfg\b", "spot::SpotConfig"),
    (r"let\s+mut\s+governance_registry\b", "governance::GovernanceRegistry"),
    (r"let\s+mut\s+spt_config\b", "spt::SocialProofTokensConfig"),
    (r"let\s+mut\s+spt_cfg\b", "spt::SocialProofTokensConfig"),
    (r"let\s+admin_cap\b", "spot::SpotAdminCap"),
    (r"let\s+cap\b", "PoCBeneficiaryAdminCap"),
    (r"let\s+mut\s+poc_config\b", "poc::PoCConfig"),
    (r"let\s+poc_config\b", "poc::PoCConfig"),
    (r"let\s+mut\s+insurance_config\b", "insurance::InsuranceConfig"),
    (r"let\s+insurance_config\b", "insurance::InsuranceConfig"),
    (r"let\s+mut\s+router_config\b", "insurance::InsuranceRouterConfig"),
    (r"let\s+mut\s+platform\b", "Platform"),
    (r"let\s+platform\b", "Platform"),
    (r"let\s+mut\s+platform_registry\b", "PlatformRegistry"),
    (r"let\s+mut\s+post\b", "Post"),
    (r"let\s+post\b", "Post"),
    (r"let\s+mut\s+profile\b", "Profile"),
    (r"let\s+profile\b", "Profile"),
    (r"let\s+mut\s+exchange_config\b", "spt::SocialProofTokensConfig"),
    (r"let\s+mut\s+tokens_config\b", "spt::SocialProofTokensConfig"),
    (r"let\s+mut\s+config\b", "spt::SocialProofTokensConfig"),
    (r"let\s+mut\s+spot_admin_cap\b", "spot::SpotAdminCap"),
    (r"let\s+mut\s+gov_cap\b", "governance::GovernanceAdminCap"),
    (r"let\s+gov_cap\b", "governance::GovernanceAdminCap"),
    (r"let\s+mut\s+gov_registry\b", "governance::GovernanceRegistry"),
    (r"let\s+gov_registry\b", "governance::GovernanceRegistry"),
    (r"let\s+mut\s+oracle\b", "spot::SpotOracle"),
    (r"let\s+oracle\b", "spot::SpotOracle"),
    (r"let\s+mut\s+record\b", "spot::SpotRecord"),
    (r"let\s+record\b", "spot::SpotRecord"),
    (r"let\s+mut\s+market\b", "spot::SpotMarket"),
    (r"let\s+market\b", "spot::SpotMarket"),
    (r"let\s+mut\s+bet\b", "spot::SpotBet"),
    (r"let\s+mut\s+insurance_vault\b", "insurance::UnderwriterVault"),
    (r"let\s+mut\s+underwriter_vault\b", "insurance::UnderwriterVault"),
    (r"let\s+mut\s+policy\b", "insurance::InsurancePolicy"),
    (r"let\s+policy\b", "insurance::InsurancePolicy"),
    (r"let\s+mut\s+claim\b", "insurance::InsuranceClaim"),
    (r"let\s+mut\s+router\b", "insurance::InsuranceRouter"),
    (r"let\s+router\b", "insurance::InsuranceRouter"),
    (r"let\s+mut\s+mem_account\b", "memory::MemoryAccount"),
    (r"let\s+mem_account\b", "memory::MemoryAccount"),
    (r"let\s+mut\s+mem_registry\b", "memory::MemoryRegistry"),
    (r"let\s+mut\s+spot_rec\b", "spot::SpotRecord"),
    (r"let\s+mut\s+proposal\b", "governance::Proposal"),
    (r"let\s+router_cfg\b", "insurance::InsuranceRouterConfig"),
    (r"let\s+mut\s+backstop\b", "insurance::InsuranceBackstopPool"),
    (r"let\s+vault\b", "insurance::UnderwriterVault"),
    (r"let\s+rec\b", "spot::SpotRecord"),
    (r"let\s+mut\s+vx\b", "insurance::UnderwriterVault"),
    (r"let\s+mut\s+v0\b", "insurance::UnderwriterVault"),
    (r"let\s+mut\s+v1\b", "insurance::UnderwriterVault"),
    (r"let\s+mut\s+v2\b", "insurance::UnderwriterVault"),
    (r"let\s+mut\s+v3\b", "insurance::UnderwriterVault"),
    (r"let\s+mydata_config\b", "mydata::MyDataConfig"),
    (r"let\s+mut\s+post_config\b", "post::PostConfig"),
    (r"let\s+post_config\b", "post::PostConfig"),
    (r"let\s+mut\s+messaging_config\b", "messaging::MessagingConfig"),
    (r"let\s+messaging_config\b", "messaging::MessagingConfig"),
]


def infer_type(line: str, next_lines: str) -> str | None:
    for pat, typ in INFER_RULES:
        if re.search(pat, line):
            return typ
    # Context from usage on next lines
    ctx = line + next_lines
    if "SpotConfig" in ctx or "spot::" in ctx:
        if "take_shared(&scen)" in line or "take_shared(&scenario)" in line:
            if "update_spot_config" in next_lines:
                return "spot::SpotConfig"
    if "InsuranceConfig" in next_lines:
        return "insurance::InsuranceConfig"
    if "PlatformRegistry" in next_lines:
        return "PlatformRegistry"
    if "SocialProofTokensConfig" in next_lines:
        return "spt::SocialProofTokensConfig"
    if "PoCConfig" in next_lines:
        return "poc::PoCConfig"
    if "GovernanceRegistry" in next_lines:
        return "governance::GovernanceRegistry"
    return None


def restore_generics(content: str) -> str:
    lines = content.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if re.search(
            r"test_scenario::(take_shared|take_from_sender|take_shared_by_id)\(&(?:scen|scenario)\)",
            line,
        ):
            next_ctx = "".join(lines[i + 1 : i + 6])
            typ = infer_type(line, next_ctx)
            if typ:
                line = re.sub(
                    r"test_scenario::(take_shared|take_from_sender|take_shared_by_id)\(&",
                    rf"test_scenario::\1<{typ}>(&",
                    line,
                    count=1,
                )
        out.append(line)
        i += 1
    return "".join(out)


def main() -> None:
    changed = []
    for d in DIRS:
        if not d.exists():
            continue
        for path in sorted(d.glob("*.move")):
            orig = path.read_text()
            new = restore_generics(orig)
            if new != orig:
                path.write_text(new)
                changed.append(path.name)
    print(f"Restored generics in {len(changed)} files: {changed}")


if __name__ == "__main__":
    main()
