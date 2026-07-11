-- Migration: Dynamic ecosystem configs (rollback)
-- Version: 20260704010000
-- Purpose: Reverse the consolidated dynamic ecosystem configs migration.
--
-- Reverses in reverse dependency order:
--   1. DROP the 6 new config hypertables (with their time triggers/functions).
--   2. DROP the columns added to the 8 existing config tables.
--
-- Note: spot_config old fee_bps / fee_split_bps_platform columns are pre-existing
--       and are intentionally NOT dropped here.

-- ============================================================================
-- 1. DROP NEW CONFIG HYPERTABLES (reverse creation order)
-- ============================================================================

DROP VIEW IF EXISTS platform_revenue_summary CASCADE;
DROP VIEW IF EXISTS spt_creator_revenue_summary CASCADE;

-- Restore revenue views without messaging columns (pre-messaging-indexer state)
CREATE OR REPLACE VIEW platform_revenue_summary AS
SELECT
    platform_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
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

CREATE OR REPLACE VIEW spt_creator_revenue_summary AS
SELECT
    creator_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'tips' THEN amount ELSE 0 END) AS total_tips_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT payer_address) AS total_unique_payers,
    MAX(amount) AS largest_single_transaction,
    COUNT(DISTINCT DATE(time)) AS active_days,
    MAX(time) AS last_revenue_date
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '30 days'
GROUP BY creator_address
ORDER BY total_revenue DESC;

DROP TRIGGER IF EXISTS set_messaging_agent_groups_time ON messaging_agent_groups;
DROP FUNCTION IF EXISTS update_messaging_agent_groups_time();
DROP TABLE IF EXISTS messaging_agent_groups CASCADE;

DROP TRIGGER IF EXISTS set_paid_message_escrows_time ON paid_message_escrows;
DROP FUNCTION IF EXISTS update_paid_message_escrows_time();
DROP TABLE IF EXISTS paid_message_escrows CASCADE;

DROP TRIGGER IF EXISTS set_messaging_config_time ON messaging_config;
DROP FUNCTION IF EXISTS update_messaging_config_time();
DROP TABLE IF EXISTS messaging_config CASCADE;

DROP TRIGGER IF EXISTS set_platform_config_time ON platform_config;
DROP FUNCTION IF EXISTS update_platform_config_time();
DROP TABLE IF EXISTS platform_config CASCADE;

DROP TRIGGER IF EXISTS set_memory_config_time ON memory_config;
DROP FUNCTION IF EXISTS update_memory_config_time();
DROP TABLE IF EXISTS memory_config CASCADE;

DROP TRIGGER IF EXISTS set_profile_config_time ON profile_config;
DROP FUNCTION IF EXISTS update_profile_config_time();
DROP TABLE IF EXISTS profile_config CASCADE;

DROP TRIGGER IF EXISTS set_subscription_config_time ON subscription_config;
DROP FUNCTION IF EXISTS update_subscription_config_time();
DROP TABLE IF EXISTS subscription_config CASCADE;

DROP TRIGGER IF EXISTS set_insurance_router_config_time ON insurance_router_config;
DROP FUNCTION IF EXISTS update_insurance_router_config_time();
DROP TABLE IF EXISTS insurance_router_config CASCADE;

-- ============================================================================
-- 2. DROP COLUMNS ADDED TO EXISTING CONFIG TABLES (reverse alter order)
-- ============================================================================

-- 2.9 profile_config
ALTER TABLE profile_config
DROP COLUMN IF EXISTS profile_sale_fee_bps;

-- 2.8 insurance_config
ALTER TABLE insurance_config
DROP COLUMN IF EXISTS odds_base_bps;

