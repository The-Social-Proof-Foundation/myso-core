-- Revert may fail if any nonce exceeds 64 characters.
ALTER TABLE transactions ALTER COLUMN nonce TYPE VARCHAR(64);
