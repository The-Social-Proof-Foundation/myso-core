# SPoT Oracle Server — Architecture (V1)

## Overview

`myso-spot-oracle` is a single-process Axum/Tokio service that:

1. Ingests pending SPoT posts from `myso-social-server`
2. Runs NLU extraction + deterministic admissibility + **Resolver Compiler**
3. Creates on-chain markets via `create_spot_record_for_post` (when chain configured)
4. Resolves markets using stored `ResolverDefinition` + live `TrustedSource` adapters
5. Submits `oracle_resolve` with auditable evidence

## Dependency boundary

SPoT depends only on:

- `myso-discovery-service-core` (traits, `HttpFetchClient`, source-config loader)
- `myso-discovery-service-schema` + `myso-spot-oracle-schema` (migrations)

It never imports the discovery runtime (scheduler, workers, store).

## Two traits, two registries

| Trait | Crate | Responsibility |
|-------|-------|----------------|
| `DiscoverySource` | discovery-core | Continuous crawl → `discovery_assets` |
| `TrustedSource` | spot-oracle | Point-in-time settlement evidence |

| Registry | Holds |
|----------|-------|
| `DiscoveryRegistry` | `DiscoverySource` impls |
| `ResolverRegistry` | `TrustedSource` impls |

No adapter implementation is shared across crates.

## Review vs resolution

**Review (once per claim):** LLM → canonicalize → rules → Resolver Compiler → persist `ResolverDefinition`

**Resolution (on schedule):** scheduler → resolver engine reads definition only → `TrustedSource::resolve()` → evidence → chain tx

The scheduler never interprets English.

## Job queue

PostgreSQL `spot_jobs` mirrors `discovery_jobs`:

- `FOR UPDATE SKIP LOCKED` claim
- `priority_score`, `run_after`, `attempts`, dead-letter

Job types: `ReviewPost`, `ResolveMarket`, `SubmitChainTx`, `RssWake`

## V1 live adapters

- `coingecko`, `coinbase`, `chainlink` (price)
- `github_releases` (release compare)
- `rss_event`, `http_official` (events / generic HTTP)

## Scripts

- `./scripts/run-spot-oracle.sh` — local dev boot (loads session env)
- `./scripts/run-spot-oracle.sh --refresh-session` — refresh `network.config/spot-oracle/spot-oracle-session.env` from GraphQL
- `./scripts/spot-oracle-runnable.sh --refresh-session` — same refresh + interactive menu
- `ASSUME_YES=1 ./scripts/spot-oracle-runnable.sh --run-all` — off-chain review+resolve E2E with live CoinGecko evidence
- `./scripts/discovery-runnable.sh --refresh-session` — write `network.config/discovery/discovery-session.env`
- `./scripts/discovery-runnable.sh` — discovery live-fetch E2E

## Session env

- `network.config/discovery/discovery-session.env`
- `network.config/spot-oracle/spot-oracle-session.env`
