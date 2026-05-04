-- SPoT-aware insurance premiums: persisted pricing metadata on policies and purchased policy events.

ALTER TABLE insurance_policies
    ADD COLUMN IF NOT EXISTS premium_raw BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS implied_probability_bps BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS risk_multiplier_bps BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS base_premium BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS market_total_amount BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS option_escrow_amount BIGINT NOT NULL DEFAULT 0;

ALTER TABLE insurance_policy_events
    ADD COLUMN IF NOT EXISTS premium_raw BIGINT,
    ADD COLUMN IF NOT EXISTS implied_probability_bps BIGINT,
    ADD COLUMN IF NOT EXISTS risk_multiplier_bps BIGINT,
    ADD COLUMN IF NOT EXISTS base_premium BIGINT,
    ADD COLUMN IF NOT EXISTS market_total_amount BIGINT,
    ADD COLUMN IF NOT EXISTS option_escrow_amount BIGINT;
