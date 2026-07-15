DROP INDEX IF EXISTS idx_poc_vault_claims_vault_time;
ALTER TABLE poc_vault_claims DROP COLUMN IF EXISTS gross_amount;
DROP TABLE IF EXISTS poc_vault_claims;

DROP INDEX IF EXISTS idx_poc_vault_deposits_vault_time;
DROP TABLE IF EXISTS poc_vault_deposits;

DROP INDEX IF EXISTS idx_poc_beneficiary_vaults_beneficiary;
DROP TABLE IF EXISTS poc_beneficiary_vaults;

ALTER TABLE poc_config DROP COLUMN IF EXISTS max_referral_bps;
ALTER TABLE poc_config DROP COLUMN IF EXISTS claim_treasury_fee_bps;

ALTER TABLE poc_badges DROP COLUMN IF EXISTS media_index;
ALTER TABLE poc_badges DROP COLUMN IF EXISTS matched_anchor_id;
ALTER TABLE poc_badges DROP COLUMN IF EXISTS beneficiary_address;

ALTER TABLE posts ADD COLUMN IF NOT EXISTS poc_escrow_balance BIGINT NULL;
