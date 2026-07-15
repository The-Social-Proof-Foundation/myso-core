-- Copyright (c) The Social Proof Foundation, LLC.
-- SPDX-License-Identifier: Apache-2.0

-- Promotion view fees: platform + ecosystem bps on PostConfig; per-view fee/net on promotion_views.

ALTER TABLE post_config
  ADD COLUMN IF NOT EXISTS platform_fee_bps BIGINT NOT NULL DEFAULT 1000,
  ADD COLUMN IF NOT EXISTS ecosystem_fee_bps BIGINT NOT NULL DEFAULT 1000;

COMMENT ON COLUMN post_config.platform_fee_bps IS 'bps of each promotion view gross to platform treasury';
COMMENT ON COLUMN post_config.ecosystem_fee_bps IS 'bps of each promotion view gross to ecosystem treasury';

ALTER TABLE promotion_views
  ADD COLUMN IF NOT EXISTS platform_fee BIGINT NOT NULL DEFAULT 0,
  ADD COLUMN IF NOT EXISTS ecosystem_fee BIGINT NOT NULL DEFAULT 0,
  ADD COLUMN IF NOT EXISTS recipient_amount BIGINT NOT NULL DEFAULT 0;

COMMENT ON COLUMN promotion_views.platform_fee IS 'platform fee taken from payment_amount gross';
COMMENT ON COLUMN promotion_views.ecosystem_fee IS 'ecosystem fee taken from payment_amount gross';
COMMENT ON COLUMN promotion_views.recipient_amount IS 'net MYSO transferred to viewer';

-- Historical rows predate fee split: treat full gross as viewer receipt.
UPDATE promotion_views
SET recipient_amount = payment_amount
WHERE recipient_amount = 0 AND payment_amount > 0;
