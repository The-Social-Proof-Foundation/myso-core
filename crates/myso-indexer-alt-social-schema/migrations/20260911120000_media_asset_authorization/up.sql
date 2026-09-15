-- MediaAsset enforcement epoch + pause, and visible license-term fields.

ALTER TABLE media_assets
    ADD COLUMN IF NOT EXISTS authorization_version BIGINT NOT NULL DEFAULT 1;
ALTER TABLE media_assets
    ADD COLUMN IF NOT EXISTS future_usage_paused BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE license_template_versions
    ADD COLUMN IF NOT EXISTS instance_revocable BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE license_template_versions
    ADD COLUMN IF NOT EXISTS legal_terms_uri TEXT NULL;
ALTER TABLE license_template_versions
    ADD COLUMN IF NOT EXISTS legal_terms_hash BYTEA NULL;
ALTER TABLE license_template_versions
    ADD COLUMN IF NOT EXISTS governing_law BYTEA NULL;
ALTER TABLE license_template_versions
    ADD COLUMN IF NOT EXISTS license_schema_version BIGINT NOT NULL DEFAULT 1;
