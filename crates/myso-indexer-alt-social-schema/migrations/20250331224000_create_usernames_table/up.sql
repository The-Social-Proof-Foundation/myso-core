CREATE TABLE IF NOT EXISTS username_registry (
    username TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL UNIQUE,
    transaction_id TEXT NOT NULL
);
