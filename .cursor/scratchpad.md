
## Fix Admin Reassign Profile Username Indexing (2026-07-14)

### Background and Motivation
`username-admin-runnable.sh` timed out waiting for GraphQL `profiles.username` after
admin reassign even though on-chain + `username_registry` were correct.

### Key Challenges and Analysis
- `UsernameReassignedEvent` sets destination username before source Claimed rename.
- `idx_profiles_username` UNIQUE + NOT NULL blocked destination Set; registry already
  committed (non-transactional passes) → stale `profiles.username`.
- Plan said NULL clear; column is NOT NULL → park conflicting holders on
  `__releasing__{profile_id|owner|id}` before SET.

### Project Status Board
- [x] ProfilesHandler: free contested username before ProfileUsernameSet
- [x] Commit unit test: destination-first reassign batch
- [x] Script: prompt SOURCE_REPLACEMENT_USERNAME (ASSUME_YES keeps default)
- [x] Shell + focused nextest
- [ ] Rebuild myso + restart localnet + E2E smoke

### Executor's Feedback
- Shell test PASS (27). Focused nextest PASS (3).
- E2E needs rebuilt `myso` (social indexer is in-process) + localnet restart.

### Lessons
- Marketplace settle avoids unique conflicts by source-first Sets; admin reassign
  split across events needs conflict-free commit.

## Reassign-Only Username Admin Cleanup (2026-07-13)

### Background and Motivation
Remove `admin_revoke_username` + `UsernameRevokedEvent` + indexer `ProfileUsernameClear`.
Keep only `admin_reassign_username`; marketplace sale carries freed buyer username on
`UsernameSaleSettledEvent`; reassign carries freed target prior on `UsernameReassignedEvent`.

### Project Status Board
- [x] Move: remove revoke/RevokedEvent; add `prior_buyer_username` / `prior_target_username`
- [x] Move tests: revoke deleted; reassign locked/taken replacement + sale event count
- [x] Indexer: remove Clear + Revoked; settle/reassign emit `UsernameRegistryDelete`
- [x] E2E: `username-admin-runnable.sh` reassign-only; shell test updated
- [x] Verification: Move `admin_` + accept offer; indexer username tests; shell call-order

### Executor's Feedback
- Live smoke (`ASSUME_YES=1 ./scripts/username-admin-runnable.sh --run-all`) needs localnet
  framework bytecode refresh for the new entry signatures / event BCS layouts.

### Lessons
- Registry deletes must travel on SaleSettled / Reassigned prior_* fields once Revoked is gone.
- Profiles stay named via Set/Claimed only; Clear path is fully removed from the indexer.

## Dynamic MemoryAccount for MyData Marketplace (2026-07-13)

### Background and Motivation
Fix stale shared-object session preserve; MemoryAccount for purchase/approve is
**manual prompt** (auto GraphQL fetch reverted — not reliable for this flow).

### Key Challenges and Analysis
- Abort 4 on purchase was `ESelfPurchase` (owner wallet buying own listing), not MA fetch.
- Shared-object preserve-key fix stays; purchase menus prompt for MemoryAccount.

### Project Status Board
- [x] Preserve-key + shared-id preflight kept
- [x] Menus 3/4/7 prompt for MemoryAccount (no auto-fetch)
- [x] Self-purchase guard kept
- [x] Unit test PASS

### Lessons
- Never put GraphQL shared-object ids in session preserve lists; refresh must win.
- Purchase as listing owner → Move `ESelfPurchase` (4); switch buyer wallet first.

## Username Admin + SPOT Insurance Runnables (2026-07-13)

### Background and Motivation
Add two local E2E runnable scripts: `username-admin-runnable.sh` for `UsernameAdminCap`
revoke/reassign, and `spot-insurance-runnable.sh` for SPOT market → insurance buy/claim.

### Key Challenges and Analysis
- Common social GraphQL refresh loads `UsernameRegistry` but not `UsernameAdminCap`; the
  username-admin script needs a script-local extras query + cap-owner resolution.
- `wait_for_gql_profile_field` cannot assert an empty username (`-n` guard); need a dedicated
  empty-username waiter after revoke.
- Insurance helpers already live in `spot-oracle-common.sh`, but oracle infra (postgres,
  waiters, worker boot) lives in `spot-oracle-runnable.sh` and must be mirrored in the
  dedicated insurance script without relying on `ENABLE_INSURANCE_E2E=1`.

