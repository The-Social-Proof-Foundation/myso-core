-- Migration: Rollback poc_config table changes
-- Version: 20251227023815
-- Purpose: Remove max_reasoning_length and max_evidence_urls columns

-- Remove the new columns
ALTER TABLE poc_config 
DROP COLUMN IF EXISTS max_reasoning_length;

ALTER TABLE poc_config 
DROP COLUMN IF EXISTS max_evidence_urls;

