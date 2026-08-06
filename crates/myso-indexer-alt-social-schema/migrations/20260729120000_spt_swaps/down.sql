ALTER TABLE spt_transactions DROP COLUMN IF EXISTS is_swap_leg;
ALTER TABLE spt_transactions DROP COLUMN IF EXISTS counterparty_pool_id;

DROP TABLE IF EXISTS spt_swaps CASCADE;
