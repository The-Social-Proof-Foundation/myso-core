# Unify Time Format to Epoch Ms – Phases 3–5 Completed (Mar 2026)

## Summary
Standardized timestamp fields to epoch milliseconds across reader, GraphQL, and social server (Phases 0–2 done previously).

## Phases Completed
- **Phase 3 (Reader)**: ProfileByAddressResponse created_at/updated_at → Option<i64>; profile_to_response and wallet_social_graph paths convert NaiveDateTime via .and_utc().timestamp_millis()
- **Phase 4 (GraphQL)**: Profile, Platform, BlockedProfileSummary, BlockedPlatformSummary, PlatformBlockedProfileSummary, PlatformMembershipSummary, PlatformMemberSummary, PlatformModeratorSummary, VestingWallet — all timestamp resolvers return i64 (epoch ms); schema.graphql updated
- **Phase 5 (Social Server)**: ProfileByAddressResponse and WalletOnlyProfile created_at/updated_at → Option<i64>; From impls convert NaiveDateTime to ms; API_RESPONSE_FORMATS.md updated

## Remaining
- **Phase 6**: Data migration and backfill for existing rows with seconds in timestamp columns

## Verification
- cargo check -p myso-indexer-alt-social-reader -p myso-indexer-alt-graphql -p myso-social-server: passes
- INSTA_UPDATE=always cargo nextest run -p myso-indexer-alt-graphql --lib: 129 tests pass

---

# Governance GraphQL Integration – Completed (Mar 2026)

## Summary
Implemented full plan: Governance types and queries in GraphQL; SocialPgReader governance module; social server platform_id support and registry-by-platform endpoint; schema row types consolidation.

## Phases Completed
1. **Social Schema**: ProposalRow, DelegateRow, GovernanceRegistryRow, GovernanceRegistryConfig, and all governance row types (DelegateVoteRow, CommunityVoteRow, etc.) in models/governance.rs
2. **SocialPgReader**: governance module with list_proposals(platform_id, status, ...), get_proposal_by_id, list_delegates, get_delegate_by_address, list_governance_registries, get_governance_registry_by_type, get_governance_registry_by_platform_id
3. **GraphQL Types**: Proposal (proposalId, registryType, status, votes), ProposalVotes, Delegate, GovernanceRegistry, GovernanceRegistryConfig
4. **GraphQL Queries**: proposals(platform_id, status), proposal(id), delegates(registry_type), delegate(address), governance_registries(), governance_registry(platform_id)
5. **Social Server**: platform_id in GovernanceProposalQuery; list_proposals(platform_id); get_governance_registry_by_platform_id; GET /governance/registries/platform/:platform_id
6. **Migration**: reader/types/governance.rs now re-exports from myso_indexer_alt_social_schema::models

## Verification
- cargo check -p myso-indexer-alt-social-schema -p myso-indexer-alt-social-reader -p myso-indexer-alt-graphql -p myso-social-server: passes
- INSTA_UPDATE=always cargo nextest run -p myso-indexer-alt-graphql -- test_schema_sdl_export: passes

---

# GraphQL Post Enrichment – Completed (Mar 2026)

## Summary
Implemented full plan: Post row expansion (media_urls, mentions, parent_post_id, updated_at); nested resolvers for comments, reactions, reposts, tips, transfers.

## Phases Completed
1. **Post Row**: Extended PostRow with media_urls, mentions, parent_post_id, updated_at; GraphQL mediaUrls, mentions, parentPostId, updatedAt
2. **Reader**: CommentRow, ReactionRow, RepostRow, TipRow, PostTransferRow; get_post_comments, get_post_reactions, get_post_reposts, get_post_tips, get_post_transfers
3. **GraphQL types**: CommentSummary, ReactionSummary, RepostSummary, TipSummary, PostTransferSummary
4. **Post resolvers**: comments, reactions, reposts, tips, transfers (all paginated)
5. **Schema**: Regenerated schema.graphql and staging.graphql

## Verification
- cargo check -p myso-indexer-alt-social-reader -p myso-indexer-alt-graphql: passes
- INSTA_UPDATE=always cargo nextest run -p myso-indexer-alt-graphql -- test_schema_sdl_export: passes (prod and staging)

---

# GraphQL Social Schema Enrichment – Completed (Mar 2026)

## Summary
Implemented full plan: platform enrichment, profile badges, social graph (followers/following), blocked profiles/platforms, schema wiring.

## Phases Completed
1. **Platform**: Extended PlatformRow with terms_of_service, privacy_policy, links, platform_names, release_date, shutdown_date, treasury, governance fields; added blockedProfiles resolver
2. **Profile badges**: ProfileBadgeRow, get_profile_badges in reader; ProfileBadge type, Profile.badges resolver
3. **Social graph**: ProfileSummaryRow, get_followers/get_following in reader; ProfileSummary type, Profile.followers/following resolvers
4. **Blocked**: BlockedProfileRow, BlockedPlatformRow, PlatformBlockedProfileRow; get_blocked_profiles, get_blocked_platforms, get_platform_blocked_profiles, check_profile_blocked, check_platform_blocked in reader; BlockedProfileSummary, BlockedPlatformSummary, PlatformBlockedProfileSummary types; Profile.blockedProfiles, Profile.blockedPlatforms, Platform.blockedProfiles; Query.checkProfileBlocked, Query.checkPlatformBlocked
5. **Schema**: Regenerated schema.graphql and staging.graphql via test_schema_sdl_export

## Verification
- cargo check -p myso-indexer-alt-social-reader -p myso-indexer-alt-graphql: passes
- cargo clippy on modified crates: passes (pre-existing warnings elsewhere)
- INSTA_UPDATE=always cargo nextest run -p myso-indexer-alt-graphql -- test_schema_sdl_export: passes (both prod and staging)

---

# Social Schema Models – mys-indexer Alignment (Mar 2026)

## Summary
Added Queryable structs, AsChangeset, Insertable, and constants from mys-indexer to myso-indexer-alt-social-schema for: treasury, subscription, governance, insurance, social_graph, wallet_social_graph.

## Completed
- **spt.rs**: `EcosystemTreasury` (Queryable)
- **subscription.rs**: `ProfileSubscriptionService`, `ProfileSubscription`, `SubscriptionEvent`, `SubscriptionRevenue`, `SubscriptionAccessLog` (Queryable); `UpdateProfileSubscriptionService`, `UpdateProfileSubscription` (AsChangeset); `NewSubscriptionAccessLog` (Insertable); constants: MIN/MAX_SUBSCRIPTION_DURATION_DAYS, MILLISECONDS_PER_DAY, REVENUE_TYPE_*, CONTENT_TYPE_PROFILE
- **governance.rs**: `GovernanceRegistry`, `Delegate`, `NominatedDelegate`, `Proposal`, `DelegateRating`, `DelegateVote`, `CommunityVote`, `RewardDistribution`, `GovernanceEvent`, `AnonymousVote`, `VoteDecryptionFailure` (Queryable)
- **insurance.rs**: `InsuranceConfig`, `InsuranceVault`, `InsurancePolicy` (Queryable); `UpdateInsuranceVault`, `UpdateInsurancePolicy` (AsChangeset)
- **social_graph.rs**: `SocialGraphRelationship`, `SocialGraphEvent` (Queryable)
- **wallet_social_graph.rs** (new module): `WalletSocialGraph` (Queryable), `NewWalletSocialGraph` (Insertable)

## Note
- **universal_user.rs**: API DTOs (UniversalUserResult, SocialProofTokenInfo, etc.) remain in social server per plan.

---

# Social Schema & Indexer Enrichment – Completed (Mar 2026)

## Summary
Implemented the full plan (excluding relay): posts_transfers, profile_offers, profile_sale_fees population; reader + endpoints; Profile enrichment.

## Phase 1: Indexer Fixes
- **posts_transfers**: NewPostTransfer model, OwnershipTransferEvent handler emits PostTransfer, commit inserts into posts_transfers
- **profile_offers**: NewProfileOffer model, ProfileOfferCreatedEvent/AcceptedEvent/RejectedEvent handlers, commit insert/update
- **profile_sale_fees**: NewProfileSaleFee model, ProfileSaleFeeEvent handler, commit insert

## Phase 2: Reader & Endpoints
- **Profile offers**: `list_profile_offers`, `get_profile_offers` handler, GET `/profiles/:address/offers`
- **Profile sale fees**: `list_profile_sale_fees`, `get_profile_sale_fees` handler, GET `/profiles/:address/sale-fees`
- **Post transfers**: `list_post_transfers`, `get_post_transfers` handler, GET `/posts/:id/transfers`

## Phase 3: Enrichment
- **ProfileByAddressResponse**: Non-X social username fields removed from contract, DB, reader, and server; only `x_username` remains for external social handle (plus core `username`).

## Verification
- `cargo check -p myso-social-server` passes
- `./scripts/lint.sh` fails on pre-existing myso-pg-db clippy errors (collapsible_if)

---

# Validator Staking UI – My Stake Column (Mar 2026)

## Request
When the user has zero stake in a validator, hide the bottom green "+X.XX MySo" (rewards) amount in the My Stake column.

## Status
Validator staking UI not found in myso-core workspace. Likely in a separate frontend app (wallet, explorer, staking dApp).

## Implementation Pattern (for frontend)
```tsx
// Only render the green rewards line when user has non-zero stake
{stakeAmount > 0 && (
  <div className="text-xs font-chakra-petch font-medium text-[var(--highlight)] mt-1">
    +{formatRewards(rewards)} MySo
  </div>
)}
```
Or: `{(stakeAmount > 0 || rewards > 0) && (...)}` if you want to show rewards even when principal is 0 but rewards exist.

---

# Treasury /treasury/current Endpoint Fix – Completed (Mar 2026)

## Problem
`GET /treasury/current` returned 404 even though `ecosystem_treasury` had data. Both social server and indexer use the same DB; the query was returning 0 rows (likely Diesel Queryable/type mismatch or connection config).

