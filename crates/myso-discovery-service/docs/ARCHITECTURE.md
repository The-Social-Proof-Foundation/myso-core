# Discovery Service Architecture

See also `proof-of-creativity/docs/discovery-architecture.md` for cross-repo integration.

## Two corpora

| Corpus | Domain | ContentKind | Consumer | Config |
|--------|--------|-------------|----------|--------|
| Creative media | `creative` | `media` (image/audio/video) | PoC embed | `sources.media.localnet.yaml` |
| Factual text | `factual` | `text` (RSS/JSON/HTML) | SPoT `TrustedSource` settlement | `sources.factual.localnet.yaml` |

**Embed gate:** jobs enqueue only when `domain=creative` **and** `content_kind=media` **and** `DISCOVERY_EMBED_ENABLED=true`. Factual assets never hit PoC.

**Factual YAML SoT:** `sources.factual.localnet.yaml` is the single source registry for price/RSS/GitHub/HTTP adapters. SPoT reads enabled sources via `GET /v1/sources` — it does not upsert local YAML when `SPOT_ORACLE_DISCOVERY_CLIENT_URL` is set.

## Crate split

| Crate | Role |
|-------|------|
| `myso-discovery-service-core` | `DiscoverySource` trait, normalized DTOs, `DiscoveryClient` trait + `HttpDiscoveryClient`, registry, lifecycle FSM, YAML loader, `HttpFetchClient` |
| `myso-discovery-service-schema` | Shared `discovery_*` migrations |
| `myso-discovery-service` | Crawl runtime: scheduler, factual `/v1/*` API, cache, rate-limit, optional embed worker, store, admin API, metrics |

**Boundary:** SPoT (`myso-spot-oracle`) depends on `-core` + spot schema only. It implements a separate `TrustedSource` for settlement evidence — adapter *impls* are not shared. SPoT never reads `discovery_assets`.

## Modules

- `sources/` (core) — `DiscoverySource` + adapters (`rss`, `github_releases`, `http_official`; creative stubs; `manual_curated` for media E2E)
- `api/` (core) — shared DTOs, `DiscoveryClient` trait, and `HttpDiscoveryClient`
- `factual_api/` (runtime) — `/v1/sources`, `/v1/prices`, `/v1/releases`, `/v1/events`, `/v1/refresh`
- `cache/` (runtime) — `discovery_factual_cache` for `/v1/*` responses
- `rate_limit/` (runtime) — per-source throttle on refresh + on-demand fetch
- `lifecycle/` (core) — asset FSM (`normalized` → `indexed` → `matched` → … → `claimed`)
- `scheduler/` — poll adapters; embed only creative media
- `embed_client/` — PoC `POST /internal/discovery/embed`
- `admin/` — secret-gated exclude + source replay (`x-discovery-admin-secret`)
- `store/` — SQLx repositories (`content_kind` on assets)
- `metrics/` — Prometheus registry (`discovery_*`, `cache_hits_total`, `refresh_total`) on `DISCOVERY_METRICS_ADDRESS`

## Factual query API (`/v1/*`)

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/v1/sources` | Enabled sources + trust/health summary |
| `GET` | `/v1/sources/{id}/health` | Per-source health |
| `GET` | `/v1/sources/health` | Bulk health |
| `GET` | `/v1/prices` | `NormalizedPrice` + provenance |
| `GET` | `/v1/releases` | `NormalizedRelease` + provenance |
| `GET` | `/v1/events` | `NormalizedEvent[]` + provenance |
| `POST` | `/v1/refresh` | Force adapter poll + return status |

Auth: `x-discovery-client-secret` when `DISCOVERY_CLIENT_SECRET` is set.

## Ports & databases

| Service | Default | Notes |
|---------|---------|-------|
| Discovery API | `127.0.0.1:8096` | `/health`, `/v1/*`, `/discovery/stats`, `/admin/*`, `/internal/*` |
| Discovery metrics | `127.0.0.1:9286` | Prometheus scrape (avoids `myso start` :9186) |
| Discovery Postgres | `127.0.0.1:5434` | DB `discovery` |
| PoC API | `127.0.0.1:8000` (or `:8001`) | Media embed only |
| SPoT oracle | `127.0.0.1:8097` | Separate DB `spot_oracle` on `:5435`; checkpoint gRPC ingest |
| Fullnode gRPC | `127.0.0.1:9000` | `SubscribeCheckpoints` for SPoT ingest |
| Social-server | `127.0.0.1:9126` | Legacy pending-posts (compat only) |
| GraphQL | `127.0.0.1:9125/graphql` | Session refresh |

## Secrets matrix

| Secret | Consumer | Header / env |
|--------|----------|--------------|
| `DISCOVERY_EMBED_SECRET` | discovery → PoC embed | `Authorization: Bearer …` |
| `DISCOVERY_ADMIN_SECRET` | admin routes | `x-discovery-admin-secret` |
| `DISCOVERY_CLIENT_SECRET` | SPoT → `/v1/*` | `x-discovery-client-secret` |
| `SPOT_ORACLE_DISCOVERY_CLIENT_SECRET` | SPoT client | same header value as `DISCOVERY_CLIENT_SECRET` |
| `SPOT_ORACLE_SYNC_SECRET` / `SPOT_ORACLE_SOCIAL_SYNC_SECRET` | social-server ↔ spot-oracle (legacy HTTP ingest) | `x-spot-oracle-sync-secret` |

## Scripts

| Script | Corpus | Proves |
|--------|--------|--------|
| `./scripts/discovery-runnable.sh` | factual | Live RSS/GitHub/HTTP → assets with `content_hash` + `source_id` + `content_kind=text` (embed off) |
| `./scripts/discovery-poc-runnable.sh` | media | Curated images → embed → `indexed` + lifecycle → `matched` |
| `./scripts/run-discovery-service.sh` | session | Dev boot from session env |
| `./scripts/spot-oracle-runnable.sh --run-all` | factual + Discovery client | Off-chain review → evidence |
| `./scripts/spot-oracle-runnable.sh --run-all-onchain` | checkpoint ingest + chain PTBs | SubscribeCheckpoints → review → resolve |

## Local development

```bash
# Factual fetch-only E2E
KEEP_STACK=1 ./scripts/discovery-runnable.sh

# PoC stack (from proof-of-creativity)
cd ../proof-of-creativity
docker compose --profile app up postgres redis api oracle-worker

# Media embed + lifecycle E2E (same secret; override port if needed)
DISCOVERY_EMBED_SECRET=devsecret \
DISCOVERY_EMBED_ENDPOINT=http://127.0.0.1:8001/internal/discovery/embed \
./scripts/discovery-poc-runnable.sh
```

## Environment

See `network.config/discovery/discovery-session.env` for local defaults.
Set `DISCOVERY_CLIENT_SECRET` before exposing `/v1/*` outside localnet.
