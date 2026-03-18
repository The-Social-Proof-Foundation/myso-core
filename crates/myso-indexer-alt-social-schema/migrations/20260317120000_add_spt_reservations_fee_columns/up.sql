-- Add fee columns to spt_reservations for reservation fee tracking
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS fee_amount BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS creator_fee BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS platform_fee BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS treasury_fee BIGINT;

-- Ensure platform_fee is nullable (reservations without platform have no platform fee)
DO $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_schema = 'public' AND table_name = 'spt_reservations'
      AND column_name = 'platform_fee' AND is_nullable = 'NO'
  ) THEN
    ALTER TABLE spt_reservations ALTER COLUMN platform_fee DROP NOT NULL;
  END IF;
END $$;

-- Add mydata_id and revenue_recipient to posts for MyData marketplace integration
ALTER TABLE posts ADD COLUMN IF NOT EXISTS mydata_id TEXT;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS revenue_recipient TEXT;
