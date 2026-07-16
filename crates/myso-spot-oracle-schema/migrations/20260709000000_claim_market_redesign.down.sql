DROP TRIGGER IF EXISTS trg_canonical_claims_hash_sync ON canonical_claims;
DROP FUNCTION IF EXISTS sync_canonical_claim_hash();

DROP TABLE IF EXISTS post_claim_links;
DROP TABLE IF EXISTS spot_markets;
DROP TABLE IF EXISTS spot_claims;

DROP INDEX IF EXISTS idx_canonical_semantic_hash;
DROP INDEX IF EXISTS idx_canonical_market_key_hash;

ALTER TABLE canonical_claims
    DROP COLUMN IF EXISTS semantic_claim_hash,
    DROP COLUMN IF EXISTS market_key_hash;

CREATE UNIQUE INDEX IF NOT EXISTS markets_post_id_key ON markets(post_id);
