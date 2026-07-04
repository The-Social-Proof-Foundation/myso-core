# Dynamic Ecosystem Configs E2E — Scratchpad

## Background and Motivation
Vertical-slice, production-grade rollout of all dynamic config fields and five new config
models from Move → social indexer → REST → GraphQL for the **greenfield first publish**
of `social_contracts` + `messaging`. Extend 9 existing on-chain configs (AiCredit, Post,
SPT, Spot, PoC, MyData, Insurance, InsuranceRouter, EcosystemTreasury) and add 5 new
config objects + admin caps (Messaging, Subscription, Profile, Memory, Platform). All
defaults preserve today's hardcoded behavior. No frontend in this plan.

Plan reference: `.cursor/plans/dynamic_config_e2e_8fffcc98.plan.md` (Phases 0–8).

## Key Challenges and Analysis
- **Greenfield first publish (no backfill):** `social_contracts` with these configs is not
  yet on a live network, so all new fields + new config objects ship at bootstrap with full
  defaults. No `migrate_*` entries are authored or run; `CURRENT_VERSION` in `upgrade.move`
  is NOT bumped; `UpgradeAdminCap` is minted by `bootstrap.move` for *future* upgrades only.
- **SpotConfig fee redo:** replaced the "fee amount + split amount" model
  (`fee_bps` + `fee_split_bps_platform`) with two direct percentages of gross
  (`platform_fee_bps` + `ecosystem_fee_bps`, default 50/50 → preserves prior
  `fee_bps=100 × split=5000/10000 ⇒ 50 bps each side`). The DB keeps the legacy
  `fee_bps`/`fee_split_bps_platform` columns for rollback safety + Move struct parity;
  the indexer zeroes them (0) and writes the new columns from the new event fields.
- **SPT non-platform split:** replaced `platform_fee / 2` at 4 distribution sites with
  explicit `non_platform_platform_to_creator_bps` / `non_platform_platform_to_treasury_bps`
  (default 5000/5000 ⇒ identical to old `/2` behavior). Invariant:
  `creator + treasury == 10000`.
- **MessagingConfig is a separate package:** `messaging` has no `bootstrap.move`;
  `MessagingConfig` + `MessagingAdminCap` are shared/minted in `messaging::init` and the
  genesis object IDs must be captured for testnet/mainnet. `messaging::version` stays at
  `PACKAGE_VERSION = 1` (no bump).
- **AiCredit pubkey indexing fix:** `AiCreditOraclePubkeyUpdated` previously carried only
  `updated_by`; the Move event now carries `new_pubkey: vector<u8>`, the indexer parses it
  to `new_pubkey_hex` and persists it via the `AiCreditConfigPubkeyUpdate` row (previously
  dropped — only `updated_by` was parsed).
- **15 schema migrations (9 ALTER + 6 new hypertables):** consolidated into a single
  idempotent migration `20260704010000_dynamic_ecosystem_configs` (guards with
  `IF NOT EXISTS` / `ADD COLUMN IF NOT EXISTS`). See "Executor's Feedback" for a
  duplication concern with the per-config migration dirs.
- **GraphQL aggregation:** new `AiCreditConfig` type (no GraphQL type existed before) +
  5 new config types + `InsuranceRouterConfig` + `EcosystemTreasury`; unified
  `InsuranceConfiguration` aggregator nests `pricing` + `router`. Powered by new
  social-reader `get_*_config` methods (incl. the gap-fix `get_ai_credit_config`) and
  `get_ecosystem_treasury` + `EcosystemTreasuryRow`.

## High-Level Task Breakdown (plan todos + status)
1. **move-batch1** — AiCreditConfig (+oracle_markup_bps, update_min_deposit, pubkey event
   fix), PostConfig promotion fields, SPT non-platform split bps, EcosystemTreasury
   profile_sale_fee_bps — **completed**
2. **move-batch2** — SpotConfig limits + fee redo, PoCConfig dispute/vault + post.move
   threading, MyDataConfig max_encryption_id_bytes, InsuranceConfig odds_base_bps +
   InsuranceRouterConfig max_route_legs (real assert) — **completed**
