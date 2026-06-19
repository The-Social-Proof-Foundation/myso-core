-- ROLLBACK TOKEN VESTING SYSTEM

DROP FUNCTION IF EXISTS calculate_vesting_claimable(BIGINT, BIGINT, BIGINT, JSONB, BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS calculate_vesting_claimable(BIGINT, BIGINT, BIGINT, BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_vesting_status(BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_vesting_progress(BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS vesting_apply_curve(DOUBLE PRECISION, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS vesting_piece_amount(BIGINT, BIGINT) CASCADE;

SELECT remove_compression_policy('vesting_events', if_exists => true);

DROP INDEX IF EXISTS idx_vesting_events_wallet_id;
DROP INDEX IF EXISTS idx_vesting_events_owner_address;
DROP INDEX IF EXISTS idx_vesting_events_event_type;
DROP INDEX IF EXISTS idx_vesting_events_event_time;

DROP TABLE IF EXISTS vesting_events CASCADE;

DROP INDEX IF EXISTS idx_vesting_wallets_owner_address;
DROP INDEX IF EXISTS idx_vesting_wallets_start_time;
DROP INDEX IF EXISTS idx_vesting_wallets_schedule_end;
DROP INDEX IF EXISTS idx_vesting_wallets_created_at;

DROP TABLE IF EXISTS vesting_wallets CASCADE;
