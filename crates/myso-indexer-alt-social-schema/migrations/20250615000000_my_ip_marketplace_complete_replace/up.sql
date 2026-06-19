-- MYDATA MARKETPLACE - COMPLETE SYSTEM REPLACEMENT
-- Production-ready TimescaleDB-optimized information finance marketplace
-- This migration completely replaces the old licensing system

-- ============================================================================
-- 1. ENABLE TIMESCALEDB EXTENSION
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
    END IF;
END
$$;

-- ============================================================================
-- 2. DROP OLD LICENSING SYSTEM TABLES (Complete Replacement)
-- ============================================================================
-- Drop old materialized views first
DROP MATERIALIZED VIEW IF EXISTS daily_license_revenue CASCADE;
DROP MATERIALIZED VIEW IF EXISTS weekly_creator_revenue CASCADE;

-- Drop old views
DROP VIEW IF EXISTS active_licenses CASCADE;
DROP VIEW IF EXISTS popular_licenses CASCADE;

-- Drop old functions
DROP FUNCTION IF EXISTS refresh_license_materialized_views() CASCADE;

-- Drop old tables (order matters due to constraints)
DROP TABLE IF EXISTS my_ip_revenue CASCADE;
DROP TABLE IF EXISTS my_ip_grants CASCADE;
DROP TABLE IF EXISTS my_ip_events CASCADE;
DROP TABLE IF EXISTS my_ip_permissions CASCADE;
DROP TABLE IF EXISTS my_ip CASCADE;

-- Remove old columns from posts table if they exist
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'my_ip_id'
    ) THEN
        ALTER TABLE posts DROP COLUMN my_ip_id;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'revenue_recipient'
    ) THEN
        ALTER TABLE posts DROP COLUMN revenue_recipient;
    END IF;
END
$$;

-- ============================================================================
-- 3. CREATE NEW MARKETPLACE SYSTEM TABLES
-- ============================================================================

-- Main data marketplace entries (Regular table - reference data)
CREATE TABLE IF NOT EXISTS mydata_data (
    mydata_id TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL,
    media_type TEXT NOT NULL,
    tags JSONB NOT NULL DEFAULT '[]'::JSONB,
    platform_id TEXT,
    timestamp_start BIGINT NOT NULL,
    timestamp_end BIGINT,
    created_at BIGINT NOT NULL,
    last_updated BIGINT NOT NULL,
    one_time_price BIGINT,
    subscription_price BIGINT,
    subscription_duration_days BIGINT NOT NULL DEFAULT 30,
    geographic_region TEXT,
    data_quality TEXT CHECK (data_quality IN ('high', 'medium', 'low')),
    sample_size BIGINT,
    collection_method TEXT,
    is_updating BOOLEAN NOT NULL DEFAULT false,
    update_frequency TEXT CHECK (update_frequency IN ('hourly', 'daily', 'weekly', 'monthly', 'yearly')),
    version BIGINT NOT NULL DEFAULT 1,
    encrypted_content_hash TEXT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Purchase records (TimescaleDB hypertable - high volume)
CREATE TABLE IF NOT EXISTS mydata_purchases (
    id SERIAL NOT NULL,
    mydata_id TEXT NOT NULL,
    buyer TEXT NOT NULL,
    price BIGINT NOT NULL,
    purchase_type TEXT NOT NULL CHECK (purchase_type IN ('one_time', 'subscription')),
    purchase_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_mydata_purchases PRIMARY KEY (id, time)
);

-- Convert to TimescaleDB hypertable with 7-day chunks for high-volume purchase analytics
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'mydata_purchases'
    ) THEN
        PERFORM create_hypertable('mydata_purchases'::regclass, 'time'::name, chunk_time_interval => INTERVAL '7 days');
    END IF;
END $$;

-- Subscription records (TimescaleDB hypertable - moderate volume)
CREATE TABLE IF NOT EXISTS mydata_subscriptions (
    id SERIAL NOT NULL,
    mydata_id TEXT NOT NULL,
    subscriber TEXT NOT NULL,
    subscription_start BIGINT NOT NULL,
    subscription_end BIGINT NOT NULL,
    price BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_mydata_subscriptions PRIMARY KEY (id, time)
);

-- Convert to TimescaleDB hypertable with 30-day chunks for subscription billing cycles
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'mydata_subscriptions'
    ) THEN
        PERFORM create_hypertable('mydata_subscriptions'::regclass, 'time'::name, chunk_time_interval => INTERVAL '30 days');
    END IF;
