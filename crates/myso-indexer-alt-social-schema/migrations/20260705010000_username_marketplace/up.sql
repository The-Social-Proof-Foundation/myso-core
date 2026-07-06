-- Greenfield username marketplace: replace profile sale tables with username marketplace tables.

DROP TABLE IF EXISTS profile_sale_fees CASCADE;
DROP TABLE IF EXISTS profile_offers CASCADE;

DROP INDEX IF EXISTS idx_profiles_min_offer_amount;
DROP INDEX IF EXISTS idx_profiles_owner_min_offer;
ALTER TABLE profiles DROP COLUMN IF EXISTS min_offer_amount;

ALTER TABLE profile_config RENAME COLUMN profile_sale_fee_bps TO username_sale_fee_bps;

COMMENT ON COLUMN profile_config.username_sale_fee_bps IS 'Fee in bps taken on username marketplace sales (default: 500; 10000 = 100%)';

CREATE TABLE IF NOT EXISTS username_listings (
    id SERIAL NOT NULL,
    username TEXT NOT NULL,
    seller_address TEXT NOT NULL,
    seller_profile_id TEXT NOT NULL,
    min_price BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at BIGINT NOT NULL,
    cancelled_at BIGINT,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_username_listings PRIMARY KEY (id, time)
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_schema = 'public' AND hypertable_name = 'username_listings'
    ) THEN
        PERFORM create_hypertable('username_listings'::regclass, 'time'::name);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_username_listings_username_time ON username_listings (username, time DESC);
CREATE INDEX IF NOT EXISTS idx_username_listings_seller_time ON username_listings (seller_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_username_listings_status_time ON username_listings (status, time DESC) WHERE status = 'active';

CREATE TABLE IF NOT EXISTS username_offers (
    id SERIAL NOT NULL,
    username TEXT NOT NULL,
    seller_profile_id TEXT NOT NULL,
    buyer_address TEXT NOT NULL,
    buyer_profile_id TEXT NOT NULL,
    amount BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    resolved_at BIGINT,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_username_offers PRIMARY KEY (id, time)
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_schema = 'public' AND hypertable_name = 'username_offers'
    ) THEN
        PERFORM create_hypertable('username_offers'::regclass, 'time'::name);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_username_offers_username_time ON username_offers (username, time DESC);
CREATE INDEX IF NOT EXISTS idx_username_offers_buyer_time ON username_offers (buyer_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_username_offers_status_time ON username_offers (status, time DESC) WHERE status = 'pending';
CREATE UNIQUE INDEX IF NOT EXISTS idx_username_offers_username_buyer_unique ON username_offers (username, buyer_address, time) WHERE status = 'pending';

CREATE TABLE IF NOT EXISTS username_sale_fees (
    id SERIAL NOT NULL,
    username TEXT NOT NULL,
    seller_address TEXT NOT NULL,
    seller_profile_id TEXT NOT NULL,
    buyer_address TEXT NOT NULL,
    buyer_profile_id TEXT NOT NULL,
    sale_amount BIGINT NOT NULL,
    fee_amount BIGINT NOT NULL,
    fee_recipient_address TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_username_sale_fees PRIMARY KEY (id, time)
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_schema = 'public' AND hypertable_name = 'username_sale_fees'
    ) THEN
        PERFORM create_hypertable('username_sale_fees'::regclass, 'time'::name);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_username_sale_fees_username_time ON username_sale_fees (username, time DESC);
CREATE INDEX IF NOT EXISTS idx_username_sale_fees_seller_time ON username_sale_fees (seller_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_username_sale_fees_buyer_time ON username_sale_fees (buyer_address, time DESC);

-- Allow username marketplace (and messaging) rows in unified_revenue.
ALTER TABLE unified_revenue DROP CONSTRAINT IF EXISTS unified_revenue_revenue_source_check;
ALTER TABLE unified_revenue ADD CONSTRAINT unified_revenue_revenue_source_check
    CHECK (revenue_source IN (
        'subscription', 'mydata', 'spt', 'tips', 'posts', 'messaging', 'username_marketplace'
    ));

DROP VIEW IF EXISTS platform_revenue_summary CASCADE;
CREATE OR REPLACE VIEW platform_revenue_summary AS
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

DROP VIEW IF EXISTS spt_creator_revenue_summary CASCADE;
CREATE OR REPLACE VIEW spt_creator_revenue_summary AS
SELECT
    creator_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'tips' THEN amount ELSE 0 END) AS total_tips_revenue,
    SUM(CASE WHEN revenue_source = 'messaging' THEN amount ELSE 0 END) AS total_messaging_revenue,
    SUM(CASE WHEN revenue_source = 'username_marketplace' THEN amount ELSE 0 END) AS total_username_marketplace_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT payer_address) AS total_unique_payers,
    MAX(amount) AS largest_single_transaction,
    COUNT(DISTINCT DATE(time)) AS active_days,
    MAX(time) AS last_revenue_date
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '30 days'
GROUP BY creator_address
ORDER BY total_revenue DESC;
