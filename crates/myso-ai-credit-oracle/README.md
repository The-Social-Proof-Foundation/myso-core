# MySo AI credit oracle

The inference gateway requires PostgreSQL. `AI_CREDIT_DATABASE_URL` is the authoritative idempotency ledger and transactional reserve/capture/cancel outbox; the former `*.reservations.json` journal is no longer read or written.

Required durability configuration:

```bash
AI_CREDIT_DATABASE_URL=postgres://user:password@host:5432/myso_ai_credit
AI_CREDIT_DATABASE_MAX_CONNECTIONS=10
AI_CREDIT_OUTBOX_LEASE_SECS=60
AI_CREDIT_REPLICA_COUNT=2
AI_CREDIT_LEGACY_USAGE_ENABLED=false
```

All replicas for one deployment must share the same database. Startup runs the embedded migration in `migrations/`. Per-balance PostgreSQL advisory locks serialize reservation nonce allocation, and reconciliation workers claim outbox actions with expiring leases and `FOR UPDATE SKIP LOCKED`.

The legacy `/usage`, `/usage-history`, and manual settlement endpoints still use the older receipt store and are removed from the router in multi-replica mode. Startup rejects `AI_CREDIT_REPLICA_COUNT > 1` unless `AI_CREDIT_LEGACY_USAGE_ENABLED=false`, preventing an unsafe mixed deployment.

Inference never starts before the reserve transaction reaches finality. Provider success and its capture/cancel outbox action are committed atomically. Ambiguous provider failures remain fail-closed until hard expiry rather than issuing an unsafe cancellation or repeating inference.

## OpenAI-compatible provider (OpenClaw / Hermes)

When the provider mapping is fully configured, the oracle also mounts:

- `GET /v1/models`
- `POST /v1/chat/completions`
- `POST /v1/responses`

```bash
AI_CREDIT_INFERENCE_ENABLED=true
AI_CREDIT_OPENROUTER_API_KEY=sk-or-...
AI_CREDIT_PROVIDER_TOKEN=local-openclaw-token
AI_CREDIT_PROVIDER_OWNER=0x...
AI_CREDIT_PROVIDER_BALANCE_ID=0x...
AI_CREDIT_PROVIDER_MEMORY_ACCOUNT_ID=0x...
AI_CREDIT_PROVIDER_AGENT_OBJECT_ID=0x...
AI_CREDIT_PROVIDER_MODELS=openai/gpt-4o-mini,openai/gpt-4o
```

These routes accept standard `Authorization: Bearer <token>` auth, translate OpenAI request bodies into the internal inference path, and return OpenAI-compatible JSON. They share `run_inference_core` with `POST /v1/ai-credit/inference` (which remains authenticated by `x-ai-credit-oracle-secret` for the Memory relayer / SDK).
