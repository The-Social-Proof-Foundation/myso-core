-- Latest mark per SPT pool (current state, not a hypertable).
-- Leaderboard / portfolio reads JOIN this instead of scanning spt_price_history.

CREATE TABLE IF NOT EXISTS spt_pool_mark (
    pool_id TEXT PRIMARY KEY,
    price BIGINT NOT NULL,
    circulating_supply BIGINT NOT NULL DEFAULT 0,
    time TIMESTAMPTZ NOT NULL,
    transaction_id TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_spt_pool_mark_time
    ON spt_pool_mark (time DESC);

COMMENT ON TABLE spt_pool_mark IS
    'Latest SPT pool price and circulating supply. Upserted on each price tick.';

INSERT INTO spt_pool_mark (pool_id, price, circulating_supply, time, transaction_id)
SELECT DISTINCT ON (pool_id)
    pool_id,
    price,
    circulating_supply,
    time,
    transaction_id
FROM spt_price_history
ORDER BY pool_id, time DESC
ON CONFLICT (pool_id) DO UPDATE SET
    price = EXCLUDED.price,
    circulating_supply = EXCLUDED.circulating_supply,
    time = EXCLUDED.time,
    transaction_id = EXCLUDED.transaction_id
WHERE EXCLUDED.time >= spt_pool_mark.time;
