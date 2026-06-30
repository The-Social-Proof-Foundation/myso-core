-- PoC beneficiary vault tables + badge/config extensions; drop on-post escrow cache.

ALTER TABLE posts DROP COLUMN IF EXISTS poc_escrow_balance;

ALTER TABLE poc_badges ADD COLUMN IF NOT EXISTS beneficiary_address TEXT NULL;
ALTER TABLE poc_badges ADD COLUMN IF NOT EXISTS matched_anchor_id TEXT NULL;
ALTER TABLE poc_badges ADD COLUMN IF NOT EXISTS media_index SMALLINT NULL;

ALTER TABLE poc_configuration ADD COLUMN IF NOT EXISTS claim_treasury_fee_bps BIGINT NOT NULL DEFAULT 100;
ALTER TABLE poc_configuration ADD COLUMN IF NOT EXISTS max_referral_bps BIGINT NOT NULL DEFAULT 500;

COMMENT ON COLUMN poc_badges.beneficiary_address IS 'Wallet receiving redirected PoC MYSO (from PoCBadgeIssuedEvent)';
COMMENT ON COLUMN poc_badges.matched_anchor_id IS 'Optional anchor object address when similarity binds to media';
COMMENT ON COLUMN poc_badges.media_index IS 'Oracle-assessed media slot (255 = unspecified)';
COMMENT ON COLUMN poc_configuration.claim_treasury_fee_bps IS 'Treasury slice at vault claim (bps)';
COMMENT ON COLUMN poc_configuration.max_referral_bps IS 'Max referral slice of amount after treasury (bps); optional referrer on claim';

CREATE TABLE IF NOT EXISTS poc_beneficiary_vaults (
    vault_id TEXT PRIMARY KEY,
    vault_routing_key TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    updated_at_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_poc_beneficiary_vaults_vault_routing_key ON poc_beneficiary_vaults (vault_routing_key);

COMMENT ON COLUMN poc_beneficiary_vaults.vault_routing_key IS
    'Vault directory lookup key stored on PoCBeneficiaryVault.beneficiary (may be identity-derived, not a wallet)';

CREATE TABLE IF NOT EXISTS poc_vault_deposits (
    id BIGSERIAL PRIMARY KEY,
    vault_id TEXT NOT NULL,
    vault_routing_key TEXT NOT NULL,
    amount BIGINT NOT NULL,
    source_post_id TEXT NULL,
    occurred_at_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_poc_vault_deposits_vault_time ON poc_vault_deposits (vault_id, time DESC);

CREATE TABLE IF NOT EXISTS poc_vault_claims (
    id BIGSERIAL PRIMARY KEY,
    vault_id TEXT NOT NULL,
    vault_routing_key TEXT NOT NULL,
    referrer_address TEXT NULL,
    treasury_amount BIGINT NOT NULL,
    referrer_amount BIGINT NOT NULL,
    beneficiary_amount BIGINT NOT NULL,
    occurred_at_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_poc_vault_claims_vault_time ON poc_vault_claims (vault_id, time DESC);

ALTER TABLE poc_vault_claims ADD COLUMN IF NOT EXISTS gross_amount BIGINT NOT NULL DEFAULT 0;
COMMENT ON COLUMN poc_vault_claims.gross_amount IS
    'Derived at index time: treasury_amount + referrer_amount + beneficiary_amount (full claim in base units).';
