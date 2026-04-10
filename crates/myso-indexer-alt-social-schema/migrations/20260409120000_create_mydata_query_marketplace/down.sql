DROP TABLE IF EXISTS mydata_query_claims CASCADE;
DROP TABLE IF EXISTS mydata_query_distribution_rounds CASCADE;
DROP TABLE IF EXISTS mydata_query_snapshot_anchors CASCADE;
DROP TABLE IF EXISTS mydata_query_merkle_roots CASCADE;
DROP TABLE IF EXISTS mydata_query_listing_sub_pools CASCADE;
DROP TABLE IF EXISTS mydata_query_sub_pools CASCADE;
DROP TABLE IF EXISTS mydata_query_broad_pools CASCADE;

DROP FUNCTION IF EXISTS update_mydata_query_claims_time();
DROP FUNCTION IF EXISTS update_mydata_query_distribution_rounds_time();
DROP FUNCTION IF EXISTS update_mydata_query_snapshot_anchors_time();
DROP FUNCTION IF EXISTS update_mydata_query_merkle_roots_time();
DROP FUNCTION IF EXISTS update_mydata_query_listing_sub_pools_time();
DROP FUNCTION IF EXISTS update_mydata_query_sub_pools_time();
DROP FUNCTION IF EXISTS update_mydata_query_broad_pools_time();
