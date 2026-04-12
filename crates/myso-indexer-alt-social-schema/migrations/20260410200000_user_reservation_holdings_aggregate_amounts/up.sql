-- spt_reservation_holdings: spt_reservations.amount is a signed per-event delta (+deposit, -withdrawal),
-- not a running balance. Aggregate SUM(amount) per (pool_id, reserver_address) for current holdings.
-- Excludes reservation pools that already have a trading pool (post-launch).
-- SUM(bigint) is numeric; ::bigint keeps `amount` as bigint so CREATE OR REPLACE VIEW succeeds.
CREATE OR REPLACE VIEW spt_reservation_holdings AS
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
AND NOT EXISTS (
    SELECT 1 FROM spt_pools tok
    WHERE (
        tok.associated_id = sp.associated_id
        OR (
            LEFT(sp.associated_id, 8) = 'profile_'
            AND LENGTH(sp.associated_id) > 8
            AND tok.associated_id = SUBSTRING(sp.associated_id FROM 9)
        )
        OR (
            LEFT(tok.associated_id, 8) = 'profile_'
            AND LENGTH(tok.associated_id) > 8
            AND sp.associated_id = SUBSTRING(tok.associated_id FROM 9)
        )
    )
    LIMIT 1
)
ORDER BY agg.reserved_at DESC;
