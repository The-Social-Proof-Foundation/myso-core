CREATE TABLE wallet_messaging_policies (
  wallet_address TEXT PRIMARY KEY,
  enabled BOOLEAN NOT NULL DEFAULT FALSE,
  min_cost BIGINT,
  updated_at BIGINT NOT NULL
);
CREATE INDEX idx_wallet_messaging_policies_enabled ON wallet_messaging_policies(enabled);
