-- Reputation aggregates, one row per (subject, role); the Bayesian score is also computed on
-- read from the sums, which are TEXT decimal strings because they are u128 on-chain (no bigdecimal dependency).
CREATE TABLE IF NOT EXISTS reputation_aggregates (
    subject_id TEXT NOT NULL,
    -- 1 seller, 2 buyer
    role SMALLINT NOT NULL,
    -- 1 agent as seller, 2 agent as buyer, 3 principal buying directly
    subject_kind SMALLINT NOT NULL,
    principal_owner TEXT NOT NULL,
    organization_id TEXT,
    review_count BIGINT NOT NULL,
    star_1 BIGINT NOT NULL,
    star_2 BIGINT NOT NULL,
    star_3 BIGINT NOT NULL,
    star_4 BIGINT NOT NULL,
    star_5 BIGINT NOT NULL,
    weight_sum_bps TEXT NOT NULL,
    weighted_star_sum TEXT NOT NULL,
    bayesian_score_milli BIGINT NOT NULL,
    confidence_bps BIGINT NOT NULL,
    last_receipt_id TEXT NOT NULL,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (subject_id, role)
);

CREATE INDEX IF NOT EXISTS idx_reputation_aggregates_score
    ON reputation_aggregates (role, bayesian_score_milli DESC);
CREATE INDEX IF NOT EXISTS idx_reputation_aggregates_principal
    ON reputation_aggregates (principal_owner);

-- Principal credibility inputs (completed, paid transactions in either role).
CREATE TABLE IF NOT EXISTS credibility (
    account_id TEXT PRIMARY KEY,
    principal_owner TEXT NOT NULL,
    settled_count BIGINT NOT NULL,
    volume_mist TEXT NOT NULL,
    credibility_bps BIGINT NOT NULL,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_credibility_principal ON credibility (principal_owner);
