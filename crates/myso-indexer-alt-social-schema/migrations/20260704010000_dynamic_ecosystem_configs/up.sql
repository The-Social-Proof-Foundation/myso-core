-- Migration: Dynamic ecosystem configs (greenfield first publish)
-- Version: 20260704010000
-- Purpose: Consolidated schema rollout for all dynamic config fields and five new
--          config hypertables from the social + messaging Move packages.
--
-- This migration is idempotent (guards with IF NOT EXISTS / ADD COLUMN IF NOT EXISTS)
-- and is safe to re-run. It:
--   1. Extends 8 existing config tables with new columns (defaults preserve today's
--      hardcoded behavior).
--   2. Creates 6 new TimescaleDB hypertables for config objects that previously had
--      no persisted state (insurance_router_config, subscription_config, profile_config,
--      memory_config, platform_config, messaging_config).
--
-- Note: spot_config keeps its old fee_bps / fee_split_bps_platform columns for
--       rollback safety and Move struct parity during the transition window; the new
--       platform_fee_bps / ecosystem_fee_bps columns are added alongside them.
--
-- Note: insurance_router_config does NOT already exist (the 20260503190000_insurance_router_marketplace
--       migration created insurance_coverage_routes / insurance_route_fills, not a router
--       config table), so it is created here as a new hypertable.

-- ============================================================================
-- 1. ALTER EXISTING CONFIG TABLES — add new columns
-- ============================================================================

-- 1.1 ai_credit_config (singleton) — oracle markup bps
ALTER TABLE ai_credit_config
ADD COLUMN IF NOT EXISTS oracle_markup_bps BIGINT NOT NULL DEFAULT 1500;
UPDATE ai_credit_config SET oracle_markup_bps = 1500 WHERE oracle_markup_bps IS NULL OR oracle_markup_bps = 0;
COMMENT ON COLUMN ai_credit_config.oracle_markup_bps IS 'Markup in basis points applied on top of oracle AI credit pricing (default: 1500 = 15%; max 10000)';

-- 1.2 post_config (hypertable) — promotion parameters
ALTER TABLE post_config
ADD COLUMN IF NOT EXISTS min_promotion_amount BIGINT NOT NULL DEFAULT 1000,
ADD COLUMN IF NOT EXISTS max_promotion_amount BIGINT NOT NULL DEFAULT 100000000,
ADD COLUMN IF NOT EXISTS min_view_duration_ms BIGINT NOT NULL DEFAULT 3000;
UPDATE post_config SET min_promotion_amount = 1000 WHERE min_promotion_amount IS NULL;
UPDATE post_config SET max_promotion_amount = 100000000 WHERE max_promotion_amount IS NULL;
UPDATE post_config SET min_view_duration_ms = 3000 WHERE min_view_duration_ms IS NULL;
COMMENT ON COLUMN post_config.min_promotion_amount IS 'Minimum payment per view for a promoted post in MIST (default: 1000)';
COMMENT ON COLUMN post_config.max_promotion_amount IS 'Maximum payment per view for a promoted post in MIST (default: 100000000)';
COMMENT ON COLUMN post_config.min_view_duration_ms IS 'Minimum view duration in ms for a promoted post view to count (default: 3000)';

-- 1.3 spt_config — exchange parameters (consolidates former spt_exchange_config hypertable)
ALTER TABLE spt_config
ADD COLUMN IF NOT EXISTS post_threshold BIGINT NOT NULL DEFAULT 1000000000000,
ADD COLUMN IF NOT EXISTS profile_threshold BIGINT NOT NULL DEFAULT 10000000000000,
ADD COLUMN IF NOT EXISTS max_individual_reservation_bps BIGINT NOT NULL DEFAULT 2000,
ADD COLUMN IF NOT EXISTS total_fee_bps BIGINT NOT NULL DEFAULT 150,
ADD COLUMN IF NOT EXISTS creator_fee_bps BIGINT NOT NULL DEFAULT 100,
ADD COLUMN IF NOT EXISTS platform_fee_bps BIGINT NOT NULL DEFAULT 25,
ADD COLUMN IF NOT EXISTS treasury_fee_bps BIGINT NOT NULL DEFAULT 25,
ADD COLUMN IF NOT EXISTS trading_creator_fee_bps BIGINT NOT NULL DEFAULT 100,
ADD COLUMN IF NOT EXISTS trading_platform_fee_bps BIGINT NOT NULL DEFAULT 25,
ADD COLUMN IF NOT EXISTS trading_treasury_fee_bps BIGINT NOT NULL DEFAULT 25,
ADD COLUMN IF NOT EXISTS reservation_creator_fee_bps BIGINT NOT NULL DEFAULT 100,
ADD COLUMN IF NOT EXISTS reservation_platform_fee_bps BIGINT NOT NULL DEFAULT 25,
ADD COLUMN IF NOT EXISTS reservation_treasury_fee_bps BIGINT NOT NULL DEFAULT 25,
ADD COLUMN IF NOT EXISTS max_reservers_per_pool BIGINT NOT NULL DEFAULT 1000,
ADD COLUMN IF NOT EXISTS base_price BIGINT NOT NULL DEFAULT 100000000,
ADD COLUMN IF NOT EXISTS quadratic_coefficient BIGINT NOT NULL DEFAULT 100000,
ADD COLUMN IF NOT EXISTS max_hold_percent_bps BIGINT NOT NULL DEFAULT 500,
ADD COLUMN IF NOT EXISTS non_platform_platform_to_creator_bps BIGINT NOT NULL DEFAULT 5000,
ADD COLUMN IF NOT EXISTS non_platform_platform_to_treasury_bps BIGINT NOT NULL DEFAULT 5000;
COMMENT ON COLUMN spt_config.non_platform_platform_to_creator_bps IS 'Non-platform path: creator share of the platform-fee bucket in bps (default: 5000)';
COMMENT ON COLUMN spt_config.non_platform_platform_to_treasury_bps IS 'Non-platform path: ecosystem treasury share of the platform-fee bucket in bps (default: 5000)';

