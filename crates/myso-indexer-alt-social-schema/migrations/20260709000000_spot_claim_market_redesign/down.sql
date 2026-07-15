DROP INDEX IF EXISTS idx_posts_spot_claim_id;
ALTER TABLE posts DROP COLUMN IF EXISTS spot_claim_id;

ALTER TABLE spot_config
    DROP COLUMN IF EXISTS creator_fee_bps,
    DROP COLUMN IF EXISTS creator_claim_window_ms,
    DROP COLUMN IF EXISTS expired_creator_ecosystem_bps,
    DROP COLUMN IF EXISTS max_bets_per_record,
    DROP COLUMN IF EXISTS max_claim_per_post;

DROP INDEX IF EXISTS idx_spot_bets_market;
ALTER TABLE spot_bets
    DROP COLUMN IF EXISTS market_object_id,
    DROP COLUMN IF EXISTS referrer_post_id;

ALTER TABLE spot_resolutions
    DROP COLUMN IF EXISTS claim_object_id,
    DROP COLUMN IF EXISTS market_object_id,
    DROP COLUMN IF EXISTS creator_fee_total;

DROP INDEX IF EXISTS idx_spot_records_claim_object_id;
DROP INDEX IF EXISTS idx_spot_records_market_object_id;
ALTER TABLE spot_records
    DROP COLUMN IF EXISTS claim_object_id,
    DROP COLUMN IF EXISTS market_object_id,
    DROP COLUMN IF EXISTS primary_post_id,
    DROP COLUMN IF EXISTS market_key_hash,
    DROP COLUMN IF EXISTS creator_fee_total;

DROP TABLE IF EXISTS spot_creator_payouts;
DROP TABLE IF EXISTS spot_post_links;
DROP TABLE IF EXISTS spot_markets;
DROP TABLE IF EXISTS spot_claims;
