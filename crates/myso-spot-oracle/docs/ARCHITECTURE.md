# SPoT Oracle Server — Architecture (V1)

## Overview

`myso-spot-oracle` is a single-process Axum/Tokio service that:

1. Ingests SPoT claims via **checkpoint gRPC** (`SubscribeCheckpoints` → `PostCreatedEvent` → `enable_spot` filter)
2. Runs NLU extraction + deterministic admissibility + **Resolver Compiler**
3. Creates on-chain markets via `create_spot_record_for_post` / `link_post_to_spot_claim` (when chain configured)
4. Resolves markets using stored `ResolverDefinition` + `TrustedSource` adapters backed by **Discovery `/v1/*`**
5. Submits `oracle_resolve` with auditable evidence — **accounting-only** (no winner/creator payout transfers)

Legacy HTTP `GET /spot/pending-posts` polling remains available only when
`SPOT_ORACLE_INGEST_MODE=http|both` (deprecated; default is `checkpoint`).

## Claim → Market → Post model

| Layer | On-chain object | Role |
|-------|-----------------|------|
| **Claim** | `SpotClaim` | Permanent semantic truth anchor (`semantic_claim_hash`) |
| **Market** | `SpotMarket` | Time-bounded liquidity pool + escrow + pending payout tables |
| **Post** | `Post` | Discovery/referral surface (`enable_spot`, `spot_claim_id`, `spot_id`) |

Multiple posts may link to one claim; one open market per claim at a time. Clients bet via
**router-only** `place_spot_bet_for_post` — never by selecting a market ID directly.

Off-chain canonicalization splits `semantic_claim_hash` (claim identity) from `market_key_hash`
(deadline/window identity). Oracle DB tables: `spot_claims`, `spot_markets`, `spot_post_links`.

## Lazy payout architecture (V1)

The oracle performs **accounting exactly once** per market resolution:

1. Transfer **only** `platform_fee` + `ecosystem_fee` to treasuries at resolve.
2. Write winner amounts to `SpotMarket.pending_payouts` (lazy `claim_payout`).
3. Write creator referral amounts to `SpotMarket.pending_creator_payouts` + `creator_payout_index`
   (lazy `claim_creator_payout(payout_id)` — single O(1) claim per tx).
4. Escrow retains winner + creator funds until claimed or expired-reclaimed.

The oracle **never** submits winner or creator payout transactions. Settlement complexity is
**O(bets + distinct referrers)** for table writes, not O(payout transfers).

| Entry | Who calls | Effect |
|-------|-----------|--------|
| `claim_payout` | Winning bettor | O(1) escrow → winner |
| `claim_creator_payout(payout_id)` | Recorded creator | O(1) escrow → creator |
| `reclaim_expired_creator_rewards(payout_id)` | Anyone (keeper) | After `creator_claim_window_ms`, unclaimed slice → ecosystem (+ platform remainder per `expired_creator_ecosystem_bps`) |
| `withdraw_spot_bet` (pre-resolve) | Bettor | O(1) refund minus protocol + **immediate** creator fee to referrer post owner |

Pre-resolution withdraws transfer the creator fee slice immediately (single bet, O(1)); post-resolve
creator fees are lazy.

## Spot Router (mandatory betting path)

Social-server exposes `GET /spot/route/:post_id` → `{ claim_id, target_market_id, link_kind }`.
GraphQL: `spotRoute(postId)`. Clients use the returned market when building
`place_spot_bet_for_post` PTBs.

## Dependency boundary

SPoT depends on:

- `myso-discovery-service-core` (shared DTOs, `DiscoveryClient` trait, and `HttpDiscoveryClient`)
- `myso-spot-oracle-schema` (migrations — spot DB only; no co-hosted discovery schema)

It never imports the discovery runtime (scheduler, workers, store) and **never reads
`discovery_assets`**. Settlement uses normalized Discovery responses only; external
HTTP/RSS/GitHub fetches happen in Discovery, not SPoT.

## Two traits, two registries

| Trait | Crate | Responsibility |
|-------|-------|----------------|
| `DiscoverySource` | discovery-core | Continuous crawl → `discovery_assets` |
| `TrustedSource` | spot-oracle | Point-in-time settlement evidence via Discovery client |

| Registry | Holds |
|----------|-------|
| `DiscoveryRegistry` | `DiscoverySource` impls (Discovery runtime) |
| `ResolverRegistry` | `TrustedSource` impls (SPoT) |

No adapter implementation is shared across crates.

## Claim ingest (checkpoint gRPC)

```
SubscribeCheckpoints (SPOT_ORACLE_STREAMING_URL)
  → for each checkpoint tx event
  → if package == SPOT_ORACLE_SOCIAL_PACKAGE_ID and type == PostCreatedEvent
  → if enable_spot == true and spot_id is None
  → if market not already in SPoT DB (idempotent)
  → insert_market + enqueue ReviewPost
```

Watermark persisted in `checkpoint_ingest_state` for restart safety.

## Review vs resolution

**Review (once per claim):** LLM → canonicalize → rules → Resolver Compiler → persist `ResolverDefinition`

**Resolution (on schedule):** scheduler → resolver engine reads definition only → `TrustedSource::resolve()` → Discovery client (`refresh=true`) → evidence → chain tx

