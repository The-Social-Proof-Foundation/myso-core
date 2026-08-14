-- MediaAsset-centric PoC registry (greenfield schema additions)

CREATE TABLE IF NOT EXISTS media_assets (
    media_asset_id TEXT NOT NULL,
    content_commitment BYTEA NOT NULL,
    media_type SMALLINT NOT NULL,
    originality_status SMALLINT NOT NULL DEFAULT 0,
    provenance_status SMALLINT NOT NULL DEFAULT 0,
    lineage_parent_id TEXT NULL,
    rights_version BIGINT NOT NULL DEFAULT 1,
    economics_version BIGINT NOT NULL DEFAULT 1,
    registered_by TEXT NOT NULL,
    registered_at BIGINT NOT NULL,
    verified_at BIGINT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (media_asset_id, time)
);

CREATE INDEX IF NOT EXISTS idx_media_assets_time
    ON media_assets (time DESC);

CREATE TABLE IF NOT EXISTS fingerprint_observations (
    fingerprint_commitment BYTEA NOT NULL,
    media_asset_id TEXT NOT NULL,
    linked_at BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (fingerprint_commitment, media_asset_id, time)
);

CREATE INDEX IF NOT EXISTS idx_fingerprint_observations_asset
    ON fingerprint_observations (media_asset_id);

CREATE TABLE IF NOT EXISTS media_asset_usages (
    container_id TEXT NOT NULL,
    container_type SMALLINT NOT NULL,
    asset_id TEXT NOT NULL,
    usage_class SMALLINT NOT NULL,
    position SMALLINT NOT NULL DEFAULT 0,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (container_id, asset_id, usage_class, position, time)
);

CREATE INDEX IF NOT EXISTS idx_media_asset_usages_asset
    ON media_asset_usages (asset_id, time DESC);

CREATE TABLE IF NOT EXISTS composition_analysis_records (
    post_id TEXT NOT NULL,
    analyzed_at BIGINT NOT NULL,
    usage_context SMALLINT NOT NULL DEFAULT 0,
    composition_status SMALLINT NOT NULL,
    monetization_status SMALLINT NOT NULL,
    analysis_json JSONB NOT NULL DEFAULT '{}',
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (post_id, analyzed_at, time)
);

CREATE TABLE IF NOT EXISTS revenue_manifests (
    post_id TEXT NOT NULL,
    manifest_version BIGINT NOT NULL,
    entries_json JSONB NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (post_id, manifest_version, time)
);

-- Post composition fields (MediaAsset-centric model)
ALTER TABLE posts ADD COLUMN IF NOT EXISTS composition_status SMALLINT NULL;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS monetization_status SMALLINT NULL;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS media_asset_ids JSONB NULL;

CREATE INDEX IF NOT EXISTS idx_posts_composition_status
    ON posts (composition_status, time DESC)
    WHERE composition_status IS NOT NULL;
