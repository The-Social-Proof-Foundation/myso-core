ALTER TABLE token_transfer ADD COLUMN IF NOT EXISTS is_finalized BOOLEAN DEFAULT false;
ALTER TABLE token_transfer_data ADD COLUMN IF NOT EXISTS is_finalized BOOLEAN DEFAULT false;