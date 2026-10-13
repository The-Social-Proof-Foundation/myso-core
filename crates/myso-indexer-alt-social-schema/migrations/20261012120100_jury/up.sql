-- AI jury (jury module events). Updated in event order by the sequential JuryHandler, so plain
-- UPDATEs are safe. A case's `state` follows jury.move: 1 evidence, 2 awaiting draw,
-- 3 selecting, 4 voting, 5 tallied, 6 final, 7 failed, 8 cancelled.
CREATE TABLE IF NOT EXISTS jury_cases (
    case_id TEXT PRIMARY KEY,
    subject_kind SMALLINT NOT NULL,
    subject_id TEXT NOT NULL,
    stage SMALLINT NOT NULL,
    menu_len SMALLINT NOT NULL,
    approval_bps BIGINT NOT NULL,
    quorum BIGINT NOT NULL,
    round_sizes TEXT NOT NULL,
    appealable BOOLEAN NOT NULL,
    failure_path SMALLINT NOT NULL,
    opened_by TEXT NOT NULL,
    fee_mist BIGINT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    evidence_deadline_ms BIGINT NOT NULL,
    state SMALLINT NOT NULL DEFAULT 1,
    current_round BIGINT NOT NULL DEFAULT 0,
    final_outcome SMALLINT,
    appeals_used BIGINT NOT NULL DEFAULT 0,
    failed_appeal BOOLEAN NOT NULL DEFAULT FALSE,
    participants BIGINT,
    per_juror_fee_mist BIGINT,
    fail_reason SMALLINT,
    finalized_at_ms BIGINT,
    cancelled_at_ms BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS jury_cases_subject_idx ON jury_cases (subject_kind, subject_id);
CREATE INDEX IF NOT EXISTS jury_cases_open_idx ON jury_cases (state) WHERE state < 6;

CREATE TABLE IF NOT EXISTS jury_rounds (
    case_id TEXT NOT NULL,
    round BIGINT NOT NULL,
    accept_deadline_ms BIGINT NOT NULL,
    commit_deadline_ms BIGINT,
    reveal_deadline_ms BIGINT,
    tally TEXT,
    abstain BIGINT,
    revealed BIGINT,
    decisive BOOLEAN,
    leading SMALLINT,
    -- When the appeal window closes and anyone may finalize; NULL if none opened.
    appeal_deadline_ms BIGINT,
    PRIMARY KEY (case_id, round)
);

CREATE TABLE IF NOT EXISTS jury_slots (
    case_id TEXT NOT NULL,
    round BIGINT NOT NULL,
    agent_id TEXT NOT NULL,
    -- 1 drafted, 2 accepted, 3 committed, 4 revealed, 5 replaced, 6 defaulted
    status SMALLINT NOT NULL DEFAULT 1,
    accept_deadline_ms BIGINT NOT NULL,
    accepted_at_ms BIGINT,
    commitment TEXT,
    outcome SMALLINT,
    rationale_digest TEXT,
    evidence_refs TEXT,
    bundle_digest TEXT,
    revealed_at_ms BIGINT,
    PRIMARY KEY (case_id, round, agent_id)
);

CREATE INDEX IF NOT EXISTS jury_slots_agent_idx ON jury_slots (agent_id, status);

CREATE TABLE IF NOT EXISTS jury_evidence (
    case_id TEXT NOT NULL,
    idx BIGINT NOT NULL,
    submitter TEXT NOT NULL,
    round BIGINT NOT NULL,
    kind SMALLINT NOT NULL,
    digest TEXT NOT NULL,
    uri TEXT NOT NULL,
    submitted_at_ms BIGINT NOT NULL,
    PRIMARY KEY (case_id, idx)
);

CREATE TABLE IF NOT EXISTS jury_payouts (
    case_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    recipient TEXT NOT NULL,
    amount_mist BIGINT NOT NULL,
    PRIMARY KEY (case_id, agent_id)
);

-- `status`: 0 left, 1 active, 3 suspended. A suspension has ended once `suspended_until_ms` is in
-- the past. Pausing availability emits no event, so it is not reflected here.
CREATE TABLE IF NOT EXISTS jury_roster (
    agent_id TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    memory_account_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    payout_address TEXT NOT NULL,
    model_family SMALLINT NOT NULL,
    exempt BOOLEAN NOT NULL,
    status SMALLINT NOT NULL DEFAULT 1,
    suspended_until_ms BIGINT,
    duty_score BIGINT,
    strikes BIGINT NOT NULL DEFAULT 0,
    joined_at_ms BIGINT NOT NULL,
    left_at_ms BIGINT,
    left_reason SMALLINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS jury_roster_owner_idx ON jury_roster (owner);
