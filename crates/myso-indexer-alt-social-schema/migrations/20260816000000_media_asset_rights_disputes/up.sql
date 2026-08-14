-- Media asset rights governance dispute linkage and audit trail

CREATE TABLE IF NOT EXISTS media_asset_governance_links (
    media_asset_id TEXT NOT NULL,
    proposal_id TEXT NOT NULL,
    submitter TEXT NOT NULL,
    claims_commitment BYTEA NOT NULL,
    status SMALLINT NOT NULL,
    related_post_id TEXT NULL,
    rights_disputes_submitted SMALLINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (media_asset_id, proposal_id, time)
);

CREATE INDEX IF NOT EXISTS idx_media_asset_governance_links_asset
    ON media_asset_governance_links (media_asset_id, time DESC);

CREATE INDEX IF NOT EXISTS idx_media_asset_governance_links_proposal
    ON media_asset_governance_links (proposal_id, time DESC);

CREATE INDEX IF NOT EXISTS idx_media_asset_governance_links_active
    ON media_asset_governance_links (media_asset_id, status, time DESC)
    WHERE status = 1;

CREATE TABLE IF NOT EXISTS media_asset_rights_updates (
    media_asset_id TEXT NOT NULL,
    rights_version BIGINT NOT NULL,
    proposal_id TEXT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (media_asset_id, rights_version, time)
);

CREATE INDEX IF NOT EXISTS idx_media_asset_rights_updates_asset
    ON media_asset_rights_updates (media_asset_id, time DESC);

ALTER TABLE poc_config
    ADD COLUMN IF NOT EXISTS media_asset_dispute_cost BIGINT NOT NULL DEFAULT 10000000000;

ALTER TABLE poc_config
    ADD COLUMN IF NOT EXISTS max_disputes_per_media_asset SMALLINT NOT NULL DEFAULT 2;

COMMENT ON COLUMN poc_config.media_asset_dispute_cost IS 'Treasury fee for initiating a media-asset rights governance dispute (default 10 MYSO)';
COMMENT ON COLUMN poc_config.max_disputes_per_media_asset IS 'Lifetime cap on rights disputes per media asset (mirrors Move u8)';
