DROP INDEX IF EXISTS idx_posts_content_bm25;
DROP INDEX IF EXISTS idx_platforms_bm25;
DROP INDEX IF EXISTS idx_profiles_bm25;
-- Leave pg_textsearch extension installed; other DBs on the cluster may use it.
