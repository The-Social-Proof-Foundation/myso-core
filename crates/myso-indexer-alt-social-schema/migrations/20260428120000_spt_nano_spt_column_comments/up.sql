-- Document SPT quantity columns: values are on-chain nano-SPT (10^9 per 1.0 display token),
-- stored as BIGINT fixed-point smallest units (same convention as social_proof_tokens.move SPT_SCALE).

COMMENT ON COLUMN spt_pools.circulating_supply IS
    'nano-SPT: 10^9 units per 1.0 display token (fixed-point integer; matches chain TokenInfo.circulating_supply).';

COMMENT ON COLUMN spt_holdings.amount IS
    'nano-SPT: 10^9 units per 1.0 display token. SUM(amount) per (pool_id, holder) is balance in smallest units.';

COMMENT ON COLUMN spt_price_history.circulating_supply IS
    'nano-SPT snapshot: 10^9 units per 1.0 display token (matches pool circulating_supply units at that row).';

COMMENT ON COLUMN spt_transactions.amount IS
    'nano-SPT traded in this row: 10^9 units per 1.0 display token (buy positive, sell negative in handler convention).';