-- 1.4 spot_config (hypertable) — betting/reasoning/evidence limits
ALTER TABLE spot_config
ADD COLUMN IF NOT EXISTS min_betting_options BIGINT NOT NULL DEFAULT 2,
ADD COLUMN IF NOT EXISTS max_betting_options BIGINT NOT NULL DEFAULT 10,
ADD COLUMN IF NOT EXISTS min_reasoning_length BIGINT NOT NULL DEFAULT 10,
ADD COLUMN IF NOT EXISTS max_reasoning_length BIGINT NOT NULL DEFAULT 5000,
ADD COLUMN IF NOT EXISTS max_evidence_urls BIGINT NOT NULL DEFAULT 10;
UPDATE spot_config SET min_betting_options = 2 WHERE min_betting_options IS NULL;
UPDATE spot_config SET max_betting_options = 10 WHERE max_betting_options IS NULL;
UPDATE spot_config SET min_reasoning_length = 10 WHERE min_reasoning_length IS NULL;
UPDATE spot_config SET max_reasoning_length = 5000 WHERE max_reasoning_length IS NULL;
UPDATE spot_config SET max_evidence_urls = 10 WHERE max_evidence_urls IS NULL;
COMMENT ON COLUMN spot_config.min_betting_options IS 'Minimum number of betting options per SPoT record (default: 2)';
COMMENT ON COLUMN spot_config.max_betting_options IS 'Maximum number of betting options per SPoT record (default: 10)';
COMMENT ON COLUMN spot_config.min_reasoning_length IS 'Minimum reasoning text length required when placing a bet (default: 10)';
COMMENT ON COLUMN spot_config.max_reasoning_length IS 'Maximum reasoning text length allowed when placing a bet (default: 5000)';
COMMENT ON COLUMN spot_config.max_evidence_urls IS 'Maximum number of evidence URLs allowed on a bet (default: 10)';

-- 1.5 spot_config (hypertable) — fee model redo (direct platform % + ecosystem % of gross)
--      Old fee_bps / fee_split_bps_platform columns are intentionally KEPT.
ALTER TABLE spot_config
ADD COLUMN IF NOT EXISTS platform_fee_bps BIGINT NOT NULL DEFAULT 50,
ADD COLUMN IF NOT EXISTS ecosystem_fee_bps BIGINT NOT NULL DEFAULT 50;
UPDATE spot_config SET platform_fee_bps = 50 WHERE platform_fee_bps IS NULL;
UPDATE spot_config SET ecosystem_fee_bps = 50 WHERE ecosystem_fee_bps IS NULL;
COMMENT ON COLUMN spot_config.platform_fee_bps IS 'Platform fee as a direct percentage of gross in bps (default: 50); platform_fee_bps + ecosystem_fee_bps <= 10000';
COMMENT ON COLUMN spot_config.ecosystem_fee_bps IS 'Ecosystem treasury fee as a direct percentage of gross in bps (default: 50); platform_fee_bps + ecosystem_fee_bps <= 10000';

-- 1.6 poc_configuration (append) — dispute cap + min vault deposit
ALTER TABLE poc_configuration
ADD COLUMN IF NOT EXISTS max_disputes_per_post SMALLINT NOT NULL DEFAULT 2,
ADD COLUMN IF NOT EXISTS min_vault_deposit_amount BIGINT NOT NULL DEFAULT 1;
UPDATE poc_configuration SET max_disputes_per_post = 2 WHERE max_disputes_per_post IS NULL;
UPDATE poc_configuration SET min_vault_deposit_amount = 1 WHERE min_vault_deposit_amount IS NULL;
COMMENT ON COLUMN poc_configuration.max_disputes_per_post IS 'Max successful dispute submissions per post (lifetime, default: 2); SMALLINT mirrors Move u8';
COMMENT ON COLUMN poc_configuration.min_vault_deposit_amount IS 'Minimum amount (per asset) accepted into a beneficiary vault deposit (default: 1)';

ALTER TABLE poc_configuration
  ADD COLUMN IF NOT EXISTS dispute_governance_registry_id TEXT NULL;
COMMENT ON COLUMN poc_configuration.dispute_governance_registry_id IS
  'Shared PoC GovernanceDAO object ID (registry_type = 1)';

-- 1.7 mydata_config (hypertable) — max encryption_id byte length
ALTER TABLE mydata_config
ADD COLUMN IF NOT EXISTS max_encryption_id_bytes BIGINT NOT NULL DEFAULT 1024;
UPDATE mydata_config SET max_encryption_id_bytes = 1024 WHERE max_encryption_id_bytes IS NULL;
COMMENT ON COLUMN mydata_config.max_encryption_id_bytes IS 'Maximum accepted encryption_id byte length (default: 1024)';

-- 1.8 insurance_config (hypertable) — base odds multiplier bps
ALTER TABLE insurance_config
ADD COLUMN IF NOT EXISTS odds_base_bps BIGINT NOT NULL DEFAULT 5000;
UPDATE insurance_config SET odds_base_bps = 5000 WHERE odds_base_bps IS NULL;
COMMENT ON COLUMN insurance_config.odds_base_bps IS 'Base odds multiplier in bps used by compute_spot_risk_quote (default: 5000; must be > 0)';

