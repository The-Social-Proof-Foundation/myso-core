-- Copyright (c) The Social Proof Foundation, LLC.
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE promotion_views
  DROP COLUMN IF EXISTS recipient_amount,
  DROP COLUMN IF EXISTS ecosystem_fee,
  DROP COLUMN IF EXISTS platform_fee;

ALTER TABLE post_config
  DROP COLUMN IF EXISTS ecosystem_fee_bps,
  DROP COLUMN IF EXISTS platform_fee_bps;
