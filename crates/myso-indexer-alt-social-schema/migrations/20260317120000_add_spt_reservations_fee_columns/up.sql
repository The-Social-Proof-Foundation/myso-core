-- Add fee columns to spt_reservations for reservation fee tracking
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS fee_amount BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS creator_fee BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS platform_fee BIGINT;
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS treasury_fee BIGINT;

-- Indexer checkpoint ms for 24h analytics (aligned with spt_transactions.created_at).
ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS created_at BIGINT NOT NULL DEFAULT 0;

UPDATE spt_reservations
SET created_at = (EXTRACT(EPOCH FROM time) * 1000)::bigint
WHERE created_at = 0;

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

-- Fix indexer bug (ThresholdMetEvent used to INSERT spt_reservation_pools with synthetic pool_id).
-- Repoint reservations to the on-chain pool object id and drop erroneous synthetic pool rows.
WITH canonical AS (
    SELECT DISTINCT ON (associated_id)
        associated_id,
        pool_id AS canonical_pool_id
    FROM spt_reservation_pools
    WHERE pool_id NOT LIKE 'reservation_pool_%'
    ORDER BY associated_id, time ASC
)
UPDATE spt_reservations r
SET pool_id = c.canonical_pool_id
FROM canonical c
WHERE r.pool_id LIKE 'reservation_pool_%'
  AND r.pool_id = ('reservation_pool_' || c.associated_id);

DELETE FROM spt_reservation_pools s
WHERE s.pool_id LIKE 'reservation_pool_%'
  AND EXISTS (
      SELECT 1
      FROM spt_reservation_pools c
      WHERE c.associated_id = s.associated_id
        AND c.pool_id NOT LIKE 'reservation_pool_%'
  );

-- Backfill hypertable `time` when chain `reserved_at` was chain-local/sim ms (~1970) but `created_at`
-- is already plausible checkpoint Unix ms (1e12+; matches MIN_PLAUSIBLE_RESERVATION_UNIX_MS in indexer).
-- If this migration was applied before this block existed, run the same UPDATE separately or apply a
-- follow-up migration. TimescaleDB: decompress affected chunks if compression blocks the UPDATE.

UPDATE spt_reservations
SET time = to_timestamp(created_at / 1000.0)
WHERE created_at >= 1000000000000
  AND (
    time < timestamptz '2001-09-09 01:46:40+00'
    OR reserved_at < 1000000000000
  );
