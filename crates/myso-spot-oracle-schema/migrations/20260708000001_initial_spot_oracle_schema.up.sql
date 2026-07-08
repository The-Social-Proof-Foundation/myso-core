-- SPoT oracle operational schema (spot_oracle DB).
-- discovery_sources is provided by myso-discovery-service-schema (run first); SPoT reads it only.
-- evidence is resolver-owned and has NO FK to discovery_assets (independence rule).

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE markets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id VARCHAR(128) NOT NULL UNIQUE,
    spot_record_id VARCHAR(128),
    creator VARCHAR(128),
    claim_text TEXT NOT NULL,
    betting_options JSONB NOT NULL DEFAULT '[]',
    status VARCHAR(32) NOT NULL DEFAULT 'pending_review',
    -- pending_review -> pending_create -> active -> resolving -> resolved -> refunded -> rejected
    review_id UUID,
    resolver_definition_id UUID,
    resolution_window_ms BIGINT NOT NULL DEFAULT 86400000,
    max_resolution_window_ms BIGINT NOT NULL DEFAULT 604800000,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_markets_status ON markets(status, created_at);
CREATE INDEX idx_markets_spot_record ON markets(spot_record_id) WHERE spot_record_id IS NOT NULL;

CREATE TABLE llm_extractions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id VARCHAR(128) NOT NULL,
    raw_response TEXT,
    parsed JSONB NOT NULL DEFAULT '{}',
    model VARCHAR(128),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_llm_extractions_post ON llm_extractions(post_id, created_at DESC);

CREATE TABLE canonical_claims (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    llm_extraction_id UUID NOT NULL REFERENCES llm_extractions(id) ON DELETE CASCADE,
    normalized_fields JSONB NOT NULL DEFAULT '{}',
    claim_hash VARCHAR(128) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_canonical_claims_hash ON canonical_claims(claim_hash);

CREATE TABLE oracle_reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id VARCHAR(128) NOT NULL,
    canonical_claim_id UUID REFERENCES canonical_claims(id),
    decision VARCHAR(32) NOT NULL,
    reject_reason TEXT,
    reviewer VARCHAR(64) NOT NULL DEFAULT 'rule_engine',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_oracle_reviews_post ON oracle_reviews(post_id, created_at DESC);
CREATE INDEX idx_oracle_reviews_decision ON oracle_reviews(decision);

CREATE TABLE resolver_definitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    canonical_claim_id UUID NOT NULL REFERENCES canonical_claims(id) ON DELETE CASCADE,
    resolver_kind VARCHAR(64) NOT NULL,
    -- Immutable execution spec: thresholds, URLs, JSON paths, option mappings — no free text.
    spec JSONB NOT NULL DEFAULT '{}',
    source_ids JSONB NOT NULL DEFAULT '[]',
    betting_options JSONB NOT NULL DEFAULT '[]',
    maturity_schedule JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_resolver_definitions_claim ON resolver_definitions(canonical_claim_id);

CREATE TABLE resolver_state (
    market_id UUID PRIMARY KEY REFERENCES markets(id) ON DELETE CASCADE,
    last_poll_at TIMESTAMPTZ,
    maturity_at TIMESTAMPTZ,
    outcome_draft VARCHAR(64),
    confidence_bps INTEGER,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE spot_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_type VARCHAR(32) NOT NULL,
    market_id UUID REFERENCES markets(id) ON DELETE CASCADE,
    resolver_definition_id UUID REFERENCES resolver_definitions(id) ON DELETE CASCADE,
    priority_score BIGINT NOT NULL DEFAULT 0,
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 5,
    run_after TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_error TEXT,
    payload JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_spot_jobs_claim ON spot_jobs(status, priority_score DESC, created_at);
CREATE INDEX idx_spot_jobs_run_after ON spot_jobs(status, run_after) WHERE status = 'pending';

CREATE TABLE evidence (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    market_id UUID NOT NULL REFERENCES markets(id) ON DELETE CASCADE,
    resolver_job_id UUID REFERENCES spot_jobs(id) ON DELETE SET NULL,
    adapter_id VARCHAR(64) NOT NULL,
    source_url TEXT NOT NULL,
    content_hash VARCHAR(128) NOT NULL,
    raw_response TEXT,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_evidence_market ON evidence(market_id, fetched_at DESC);

CREATE TABLE source_health (
    adapter_id VARCHAR(64) PRIMARY KEY,
    healthy BOOLEAN NOT NULL DEFAULT true,
    latency_ms INTEGER,
    error_count INTEGER NOT NULL DEFAULT 0,
    circuit_state VARCHAR(16) NOT NULL DEFAULT 'closed',
    last_probe_at TIMESTAMPTZ,
    last_error TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    market_id UUID REFERENCES markets(id) ON DELETE CASCADE,
    tx_kind VARCHAR(32) NOT NULL,
    -- Idempotency key: (market_id, tx_kind, nonce) unique per attempt family.
    nonce VARCHAR(64) NOT NULL,
    digest VARCHAR(128),
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    submitted_at TIMESTAMPTZ,
    confirmed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_transactions_dedup ON transactions(market_id, tx_kind, nonce);

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    actor VARCHAR(128) NOT NULL,
    action VARCHAR(64) NOT NULL,
    target_type VARCHAR(32),
    target_id UUID,
    details JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_target ON audit_logs(target_type, target_id, created_at DESC);

CREATE TABLE retries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type VARCHAR(32) NOT NULL,
    entity_id UUID NOT NULL,
    attempt INTEGER NOT NULL DEFAULT 0,
    next_run_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_retries_next_run ON retries(entity_type, next_run_at);
