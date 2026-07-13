DROP TABLE IF EXISTS ai_spend_reservations;

ALTER TABLE ai_credit_events
    ADD COLUMN IF NOT EXISTS credits BIGINT,
    ADD COLUMN IF NOT EXISTS credits_remaining BIGINT;

ALTER TABLE ai_credit_agent_budgets
    DROP COLUMN IF EXISTS reserved_mist;

ALTER TABLE ai_credit_balances
    DROP COLUMN IF EXISTS reservation_nonce,
    DROP COLUMN IF EXISTS reserved_mist;
