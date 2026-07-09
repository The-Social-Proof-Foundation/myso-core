-- Daily rollup of claimed SPoT creator payouts (UTC day buckets).

CREATE TABLE IF NOT EXISTS spot_creator_earnings_daily (
    creator_address TEXT NOT NULL,
    day DATE NOT NULL,
    amount BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (creator_address, day)
);

CREATE INDEX IF NOT EXISTS idx_spot_creator_earnings_daily_day
    ON spot_creator_earnings_daily (day DESC);

COMMENT ON TABLE spot_creator_earnings_daily IS
    'UTC daily rollup of claimed SPoT creator referral payouts';
