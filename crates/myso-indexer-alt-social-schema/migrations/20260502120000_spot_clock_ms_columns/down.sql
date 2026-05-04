ALTER TABLE spot_records RENAME COLUMN created_at_ms TO created_epoch;
ALTER TABLE spot_records RENAME COLUMN last_resolution_at_ms TO last_resolution_epoch;
ALTER TABLE spot_records RENAME COLUMN resolution_window_ms TO resolution_window_epochs;
ALTER TABLE spot_records RENAME COLUMN max_resolution_window_ms TO max_resolution_window_epochs;

ALTER TABLE spot_config RENAME COLUMN resolution_window_ms TO resolution_window_epochs;
ALTER TABLE spot_config RENAME COLUMN max_resolution_window_ms TO max_resolution_window_epochs;

ALTER TABLE spot_bets RENAME COLUMN timestamp_ms TO timestamp_epoch;
ALTER TABLE spot_payouts RENAME COLUMN timestamp_ms TO timestamp_epoch;
ALTER TABLE spot_refunds RENAME COLUMN timestamp_ms TO timestamp_epoch;
ALTER TABLE spot_resolutions RENAME COLUMN resolved_at_ms TO resolved_epoch;
ALTER TABLE spot_bet_withdrawals RENAME COLUMN timestamp_ms TO timestamp_epoch;
