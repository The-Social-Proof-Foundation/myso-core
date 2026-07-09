-- Claim → Market → Post redesign: semantic claims, shared markets, post links, creator payouts.

CREATE TABLE IF NOT EXISTS spot_claims (
    id SERIAL PRIMARY KEY,
    claim_object_id TEXT NOT NULL UNIQUE,
    semantic_claim_hash TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_spot_claims_semantic_hash ON spot_claims(semantic_claim_hash);

CREATE TABLE IF NOT EXISTS spot_markets (
    id SERIAL PRIMARY KEY,
    market_object_id TEXT NOT NULL UNIQUE,
    claim_object_id TEXT NOT NULL REFERENCES spot_claims(claim_object_id),
    market_key_hash TEXT NOT NULL UNIQUE,
    primary_post_id TEXT NOT NULL,
    primary_creator TEXT,
    status SMALLINT NOT NULL DEFAULT 1,
    outcome SMALLINT,
    betting_options JSONB NOT NULL DEFAULT '[]',
    option_escrow JSONB NOT NULL DEFAULT '{}',
    resolution_window_ms BIGINT,
    max_resolution_window_ms BIGINT,
    created_at_ms BIGINT NOT NULL,
    last_resolution_at_ms BIGINT,
    resolution_timestamp_ms BIGINT,
    creator_fee_total BIGINT,
    transaction_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_spot_markets_claim ON spot_markets(claim_object_id, status);
CREATE INDEX IF NOT EXISTS idx_spot_markets_primary_post ON spot_markets(primary_post_id);

CREATE TABLE IF NOT EXISTS spot_post_links (
    id SERIAL PRIMARY KEY,
    post_id TEXT NOT NULL UNIQUE,
    claim_object_id TEXT NOT NULL,
    market_object_id TEXT,
    link_kind TEXT NOT NULL DEFAULT 'primary',
    transaction_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_spot_post_links_claim ON spot_post_links(claim_object_id);
CREATE INDEX IF NOT EXISTS idx_spot_post_links_market ON spot_post_links(market_object_id)
    WHERE market_object_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS spot_creator_payouts (
    id SERIAL PRIMARY KEY,
    market_object_id TEXT NOT NULL,
    payout_id BIGINT NOT NULL,
    creator_address TEXT NOT NULL,
    referrer_post_id TEXT NOT NULL,
    amount BIGINT NOT NULL,
    expires_at_ms BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'accrued',
    ecosystem_amount BIGINT,
    platform_amount BIGINT,
    claimed_at_ms BIGINT,
    reclaimed_at_ms BIGINT,
    transaction_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_spot_creator_payouts_market_payout UNIQUE (market_object_id, payout_id)
);

CREATE INDEX IF NOT EXISTS idx_spot_creator_payouts_creator ON spot_creator_payouts(creator_address, status);
CREATE INDEX IF NOT EXISTS idx_spot_creator_payouts_market_status ON spot_creator_payouts(market_object_id, status);

ALTER TABLE spot_records
    ADD COLUMN IF NOT EXISTS claim_object_id TEXT,
    ADD COLUMN IF NOT EXISTS market_object_id TEXT,
    ADD COLUMN IF NOT EXISTS primary_post_id TEXT,
    ADD COLUMN IF NOT EXISTS market_key_hash TEXT,
    ADD COLUMN IF NOT EXISTS creator_fee_total BIGINT;

CREATE INDEX IF NOT EXISTS idx_spot_records_market_object_id
    ON spot_records(market_object_id)
    WHERE market_object_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_spot_records_claim_object_id
    ON spot_records(claim_object_id)
    WHERE claim_object_id IS NOT NULL;

ALTER TABLE spot_resolutions
    ADD COLUMN IF NOT EXISTS claim_object_id TEXT,
    ADD COLUMN IF NOT EXISTS market_object_id TEXT,
    ADD COLUMN IF NOT EXISTS creator_fee_total BIGINT;

ALTER TABLE spot_bets
    ADD COLUMN IF NOT EXISTS market_object_id TEXT,
    ADD COLUMN IF NOT EXISTS referrer_post_id TEXT;

CREATE INDEX IF NOT EXISTS idx_spot_bets_market ON spot_bets(market_object_id, time)
    WHERE market_object_id IS NOT NULL;

ALTER TABLE spot_config
    ADD COLUMN IF NOT EXISTS creator_fee_bps BIGINT,
    ADD COLUMN IF NOT EXISTS creator_claim_window_ms BIGINT,
    ADD COLUMN IF NOT EXISTS expired_creator_ecosystem_bps BIGINT;

UPDATE spot_config
SET creator_fee_bps = COALESCE(creator_fee_bps, 0),
    creator_claim_window_ms = COALESCE(creator_claim_window_ms, 0),
    expired_creator_ecosystem_bps = COALESCE(expired_creator_ecosystem_bps, 0)
WHERE creator_fee_bps IS NULL
   OR creator_claim_window_ms IS NULL
   OR expired_creator_ecosystem_bps IS NULL;

ALTER TABLE posts
    ADD COLUMN IF NOT EXISTS spot_claim_id TEXT;

CREATE INDEX IF NOT EXISTS idx_posts_spot_claim_id ON posts(spot_claim_id, time)
    WHERE spot_claim_id IS NOT NULL;

COMMENT ON TABLE spot_claims IS 'On-chain SpotClaim objects indexed by claim object address';
COMMENT ON TABLE spot_markets IS 'On-chain SpotMarket objects (shared prediction markets per semantic claim)';
COMMENT ON TABLE spot_post_links IS 'Links posts to semantic claims and optional open markets';
COMMENT ON TABLE spot_creator_payouts IS 'Creator fee payouts accrued on market resolution';
COMMENT ON COLUMN posts.spot_claim_id IS 'Address of the linked SpotClaim object (set when post joins a claim)';
COMMENT ON COLUMN spot_records.claim_object_id IS 'Linked SpotClaim object address';
COMMENT ON COLUMN spot_records.market_object_id IS 'Linked SpotMarket object address';
COMMENT ON COLUMN spot_records.primary_post_id IS 'Primary post that opened the market (may differ from spot_records.post_id for legacy rows)';