### High-Level Task Breakdown
1. Implement `scripts/username-admin-runnable.sh` (menu 0 refresh / 1 revoke+reassign).
2. Implement `scripts/spot-insurance-runnable.sh` using SPOT walkthrough + insurance helpers.
3. Add lightweight shell syntax/call-order tests and `bash -n` validation.
4. Log progress/verification here.

### Project Status Board
- [x] Username-admin runnable created
- [x] Spot-insurance runnable created
- [x] Shell syntax / call-order tests
- [x] Verification (`bash -n` + tests)

### Executor's Feedback or Assistance Requests
- Scripts are ready for live localnet smoke:
  - `ASSUME_YES=1 ./scripts/username-admin-runnable.sh --run-all`
  - `ASSUME_YES=1 ./scripts/spot-insurance-runnable.sh --run-all`
- Live runs were not executed here (require bootstrap + GraphQL + oracle stack).

### Lessons
- `spot-oracle-post-runnable.sh` always writes its own `spot-oracle-session.env`; a sibling
  runnable must extract `POST_ID` from that file rather than expecting shared session path.
- GraphQL profile waiters that require non-empty values cannot assert username cleared after
  revoke; use a dedicated empty-username waiter.

## MyData P2P/Profile Runnable Fix (2026-07-13)

### Background and Motivation
Fix `scripts/mydata-marketplace-runnable.sh` so local E2E testing can focus on profile-gated MyData plus p2p one-time listing/purchase flows, while avoiding the oracle/pool marketplace actions.

### Key Challenges and Analysis
- `create_and_share_internal` gates all profile, one-time, and recurring MyData creation behind `MyDataConfig.marketplace_enabled`; this is separate from the oracle/pool query marketplace flow.
- The admin cap owner is a different local wallet (`0x751ec787...`), so the config toggle had to be submitted from that wallet and the active address restored after.
- The script did not persist newly created `MyData` object IDs or encryption IDs, making subsequent purchase/approve steps depend on manual effects parsing.
- The admin-cap lookup `jq` filter assumed `.data.Move.type_.Other` was always an object, but the CLI can emit string type shapes.

### High-Level Task Breakdown
1. Toggle `mydataConfiguration.marketplaceEnabled` on localnet using the admin-cap owner wallet.
2. Patch only `scripts/mydata-marketplace-runnable.sh` to improve session handoff and robust ID lookup.
3. Run profile, p2p one-time, purchase, approve, owner update, access grant/revoke, and registry maintenance paths.
4. Skip oracle/pool marketplace menus (`create_broad_pool` through claim/refund flow).

### Project Status Board
- [x] Switched to admin-cap owner and enabled MyData config; switched back to original active wallet.
- [x] Fixed admin-cap object type detection for mixed CLI JSON shapes.
- [x] Persisted `MYDATA_ENCRYPTION_ID` in the session.
- [x] Captured new `LISTING_ID` from create transaction effects.
- [x] Refreshed active profile `MEMORY_ACCOUNT_ID` during GraphQL refresh.
- [x] Created one-time p2p listing and purchased it from a separate wallet.
- [x] Ran `mydata_approve` for the purchased one-time listing.
- [x] Created profile-subscription-gated MyData.
- [x] Ran pricing update, tags update, grant access, revoke access, unregister, and re-register.
- [x] `bash -n scripts/mydata-marketplace-runnable.sh` passes.
- [x] `bash scripts/tests/mydata-marketplace-runnable-test.sh` passes.

### Executor's Feedback or Assistance Requests
- Oracle/pool query marketplace actions were intentionally not exercised.
- A no-op `update_content` attempt aborted with `EInvalidInput` as expected; reran with a tags update and it succeeded.

### Lessons
- `MYDATA_FORCE_PROMPT=1` is needed when overriding session-backed listing IDs in piped menu runs.
- GraphQL may lag the on-chain config toggle for a moment; rechecking after the successful tx showed `marketplaceEnabled=true`.

## Subscription + MyData Access Hardening (2026-07-10)

Completed per `.cursor/plans/subscription_access_hardening_720381aa.plan.md`:
- Move: `PostAccess` on `Post`, `AccessConfiguration` on `MyData`, gate removed, approve paths hardened
- Indexer/GraphQL: `post_access_kind`, `access_configuration_kind`, `Post.access` / `MyData.accessConfiguration`
- E2E: menu 14 marketplace one-time flow; profile PTB passes `post_service_id` + `post_linked_mydata_id`
- Docs: `UPDATE=1 build_system_packages` refreshed `published_api.txt` + `post.md`/`mydata.md`
- Migrations: backfill SQL in existing `20250620000001` and `20250615000000` migration files


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

