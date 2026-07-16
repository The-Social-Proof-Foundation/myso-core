-- Claim → Market → Post redesign: semantic claims, shared markets, post links.

CREATE TABLE spot_claims (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    semantic_claim_hash VARCHAR(128) NOT NULL UNIQUE,
    spot_claim_object_id VARCHAR(128),
    canonical_claim_id UUID REFERENCES canonical_claims(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE spot_markets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    claim_id UUID NOT NULL REFERENCES spot_claims(id) ON DELETE CASCADE,
    market_key_hash VARCHAR(128) NOT NULL UNIQUE,
    spot_market_object_id VARCHAR(128),
    deadline TIMESTAMPTZ,
    betting_options JSONB NOT NULL DEFAULT '[]',
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    resolver_definition_id UUID REFERENCES resolver_definitions(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_spot_markets_claim ON spot_markets(claim_id, status);

CREATE TABLE post_claim_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id VARCHAR(128) NOT NULL UNIQUE,
    claim_id UUID NOT NULL REFERENCES spot_claims(id) ON DELETE CASCADE,
    market_id UUID REFERENCES spot_markets(id) ON DELETE SET NULL,
    oracle_market_id UUID REFERENCES markets(id) ON DELETE SET NULL,
    review_id UUID REFERENCES oracle_reviews(id),
    link_kind VARCHAR(32) NOT NULL DEFAULT 'primary',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_post_claim_links_claim ON post_claim_links(claim_id);

ALTER TABLE canonical_claims
    ADD COLUMN IF NOT EXISTS semantic_claim_hash VARCHAR(128),
    ADD COLUMN IF NOT EXISTS market_key_hash VARCHAR(128);

CREATE UNIQUE INDEX IF NOT EXISTS idx_canonical_semantic_hash
    ON canonical_claims(semantic_claim_hash)
    WHERE semantic_claim_hash IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_canonical_market_key_hash
    ON canonical_claims(market_key_hash)
    WHERE market_key_hash IS NOT NULL;

-- Legacy claim_hash (initial schema NOT NULL) mirrors market_key_hash for inserts that only set the new columns.
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

-- Relax one-market-per-post; post identity lives in post_claim_links.
ALTER TABLE markets DROP CONSTRAINT IF EXISTS markets_post_id_key;
CREATE INDEX IF NOT EXISTS idx_markets_post_id ON markets(post_id);
