DROP TABLE IF EXISTS post_access_purchases;
DROP TABLE IF EXISTS org_treasury_events;
DROP TABLE IF EXISTS org_treasury_access;
DROP TABLE IF EXISTS org_treasury_balances;
ALTER TABLE jury_roster DROP COLUMN IF EXISTS refreshed_at_ms;
ALTER TABLE jury_cases DROP COLUMN IF EXISTS remainder_swept_mist;
ALTER TABLE jury_cases DROP COLUMN IF EXISTS remainder_recipient;
