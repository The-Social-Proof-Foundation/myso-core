# SPoT Oracle Server — Architecture (V1)

## Overview

`myso-spot-oracle` is a single-process Axum/Tokio service that:

1. Ingests SPoT claims via **checkpoint gRPC** (`SubscribeCheckpoints` → `PostCreatedEvent` → `enable_spot` filter)
2. Runs NLU extraction + deterministic admissibility + **Resolver Compiler**
3. Creates on-chain markets via `create_spot_claim` + `create_spot_market_for_claim` / `link_post_to_spot_claim` (when chain configured)
4. Resolves markets using stored `ResolverDefinition` + **direct HTTP** `TrustedSource` adapters (no Discovery dependency)
5. Submits `oracle_resolve` with auditable evidence — **accounting-only** (no winner/creator payout transfers)

Legacy HTTP `GET /spot/pending-posts` polling remains available only when
`SPOT_ORACLE_INGEST_MODE=http|both` (deprecated; default is `checkpoint`).

**Discovery is not required.** PoC may still run Discovery separately; SPoT never imports or calls it.

## Claim → Market → Post model

| Layer | On-chain object | Role |
|-------|-----------------|------|
| **Claim** | `SpotClaim` | Permanent semantic truth anchor (`semantic_claim_hash`) |
| **Market** | `SpotMarket` | Time-bounded liquidity pool + escrow + pending payout tables |
| **Post** | `Post` | Referral surface (`enable_spot`, `spot_claim_id`, `spot_id` = market) |

Multiple posts may link to one claim; one open market per claim at a time. Clients bet via
**router-only** `place_spot_bet_for_post` — never by selecting a market ID directly.

Off-chain canonicalization splits `semantic_claim_hash` (claim identity) from `market_key_hash`
(deadline/window identity). Oracle DB tables: `spot_claims`, `spot_markets`, `post_claim_links`.

## Resolution timing

Every accepted claim must include an extractable UTC deadline. Review uses a **two-phase gate**:

1. **Provability** (`rules::evaluate_provably`) — category, subject/predicate, options, trusted-source availability.
2. **Context-Aware Deadline Resolver (CADR)** (`context_deadline::resolve_context_deadline` + `rules::resolve_and_validate_deadline`) — infers deadlines only after provability passes.

CADR tiers (in order):

- Explicit text parsing (relative dates, ISO dates, year + election phrases, quarters)
- Calendar templates (`calendar_template` event provider — U.S. presidential/midterm election days)
- `EventRegistry` scored matching (keywords, year tokens, entity aliases, fuzzy election typos)
- Price ongoing 30-minute spacing (price claims only)

Rejected when no deadline can be inferred (`missing_deadline`), except:

- **Scheduled events** — discovered by pluggable **Event Providers** (`events/`) into Postgres and matched at review time from the in-memory `EventRegistry` (player names, tournament keywords, election phrases infer end dates).
- **Ongoing price claims** — default to the next 30-minute boundary; identical semantic claims in the same bucket share one market.

Category-aware deadline horizons (configurable via env):

| Context | Default max horizon |
|---------|---------------------|
| Default | 730 days |
| Election events | 1,460 days (4 years) |
| Sports mega-events | 1,095 days (3 years) |

`deadline_provenance` is stored in `resolver_hints` on the canonical claim for audit.

On market creation the oracle stores immutable `SpotMarket.resolution_at_ms`
on-chain and schedules:

- **ResolveMarket** — poll from `maturity_at`, submit `oracle_resolve` only at or after `resolution_at_ms`
- **refund_unresolved** — eligible at `resolution_at_ms + max_resolution_buffer_ms`

Creator payout windows remain anchored to actual `resolution_timestamp_ms` after settlement.

## Lazy payout architecture (V1)

The oracle performs **accounting exactly once** per market resolution:

1. Transfer **only** `platform_fee` + `ecosystem_fee` to treasuries at resolve.
2. Write winner amounts to `SpotMarket.pending_payouts` (lazy `claim_payout`).
3. Write creator referral amounts to `SpotMarket.pending_creator_payouts` + `creator_payout_index`
   (lazy `claim_creator_payout(payout_id)` — single O(1) claim per tx).
4. Escrow retains winner + creator funds until claimed or expired-reclaimed.

The oracle **never** submits winner or creator payout transactions. Settlement complexity is
O(1) per claimer, not O(n) bettors.

## Trusted sources (SPoT-owned)

Sources load from `SPOT_ORACLE_SOURCES_CONFIG` (default `config/sources.localnet.yaml`) into
`spot_trusted_sources` and the in-process adapter registry.

Adapters fetch HTTP directly (`sources/http_fetch.rs` + per-adapter normalize):

- `coingecko`, `coinbase`, `chainlink` (price)
- `github_releases` (release compare)
- `rss_event`, `http_official` (events / generic HTTP)
- `wikipedia` (factual/historical REST summary)
- `stub` (off-chain / unit tests when `SPOT_ORACLE_LIVE_SOURCES=false`)

