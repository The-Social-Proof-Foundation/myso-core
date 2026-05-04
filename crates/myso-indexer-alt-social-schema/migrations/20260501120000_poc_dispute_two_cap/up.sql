-- PoC: lifetime max two disputes per post + second-round fee/quorum config + dispute snapshots

ALTER TABLE posts
    ADD COLUMN IF NOT EXISTS poc_disputes_submitted SMALLINT NOT NULL DEFAULT 0;

COMMENT ON COLUMN posts.poc_disputes_submitted IS 'Successful PoC dispute submissions for this post (max 2); not reset when PoC cleared';

ALTER TABLE poc_configuration
    ADD COLUMN IF NOT EXISTS dispute_quorum_base_stake BIGINT NOT NULL DEFAULT 0;

ALTER TABLE poc_configuration
    ADD COLUMN IF NOT EXISTS dispute_second_round_fee_multiplier_bps BIGINT NOT NULL DEFAULT 10000;

ALTER TABLE poc_configuration
    ADD COLUMN IF NOT EXISTS dispute_second_round_quorum_multiplier_bps BIGINT NOT NULL DEFAULT 10000;

COMMENT ON COLUMN poc_configuration.dispute_quorum_base_stake IS 'Round-1 minimum total voting stake for full dispute resolution (0 = disabled)';
COMMENT ON COLUMN poc_configuration.dispute_second_round_fee_multiplier_bps IS 'Round-2 dispute fee = dispute_cost * bps / 10000; must be >= 10000 on-chain';
COMMENT ON COLUMN poc_configuration.dispute_second_round_quorum_multiplier_bps IS 'Round-2 quorum = base * bps / 10000; must be >= 10000 on-chain';

ALTER TABLE poc_disputes
    ADD COLUMN IF NOT EXISTS dispute_round SMALLINT NOT NULL DEFAULT 1;

ALTER TABLE poc_disputes
    ADD COLUMN IF NOT EXISTS effective_dispute_fee BIGINT NOT NULL DEFAULT 0;

ALTER TABLE poc_disputes
    ADD COLUMN IF NOT EXISTS required_total_stake_quorum BIGINT NOT NULL DEFAULT 0;

ALTER TABLE poc_disputes
    ADD COLUMN IF NOT EXISTS quorum_met BOOLEAN NULL;

COMMENT ON COLUMN poc_disputes.dispute_round IS '1 = first dispute on post, 2 = second';
COMMENT ON COLUMN poc_disputes.effective_dispute_fee IS 'MYSO fee charged when dispute opened';
COMMENT ON COLUMN poc_disputes.required_total_stake_quorum IS 'Minimum uphold+overturn stake for stake-weighted outcome';
COMMENT ON COLUMN poc_disputes.quorum_met IS 'Set on resolution; false when defaulted to uphold for insufficient stake';

UPDATE poc_disputes
SET effective_dispute_fee = stake_amount
WHERE effective_dispute_fee = 0 AND stake_amount IS NOT NULL;
