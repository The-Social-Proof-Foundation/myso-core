-- Per-buyer MyData access revocation columns and user_has_access helper.

ALTER TABLE mydata_purchases
    ADD COLUMN IF NOT EXISTS revoked BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS revoked_at BIGINT,
    ADD COLUMN IF NOT EXISTS revoked_by TEXT;

ALTER TABLE mydata_subscriptions
    ADD COLUMN IF NOT EXISTS revoked BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS revoked_at BIGINT,
    ADD COLUMN IF NOT EXISTS revoked_by TEXT;

CREATE INDEX IF NOT EXISTS idx_mydata_purchases_active
    ON mydata_purchases (mydata_id, buyer, time DESC)
    WHERE revoked = FALSE;

CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_active
    ON mydata_subscriptions (mydata_id, subscriber, time DESC)
    WHERE revoked = FALSE;

CREATE OR REPLACE FUNCTION user_has_access(
    p_mydata_id TEXT,
    p_user_address TEXT,
    p_current_time_ms BIGINT DEFAULT (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT
) RETURNS BOOLEAN AS $$
DECLARE
    data_owner TEXT;
    subscription_end BIGINT;
    has_purchase BOOLEAN := FALSE;
BEGIN
    SELECT owner INTO data_owner FROM mydata_data WHERE mydata_id = p_mydata_id;

    IF data_owner = p_user_address THEN
        RETURN TRUE;
    END IF;

    SELECT TRUE INTO has_purchase
    FROM mydata_purchases
    WHERE mydata_id = p_mydata_id
      AND buyer = p_user_address
      AND purchase_type = 'one_time'
      AND revoked = FALSE
    LIMIT 1;

    IF has_purchase THEN
        RETURN TRUE;
    END IF;

    SELECT MAX(subscription_end) INTO subscription_end
    FROM mydata_subscriptions
    WHERE mydata_id = p_mydata_id
      AND subscriber = p_user_address
      AND revoked = FALSE;

    IF subscription_end IS NOT NULL AND subscription_end >= p_current_time_ms THEN
        RETURN TRUE;
    END IF;

    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;
