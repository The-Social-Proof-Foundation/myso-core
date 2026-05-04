-- SPoT: rename epoch-based columns to millisecond semantics (on-chain + events now use Clock ms).

ALTER TABLE spot_records RENAME COLUMN created_epoch TO created_at_ms;
ALTER TABLE spot_records RENAME COLUMN last_resolution_epoch TO last_resolution_at_ms;
ALTER TABLE spot_records RENAME COLUMN resolution_window_epochs TO resolution_window_ms;
ALTER TABLE spot_records RENAME COLUMN max_resolution_window_epochs TO max_resolution_window_ms;

ALTER TABLE spot_config RENAME COLUMN resolution_window_epochs TO resolution_window_ms;
ALTER TABLE spot_config RENAME COLUMN max_resolution_window_epochs TO max_resolution_window_ms;

ALTER TABLE spot_bets RENAME COLUMN timestamp_epoch TO timestamp_ms;
ALTER TABLE spot_payouts RENAME COLUMN timestamp_epoch TO timestamp_ms;
ALTER TABLE spot_refunds RENAME COLUMN timestamp_epoch TO timestamp_ms;
ALTER TABLE spot_resolutions RENAME COLUMN resolved_epoch TO resolved_at_ms;
ALTER TABLE spot_bet_withdrawals RENAME COLUMN timestamp_epoch TO timestamp_ms;

COMMENT ON COLUMN spot_records.created_at_ms IS 'Record creation time from chain Clock (ms)';
COMMENT ON COLUMN spot_records.resolution_window_ms IS 'Optional min ms after creation before oracle may resolve';
COMMENT ON COLUMN spot_records.max_resolution_window_ms IS 'Optional max ms after creation for refund_unresolved';
COMMENT ON COLUMN spot_records.last_resolution_at_ms IS 'Wall-clock ms at last resolution/refund-unresolved';
