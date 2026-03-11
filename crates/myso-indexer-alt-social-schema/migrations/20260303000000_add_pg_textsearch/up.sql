-- Migration: Add full-text search indexes for profiles and posts
-- Version: 20260303000000
-- Purpose: Enable relevance-ranked search for posts (content) and profiles (username, display_name)
-- Uses standard PostgreSQL tsvector/GIN (works on Railway, managed Postgres, etc.)
-- Local dev with pg_textsearch uses custom Dockerfile.postgres for BM25

-- Drop legacy pg_textsearch indexes if migrating from previous version
DROP INDEX IF EXISTS idx_posts_content_bm25;
DROP INDEX IF EXISTS idx_profiles_search_bm25;

-- ============================================================================
-- 1. PROFILES: Add search_text generated column and GIN index
-- ============================================================================
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS search_text TEXT
    GENERATED ALWAYS AS (
        coalesce(username, '') || ' ' || coalesce(display_name, '')
    ) STORED;

CREATE INDEX IF NOT EXISTS idx_profiles_search_gin ON profiles
    USING GIN(to_tsvector('english', coalesce(search_text, '')));

-- ============================================================================
-- 2. POSTS: GIN index on content for full-text search
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_posts_content_gin ON posts
    USING GIN(to_tsvector('english', coalesce(content, '')));
