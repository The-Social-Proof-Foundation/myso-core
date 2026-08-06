-- Enable pg_textsearch (BM25) for social discovery search.
-- Extension binary is provided by the social indexer Postgres image (Dockerfile.postgres).

CREATE EXTENSION IF NOT EXISTS pg_textsearch;

-- Profiles: username + display_name + bio
CREATE INDEX IF NOT EXISTS idx_profiles_bm25
ON profiles
USING bm25 (
    (coalesce(username, '') || ' ' || coalesce(display_name, '') || ' ' || coalesce(bio, ''))
)
WITH (text_config = 'english');

-- Platforms: name + tagline + description
CREATE INDEX IF NOT EXISTS idx_platforms_bm25
ON platforms
USING bm25 (
    (name || ' ' || coalesce(tagline, '') || ' ' || coalesce(description, ''))
)
WITH (text_config = 'english');

-- Posts: content (hypertable; chunk-local BM25 stats)
CREATE INDEX IF NOT EXISTS idx_posts_content_bm25
ON posts
USING bm25 (content)
WITH (text_config = 'english');