-- 1.10 mydata_config — P2P + MyData marketplace fee bps + non-platform split
ALTER TABLE mydata_config
ADD COLUMN IF NOT EXISTS p2p_platform_fee_bps BIGINT NOT NULL DEFAULT 250,
ADD COLUMN IF NOT EXISTS p2p_ecosystem_fee_bps BIGINT NOT NULL DEFAULT 250,
ADD COLUMN IF NOT EXISTS mydata_marketplace_platform_fee_bps BIGINT NOT NULL DEFAULT 250,
ADD COLUMN IF NOT EXISTS mydata_marketplace_ecosystem_fee_bps BIGINT NOT NULL DEFAULT 250,
ADD COLUMN IF NOT EXISTS non_platform_platform_to_creator_bps BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS non_platform_platform_to_treasury_bps BIGINT NOT NULL DEFAULT 10000;
UPDATE mydata_config SET p2p_platform_fee_bps = 250 WHERE p2p_platform_fee_bps IS NULL;
UPDATE mydata_config SET p2p_ecosystem_fee_bps = 250 WHERE p2p_ecosystem_fee_bps IS NULL;
UPDATE mydata_config SET mydata_marketplace_platform_fee_bps = 250 WHERE mydata_marketplace_platform_fee_bps IS NULL;
UPDATE mydata_config SET mydata_marketplace_ecosystem_fee_bps = 250 WHERE mydata_marketplace_ecosystem_fee_bps IS NULL;
UPDATE mydata_config SET non_platform_platform_to_creator_bps = 0 WHERE non_platform_platform_to_creator_bps IS NULL;
UPDATE mydata_config SET non_platform_platform_to_treasury_bps = 10000 WHERE non_platform_platform_to_treasury_bps IS NULL;
COMMENT ON COLUMN mydata_config.p2p_platform_fee_bps IS 'P2P marketplace platform fee as % of gross in bps (default: 250)';
COMMENT ON COLUMN mydata_config.p2p_ecosystem_fee_bps IS 'P2P marketplace ecosystem fee as % of gross in bps (default: 250)';
COMMENT ON COLUMN mydata_config.mydata_marketplace_platform_fee_bps IS 'MyData marketplace pool claim platform fee as % of gross in bps (default: 250)';
COMMENT ON COLUMN mydata_config.mydata_marketplace_ecosystem_fee_bps IS 'MyData marketplace pool claim ecosystem fee as % of gross in bps (default: 250)';
COMMENT ON COLUMN mydata_config.non_platform_platform_to_creator_bps IS 'Non-platform path: recipient share of platform-fee bucket in bps (default: 0)';
COMMENT ON COLUMN mydata_config.non_platform_platform_to_treasury_bps IS 'Non-platform path: ecosystem share of platform-fee bucket in bps (default: 10000)';

-- 1.10b mydata_config — production input and settlement bounds
ALTER TABLE mydata_config
ADD COLUMN IF NOT EXISTS max_encrypted_data_bytes BIGINT NOT NULL DEFAULT 262144,
ADD COLUMN IF NOT EXISTS max_tag_bytes BIGINT NOT NULL DEFAULT 64,
ADD COLUMN IF NOT EXISTS max_metadata_bytes BIGINT NOT NULL DEFAULT 1024,
ADD COLUMN IF NOT EXISTS max_payment_reference_bytes BIGINT NOT NULL DEFAULT 256,
ADD COLUMN IF NOT EXISTS max_pool_assignments BIGINT NOT NULL DEFAULT 32,
ADD COLUMN IF NOT EXISTS max_merkle_proof_depth BIGINT NOT NULL DEFAULT 64,
ADD COLUMN IF NOT EXISTS max_paid_access_entries BIGINT NOT NULL DEFAULT 100000,
ADD COLUMN IF NOT EXISTS default_claim_window_ms BIGINT NOT NULL DEFAULT 2592000000;

-- 1.11 subscription_revenue — fee breakdown
ALTER TABLE subscription_revenue
ADD COLUMN IF NOT EXISTS platform_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS ecosystem_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS creator_amount BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS platform_address TEXT;
COMMENT ON COLUMN subscription_revenue.platform_fee IS 'Platform fee slice deducted from gross payment (MYSO base units)';
COMMENT ON COLUMN subscription_revenue.ecosystem_fee IS 'Ecosystem treasury fee slice deducted from gross payment';
COMMENT ON COLUMN subscription_revenue.creator_amount IS 'Net amount credited to profile owner after fees';
COMMENT ON COLUMN subscription_revenue.platform_address IS 'Platform treasury recipient when platform fee was routed on-chain';

-- 1.13 mydata purchases/revenue/claims — fee breakdown
ALTER TABLE mydata_purchases
ADD COLUMN IF NOT EXISTS platform_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS ecosystem_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS creator_amount BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS platform_address TEXT;

ALTER TABLE mydata_revenue
ADD COLUMN IF NOT EXISTS platform_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS ecosystem_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS creator_amount BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS platform_address TEXT;

ALTER TABLE mydata_claims
ADD COLUMN IF NOT EXISTS gross_amount BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS platform_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS ecosystem_fee BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS net_amount BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS platform_address TEXT;
UPDATE mydata_claims SET gross_amount = amount WHERE gross_amount = 0 AND amount > 0;
UPDATE mydata_claims SET net_amount = amount WHERE net_amount = 0 AND amount > 0;

