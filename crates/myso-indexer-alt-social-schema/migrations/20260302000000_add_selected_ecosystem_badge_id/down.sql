-- Migration: Remove selected_ecosystem_badge_id from profiles
-- Version: 20260302000000
-- Purpose: Rollback - Remove selected_ecosystem_badge_id column

DROP INDEX IF EXISTS idx_profiles_selected_ecosystem_badge_id;
ALTER TABLE profiles DROP COLUMN IF EXISTS selected_ecosystem_badge_id;
