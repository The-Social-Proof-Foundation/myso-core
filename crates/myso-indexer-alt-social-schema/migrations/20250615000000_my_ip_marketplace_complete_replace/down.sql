-- REVERSE MYDATA MARKETPLACE - COMPLETE SYSTEM ROLLBACK

SELECT remove_continuous_aggregate_policy('mydata_daily_revenue', if_exists => true);
SELECT remove_continuous_aggregate_policy('mydata_daily_access', if_exists => true);
SELECT remove_continuous_aggregate_policy('mydata_popular_data', if_exists => true);

DROP MATERIALIZED VIEW IF EXISTS mydata_daily_revenue CASCADE;
DROP MATERIALIZED VIEW IF EXISTS mydata_daily_access CASCADE;
DROP MATERIALIZED VIEW IF EXISTS mydata_popular_data CASCADE;

DROP VIEW IF EXISTS active_mydata CASCADE;
DROP VIEW IF EXISTS mydata_popular_30_days CASCADE;
DROP VIEW IF EXISTS mydata_creator_revenue_summary CASCADE;

DROP FUNCTION IF EXISTS user_has_mydata_access(TEXT, TEXT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_mydata_pricing(TEXT) CASCADE;
DROP FUNCTION IF EXISTS user_has_access(TEXT, TEXT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_data_pricing(TEXT) CASCADE;

SELECT remove_compression_policy('mydata_purchases', if_exists => true);
SELECT remove_compression_policy('mydata_subscriptions', if_exists => true);
SELECT remove_compression_policy('mydata_revenue', if_exists => true);
SELECT remove_compression_policy('mydata_access_logs', if_exists => true);

SELECT remove_retention_policy('mydata_access_logs', if_exists => true);

DROP TABLE IF EXISTS mydata_access_logs CASCADE;
DROP TABLE IF EXISTS mydata_revenue CASCADE;
DROP TABLE IF EXISTS mydata_subscriptions CASCADE;
DROP TABLE IF EXISTS mydata_purchases CASCADE;
DROP TABLE IF EXISTS mydata_data CASCADE;