3. **move-batch3** — SubscriptionConfig+cap, ProfileConfig+cap, MemoryConfig+cap,
   PlatformConfig (reuses PlatformAdminCap); bootstrap.move wiring — **completed**
4. **move-messaging** — MessagingConfig+MessagingAdminCap in messaging package; refactor
   paid_escrow_settlement + message_log to read config — **completed**
5. **schema-migrations** — 15 migrations (9 ALTER + 6 new hypertables) consolidated into
   `20260704010000_dynamic_ecosystem_configs`; schema.rs + models/ updated — **completed**
6. **indexer-handlers** — events.rs BCS structs + parse arms; domain handlers +
   SocialEventRow variants; new MessagingConfigHandler; AiCreditOraclePubkeyUpdated arm
   fixed; registered in lib.rs — **completed**
7. **reader-rest** — social-reader get_*_config + pg_reader wrappers (incl.
   get_ai_credit_config gap fix); REST routes + handlers (8 extended + 5 new) — **completed**
8. **graphql** — social_config.rs types extended + new types; resolvers in query.rs
   (aiCreditConfiguration + 5 new + ecosystemTreasury); unified InsuranceConfiguration;
   SDL snapshot updated — **completed**
9. **e2e-tests** — Move unit tests; indexer BCS roundtrip tests; REST reader tests; GraphQL
   SDL snapshot; release smoke checklist — **in progress (this scratchpad + Rust tests)**
10. **publish-migrate** — greenfield first publish; diesel migration run; verify bootstrap
    defaults on REST+GraphQL; lint — **in progress (smoke checklist below)**

## Project Status Board
- [x] Move batch 1 (AiCredit, Post, SPT, EcosystemTreasury) — greenfield, no migrate
- [x] Move batch 2 (Spot fee redo + limits, PoC, MyData, Insurance, InsuranceRouter)
- [x] Move batch 3 (Subscription, Profile, Memory, Platform) + bootstrap wiring
- [x] Move batch 4 (MessagingConfig in messaging package)
- [x] Schema: consolidated migration `20260704010000_dynamic_ecosystem_configs` (up/down)
- [x] Diesel `schema.rs` + `models/` updated (post_config inline-insert; spot_config keeps legacy fee cols)
- [x] Indexer events.rs BCS structs + parse arms for every new/extended event
- [x] Indexer domain handlers + SocialEventRow variants + MessagingConfigHandler
- [x] Indexer AiCreditOraclePubkeyUpdated arm fixed (carries `new_pubkey`)
- [x] Messaging indexer pipelines consolidated into single `messaging` handler (policy + config + payment)
- [x] social-reader get_*_config + pg_reader wrappers (incl. get_ai_credit_config gap fix + get_ecosystem_treasury)
- [x] REST: 8 extended + 5 new `/configuration` endpoints (subscription, profile, memory, platform, messaging)
- [x] GraphQL: AiCreditConfig + 5 new types + InsuranceRouterConfig + EcosystemTreasury + unified InsuranceConfiguration
- [x] GraphQL resolvers (aiCreditConfiguration, messagingConfiguration, subscriptionConfiguration, profileConfiguration, memoryConfiguration, platformConfiguration, ecosystemTreasury, insuranceConfiguration)
- [x] GraphQL SDL snapshot regenerated (`schema.graphql` + `.snap`)
- [x] `cargo fmt --all --check` passes
- [x] Rust lib tests (287/287) across schema, social indexer, social-reader, graphql — **PASS**
- [x] GraphQL SDL snapshot (`test_schema_sdl_export`) — updated and passing
- [x] SPT handler unit test fixed (`non_platform_platform_to_*` event fields)
- [x] Move source build (myso-social + messaging) — **PASS**
- [x] Move unit tests — **356/356 PASS** (295 myso-social + 61 messaging); all compile errors resolved
- [x] New indexer BCS roundtrip + handler tests added (events.rs, ai_credit.rs, spot.rs)
- [x] Greenfield publish smoke checklist documented below
- [ ] (Owner) greenfield publish on-chain + diesel migration run + REST/GraphQL default verification