## Remove PoC Opt-In + SPT Auto-Bootstrap (2026-07-09)
- **Move:** `Post.enable_flags` → explicit `enable_spt` / `enable_spot`; removed `enable_poc` and `is_poc_enabled` gate in PoC; `poc_disputes_submitted` moved to dynamic field (VM 32-field limit). SPT pool creation is explicit via `social_proof_tokens::create_reservation_pool_for_post` (not auto-bootstrapped at post create).
- **Off-chain:** `enable_poc` removed from migration (in-place), schema, indexer events/handlers, reader, GraphQL, social server, spot-oracle ingest.
- **Scripts:** PTBs use `post::create_post`; separate `create_reservation_pool_for_post` when user enables SPT.
- **Verify:** `myso move test -e localnet` 316/316; `cargo nextest run -p myso-indexer-alt-social -p myso-indexer-alt-graphql --lib` 292/292; e2e runnable scripts require live localnet (not run here).

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

---

## Username Marketplace + Post Promotion E2E Scripts (2026-07-05)

Plan: `.cursor/plans/username_marketplace_e2e_scripts_fb85eb6e.plan.md` (scripts-only; Move + indexer already shipped).

### Deliverables
- [x] `scripts/lib/social-runtime-common.sh` — GraphQL session refresh (incl. `UsernameMarketplace`), PTB/call helpers, `create_profile_for_address` (current sig), wallet/faucet, promotion helpers
- [x] `scripts/username-marketplace-runnable.sh` — list → offer → accept + REST/GraphQL asserts; optional `--reject-flow`
- [x] `scripts/post-promotion-runnable.sh` — platform + promoted post + activate + confirm view + GraphQL asserts

### Fix applied during validation
- Both runnable scripts now set `SOCIAL_SESSION_SAVE_PATH` **before** sourcing `social-runtime-common.sh` (required by lib guard).

### Automated validation (this session)
| Check | Result |
|-------|--------|
| `bash -n` on all three scripts | **PASS** |
| `myso move test -e testnet … profile_tests` | **PASS** (32 tests) |
| `myso move test -e testnet … test_accept_username_offer` | **PASS** |
| `myso move test -e testnet … test_promoted_post_creation` | **PASS** (if run) |
| `ASSUME_YES=1 ./scripts/username-marketplace-runnable.sh --refresh-session` | **BLOCKED** — GraphQL :9125 not reachable |
| `ASSUME_YES=1 ./scripts/*-runnable.sh --run-all` on localnet | **BLOCKED** — localnet could not be started |

### Localnet blocker (environment)
- Default `~/.myso/myso_config` binds validators to `69.10.63.78` → `Can't assign requested address (os error 49)`.
- `--force-regenesis` fails genesis publish unless `MYSO_PROTOCOL_CONFIG_OVERRIDE_max_move_package_size>=512000` (social package ~255 KiB > 200 KiB default).
- Fresh genesis + start still hit `Address already in use (os error 48)` in swarm container bind (multi-validator local start).

### Manual E2E when localnet is up
Prerequisites: `myso start --with-faucet --with-graphql --with-social-indexer` (with package size override if regenesis), then `./scripts/bootstrap.sh`.

```bash
ASSUME_YES=1 ./scripts/username-marketplace-runnable.sh --refresh-session --run-all
ASSUME_YES=1 ./scripts/post-promotion-runnable.sh --refresh-session --run-all
```

Session files: `network.config/username-marketplace/marketplace-session.env`, `network.config/post-promotion/promotion-session.env`.

---

## Discovery Framework E2E Completion (2026-07-08)

Plan: discovery E2E completion (Phases 1–4). Do not edit the plan file.

### Project Status Board
- [x] Phase 1 — Harden discovery (`source_id`, embed Fail lifecycle, metrics/admin/retries, manual_curated gate)
- [x] Phase 2 — SPoT post helper + pending-posts + `--run-all-onchain`
- [x] Phase 3 — `discovery-poc-runnable.sh` + ARCHITECTURE.md ports/secrets matrix
- [x] Phase 4 — session/compose polish, dead config cleanup, clippy `--no-deps -D warnings`
- [x] Greenfield migration fold — `max_attempts`/`run_after` inlined into `20260708000000_initial_discovery_schema`; deleted `20260708000002_discovery_jobs_retry`

