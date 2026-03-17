-- Remove fee columns from spt_reservations
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS treasury_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS platform_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS creator_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS fee_amount;

-- Remove mydata columns from posts
ALTER TABLE posts DROP COLUMN IF EXISTS mydata_id;
ALTER TABLE posts DROP COLUMN IF EXISTS revenue_recipient;