-- 1.9 config semantic renames + defaults
ALTER TABLE mydata_config RENAME COLUMN enable_flag TO marketplace_enabled;
ALTER TABLE spot_config RENAME COLUMN enable_flag TO truth_enabled;
ALTER TABLE insurance_config RENAME COLUMN enable_flag TO insurance_enabled;
COMMENT ON COLUMN mydata_config.marketplace_enabled IS 'Whether the MyData marketplace is enabled (default: false)';
COMMENT ON COLUMN spot_config.truth_enabled IS 'Whether Social Proof of Truth (SPoT) is enabled (default: false)';
COMMENT ON COLUMN insurance_config.insurance_enabled IS 'Whether insurance is enabled (default: false)';
ALTER TABLE spt_config ALTER COLUMN trading_enabled SET DEFAULT TRUE;

-- ============================================================================
-- 2. CREATE NEW CONFIG HYPERTABLES
--    Each hypertable mirrors the post_config / mydata_config pattern:
--    `time` is derived from `updated_at` via a BEFORE INSERT trigger so the
--    hypertable dimension reflects the on-chain event time, not DB wall-clock.
-- ============================================================================

-- 2.1 insurance_router_config
CREATE TABLE IF NOT EXISTS insurance_router_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    paused BOOLEAN NOT NULL DEFAULT FALSE,
    max_route_reserve_market BIGINT NOT NULL DEFAULT 0,
    max_route_reserve_user BIGINT NOT NULL DEFAULT 0,
    max_route_reserve_option BIGINT NOT NULL DEFAULT 0,
    max_vault_concentration_bps BIGINT NOT NULL DEFAULT 10000,
    min_vault_health_factor_bps BIGINT NOT NULL DEFAULT 10000,
    max_route_legs BIGINT NOT NULL DEFAULT 4,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE insurance_router_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_insurance_router_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_insurance_router_config_time ON insurance_router_config;
CREATE TRIGGER set_insurance_router_config_time
BEFORE INSERT ON insurance_router_config
FOR EACH ROW
EXECUTE FUNCTION update_insurance_router_config_time();
SELECT create_hypertable('insurance_router_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'insurance_router_config_pkey'
    ) THEN
        ALTER TABLE insurance_router_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_insurance_router_config_time ON insurance_router_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_insurance_router_config_updated_by ON insurance_router_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_insurance_router_config_transaction_id ON insurance_router_config(transaction_id);
COMMENT ON TABLE insurance_router_config IS 'Tracks InsuranceRouterConfig changes over time (router pause + reserve/health caps + max_route_legs). Each row represents a configuration update.';
COMMENT ON COLUMN insurance_router_config.paused IS 'Whether the coverage router is paused (default: FALSE)';
COMMENT ON COLUMN insurance_router_config.max_route_reserve_market IS 'Max reserve that may be locked per market across a route (default: 0)';
COMMENT ON COLUMN insurance_router_config.max_route_reserve_user IS 'Max reserve that may be locked per user across a route (default: 0)';
COMMENT ON COLUMN insurance_router_config.max_route_reserve_option IS 'Max reserve that may be locked per option across a route (default: 0)';
COMMENT ON COLUMN insurance_router_config.max_vault_concentration_bps IS 'Max share of a route reserve one vault may take in bps (default: 10000)';
COMMENT ON COLUMN insurance_router_config.min_vault_health_factor_bps IS 'Min vault health factor required to participate in routing in bps (default: 10000)';
COMMENT ON COLUMN insurance_router_config.max_route_legs IS 'Maximum number of legs (vault fills) allowed in a single coverage route (default: 4)';

-- 2.2 subscription_config
CREATE TABLE IF NOT EXISTS subscription_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    default_billing_period_ms BIGINT NOT NULL DEFAULT 2592000000,
    max_renewal_months BIGINT NOT NULL DEFAULT 12,
    platform_fee_bps BIGINT NOT NULL DEFAULT 250,
    ecosystem_fee_bps BIGINT NOT NULL DEFAULT 250,
    non_platform_platform_to_creator_bps BIGINT NOT NULL DEFAULT 0,
    non_platform_platform_to_treasury_bps BIGINT NOT NULL DEFAULT 10000,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE subscription_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_subscription_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_subscription_config_time ON subscription_config;
CREATE TRIGGER set_subscription_config_time
BEFORE INSERT ON subscription_config
FOR EACH ROW
EXECUTE FUNCTION update_subscription_config_time();
SELECT create_hypertable('subscription_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'subscription_config_pkey'
    ) THEN
        ALTER TABLE subscription_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_subscription_config_time ON subscription_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_subscription_config_updated_by ON subscription_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_subscription_config_transaction_id ON subscription_config(transaction_id);
COMMENT ON TABLE subscription_config IS 'Tracks SubscriptionConfig changes over time (global subscription billing parameters). Each row represents a configuration update.';
COMMENT ON COLUMN subscription_config.default_billing_period_ms IS 'Default plan billing period in ms when duration_ms is zero (default: 2592000000 = 30 days)';
COMMENT ON COLUMN subscription_config.max_renewal_months IS 'Maximum number of renewal months allowed per subscription (default: 12)';
COMMENT ON COLUMN subscription_config.platform_fee_bps IS 'Platform fee as direct % of gross subscription payment in bps (default: 250 = 2.5%)';
COMMENT ON COLUMN subscription_config.ecosystem_fee_bps IS 'Ecosystem treasury fee as direct % of gross in bps (default: 250 = 2.5%)';
COMMENT ON COLUMN subscription_config.non_platform_platform_to_creator_bps IS 'Non-platform path: creator share of platform-fee bucket in bps (default: 0)';
COMMENT ON COLUMN subscription_config.non_platform_platform_to_treasury_bps IS 'Non-platform path: ecosystem share of platform-fee bucket in bps (default: 10000)';

