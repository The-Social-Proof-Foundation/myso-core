-- Your SQL goes here

CREATE TABLE IF NOT EXISTS myso_burned
(
    event_digest                TEXT         PRIMARY KEY,
    digest                      TEXT         NOT NULL,
    sender                      TEXT         NOT NULL,
    checkpoint                  BIGINT       NOT NULL,
    timestamp                   TIMESTAMP    DEFAULT CURRENT_TIMESTAMP NOT NULL,
    checkpoint_timestamp_ms     BIGINT       NOT NULL,
    package                     TEXT         NOT NULL,
    pool_id                     TEXT         NOT NULL,
    burned_amount               BIGINT       NOT NULL
);

CREATE INDEX idx_myso_burned_pool_id ON myso_burned(pool_id);
CREATE INDEX idx_myso_burned_checkpoint ON myso_burned(checkpoint);