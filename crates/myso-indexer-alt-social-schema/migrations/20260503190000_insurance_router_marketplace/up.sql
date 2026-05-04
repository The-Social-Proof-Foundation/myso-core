-- Insurance marketplace: vault status columns, policy routing + backstop sweep, routed coverage tables

ALTER TABLE insurance_vaults
    ADD COLUMN IF NOT EXISTS max_exposure_per_option BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS enabled BOOLEAN NOT NULL DEFAULT TRUE,
    ADD COLUMN IF NOT EXISTS paused BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE insurance_policies
    ADD COLUMN IF NOT EXISTS route_id TEXT,
    ADD COLUMN IF NOT EXISTS route_leg_index SMALLINT,
    ADD COLUMN IF NOT EXISTS backstop_sweep_amount BIGINT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_insurance_policies_route_id ON insurance_policies(route_id);

CREATE TABLE IF NOT EXISTS insurance_coverage_routes (
    route_id TEXT NOT NULL PRIMARY KEY,
    insured TEXT NOT NULL,
    market_id TEXT NOT NULL,
    option_id SMALLINT NOT NULL,
    coverage_bps BIGINT NOT NULL,
    duration_ms BIGINT NOT NULL,
    total_covered BIGINT NOT NULL,
    total_premium BIGINT NOT NULL,
    total_reserve BIGINT NOT NULL,
    total_backstop_sweep BIGINT NOT NULL,
    expiry_time_ms BIGINT NOT NULL,
    policy_ids JSONB NOT NULL,
    vault_ids JSONB NOT NULL,
    transaction_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_insurance_coverage_routes_market_id ON insurance_coverage_routes(market_id);
CREATE INDEX IF NOT EXISTS idx_insurance_coverage_routes_insured ON insurance_coverage_routes(insured);
CREATE INDEX IF NOT EXISTS idx_insurance_coverage_routes_transaction_id ON insurance_coverage_routes(transaction_id);

CREATE TABLE IF NOT EXISTS insurance_route_fills (
    id BIGSERIAL PRIMARY KEY,
    route_id TEXT NOT NULL,
    leg_index SMALLINT NOT NULL,
    vault_id TEXT NOT NULL,
    policy_id TEXT NOT NULL,
    covered_amount BIGINT NOT NULL,
    premium_paid BIGINT NOT NULL,
    reserve_locked BIGINT NOT NULL,
    backstop_sweep_amount BIGINT NOT NULL,
    event_id TEXT NOT NULL UNIQUE,
    transaction_id TEXT NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_insurance_route_fills_route_id ON insurance_route_fills(route_id);
CREATE INDEX IF NOT EXISTS idx_insurance_route_fills_policy_id ON insurance_route_fills(policy_id);
