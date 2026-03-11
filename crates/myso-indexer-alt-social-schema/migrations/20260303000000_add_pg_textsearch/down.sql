-- Rollback full-text search indexes

DROP INDEX IF EXISTS idx_posts_content_gin;
DROP INDEX IF EXISTS idx_profiles_search_gin;
ALTER TABLE profiles DROP COLUMN IF EXISTS search_text;
