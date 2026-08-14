ALTER TABLE poc_config
    ADD COLUMN IF NOT EXISTS max_embedded_asset_redirect_bps BIGINT NOT NULL DEFAULT 5000;

COMMENT ON COLUMN poc_config.max_embedded_asset_redirect_bps IS
    'Max bps any embedded source asset may redirect from post pool (mirrors Move PoCConfig)';
