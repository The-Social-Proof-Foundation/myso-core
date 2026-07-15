ALTER TABLE spot_post_links DROP CONSTRAINT IF EXISTS spot_post_links_post_claim_uniq;
DROP INDEX IF EXISTS idx_spot_post_links_post_claim;
ALTER TABLE spot_post_links DROP COLUMN IF EXISTS claim_index;
ALTER TABLE spot_post_links DROP COLUMN IF EXISTS policy_hash;

DROP TABLE IF EXISTS spot_claim_verdicts;
DROP TABLE IF EXISTS spot_post_analyses;

ALTER TABLE posts
    DROP COLUMN IF EXISTS spot_detected_claim_count,
    DROP COLUMN IF EXISTS spot_rejected_claim_count,
    DROP COLUMN IF EXISTS spot_truncated_claim_count,
    DROP COLUMN IF EXISTS spot_future_accepted_count,
    DROP COLUMN IF EXISTS spot_past_verified_count,
    DROP COLUMN IF EXISTS spot_max_claim_per_post_applied,
    DROP COLUMN IF EXISTS spot_claim_indexes,
    DROP COLUMN IF EXISTS spot_claim_ids,
    DROP COLUMN IF EXISTS spot_market_ids,
    DROP COLUMN IF EXISTS spot_policy_hashes,
    DROP COLUMN IF EXISTS spot_claim_manifest_hash,
    DROP COLUMN IF EXISTS spot_veracity_manifest_hash,
    DROP COLUMN IF EXISTS spot_analysis_tx_digest,
    DROP COLUMN IF EXISTS spot_analyzed_checkpoint;
