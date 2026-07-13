CREATE TABLE IF NOT EXISTS ai_inference_reservations (
    idempotency_scope TEXT PRIMARY KEY,
    idempotency_key TEXT NOT NULL,
    owner_address TEXT NOT NULL,
    balance_id TEXT NOT NULL,
    memory_account_id TEXT NOT NULL,
    agent_object_id TEXT NOT NULL,
    model_id TEXT NOT NULL,
    reservation_nonce BIGINT NOT NULL CHECK (reservation_nonce > 0),
    max_amount_mist BIGINT NOT NULL CHECK (max_amount_mist >= 0),
    provider_envelope_hash_hex TEXT NOT NULL,
    request_hash_hex TEXT NOT NULL,
    fx_quote_id_hex TEXT NOT NULL,
    myso_usd_e8 BIGINT NOT NULL CHECK (myso_usd_e8 > 0),
    markup_bps BIGINT NOT NULL CHECK (markup_bps >= 0),
    capture_deadline_ms BIGINT NOT NULL,
    hard_expiry_ms BIGINT NOT NULL,
    status TEXT NOT NULL,
    reserve_digest TEXT,
    capture_digest TEXT,
    cancel_digest TEXT,
    amount_mist BIGINT CHECK (amount_mist >= 0),
    provider_cost_usd_micros BIGINT CHECK (provider_cost_usd_micros >= 0),
    upstream_cost_usd_micros BIGINT CHECK (upstream_cost_usd_micros >= 0),
    provider_generation_id TEXT,
    content TEXT,
    tokens_in BIGINT CHECK (tokens_in >= 0),
    tokens_out BIGINT CHECK (tokens_out >= 0),
    last_error TEXT,
    created_at_ms BIGINT NOT NULL,
    updated_at_ms BIGINT NOT NULL,
    CONSTRAINT ai_inference_status_check CHECK (
        status IN (
            'preparing', 'reserved', 'provider_succeeded', 'captured',
            'cancelled', 'ambiguous_provider_failure'
        )
    ),
    CONSTRAINT ai_inference_idempotency_unique UNIQUE (
        balance_id, agent_object_id, idempotency_key
    ),
    CONSTRAINT ai_inference_nonce_unique UNIQUE (balance_id, reservation_nonce),
    CONSTRAINT ai_inference_deadline_order CHECK (hard_expiry_ms > capture_deadline_ms)
);

CREATE TABLE IF NOT EXISTS ai_inference_outbox (
    id BIGSERIAL PRIMARY KEY,
    idempotency_scope TEXT NOT NULL REFERENCES ai_inference_reservations(idempotency_scope),
    action TEXT NOT NULL CHECK (action IN ('reserve', 'capture', 'cancel')),
    state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'processing', 'delivered')),
    payload JSONB NOT NULL,
    attempt_count INTEGER NOT NULL DEFAULT 0,
    lease_owner TEXT,
    lease_expires_at TIMESTAMPTZ,
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_digest TEXT,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ai_inference_outbox_action_unique UNIQUE (idempotency_scope, action)
);

CREATE INDEX IF NOT EXISTS idx_ai_inference_outbox_claim
    ON ai_inference_outbox (next_attempt_at, id)
    WHERE state <> 'delivered';

CREATE INDEX IF NOT EXISTS idx_ai_inference_incomplete
    ON ai_inference_reservations (updated_at_ms)
    WHERE status NOT IN ('captured', 'cancelled');
