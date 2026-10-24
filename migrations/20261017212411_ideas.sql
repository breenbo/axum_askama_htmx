-- Add migration script here
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS ideas (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    user_id     INTEGER,
    created_at  INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at  INTEGER NOT NULL DEFAULT (unixepoch()),
    deleted_at  INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
)
