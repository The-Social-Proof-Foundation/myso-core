-- PoC username beneficiary provisioning, identity links, and audit events.

CREATE TABLE IF NOT EXISTS poc_username_beneficiaries (
    beneficiary_id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    status SMALLINT NOT NULL,
    creator_identity_source SMALLINT NOT NULL,
    creator_identity_hash TEXT NOT NULL,
    vault_routing_key TEXT NOT NULL,
    vault_id TEXT NOT NULL,
    required_x_handle TEXT NOT NULL,
    oracle_evidence_hash TEXT NOT NULL DEFAULT '',
    provisioned_at_ms BIGINT NOT NULL,
    provisioned_by TEXT NOT NULL,
    claimed_profile_id TEXT NULL,
    claimed_by TEXT NULL,
    claimed_at_ms BIGINT NULL,
    ended_at_ms BIGINT NULL,
    ended_by TEXT NULL,
    end_reason_code SMALLINT NULL,
    join_referrer TEXT NULL,
    join_referral_paid BOOLEAN NOT NULL DEFAULT FALSE,
    join_referral_paid_at_ms BIGINT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_poc_username_beneficiaries_username ON poc_username_beneficiaries (username);
CREATE INDEX IF NOT EXISTS idx_poc_username_beneficiaries_vault ON poc_username_beneficiaries (vault_id);
CREATE INDEX IF NOT EXISTS idx_poc_username_beneficiaries_vault_routing_key
    ON poc_username_beneficiaries (vault_routing_key);
CREATE INDEX IF NOT EXISTS idx_poc_username_beneficiaries_active
    ON poc_username_beneficiaries (username, status)
    WHERE status = 1;

CREATE TABLE IF NOT EXISTS poc_creator_identity_links (
    creator_identity_source SMALLINT NOT NULL,
    creator_identity_hash TEXT NOT NULL,
    wallet_address TEXT NOT NULL,
    beneficiary_id TEXT NOT NULL,
    linked_at_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (creator_identity_source, creator_identity_hash)
);

CREATE INDEX IF NOT EXISTS idx_poc_creator_identity_links_wallet ON poc_creator_identity_links (wallet_address);
CREATE INDEX IF NOT EXISTS idx_poc_creator_identity_links_beneficiary ON poc_creator_identity_links (beneficiary_id);

CREATE TABLE IF NOT EXISTS poc_username_beneficiary_events (
    id BIGSERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    beneficiary_id TEXT NULL,
    username TEXT NULL,
    payload_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    transaction_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_poc_username_beneficiary_events_beneficiary_time
    ON poc_username_beneficiary_events (beneficiary_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_poc_username_beneficiary_events_username_time
    ON poc_username_beneficiary_events (username, time DESC);

CREATE UNIQUE INDEX IF NOT EXISTS idx_poc_username_beneficiary_events_event_id
    ON poc_username_beneficiary_events (event_id);

ALTER TABLE poc_configuration
    ADD COLUMN IF NOT EXISTS username_beneficiary_join_referral_bps BIGINT NOT NULL DEFAULT 500;

COMMENT ON COLUMN poc_configuration.username_beneficiary_join_referral_bps IS
    'One-time join-referral fee (bps of post-treasury gross) on first username-beneficiary vault claim';

ALTER TABLE poc_vault_claims
    ADD COLUMN IF NOT EXISTS claim_kind TEXT NULL;

COMMENT ON COLUMN poc_vault_claims.claim_kind IS
    'Vault claim classification: standard, join_referral, etc.';

COMMENT ON TABLE poc_username_beneficiaries IS 'Off-platform creator username beneficiary provisions (PoC)';
COMMENT ON TABLE poc_creator_identity_links IS 'Creator identity to wallet links after beneficiary claim';
COMMENT ON TABLE poc_username_beneficiary_events IS 'Append-only audit log for username beneficiary lifecycle events';

COMMENT ON COLUMN poc_username_beneficiaries.vault_routing_key IS
    'Identity-derived vault directory lookup key (not a user wallet or standalone object); see claimed_by for linked wallet';

INSERT INTO poc_beneficiary_vaults (vault_id, vault_routing_key, updated_at_ms, transaction_id, time)
SELECT ub.vault_id, ub.vault_routing_key, ub.provisioned_at_ms, ub.transaction_id, ub.time
FROM poc_username_beneficiaries ub
WHERE ub.vault_id IS NOT NULL AND ub.vault_id <> ''
ON CONFLICT (vault_id) DO UPDATE SET
    vault_routing_key = EXCLUDED.vault_routing_key,
    updated_at_ms = EXCLUDED.updated_at_ms,
    transaction_id = EXCLUDED.transaction_id,
    time = EXCLUDED.time;