## Executor's Feedback or Assistance Requests
- **Move tests fully passing (356/356):** All compile errors (originally ~444) in `myso-social` and `messaging`
  packages have been resolved. Fixes included: adding `take_shared`/`return_shared` for new config objects
  (`MemoryConfig`, `ProfileConfig`, `PlatformConfig`, `MessagingConfig`), prepending `&config` args to updated
  function signatures (`register_sub_agent`, `create_agent_group`, `send_agent_paid_message_digest`, etc.),
  fixing double-take issues (holding config across blocks while helpers also take it), and ensuring `next_tx`
  between object creation and `take_shared`. Key test files fixed: `memory_org_governance_tests`,
  `memory_organization_tests`, `paid_escrow_settlement_tests`, `agent_messaging_tests`, and 17+ others.
- **Rust side is green:** 287/287 lib tests pass; SDL + pipeline snapshots updated.
- **Migration duplication concern (needs owner decision):** the repo currently contains BOTH
  the consolidated migration `20260704010000_dynamic_ecosystem_configs` (416-line up.sql
  covering all 9 ALTERs + 6 new hypertables) AND 15 individual per-config migration dirs
  (`20260704010000_ai_credit_config_markup`, `20260704020000_post_config_promotion`, …,
  `20260704140000_create_messaging_config`). Two issues: (1) `20260704010000_ai_credit_config_markup`
  shares the exact same timestamp prefix `20260704010000` as the consolidated migration,
  which makes diesel's ordering ambiguous; (2) running both sets applies every change twice.
  The consolidated migration is idempotent (`IF NOT EXISTS`), so a double-apply is safe but
  noisy and the duplicate timestamp should be resolved. **Recommendation:** keep the
  consolidated migration (per the smoke checklist) and remove the 15 individual dirs (or
  vice-versa) before `diesel migration run`. Not acted on here — in-place edits only and the
  owner owns the final schema-set decision.
- **Pre-existing fmt debt:** `cargo fmt --all --check` initially flagged 44 files. Most were
  in the 5 target crates (dynamic config E2E work), but `crates/myso-ai-credit-oracle/**`
  had pre-existing formatting debt unrelated to this work (not in the git-modified set).
  `cargo fmt --all` was run per the task instruction to make the repo-wide check green;
  the oracle cleanup is a side-effect the owner should be aware of.
