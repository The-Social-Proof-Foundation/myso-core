-- Materialized platform treasury balances (hybrid with on-chain RPC validation)
CREATE TABLE IF NOT EXISTS platform_treasury_balances (
    platform_id TEXT PRIMARY KEY,
    balance_mist BIGINT NOT NULL DEFAULT 0,
    last_funded_at BIGINT,
    last_withdrawn_at BIGINT,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_platform_treasury_balances_updated_at
    ON platform_treasury_balances (updated_at DESC);
