-- Phase 5: PoC derivative-work discovery proposals

CREATE TABLE IF NOT EXISTS detected_asset_relationships (
    proposal_id TEXT NOT NULL,
    accused_pending_id TEXT NOT NULL,
    accused_asset_id TEXT NULL,
    original_asset_id TEXT NOT NULL,
    similarity_bps BIGINT NOT NULL,
    evidence_commitment BYTEA NULL,
    detected_by TEXT NOT NULL,
    detected_at BIGINT NOT NULL,
    status SMALLINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (proposal_id, time)
);

CREATE INDEX IF NOT EXISTS idx_detected_relationships_accused_pending
    ON detected_asset_relationships (accused_pending_id, time DESC);

CREATE INDEX IF NOT EXISTS idx_detected_relationships_accused_asset
    ON detected_asset_relationships (accused_asset_id, time DESC)
    WHERE accused_asset_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_detected_relationships_original
    ON detected_asset_relationships (original_asset_id, time DESC);
