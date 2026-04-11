-- Restore previous definition (latest row per reserver/pool, not summed deltas).
CREATE OR REPLACE VIEW user_reservation_holdings AS
SELECT
    s.reserver_address,
    s.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    s.amount,
    s.reserved_at,
    sp.total_reserved,
    sp.required_threshold,
    (sp.total_reserved >= sp.required_threshold) AS threshold_met,
    sp.status AS pool_status
FROM
    spt_reservations s
JOIN
    spt_reservation_pools sp ON s.pool_id = sp.pool_id
WHERE
    s.time = (
        SELECT sub.time
        FROM spt_reservations sub
        WHERE sub.pool_id = s.pool_id AND sub.reserver_address = s.reserver_address
        ORDER BY GREATEST(
            sub.time,
            CASE
                WHEN sub.created_at >= 1000000000000 THEN to_timestamp(sub.created_at / 1000.0)
                ELSE '-infinity'::timestamptz
            END
        ) DESC,
        sub.time DESC
        LIMIT 1
    )
    AND sp.time = (
        SELECT MAX(time) FROM spt_reservation_pools sub
        WHERE sub.pool_id = sp.pool_id
    )
    AND s.amount > 0
ORDER BY
    s.reserved_at DESC;
