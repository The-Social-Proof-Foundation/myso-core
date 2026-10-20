-- A jury that cannot be seated defers the proposal instead of forfeiting it. The review row keeps
-- when that started (the 30-day bound runs from the first deferral), the last time it happened (the
-- escalation crank backs off from it), and the seat counts the chain reported.
ALTER TABLE proposal_jury_reviews ADD COLUMN IF NOT EXISTS jury_unavailable_at_ms BIGINT;
ALTER TABLE proposal_jury_reviews ADD COLUMN IF NOT EXISTS jury_deferred_since_ms BIGINT;
ALTER TABLE proposal_jury_reviews ADD COLUMN IF NOT EXISTS jury_formable_seats BIGINT;
ALTER TABLE proposal_jury_reviews ADD COLUMN IF NOT EXISTS jury_required_seats BIGINT;

-- A community step that waited 30 days without a jury refunds everyone. A separate table, like the
-- other jury tables, so the proposal rows and their raw queries stay as they are.
-- `outstanding_mist` is the voter pool less what has been claimed; the chain's own guard on the
-- sweep counts unclaimed voters, which `proposal_vote_refunds` lets a query check.
CREATE TABLE IF NOT EXISTS proposal_refunds (
    proposal_id TEXT PRIMARY KEY,
    submitter TEXT NOT NULL,
    submitter_refund_mist BIGINT NOT NULL,
    voter_refund_pool_mist BIGINT NOT NULL,
    outstanding_mist BIGINT NOT NULL,
    claim_deadline_ms BIGINT NOT NULL,
    refunded_at_ms BIGINT NOT NULL,
    swept_at_ms BIGINT,
    swept_mist BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS proposal_refunds_unswept_idx
    ON proposal_refunds (claim_deadline_ms) WHERE swept_at_ms IS NULL;

-- One row per voter who claimed. What each voter paid is in `community_votes.vote_cost`.
CREATE TABLE IF NOT EXISTS proposal_vote_refunds (
    proposal_id TEXT NOT NULL,
    voter TEXT NOT NULL,
    amount_mist BIGINT NOT NULL,
    claimed_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    PRIMARY KEY (proposal_id, voter)
);
