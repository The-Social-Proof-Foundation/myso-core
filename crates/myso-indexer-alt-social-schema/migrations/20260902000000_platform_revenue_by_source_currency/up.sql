-- Per-currency platform revenue so MYSO/MYUSD/BTC/ETH amounts are not summed together.

CREATE OR REPLACE VIEW platform_revenue_by_source_currency AS
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

COMMENT ON VIEW platform_revenue_by_source_currency IS
    'Platform fee inflows grouped by revenue_source and normalized currency (12-month window)';
