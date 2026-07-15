-- Multi-claim SPoT projection: per-post analysis denorm on `posts`, an analysis sidecar, a
-- first-class past-verdict table, and multi-row per-claim links.

-- posts: analysis denorm (rewritten atomically only by the batch-finalize handler).
ALTER TABLE posts
    ADD COLUMN IF NOT EXISTS spot_detected_claim_count BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS spot_rejected_claim_count BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS spot_truncated_claim_count BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS spot_future_accepted_count BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS spot_past_verified_count BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS spot_max_claim_per_post_applied BIGINT NOT NULL DEFAULT 10,
    ADD COLUMN IF NOT EXISTS spot_claim_indexes JSONB NOT NULL DEFAULT '[]',
    ADD COLUMN IF NOT EXISTS spot_claim_ids JSONB NOT NULL DEFAULT '[]',
    ADD COLUMN IF NOT EXISTS spot_market_ids JSONB NOT NULL DEFAULT '[]',
    ADD COLUMN IF NOT EXISTS spot_policy_hashes JSONB NOT NULL DEFAULT '[]',
    ADD COLUMN IF NOT EXISTS spot_claim_manifest_hash TEXT,
    ADD COLUMN IF NOT EXISTS spot_veracity_manifest_hash TEXT,
    ADD COLUMN IF NOT EXISTS spot_analysis_tx_digest TEXT,
    ADD COLUMN IF NOT EXISTS spot_analyzed_checkpoint BIGINT;

-- Analysis sidecar: one row per post; drives pending/status reads without scanning JSONB.
CREATE TABLE IF NOT EXISTS spot_post_analyses (
    post_id TEXT PRIMARY KEY,
    status SMALLINT NOT NULL DEFAULT 0,
    detected_claim_count BIGINT NOT NULL DEFAULT 0,
    rejected_claim_count BIGINT NOT NULL DEFAULT 0,
    truncated_claim_count BIGINT NOT NULL DEFAULT 0,
    future_accepted_count BIGINT NOT NULL DEFAULT 0,
    past_verified_count BIGINT NOT NULL DEFAULT 0,
    max_claim_per_post_applied BIGINT NOT NULL DEFAULT 10,
    claim_manifest_hash TEXT,
    veracity_manifest_hash TEXT,
    finalize_tx_digest TEXT,
    checkpoint BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Past-claim verdicts (first-class rows). verdict: 1=true, 2=false, 3=unverifiable.
CREATE TABLE IF NOT EXISTS spot_claim_verdicts (
    id SERIAL PRIMARY KEY,
    post_id TEXT NOT NULL,
    claim_index BIGINT NOT NULL,
    time_class TEXT NOT NULL DEFAULT 'past',
    verdict SMALLINT NOT NULL,
    semantic_claim_hash TEXT,
    policy_hash TEXT NOT NULL DEFAULT '',
    evidence_manifest_hash TEXT NOT NULL DEFAULT '',
    related_market_object_id TEXT,
    related_claim_object_id TEXT,
    evidence_urls JSONB NOT NULL DEFAULT '[]',
    summary TEXT,
    transaction_id TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (post_id, claim_index)
);
CREATE INDEX IF NOT EXISTS idx_spot_claim_verdicts_market ON spot_claim_verdicts(related_market_object_id);
CREATE INDEX IF NOT EXISTS idx_spot_claim_verdicts_verdict ON spot_claim_verdicts(verdict);
CREATE INDEX IF NOT EXISTS idx_spot_claim_verdicts_post ON spot_claim_verdicts(post_id);

-- spot_post_links: relax single-link-per-post to per-claim rows.
ALTER TABLE spot_post_links ADD COLUMN IF NOT EXISTS claim_index BIGINT NOT NULL DEFAULT 0;
ALTER TABLE spot_post_links ADD COLUMN IF NOT EXISTS policy_hash TEXT NOT NULL DEFAULT '';
ALTER TABLE spot_post_links DROP CONSTRAINT IF EXISTS spot_post_links_post_id_key;
ALTER TABLE spot_post_links DROP CONSTRAINT IF EXISTS spot_post_links_post_claim_uniq;
ALTER TABLE spot_post_links ADD CONSTRAINT spot_post_links_post_claim_uniq UNIQUE (post_id, claim_index);
CREATE INDEX IF NOT EXISTS idx_spot_post_links_post_claim ON spot_post_links(post_id, claim_index);
