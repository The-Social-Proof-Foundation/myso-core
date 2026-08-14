ALTER TABLE poc_config DROP COLUMN IF EXISTS max_disputes_per_media_asset;
ALTER TABLE poc_config DROP COLUMN IF EXISTS media_asset_dispute_cost;

DROP TABLE IF EXISTS media_asset_rights_updates;
DROP TABLE IF EXISTS media_asset_governance_links;
