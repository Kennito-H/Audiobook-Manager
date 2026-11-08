-- Key/value app settings. Secrets (API keys) never go here; they live in the OS keyring.
CREATE TABLE settings (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);