END $$;

-- Revenue tracking (TimescaleDB hypertable - high volume)
CREATE TABLE IF NOT EXISTS mydata_revenue (
    id SERIAL NOT NULL,
    mydata_id TEXT NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    revenue_type TEXT NOT NULL CHECK (revenue_type IN ('one_time', 'subscription', 'grant')),
    revenue_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_mydata_revenue PRIMARY KEY (id, time)
);

-- Convert to TimescaleDB hypertable with 7-day chunks for real-time revenue tracking
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'mydata_revenue'
    ) THEN
        PERFORM create_hypertable('mydata_revenue'::regclass, 'time'::name, chunk_time_interval => INTERVAL '7 days');
    END IF;
END $$;

-- Access logs for analytics (TimescaleDB hypertable - very high volume)
CREATE TABLE IF NOT EXISTS mydata_access_logs (
    id SERIAL NOT NULL,
    mydata_id TEXT NOT NULL,
    user_address TEXT NOT NULL,
    access_type TEXT NOT NULL CHECK (access_type IN ('one_time', 'subscription', 'grant', 'preview')),
    access_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_mydata_access_logs PRIMARY KEY (id, time)
);

-- Convert to TimescaleDB hypertable with 1-day chunks for short-term analytics
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'mydata_access_logs'
    ) THEN
        PERFORM create_hypertable('mydata_access_logs'::regclass, 'time'::name, chunk_time_interval => INTERVAL '1 day');
    END IF;
END $$;

-- ============================================================================
-- 4. CREATE OPTIMIZED INDEXES FOR MARKETPLACE QUERIES
-- ============================================================================

