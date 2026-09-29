ALTER TABLE media_asset_governance_links DROP COLUMN IF EXISTS identity_hash;
ALTER TABLE media_asset_governance_links DROP COLUMN IF EXISTS identity_source;
ALTER TABLE media_asset_governance_links DROP COLUMN IF EXISTS target_username;
ALTER TABLE media_asset_governance_links DROP COLUMN IF EXISTS target_vault_id;
ALTER TABLE media_asset_governance_links DROP COLUMN IF EXISTS beneficiary_address;
ALTER TABLE media_asset_governance_links DROP COLUMN IF EXISTS target_kind;
