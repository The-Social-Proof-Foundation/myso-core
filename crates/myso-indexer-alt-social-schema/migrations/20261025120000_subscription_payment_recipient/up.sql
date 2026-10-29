-- Future subscription payouts can leave the profile owner. Existing services pay the owner
-- until a ServicePaymentRecipientUpdatedEvent changes this column.
ALTER TABLE profile_subscription_services
    ADD COLUMN IF NOT EXISTS payment_recipient TEXT;

UPDATE profile_subscription_services
SET payment_recipient = profile_owner
WHERE payment_recipient IS NULL;

ALTER TABLE profile_subscription_services
    ALTER COLUMN payment_recipient SET NOT NULL;
