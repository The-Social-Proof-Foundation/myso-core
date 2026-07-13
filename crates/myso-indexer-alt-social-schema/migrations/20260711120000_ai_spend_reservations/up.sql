ALTER TABLE ai_credit_balances
    ADD COLUMN IF NOT EXISTS reserved_mist BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS reservation_nonce BIGINT NOT NULL DEFAULT 0;

ALTER TABLE ai_credit_agent_budgets
    ADD COLUMN IF NOT EXISTS reserved_mist BIGINT NOT NULL DEFAULT 0;

-- AI credit is a product balance backed by exact MIST; there is no separate,
-- lossy whole-token "credit" accounting unit.
ALTER TABLE ai_credit_events
    DROP COLUMN IF EXISTS credits,
    DROP COLUMN IF EXISTS credits_remaining;

CREATE TABLE IF NOT EXISTS ai_spend_reservations (
    balance_id TEXT NOT NULL REFERENCES ai_credit_balances(balance_id),
    reservation_nonce BIGINT NOT NULL,
    agent_object_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'reserved',
    max_amount_mist BIGINT NOT NULL,
    captured_mist BIGINT,
    released_mist BIGINT,
    provider_envelope_hash_hex TEXT NOT NULL,
    request_hash_hex TEXT NOT NULL,
    fx_quote_id_hex TEXT NOT NULL,
    myso_usd_e8 BIGINT NOT NULL,
    markup_bps BIGINT NOT NULL,
    provider_cost_usd_micros BIGINT,
    provider_generation_hash_hex TEXT,
    capture_deadline_ms BIGINT NOT NULL,
    hard_expiry_ms BIGINT NOT NULL,
    available_mist BIGINT NOT NULL,
    reserve_event_id TEXT NOT NULL UNIQUE,
    reserve_transaction_id TEXT NOT NULL,
    terminal_event_id TEXT UNIQUE,
    terminal_transaction_id TEXT,
    terminal_at_ms BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (balance_id, reservation_nonce),
    CONSTRAINT ai_spend_reservations_status_check
        CHECK (status IN ('reserved', 'captured', 'cancelled', 'expired')),
    CONSTRAINT ai_spend_reservations_amounts_check
        CHECK (
            max_amount_mist > 0
            AND COALESCE(captured_mist, 0) >= 0
            AND COALESCE(released_mist, 0) >= 0
            AND COALESCE(captured_mist, 0) + COALESCE(released_mist, 0) <= max_amount_mist
        )
);

CREATE INDEX IF NOT EXISTS idx_ai_spend_reservations_balance_status
    ON ai_spend_reservations (balance_id, status, reservation_nonce DESC);
CREATE INDEX IF NOT EXISTS idx_ai_spend_reservations_agent_status
    ON ai_spend_reservations (agent_object_id, status, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_spend_reservations_expiry
    ON ai_spend_reservations (hard_expiry_ms)
    WHERE status = 'reserved';

COMMENT ON TABLE ai_spend_reservations IS
    'Chain-authoritative AI spend reservation lifecycle and provider/FX billing evidence.';
COMMENT ON COLUMN ai_credit_balances.balance_mist IS
    'Total on-chain MIST held. Spendable amount is balance_mist - reserved_mist.';
COMMENT ON COLUMN ai_credit_balances.reserved_mist IS
    'MIST locked by live AI spend reservations.';