-- 2.3 profile_config
CREATE TABLE IF NOT EXISTS profile_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    max_vesting_pieces BIGINT NOT NULL DEFAULT 10,
    curve_factor_min BIGINT NOT NULL DEFAULT 100,
    curve_factor_max BIGINT NOT NULL DEFAULT 10000,
    curve_precision BIGINT NOT NULL DEFAULT 1000,
    min_claim_threshold_divisor BIGINT NOT NULL DEFAULT 1000,
    min_username_length BIGINT NOT NULL DEFAULT 2,
    max_username_length BIGINT NOT NULL DEFAULT 50,
    profile_sale_fee_bps BIGINT NOT NULL DEFAULT 500,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE profile_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_profile_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_profile_config_time ON profile_config;
CREATE TRIGGER set_profile_config_time
BEFORE INSERT ON profile_config
FOR EACH ROW
EXECUTE FUNCTION update_profile_config_time();
SELECT create_hypertable('profile_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'profile_config_pkey'
    ) THEN
        ALTER TABLE profile_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_profile_config_time ON profile_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_config_updated_by ON profile_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_profile_config_transaction_id ON profile_config(transaction_id);
COMMENT ON TABLE profile_config IS 'Tracks ProfileConfig changes over time (vesting curve + username length bounds). Each row represents a configuration update.';
COMMENT ON COLUMN profile_config.max_vesting_pieces IS 'Maximum number of vesting pieces per vesting wallet (default: 10)';
COMMENT ON COLUMN profile_config.curve_factor_min IS 'Minimum curve factor for vesting schedules (default: 100)';
COMMENT ON COLUMN profile_config.curve_factor_max IS 'Maximum curve factor for vesting schedules (default: 10000)';
COMMENT ON COLUMN profile_config.curve_precision IS 'Curve factor precision divisor (default: 1000)';
COMMENT ON COLUMN profile_config.min_claim_threshold_divisor IS 'Minimum claim threshold divisor (default: 1000)';
COMMENT ON COLUMN profile_config.min_username_length IS 'Minimum username length (default: 2)';
COMMENT ON COLUMN profile_config.max_username_length IS 'Maximum username length (default: 50)';
COMMENT ON COLUMN profile_config.profile_sale_fee_bps IS 'Fee in bps taken on profile sales (default: 500; 10000 = 100%)';

-- 2.4 memory_config
--    u8 fields (max_organizations_per_user, max_agent_depth) use SMALLINT to match
--    the existing u8 column convention in this schema (e.g. poc media_type, spot option_id).
CREATE TABLE IF NOT EXISTS memory_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    max_organizations_per_user SMALLINT NOT NULL DEFAULT 8,
    org_category_update_cooldown_ms BIGINT NOT NULL DEFAULT 604800000,
    max_agent_depth SMALLINT NOT NULL DEFAULT 8,
    max_label_length BIGINT NOT NULL DEFAULT 64,
    max_org_name_length BIGINT NOT NULL DEFAULT 100,
    max_org_description_length BIGINT NOT NULL DEFAULT 1200,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE memory_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_memory_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_memory_config_time ON memory_config;
CREATE TRIGGER set_memory_config_time
BEFORE INSERT ON memory_config
FOR EACH ROW
EXECUTE FUNCTION update_memory_config_time();
SELECT create_hypertable('memory_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'memory_config_pkey'
    ) THEN
        ALTER TABLE memory_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_memory_config_time ON memory_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_memory_config_updated_by ON memory_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_memory_config_transaction_id ON memory_config(transaction_id);
COMMENT ON TABLE memory_config IS 'Tracks MemoryConfig changes over time (memory organization/agent/label bounds). Each row represents a configuration update.';
COMMENT ON COLUMN memory_config.max_organizations_per_user IS 'Maximum number of organizations a user may own (default: 8)';
COMMENT ON COLUMN memory_config.org_category_update_cooldown_ms IS 'Cooldown in ms between organization category updates (default: 604800000 = 7 days)';
COMMENT ON COLUMN memory_config.max_agent_depth IS 'Maximum nesting depth for sub-agents (default: 8)';
COMMENT ON COLUMN memory_config.max_label_length IS 'Maximum label length (default: 64)';
COMMENT ON COLUMN memory_config.max_org_name_length IS 'Maximum organization name length (default: 100)';
COMMENT ON COLUMN memory_config.max_org_description_length IS 'Maximum organization description length (default: 1200)';

-- 2.5 platform_config
CREATE TABLE IF NOT EXISTS platform_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    max_reasoning_length BIGINT NOT NULL DEFAULT 2000,
    max_cover_photo_url_length BIGINT NOT NULL DEFAULT 2048,
    max_media_previews BIGINT NOT NULL DEFAULT 10,
    max_media_preview_url_length BIGINT NOT NULL DEFAULT 2048,
    max_badge_name_length BIGINT NOT NULL DEFAULT 100,
    max_badge_description_length BIGINT NOT NULL DEFAULT 500,
    max_badge_media_url_length BIGINT NOT NULL DEFAULT 2048,
    max_badge_icon_url_length BIGINT NOT NULL DEFAULT 2048,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE platform_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_platform_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_platform_config_time ON platform_config;
