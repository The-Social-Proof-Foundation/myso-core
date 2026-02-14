-- Add config_json column to orderbook_pool_registered table
ALTER TABLE orderbook_pool_registered ADD COLUMN config_json JSONB;