Resolution uses **quorum** across evidence: unanimous agreement → high confidence; conflict →
`confidence_bps=0` → on-chain `DAO_REQUIRED`.

## Event providers (SPoT-owned)

Event providers load from `SPOT_ORACLE_EVENT_PROVIDERS_CONFIG` (default
`config/event_providers.localnet.yaml`) into `spot_event_providers`. A background sync loop
calls each enabled provider's `discover()` implementation, upserts normalized rows into
`spot_scheduled_events`, and reloads the in-memory `EventRegistry`.

v1 provider types:

- `yaml_seed` — bootstrap seed events from YAML (localnet/dev)
- `calendar_template` — computable recurring public events (U.S. presidential/midterm elections)
- `ical_feed` — parse ICS/VCALENDAR feeds when `SPOT_ORACLE_LIVE_SOURCES=true`
- `stub` — deterministic test events

New event domains are added by implementing `EventProvider` and registering a YAML row — review
matching logic does not change.

Admin API: `GET /v1/events`, `GET /v1/event-providers/health`, `POST /v1/event-providers/:key/sync`,
`PATCH /v1/events/:id/override` (admin secret).

## Evidence

On-chain `oracle_resolve` carries **URLs only** (contract requirement). Off-chain audit trail
carries hash + payload so decisions remain verifiable after source content changes.

`GET /v1/evidence/:market_id` returns evidence rows and bundle metadata.

## Job queue

PostgreSQL `spot_jobs`:

- `FOR UPDATE SKIP LOCKED` claim
- `priority_score`, `run_after`, `attempts`, dead-letter

Job types: `ReviewPost`, `ResolveMarket`, `SubmitChainTx`

## Scripts

Two-terminal local E2E (runnable scripts never start, stop, wipe, or migrate the oracle stack):

```bash
# Terminal 1 — postgres (:5435) + oracle workers (:8097); only place that may wipe DB
./scripts/run-spot-oracle.sh

# Terminal 2 — PTBs, session refresh, poll shared postgres / GraphQL
./scripts/spot-oracle-runnable.sh
```

| Script | Purpose |
|--------|---------|
| `./scripts/run-spot-oracle.sh` | Start postgres + oracle service; optional DB wipe on launch; `--refresh-session` from GraphQL |
| `./scripts/spot-oracle-runnable.sh` | E2E menu: walkthrough, create post, verify analysis (requires oracle running in terminal 1) |
| `ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all-onchain` | On-chain pipeline: post → review → resolve (no bet) |
| `./scripts/spot-oracle-runnable.sh --run-walkthrough` | Full bet → resolve → payout; optional `ENABLE_INSURANCE_E2E=1` |
| `./scripts/spot-oracle-runnable.sh --create-post` | Create always-on SPoT post on-chain |
| `./scripts/spot-oracle-runnable.sh --run-router-only` | Router rejects unlinked post bets (Move) |
| `./scripts/spot-oracle-runnable.sh --run-ownership-transfer` | Settlement creator gate on claim (Move) |
| `./scripts/spot-insurance-runnable.sh --run-all` | Insurance walkthrough (also requires `./scripts/run-spot-oracle.sh`) |

## Session env

| File | Owner script |
|------|----------------|
| `network.config/spot-oracle/spot-oracle-session.env` | `run-spot-oracle.sh` / `spot-oracle-runnable.sh --refresh-session` |

Key SPoT vars: `SPOT_ORACLE_STREAMING_URL`, `SPOT_ORACLE_INGEST_MODE` (default `checkpoint`),
`SPOT_ORACLE_SOURCES_CONFIG`, `SPOT_ORACLE_EVENT_PROVIDERS_CONFIG`, `SPOT_ORACLE_LIVE_SOURCES`,
`SPOT_ORACLE_MAX_ELECTION_HORIZON_SECS`, `SPOT_ORACLE_MAX_SPORTS_HORIZON_SECS`.

## Local stack (compose, not merged)

Prefer `./scripts/run-spot-oracle.sh` in one terminal; use `spot-oracle-runnable.sh` or
`spot-insurance-runnable.sh` in a second terminal for E2E flows.

Manual compose (postgres only):

```bash
# SPoT DB (:5435) — Discovery not required
docker compose -f crates/myso-spot-oracle/docker-compose.yml up -d spot-oracle-postgres
```

Social-server (:9126) + GraphQL (:9125) + fullnode gRPC (:9000) — localnet stack.
Optional: `myso start --with-social-indexer --with-spot`.

Secrets: align `SPOT_ORACLE_SYNC_SECRET` ↔ `SPOT_ORACLE_SOCIAL_SYNC_SECRET`.
On-chain resolve also needs `SPOT_ORACLE_PLATFORM_OBJECT_ID` +
`SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID` + `SPOT_ORACLE_REGISTRY_OBJECT_ID`.

## GraphQL

Indexer GraphQL exposes `spotClaim`, `spotMarket`, `spotRoute`, and `Post.spotClaimId`
alongside legacy `spotRecord` / `spotResolution` fields.
