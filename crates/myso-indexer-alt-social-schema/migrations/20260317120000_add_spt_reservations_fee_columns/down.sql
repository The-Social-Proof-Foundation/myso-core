-- Remove fee columns from spt_reservations
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS treasury_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS platform_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS creator_fee;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS fee_amount;