-- 2.7 mydata_config
ALTER TABLE mydata_config
DROP COLUMN IF EXISTS default_claim_window_ms,
DROP COLUMN IF EXISTS max_paid_access_entries,
DROP COLUMN IF EXISTS max_merkle_proof_depth,
DROP COLUMN IF EXISTS max_pool_assignments,
DROP COLUMN IF EXISTS max_payment_reference_bytes,
DROP COLUMN IF EXISTS max_metadata_bytes,
DROP COLUMN IF EXISTS max_tag_bytes,
DROP COLUMN IF EXISTS max_encrypted_data_bytes,
DROP COLUMN IF EXISTS non_platform_platform_to_treasury_bps,
DROP COLUMN IF EXISTS non_platform_platform_to_creator_bps,
DROP COLUMN IF EXISTS mydata_marketplace_ecosystem_fee_bps,
DROP COLUMN IF EXISTS mydata_marketplace_platform_fee_bps,
DROP COLUMN IF EXISTS p2p_ecosystem_fee_bps,
DROP COLUMN IF EXISTS p2p_platform_fee_bps,
DROP COLUMN IF EXISTS max_encryption_id_bytes;

-- 2.7b subscription_revenue fee breakdown (subscription_config is dropped above)
ALTER TABLE subscription_revenue
DROP COLUMN IF EXISTS platform_address,
DROP COLUMN IF EXISTS creator_amount,
DROP COLUMN IF EXISTS ecosystem_fee,
DROP COLUMN IF EXISTS platform_fee;

ALTER TABLE mydata_purchases
DROP COLUMN IF EXISTS platform_address,
DROP COLUMN IF EXISTS creator_amount,
DROP COLUMN IF EXISTS ecosystem_fee,
DROP COLUMN IF EXISTS platform_fee;

ALTER TABLE mydata_revenue
DROP COLUMN IF EXISTS platform_address,
DROP COLUMN IF EXISTS creator_amount,
DROP COLUMN IF EXISTS ecosystem_fee,
DROP COLUMN IF EXISTS platform_fee;

ALTER TABLE mydata_claims
DROP COLUMN IF EXISTS platform_address,
DROP COLUMN IF EXISTS net_amount,
DROP COLUMN IF EXISTS ecosystem_fee,
DROP COLUMN IF EXISTS platform_fee,
DROP COLUMN IF EXISTS gross_amount;

-- 2.6 poc_configuration
ALTER TABLE poc_configuration
DROP COLUMN IF EXISTS dispute_governance_registry_id,
DROP COLUMN IF EXISTS max_disputes_per_post,
DROP COLUMN IF EXISTS min_vault_deposit_amount;

-- 2.5 spot_config — fee model redo (old fee_bps / fee_split_bps_platform kept)
ALTER TABLE spot_config
DROP COLUMN IF EXISTS ecosystem_fee_bps,
DROP COLUMN IF EXISTS platform_fee_bps;

-- 2.4 spot_config — limits
ALTER TABLE spot_config
DROP COLUMN IF EXISTS max_evidence_urls,
DROP COLUMN IF EXISTS max_reasoning_length,
DROP COLUMN IF EXISTS min_reasoning_length,
DROP COLUMN IF EXISTS max_betting_options,
DROP COLUMN IF EXISTS min_betting_options;

-- 2.3 spt_exchange_config
ALTER TABLE spt_exchange_config
DROP COLUMN IF EXISTS non_platform_platform_to_treasury_bps,
DROP COLUMN IF EXISTS non_platform_platform_to_creator_bps;

-- 2.2 post_config
ALTER TABLE post_config
DROP COLUMN IF EXISTS min_view_duration_ms,
DROP COLUMN IF EXISTS max_promotion_amount,
DROP COLUMN IF EXISTS min_promotion_amount;

-- 1.9 config semantic renames + defaults (reverse)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'spt_config'
    ) THEN
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_admin_address;
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_reason;
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_timestamp;
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_transaction_id;
        ALTER TABLE spt_config
            ADD CONSTRAINT valid_admin_address CHECK (length(admin_address) > 0);
        ALTER TABLE spt_config
            ADD CONSTRAINT valid_reason CHECK (length(reason) > 0 AND length(reason) <= 512);
        ALTER TABLE spt_config
            ADD CONSTRAINT valid_timestamp CHECK (updated_at >= 0);
        ALTER TABLE spt_config
            ADD CONSTRAINT valid_transaction_id
            CHECK (length(transaction_id) > 0 AND length(transaction_id) <= 255);
    END IF;
