-- Phase 2: derivative graph + license registry (payload-complete finalize events)

ALTER TABLE media_assets ADD COLUMN IF NOT EXISTS asset_kind SMALLINT NOT NULL DEFAULT 0;

CREATE TABLE IF NOT EXISTS media_asset_derivative_edges (
    child_asset_id TEXT NOT NULL,
    parent_asset_id TEXT NOT NULL,
    relationship_id BIGINT NOT NULL,
    relationship_type SMALLINT NOT NULL,
    license_instance_id TEXT NOT NULL,
    template_version_id TEXT NOT NULL,
    parent_share_bps BIGINT NOT NULL,
    ancestry_version BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (child_asset_id, parent_asset_id, relationship_id, time)
);

CREATE INDEX IF NOT EXISTS idx_deriv_edges_parent
    ON media_asset_derivative_edges (parent_asset_id, time DESC);

CREATE INDEX IF NOT EXISTS idx_deriv_edges_child
    ON media_asset_derivative_edges (child_asset_id, time DESC);

CREATE TABLE IF NOT EXISTS media_asset_ancestry_snapshots (
    media_asset_id TEXT NOT NULL,
    ancestry_version BIGINT NOT NULL,
    ancestor_ids JSONB NOT NULL,
    ancestry_hash TEXT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (media_asset_id, ancestry_version, time)
);

CREATE TABLE IF NOT EXISTS license_template_versions (
    template_version_id TEXT NOT NULL,
    family_id TEXT NOT NULL,
    version BIGINT NOT NULL,
    creator TEXT NOT NULL,
    granted_rights BIGINT NOT NULL,
    allow_derivatives BOOLEAN NOT NULL,
    attribution_required BOOLEAN NOT NULL,
    royalty_bps BIGINT NOT NULL,
    derivative_royalty_bps BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (template_version_id, time)
);

CREATE TABLE IF NOT EXISTS license_instances (
    license_instance_id TEXT NOT NULL,
    template_version_id TEXT NOT NULL,
    licensor_asset_id TEXT NOT NULL,
    licensee TEXT NOT NULL,
    status SMALLINT NOT NULL,
    accepted_at BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (license_instance_id, time)
);

CREATE INDEX IF NOT EXISTS idx_license_instances_licensor
    ON license_instances (licensor_asset_id, time DESC);

CREATE INDEX IF NOT EXISTS idx_license_instances_licensee
    ON license_instances (licensee, time DESC);