## Fix Implemented
1. **Raw SQL for get_current_treasury** (`reader/revenue.rs`): Replaced Diesel query builder with `diesel::sql_query()` and `QueryableByName` + `TreasuryRow` struct to avoid deserialization issues.
2. **Return 200 with null when no rows** (`handlers/revenue.rs`): Replaced `.ok_or_else(|| SocialError::not_found(...))` with `.unwrap_or(serde_json::Value::Null)` so the endpoint returns 200 with `null` when the table is empty instead of 404.

## Verification
- `crates/myso-social-server` changes compile; full workspace build blocked by pre-existing errors in `myso-indexer-alt-social-schema` (mydata_id, platform_names, spt_reservations fee columns, etc.).
- After fix: `GET /treasury/current` returns 200 with treasury data when rows exist, or `null` when none.

---

# Faucet WALLET_MNEMONIC Fallback Plan (Mar 2026)

## Background and Motivation

The faucet currently requires `WALLET_PRIVATE_KEY` to be set. When only `WALLET_MNEMONIC` is provided (e.g. from a wallet backup or hardware wallet export), the setup fails. The user wants the faucet to **fall back to `WALLET_MNEMONIC`** when `WALLET_PRIVATE_KEY` is not set.

**Current behavior:**
- `create_wallet_context()` in `server.rs` loads wallet from config file (`client.yaml` + `myso.keystore`) only
- `setup-wallet.sh` (Railway/Docker) creates those files from env vars; it **exits with error** when only `WALLET_MNEMONIC` is set ("Mnemonic derivation not yet implemented in this script")

**Desired behavior:**
- Priority: `WALLET_PRIVATE_KEY` first, then `WALLET_MNEMONIC` as fallback
- When either is set, the faucet should derive/load the key and create a working `WalletContext` without depending on the setup script

## Key Challenges and Analysis

1. **Where to implement**: Two options:
   - **Option A (Rust)**: Modify `create_wallet_context` in `server.rs` to check env vars and build keystore from them. Works regardless of deployment (with or without setup script). Single source of truth.
   - **Option B (Shell)**: Implement mnemonic derivation in `setup-wallet.sh` using `myso keytool import` or similar. Requires `myso` binary in the Docker image; script stays complex.

   **Recommendation: Option A** – Implement in Rust. The `myso-keys` crate already has `derive_key_pair_from_path` and `import_from_mnemonic`; we can reuse this logic. The faucet becomes self-contained.

2. **Async vs sync**: `AccountKeystore::import` is async. `create_wallet_context` is currently sync. The caller (`init_faucet_with_retry`) is async, so we can make `create_wallet_context` async.

3. **Config source when using env wallet**: When building from env vars, we need network config (RPC URL, etc.). Options:
   - Load config file for network, replace keystore with env-derived one
   - Build full config from env vars (`NETWORK_URL`, `NETWORK_ALIAS`, `WALLET_ADDRESS` optional—we derive address from key)
   - **Recommendation**: Try config file first for network; if it exists, use its `envs` and replace keystore. If config doesn't exist, build from `NETWORK_URL` (default: testnet), `NETWORK_ALIAS` (default: "testnet").

## High-Level Task Breakdown

### Task 1: Make `create_wallet_context` async
- **File**: `crates/myso-faucet/src/server.rs`
- **Change**: `pub fn create_wallet_context(...)` → `pub async fn create_wallet_context(...)`
- **File**: `crates/myso-faucet/src/main.rs`
- **Change**: `create_wallet_context(...)?` → `create_wallet_context(...).await?`

### Task 2: Add env-based wallet initialization in `create_wallet_context`
- **File**: `crates/myso-faucet/src/server.rs`
- **Logic**:
  1. Check `WALLET_PRIVATE_KEY` env var. If set: decode base64 → `MySoKeyPair`, create `InMemKeystore`, import key.
  2. Else check `WALLET_MNEMONIC` env var. If set: use `derive_key_pair_from_path(seed, None, ED25519)` from `myso_keys::key_derive`, create `InMemKeystore`, import key.
  3. If neither set: fall through to current behavior (`WalletContext::new(&wallet_conf)`).
  4. When env-based: create `WalletContext` via `WalletContext::new_for_tests(keystore, None, Some(config_path))`, then populate `config.envs`, `config.active_env`, `config.active_address` from config file (if exists) or from env vars (`NETWORK_URL`, `NETWORK_ALIAS`).

### Task 3: Implement env-to-keystore logic
- **WALLET_PRIVATE_KEY**: `MySoKeyPair::decode_base64(&key)` (from `myso_types::crypto::EncodeDecodeBase64`)
- **WALLET_MNEMONIC**: `Mnemonic::from_phrase(phrase, Language::English)` → `Seed::new(&mnemonic, "")` → `derive_key_pair_from_path(seed.as_bytes(), None, &SignatureScheme::ED25519)`
- **Dependencies**: `myso-keys` (already in Cargo.toml), `bip39` (via myso-keys). May need to add `myso-types` for `EncodeDecodeBase64` if not already accessible—faucet already uses `myso-types`.

### Task 4: Network config when using env wallet
- If `config_dir.join(MYSO_CLIENT_CONFIG).exists()`: load config, replace `config.keystore` with env-derived keystore, set `config.active_address` from keystore's first address (or `WALLET_ADDRESS` if set and matches).
- If config doesn't exist: build `MySoEnv` from `NETWORK_URL` (default: `http://fullnode.testnet.mysocial.network:9000`), `NETWORK_ALIAS` (default: "testnet"), set `active_address` from keystore.

### Task 5: Update `setup-wallet.sh` (optional)
- Remove the "Mnemonic derivation not yet implemented" error block. When only `WALLET_MNEMONIC` is set, the script can either:
  - Skip keystore creation and let the faucet handle it (create minimal `client.yaml` for network only), or
  - Keep the current error for now—the Rust fix will allow the faucet to work even if the script fails, as long as `client.yaml` exists for network config. Actually, if the script exits when only mnemonic is set, the faucet never runs. So we need the script to NOT exit. Options:
    - Script creates `client.yaml` with network config but no keystore file when only mnemonic is set; faucet will use env vars for the key.
    - Or: script could run the faucet with env vars passed through, and the faucet handles everything. The script would need to create `client.yaml` for network. Simplest: when only `WALLET_MNEMONIC`, create `client.yaml` (network config) and an empty or placeholder keystore path—but `WalletContext::new` would fail on invalid keystore. So we must have the Rust code check env vars *before* trying to load from file. If env vars are set, we never call `WalletContext::new` with a broken keystore. The script could create a dummy `client.yaml` with a File keystore path that doesn't exist—`WalletContext::new` would fail when it tries to load the keystore. So the fix must be: **Rust checks env vars first**. If `WALLET_PRIVATE_KEY` or `WALLET_MNEMONIC` is set, we build the wallet from env and never touch the file-based keystore. For network config, we still need `client.yaml` or we build from env. When the script exits with error on mnemonic-only, it never creates `client.yaml`. So we need the script to NOT exit—create `client.yaml` for network, and either create a dummy keystore or leave it out. The Rust code will use env for the key and can build config from env for network if `client.yaml` doesn't exist. So the script should: when only `WALLET_MNEMONIC`, create `client.yaml` (network config) and **not** create myso.keystore (or create empty). Then the faucet will fail on `WalletContext::new` because the keystore file is missing or empty. Unless—we check env vars **before** loading. So the flow is: 1) Check WALLET_PRIVATE_KEY, 2) Check WALLET_MNEMONIC, 3) If either, build from env (and we need network—from config file or from env). The config file might not exist if the script exited. So we need to support building network config from env when using env-based wallet. The script, when only mnemonic: we could change it to create `client.yaml` with network config and a dummy/empty keystore path. `WalletContext::new` would try to load that and fail. So we must **not** use `WalletContext::new` when env vars are set. We use the env-based path which builds everything from env (including network from NETWORK_URL). So the script, when only mnemonic: we can change it to **not** exit, and instead create `client.yaml` with network config and a placeholder keystore. But `WalletContext::new` would fail. The solution: **always check env vars first in create_wallet_context**. If set, we never call `WalletContext::new`. We build WalletContext from env. For network, we use NETWORK_URL, NETWORK_ALIAS from env (with defaults). The script can then: when only WALLET_MNEMONIC, create `client.yaml` for network (for consistency) but the faucet will use env for the key. Actually the faucet gets config_dir from `myso_config_dir()`. So the config path is `config_dir.join(MYSO_CLIENT_CONFIG)`. If we use env-based wallet, we might still want to read network from that file if it exists. So: 1) If WALLET_PRIVATE_KEY or WALLET_MNEMONIC set: build keystore from env. 2) Try to load config from file for network. If successful, use its envs, replace keystore, set active_address. 3) If config load fails (file missing): build envs from NETWORK_URL, NETWORK_ALIAS. 4) Create WalletContext with that config. So we never call `WalletContext::new` when env wallet is used—we build the config ourselves and use `new_for_tests` or a similar path. Looking at the code again: `WalletContext::new_for_tests` creates a PersistedConfig. We need to add envs to it. The `config` in WalletContext is `PersistedConfig<MySoClientConfig>`. We can mutate it via deref. So we'd need to add a way to create WalletContext with a custom config. The `new_for_tests` creates config with empty envs. We need to add envs. We can do: `let mut ctx = WalletContext::new_for_tests(keystore, None, Some(path)); ctx.config.add_env(...); ctx.config.active_env = Some(...); ctx.config.active_address = Some(addr);` The path for persisted could be a temp path or the config_dir path. Good.

### Task 6: Error handling
- If `WALLET_MNEMONIC` is set but invalid (bad phrase): return clear error
- If `WALLET_PRIVATE_KEY` is set but invalid base64: return clear error
- Ensure we don't log or expose secrets

