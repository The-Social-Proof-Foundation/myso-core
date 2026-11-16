-- First join survives rejoins (rejoin overwrites joined_at). NULL on rows indexed before this.
ALTER TABLE platform_memberships ADD COLUMN IF NOT EXISTS first_joined_at TIMESTAMP;
