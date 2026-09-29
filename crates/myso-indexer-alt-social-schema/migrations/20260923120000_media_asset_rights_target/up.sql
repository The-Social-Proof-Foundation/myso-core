-- Committed beneficiary target for a media-asset rights dispute.

ALTER TABLE media_asset_governance_links
    ADD COLUMN IF NOT EXISTS target_kind SMALLINT NOT NULL DEFAULT 0;

ALTER TABLE media_asset_governance_links
    ADD COLUMN IF NOT EXISTS beneficiary_address TEXT NOT NULL DEFAULT '0x0';

ALTER TABLE media_asset_governance_links
    ADD COLUMN IF NOT EXISTS target_vault_id TEXT NULL;

ALTER TABLE media_asset_governance_links
    ADD COLUMN IF NOT EXISTS target_username TEXT NULL;

ALTER TABLE media_asset_governance_links
    ADD COLUMN IF NOT EXISTS identity_source SMALLINT NULL;

ALTER TABLE media_asset_governance_links
    ADD COLUMN IF NOT EXISTS identity_hash TEXT NULL;

COMMENT ON COLUMN media_asset_governance_links.target_kind IS 'RightsResolutionTarget kind: 0 wallet, 1 vault, 2 on-chain username, 3 off-chain identity';
COMMENT ON COLUMN media_asset_governance_links.beneficiary_address IS 'Address the dispute committed at submit. Kind 0 stores 0x0; implement rebuilds the wallet beneficiary from claims.';
COMMENT ON COLUMN media_asset_governance_links.target_vault_id IS 'PoCBeneficiaryVault object id when target_kind = 1';
COMMENT ON COLUMN media_asset_governance_links.target_username IS 'Canonical username for kind 2, or x_{handle} / existing row username for kind 3';
COMMENT ON COLUMN media_asset_governance_links.identity_source IS 'Off-chain identity source byte when target_kind = 3';
COMMENT ON COLUMN media_asset_governance_links.identity_hash IS 'Off-chain identity hash when target_kind = 3';
