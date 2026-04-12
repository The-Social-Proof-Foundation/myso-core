-- user_reservation_holdings: spt_reservations.amount is a signed per-event delta (+deposit, -withdrawal),
-- not a running balance. Aggregate SUM(amount) per (pool_id, reserver_address) for current holdings.
-- SUM(bigint) is numeric; ::bigint keeps `amount` as bigint so CREATE OR REPLACE VIEW succeeds.
CREATE OR REPLACE VIEW user_reservation_holdings AS
SELECT
    agg.reserver_address,
    agg.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    agg.amount,
    agg.reserved_at,
    sp.total_reserved,
    sp.required_threshold,
    (sp.total_reserved >= sp.required_threshold) AS threshold_met,
    sp.status AS pool_status
FROM (
    SELECT
        reserver_address,
        pool_id,
        SUM(amount)::bigint AS amount,
        MAX(reserved_at) AS reserved_at
    FROM spt_reservations
    GROUP BY reserver_address, pool_id
    HAVING SUM(amount) > 0
) agg
JOIN spt_reservation_pools sp ON agg.pool_id = sp.pool_id
WHERE sp.time = (
    SELECT MAX(time) FROM spt_reservation_pools sub
    WHERE sub.pool_id = sp.pool_id
)
ORDER BY agg.reserved_at DESC;
