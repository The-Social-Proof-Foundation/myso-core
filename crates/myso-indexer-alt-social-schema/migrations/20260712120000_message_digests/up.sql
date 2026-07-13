CREATE TABLE IF NOT EXISTS message_digests (
    id SERIAL NOT NULL,
    group_id TEXT NOT NULL,
    seq BIGINT NOT NULL,
    sender TEXT NOT NULL,
    recipient TEXT NOT NULL,
    content_digest TEXT NOT NULL,
    content_uri TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    transaction_id TEXT NOT NULL,
    PRIMARY KEY (id, time),
    UNIQUE (group_id, seq, time)
);
CREATE INDEX IF NOT EXISTS message_digests_group_time_idx ON message_digests (group_id, time DESC);
CREATE INDEX IF NOT EXISTS message_digests_recipient_time_idx ON message_digests (recipient, time DESC);