CREATE TRIGGER set_platform_config_time
BEFORE INSERT ON platform_config
FOR EACH ROW
EXECUTE FUNCTION update_platform_config_time();
SELECT create_hypertable('platform_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'platform_config_pkey'
    ) THEN
        ALTER TABLE platform_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_platform_config_time ON platform_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_platform_config_updated_by ON platform_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_platform_config_transaction_id ON platform_config(transaction_id);
COMMENT ON TABLE platform_config IS 'Tracks PlatformConfig changes over time (platform reasoning/media/badge length bounds). Each row represents a configuration update.';
COMMENT ON COLUMN platform_config.max_reasoning_length IS 'Maximum reasoning text length for platform approval (default: 2000)';
COMMENT ON COLUMN platform_config.max_cover_photo_url_length IS 'Maximum cover photo URL length (default: 2048)';
COMMENT ON COLUMN platform_config.max_media_previews IS 'Maximum number of media previews per platform (default: 10)';
COMMENT ON COLUMN platform_config.max_media_preview_url_length IS 'Maximum media preview URL length (default: 2048)';
COMMENT ON COLUMN platform_config.max_badge_name_length IS 'Maximum platform badge name length (default: 100)';
COMMENT ON COLUMN platform_config.max_badge_description_length IS 'Maximum platform badge description length (default: 500)';
COMMENT ON COLUMN platform_config.max_badge_media_url_length IS 'Maximum platform badge media URL length (default: 2048)';
COMMENT ON COLUMN platform_config.max_badge_icon_url_length IS 'Maximum platform badge icon URL length (default: 2048)';

-- 2.6 messaging_config
CREATE TABLE IF NOT EXISTS messaging_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    paid_msg_platform_fee_bps BIGINT NOT NULL DEFAULT 250,
    paid_msg_treasury_fee_bps BIGINT NOT NULL DEFAULT 250,
    payment_expiration_ms BIGINT NOT NULL DEFAULT 2592000000,
    min_reply_chars BIGINT NOT NULL DEFAULT 6,
    max_dedupe_key_bytes BIGINT NOT NULL DEFAULT 256,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE messaging_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_messaging_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_messaging_config_time ON messaging_config;
CREATE TRIGGER set_messaging_config_time
BEFORE INSERT ON messaging_config
FOR EACH ROW
EXECUTE FUNCTION update_messaging_config_time();
SELECT create_hypertable('messaging_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'messaging_config_pkey'
    ) THEN
        ALTER TABLE messaging_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_messaging_config_time ON messaging_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_messaging_config_updated_by ON messaging_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_messaging_config_transaction_id ON messaging_config(transaction_id);
COMMENT ON TABLE messaging_config IS 'Tracks MessagingConfig changes over time (paid messaging fees + reply/dedupe limits). Each row represents a configuration update.';
COMMENT ON COLUMN messaging_config.paid_msg_platform_fee_bps IS 'Platform fee in bps taken on paid messages (default: 250; <= 10000)';
COMMENT ON COLUMN messaging_config.paid_msg_treasury_fee_bps IS 'Ecosystem treasury fee in bps taken on paid messages (default: 250; <= 10000)';
COMMENT ON COLUMN messaging_config.payment_expiration_ms IS 'Payment escrow expiration in ms (default: 2592000000 = 30 days; > 0)';
COMMENT ON COLUMN messaging_config.min_reply_chars IS 'Minimum character count for a paid message reply (default: 6; > 0)';
COMMENT ON COLUMN messaging_config.max_dedupe_key_bytes IS 'Maximum dedupe key byte length for paid message claims (default: 256; > 0)';