### Task 7: Update README
- Document that `WALLET_MNEMONIC` is a fallback when `WALLET_PRIVATE_KEY` is not set
- Document `NETWORK_URL`, `NETWORK_ALIAS` for env-based wallet mode

## Project Status Board

- [ ] Task 1: Make create_wallet_context async
- [ ] Task 2: Add env-based wallet initialization in create_wallet_context
- [ ] Task 3: Implement env-to-keystore logic (private key + mnemonic)
- [ ] Task 4: Network config when using env wallet
- [ ] Task 5: Update setup-wallet.sh to support mnemonic-only (create client.yaml, skip keystore)
- [ ] Task 6: Error handling and logging
- [ ] Task 7: Update README

## Success Criteria

- With `WALLET_PRIVATE_KEY` set: faucet works (unchanged behavior)
- With only `WALLET_MNEMONIC` set: faucet derives key and works
- With neither set: faucet uses config file (unchanged behavior)
- No secrets in logs
- `setup-wallet.sh` allows mnemonic-only deployment (no exit)

## Key Files

- `crates/myso-faucet/src/server.rs` – `create_wallet_context`
- `crates/myso-faucet/src/main.rs` – caller
- `crates/myso-faucet/setup-wallet.sh` – optional script update
- `crates/myso-faucet/README.md` – docs
- `crates/myso-keys/src/key_derive.rs` – `derive_key_pair_from_path`
- `crates/myso-keys/src/keystore.rs` – `InMemKeystore`, `AccountKeystore::import`

---

# Social Indexer – Zero Checkpoints / FIRST_CHECKPOINT (Mar 2026)

## Symptom
Grafana shows 0 for Latest Collected/Ingested/Watermark checkpoints. Throughput drops to 0 after initial spike.

## Root cause
Indexer resumes from watermark (or 0 if none). If watermark is very old or fullnode buffer doesn't have those checkpoints, fetches fail (NotFound) and no progress.

## Fix
1. **Add FIRST_CHECKPOINT** – Entrypoint now passes `--first-checkpoint $FIRST_CHECKPOINT` when set.
2. **Set near chain head** – e.g. `FIRST_CHECKPOINT=2000000` (update as chain grows).
3. **Reset watermark if needed** – If pipeline has old watermark, it ignores FIRST_CHECKPOINT. Run:
   ```sql
   DELETE FROM watermarks WHERE pipeline = 'social_events';
   ```
   Then restart with FIRST_CHECKPOINT set.

## Files changed
- `Dockerfile`: Add FIRST_CHECKPOINT to entrypoint
- `railway.toml`: Document FIRST_CHECKPOINT and watermark reset
- `docker-compose.yml`: Comment for FIRST_CHECKPOINT

---

# Platform Governance + Config Fix – Completed (Mar 2026)

## Summary

### Platform API (code changes kept)
Added governance fields to `PlatformRow` and platform reader so the API returns `wants_dao_governance`, `governance_registry_id`, and governance parameters. Files: `reader/types/platform.rs`, `reader/platform.rs`.

### Migration removed
User preferred no new migrations; the previous social indexer handled governance without that migration. The governance_registries unique constraint issue remains (registry_type blocks multiple platform registries); consider addressing via existing schema or future migration if needed.

### SPT / social_proof_tokens config deduplication (code fix)
**Root cause**: `spt_exchange_config` and `social_proof_tokens_config` used plain INSERT on every config update event, producing 500k+ duplicate rows.

**Fix**: Handler logic now updates the latest row instead of inserting when a row exists:
- **spt_exchange_config**: Select latest row by `time DESC`; if found, UPDATE that row; else INSERT.
- **social_proof_tokens_config**: Select max(id); if found, UPDATE that row; else INSERT.

No migrations. Stops new duplicates; existing rows are unchanged.

---

# Configuration Tables Fix – Completed (Mar 2026)

## Summary
Implemented the Configuration Tables Fix plan for myso-indexer-alt-social. All config events (SPT, PoC, MyData, Insurance, Spot, Platform) are now correctly processed with diagnostic logging.

## Completed Work

### Phase 0: Missing Table
- **Migration**: `20260303000000_create_social_proof_tokens_events` creates `social_proof_tokens_events` table (event_type, event_data, event_id, created_at) with indexes
- **Down migration**: Drops table for rollback
- Root cause: SPT handler expected `social_proof_tokens_events` but only `token_exchange_events` existed from prior migration

### Phase 1: Logging
- **handlers/mod.rs**: Added `info!` when config rows are written: SocialProofTokensConfig, SptExchangeConfig, PocConfiguration, MyDataConfig, InsuranceConfig, SpotConfig, PlatformUpdate, SocialProofTokensEvent
- **handlers/events.rs**: Added `warn!` when config event BCS parsing fails (module, event_name, error) for: PoCConfigUpdatedEvent, MyDataConfigUpdatedEvent, insurance ConfigUpdatedEvent, SpotConfigUpdatedEvent, SPT ConfigUpdatedEvent, PlatformUpdatedEvent

### Phase 1: BCS Verification
- Verified all Bcs*ConfigUpdatedEvent structs match Move event layouts: PoC, MyData, Insurance, Spot, SPT, Platform. No mismatches found.

### Phase 2: Fixes
- No BCS/JSON mismatches or commit logic fixes required. All structs and handlers are correct.

## Verification
- `cargo check -p myso-indexer-alt-social` passes
- Migration runs on indexer startup via `store.run_migrations(Some(&MIGRATIONS))`

---

# Social Server Refactor – Completed (Mar 2026)

## Summary
Split 8000-line reader.rs and 2500-line server.rs into domain modules per the refactoring plan.