-- mydata_data indexes (reference table)
CREATE INDEX IF NOT EXISTS idx_mydata_data_owner ON mydata_data (owner);
CREATE INDEX IF NOT EXISTS idx_mydata_data_media_type ON mydata_data (media_type);
CREATE INDEX IF NOT EXISTS idx_mydata_data_tags ON mydata_data USING GIN (tags);
CREATE INDEX IF NOT EXISTS idx_mydata_data_platform_id ON mydata_data (platform_id) WHERE platform_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_mydata_data_time ON mydata_data (time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_data_pricing ON mydata_data (one_time_price, subscription_price) WHERE one_time_price IS NOT NULL OR subscription_price IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_mydata_data_geographic ON mydata_data (geographic_region) WHERE geographic_region IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_mydata_data_quality ON mydata_data (data_quality) WHERE data_quality IS NOT NULL;

-- mydata_purchases indexes (hypertable)
CREATE INDEX IF NOT EXISTS idx_mydata_purchases_time_mydata ON mydata_purchases (time DESC, mydata_id);
CREATE INDEX IF NOT EXISTS idx_mydata_purchases_buyer_time ON mydata_purchases (buyer, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_purchases_type_time ON mydata_purchases (purchase_type, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_purchases_mydata_time ON mydata_purchases (mydata_id, time DESC);

-- mydata_subscriptions indexes (hypertable)
CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_time_mydata ON mydata_subscriptions (time DESC, mydata_id);
CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_subscriber_time ON mydata_subscriptions (subscriber, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_end_time ON mydata_subscriptions (subscription_end, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_mydata_time ON mydata_subscriptions (mydata_id, time DESC);

-- mydata_revenue indexes (hypertable)
CREATE INDEX IF NOT EXISTS idx_mydata_revenue_time_mydata ON mydata_revenue (time DESC, mydata_id);
CREATE INDEX IF NOT EXISTS idx_mydata_revenue_to_time ON mydata_revenue (to_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_revenue_type_time ON mydata_revenue (revenue_type, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_revenue_mydata_time ON mydata_revenue (mydata_id, time DESC);

-- mydata_access_logs indexes (hypertable)
CREATE INDEX IF NOT EXISTS idx_mydata_access_time_mydata ON mydata_access_logs (time DESC, mydata_id);
CREATE INDEX IF NOT EXISTS idx_mydata_access_user_time ON mydata_access_logs (user_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_access_type_time ON mydata_access_logs (access_type, time DESC);

-- ============================================================================
-- 5. ENABLE TIMESCALEDB COMPRESSION POLICIES
-- ============================================================================

-- Enable compression on hypertables with appropriate segment and order by settings
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'mydata_purchases'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress%'
    ) THEN
        ALTER TABLE mydata_purchases SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'mydata_id,purchase_type',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'mydata_subscriptions'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress%'
    ) THEN
        ALTER TABLE mydata_subscriptions SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'mydata_id,subscriber',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'mydata_revenue'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress%'
    ) THEN
        ALTER TABLE mydata_revenue SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'mydata_id,revenue_type',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'mydata_access_logs'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress%'
    ) THEN
        ALTER TABLE mydata_access_logs SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'mydata_id,access_type',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'mydata_purchases'
    ) THEN
        PERFORM add_compression_policy('mydata_purchases', INTERVAL '30 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'mydata_subscriptions'
    ) THEN
        PERFORM add_compression_policy('mydata_subscriptions', INTERVAL '90 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'mydata_revenue'
    ) THEN
        PERFORM add_compression_policy('mydata_revenue', INTERVAL '30 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'mydata_access_logs'
    ) THEN
        PERFORM add_compression_policy('mydata_access_logs', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 6. CREATE TIMESCALEDB CONTINUOUS AGGREGATES FOR ANALYTICS
-- ============================================================================

DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'mydata_daily_revenue'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW mydata_daily_revenue
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 day', time) AS day,
            mydata_id,
            to_address AS creator,
            revenue_type,
            SUM(amount) AS daily_revenue,
            COUNT(*) AS transaction_count
        FROM mydata_revenue
        GROUP BY time_bucket('1 day', time), mydata_id, to_address, revenue_type
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('mydata_daily_revenue',
            start_offset => INTERVAL '3 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'mydata_daily_access'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW mydata_daily_access
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 day', time) AS day,
            mydata_id,
            access_type,
            COUNT(DISTINCT user_address) AS unique_users,
            COUNT(*) AS total_accesses
        FROM mydata_access_logs
        GROUP BY time_bucket('1 day', time), mydata_id, access_type
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('mydata_daily_access',
            start_offset => INTERVAL '3 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'mydata_popular_data'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW mydata_popular_data
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 hour', time) AS hour,
            mydata_id,
            COUNT(DISTINCT buyer) AS unique_purchasers,
            SUM(CASE WHEN purchase_type = 'one_time' THEN 1 ELSE 0 END) AS one_time_purchases,
            SUM(CASE WHEN purchase_type = 'subscription' THEN 1 ELSE 0 END) AS subscriptions,
            SUM(price) AS total_revenue
        FROM mydata_purchases
        GROUP BY time_bucket('1 hour', time), mydata_id
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('mydata_popular_data',
            start_offset => INTERVAL '1 day',
            end_offset => INTERVAL '15 minutes',
            schedule_interval => INTERVAL '15 minutes');
    END IF;
END $$;

-- ============================================================================
-- 7. CREATE DATA LIFECYCLE MANAGEMENT POLICIES
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'mydata_access_logs'
    ) THEN
        PERFORM add_retention_policy('mydata_access_logs', INTERVAL '6 months');
    END IF;
END $$;

-- ============================================================================
-- 8. CREATE MARKETPLACE VIEWS FOR COMMON QUERIES
-- ============================================================================

CREATE OR REPLACE VIEW active_mydata AS
SELECT 
    d.*,
    CASE 
        WHEN d.one_time_price IS NOT NULL AND d.subscription_price IS NOT NULL THEN 'both'
        WHEN d.one_time_price IS NOT NULL THEN 'one_time'
        WHEN d.subscription_price IS NOT NULL THEN 'subscription'
        ELSE 'free'
    END AS pricing_model,
    CASE
        WHEN d.timestamp_end IS NOT NULL AND d.timestamp_end < EXTRACT(EPOCH FROM NOW()) THEN false
        ELSE true
    END AS is_current
FROM mydata_data d;

CREATE OR REPLACE VIEW mydata_popular_30_days AS
SELECT 
    d.mydata_id,
    d.owner,
    d.media_type,
    d.tags,
    COUNT(DISTINCT p.buyer) AS unique_purchasers,
    SUM(p.price) AS total_revenue,
    COUNT(p.id) AS total_purchases,
    MAX(p.time) AS last_purchase
FROM mydata_data d
LEFT JOIN mydata_purchases p ON d.mydata_id = p.mydata_id 
    AND p.time >= NOW() - INTERVAL '30 days'
GROUP BY d.mydata_id, d.owner, d.media_type, d.tags
ORDER BY unique_purchasers DESC, total_revenue DESC;

CREATE OR REPLACE VIEW mydata_creator_revenue_summary AS
SELECT 
    d.owner AS creator,
    COUNT(DISTINCT d.mydata_id) AS data_entries,
    SUM(r.amount) AS total_revenue,
    COUNT(DISTINCT r.from_address) AS unique_customers,
    MAX(r.time) AS last_revenue
FROM mydata_data d
LEFT JOIN mydata_revenue r ON d.mydata_id = r.mydata_id
GROUP BY d.owner
ORDER BY total_revenue DESC NULLS LAST;

-- ============================================================================
-- 9. CREATE HELPER FUNCTIONS
-- ============================================================================

DROP FUNCTION IF EXISTS user_has_access(TEXT, TEXT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS user_has_mydata_access(TEXT, TEXT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_data_pricing(TEXT) CASCADE;
DROP FUNCTION IF EXISTS get_mydata_pricing(TEXT) CASCADE;

CREATE OR REPLACE FUNCTION user_has_mydata_access(
    p_mydata_id TEXT,
    p_user_address TEXT,
    p_current_time BIGINT DEFAULT EXTRACT(EPOCH FROM NOW())
) RETURNS BOOLEAN AS $$
DECLARE
    data_owner TEXT;
    subscription_end BIGINT;
    has_purchase BOOLEAN := FALSE;
BEGIN
    SELECT owner INTO data_owner FROM mydata_data WHERE mydata_id = p_mydata_id;
    
    IF data_owner = p_user_address THEN
        RETURN TRUE;
    END IF;
    
    SELECT TRUE INTO has_purchase 
    FROM mydata_purchases 
    WHERE mydata_id = p_mydata_id AND buyer = p_user_address AND purchase_type = 'one_time'
    LIMIT 1;
    
    IF has_purchase THEN
        RETURN TRUE;
    END IF;
    
    SELECT MAX(subscription_end) INTO subscription_end
    FROM mydata_subscriptions 
    WHERE mydata_id = p_mydata_id AND subscriber = p_user_address;
    
    IF subscription_end IS NOT NULL AND subscription_end >= p_current_time THEN
        RETURN TRUE;
    END IF;
    
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_mydata_pricing(p_mydata_id TEXT)
RETURNS TABLE(
    mydata_id TEXT,
    one_time_price BIGINT,
    subscription_price BIGINT,
    subscription_duration_days BIGINT,
    total_purchasers BIGINT,
    total_subscribers BIGINT,
    total_revenue BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        d.mydata_id,
        d.one_time_price,
        d.subscription_price,
        d.subscription_duration_days,
        COALESCE(p.purchaser_count, 0) AS total_purchasers,
        COALESCE(s.subscriber_count, 0) AS total_subscribers,
        COALESCE(r.total_revenue, 0) AS total_revenue
    FROM mydata_data d
    LEFT JOIN (
        SELECT mydata_id, COUNT(DISTINCT buyer) AS purchaser_count
        FROM mydata_purchases 
        WHERE purchase_type = 'one_time'
        GROUP BY mydata_id
    ) p ON d.mydata_id = p.mydata_id
    LEFT JOIN (
        SELECT mydata_id, COUNT(DISTINCT subscriber) AS subscriber_count
        FROM mydata_subscriptions
        GROUP BY mydata_id
    ) s ON d.mydata_id = s.mydata_id
    LEFT JOIN (
        SELECT mydata_id, SUM(amount) AS total_revenue
        FROM mydata_revenue
        GROUP BY mydata_id
    ) r ON d.mydata_id = r.mydata_id
    WHERE d.mydata_id = p_mydata_id;
END;
$$ LANGUAGE plpgsql;

COMMENT ON TABLE mydata_data IS 'MyData marketplace entries with metadata and pricing';
COMMENT ON TABLE mydata_purchases IS 'Purchase records for one-time and subscription access (TimescaleDB)';
COMMENT ON TABLE mydata_subscriptions IS 'Active subscription tracking with expiry times (TimescaleDB)';
COMMENT ON TABLE mydata_revenue IS 'Revenue distribution and tracking (TimescaleDB)';
COMMENT ON TABLE mydata_access_logs IS 'Access pattern analytics and logs (TimescaleDB)';
COMMENT ON VIEW mydata_popular_30_days IS 'MyData listings ranked by 30-day purchase activity';
COMMENT ON VIEW mydata_creator_revenue_summary IS 'Per-creator MyData revenue summary across all listings';