-- 2.7 paid_message_escrows (append-only lifecycle events for paid messaging)
CREATE TABLE IF NOT EXISTS paid_message_escrows (
    id SERIAL NOT NULL,
    group_id TEXT NOT NULL,
    seq BIGINT NOT NULL,
    payer TEXT NOT NULL,
    recipient TEXT NOT NULL,
    amount BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    platform_fee BIGINT,
    treasury_fee BIGINT,
    net_amount BIGINT,
    platform_fee_recipient TEXT,
    ecosystem_fee_recipient TEXT,
    reply_char_count BIGINT,
    created_at_ms BIGINT NOT NULL,
    claimed_at_ms BIGINT,
    refunded_at_ms BIGINT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE paid_message_escrows SET time = to_timestamp(created_at_ms / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_paid_message_escrows_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_paid_message_escrows_time ON paid_message_escrows;
CREATE TRIGGER set_paid_message_escrows_time
BEFORE INSERT ON paid_message_escrows
FOR EACH ROW
EXECUTE FUNCTION update_paid_message_escrows_time();
SELECT create_hypertable('paid_message_escrows', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'paid_message_escrows_pkey'
    ) THEN
        ALTER TABLE paid_message_escrows ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_paid_message_escrows_time ON paid_message_escrows(time DESC);
CREATE INDEX IF NOT EXISTS idx_paid_message_escrows_payer ON paid_message_escrows(payer, time DESC);
CREATE INDEX IF NOT EXISTS idx_paid_message_escrows_recipient ON paid_message_escrows(recipient, time DESC);
CREATE INDEX IF NOT EXISTS idx_paid_message_escrows_group_seq ON paid_message_escrows(group_id, seq, time DESC);
CREATE INDEX IF NOT EXISTS idx_paid_message_escrows_transaction_id ON paid_message_escrows(transaction_id);
COMMENT ON TABLE paid_message_escrows IS 'Append-only paid-message escrow lifecycle events (sent, claimed, settled, refunded).';

-- 2.8 messaging_agent_groups (sub-agent created messaging groups)
CREATE TABLE IF NOT EXISTS messaging_agent_groups (
    id SERIAL NOT NULL,
    group_id TEXT NOT NULL,
    creator_actor TEXT NOT NULL,
    creator_principal TEXT NOT NULL,
    creator_sub_agent_id TEXT,
    creator_identity_class BIGINT NOT NULL DEFAULT 0,
    organization_id TEXT,
    group_name TEXT NOT NULL,
    group_uuid TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);
UPDATE messaging_agent_groups SET time = to_timestamp(created_at_ms / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_messaging_agent_groups_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_messaging_agent_groups_time ON messaging_agent_groups;
CREATE TRIGGER set_messaging_agent_groups_time
BEFORE INSERT ON messaging_agent_groups
FOR EACH ROW
EXECUTE FUNCTION update_messaging_agent_groups_time();
SELECT create_hypertable('messaging_agent_groups', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'messaging_agent_groups_pkey'
    ) THEN
        ALTER TABLE messaging_agent_groups ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_messaging_agent_groups_time ON messaging_agent_groups(time DESC);
CREATE INDEX IF NOT EXISTS idx_messaging_agent_groups_org ON messaging_agent_groups(organization_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_messaging_agent_groups_creator ON messaging_agent_groups(creator_actor, time DESC);
CREATE INDEX IF NOT EXISTS idx_messaging_agent_groups_group_id ON messaging_agent_groups(group_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_messaging_agent_groups_transaction_id ON messaging_agent_groups(transaction_id);
COMMENT ON TABLE messaging_agent_groups IS 'Agent-created messaging groups indexed from AgentGroupCreated events.';

-- 2.9 Revenue views: add messaging revenue buckets
DROP VIEW IF EXISTS platform_revenue_summary CASCADE;
CREATE OR REPLACE VIEW platform_revenue_summary AS
SELECT
    platform_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'messaging' THEN amount ELSE 0 END) AS total_messaging_revenue,
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
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT payer_address) AS total_unique_payers,
    MAX(amount) AS largest_single_transaction,
    COUNT(DISTINCT DATE(time)) AS active_days,
    MAX(time) AS last_revenue_date
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '30 days'
GROUP BY creator_address
ORDER BY total_revenue DESC;

-- 2.10 Consolidate messaging indexer pipelines into single `messaging` watermark
DELETE FROM watermarks
WHERE pipeline IN ('paid_messaging_policy', 'messaging_config', 'messaging_payment');

-- ============================================================================
-- 3. METADATA STANDARDIZATION
--    Canonical block: updated_by, version, updated_at (BIGINT ms), time (trigger),
--    transaction_id. Append-only hypertables use PK (id, time).
-- ============================================================================

-- Helper: rename timestamp_ms -> updated_at when present
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'spot_config' AND column_name = 'timestamp_ms'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'spot_config' AND column_name = 'updated_at'
    ) THEN
        ALTER TABLE spot_config RENAME COLUMN timestamp_ms TO updated_at;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'mydata_config' AND column_name = 'timestamp_ms'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'mydata_config' AND column_name = 'updated_at'
    ) THEN
        ALTER TABLE mydata_config RENAME COLUMN timestamp_ms TO updated_at;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'insurance_config' AND column_name = 'timestamp_ms'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'insurance_config' AND column_name = 'updated_at'
    ) THEN
        ALTER TABLE insurance_config RENAME COLUMN timestamp_ms TO updated_at;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'ecosystem_treasury' AND column_name = 'timestamp_ms'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'ecosystem_treasury' AND column_name = 'updated_at'
    ) THEN
        ALTER TABLE ecosystem_treasury RENAME COLUMN timestamp_ms TO updated_at;
    END IF;
END $$;

-- Add version where missing
ALTER TABLE mydata_config ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 0;
ALTER TABLE poc_configuration ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 0;
ALTER TABLE ecosystem_treasury ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 0;

-- spt_config: normalize to standard metadata + hypertable
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'spt_config'
    ) THEN
        -- Drop TIMESTAMPTZ updated_at if present (conflicts with BIGINT standard)
        IF EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_schema = 'public' AND table_name = 'spt_config'
              AND column_name = 'updated_at' AND data_type = 'timestamp with time zone'
        ) THEN
            ALTER TABLE spt_config DROP COLUMN updated_at;
        END IF;
        IF EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_schema = 'public' AND table_name = 'spt_config' AND column_name = 'timestamp_ms'
        ) AND NOT EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_schema = 'public' AND table_name = 'spt_config' AND column_name = 'updated_at'
        ) THEN
            ALTER TABLE spt_config RENAME COLUMN timestamp_ms TO updated_at;
        END IF;
        ALTER TABLE spt_config ADD COLUMN IF NOT EXISTS updated_by TEXT NOT NULL DEFAULT '';
        ALTER TABLE spt_config ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 0;
        ALTER TABLE spt_config ADD COLUMN IF NOT EXISTS time TIMESTAMPTZ NOT NULL DEFAULT NOW();
        -- Unified spt_config: ConfigUpdatedEvent rows omit kill-switch audit fields.
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_admin_address;
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_reason;
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_timestamp;
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS valid_transaction_id;
        ALTER TABLE spt_config ALTER COLUMN admin_address SET DEFAULT '';
        ALTER TABLE spt_config ALTER COLUMN reason SET DEFAULT '';
        UPDATE spt_config SET updated_by = admin_address WHERE updated_by = '' AND admin_address IS NOT NULL;
        UPDATE spt_config SET time = to_timestamp(updated_at / 1000) WHERE updated_at > 0;
    END IF;
