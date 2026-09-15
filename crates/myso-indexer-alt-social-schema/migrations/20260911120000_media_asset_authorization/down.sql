ALTER TABLE media_assets DROP COLUMN IF EXISTS authorization_version;
ALTER TABLE media_assets DROP COLUMN IF EXISTS future_usage_paused;

ALTER TABLE license_template_versions DROP COLUMN IF EXISTS instance_revocable;
ALTER TABLE license_template_versions DROP COLUMN IF EXISTS legal_terms_uri;
ALTER TABLE license_template_versions DROP COLUMN IF EXISTS legal_terms_hash;
ALTER TABLE license_template_versions DROP COLUMN IF EXISTS governing_law;
ALTER TABLE license_template_versions DROP COLUMN IF EXISTS license_schema_version;
