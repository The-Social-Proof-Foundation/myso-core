-- Phase 4: post-level application enforcement (bindings, decisions, denials)

ALTER TABLE posts ADD COLUMN IF NOT EXISTS embedded_bindings JSONB NULL;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS usage_decisions JSONB NULL;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS usage_denials JSONB NULL;

CREATE TABLE IF NOT EXISTS post_usage_decision_events (
    post_id TEXT NOT NULL,
    binding_id BIGINT NOT NULL,
    playback_permitted BOOLEAN NOT NULL,
    payout_permitted BOOLEAN NOT NULL,
    policy_reason_code SMALLINT NOT NULL,
    policy_version BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (post_id, binding_id, time)
);

CREATE INDEX IF NOT EXISTS idx_post_usage_decisions_post
    ON post_usage_decision_events (post_id, time DESC);
