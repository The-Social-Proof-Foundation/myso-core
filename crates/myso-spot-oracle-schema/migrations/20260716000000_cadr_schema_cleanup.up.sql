-- CADR schema cleanup for databases created before unused columns were removed from earlier migrations.
-- Fresh installs skip most of these drops; claim_hash sync trigger is ensured for upgrade paths.

ALTER TABLE spot_trusted_sources
    DROP COLUMN IF EXISTS health_healthy,
    DROP COLUMN IF EXISTS health_message,
    DROP COLUMN IF EXISTS source_url;

ALTER TABLE spot_markets
    DROP COLUMN IF EXISTS winner_pool,
    DROP COLUMN IF EXISTS creator_fee_total,
    DROP COLUMN IF EXISTS resolution_timestamp_ms;

ALTER TABLE post_claim_links DROP COLUMN IF EXISTS similarity_bps;

ALTER TABLE resolver_state DROP COLUMN IF EXISTS maturity_at;

-- Ensure legacy claim_hash stays populated when only market_key_hash is written.
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

DROP TRIGGER IF EXISTS trg_canonical_claims_hash_sync ON canonical_claims;
CREATE TRIGGER trg_canonical_claims_hash_sync
    BEFORE INSERT OR UPDATE ON canonical_claims
    FOR EACH ROW
    EXECUTE FUNCTION sync_canonical_claim_hash();
