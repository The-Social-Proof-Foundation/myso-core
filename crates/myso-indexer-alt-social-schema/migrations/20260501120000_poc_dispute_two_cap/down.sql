ALTER TABLE poc_disputes DROP COLUMN IF EXISTS quorum_met;
ALTER TABLE poc_disputes DROP COLUMN IF EXISTS required_total_stake_quorum;
ALTER TABLE poc_disputes DROP COLUMN IF EXISTS effective_dispute_fee;
ALTER TABLE poc_disputes DROP COLUMN IF EXISTS dispute_round;

ALTER TABLE poc_config DROP COLUMN IF EXISTS dispute_second_round_quorum_multiplier_bps;
ALTER TABLE poc_config DROP COLUMN IF EXISTS dispute_second_round_fee_multiplier_bps;
ALTER TABLE poc_config DROP COLUMN IF EXISTS dispute_quorum_base_stake;

ALTER TABLE posts DROP COLUMN IF EXISTS poc_disputes_submitted;
