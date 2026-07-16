-- Platform treasury withdrawal history
CREATE TABLE IF NOT EXISTS platform_treasury_withdrawals (
    id SERIAL PRIMARY KEY,
    platform_id TEXT NOT NULL,
    recipient TEXT NOT NULL,
    amount BIGINT NOT NULL,
    reason_code SMALLINT NOT NULL,
    executed_by TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    event_id TEXT
);

CREATE INDEX IF NOT EXISTS idx_platform_treasury_withdrawals_platform_id
    ON platform_treasury_withdrawals (platform_id);
CREATE INDEX IF NOT EXISTS idx_platform_treasury_withdrawals_recipient
    ON platform_treasury_withdrawals (recipient);
CREATE INDEX IF NOT EXISTS idx_platform_treasury_withdrawals_timestamp
    ON platform_treasury_withdrawals (timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_platform_treasury_withdrawals_executed_by
    ON platform_treasury_withdrawals (executed_by);
CREATE INDEX IF NOT EXISTS idx_platform_treasury_withdrawals_platform_timestamp
    ON platform_treasury_withdrawals (platform_id, timestamp DESC);
