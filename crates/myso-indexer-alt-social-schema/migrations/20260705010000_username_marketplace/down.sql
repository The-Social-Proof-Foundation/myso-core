DROP TABLE IF EXISTS username_sale_fees CASCADE;
DROP TABLE IF EXISTS username_offers CASCADE;
DROP TABLE IF EXISTS username_listings CASCADE;

ALTER TABLE profile_config RENAME COLUMN username_sale_fee_bps TO profile_sale_fee_bps;

ALTER TABLE profiles ADD COLUMN IF NOT EXISTS min_offer_amount BIGINT NULL;

CREATE TABLE IF NOT EXISTS profile_offers (
    id SERIAL NOT NULL,
    profile_id TEXT NOT NULL,
    offeror_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    resolved_at BIGINT,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_profile_offers PRIMARY KEY (id, time)
);

CREATE TABLE IF NOT EXISTS profile_sale_fees (
    id SERIAL NOT NULL,
    profile_id TEXT NOT NULL,
    offeror_address TEXT NOT NULL,
    previous_owner_address TEXT NOT NULL,
    sale_amount BIGINT NOT NULL,
    fee_amount BIGINT NOT NULL,
    fee_recipient_address TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_profile_sale_fees PRIMARY KEY (id, time)
);
