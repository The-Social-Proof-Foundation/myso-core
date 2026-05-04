ALTER TABLE insurance_policy_events DROP COLUMN IF EXISTS option_escrow_amount;
ALTER TABLE insurance_policy_events DROP COLUMN IF EXISTS market_total_amount;
ALTER TABLE insurance_policy_events DROP COLUMN IF EXISTS base_premium;
ALTER TABLE insurance_policy_events DROP COLUMN IF EXISTS risk_multiplier_bps;
ALTER TABLE insurance_policy_events DROP COLUMN IF EXISTS implied_probability_bps;
ALTER TABLE insurance_policy_events DROP COLUMN IF EXISTS premium_raw;

ALTER TABLE insurance_policies DROP COLUMN IF EXISTS option_escrow_amount;
ALTER TABLE insurance_policies DROP COLUMN IF EXISTS market_total_amount;
ALTER TABLE insurance_policies DROP COLUMN IF EXISTS base_premium;
ALTER TABLE insurance_policies DROP COLUMN IF EXISTS risk_multiplier_bps;
ALTER TABLE insurance_policies DROP COLUMN IF EXISTS implied_probability_bps;
ALTER TABLE insurance_policies DROP COLUMN IF EXISTS premium_raw;
