-- Tips carry Move coin type (qualified type string). Vault deposits/claims keyed per asset; aggregate balance moves off poc_beneficiary_vaults.

ALTER TABLE tips ADD COLUMN IF NOT EXISTS coin_type TEXT NOT NULL DEFAULT '';

ALTER TABLE poc_vault_deposits ADD COLUMN IF NOT EXISTS coin_type TEXT NOT NULL DEFAULT '';

ALTER TABLE poc_vault_claims ADD COLUMN IF NOT EXISTS coin_type TEXT NOT NULL DEFAULT '';

CREATE TABLE IF NOT EXISTS poc_vault_coin_balances (
    vault_id TEXT NOT NULL,
    coin_type TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    updated_at_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (vault_id, coin_type)
);

CREATE INDEX IF NOT EXISTS idx_poc_vault_coin_balances_vault ON poc_vault_coin_balances (vault_id);

INSERT INTO poc_vault_coin_balances (vault_id, coin_type, balance, updated_at_ms, transaction_id, time)
SELECT vault_id, '__legacy_aggregate__', balance, updated_at_ms, transaction_id, time
FROM poc_beneficiary_vaults
ON CONFLICT (vault_id, coin_type) DO NOTHING;

ALTER TABLE poc_beneficiary_vaults DROP COLUMN IF EXISTS balance;

-- Max redirect ceiling (bps 0-10000) for VIDEO when only embedded audio matches upstream work.
-- Default 3000 matches on-chain DEFAULT_VIDEO_EMBEDDED_AUDIO_REDIRECT_BPS (~30% after bps→percent rounding).
ALTER TABLE poc_configuration
    ADD COLUMN IF NOT EXISTS video_embedded_audio_redirect_bps BIGINT NOT NULL DEFAULT 3000;

COMMENT ON COLUMN poc_configuration.video_embedded_audio_redirect_bps IS
    'Max redirect ceiling as bps for VIDEO posts when oracle sets embedded-audio-only derivative; multiplied by similarity delta ramp on-chain';
