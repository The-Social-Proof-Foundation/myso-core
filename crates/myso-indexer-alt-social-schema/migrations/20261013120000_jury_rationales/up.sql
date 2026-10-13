-- The written reasons behind revealed jury votes, kept where the case is read.
--
-- A vote's rationale is committed on chain only as a digest. The juror's runner publishes the text
-- after the reveal, and a row exists only if the text hashes to the digest the chain recorded for
-- that seat, so what is stored is exactly what the juror committed to and nothing else.
CREATE TABLE IF NOT EXISTS jury_rationales (
    case_id TEXT NOT NULL,
    round BIGINT NOT NULL,
    agent_id TEXT NOT NULL,
    rationale TEXT NOT NULL,
    rationale_digest TEXT NOT NULL,
    stored_at_ms BIGINT NOT NULL,
    PRIMARY KEY (case_id, round, agent_id)
);

CREATE INDEX IF NOT EXISTS jury_rationales_case_idx ON jury_rationales (case_id);
