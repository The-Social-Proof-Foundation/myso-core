-- Rollback pg_textsearch BM25 indexes and extension

DROP INDEX IF EXISTS idx_posts_content_bm25;
DROP INDEX IF EXISTS idx_profiles_search_bm25;
ALTER TABLE profiles DROP COLUMN IF EXISTS search_text;
DROP EXTENSION IF EXISTS pg_textsearch;
