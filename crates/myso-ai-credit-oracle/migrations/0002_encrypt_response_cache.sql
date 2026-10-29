-- Prior to schema version 2, completed assistant responses were stored in plaintext.
-- They are only an idempotency cache; discard them rather than leave sensitive text at rest.
UPDATE ai_inference_reservations
SET content = NULL
WHERE content IS NOT NULL AND content NOT LIKE 'v1:%';
