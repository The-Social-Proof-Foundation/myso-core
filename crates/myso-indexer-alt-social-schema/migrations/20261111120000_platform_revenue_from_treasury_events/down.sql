DROP VIEW IF EXISTS platform_revenue_by_source_currency;
DROP VIEW IF EXISTS platform_revenue_summary;
DROP VIEW IF EXISTS platform_treasury_inflows;

DROP INDEX IF EXISTS idx_spt_transactions_tx_digest;
DROP INDEX IF EXISTS idx_spt_reservations_tx_digest;
DROP INDEX IF EXISTS idx_unified_revenue_tx_digest;
DROP INDEX IF EXISTS idx_platform_events_type_platform_time;

CREATE VIEW platform_revenue_summary AS
SELECT
    platform_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'messaging' THEN amount ELSE 0 END) AS total_messaging_revenue,
    SUM(CASE WHEN revenue_source = 'username_marketplace' THEN amount ELSE 0 END) AS total_username_marketplace_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT creator_address) AS total_creators,
    COUNT(DISTINCT payer_address) AS total_payers,
    AVG(amount) AS avg_transaction_amount,
    COUNT(DISTINCT DATE_TRUNC('month', time)) AS active_months,
    DATE_TRUNC('month', MAX(time))::DATE AS last_active_month
FROM unified_revenue
WHERE platform_address IS NOT NULL
    AND time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address
ORDER BY total_revenue DESC;

CREATE VIEW platform_revenue_by_source_currency AS
SELECT
    platform_address,
    revenue_source,
    currency,
    SUM(amount) AS total_amount,
    COUNT(*) AS transaction_count
FROM (
    SELECT
        platform_address,
        revenue_source,
        CASE
            WHEN UPPER(currency) IN ('MYSO', '0X2::MYSO::MYSO')
                 OR currency ILIKE '%::myso::MYSO'
            THEN '0x2::myso::MYSO'
            ELSE currency
        END AS currency,
        amount
    FROM unified_revenue
    WHERE platform_address IS NOT NULL
      AND time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
) normalized
GROUP BY platform_address, revenue_source, currency;
