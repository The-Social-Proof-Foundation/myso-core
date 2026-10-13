-- AI jury behind the governance steps. Separate tables, not columns on `proposals`, so the
-- proposal rows and their raw queries stay as they are.
CREATE TABLE IF NOT EXISTS registry_jury_policies (
    registry_id TEXT PRIMARY KEY,
    council_stage_enabled BOOLEAN NOT NULL,
    community_stage_enabled BOOLEAN NOT NULL,
    council_window_ms BIGINT NOT NULL,
    jury_size BIGINT NOT NULL,
    round2_size BIGINT NOT NULL,
    jury_fee_bps BIGINT NOT NULL,
    jury_fee_cap_mist BIGINT NOT NULL,
    min_pool_for_jury_mist BIGINT NOT NULL,
    quorum BIGINT NOT NULL,
    approval_bps_council BIGINT NOT NULL,
    approval_bps_community BIGINT NOT NULL,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL
);

-- One row per proposal per human step the jury can take over. `stage`: 1 council, 2 community.
-- `state`: 1 referring, 2 awaiting escalation, 3 with the jury, 4 decided. `trigger`: 1 referral
-- majority, 2 council tie, 3 council deadline, 4 quorum not met, 5 tied vote.
-- `decision`: 0 forward (advance or approve), 1 reject; `failed` means the jury could not decide
-- and the step's default applied.
CREATE TABLE IF NOT EXISTS proposal_jury_reviews (
    proposal_id TEXT NOT NULL,
    stage SMALLINT NOT NULL,
    state SMALLINT NOT NULL,
    trigger SMALLINT NOT NULL DEFAULT 0,
    case_id TEXT,
    referral_count BIGINT NOT NULL DEFAULT 0,
    escalated_at_ms BIGINT,
    decided_at_ms BIGINT,
    decision SMALLINT,
    failed BOOLEAN NOT NULL DEFAULT FALSE,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (proposal_id, stage)
);

CREATE INDEX IF NOT EXISTS proposal_jury_reviews_case_idx ON proposal_jury_reviews (case_id);

CREATE TABLE IF NOT EXISTS proposal_jury_referrals (
    proposal_id TEXT NOT NULL,
    delegate_address TEXT NOT NULL,
    reason TEXT,
    referred_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    PRIMARY KEY (proposal_id, delegate_address)
);