The scheduler never interprets English. See [RESOLVER_COMPILER.md](RESOLVER_COMPILER.md) for the LLM → `ResolverDefinition` contract.

## Claim lifecycle (off-chain)

```
post_created → pending_review → pending_create → waiting → resolving → resolved | refunded
                     ↓                                    ↓ (low confidence)
                  rejected                            dao_required
```

| Off-chain status | On-chain `SpotRecord.status` |
|------------------|------------------------------|
| `waiting` / `resolving` | `OPEN` (1) |
| `dao_required` | `DAO_REQUIRED` (2) |
| `resolved` | `RESOLVED` (3) |
| `refunded` | `REFUNDABLE` (4) |

Transitions are enforced in `claim/lifecycle.rs` and audited in `market_transitions`.
Refund is enqueued at market creation (`run_after = created_at + max_resolution_window_ms`)
and submitted via `refund_unresolved` when the resolve deadline passes without outcome.

## Evidence model

Every resolve attempt persists an **`EvidenceBundle`**:

- `evidence_bundles`: `bundle_hash`, `market_id`, `resolver_job_id`
- `evidence` rows: `payload`, `provenance` (aligned with Discovery `FetchProvenance`), optional `signature`, `content_hash`, `source_url`

On-chain `oracle_resolve` carries **URLs only** (contract requirement). Off-chain audit trail
carries hash + payload so decisions remain verifiable after source content changes.

`GET /v1/evidence/:market_id` returns evidence rows and bundle metadata.

## Job queue

PostgreSQL `spot_jobs` mirrors `discovery_jobs`:

- `FOR UPDATE SKIP LOCKED` claim
- `priority_score`, `run_after`, `attempts`, dead-letter

Job types: `ReviewPost`, `ResolveMarket`, `SubmitChainTx`, `RssWake`

## V1 live adapters (Discovery-backed)

All factual adapters call `HttpDiscoveryClient` in `-core` (`/v1/prices`, `/v1/releases`, `/v1/events`).
Registry is built from `GET /v1/sources` when `SPOT_ORACLE_DISCOVERY_CLIENT_URL` is set.

- `coingecko`, `coinbase`, `chainlink` (price)
- `github_releases` (release compare)
- `rss_event`, `http_official` (events / generic HTTP)

## Scripts

| Script | Purpose |
|--------|---------|
| `./scripts/run-spot-oracle.sh` | Local boot (postgres + cargo); `--refresh-session` from GraphQL |
| `ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all` | Off-chain review → evidence → resolved (starts Discovery) |
| `./scripts/spot-oracle-runnable.sh --run-all-onchain` | Checkpoint ingest + create/resolve PTBs (funded oracle key) |
| `SPOT_ORACLE_ONCHAIN=1 … --run-all` | Same as `--run-all-onchain` |
| `./scripts/spot-oracle-post-runnable.sh --run-all` | Create `enable_spot=true` post via social GraphQL |
| `./scripts/discovery-runnable.sh` | Discovery fetch E2E (`source_id` + `content_hash`) |
| `./scripts/spot-oracle-runnable.sh --run-router-only` | Router rejects unlinked post bets (Move) |
| `./scripts/spot-oracle-runnable.sh --run-ownership-transfer` | Settlement creator gate on claim (Move) |

## Session env

| File | Owner script |
|------|----------------|
| `network.config/discovery/discovery-session.env` | `discovery-runnable.sh --refresh-session` |
| `network.config/spot-oracle/spot-oracle-session.env` | `run-spot-oracle.sh` / `spot-oracle-runnable.sh --refresh-session` |
| `network.config/poc-oracle/poc-oracle-session.env` | PoC / social GraphQL refresh |

Key SPoT vars: `SPOT_ORACLE_STREAMING_URL`, `SPOT_ORACLE_INGEST_MODE` (default `checkpoint`),
`SPOT_ORACLE_DISCOVERY_CLIENT_URL`, `SPOT_ORACLE_DISCOVERY_CLIENT_SECRET`.

## Local stack (compose, not merged)

Bring up pieces independently — do not require a single mega-compose:

```bash
# Discovery DB (:5434) + service (:8096)
docker compose -f crates/myso-discovery-service/docker-compose.yml up -d discovery-postgres
./scripts/run-discovery-service.sh

# SPoT DB (:5435)
docker compose -f crates/myso-spot-oracle/docker-compose.yml up -d spot-oracle-postgres

# Social-server (:9126) + GraphQL (:9125) + fullnode gRPC (:9000) — localnet stack
```

Secrets: align `SPOT_ORACLE_SYNC_SECRET` ↔ `SPOT_ORACLE_SOCIAL_SYNC_SECRET`,
`DISCOVERY_CLIENT_SECRET` ↔ `SPOT_ORACLE_DISCOVERY_CLIENT_SECRET` on `/v1/*`,
and `DISCOVERY_EMBED_SECRET` on both discovery and PoC. On-chain resolve also needs
`SPOT_ORACLE_PLATFORM_OBJECT_ID` + `SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID`.
See `crates/myso-discovery-service/docs/ARCHITECTURE.md` for the full ports/secrets matrix.
