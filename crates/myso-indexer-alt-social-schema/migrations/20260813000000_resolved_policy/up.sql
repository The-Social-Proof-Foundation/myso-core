-- Phase 3: resolved rights policy + obligations

CREATE TABLE IF NOT EXISTS media_asset_resolved_policies (
    media_asset_id TEXT NOT NULL,
    policy_version BIGINT NOT NULL,
    effective_rights BIGINT NOT NULL,
    derivatives_allowed BOOLEAN NOT NULL,
    attribution_required BOOLEAN NOT NULL,
    commercial_allowed BOOLEAN NOT NULL,
    lineage_json JSONB NOT NULL,
    lineage_hash TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (media_asset_id, policy_version, time)
);

CREATE TABLE IF NOT EXISTS media_asset_resolved_obligations (
    media_asset_id TEXT NOT NULL,
    policy_version BIGINT NOT NULL,
    obligation_index INT NOT NULL,
    beneficiary_asset_id TEXT NULL,
    beneficiary_address TEXT NOT NULL,
    share_bps BIGINT NOT NULL,
    source_relationship_id BIGINT NULL,
    source_license_instance_id TEXT NULL,
    obligation_kind SMALLINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (media_asset_id, policy_version, obligation_index, time)
);

CREATE INDEX IF NOT EXISTS idx_resolved_obligations_beneficiary
    ON media_asset_resolved_obligations (beneficiary_address, time DESC);
