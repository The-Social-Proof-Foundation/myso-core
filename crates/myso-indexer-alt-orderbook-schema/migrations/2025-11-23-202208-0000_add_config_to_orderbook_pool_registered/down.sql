-- Remove config_json column from orderbook_pool_registered table
ALTER TABLE orderbook_pool_registered DROP COLUMN IF EXISTS config_json;