END $$;

-- Replace spot/mydata/insurance time triggers to use updated_at
CREATE OR REPLACE FUNCTION update_spot_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_spot_config_time ON spot_config;
CREATE TRIGGER set_spot_config_time
BEFORE INSERT ON spot_config FOR EACH ROW
EXECUTE FUNCTION update_spot_config_time();

CREATE OR REPLACE FUNCTION update_mydata_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_mydata_config_time ON mydata_config;
CREATE TRIGGER set_mydata_config_time
BEFORE INSERT ON mydata_config FOR EACH ROW
EXECUTE FUNCTION update_mydata_config_time();

CREATE OR REPLACE FUNCTION update_insurance_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_insurance_config_time ON insurance_config;
CREATE TRIGGER set_insurance_config_time
BEFORE INSERT ON insurance_config FOR EACH ROW
EXECUTE FUNCTION update_insurance_config_time();

CREATE OR REPLACE FUNCTION update_ecosystem_treasury_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_ecosystem_treasury_time ON ecosystem_treasury;
CREATE TRIGGER set_ecosystem_treasury_time
BEFORE INSERT ON ecosystem_treasury FOR EACH ROW
EXECUTE FUNCTION update_ecosystem_treasury_time();

CREATE OR REPLACE FUNCTION update_poc_configuration_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_poc_configuration_time ON poc_configuration;
CREATE TRIGGER set_poc_configuration_time
BEFORE INSERT ON poc_configuration FOR EACH ROW
EXECUTE FUNCTION update_poc_configuration_time();

CREATE OR REPLACE FUNCTION update_spt_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_spt_config_time ON spt_config;
CREATE TRIGGER set_spt_config_time
BEFORE INSERT ON spt_config FOR EACH ROW
EXECUTE FUNCTION update_spt_config_time();

-- poc_configuration -> hypertable (if not already)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'poc_configuration'
    ) AND NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_name = 'poc_configuration'
    ) THEN
        -- TimescaleDB rejects hypertables with a unique/PK that omits the partition column.
        ALTER TABLE poc_configuration DROP CONSTRAINT IF EXISTS poc_configuration_pkey;
        PERFORM create_hypertable('poc_configuration', 'time', if_not_exists => TRUE,
                                  create_default_indexes => FALSE,
                                  chunk_time_interval => INTERVAL '1 month');
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'poc_configuration'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'poc_configuration_pkey'
    ) THEN
        ALTER TABLE poc_configuration ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- spt_config -> hypertable (migrate from id-only PK)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'spt_config'
    ) AND NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_name = 'spt_config'
    ) THEN
        ALTER TABLE spt_config DROP CONSTRAINT IF EXISTS spt_config_pkey;
        PERFORM create_hypertable('spt_config', 'time', if_not_exists => TRUE,
                                  create_default_indexes => FALSE,
                                  chunk_time_interval => INTERVAL '1 month');
        IF NOT EXISTS (
            SELECT 1 FROM pg_constraint WHERE conname = 'spt_config_pkey'
        ) THEN
            ALTER TABLE spt_config ADD PRIMARY KEY (id, time);
        END IF;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_poc_configuration_time ON poc_configuration(time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_config_time ON spt_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_ecosystem_treasury_time ON ecosystem_treasury(time DESC);

-- Drop legacy spt_exchange_config hypertable; all SPT config lives in spt_config.
DROP TABLE IF EXISTS spt_exchange_config CASCADE;
DROP FUNCTION IF EXISTS update_spt_exchange_config_time();

CREATE OR REPLACE FUNCTION get_current_exchange_config()
RETURNS TABLE(
    post_threshold BIGINT,
    profile_threshold BIGINT,
    max_individual_reservation_bps BIGINT,
    trading_enabled BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        c.post_threshold,
        c.profile_threshold,
        c.max_individual_reservation_bps,
        c.trading_enabled
    FROM spt_config c
    ORDER BY c.time DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- 1.11 profiles on-chain fields cleanup
-- Drop legacy off-chain-only columns; unify location naming.
-- ============================================================================

ALTER TABLE profiles
  DROP COLUMN IF EXISTS raised_location,
  DROP COLUMN IF EXISTS phone,
  DROP COLUMN IF EXISTS email,
  DROP COLUMN IF EXISTS gender,
  DROP COLUMN IF EXISTS political_view,
  DROP COLUMN IF EXISTS religion,
  DROP COLUMN IF EXISTS education,
  DROP COLUMN IF EXISTS primary_language,
  DROP COLUMN IF EXISTS relationship_status,
  DROP COLUMN IF EXISTS sensitive_data_updated_at;

DO $$ BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'current_location'
  ) AND NOT EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'location'
  ) THEN
    ALTER TABLE profiles RENAME COLUMN current_location TO location;
  END IF;
END $$;

ALTER TABLE profiles ADD COLUMN IF NOT EXISTS location TEXT;

-- Backfill SPoT governance registry ID from bootstrap governance registry (registry_type = 2).
UPDATE spot_config sc
SET spot_governance_registry_id = gr.registry_id
FROM (
    SELECT registry_id
    FROM governance_registries
    WHERE registry_type = 2
    ORDER BY time DESC
    LIMIT 1
) gr
WHERE sc.spot_governance_registry_id IS NULL
  AND sc.time = (SELECT MAX(time) FROM spot_config);