### Lessons
- Prefer editing the initial greenfield migration over additive ALTER migrations while schemas are still unpublished.
- `cargo xclippy -p` is unsupported; use `cargo clippy -p … --no-deps -- -D warnings` for touched crates.
- On-chain SPoT E2E needs full `oracle_resolve` PTB (platform + treasury + spot_record_id from create objectChanges); script asserts accepted → active+spot_id → evidence → resolved.
- `SPOT_ORACLE_ONCHAIN=1` is an alias for `--run-all-onchain`.

---

## Discovery Media vs Text Split (2026-07-08)

Plan: `discovery_media_text_split_*.plan.md` (do not edit). Creative media → PoC; factual text → SPoT.

### Project Status Board
- [x] Core: `ContentKind`, `normalize_media_type`, adapters + normalizer
- [x] Schema/store: `content_kind` on `discovery_assets` (greenfield initial migration) + upsert
- [x] Scheduler: embed only `creative` + `media` when embed enabled; `skipped_non_media` metric
- [x] Configs/scripts: `sources.media.localnet.yaml`; poc-runnable defaults to media + `MANUAL_CURATED=1`
- [x] PoC: 400 non-media / 422 unreadable; unit tests + docs
- [x] SPoT docs: factual TrustedSource only; no `discovery_assets` reads
- [x] E2E asserts: factual runnable checks `content_kind=text`; poc runnable checks `content_kind=media`

### Manual verify (when stacks up)
```bash
# Wipe discovery volume if schema changed (or omit REUSE_DB)
KEEP_STACK=1 ./scripts/discovery-runnable.sh

DISCOVERY_EMBED_SECRET=… \
DISCOVERY_EMBED_ENDPOINT=http://127.0.0.1:8001/internal/discovery/embed \
  ./scripts/discovery-poc-runnable.sh
```

---

## `myso start` sidecars (2026-07-08)

Plan: `myso_start_sidecars_*.plan.md` (do not edit).

### Project Status Board
- [x] CLI: `--with-spot` / `--with-poc` / `--with-messaging` (+ `--poc-repo` / `--messaging-repo`)
- [x] `sources.combined.localnet.yaml`
- [x] `local_discovery` / `local_spot_oracle` / `local_poc` / `local_messaging` + `LocalSidecars` lifecycle
- [x] Docs: `local-network.mdx`

### Discovery policy
| Flags | Discovery | YAML |
|-------|-----------|------|
| spot only | once | factual |
| poc only | once | media + embed |
| both | once | combined + embed |
| neither | none | — |

---

## SPoT ↔ Discovery Architecture Unification (2026-07-08)

Plan: `.cursor/plans/spot_discovery_unification_36bff8df.plan.md` (read-only).

### Background
Unify factual data through Discovery `/v1/*` + `myso-discovery-client`; replace SPoT HTTP
pending-posts poller with `SubscribeCheckpoints` gRPC ingest filtering `PostCreatedEvent.enable_spot`.

### Project Status Board
- [x] Phase 0: DTOs + `DiscoveryClient` trait; ARCHITECTURE.md updates
- [x] Phase 1: `discovery_factual_cache`, rate-limit, factual YAML SoT
- [x] Phase 2: `/v1/*` handlers + `myso-discovery-client` crate
- [x] Phase 2b: checkpoint ingest + watermark store + dual-path ingest mode
- [x] Phase 3–5: Discovery-only `TrustedSource` adapters; registry from `/v1/sources`
- [x] Phase 6: metrics, auth secrets, spot DB schema-only (no co-hosted discovery migrations)
- [x] Scripts/env: `spot-oracle-common.sh`, session env, `local_spot_oracle.rs`, runnable starts Discovery
- [x] `cargo check` green on `myso-discovery-service`, `myso-spot-oracle`, `myso`

### Executor's Feedback
- On-chain E2E reordered: start oracle **before** post creation so checkpoint stream catches `PostCreatedEvent`.
- `SPOT_ORACLE_INGEST_MODE` default `checkpoint`; HTTP poller kept only for explicit `http|both`.
- Factual settlement requires `SPOT_ORACLE_DISCOVERY_CLIENT_URL`; direct `HttpFetchClient` removed from SPoT adapters.