END $$;
ALTER TABLE spt_config ALTER COLUMN trading_enabled SET DEFAULT FALSE;
ALTER TABLE spt_exchange_config ALTER COLUMN trading_enabled SET DEFAULT FALSE;
ALTER TABLE insurance_config RENAME COLUMN insurance_enabled TO enable_flag;
ALTER TABLE spot_config RENAME COLUMN truth_enabled TO enable_flag;
ALTER TABLE mydata_config RENAME COLUMN marketplace_enabled TO enable_flag;

-- 2.1 ai_credit_config
ALTER TABLE ai_credit_config
DROP COLUMN IF EXISTS oracle_markup_bps;

-- ============================================================================
-- 3. REVERSE METADATA STANDARDIZATION
-- ============================================================================

DROP INDEX IF EXISTS idx_ecosystem_treasury_time;
DROP INDEX IF EXISTS idx_spt_config_time;
DROP INDEX IF EXISTS idx_poc_configuration_time;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_name = 'spt_config'
    ) THEN
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS spt_config_pkey;
    END IF;
    IF EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_name = 'poc_configuration'
    ) THEN
        ALTER TABLE poc_configuration DROP CONSTRAINT IF EXISTS poc_configuration_pkey;
    END IF;
END $$;

DROP TRIGGER IF EXISTS set_spt_config_time ON spt_config;
DROP FUNCTION IF EXISTS update_spt_config_time();
DROP TRIGGER IF EXISTS set_poc_configuration_time ON poc_configuration;
DROP FUNCTION IF EXISTS update_poc_configuration_time();
DROP TRIGGER IF EXISTS set_ecosystem_treasury_time ON ecosystem_treasury;
DROP FUNCTION IF EXISTS update_ecosystem_treasury_time();
DROP TRIGGER IF EXISTS set_spt_exchange_config_time ON spt_exchange_config;
DROP FUNCTION IF EXISTS update_spt_exchange_config_time();

ALTER TABLE ecosystem_treasury DROP COLUMN IF EXISTS version;
ALTER TABLE poc_configuration DROP COLUMN IF EXISTS version;
ALTER TABLE spt_exchange_config DROP COLUMN IF EXISTS version;
ALTER TABLE mydata_config DROP COLUMN IF EXISTS version;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'spot_config' AND column_name = 'updated_at'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'spot_config' AND column_name = 'timestamp_ms'
    ) THEN
        ALTER TABLE spot_config RENAME COLUMN updated_at TO timestamp_ms;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'mydata_config' AND column_name = 'updated_at'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'mydata_config' AND column_name = 'timestamp_ms'
    ) THEN
        ALTER TABLE mydata_config RENAME COLUMN updated_at TO timestamp_ms;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'insurance_config' AND column_name = 'updated_at'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'insurance_config' AND column_name = 'timestamp_ms'
    ) THEN
        ALTER TABLE insurance_config RENAME COLUMN updated_at TO timestamp_ms;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'ecosystem_treasury' AND column_name = 'updated_at'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'ecosystem_treasury' AND column_name = 'timestamp_ms'
    ) THEN
        ALTER TABLE ecosystem_treasury RENAME COLUMN updated_at TO timestamp_ms;
    END IF;
END $$;

-- ============================================================================
-- Rollback 1.11 profiles on-chain fields cleanup
-- ============================================================================

ALTER TABLE profiles ADD COLUMN IF NOT EXISTS raised_location TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS phone TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS email TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS gender TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS political_view TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS religion TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS education TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS primary_language TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS relationship_status TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS sensitive_data_updated_at TIMESTAMP;

DO $$ BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'location'
  ) AND NOT EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'current_location'
  ) THEN
    ALTER TABLE profiles RENAME COLUMN location TO current_location;
  END IF;
END $$;
