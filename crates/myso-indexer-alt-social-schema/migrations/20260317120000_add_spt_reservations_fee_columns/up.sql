-- Add fee columns to spt_reservations for reservation fee tracking
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS fee_amount BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS creator_fee BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS platform_fee BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS treasury_fee BIGINT;