- **No REST handler unit tests added:** the new `/configuration` routes are thin DB-backed
  wrappers (`state.reader.get_*_config().await?.ok_or(...).map(Json)`) with no
  pure-function logic. The only pure unit tests in `myso-social-server` target helpers like
  `parse_profile_pnl_windows`; DB-backed handler tests use `TempDb` (real Postgres), which
  is integration-test territory. Per the task guidance ("do NOT force tests where no harness
  exists"), I did not invent a new REST test harness. The reader/REST path is covered by the
  GraphQL snapshot + indexer BCS roundtrip tests + existing reader tests.
- **`cargo xclippy` NOT run** per user instruction. Lint verification limited to
  `cargo fmt --all --check`.
- **No git/PRs** per workflow constraints — all work is in-place file edits; owner handles
  version control.

## Lessons
- `messaging_config.move` needed `object`/`transfer`/`TxContext` imports — the messaging
  package has no `bootstrap.move`, so `MessagingConfig` + `MessagingAdminCap` are shared
  and minted in `messaging::init` directly.
- `bootstrap.move` needed `subscription::bootstrap_init` insertion + 3 new cap mints
  (`SubscriptionAdminCap`, `ProfileAdminCap`, `MemoryAdminCap`); `PlatformConfig` reuses
  the existing `PlatformAdminCap` (no new cap). Cap block slot order matters for dependency
  ordering (governance before spot).
- `memory.move` needed an `update_memory_config` entry + cap creator added; `MemoryConfig`
  follows memory's internal `VERSION = 4` convention, NOT `upgrade::CURRENT_VERSION`, and a
  future `migrate_memory_config` would take `&UpgradeCap` (matching `migrate_registry`),
  not `&UpgradeAdminCap`.
- GraphQL needed `EcosystemTreasuryRow` + `get_ecosystem_treasury` added to the
  social-reader to power the top-level `ecosystemTreasury` resolver (the treasury is a
  capital/fee object, not a pure config, so it gets its own reader method).
- **New BCS event structs have private fields** (unlike `BcsPaidMessagingPolicyUpdated`
  which has `pub` fields). BCS roundtrip tests that construct the struct with a literal must
  live inside `events.rs::tests` (which can access private fields), not in handler test
  modules. The handler-level JSON→row mapping is tested separately in the handler's own
  `mod tests` (e.g. `ai_credit.rs`, `spot.rs`) using hand-built JSON, mirroring the
  existing `usage_settled_emits_...` pattern.
- SpotConfig fee redo: the indexer sets legacy `fee_bps`/`fee_split_bps_platform` to `0`
  and writes `platform_fee_bps`/`ecosystem_fee_bps` from the new event; the legacy columns
  are retained on the `spot_config` table for rollback safety + Move struct parity during
  the transition window.
- The GraphQL SDL snapshot test (`test_schema_sdl_export` in `lib.rs`) writes
  `schema.graphql` from the live SDL and then `assert_snapshot!`s it against
  `src/snapshots/myso_indexer_alt_graphql__tests__schema.graphql.snap`. Regenerate a stale
  snapshot with `INSTA_UPDATE=always cargo nextest run -p myso-indexer-alt-graphql --lib test_schema_sdl_export`
  then re-run to confirm.

## Greenfield Publish Smoke Checklist (Phase 8.5 — adapted for greenfield NO-migrate)
- [x] **Move build (social):** `cargo run -p myso-move -- -p crates/myso-framework/packages/myso-social -e localnet build` — **PASS** (sources only; tests still compiling)
- [x] **Move build (messaging):** `cargo run -p myso-move -- -p crates/myso-framework/packages/messaging -e localnet build` — **PASS**
- [ ] **No version bump:** `CURRENT_VERSION` in `crates/myso-framework/packages/myso-social/sources/upgrade.move` is UNCHANGED (greenfield — no bump); `messaging::version` `PACKAGE_VERSION = 1` unchanged.
- [ ] **No migrate runs:** NO `migrate_*` entries are run on-chain (nothing to backfill). `UpgradeAdminCap` is minted by `bootstrap.move` for future upgrades only.
- [ ] **Bootstrap wiring:** `bootstrap_init` calls + cap mints in `bootstrap.move` are in the correct slot order (platform → social_graph → profile → block_list → mydata → memory → governance → post → subscription → social_proof_tokens → proof_of_creativity → social_proof_of_truth(spot_gov_id) → insurance → ai_credit); `SubscriptionAdminCap`, `ProfileAdminCap`, `MemoryAdminCap` minted + transferred; `PlatformConfig` reuses existing `PlatformAdminCap`.
- [ ] **Messaging genesis:** `messaging::init` shares `MessagingConfig` + mints `MessagingAdminCap`; record the genesis object IDs for testnet/mainnet in `network.config/`.
- [ ] **Diesel:** `migrations/20260704010000_dynamic_ecosystem_configs/{up,down}.sql` is the single consolidated migration; run `diesel migration run --database-url $DATABASE_URL` against the social DB; verify rollback with `diesel migration redo`. (Resolve the duplicate-timestamp/individual-migration concern in "Executor's Feedback" first.)
- [ ] **Indexer:** build + run `myso-indexer-alt-social`; confirm new events parse without errors (AiCredit pubkey fix, Spot fee redo, SPT split, 5 new configs, MessagingConfig) and rows land in the 6 new + 8 extended tables.
- [ ] **REST defaults:** hit the 8 extended + 5 new config endpoints; verify defaults preserve prior behavior:
  - spot `platformFeeBps=50` / `ecosystemFeeBps=50` (== old 100×5000/10000 ⇒ 50/50 of gross)
  - spt `nonPlatformPlatformToCreatorBps=5000` / `nonPlatformPlatformToTreasuryBps=5000` (== old `/2`)
  - profile `profileSaleFeeBps=500`
  - ai credit `oracleMarkupBps=0`
  - insurance `oddsBaseBps=5000`; router `maxRouteLegs=4`
  - post `minPromotionAmount=1000` / `maxPromotionAmount=100000000` / `minViewDurationMs=3000`
  - mydata `maxEncryptionIdBytes=1024`; poc `maxDisputesPerPost=2` / `minVaultDepositAmount=1`
  - subscription `billingPeriodMs=2592000000` / `maxRenewalMonths=120`
  - messaging `paidMsgPlatformFeeBps=250` / `paidMsgTreasuryFeeBps=250` / `paymentExpirationMs=2592000000` / `minReplyChars=6` / `maxDedupeKeyBytes=256`
  - platform/memory/profile config defaults per plan §2.3/§2.4/§2.2
- [ ] **GraphQL:** query `aiCreditConfiguration` + the 5 new `*Configuration` resolvers + unified `insuranceConfiguration` (with nested `router.maxRouteLegs`) + `ecosystemTreasury`; confirm SDL snapshot matches (`test_schema_sdl_export` passes).
- [ ] **Update tx round-trip:** submit one `update_*` tx per config via PTB; verify REST + GraphQL reflect the new values within one checkpoint.
- [ ] **Lint:** `cargo fmt --all --check` passes (verified green in this work). Do NOT run `cargo xclippy` (user instruction). Optionally `./scripts/lint.sh` for Move formatting.
- [ ] **No git/PRs:** owner handles version control.

---

## Mysocial Frontend Dynamic Config E2E (plan: frontend_dynamic_config_e2e)

### Status — **completed** (2026-07-04)

All five market admin config dialogs in `mysocial-frontend` wired to new on-chain dynamic configs:

| Dialog | File | Changes |
|--------|------|---------|
| MyData | `components/sections/ecosystem/mydata-admin.tsx` | +`max_encryption_id_bytes`; save/toggle TX + clock |
| SPT | `components/sections/ecosystem/social-proof-token-admin.tsx` | +`non_platform_platform_to_*` bps; save TX + clock |
| PoC | `components/sections/ecosystem/proof-of-creativity-admin.tsx` | +3 fields; save TX + clock; `dispute_governance_id` read-only |
| Insurance | `components/sections/insurance/insurance-admin.tsx` | +`odds_base_bps`; router section; `set_router_flags`/`set_router_limits`; unified REST parse; clock on all TX |
| SPOT | `components/sections/ecosystem/spot-admin.tsx` | Fee redo (`platform_fee_bps`/`ecosystem_fee_bps`); +5 limits; GraphQL `spot_governance_registry_id`; 16-arg TX + clock |

### Automated verification
- `pnpm exec tsc --noEmit` — **PASS**
- `pnpm build` — **PASS**

### Manual smoke checklist (admin-cap wallet on localnet/testnet)
1. Open each explore page → Configurations dialog
2. Confirm new rows render with indexer values (non-admin: read-only)
3. Edit one field per module → Save → tx succeeds
4. Re-open dialog → value matches indexer after refresh

Suggested order: MyData → SPT → PoC → Insurance → SPOT

### Frontend config fetch → GraphQL (2026-07-04 follow-up)

**Problem:** Admin panels still proxied to social-indexer REST; newer fields (insurance router, spot fee split, etc.) were stale or missing.

**Fix (mysocial-frontend):**
- `app/api/graphql/queries/ecosystem-configurations.ts` — full-field queries for all 12 config types + ecosystem treasury
- `lib/ecosystem-configuration-graphql.ts` — server GraphQL fetch + camelCase→snake_case normalization
- `lib/ecosystem-configuration-route.ts` — shared Next.js route handler
- All `/api/*/configuration` routes now query GraphQL (not REST indexer)
- Added `/api/post/configuration` + `/api/poc/configuration`; post-admin + poc-admin updated to use them
- Insurance route flattens `insuranceConfiguration { pricing, router }` for existing admin normalizers

**Verification:** `npx tsc --noEmit` — **PASS**
