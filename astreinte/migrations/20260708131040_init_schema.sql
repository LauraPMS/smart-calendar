-- Add migration script here
-- 1. Table users (Gestion des collaborateurs)
CREATE TABLE users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('Admin', 'User')),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    must_change_password INTEGER NOT NULL DEFAULT 1 
);

-- 2. Table slots (Créneaux disponibles)
CREATE TABLE slots (
    slot_id INTEGER PRIMARY KEY AUTOINCREMENT,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('Semaine', 'WE/Nuit'))
);

-- 3. Table preferences (Vœux des utilisateurs)
CREATE TABLE preferences (
    pref_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    slot_id INTEGER NOT NULL,
    score INTEGER NOT NULL CHECK(score IN (1, 2, 3)),
    FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY(slot_id) REFERENCES slots(slot_id) ON DELETE CASCADE
);

-- 4. Table assignments (Planning final validé)
CREATE TABLE assignments (
    assignment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    slot_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('Brouillon', 'En attente', 'Validé')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY(slot_id) REFERENCES slots(slot_id) ON DELETE CASCADE,
    FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- 5. Table access_tokens (Sécurité ICS)
CREATE TABLE access_tokens (
    token_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token_value TEXT NOT NULL UNIQUE,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
);