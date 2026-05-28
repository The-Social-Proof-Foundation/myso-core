DROP TABLE IF EXISTS mydata_claims CASCADE;
DROP TABLE IF EXISTS mydata_distribution_rounds CASCADE;
DROP TABLE IF EXISTS mydata_snapshot_anchors CASCADE;
DROP TABLE IF EXISTS mydata_merkle_roots CASCADE;
DROP TABLE IF EXISTS mydata_listing_sub_pools CASCADE;
DROP TABLE IF EXISTS mydata_sub_pools CASCADE;
DROP TABLE IF EXISTS mydata_broad_pools CASCADE;

DROP FUNCTION IF EXISTS update_mydata_claims_time();
DROP FUNCTION IF EXISTS update_mydata_distribution_rounds_time();
DROP FUNCTION IF EXISTS update_mydata_snapshot_anchors_time();
DROP FUNCTION IF EXISTS update_mydata_merkle_roots_time();
DROP FUNCTION IF EXISTS update_mydata_listing_sub_pools_time();
DROP FUNCTION IF EXISTS update_mydata_sub_pools_time();
DROP FUNCTION IF EXISTS update_mydata_broad_pools_time();
