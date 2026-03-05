-- Migration: Add pg_textsearch extension and BM25 indexes for full-text search
-- Version: 20260303000000
-- Purpose: Enable BM25 relevance-ranked search for posts (content) and profiles (username, display_name)

-- ============================================================================
-- 1. ENABLE PG_TEXTSEARCH EXTENSION
-- ============================================================================
-- Requires shared_preload_libraries = 'timescaledb,pg_textsearch' in postgresql.conf
CREATE EXTENSION IF NOT EXISTS pg_textsearch;

-- ============================================================================
-- 2. PROFILES: Add search_text generated column and BM25 index
-- ============================================================================
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS search_text TEXT
    GENERATED ALWAYS AS (
        coalesce(username, '') || ' ' || coalesce(display_name, '')
    ) STORED;

CREATE INDEX IF NOT EXISTS idx_profiles_search_bm25 ON profiles
    USING bm25(search_text) WITH (text_config='english');

-- ============================================================================
-- 3. POSTS: BM25 index on content
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_posts_content_bm25 ON posts
    USING bm25(content) WITH (text_config='english');
