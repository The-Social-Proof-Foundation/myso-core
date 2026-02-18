-- Revert spt_revenue transaction_type CHECK to original lowercase values
ALTER TABLE spt_revenue DROP CONSTRAINT IF EXISTS spt_revenue_transaction_type_check;

ALTER TABLE spt_revenue ADD CONSTRAINT spt_revenue_transaction_type_check
    CHECK (transaction_type IN ('buy', 'sell'));
