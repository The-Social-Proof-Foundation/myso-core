-- Migration: Add selected_ecosystem_badge_id to profiles
-- Version: 20260302000000
-- Purpose: Add selected_ecosystem_badge_id for ecosystem badge display selection (parallel to selected_badge_id)

-- ============================================================================
-- 1. ADD COLUMN
-- ============================================================================

ALTER TABLE profiles ADD COLUMN IF NOT EXISTS selected_ecosystem_badge_id VARCHAR NULL;

-- ============================================================================
-- 2. CREATE INDEX
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_profiles_selected_ecosystem_badge_id
    ON profiles (selected_ecosystem_badge_id) WHERE selected_ecosystem_badge_id IS NOT NULL;

-- ============================================================================
-- 3. DOCUMENTATION
-- ============================================================================

COMMENT ON COLUMN profiles.selected_ecosystem_badge_id IS 'The badge_id of the currently selected ecosystem badge for this profile. NULL means no ecosystem badge is selected. Ecosystem badges have badge_id prefix ecosystem_badge_.';
