-- Restore columns dropped by CADR schema cleanup (rollback).

DROP TRIGGER IF EXISTS trg_canonical_claims_hash_sync ON canonical_claims;
DROP FUNCTION IF EXISTS sync_canonical_claim_hash();

ALTER TABLE spot_trusted_sources
    ADD COLUMN IF NOT EXISTS health_healthy BOOLEAN,
    ADD COLUMN IF NOT EXISTS health_message TEXT,
    ADD COLUMN IF NOT EXISTS source_url TEXT;

ALTER TABLE spot_markets
    ADD COLUMN IF NOT EXISTS winner_pool BIGINT,
    ADD COLUMN IF NOT EXISTS creator_fee_total BIGINT,
    ADD COLUMN IF NOT EXISTS resolution_timestamp_ms BIGINT;

ALTER TABLE post_claim_links
    ADD COLUMN IF NOT EXISTS similarity_bps INTEGER NOT NULL DEFAULT 10000;

ALTER TABLE resolver_state ADD COLUMN IF NOT EXISTS maturity_at TIMESTAMPTZ;

-- Re-create hash sync trigger after rollback (matches claim_market_redesign).
CREATE OR REPLACE FUNCTION sync_canonical_claim_hash()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.market_key_hash IS NOT NULL THEN
        NEW.claim_hash := NEW.market_key_hash;
    ELSIF NEW.claim_hash IS NOT NULL AND (NEW.market_key_hash IS NULL OR NEW.market_key_hash = '') THEN
        NEW.market_key_hash := NEW.claim_hash;
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_canonical_claims_hash_sync
    BEFORE INSERT OR UPDATE ON canonical_claims
    FOR EACH ROW
    EXECUTE FUNCTION sync_canonical_claim_hash();
