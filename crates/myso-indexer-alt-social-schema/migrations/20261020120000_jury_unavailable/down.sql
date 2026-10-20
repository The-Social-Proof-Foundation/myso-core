DROP TABLE IF EXISTS proposal_vote_refunds;
DROP TABLE IF EXISTS proposal_refunds;
ALTER TABLE proposal_jury_reviews DROP COLUMN IF EXISTS jury_required_seats;
ALTER TABLE proposal_jury_reviews DROP COLUMN IF EXISTS jury_formable_seats;
ALTER TABLE proposal_jury_reviews DROP COLUMN IF EXISTS jury_deferred_since_ms;
ALTER TABLE proposal_jury_reviews DROP COLUMN IF EXISTS jury_unavailable_at_ms;
