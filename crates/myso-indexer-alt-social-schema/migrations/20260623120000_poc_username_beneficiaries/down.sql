ALTER TABLE poc_vault_claims DROP COLUMN IF EXISTS claim_kind;
ALTER TABLE poc_config DROP COLUMN IF EXISTS username_beneficiary_join_referral_bps;

DROP TABLE IF EXISTS poc_username_beneficiary_events CASCADE;
DROP TABLE IF EXISTS poc_creator_identity_links CASCADE;
DROP TABLE IF EXISTS poc_username_beneficiaries CASCADE;
