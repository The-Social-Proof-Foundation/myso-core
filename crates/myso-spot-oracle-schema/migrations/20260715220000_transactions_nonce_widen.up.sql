-- finalize-post-{0x...66-char object id} exceeds the original VARCHAR(64).
ALTER TABLE transactions ALTER COLUMN nonce TYPE VARCHAR(160);
