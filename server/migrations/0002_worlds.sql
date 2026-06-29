CREATE TABLE IF NOT EXISTS worlds (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    game_system TEXT NOT NULL DEFAULT 'dnd5e',
    db_path     TEXT NOT NULL,
    created_at  INTEGER NOT NULL DEFAULT (unixepoch()),
    last_opened INTEGER
);