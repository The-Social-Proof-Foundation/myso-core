DROP INDEX IF EXISTS idx_license_instances_licensee;
DROP INDEX IF EXISTS idx_license_instances_licensor;
DROP TABLE IF EXISTS license_instances;
DROP TABLE IF EXISTS license_template_versions;
DROP TABLE IF EXISTS media_asset_ancestry_snapshots;
DROP INDEX IF EXISTS idx_deriv_edges_child;
DROP INDEX IF EXISTS idx_deriv_edges_parent;
DROP TABLE IF EXISTS media_asset_derivative_edges;
ALTER TABLE media_assets DROP COLUMN IF EXISTS asset_kind;
