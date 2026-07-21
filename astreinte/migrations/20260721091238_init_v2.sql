-- Add migration script here
-- 1. Table des Services (Groupes)
CREATE TABLE IF NOT EXISTS services (
    service_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    tag TEXT NOT NULL UNIQUE -- ex: [SSI], [RESEAU]
);

-- 2. Table des Utilisateurs
CREATE TABLE IF NOT EXISTS users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    service_id INTEGER, -- NULL pour les Admins globaux
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL, -- 'Admin', 'Manager', 'User'
    user_tag TEXT NOT NULL UNIQUE, -- Identifiant visuel court (ex: LPM-4B)
    must_change_password BOOLEAN NOT NULL DEFAULT 1,
    FOREIGN KEY (service_id) REFERENCES services(service_id)
);

-- 3. Table des Demandes d'Astreintes (Par Blocs)
CREATE TABLE IF NOT EXISTS shift_requests (
    request_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    service_id INTEGER NOT NULL,
    period_type TEXT NOT NULL, -- 'Semaine' ou 'Weekend'
    start_date TEXT NOT NULL, -- Date d'ancrage : Date du Lundi (pour Semaine) ou du Samedi (pour Weekend)
    status TEXT NOT NULL DEFAULT 'En attente', -- 'En attente', 'Validée', 'Refusée'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (service_id) REFERENCES services(service_id),
    -- On empêche un même utilisateur de faire deux fois la même demande
    UNIQUE(user_id, period_type, start_date) 
);