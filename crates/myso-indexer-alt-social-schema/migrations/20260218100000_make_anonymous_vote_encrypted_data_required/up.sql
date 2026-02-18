-- Align anonymous_votes.encrypted_vote_data with contract: AnonymousVoteEvent
-- always emits encrypted_vote_data (vector<u8>), so column must be NOT NULL.

UPDATE anonymous_votes SET encrypted_vote_data = ''::bytea WHERE encrypted_vote_data IS NULL;

ALTER TABLE anonymous_votes ALTER COLUMN encrypted_vote_data SET NOT NULL;
