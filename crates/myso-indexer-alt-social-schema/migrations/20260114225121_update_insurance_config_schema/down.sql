-- Migration: Revert insurance_config schema changes
-- Version: 20260114225121
-- Purpose: Revert insurance_config table to previous structure

-- ============================================================================
-- 1. DROP SPOT RISK PRICING COLUMNS
-- ============================================================================

ALTER TABLE insurance_config
DROP COLUMN IF EXISTS min_spot_total_liquidity,
DROP COLUMN IF EXISTS max_coverage_fraction_of_option_bps,
DROP COLUMN IF EXISTS max_risk_multiplier_bps,
DROP COLUMN IF EXISTS min_premium_amount,
DROP COLUMN IF EXISTS spot_smoothing_per_option,
DROP COLUMN IF EXISTS implied_prob_floor_bps,
DROP COLUMN IF EXISTS odds_floor_1x,
DROP COLUMN IF EXISTS odds_cap_bps,
DROP COLUMN IF EXISTS liq_cap_bps,
DROP COLUMN IF EXISTS liq_ref_amount,
DROP COLUMN IF EXISTS exposure_cap_bps,
DROP COLUMN IF EXISTS exposure_k_bps;

-- ============================================================================
-- 2. ADD BACK TREASURY COLUMN
-- ============================================================================

-- Add back treasury column
ALTER TABLE insurance_config 
ADD COLUMN IF NOT EXISTS treasury TEXT NOT NULL DEFAULT '';

-- ============================================================================
-- 3. RENAME COLUMN BACK AND INVERT VALUES
-- ============================================================================

-- Add back paused column
ALTER TABLE insurance_config 
ADD COLUMN IF NOT EXISTS paused BOOLEAN NOT NULL DEFAULT FALSE;

-- Migrate data back: invert enable_flag values (enable_flag=true means paused=false)
UPDATE insurance_config 
SET paused = NOT enable_flag;

-- Drop the enable_flag column
ALTER TABLE insurance_config DROP COLUMN IF EXISTS enable_flag;
