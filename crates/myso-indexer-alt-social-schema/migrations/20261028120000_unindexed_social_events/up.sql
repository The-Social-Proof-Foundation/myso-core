-- Projections for social events that were emitted but dropped by the indexer.

-- JuryCaseRemainderSweptEvent: what the jurors left unclaimed and where it went. The event has no
-- timestamp; `remainder_swept_mist` being set marks the case as swept.
ALTER TABLE jury_cases ADD COLUMN IF NOT EXISTS remainder_recipient TEXT;
ALTER TABLE jury_cases ADD COLUMN IF NOT EXISTS remainder_swept_mist BIGINT;

-- RosterEntryRefreshed: the last time the entry's eligibility snapshot was retaken.
ALTER TABLE jury_roster ADD COLUMN IF NOT EXISTS refreshed_at_ms BIGINT;

-- OrgTreasuryFunded / OrgTreasuryWithdrawn: one balance per organization and coin type, as of the
-- latest event (the chain reports the balance after each change).
CREATE TABLE IF NOT EXISTS org_treasury_balances (
    organization_id TEXT NOT NULL,
    coin_type TEXT NOT NULL,
    balance BIGINT NOT NULL,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (organization_id, coin_type)
);

-- OrgTreasuryAccessChanged: who may withdraw from an organization's treasury.
CREATE TABLE IF NOT EXISTS org_treasury_access (
    organization_id TEXT NOT NULL,
    recipient TEXT NOT NULL,
    has_access BOOLEAN NOT NULL,
    updated_by TEXT NOT NULL,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (organization_id, recipient)
);

-- Every treasury event, for history. `event_type` is funded, withdrawn or access_changed.
CREATE TABLE IF NOT EXISTS org_treasury_events (
    event_id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    coin_type TEXT,
    amount BIGINT,
    balance_after BIGINT,
    actor TEXT NOT NULL,
    recipient TEXT,
    has_access BOOLEAN,
    timestamp_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS org_treasury_events_org_time_idx
    ON org_treasury_events (organization_id, timestamp_ms DESC);

-- PostAccessPurchaseEvent: which post a MyData purchase unlocked. Price and fees stay on the
-- matching mydata_purchases row.
CREATE TABLE IF NOT EXISTS post_access_purchases (
    event_id TEXT PRIMARY KEY,
    post_id TEXT NOT NULL,
    mydata_id TEXT NOT NULL,
    buyer TEXT NOT NULL,
    seller TEXT NOT NULL,
    purchased_at_ms BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS post_access_purchases_post_idx
    ON post_access_purchases (post_id, purchased_at_ms DESC);
CREATE INDEX IF NOT EXISTS post_access_purchases_buyer_idx
    ON post_access_purchases (buyer, post_id);