---

## PoC E2E Gap Fill (2026-07-09)

Closed operational gaps between Move contracts, proof-of-creativity docker stack, and myso-core runnable scripts.

### Project Status Board
- [x] Oracle worker import fix + unit test (proof-of-creativity)
- [x] Dedupe `build_reserve_towards_post_with_platform_call` in move_calls.py
- [x] `validate_registry_objects` for poc_config + poc_beneficiary_admin_cap
- [x] `scripts/lib/poc-oracle-common.sh` + `network.config/poc/oracle-localnet.env`
- [x] `local_poc.rs`: grpc-sync, full PoC `.env`, sync status wait, e2e hint
- [x] Chain event asserts wired into poc-oracle-post + proof-of-creativity runnables
- [x] `scripts/poc-e2e-runnable.sh` unified loop
- [x] `proof-of-creativity/scripts/poc-claim-runnable.sh` mock claim E2E
- [x] Docs: oracle-runbook, discovery-runbook, local-network.mdx (mock_mode fix)
- [ ] Live `poc-e2e-runnable.sh --run-all` (requires running localnet + PoC stack)

### Lessons
- Chain events via `myso client tx-block` are the E2E gate; GraphQL is downstream cross-check only.
- `tip_post_simple` does not emit `PoCBeneficiaryVaultDepositEvent`; tip leg uses `tip_post` with vault.
- Localnet `grpc_sync.mock_mode` defaults to `false` for live checkpoint sync with `myso start --with-poc`.


Moved `HttpDiscoveryClient` from deleted `myso-discovery-client` crate into
`myso-discovery-service-core/src/api/http.rs`. SPoT now imports from `-core` only.

### Project Status Board
- [x] Move `HttpDiscoveryClient` + unit test into `-core` `api/`
- [x] Rewire `myso-spot-oracle` imports; remove `-client` dependency
- [x] Delete `crates/myso-discovery-client`; remove from workspace `Cargo.toml`

### Lessons
- Separate `-client` crate added no dependency-boundary value once SPoT already depended on `-core`.

---

## SPoT Independence from Discovery (2026-07-10)

### Background and Motivation
Make `myso-spot-oracle` compile and run without any Discovery crate, schema, API, source
table, configuration, or sidecar. Discovery remains unchanged for Proof of Creativity.
The target lifecycle is `Post → SpotClaim → SpotMarket → PostLink → scheduled direct
source resolution → indexed result → optional insurance claim`.

### Key Challenges and Analysis
- The six live `TrustedSource` adapters currently proxy all factual fetches through
  `HttpDiscoveryClient`; direct fetch and normalization must move into the existing SPoT
  adapter layer.
- The SPoT database still reads `discovery_sources`, and its schema documentation requires
  Discovery migrations even though runtime currently executes only SPoT migrations.
- Oracle creation still calls legacy `create_spot_record_for_post`; the Move contract's
  canonical model exposes separate `create_spot_claim` and `create_spot_market_for_claim`
  entry points.
- The refund PTB omits the `SpotClaimRegistry` argument required by `refund_unresolved`.
- Local scripts and `myso start --with-spot` currently start or configure Discovery.

### High-Level Task Breakdown
1. Replace Discovery source rows/config with SPoT-owned source definitions.
2. Port direct HTTP fetch and factual normalization into `TrustedSource` adapters.
3. Remove all Discovery Cargo imports, clients, metrics, jobs, and runtime wiring.
4. Align creation/refund PTBs with the Claim → Market → Post contract model.
5. Add deterministic source quorum and DAO escalation for conflicts.
6. Decouple CLI, Docker, environment, and E2E scripts from Discovery.
7. Expose existing claim/market/resolution data through GraphQL.
8. Add the minimal existing-contract insurance leg to the runnable.

### Project Status Board
- [x] Inventory SPoT compile-time/runtime Discovery coupling.
- [x] Add SPoT-owned trusted-source schema and config.
- [x] Implement direct trusted-source HTTP resolvers.
- [x] Remove Discovery Cargo dependencies and imports.
- [x] Fix Claim → Market PTBs and refund registry input.
- [x] Add quorum/conflict escalation.
- [x] Decouple scripts, CLI, Docker, and environment.
- [x] Complete no-Discovery SPoT E2E (scripts no longer start Discovery).
- [x] Export claim/market/resolution GraphQL schema (`spotClaim` / `spotMarket` / `spotRoute` / `Post.spotClaimId`).
- [x] Add minimal insurance E2E leg (`ENABLE_INSURANCE_E2E=1` walkthrough helpers).
- [x] Remove legacy SPoT oracle paths and update documentation.

