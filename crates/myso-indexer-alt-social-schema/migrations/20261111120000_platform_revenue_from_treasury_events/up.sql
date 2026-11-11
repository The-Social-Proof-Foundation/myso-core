-- Platform revenue is whatever the chain deposited into the platform treasury.
-- Every platform fee (SPT, subscriptions, MyData, messaging, promotions, SPoT, agent tasks,
-- governance forfeits) goes through `platform::deposit_to_treasury`, which emits
-- `PlatformTreasuryFundedEvent` with the exact platform id. The previous views relied on
-- `unified_revenue.platform_address`, which the indexer infers from post/badge links and which
-- is often NULL or a different platform, so treasury-funded fees never showed up.

CREATE INDEX IF NOT EXISTS idx_platform_events_type_platform_time
    ON platform_events (event_type, platform_id, created_at DESC);

-- Joins below match rows from the same transaction; transaction ids are `<digest>` or
-- `<digest>:<suffix>`, so match on the digest part.
CREATE INDEX IF NOT EXISTS idx_unified_revenue_tx_digest
    ON unified_revenue ((split_part(transaction_id, ':', 1)));
CREATE INDEX IF NOT EXISTS idx_spt_reservations_tx_digest
    ON spt_reservations ((split_part(transaction_id, ':', 1)));
CREATE INDEX IF NOT EXISTS idx_spt_transactions_tx_digest
    ON spt_transactions ((split_part(transaction_id, ':', 1)));

CREATE OR REPLACE VIEW platform_treasury_inflows AS
SELECT
    pe.platform_id AS platform_address,
    split_part(pe.event_id, ':', 1) AS tx_digest,
    (pe.event_data ->> 'amount')::bigint AS amount,
    CASE
        WHEN UPPER(pe.event_data ->> 'coin_type') IN ('MYSO', '0X2::MYSO::MYSO')
             OR (pe.event_data ->> 'coin_type') ILIKE '%::myso::MYSO'
        THEN '0x2::myso::MYSO'
        ELSE pe.event_data ->> 'coin_type'
    END AS currency,
    pe.event_data ->> 'funded_by' AS payer_address,
    src.creator_address,
    COALESCE(
        src.revenue_source,
        CASE
            WHEN EXISTS (
                SELECT 1 FROM spt_reservations r
                WHERE split_part(r.transaction_id, ':', 1) = split_part(pe.event_id, ':', 1)
            ) OR EXISTS (
                SELECT 1 FROM spt_transactions t
                WHERE split_part(t.transaction_id, ':', 1) = split_part(pe.event_id, ':', 1)
            )
            THEN 'spt'
        END,
        'other'
    ) AS revenue_source,
    pe.created_at AT TIME ZONE 'UTC' AS time
FROM platform_events pe
LEFT JOIN LATERAL (
    SELECT ur.revenue_source, ur.creator_address
    FROM unified_revenue ur
    WHERE split_part(ur.transaction_id, ':', 1) = split_part(pe.event_id, ':', 1)
    ORDER BY (ur.revenue_type = 'platform_fee') DESC, ur.time
    LIMIT 1
) src ON TRUE
WHERE pe.event_type = 'PlatformTreasuryFunded'
  AND pe.event_id IS NOT NULL;

COMMENT ON VIEW platform_treasury_inflows IS
    'One row per PlatformTreasuryFundedEvent, classified by the revenue rows in the same transaction';

DROP VIEW IF EXISTS platform_revenue_summary;
CREATE VIEW platform_revenue_summary AS
SELECT
    platform_address,
    SUM(amount)::bigint AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END)::bigint AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END)::bigint AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END)::bigint AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'messaging' THEN amount ELSE 0 END)::bigint AS total_messaging_revenue,
    SUM(CASE WHEN revenue_source = 'username_marketplace' THEN amount ELSE 0 END)::bigint AS total_username_marketplace_revenue,
    COUNT(*)::bigint AS total_transactions,
    COUNT(DISTINCT creator_address)::bigint AS total_creators,
    COUNT(DISTINCT payer_address)::bigint AS total_payers,
    AVG(amount)::float8 AS avg_transaction_amount,
    COUNT(DISTINCT DATE_TRUNC('month', time))::bigint AS active_months,
    DATE_TRUNC('month', MAX(time))::DATE AS last_active_month
FROM platform_treasury_inflows
WHERE time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address;

DROP VIEW IF EXISTS platform_revenue_by_source_currency;
CREATE VIEW platform_revenue_by_source_currency AS
SELECT
    platform_address,
    revenue_source,
    currency,
    SUM(amount)::bigint AS total_amount,
    COUNT(*)::bigint AS transaction_count
FROM platform_treasury_inflows
WHERE time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address, revenue_source, currency;

COMMENT ON VIEW platform_revenue_by_source_currency IS
    'Platform treasury inflows grouped by revenue_source and normalized currency (12-month window)';
