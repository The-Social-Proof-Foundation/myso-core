# Discovery Service Architecture

See also `proof-of-creativity/docs/discovery-architecture.md` for cross-repo integration.

## Modules

- `sources/` — `DiscoverySource` trait + adapter registry (mirrors SPoT `TrustedSource` pattern)
- `lifecycle/` — asset FSM (`DISCOVERED` → `CLAIMED`)
- `scheduler/` — poll adapters, priority queue, embed worker loop
- `embed_client/` — delegates ML embedding to PoC `POST /internal/discovery/embed`
- `store/` — SQLx repositories for discovery DB

## Local development

```bash
# Discovery DB + service (from myso-core)
docker compose -f crates/myso-discovery-service/docker-compose.yml up

# PoC stack with embed endpoint
cd ../proof-of-creativity
docker compose --profile app up postgres redis api oracle-worker
```

## Environment

| Variable | Purpose |
|----------|---------|
| `DISCOVERY_DATABASE_URL` | Discovery Postgres (separate from PoC pgvector DB) |
| `DISCOVERY_EMBED_ENDPOINT` | PoC internal embed URL |
| `DISCOVERY_EMBED_SECRET` | Shared secret with PoC |
| `DISCOVERY_ACTIVE_EMBEDDING_VERSION` | Active corpus version filter |
