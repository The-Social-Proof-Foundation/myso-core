DROP INDEX IF EXISTS idx_posts_spot_analysis_pending;
ALTER TABLE posts DROP COLUMN IF EXISTS spot_analysis_status;

ALTER TABLE posts ADD COLUMN IF NOT EXISTS enable_spot BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS spot_id TEXT;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS spot_claim_id TEXT;

CREATE INDEX IF NOT EXISTS idx_posts_enable_spot ON posts(enable_spot, time) WHERE enable_spot = true;
CREATE INDEX IF NOT EXISTS idx_posts_spot_id ON posts(spot_id, time) WHERE spot_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_posts_spot_claim_id ON posts(spot_claim_id, time) WHERE spot_claim_id IS NOT NULL;