### Reader (was ~8000 lines, now ~1543 in reader.rs + domain modules)
- **reader/types/** – DTOs: common, mydata, insurance, spot, spt, governance, revenue, subscription, upgrade, post, platform, social_graph, poc, vesting
- **reader/** domain modules: profile, mydata, insurance, spot, governance, spt, revenue, subscription, upgrade, platform, post, promotion, poc, vesting, social_graph, system, search
- **reader.rs** – Reader struct, delegating impl, `pub use types::*`

### Server (was ~2500 lines, now ~581 in server/mod.rs + handlers)
- **server/handlers/** – health, system, profiles, social_graph, platforms, posts, promotions, poc, subscription, vesting, mydata, insurance, spot, spt, governance, revenue, upgrade, search
- **server/mod.rs** – AppState, query structs, make_router, start_server, run_server

### Verification
- `cargo check -p myso-social-server` – passes
- `cargo clippy -p myso-social-server` – passes (6 warnings)

---

# Social Indexer Fix – Project Status

## Analytics Indexer ClickHouse – Objects/Events/MoveCalls/BalanceChanges Not Showing (Mar 2026)

### Background
User reports: ClickHouse mode shows **transactions** but NOT objects, move calls, events, or balance changes. Only transactions table is populated.

### Architecture Summary

**Data flow:**
1. **Ingestion**: Indexer fetches checkpoints via (a) gRPC streaming `SubscribeCheckpoints` or (b) gRPC `GetCheckpoint` (fallback/historical)
2. **Processors**: Each pipeline (Transaction, Event, MoveCall, Object, BalanceChange) receives `Arc<Checkpoint>` and extracts rows
3. **Store**: ClickHouse mode uses direct HTTP insert (no Parquet serialization) via `uploader.rs`

**Data dependencies per pipeline:**

| Pipeline       | Data source                                      | Requires object_set? |
|----------------|--------------------------------------------------|----------------------|
| Transaction    | `checkpoint.transactions`, `checkpoint.contents` | No                   |
| Event          | `executed_tx.events`                             | No                   |
| MoveCall       | `executed_tx.transaction.move_calls()`          | No                   |
| Object         | `checkpoint_transaction.output_objects(&checkpoint.object_set)` | **Yes** |
| BalanceChange  | `transaction.input_objects(&object_set)`, `output_objects(&object_set)` | **Yes** |

### Root Cause Hypotheses

1. **Fullnode not returning object_set (most likely)**
   - `Checkpoint` has `object_set: ObjectSet` – required for Object and BalanceChange
   - **Streaming**: `SubscribeCheckpoints` uses `Checkpoint::merge_from(checkpoint, &read_mask)`. Client sends `proto_field_mask()` which includes `objects.objects.bcs`. The subscription service receives checkpoints from the checkpoint executor. Checkpoint executor only produces full checkpoint data when `checkpoint_data_enabled()` = `rpc_index.is_some() || data_ingestion_dir.is_some()`. If the testnet fullnode runs without RPC index or data ingestion, it may not produce full checkpoints for subscription.
   - **GetCheckpoint**: RPC client uses explicit mask with `objects.objects.bcs`. Standard fullnode (`myso-rpc-api`) loads via `get_checkpoint_data()` from authority store. KV-RPC fullnode (`myso-kv-rpc`) fetches from `checkpoint_bucket` (remote object store) when objects requested – requires blob indexer to have written full checkpoints.
   - **Action**: Verify fullnode returns objects. Add instrumentation to log `checkpoint.object_set.len()` when processors run.

2. **Data characteristics**
   - Checkpoint range 2587781+ might have mostly system/consensus transactions with no events, no move calls, no balance changes. Transactions always exist (1 per checkpoint minimum).
   - **Action**: Log `checkpoint.transactions.len()`, `tx.events`, `tx.transaction.move_calls().len()` per checkpoint.

3. **Pipeline ordering / watermark**
   - All pipelines run in parallel; watermarks are per-pipeline. Unlikely to cause selective failure.
   - **Action**: Check if Event/MoveCall produce rows but ClickHouse block conversion fails (empty block warning in uploader.rs:373-381).

4. **Schema / block conversion**
   - `object_rows_to_block` expects columns like `owner_type`, `owner_address`. ObjectRow uses `Option<OwnerType>`, `Option<String>`. Schema and block conversion appear correct.
   - **Action**: If processors produce rows but blocks are empty, the `col_idx`/`get_str` lookup may fail for a mismatched schema.

5. **Streaming vs ingestion source**
   - When `streaming_url` is set, broadcaster uses streaming for live tail and ingestion (GetCheckpoint) for historical. Config has `first_checkpoint: 2587781` – historical. So ingestion (GetCheckpoint) is used for that range.
   - If fullnode's GetCheckpoint doesn't return objects (e.g. KV-RPC without checkpoint_bucket, or bucket has blobs without objects), Object/BalanceChange get empty object_set.

### Recommended Debug Steps

1. **Add instrumentation** (see debug_mode_logging) to log at processor level:
   - `checkpoint.object_set.len()` (Object, BalanceChange)
   - `checkpoint.transactions.iter().filter(|t| t.events.is_some()).count()` (Event)
   - `checkpoint.transactions.iter().filter(|t| !t.transaction.move_calls().is_empty()).count()` (MoveCall)
   - Rows produced per pipeline per checkpoint

2. **Verify fullnode capability**:
   - `grpcurl -plaintext -d '{"sequence_number":"2587781","read_mask":{"paths":["objects.objects.bcs","transactions"]}}' fullnode.testnet.mysocial.network:9000 myso.rpc.v2.LedgerService/GetCheckpoint`
   - Inspect response for `objects` field presence and size.

3. **Try without streaming** (force ingestion only):
   - Remove `streaming_url` from config; set `remote_store_url` or rely on `rpc_api_url` for GetCheckpoint.
   - Compare behavior – if ingestion returns full data but streaming doesn't, the fullnode's subscription path may not produce full checkpoints.

4. **Check ClickHouse tables**:
   - `SELECT count() FROM default.events`, `default.move_calls`, `default.objects`, `default.balance_changes`
   - Confirm they are 0 vs transactions having rows.

### Key Files
- `crates/myso-analytics-indexer/src/handlers/tables/{object,event,move_call,balance_change}.rs` – processors
- `crates/myso-analytics-indexer/src/store/uploader.rs` – ClickHouse direct insert, empty-block warn for Event/MoveCall
- `crates/myso-types/src/full_checkpoint_content.rs` – Checkpoint struct, proto_field_mask
- `crates/myso-indexer-alt-framework/src/ingestion/rpc_client.rs` – GetCheckpoint read mask
- `crates/myso-rpc-api/src/grpc/v2/subscription_service.rs` – SubscribeCheckpoints merge
- `crates/myso-core/src/checkpoints/checkpoint_executor/mod.rs` – checkpoint_data_enabled, full checkpoint production

---

## Analytics Indexer Full Pipeline Coverage (Mar 2026)

### Completed
1. **GCS mode**: Default pipelines `Checkpoint,Transaction,Event,Object,MoveCall` in entrypoint.sh
2. **ClickHouse mode**: Default pipelines `Transaction,Event,MoveCall,Object,BalanceChange`; `objects` and `balance_changes` tables in indexer.rs
3. **Store**: `pipeline_output_prefix_to_table` maps Object→objects, BalanceChange→balance_changes; uploader direct-insert for both
4. **ClickHouse block converters**: `object_rows_to_block`, `balance_change_rows_to_block` in clickhouse.rs
5. **BalanceChange pipeline**: New enum variant, BalanceChangeRow, BalanceChangeProcessor (address_balance_changes_from_accumulator_events + Coin::extract_balance_if_coin)
6. **Config**: config-clickhouse.yaml, config-clickhouse.yaml.example add Object and BalanceChange pipelines
7. **Deployment**: railway.toml and docker-compose.yml use full GCS pipelines
8. **Clippy fixes**: useless_conversion (clickhouse.rs), collapsible_if (store/mod.rs)

### Verification
- `cargo check -p myso-analytics-indexer` passes
- `cargo nextest run -p myso-analytics-indexer --lib` — 20 tests pass
- `cargo clippy -p myso-analytics-indexer --lib` passes (pre-existing warnings in myso-pg-db)

---

## Analytics Indexer – ClickHouse Output (Mar 2026)

### Background
Analytics indexer was extended to write to ClickHouse instead of GCS. The native protocol insert (`clickhouse-native-client`) blocked indefinitely; inserts never completed.

### Fix (applied)
- **HTTP insert**: Replaced native `client.insert()` with HTTP POST to port 8123 using JSONEachRow format. Native client retained for execute (CREATE TABLE) and query (watermark).
- **Port mapping**: When native port is 9000, HTTP uses 8123 (ClickHouse default).
- **store/clickhouse.rs**: `insert_http()` builds JSON lines from Block and POSTs to `http://host:8123/?query=INSERT INTO transactions FORMAT JSONEachRow`.

### Single command to run
```bash
# 1. Start ClickHouse (if not running)
clickhouse server -- --path="$HOME/clickhouse-data" &

# 2. Run analytics indexer with ClickHouse
cargo run -p myso-analytics-indexer -- crates/myso-analytics-indexer/config-clickhouse.yaml
```

Or with env vars via entrypoint:
```bash
CLICKHOUSE_HOST=localhost CLICKHOUSE_PORT=9000 RPC_API_URL=... STREAMING_URL=... sh crates/myso-analytics-indexer/entrypoint.sh
```

### Verification
- Logs show "Inserted into ClickHouse" with row counts.
- `curl -s "http://localhost:8123/?query=SELECT count() FROM transactions"` shows increasing count.

---

## ClickHouse Indexer – Connection Recovery (Mar 2026)

### Background
Indexer hit "operation timeout" (60s) then cascaded into "Broken pipe" errors. The native bridge used a single connection; when it timed out or the server closed it, all subsequent inserts failed.

### Fix (applied)
- **native_bridge.rs**: After each execute/query/insert, if the client returns a recoverable error (Connection, Io BrokenPipe/ConnectionReset/ConnectionAborted/TimedOut), reconnect before processing the next request.
- **README**: Added "Operation timeout / Broken pipe" troubleshooting with data path, batch size, and ClickHouse config tips.

### Verification
Run indexer; after timeout or Broken pipe, it should reconnect and continue. Check logs for "ClickHouse native client reconnected after connection error".

---

## ClickHouse Indexer – Throughput Improvements (Mar 2026)

### Background
Indexer could not keep up with ~40 tx/sec chain throughput. Root causes: MIN_EAGER_ROWS=10000 (250 sec to fill one batch), no streaming docs, data in project dir.

### Changes Implemented
1. **handlers.rs**: MIN_EAGER_ROWS 10000→500, MAX_PENDING_ROWS 20000 (was default 5000). Fixes comment to match value.
2. **main.rs**: collect_interval_ms 250→100 for faster timer-based flushes.
3. **README**: gRPC streaming section (--streaming-url, STREAMING_URL); recommend --path for ClickHouse data; start script env overrides.
4. **scripts/start-local.sh**: CLICKHOUSE_DATA_PATH support; STREAMING_URL support for indexer args.

### Usage
- `--streaming-url http://fullnode:9000` for live sync (already in ClientArgs)
- `CLICKHOUSE_DATA_PATH=/var/lib/clickhouse` when starting ClickHouse
- `STREAMING_URL=... ./scripts/start-local.sh` for streaming in local dev

---

## ClickHouse Indexer – Local Dev: Three Services + Start/Stop (Mar 2026)

### Background
User runs ClickHouse, indexer (Cargo), and ch-ui locally. Connections were failing because ClickHouse was not running when ch-ui started. Needed clear startup/shutdown order, port reference, and deploy docs.

### Changes
1. **README**: Added "Local Development: Three Services" section with port table, startup order (1. ClickHouse, 2. Indexer, 3. ch-ui), shutdown order, verification step, optional scripts.
2. **scripts/start-local.sh**: Starts native ClickHouse, waits for readiness, starts indexer and ch-ui. Requires `clickhouse` and `ch-ui` binaries.
3. **scripts/stop-local.sh**: Kills PIDs from .local-dev.pids, pkill clickhouse.
4. **.gitignore**: Added .local-dev.pids.
5. **Production Deployment**: Added deploy order (same as local).

### Usage
```bash
cd examples/rust/clickhouse-myso-indexer
./scripts/start-local.sh   # or follow manual steps in README
./scripts/stop-local.sh
```

---

## ClickHouse Indexer – Reset for Clean Watermarks (Mar 2025)

### Background
Watermarks from a previous run (e.g. Docker at 6M checkpoints) can persist when switching to native ClickHouse or changing checkpoint ranges. The indexer then skips ingestion because it thinks it has already processed those checkpoints.

### Fix (applied)
Added `reset` subcommand to clickhouse-myso-indexer:
- `cargo run -- reset` – drops watermarks and transactions tables
- Uses same ClickHouse connection as indexer (no external clickhouse-client needed)
- Run from `examples/rust/clickhouse-myso-indexer` with `CLICKHOUSE_USER=default`

### Usage
1. Ensure ClickHouse is running (`clickhouse server` for native)
2. `CLICKHOUSE_USER=default cargo run -- reset`
3. `CLICKHOUSE_USER=default cargo run -- run --remote-store-url ... --first-checkpoint=2587781`

---

## Social Indexer – exec: myso-social-indexer: not found (Mar 2025)

### Root Cause
Railway service failed at startup with `exec: myso-social-indexer: not found`. The `startCommand` in railway.toml was overriding the Dockerfile ENTRYPOINT and could cause Railway to use a different execution path. Nixpacks-related config (`buildCommand`, `NIXPACKS_*`) was also present despite using Dockerfile builder.

### Fix (applied)
1. **railway.toml**: Removed `startCommand`, `buildCommand`, and `[build.env]` (NIXPACKS_*). Rely entirely on Dockerfile for build and ENTRYPOINT.
2. **Dockerfile**: Changed `ENTRYPOINT /usr/local/bin/entrypoint.sh` to `ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]` for proper exec form.

### Railway Service Setup
- **Social Indexer** (background indexer): Config at `crates/myso-indexer-alt-social/railway.toml`. Root Directory must be **repo root** (`.` or empty) so Docker build context includes full workspace.
- **Social Server** (REST API): Config at `crates/myso-social-server/railway.toml`. Different service; do not confuse with the indexer.

---

## Orderbook + Social Server Railway Deployment (Plan)

**Port simplification (per user):** On Railway, use a single `$PORT` per service. No need for different port numbers (9008 vs 9184, etc.). Both Orderbook Server and Social Server will bind their API to `$PORT` in the entrypoint. Metrics (currently on a separate port) will remain internal—Railway exposes only one port per service anyway.

## Orderbook Indexer – Railway Log Rate Limit (Mar 2025)

### Root Cause
Orderbook indexer hits Railway's 500 logs/sec rate limit at startup. With 50+ pipelines (balances, order_fill, order_update, pool_price, margin events, etc.), each pipeline logs 6–8 INFO lines ("Starting pipeline with config", "Starting committer", "Starting collector", "Skipping pruner task", etc.). Burst of 300+ logs in ~1 second triggers rate limit; messages dropped.

### Fix (applied)
Railway `RUST_LOG` env var was not reliably applied. **Framework-level fix**: Downgraded pipeline startup logs from `info!` to `debug!` in `myso-indexer-alt-framework`:
- `concurrent/mod.rs`, `sequential/mod.rs`: "Starting pipeline with config"
- `concurrent/collector.rs`, `sequential/committer.rs`: "Starting committer"
- `processor.rs`: "Starting processor"
- `concurrent/commit_watermark.rs`: "Starting commit watermark task"
- `concurrent/main_reader_lo.rs`: "Not a tasked indexer, skipping main reader lo task"
- `concurrent/reader_watermark.rs`: "Skipping reader watermark task"
- `concurrent/pruner.rs`: "Skipping pruner task", "Starting pruner with config"

With default `RUST_LOG=info`, these no longer emit; framework WARN/ERROR still visible.

### Optional: railway.toml
`RUST_LOG = "info,myso_indexer_alt_orderbook=info"` – redundant for startup fix but useful for deployment defaults.

---

## Bridge Indexer Alt – No Checkpoints / Grafana Zeros / Not-Found Retries (Mar 2025)

### Root Cause
Grafana showed all checkpoint metrics at 0 and high "Not-Found Retries/sec". Two issues:
1. **FIRST_CHECKPOINT too old**: 43917829 is beyond chain head (~1.1M), causing NotFound for every fetch. Analytics indexer works because it uses FIRST_CHECKPOINT near chain head (e.g. 1142000).
2. **Outside buffer**: When FIRST_CHECKPOINT is far from head, broadcaster uses HTTP ingestion. Wrong REMOTE_STORE_URL or missing checkpoint-blob data → 404s → retries.

### Fix
1. **FIRST_CHECKPOINT near chain head** (like analytics): Set to ~1140000 so we're within buffer. Streaming then works; no HTTP ingestion needed.
2. **rpc_api_url for ingestion fallback**: When streaming is set, use gRPC GetCheckpoint instead of remote_store for any fallback path.

### Changes
- `main.rs`: When streaming_url is set, pass rpc_api_url to IngestionClientArgs instead of remote_store_url.
- `railway.toml`: FIRST_CHECKPOINT = "1140000" (update as chain grows). For genesis backfill (439178) use checkpoint-blob-indexer + REMOTE_STORE_URL separately.
- `docker-compose.yml`: Same FIRST_CHECKPOINT default.

### Verification
Ensure STREAMING_URL and FIRST_CHECKPOINT (near head) are set. Checkpoint metrics should advance; Not-Found retries should drop to 0.

---

## Bridge Indexer Alt – Cargo Not Found Fix (Feb 2025)

### Root Cause
Container failed to start with "The executable `cargo` could not be found." Railway was inferring a Rust start command (e.g. `cargo run`) that overrides the Dockerfile ENTRYPOINT. The runtime image is debian:bullseye-slim and does not include cargo.

### Fix
Added explicit `startCommand = "/usr/local/bin/entrypoint.sh"` to `crates/myso-bridge-indexer-alt/railway.toml` so Railway uses the entrypoint script instead of inferring a cargo-based command.

---

## Bridge Indexer Entrypoint (Feb 2025)

### Root Cause
Original myso-bridge-indexer deploy failed with `Error: EOF while parsing a value` because config.yaml contained only comments/placeholders; binary expects valid YAML.

### Changes Implemented
1. **entrypoint.sh**: Reads env vars (DATABASE_URL, ETH_RPC_URL, MYSO_RPC_URL, ETH_MYSO_BRIDGE_CONTRACT_ADDRESS, etc.), writes valid config.yaml to /myso/config.yaml, execs bridge-indexer --config-path.
2. **config.yaml**: Replaced template with real testnet defaults for local dev and docs.
3. **Dockerfile**: Added entrypoint.sh, ENTRYPOINT, EXPOSE 9184, HEALTHCHECK for /metrics.
4. **railway.toml**: Removed startCommand; added healthcheckPath, variables for required env vars.

### Deployment
Set ETH_RPC_URL, ETH_WS_URL (or derived from ETH_RPC_URL), MYSO_RPC_URL, ETH_MYSO_BRIDGE_CONTRACT_ADDRESS in Railway. DATABASE_URL from Postgres. Optional: MYSO_BRIDGE_GENESIS_CHECKPOINT, ETH_BRIDGE_GENESIS_BLOCK (defaults in entrypoint).

---

## Social Indexer gRPC Fix (Feb 2025)

### Root Cause
Indexer stuck at checkpoint 0 because HTTP checkpoint store (GCS `mysocial-testnet-checkpoints`) was failing (wrong format, timeouts, 791K not-found retries).

### Changes Implemented
1. **Dockerfile**: Entrypoint now passes `--streaming-url` and `--rpc-api-url` when `STREAMING_URL` and `RPC_API_URL`/`RPC_URL` env vars are set. Uses gRPC fullnode instead of HTTP checkpoint store.
2. **railway.toml**: Added `STREAMING_URL` and `RPC_API_URL` pointing to `https://fullnode.testnet.mysocial.network:9000`; updated `startCommand` to pass these as CLI args.
3. **docker-compose.yml**: Added `STREAMING_URL` and `RPC_API_URL`; removed unused `CHECKPOINT_URL` and `START_CHECKPOINT`; fixed `RUST_LOG` (mys_social_indexer → myso_social_indexer).

### Deployment
Both Railway and docker-compose now use gRPC streaming + ingestion via fullnode instead of GCS checkpoints.

---

## Analytics Indexer gRPC Streaming Fix (Feb 2025)

### Root Cause
Grafana showed all zeros (Latest Streamed/Ingested Checkpoint, throughput). URL `http://fullnode.testnet.mysocial.network:9000` is correct (port 443 fails; grpcurl -plaintext :9000 works).

### Changes Implemented
1. **entrypoint.sh**: When `STREAMING_URL` is set, now adds both `streaming_url` and `remote_store_url` to config. Fallback chain: streaming (primary) → HTTP checkpoint store when streaming fails.
2. **entrypoint.sh**: Added config dump (`cat /app/config.yaml`) before exec for debugging.
3. **entrypoint.sh**: Updated logging to show "REMOTE_STORE_URL (fallback)" when USE_GRPC=true.

### Verification
- docker-compose already has REMOTE_STORE_URL set for both services.
- No URL changes; keep `http://fullnode.testnet.mysocial.network:9000`.

---

## GraphQL Balances FEATURE_UNAVAILABLE Fix (Feb 2025)

### Root Cause
Balances require the **Consistent Store** - a separate gRPC service. The error "fetching balances for addresses not available" occurs when `ConsistentReader` returns `NotConfigured` because `--consistent-store-url` was not passed.

### Changes Implemented
1. **Dockerfile**: Added optional `--consistent-store-url` when `CONSISTENT_STORE_URL` env var is set. Added debug echo for the env var.
2. **railway.toml**: Added `CONSISTENT_STORE_URL` to deploy.envVars (optional).
3. **config/indexer-config.toml**: Added comment explaining that balances require Consistent Store.

### Deployment
To enable balances: deploy myso-indexer-alt-consistent-store, then set `CONSISTENT_STORE_URL` to its gRPC URL (e.g. `http://consistent-store:9124`).

---

## GraphQL Watermark Error Fix (Feb 2025)

### Root Cause
GraphQL server failed with "Indexer not tracking any pipelines" because the Docker entrypoint did not pass `--indexer-config`, leaving `pg_pipelines` empty. The watermark task in `watermark.rs` requires non-empty pipelines.

### Changes Implemented
1. **config/indexer-config.toml**: Minimal indexer config with all indexer-alt pipeline names (coin_balance_buckets, obj_info, sum_displays, kv_*, obj_versions, tx_*, ev_*, cp_sequence_numbers).
2. **Dockerfile**: COPY config into `/app/config/indexer-config.toml`; added `--indexer-config /app/config/indexer-config.toml` to the entrypoint exec command.

### Deployment Note
The database must have watermark rows populated by the indexer before the GraphQL server can succeed. Run the indexer first against the same Postgres instance, or use a shared DB where the indexer has already written watermarks.

---

## SPoT (Social Proof of Truth) Indexer Alt Completion (Feb 2025)

### Completed
1. **BCS event parsing** (`handlers/events.rs`): All 8 SPoT events (SpotBetPlacedEvent, SpotResolvedEvent, SpotDaoRequiredEvent, SpotPayoutEvent, SpotRefundEvent, SpotConfigUpdatedEvent, SpotRecordCreatedEvent, SpotBetWithdrawnEvent) with legacy name support
2. **Schema models** (`myso-indexer-alt-social-schema/models.rs`): NewSpotBet, NewSpotRecord, NewSpotPayout, NewSpotRefund, NewSpotResolution, NewSpotEventLog, NewSpotConfig, NewSpotBetWithdrawal
3. **Epoch/timestamp** (`handlers/mod.rs`): route_event and handle_spot_event receive epoch and timestamp_ms from checkpoint
4. **Spot handler** (`handlers/spot.rs`): Full logic for all 8 events producing SocialEventRow variants
5. **SocialEventRow + commit** (`handlers/mod.rs`): SpotBet, SpotResolution, SpotPayout, SpotRefund, SpotEventLog, SpotConfig, SpotBetWithdrawal, SpotRecordUpsert, SpotRecordUpdate with on_conflict for spot_records
6. **Social server reader** (`reader.rs`): get_spot_record, list_spot_bets, list_spot_payouts, list_spot_refunds, get_spot_configuration
7. **Social server routes** (`server.rs`): GET /spot/records/:post_id, /spot/records/:post_id/bets, /spot/records/:post_id/payouts, /spot/records/:post_id/refunds, /spot/configuration

### Notes
- SPT (social_proof_tokens) variants in commit use catch-all `_ => {}`; SPT commit arms not in plan scope
- spot_config omits max_bets_per_record per plan (schema doesn't have it)

---

## MyData Alt Indexer Implementation (Feb 2025)

### Completed
1. **BCS event parsing** (`handlers/events.rs`): All 6 MyData events parse correctly with legacy name support
2. **Schema models** (`myso-indexer-alt-social-schema/models.rs`): NewMyDataData, NewMyDataPurchase, NewMyDataSubscription, NewMyDataRevenue, NewMyDataAccessLog, NewMyDataRegistry, NewMyDataConfig
3. **SocialEventRow variants + commit** (`handlers/mod.rs`): MyDataData, MyDataPurchase, MyDataSubscription, MyDataRevenue (with owner lookup when to_address empty), MyDataAccessLog, MyDataRegistry, MyDataRegistryUpdate, MyDataConfig, MyDataContentUpdate
4. **Mydata handler** (`handlers/mydata.rs`): Full logic for MyDataCreatedEvent, PurchaseEvent, AccessGrantedEvent, MyDataRegisteredEvent, MyDataUnregisteredEvent, MyDataConfigUpdatedEvent
5. **Social server API** (`myso-social-server`): list_mydata, get_mydata_configuration, get_popular_mydata, get_mydata_by_id, get_mydata_purchases, get_mydata_subscriptions, get_mydata_revenue, get_mydata_access_logs, get_mydata_stats, get_mydata_revenue_timeline, get_mydata_access_analytics, get_creator_mydata

### Notes
- access_type "pricing_update" and "content_update" mapped to "grant" for mydata_access_logs CHECK constraint
- MyDataRevenue from PurchaseEvent: to_address looked up from mydata_data at commit when empty

---

## Background and Motivation

Two issues with the social indexer:
1. Profile creation events were not being reliably indexed to the database (governance worked fine)
2. The `handlers/` directory had a deeply nested structure with redundant `mod.rs` files

## Completed Work

### Task 1: Flatten Module Structure
- **Before**: `handlers/mod.rs` -> `social_events/mod.rs` -> `event_handlers/mod.rs` (3 levels of nesting)
- **After**: All handler files live directly under `handlers/`, single `mod.rs` consolidates routing + pipeline
- Moved: `social_events/event_handlers/{profile,governance,post,...}.rs` -> `handlers/`
- Moved: `social_events/events.rs` -> `handlers/events.rs`
- Deleted: `social_events/mod.rs`, `event_handlers/mod.rs`, `social_events/` and `event_handlers/` dirs
- Updated `lib.rs`: `pub mod handlers` -> `mod handlers; pub use handlers::SocialEvents;`
- Updated `main.rs`: `use social_indexer::handlers::social_events::SocialEvents` -> `use social_indexer::SocialEvents`

### Task 2: Context-Aware BCS Parsing
- **Root cause**: `parse_event_contents` tried every BCS struct sequentially against raw bytes. A `ProfileCreatedEvent` could silently match a different struct's byte layout, returning wrong data or `None`.
- **Fix**: Changed `parse_event_contents(contents)` -> `parse_event_contents(module, event_name, contents)`. Now dispatches to module-specific parsers (`parse_profile_event`, `parse_governance_event`, etc.) that select the correct BCS struct by event name.
- Added `warn!` logging in `mod.rs` when BCS parsing fails (module, event_name, event_id, contents_len)
- JSON fallback preserved for events without dedicated BCS parsers

### Task 3: Profile Event Audit Trail
- **Root cause**: `process_profile_created_event` only emitted `SocialEventRow::Profile` but NOT a `ProfileEvent` audit entry. Badge events did emit audit entries — profiles did not.
- **Fix**: Both `process_profile_created_event` and `process_profile_updated_event` now emit `SocialEventRow::ProfileEvent` audit entries alongside their primary rows
- Added `info!` tracing for profile creation events
- Added `warn!` tracing when profile event JSON deserialization fails

### Task 4: Code Quality
- Removed `#[allow(dead_code)]` on `BcsProfileCreatedEvent.created_at` — changed to `pub(super)` since it's used in the JSON output
- All compilation: `cargo check -p myso-indexer-alt-social` passes with 0 errors, 0 warnings

## Project Status Board

- [x] Flatten module structure (handlers/social_events/event_handlers/ -> handlers/)
- [x] Make parse_event_contents context-aware (module + event_name dispatch)
- [x] Fix profile handler: add ProfileEvent audit trail for creation/update
- [x] Remove #[allow(dead_code)] violation
- [x] Update lib.rs/main.rs imports
- [x] Verify clean compilation

## Lessons

- BCS parsing order matters: sequential trial-and-error can silently match wrong structs
- Context-aware dispatching (module + event_name) eliminates BCS ambiguity entirely
- Audit trail parity: all domain events should emit both their primary row AND a corresponding event log
- Flattening nested module trees improves discoverability without losing encapsulation

## Profile vs Governance – Analysis (Feb 2025)

### Facts
- **Governance works**: Postgres captures governance bootstrap transactions (governance_registries, proposals, etc.).
- **Profile does not**: Profile creation/updates not reaching DB.
- **GraphQL events query returns zero** for both profile AND governance, even though Postgres has governance data.

### GraphQL Events – Different Data Source (confirmed)
- GraphQL events query reads from **`ev_emit_mod`** and **`ev_struct_inst`** tables.
- Those tables are in **myso-indexer-alt-schema** and are populated by the **main indexer-alt** (`EvEmitMod`, `EvStructInst` pipelines).
- The **social indexer** writes to a different schema: `profiles`, `governance_registries`, `profile_events`, etc. (myso-indexer-alt-social-schema).
- **Conclusion**: GraphQL events and social indexer are separate systems. GraphQL events = main indexer DB. Social data = social indexer DB. GraphQL returning zero for governance events is expected if the main indexer (ev_emit_mod/ev_struct_inst) is not populated for 0x50c1, or if GraphQL is pointed at the wrong DB.

### Profile vs Governance – Same Pipeline, Different Handlers
Both use the same SocialEvents pipeline, same route_event, same commit. Differences:
1. **Event name matching**: Governance uses `event_name.strip_suffix("Event")` then matches "GovernanceRegistryCreated", etc. Profile uses exact match: "ProfileCreatedEvent", "ProfileUpdatedEvent".
2. **BCS parsing**: `parse_profile_event` vs `parse_governance_event` – each selects the BCS struct by event name. Profile: BcsProfileCreatedEvent, etc. Governance: BcsGovernanceRegistryCreatedEvent, etc.
3. **Handler output**: Governance produces SocialEventRow::GovernanceRegistry, etc. Profile produces SocialEventRow::Profile, SocialEventRow::ProfileEvent.

### Hypothesis for Profile Failure
If governance reaches the DB but profile does not, the failure is likely before commit:
- **Package filter**: Both pass is_social_package_event (same package 0x50c1).
- **BCS parse**: ProfileCreatedEvent BCS layout may not match Move struct (field order, Option encoding).
- **Handler**: Profile handler JSON deserialization may fail (field name mismatch: profile_picture vs profile_photo, etc.).

### BCS Layout Verification (completed)
- Added `test_parse_profile_created_event_from_live_transaction` with the user's exact event bytes from the successful `create_profile` transaction.
- **Test passes**: BCS parsing works correctly for the live ProfileCreatedEvent format.
- Package filter is correct (0x50c1 == MYSO_SOCIAL_PACKAGE_ID).

### Logging Added
- `info!` when successfully indexing ProfileCreatedEvent (owner_address, username).
- `warn!` when ProfileCreatedEvent JSON deserialization fails.
- Default log level in `myso start` is `"error"` — user must set `RUST_LOG` to see these.

### Next Steps for User
1. **Run with logging**: `RUST_LOG=info cargo run --bin myso -- start --with-faucet --force-regenesis --with-indexer=postgres://... --with-social-indexer --with-graphql`
2. **Create a profile** and watch for `indexing ProfileCreatedEvent` in logs.
3. If no log appears: check for `skipping event: empty contents` (ingestion may not populate event.contents) or `skipping event: failed to parse BCS` (layout mismatch in checkpoint format vs RPC).
4. If `ProfileCreatedEvent JSON deserialization failed` appears: field name mismatch between BCS output and handler.

## Smart Contract Verification (Feb 2025)

### ProfileCreatedEvent – Contract is Correct
- **Move struct** (profile.move:272-281): `profile_id`, `display_name`, `username`, `bio`, `profile_picture`, `cover_photo`, `owner`, `created_at`
- **create_profile** (profile.move:596-604): Emits all fields correctly. Uses `display_name_value` (empty string if None), `profile.username`, `profile.bio`, Option<String> for URLs.
- **Transaction output**: The `events.data[].contents` in the JSON-RPC response is BCS-encoded. Decoded manually: bytes contain "Brandon Shaw", "brandon", "Web8 developer and crypto enthusiast", profile/cover URLs, owner address, created_at=1. **The chain has the data.**

### Why Transaction Output Shows Bytes, Not Strings
- `myso client call` returns raw BCS in `events.data[].contents` as a byte array. The client does not decode Move events for display. This is expected. The data is present; it is just encoded.

### Indexer Flow (Confirmed)
- BcsProfileCreatedEvent layout matches Move struct (field order, Option encoding).
- `test_parse_profile_created_event_from_live_transaction` passes with user's exact bytes.
- events.rs `parse_profile_event` outputs JSON with profile_id, display_name, username, bio, profile_picture, cover_photo, owner_address.
- profile.rs handler accepts profile_photo (alias profile_picture), owner_address (alias owner).

## Profile Indexer Schema Alignment (Feb 2025)

### Root Cause
Handler structs had `username` and `bio` as `Option<String>` while Move contract has them as required `String`. Deserialization quirks could yield `None`, and `into_model()` used `"user_{owner_prefix}"` fallback for username.

### Changes Implemented
1. **ProfileCreatedEvent**: `username: String`, `bio: String` (required); removed `#[serde(default)]` from required fields; `into_model()` uses `self.username`/`self.bio` directly; empty bio → `None` for DB.
2. **ProfileUpdatedEvent**: `username: String`, `bio: String`; `ProfileUpdate.bio` maps empty → `None`.
3. **parse_profile_event**: Removed duplicate `"owner"` from JSON output (kept `owner_address` only) to fix serde "duplicate field" error when both were present.
4. **Diagnostic logging**: On deserialization failure, log error + `json_keys` for schema drift debugging.
5. **Integration test**: `test_profile_created_bcs_to_handler_to_new_profile` verifies BCS → JSON → handler → NewProfile chain.

### Verification
- `cargo test -p myso-indexer-alt-social` — 9 tests pass
- `cargo clippy -p myso-indexer-alt-social` — clean

## PoC Logic Implementation (Feb 2025)

### Summary
Implemented full Proof of Creativity (PoC) event handling in myso-indexer-alt-social, mirroring the original mys-indexer implementation.

### Changes
1. **myso-indexer-alt-social-schema/models.rs**: Added NewPocBadge, NewPocRevenueRedirection, NewPocAnalysisResult, NewPocDispute, NewPocDisputeVote, NewPocConfiguration Insertable structs.
2. **handlers/events.rs**: Added BCS structs for all 9 PoC events; added parse_poc_event with module branch in parse_event_contents.
3. **handlers/mod.rs**: Added SocialEventRow variants (PocBadge, PocAnalysisResult, PocRevenueRedirection, PocDispute, PocDisputeVote, PocConfiguration, PostPocUpdate, PostRevenueRedirectUpdate, PocDisputeResolved, PocVoteRewardClaimed); Handler::commit arms for each.
4. **handlers/poc.rs**: Implemented process_* functions that parse JSON, validate, and return SocialEventRow vectors. All 9 events covered: AnalysisSubmittedEvent, PoCBadgeIssuedEvent, RevenueRedirectionActivatedEvent, PoCDisputeSubmittedEvent, DisputeVoteCastEvent, PoCDisputeResolvedEvent, VotingRewardClaimedEvent, PoCConfigUpdatedEvent, TokenPoolSyncNeededEvent (log only).

### Events Covered
| Event | Action |
|-------|--------|
| AnalysisSubmittedEvent | Insert poc_analysis_results; update posts (poc_* fields) |
| PoCBadgeIssuedEvent | Insert poc_badges |
| RevenueRedirectionActivatedEvent | Insert poc_revenue_redirections; update posts (revenue_redirect_*) |
| PoCDisputeSubmittedEvent | Insert poc_disputes |
| DisputeVoteCastEvent | Insert poc_dispute_votes |
| PoCDisputeResolvedEvent | Update poc_disputes; optionally revoke badge/remove redirection |
| VotingRewardClaimedEvent | Update poc_dispute_votes (reward_claimed, reward_amount) |
| PoCConfigUpdatedEvent | Insert poc_configuration |
| TokenPoolSyncNeededEvent | Log only |

## Insurance Alt Indexer Implementation (Feb 2025)

### Summary
Completed full insurance support in myso-indexer-alt stack per plan: schema models, event parsing, handler logic, commit branches, and social server API.

### Part 1: Alt Schema – Insurance Models
- **models.rs**: NewInsuranceConfig, NewInsuranceVault, NewInsurancePolicy, NewInsuranceEventLog, NewInsuranceVaultTransaction, NewInsurancePolicyEvent, NewInsuranceMarketExposure, NewInsuranceUserExposure

### Part 2: Event Parsing – Insurance BCS
- **events.rs**: BCS structs for all 9 insurance events; parse_insurance_event wired into parse_event_contents for module == "insurance"

### Part 3: Indexer Alt – Insurance Handler Logic
- **insurance.rs**: handle_insurance_event with full logic for ConfigInitializedEvent, ConfigUpdatedEvent, UnderwriterVaultCreatedEvent, UnderwriterVaultDepositedEvent, UnderwriterVaultWithdrawnEvent, CoveragePurchasedEvent, CoverageCancelledEvent, CoverageClaimedEvent, PolicyExpiredEvent
- **mod.rs**: SocialEventRow variants (InsuranceConfig, InsuranceVault, InsuranceVaultTransaction, InsuranceVaultBalanceUpdate, InsurancePolicy, InsurancePolicyEvent, InsuranceMarketExposure, InsuranceUserExposure, InsuranceEventLog, InsurancePolicyStatusUpdate, InsurancePolicyEventFromPolicy); commit arms for all; InsurancePolicyEventFromPolicy queries policy then inserts policy_event (reserve_locked computed or from reserve_released)

### Part 4: Social Server – Insurance API
- **reader.rs**: get_insurance_configuration, list_insurance_vaults, get_insurance_vault, list_insurance_vault_transactions, get_insurance_vault_exposures, list_insurance_policies, get_insurance_policy, list_insurance_market_policies
- **server.rs**: Routes GET /insurance/configuration, /insurance/vaults, /insurance/vaults/:vault_id, /insurance/vaults/:vault_id/transactions, /insurance/vaults/:vault_id/exposures, /insurance/policies, /insurance/policies/:policy_id, /insurance/markets/:market_id/policies

### Verification
- cargo check -p myso-indexer-alt-social -p myso-indexer-alt-social-schema -p myso-social-server: passes
- cargo xclippy: fails due to pre-existing errors in governance.rs and mod.rs (ProposalUpdate submitter_filter type, delegate_address/submitter move/borrow)

## Social API Gaps Implementation (Feb 2025)

### Completed – SPT Analytics (5 endpoints)
1. **GET /spt/analytics/top-performers**: Reader `get_spt_analytics_top_performers` – CTEs for current/previous prices, 24h volume, price/volume change %
2. **GET /spt/portfolios/:address/performance**: Reader `get_spt_portfolio_performance` – holdings, current/initial value, ROI per pool
3. **GET /spt/creators/:address/revenue-streams**: Reader `get_spt_creator_revenue_streams` – token pools by owner, buy/sell revenue, time range params (from/to)
4. **GET /spt/market-sentiment**: Reader `get_spt_market_sentiment` – buy/sell volume 24h, sentiment score, unique buyers/sellers
5. **GET /spt/pools/:id/liquidity-profile**: Reader `get_spt_liquidity_profile` – volume, tx count, buy/sell ratio, unique traders

### Data sources
- spt_pools, spt_price_history, spt_transactions, spt_holdings (raw SQL)
- transaction_type 'BUY'/'SELL' per schema constants

### Lint
- cargo clippy -p myso-social-server: passes (warnings in myso-pg-db, type_complexity in reader pre-existing)
- ./scripts/lint.sh: fails on pre-existing mysop-cli, myso-pg-db issues

---

## Faucet Speed Optimization Plan (Feb 2025)

### Primary Fix: Transaction Lock Timeout

The faucet currently **locks a coin for up to 5 minutes** (300 sec hardcoded) while waiting for fullnode confirmation. This is unacceptable—the coin is unusable for other requests the entire time. **Task 3** adds a configurable `txn_execution_timeout_secs` (default 60) so the faucet fails sooner and the coin can be retried via WAL instead of sitting locked.

### Background and Motivation

User reports faucet requests take too long to reach users. Log analysis revealed:

1. **Batch wait delay**: First request arrives → faucet waits up to **10 sec** (BATCH_TIMEOUT) to gather more requests → then processes. Single request = 10 sec delay before processing starts.
2. **Pool fallback delay**: Batch pool is empty (1 coin goes to regular pool) → wait **5 sec** (RECV_TIMEOUT) before trying regular pool fallback.
3. **Transaction timeout (critical)**: Tx submitted at 20:19:42, failed at 20:24:42 = **5 min** (300 sec). Coin is locked for the entire duration—ridiculous. Pool exhausted while waiting.
4. **Single-coin bottleneck**: Only 1 coin; when in flight, pool is empty. On-demand split triggers but adds latency.

### Root Causes (from logs)

| Bottleneck | Current | Impact |
|------------|---------|--------|
| Batch gather | 10 sec | User waits 10 sec before processing starts |
| Recv timeout | 5 sec | Extra 5 sec when batch pool empty |
| Txn execution | 300 sec (hardcoded) | Coin locked 5 min—unacceptable; pool exhausted |
| min_coin_threshold | 5 | Split triggers only when pool=0 |

### High-Level Task Breakdown

#### Task 1: Add configurable batch wait timeout
- **File**: `crates/myso-faucet/src/faucet/mod.rs`
- **Change**: Add `batch_wait_timeout_secs: u64` (default 3)
- **File**: `crates/myso-faucet/src/faucet/simple_faucet.rs`
- **Change**: In `batch_transfer_gases`, replace `BATCH_TIMEOUT` with `Duration::from_secs(faucet.config.batch_wait_timeout_secs)`
- **Effect**: Single request starts processing in ~3 sec instead of 10 sec

#### Task 2: Add configurable recv timeout
- **File**: `crates/myso-faucet/src/faucet/mod.rs`
- **Change**: Add `recv_timeout_secs: u64` (default 2)
- **File**: `crates/myso-faucet/src/faucet/simple_faucet.rs`
- **Change**: In `pop_gas_coin` and `pop_gas_coin_for_batch`, replace `RECV_TIMEOUT` with `Duration::from_secs(self.config.recv_timeout_secs)`
- **Effect**: Fallback to regular pool in ~2 sec instead of 5 sec when batch pool empty

#### Task 3: Add configurable transaction execution timeout (PRIORITY)
- **Problem**: Hardcoded 300 sec (5 min) in `sign_and_execute_txn`—coin is locked the entire time. Unacceptable.
- **File**: `crates/myso-faucet/src/faucet/mod.rs`
- **Change**: Add `txn_execution_timeout_secs: u64` (default **60**)
- **File**: `crates/myso-faucet/src/faucet/simple_faucet.rs`
- **Change**: In `sign_and_execute_txn`, replace `Duration::from_secs(300)` with `Duration::from_secs(self.config.txn_execution_timeout_secs)`
- **Effect**: Fail after 1 min max; coin goes back to WAL for retry instead of being locked 5 min. Pool can serve other requests sooner.

#### Task 4: Update FaucetConfig Default impl
- **File**: `crates/myso-faucet/src/faucet/mod.rs`
- **Change**: Add the three new fields to `impl Default for FaucetConfig`

#### Task 5 (Optional): Lower default min_coin_threshold
- **File**: `crates/myso-faucet/src/faucet/mod.rs`
- **Change**: Consider `min_coin_threshold: 1` for single-coin deployments (triggers split earlier)
- **Trade-off**: May cause more frequent splits; keep at 5 if splits are expensive

#### Task 6 (Optional): Pre-split on startup
- **File**: `crates/myso-faucet/src/faucet/simple_faucet.rs`
- **Change**: In `SimpleFaucet::new`, if `coins.len() == 1` and coin value >> (amount * num_coins), split before adding to pool
- **Effect**: Start with multiple coins instead of 1; reduces pool exhaustion

### Project Status Board

- [ ] Task 1: Add batch_wait_timeout_secs config
- [ ] Task 2: Add recv_timeout_secs config
- [ ] Task 3: Add txn_execution_timeout_secs config
- [ ] Task 4: Update FaucetConfig Default
- [ ] Task 5: (Optional) Lower min_coin_threshold
- [ ] Task 6: (Optional) Pre-split on startup

### Deployment Note (Railway)

After implementation, user can pass flags in setup-wallet.sh or via env:
```
--batch-wait-timeout-secs 3 --recv-timeout-secs 2 --txn-execution-timeout-secs 60
```

Or add CLI args to the faucet exec line. Defaults will improve latency without config changes.

### Infrastructure Consideration

The 5 min transaction timeout suggests the fullnode at `http://fullnode.testnet.mysocial.network:8082` may be slow to confirm from Railway's region. User should verify:
- Fullnode RPC latency from Railway
- Whether a geographically closer fullnode exists

---

# Proof of Creativity production escrow (Apr 2026)

## Spec (implementation-aligned)
- **Outcomes / redirect kinds**: Encoded as `u8` on `Post` (`poc_outcome`, `poc_redirection_kind`) and mirrored in index DB; `PoCResultAppliedEvent` for indexers.
- **Escrow**: MYSO held in `Post.poc_escrow`; tips/reservation fee paths use MYSO-specific redirection or abort for non-MYSO when escrow mode; generic coin tips disallow escrow.
- **Claim authority**: `claim_poc_escrow` / internal drain with events; dispute overturn refunds emit `PoCEscrowClaimedEvent`; post delete refunds emit same for balance reconciliation.
- **Replay**: `PoCDispute.voting_rewards_claimed` table records per-voter claims.

## Data stack
- Migration `20260429120000_add_post_poc_escrow_fields`: `posts.poc_outcome`, `poc_redirection_kind`, `poc_escrow_balance`.
- Indexer: `PoCResultAppliedEvent`, `PoCEscrowDepositEvent`, `PoCEscrowClaimedEvent`; SPT `PocRedirectionUpdatedEvent` BCS v1/v2 compat; revenue redirection post updates include `poc_redirection_kind`.
- GraphQL / social-server: `pocOutcome`, `pocRedirectionKind`, `pocEscrowBalance` on `Post`; `PostBasicRow` extended for REST.

## Rollout
- Publish Move → run DB migration → deploy indexer → GraphQL → social-server; reindex if historical escrow events needed for `poc_escrow_balance`.

## Verification
- `UPDATE=1 cargo test -p myso-framework --test build-system-packages`
- `cargo check` on social-schema, indexer, reader, graphql, social-server
- `cargo insta accept` for GraphQL schema + pipeline snapshots after SDL export test

---

# PoC: Clock-based dispute voting, ms config, single dispute fee (Apr 2026)

## Done
- **Move** (`proof_of_creativity.move`): `Clock` on submit/vote/resolve; `voting_duration_ms`; `voting_start_ms` / `voting_end_ms`; removed `dispute_protocol_fee`; single `dispute_cost`; `get_dispute_voting_status(..., current_time_ms)`; tests + `interact.sh` updated.
- **DB**: In-place edits to `20250620000000_create_poc_tables` and dependent PoC migrations; Diesel `schema.rs` / `models/poc.rs`.
- **Indexer**: `handlers/events.rs`, `handlers/poc.rs` event shapes; `posts_handler` redirect-only clears when `redirection_removed` without `badge_revoked`.
- **Reader / GraphQL / social-server**: `PocDisputeRow` voting ms; `PocConfig.votingDurationMs`; `PocDispute.votingStartMs` / `votingEndMs`; social-server `PostBasicRow` aligned with denormalized posts PoC columns.
- **Framework**: Packages rebuild, `published_api.txt`, docs, bytecode snapshot (`cargo run -p myso-framework-snapshot`).

## Verification
- `cargo insta accept` in `crates/myso-indexer-alt-graphql` for `schema.graphql.snap`
- `MYSO_SKIP_SIMTESTS=1 cargo nextest run -p myso-indexer-alt-graphql -p myso-indexer-alt-social -p myso-indexer-alt-social-schema -p myso-social-server`: 192 passed
- `cargo xclippy`: exit 0 (pre-existing warnings in other crates)

---

# Indexer posts `poc_outcome` schema drift (May 2026)

## Fix
- Extended [`20251230000001_add_poc_metadata_fields/up.sql`](crates/myso-indexer-alt-social-schema/migrations/20251230000001_add_poc_metadata_fields/up.sql) with `posts.poc_outcome` and `posts.poc_redirection_kind` (conditional adds + comments); header comment documents one-time `ALTER TABLE ... IF NOT EXISTS` for DBs that already ran the migration before this edit.
- Extended [`down.sql`](crates/myso-indexer-alt-social-schema/migrations/20251230000001_add_poc_metadata_fields/down.sql) to drop those columns on rollback.

## Ops
- Existing databases: run the two `ADD COLUMN IF NOT EXISTS` statements from the migration header (diesel will not re-run `20251230000001`).
- Then run remaining migrations / deploy indexer as usual.

## Verification
- `cargo check -p myso-indexer-alt-social-schema -p myso-indexer-alt-social`: passes

---

# Orderbook indexer localnet: empty event tables (May 2026)

## Done
- **Regression test** (`myso-types`): `data_ingestion_field_mask_roundtrip_preserves_transaction_events` — data-ingestion field mask + merge → encode/decode → `FullCheckpoint::try_from` preserves `tx.events`; requires `FieldMaskUtil` in scope for `from_paths`.
- **`OrderbookEnv::Local`**: orderbook crate + standalone `main` requires checkpoint source for Local; `myso start` uses Local.
- **Stable ingestion dir**: with `--with-indexer`, default `data_ingestion` under config dir (not temp `keep()`).
- **Diagnostics**: local warning when orderbook tx has `events: None`.
- **Docs**: `myso-orderbook-server` + `myso-indexer-alt-orderbook` READMEs — `pools` (admin) vs `pool_created` (indexer); local paths and metrics.

## Verification
- `cargo test -p myso-types --lib data_ingestion_field_mask_roundtrip_preserves_transaction_events`: pass
- `cargo check -p myso-indexer-alt-orderbook -p myso`: pass

---

# Contra private-transactions runnable script (Jun 2026)

## Summary
Implemented E2E helper for `contra::contra` confidential transfers: extended `contra-crypto-fixtures`, added `contra-e2e` Move package, and `scripts/contra-runnable.sh`.

## Deliverables
- **`crates/contra-crypto-fixtures`**: twisted-ElGamal, NIZK provers, session DST, `account_id` derivation, `build_transfer_bundle` CLI (`transfer`, `unwrap`, `keygen`, `session-info`, `account-id`)
- **`crates/myso-framework/packages/contra-e2e`**: post-genesis test coin via `coin_registry::new_currency` + `CoinCreationAdminCap`
- **`scripts/contra-runnable.sh`**: GraphQL session refresh, publish/register test coin, setup token/accounts, wrap/transfer/unwrap PTBs, menu + `--run-all`

## Verification
- `cargo test -p contra-crypto-fixtures --lib`: 6 tests pass
- `cargo clippy -p contra-crypto-fixtures -- -D warnings`: pass
- `./scripts/contra-runnable.sh --refresh-session`: resolves TokenRegistry on local GraphQL
- Full `--run-all` on current localnet blocked by missing `AccountRegistry` and `CoinCreationAdminCap` (needs contra genesis + social bootstrap)

