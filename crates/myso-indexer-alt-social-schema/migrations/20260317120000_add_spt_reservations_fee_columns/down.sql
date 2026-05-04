-- Removing columns below does not restore `time` values possibly rewritten by the backfill in up.sql.

-- Remove fee and indexer-timestamp columns from spt_reservations
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS created_at;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS treasury_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS platform_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS creator_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS fee_amount;

-- Remove mydata / PostCreated-related columns from posts
ALTER TABLE posts DROP COLUMN IF EXISTS permissions;
ALTER TABLE posts DROP COLUMN IF EXISTS platform_id;
ALTER TABLE posts DROP COLUMN IF EXISTS mydata_id;
ALTER TABLE posts DROP COLUMN IF EXISTS revenue_recipient;