### Executor's Feedback or Assistance Requests
- Compile green: `cargo check -p myso-spot-oracle`; resolver quorum unit tests pass.
- GraphQL SDL regenerated via `test_schema_sdl_export`.
- Live walkthrough / insurance E2E still needs a funded localnet to exercise end-to-end (helpers are in place).

### Lessons
- `SpotRecord` is no longer an on-chain object; legacy names now refer to `SpotMarket`.
- Existing SPoT evidence types already provide the correct persistence boundary, so no
  generic HTTP/provenance crate is needed.
- Quorum conflict must lower confidence (DAO_REQUIRED) rather than picking a silent winner.

---

## Subscription E2E decrypt ArityMismatch fix (2026-07-10)

### Root cause
`mydata_resolve_mydata()` preferred `target/release/mydata` over `target/debug/mydata`.
`mydata_ensure_fresh_cli()` only rebuilds debug (`cargo build -p mydata-cli`), so an older
release binary still emitted a 7-arg PTB while on-chain `mydata_approve_profile_subscription`
expects 9 args → key-server `ArityMismatch in command 1`.

### Fix (scripts only)
- `scripts/lib/mydata-test-common.sh`: staleness check + export `MYDATA` on debug binary;
  resolve order debug → release.
- `scripts/lib/subscription-test-common.sh`: clearer ArityMismatch hint.

### Verified
Menu 12 (`flow_subscriber_decrypt_encrypted_post`) decrypts successfully; non-subscriber
negative still fails as expected.

### Menus 11–14 PTB fixes (2026-07-10)
- **Menu 14 post create:** `create_marketplace_one_time_post` missing `enable_spt` / `enable_spot`
  (`none none`) — 19 → 21 args.
- **Menu 13 policy dry-run:** `ptb_pure_id` now emits `@0x…` so `object::ID` pure args parse correctly
  in PTB (bare hex was rejected).
- **Menu 14 purchase:** `purchase_one_time` now passes `MEMORY_CONFIG_ID` + `ECOSYSTEM_TREASURY_ID`
  (7 args total).

All menus 11–14 pass in one interactive session run.

---

## MyData paid one-time buy E2E + confirm-decline summary fix (2026-07-13)

### Context
User noted the profile listing was effectively "free" and asked to make a paid MyData
object and actually buy access to it.

### Findings
- The `profile` listing type is `AccessConfiguration::ProfileSubscription`: it has **no
  price** and `update_pricing` aborts with `ENotForSale`. Access derives from holding a
  profile subscription, not a p2p purchase — so it cannot be "made paid".
- The real paid/buyable p2p type is **one-time** (or recurring) via `purchase_one_time`.

### Verified E2E (localnet)
- Created paid one-time listing `0xe760…` at price `2_000_000_000` (2 MYSO).
- Bought as buyer `0x751ec787…`: tx `Success`; buyer balance `10_242_980_079 →
  8_342_936_412`; creator received `1_900_000_000` (2 MYSO − 5% p2p fees:
  250bps platform + 250bps ecosystem).
- Access proof via `has_access` dry-run view: buyer → `true`, unrelated address → `false`.

### Script bug fixed (no codebase change)
`run_myso_call` used `confirm_run || return 0`, and `print_mydata_operation_summary`
always printed "… completed". A declined confirmation therefore printed a misleading
"completed" summary while executing nothing (this masked the first two purchase attempts,
which silently no-op'd because a pre-filled session `MEMORY_ACCOUNT_ID` shifted stdin so
the blank line landed on the `[y/N]` confirm).
- `run_myso_call` now sets `MYDATA_LAST_CALL_EXECUTED` (0 on decline, 1 on run) and prints
  a `[skipped]` notice on decline.
- `print_mydata_operation_summary` prints "… NOT executed (confirmation declined)" when the
  call did not run.

### Lessons
- Interactive prompts that conditionally skip reads (session pre-fill) can misalign piped
  stdin; the confirm prompt then eats a data line. Prefer feeding exactly the lines that
  will be read, or use `SKIP_CONFIRM_RUN=1`/`ASSUME_YES=1` for non-interactive runs.
- Never print a "completed" summary independent of the executed/aborted status of the call.

