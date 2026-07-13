-- Add migration script here
CREATE TABLE IF NOT EXISTS requests (
    request_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_email TEXT NOT NULL,
    date TEXT NOT NULL,
    shift_type TEXT NOT NULL,
    preference INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'En attente'
);