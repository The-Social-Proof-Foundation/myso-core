ALTER TABLE poc_beneficiary_vaults ADD COLUMN IF NOT EXISTS balance BIGINT NOT NULL DEFAULT 0;

UPDATE poc_beneficiary_vaults v
SET balance = COALESCE(
    (
        SELECT SUM(b.balance)
        FROM poc_vault_coin_balances b
        WHERE b.vault_id = v.vault_id
    ),
    0
);

DROP INDEX IF EXISTS idx_poc_vault_coin_balances_vault;

DROP TABLE IF EXISTS poc_vault_coin_balances;

ALTER TABLE poc_vault_claims DROP COLUMN IF EXISTS coin_type;

ALTER TABLE poc_vault_deposits DROP COLUMN IF EXISTS coin_type;

ALTER TABLE tips DROP COLUMN IF EXISTS coin_type;

ALTER TABLE poc_configuration DROP COLUMN IF EXISTS video_embedded_audio_redirect_bps;