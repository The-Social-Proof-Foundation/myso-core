-- Document unified chain clock timestamp semantics (myso::clock::Clock at 0x6).
-- Indexer stores Unix milliseconds from on-chain events; hypertable `time` columns derive via ms / 1000.

COMMENT ON COLUMN posts.created_at IS 'Unix ms from myso::clock::Clock (0x6) at post creation';
COMMENT ON COLUMN posts.time IS 'Hypertable partition time derived from posts.created_at ms';

COMMENT ON COLUMN comments.created_at IS 'Unix ms from myso::clock::Clock (0x6) at comment creation';
COMMENT ON COLUMN comments.time IS 'Hypertable partition time derived from comments.created_at ms';

COMMENT ON COLUMN reactions.created_at IS 'Unix ms from myso::clock::Clock (0x6) at reaction time';
COMMENT ON COLUMN reactions.time IS 'Hypertable partition time derived from reactions.created_at ms';

COMMENT ON COLUMN profiles.created_at IS 'Profile creation time converted from chain clock ms at index time';
COMMENT ON COLUMN profiles.updated_at IS 'Profile update time converted from chain clock ms at index time';

COMMENT ON COLUMN spot_bets.timestamp_ms IS 'Unix ms from myso::clock::Clock (0x6) when bet was placed';
COMMENT ON COLUMN spot_records.created_at_ms IS 'Record creation time from chain Clock (ms)';

COMMENT ON COLUMN spt_reservations.reserved_at IS 'Unix ms from myso::clock::Clock (0x6) when reservation was created';
COMMENT ON COLUMN spt_reservations.time IS 'Hypertable partition time derived from reserved_at ms';
