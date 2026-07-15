-- Multi-claim SPoT: SPoT is now always-on and multi-claim. The per-post opt-in flag and the
-- singular claim/market pointers on `posts` are removed; per-claim links live in
-- `spot_post_links` and (in the multi-claim projection) the `posts` analysis columns.

DROP INDEX IF EXISTS idx_posts_enable_spot;
DROP INDEX IF EXISTS idx_posts_spot_id;
DROP INDEX IF EXISTS idx_posts_spot_claim_id;

ALTER TABLE posts DROP COLUMN IF EXISTS enable_spot;
ALTER TABLE posts DROP COLUMN IF EXISTS spot_id;
ALTER TABLE posts DROP COLUMN IF EXISTS spot_claim_id;

-- Always-on analysis lifecycle: 0=pending, 1=completed, 2=completed_no_actionable.
-- (The full multi-claim denorm columns are added in the multi-claim projection migration.)
ALTER TABLE posts ADD COLUMN IF NOT EXISTS spot_analysis_status SMALLINT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_posts_spot_analysis_pending
    ON posts(created_at ASC)
    WHERE spot_analysis_status = 0 AND deleted_at IS NULL;
